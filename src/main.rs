#![allow(unused_variables)]
#![allow(unused_imports)]
mod common;
mod ui;

//use std::env;
use crate::common::*;
use std::io::{self, stderr, Write, Error};
use tokio::sync::{mpsc};
use sqlite::State;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use futures_util::{future, pin_mut, StreamExt, SinkExt, stream::SplitSink, stream::SplitStream, TryStreamExt};
use std::thread;
use tokio::runtime::Runtime;
//use tokio::time::{self, Duration};


//EGUI offers both native and web assembly compilation targets, I don't intend to use WASM.
#[cfg(not(target_arch = "wasm32"))] 
fn main() {

    let (tx_to_ui, rx_from_main) = mpsc::channel::<Action>(32);
    let (tx_to_websocket, rx_from_app) = mpsc::channel::<Message>(32);

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
       
{
    let mut statement = db.prepare("SELECT version FROM raspberrytracker LIMIT 1;").unwrap();
    if let State::Row = statement.next().unwrap() {
        println!("db ver = {}", statement.read::<f64>(0).unwrap());
    }
}

    let mut character_list = CharacterList::new(tx_to_websocket.clone());

{
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
        };
        match row[2].as_string() {
            Some(outfit_alias) => achar.outfit = Some(outfit_alias.to_string()),
            None => {},
        }
        match row[3].as_string() {
            Some(outfit_name) => achar.outfit_full = Some(outfit_name.to_string()),
            None => {},
        }
        character_list.push(achar);
    }
}

    /*tokio::spawn(async move {
        let mut looking = true;
        while looking {
           ws_read. */


    let mut native_options = eframe::NativeOptions::default();

    let mut x_size:f64 = 800.0;
    let mut y_size:f64 = 480.0;
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
        from_main: rx_from_main,
        in_character_ui: true,
        char_list: character_list,
        db: db,
        lastx: x_size as f32,
        lasty: y_size as f32,
        size_changed: false,
    };

    let _ws_threads = thread::spawn(move || {runtime_thread(rx_from_app)});

    eframe::run_native(Box::new(app), native_options);
}

fn runtime_thread(rx_from_app: tokio::sync::mpsc::Receiver<Message>){
    let rt = Runtime::new().unwrap();
    let _x = rt.enter();
    rt.block_on(websocket_threads(rx_from_app));
}

async fn websocket_threads(rx_from_app: tokio::sync::mpsc::Receiver<Message>){
        let ws_url = url::Url::parse("wss://push.planetside2.com/streaming?environment=ps2&service-id=s:example").unwrap();
        let (ws_str, _) = connect_async(ws_url).await.unwrap();//.expect("failed to connect to streaming api");
        //println!("{:?}", ws_str);
        let (ws_write, ws_read) = ws_str.split();
        println!("{:?}", ws_read);
       let out_task = tokio::spawn(ws_outgoing(rx_from_app, ws_write));
       let in_task = tokio::spawn(ws_incoming(ws_read));

       tokio::select!{
           _ = out_task => {},
           _ = in_task => {},
        }

}

async fn ws_outgoing(mut rx_from_app: mpsc::Receiver<Message>, mut ws_out: futures_util::stream::SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>){
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

async fn ws_incoming(mut ws_in: futures_util::stream::SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>){
 let mut looking = true;
        //while looking {
         let print_task =    ws_in.for_each(|message| async {
                match message.unwrap().into_text() {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => {
                        println!("DIH {:?}", e);
                    },
                }
            });
         tokio::select! {
             _ = print_task => {},
         }

        //}
}
