#![allow(unused_variables)]
#![allow(unused_imports)]
mod common;
mod session;
mod ui;

//use std::env;
use crate::common::*;
use crate::session::*;
use std::io::{self, stderr, Write, Error};
use tokio::sync::{mpsc};
use sqlite::State;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use futures_util::{future, pin_mut, StreamExt, SinkExt, stream::SplitSink, stream::SplitStream, TryStreamExt};
use std::thread;
use tokio::runtime::Runtime;
use std::sync::{Arc, RwLock};
//use tokio::time::{self, Duration};


//EGUI offers both native and web assembly compilation targets, I don't intend to use WASM.
#[cfg(not(target_arch = "wasm32"))] 
fn main() {

  //  let (tx_to_ui, rx_from_main) = mpsc::channel::<Action>(32);
    let session_list = Arc::new(RwLock::new(Vec::<Session>::new()));
    let (tx_to_websocket, rx_from_app) = mpsc::channel::<Message>(32);
    let (report_to_main, report_from_ws) = mpsc::channel::<serde_json::Value>(64);

    let mut dbo: Option<sqlite::Connection> = None ;
    if let Some(dir_gen) = directories_next::ProjectDirs::from("com","JTNBrickWorks","Raspberry Tracker") {
        let data_dir = dir_gen.data_dir().to_path_buf();
        if let Ok(()) = std::fs::create_dir_all(&data_dir) {
            let db_path = data_dir.join("tracker_data.sqlite");
            let connection = sqlite::open(db_path).unwrap();
            dbo = Some(connection);
        } else {
            println!("couldn't save to disk, use ram instead.");
            let connection = sqlite::open(":memory:").unwrap();
            dbo = Some(connection);
        }
    }

    let db = dbo.unwrap();

    match db.execute("SELECT version FROM raspberrytracker LIMIT 1;") {
        Err(err) => {
            if err.message == Some("no such table: raspberrytracker".to_string()) {
                println!("Setting up local database.");
                db.execute("CREATE TABLE raspberrytracker (version NUMBER);
                        INSERT INTO raspberrytracker VALUES (0.1); 
                        CREATE TABLE windows (name TEXT, width NUMBER, height NUMBER);
                        INSERT INTO windows VALUES ('main', 800.0, 480.0);
                        CREATE TABLE characters (name TEXT, lower_name TEXT, outfit TEXT, outfit_full TEXT, id TEXT NOT NULL, auto_track INTEGER, server INTEGER, faction INTEGER, PRIMARY KEY (id));
                        CREATE TABLE weapons (name TEXT, id TEXT);
                        ",).unwrap() ;
            } else {
                println!("sqlhuh? {:?}", err.message);
            }},
        Ok(_) => {},
    };
      
    //Scope the DB statement
    {
        let mut statement = db.prepare("SELECT version FROM raspberrytracker LIMIT 1;").unwrap();
        if let State::Row = statement.next().unwrap() {
            println!("db ver = {}", statement.read::<f64>(0).unwrap());
        }
    }

    let mut character_list = Arc::new(RwLock::new(CharacterList::new(tx_to_websocket.clone())));

    //Scope the DB cursor and write access to character_list
    {
        let mut char_builder = character_list.write().unwrap();
        let mut cursor = db.prepare("SELECT * FROM characters;").unwrap().into_cursor();
        while let Some(row) = cursor.next().unwrap() {
            //println!("{:?}", row);
            let mut achar = Character {
                full_name: row[0].as_string().unwrap().to_string(),
                lower_name: row[1].as_string().unwrap().to_string(),
                outfit: None,
                outfit_full: None,
                character_id: row[4].as_string().unwrap().to_string(),
                auto_track: row[5].as_integer().unwrap() > 0,
                server: row[6].as_integer().unwrap().into(),
                faction: row[7].as_integer().unwrap().into(),
                to_remove: false,
                confirm_visible: false,
                to_track: false,
                changed_auto_track: false,
            };
            match row[2].as_string() {
                Some(outfit_alias) => achar.outfit = Some(outfit_alias.to_string()),
                None => {},
            }
            match row[3].as_string() {
                Some(outfit_name) => achar.outfit_full = Some(outfit_name.to_string()),
                None => {},
            }


            let _res = tx_to_websocket.blocking_send(
                Message::Text(format!("{{\"service\":\"event\",\"action\":\"subscribe\",\"characters\":[\"{}\"],\"eventNames\":[\"PlayerLogin\",\"PlayerLogout\"]}}",
                achar.character_id).to_owned()));

            char_builder.push(achar);

        }
    }

    let mut native_options = eframe::NativeOptions::default();
    let mut x_size:f64 = 800.0;
    let mut y_size:f64 = 480.0;

    //Scope the DB statement
    {
        let mut statement = db.prepare("SELECT * FROM windows WHERE name LIKE 'main' LIMIT 1;").unwrap();
        if let State::Row = statement.next().unwrap() {
            x_size =  statement.read::<f64>(1).unwrap();
            y_size =  statement.read::<f64>(2).unwrap();
            println!("setting window as {} x {} ", x_size, y_size);
            native_options.initial_window_size = Some(egui::Vec2{ x: x_size as f32, y: y_size as f32});
        }
    }


    let app = ui::TrackerApp{
        //from_main: rx_from_main,
        in_character_ui: true,
        char_list: character_list.clone(),
        session_list: session_list.clone(),
        db: db,
        lastx: x_size as f32,
        lasty: y_size as f32,
        size_changed: false,
        ws_messages: report_from_ws,
        ws_out: tx_to_websocket.clone(),
    };

    let _ws_threads = thread::spawn(move || {runtime_thread(rx_from_app,
                                                            tx_to_websocket.clone(),
                                                            report_to_main,
                                                            character_list.clone(),
                                                            session_list.clone(),
                                                            )
                                             });

    let bill: Session;

    eframe::run_native(Box::new(app), native_options);
}

fn runtime_thread(rx_from_app: mpsc::Receiver<Message>,
                  ws_out: mpsc::Sender<Message>,
                  report_to_main: mpsc::Sender<serde_json::Value>,
                  char_list: Arc<RwLock<CharacterList>>,
                  session_list:  Arc<RwLock<Vec<Session>>>,
                  ){
    let rt = Runtime::new().unwrap();
    let _x = rt.enter();
    rt.block_on(websocket_threads(rx_from_app, ws_out, report_to_main, char_list, session_list));
}

async fn websocket_threads(rx_from_app: mpsc::Receiver<Message>,
                           ws_out: mpsc::Sender<Message>,
                           report_to_main: mpsc::Sender<serde_json::Value>,
                           char_list: Arc<RwLock<CharacterList>>,
                           session_list:  Arc<RwLock<Vec<Session>>>,
                           ){
    let ws_url = url::Url::parse("wss://push.planetside2.com/streaming?environment=ps2&service-id=s:example").unwrap();
    let (ws_str, _) = connect_async(ws_url).await.unwrap();//.expect("failed to connect to streaming api");
    //println!("{:?}", ws_str);
    let (ws_write, ws_read) = ws_str.split();
    let (report_to_parser, ws_messages) = mpsc::channel::<serde_json::Value>(64);
    let out_task = tokio::spawn(ws_outgoing(rx_from_app, ws_write));
    let in_task = tokio::spawn(ws_incoming(ws_read, ws_out.clone(), report_to_main.clone(), report_to_parser));
    let parse_task = tokio::spawn(parse_messages(report_to_main, ws_messages,char_list, ws_out.clone(),session_list));

    tokio::select!{
        _ = out_task => {},
        _ = in_task => {},
        _ = parse_task => {},
    }

}

async fn ws_outgoing(mut rx_from_app: mpsc::Receiver<Message>, 
                mut ws_out: futures_util::stream::SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>){
 let mut looking = true;
        while looking {
            match rx_from_app.recv().await {
                Some(msg) => {
                    println!("Want to send {}", msg);
                    let _result = ws_out.send(msg).await;

                },
                None => {
                    println!("DOH!");
                    looking = false;
                },
            }
        }
}

async fn ws_incoming(ws_in: futures_util::stream::SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
                ws_out: mpsc::Sender<Message>,
                report_to_main: mpsc::Sender<serde_json::Value>,
                report_to_parser: mpsc::Sender<serde_json::Value>,
                ){
         let print_task =    ws_in.for_each(|message| async {
                match message.unwrap().into_text() {
                    Ok(msg) => {
                        //println!("{}", msg);
                        let json: serde_json::Value = serde_json::from_str(&msg).expect("parse JSON fail");
                        println!("{}", json);
                        if json["payload"]["event_name"].eq("PlayerLogin") {
                            println!("online!");
                            let _ignore = report_to_parser.send(json).await;
                        }else if json["payload"]["event_name"].eq("PlayerLogout") {
                            println!("offline!");
                            let _res = ws_out.send(
                                Message::Text("{\"service\":\"event\",\"action\":\"clearSubscribe\",\"eventNames\":[\"Death\"]}"
                                .to_string())).await;
                        }
                    },
                    Err(e) => {
                        println!("DIH {:?}", e);
                    },
                }
            });
         tokio::select! {
             _ = print_task => {},
         }

}

async fn parse_messages(
                report_to_main: mpsc::Sender<serde_json::Value>,
                mut ws_messages: mpsc::Receiver<serde_json::Value>,
                char_list: Arc<RwLock<CharacterList>>,
                ws_out: mpsc::Sender<Message>,
                session_list:  Arc<RwLock<Vec<Session>>>,
                ) {
    let mut parsing = true;
    while parsing {
        match ws_messages.recv().await {
            Some(json) => {
                println!("Want to parse {}", json);
                if json["payload"]["event_name"].eq("PlayerLogin") {
                    println!("^");
                    let mut is_tracked = false;
                    {
                        let char_list_ro = char_list.read().unwrap();
                        is_tracked =char_list_ro.has_auto_tracked(json["payload"]["character_id"].to_string().unquote());
                    }
                    if is_tracked { 
                    match ws_out
                        .send(
                         Message::Text(format!("{{\"service\":\"event\",\"action\":\"subscribe\",\"characters\":[{}],\"eventNames\":[\"Death\"]}}",
                         json["payload"]["character_id"].to_owned()))).await {
                        Err(e) => println!("dah {:?}",e),
                        Ok(_) => {},
                        }
                    } else {
                        println!("Unknown or not auto-tracked, ignoring.");
                    }
                }

            },
            None => {
                println!("DOH!");
                parsing = false;
            },
        }

    }
}
