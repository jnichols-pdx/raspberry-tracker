#![allow(unused_variables)]
#![allow(unused_imports)]
mod common;
mod session;
mod ui;
mod db;

//use std::env;
use crate::common::*;
use crate::session::*;
use crate::db::*;
use std::io::{self, stderr, Write, Error};
use tokio::sync::{mpsc, oneshot};
//use sqlite::State;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use futures_util::{future, pin_mut, StreamExt, SinkExt, stream::SplitSink, stream::SplitStream, TryStreamExt};
use std::thread;
use tokio::runtime::Runtime;
use std::sync::{Arc, RwLock};
use sqlx::sqlite::SqlitePool;
use time::OffsetDateTime;
use time_tz::OffsetDateTimeExt;
use image::io::Reader as ImageReader;
use std::io::Cursor;
//use tokio::time::{self, Duration};


//EGUI offers both native and web assembly compilation targets, I don't intend to use WASM.
#[cfg(not(target_arch = "wasm32"))] 
fn main() {

    let rt = Runtime::new().unwrap();

    let session_list = Arc::new(RwLock::new(Vec::<Session>::new()));
    let (tx_to_websocket, rx_from_app) = mpsc::channel::<Message>(32);
    let (report_to_main, report_from_ws) = mpsc::channel::<serde_json::Value>(64);
    let (tx_frame_to_ws, rx_frame_from_ui) = oneshot::channel::<epi::Frame>();

   
    //let mut dbo: Option<sqlite::Connection> = None ;
    let db_url;
    if let Some(dir_gen) = directories_next::ProjectDirs::from("com","JTNBrickWorks","Raspberry Tracker") {
        let data_dir = dir_gen.data_dir().to_path_buf();
        if let Ok(()) = std::fs::create_dir_all(&data_dir) {
            let db_path = data_dir.join("tracker_data.sqlite");
            db_url = format!("sqlite:{}?mode=rwc", db_path.to_string_lossy());
        } else {
            println!("couldn't find/make db directory, using ram instead.");
            db_url = format!("sqlite::memory:");
            
        }
    } else
    {
        println!("unable to determine platform directories, using ram instead.");
        db_url = format!("sqlite::memory:");
    }

    let db_pool;
    match rt.block_on(SqlitePool::connect(&db_url)) {
        Ok(pool) => db_pool = pool,
        Err(err) => {println!("DB OPEN ERRROR: {:?}", err);
                    std::process::exit(-2);},
    }

    let db = DatabaseCore::new(db_pool);
   

    let mut sync_db = DatabaseSync{
       dbc: db,
       rt: rt,
    };

    sync_db.init_sync();

    let mut char_to_track = None;
    let character_list = Arc::new(RwLock::new(sync_db.get_character_list_sync(tx_to_websocket.clone())));
    {
        let mut char_list_rw  = character_list.write().unwrap();
        for achar in &char_list_rw.characters {
            let _res = tx_to_websocket.blocking_send(
                Message::Text(
                    format!("{{\"service\":\"event\",\"action\":\"subscribe\",\"characters\":[\"{}\"],\"eventNames\":[\"PlayerLogin\",\"PlayerLogout\"]}}",
                    achar.character_id)
                .to_owned()));

            match is_online(&achar.character_id){
                Ok(true) => {
                    println!("{} is already online!", achar.full_name);
                    char_to_track = Some(achar.character_id.to_owned());
                    },
                Ok(false) => {},
                Err(e) => println!("Failed to get online status for {}:\n {}", &achar.character_id, e),
            }
        }
        if let Some(active_char_id) = char_to_track {
            match  lookup_new_char_details(&active_char_id) {
                Err(whut) => println!("{}", whut),
                Ok(details) => {

                    let active_char = full_character_from_json(&details).unwrap();

                    sync_db.update_char_with_full_sync(&active_char);

                    char_list_rw.update_entry_from_full(&active_char);
                    let _res = tx_to_websocket.blocking_send(
                        Message::Text(
                        format!("{{\"service\":\"event\",\"action\":\"subscribe\",\"characters\":[{}],\"eventNames\":[\"Death\"]}}",
                        &active_char.character_id).to_owned()));

                    {
                        let mut session_list_rw = session_list.write().unwrap();
                        session_list_rw.push(Session::new_from_full(active_char, OffsetDateTime::now_utc().unix_timestamp()));
                    }
                },
            }
        }

    }

    let mut native_options = eframe::NativeOptions::default();
    let (x_size, y_size) = sync_db.get_window_specs_sync();    


    println!("setting window as {} x {} ", x_size, y_size);
    native_options.initial_window_size = Some(egui::Vec2{ x: x_size as f32, y: y_size as f32});

    match  ImageReader::with_format(Cursor::new(include_bytes!("../Images/RaspberryTrackerIcon.png")), image::ImageFormat::Png)
        .decode() {
            Ok(image) => {
                let image_buffer = image.to_rgba8();
                native_options.icon_data = Some(eframe::epi::IconData {
                    rgba: image_buffer.into_raw(),
                    width: image.width(),
                    height: image.height(),
                });
            },
            Err(e) => {},
    }

    sync_db.rt.spawn(websocket_threads(rx_from_app,
                               tx_to_websocket.clone(),
                               report_to_main,
                               character_list.clone(),
                               session_list.clone(),
                               rx_frame_from_ui,
                               sync_db.dbc.clone(),
                              ));

    let app = ui::TrackerApp{
        in_character_ui: true,
        char_list: character_list.clone(),
        session_list: session_list.clone(),
        db: sync_db,
        lastx: x_size as f32,
        lasty: y_size as f32,
        size_changed: false,
        ws_messages: report_from_ws,
        ws_out: tx_to_websocket.clone(),
        frame_cb: Some(tx_frame_to_ws),
        session_count: 0,
        images: None,
    };


    eframe::run_native(Box::new(app), native_options);
}


async fn websocket_threads(rx_from_app: mpsc::Receiver<Message>,
                           ws_out: mpsc::Sender<Message>,
                           report_to_main: mpsc::Sender<serde_json::Value>,
                           char_list: Arc<RwLock<CharacterList>>,
                           session_list:  Arc<RwLock<Vec<Session>>>,
                           rx_ui_frame: oneshot::Receiver<epi::Frame>,
                           db: DatabaseCore,
                           ){
    let ws_url = url::Url::parse("wss://push.planetside2.com/streaming?environment=ps2&service-id=s:raspberrytracker").unwrap();
    let (ws_str, _) = connect_async(ws_url).await.unwrap();//.expect("failed to connect to streaming api");
    //println!("{:?}", ws_str);
    let (ws_write, ws_read) = ws_str.split();
    let (report_to_parser, ws_messages) = mpsc::channel::<serde_json::Value>(64);
    let ui_frame =  rx_ui_frame.await.unwrap();
    let out_task = tokio::spawn(ws_outgoing(rx_from_app, ws_write));
    let in_task = tokio::spawn(ws_incoming(ws_read, ws_out.clone(), report_to_main.clone(), report_to_parser));
    let parse_task = tokio::spawn(parse_messages(report_to_main, ws_messages,char_list, ws_out.clone(),session_list, ui_frame, db.clone()));

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
                        if json["payload"].is_object() {
                            let _ignore = report_to_parser.send(json).await;
                        } else if json["type"].is_string() {
                            let _ignore = report_to_parser.send(json).await;
                        } else  {
                            println!("-{}",json);
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
                ui_frame: epi::Frame,
                mut db: DatabaseCore,
        ) 
{
    let mut parsing = true;
    let local_tz;
    let local_tz_q = time_tz::system::get_timezone();
    match local_tz_q {
        Ok(local) => local_tz = local,
        Err(e) => {println!("Error finding system timezone: {}", e);
                        std::process::exit(-2);
        },
    }
    while parsing {
        match ws_messages.recv().await {
            Some(json) => {
                if json["type"].eq("heartbeat") {
                    println!(".");
                } else if json["payload"]["event_name"].eq("PlayerLogin") {
                    println!("online!");
                    let is_tracked;
                    {
                      let char_list_ro = char_list.read().unwrap();
                      is_tracked =char_list_ro.has_auto_tracked(json["payload"]["character_id"].to_string().unquote());
                    }
                    if is_tracked { 
                        match ws_out
                            .send(
                                    Message::Text(
                                    format!("{{\"service\":\"event\",\"action\":\"subscribe\",\"characters\":[{}],\"eventNames\":[\"Death\"]}}",
                                    json["payload"]["character_id"].to_owned()))).await 
                            {
                                Err(e) => println!("dah {:?}",e),
                                Ok(_) => {},
                            }

                        match  lookup_new_char_details(&json["payload"]["character_id"].to_string().unquote()) {
                            Err(whut) => println!("{}", whut),
                            Ok(details) => {

                                let bob = full_character_from_json(&details).unwrap();
                                {
                                    let mut char_list_rw = char_list.write().unwrap();
                                    char_list_rw.update_entry_from_full(&bob);
                                    //println!("did update");
                                }

                                //  db_update_char_with_full(&bob, 
                                
                                //ISSUE: can't hold an sqlite db connection here, it isn't sync/send so not for use with tokio.

                                {
                                    //HERE
                                    let mut session_list_rw = session_list.write().unwrap();
                                    session_list_rw.push(Session::new_from_full(bob, json["payload"]["timestamp"].to_string().unquote().parse::<i64>().unwrap()));
                                    ui_frame.request_repaint();
                                }
                            },
                        }

                    } else {
                        println!("Unknown or not auto-tracked, ignoring.");
                    }
                } else if json["payload"]["event_name"].eq("Death") {
                    println!("Found a death");
                    println!("{:?}", json);
                    let weapon_name = db.get_weapon_name(&json["payload"]["attacker_weapon_id"].to_string().unquote()).await;
                    let timestamp = json["payload"]["timestamp"].to_string().unquote().parse::<i64>().unwrap_or_else(|_| {0});
                    let datetime = OffsetDateTime::from_unix_timestamp(timestamp).unwrap_or_else(|_| {OffsetDateTime::now_utc()}).to_timezone(local_tz);
                    let formatted_time;
                    //let formatter = time::format_description::parse("[hour repr:12]:[minute]:[second] [period] [year]-[month]-[day]",).unwrap();
                    let formatter = time::format_description::parse("[hour repr:12]:[minute]:[second] [period]",).unwrap();
                    if let Ok(tstamp) = datetime.format(&formatter) {
                        formatted_time = tstamp;
                    } else {
                        formatted_time = "?-?-? ?:?:?".to_owned();
                    }
                    let vehicle_num =  json["payload"]["attacker_vehicle_id"].to_string().unquote().parse::<i64>().unwrap_or_else(|_| {-1});
                    let vehicle;
                    if vehicle_num <= 0 {
                        vehicle = None;
                    } else {
                        vehicle = Some( Vehicle::from(vehicle_num));
                    }

                    let mut attacker = false;
                    let mut some_player_char: Option<FullCharacter> = None;
                    {
                        let session_list_ro = session_list.read().unwrap();
                        if let Some(current_session) = session_list_ro.last() {
                            if current_session.match_player_id(&json["payload"]["attacker_character_id"].to_string().unquote()) {
                                println!("You killed someone!");
                                attacker = true;
                            } else {
                                println!("You died!!!!");
                                attacker = false;
                            }
                            some_player_char = Some(current_session.current_character()); //May be incomplete if KD ratio has shifted during session?
                        }
                    }
                    let player_char = some_player_char.unwrap();
//////////////////////

                    let mut event_type = EventType::Unknown;
                    let mut br = 0;
                    let mut asp = 0;
                    let mut name = "Unknown".to_owned();
                    let mut class = Class::Unknown;
                    let mut ratio = 0.5;
                    let mut faction = Faction::Unknown ;

                    //Suicide
                    if json["payload"]["character_id"] == json["payload"]["attacker_character_id"] {
                        event_type =  EventType::Suicide;
                        if let Some(outfit_alias) = player_char.outfit {
                            if outfit_alias == "" {
                                if let Some(outfit_name) = player_char.outfit_full {
                                    name = format!("[{}] {}", outfit_name, player_char.full_name);
                                } else {
                                    name = player_char.full_name.to_owned();
                                }
                            } else {
                                name = format!("[{}] {}", outfit_alias, player_char.full_name);
                            }
                        } else {
                            name = player_char.full_name.to_owned();
                        }
                        class = Class::from(json["payload"]["character_loadout_id"].to_string().unquote().parse::<i64>().unwrap_or_else(|_| {0}));
                        br = player_char.br;
                        asp = player_char.asp;
                        faction = player_char.faction;
                        ratio = 0.5;//TODO - get from current player details.

                    } else {
                        let mut deets = None;
                        if attacker { //Player character's ID was the attacker
                            //Killed other player
                            match  lookup_new_char_details(&json["payload"]["character_id"].to_string().unquote()) {
                                Err(whut) => println!("{}", whut),
                                Ok(details) => {
                                    println!("YOUR VICTIM:");
                                    println!("{:?}", details);
                                    let faction_num = details["character_list"][0]["faction_id"].to_string().unquote().parse::<i64>().unwrap_or_else(|_| {0});
                                    faction = Faction::from(faction_num);
                                    if faction == player_char.faction {
                                        event_type = EventType::TeamKill;
                                    } else {
                                        event_type = EventType::Kill;
                                    }
                                    class = Class::from(json["payload"]["character_loadout_id"].to_string().unquote().parse::<i64>().unwrap_or_else(|_| {0}));
                                    deets = Some(details["character_list"][0].clone());
                                }
                            }
                        } else { //Unrecognized character ID was attacker
                            //Died to other player
                            match  lookup_new_char_details(&json["payload"]["attacker_character_id"].to_string().unquote()) {
                                Err(whut) => println!("{}", whut),
                                Ok(details) => {
                                    println!("YOUR KILLER:");
                                    println!("{:?}", details);
                                    let faction_num = details["character_list"][0]["faction_id"].to_string().unquote().parse::<i64>().unwrap_or_else(|_| {0});
                                    faction = Faction::from(faction_num);
                                    if faction == player_char.faction {
                                        event_type = EventType::TeamDeath;
                                    } else {
                                        event_type = EventType::Death;
                                    }
                                    class = Class::from(json["payload"]["attacker_loadout_id"].to_string().unquote().parse::<i64>().unwrap_or_else(|_| {0}));
                                    deets = Some(details["character_list"][0].clone());
                                }
                            }

                        }
                        //Pull relevant data from the Census details for the attacker/victim.
                        if let Some(deets) = deets {
                            let player_name = deets["name"]["first"].to_string().unquote();
                            if deets["outfit"].is_object() {
                                let outfit_alias =  deets["outfit"]["alias"].to_string().unquote();
                                let outfit_name =  deets["outfit"]["name"].to_string().unquote();
                                if outfit_alias == "" {
                                    name = format!("[{}] {}", outfit_name, player_name);
                                } else {
                                    name = format!("[{}] {}", outfit_alias, player_name);
                                }

                            } else {
                                name = player_name;
                            }
                            br = deets["battle_rank"]["value"].to_string().unquote().parse::<u8>().unwrap_or_else(|_| {0});
                            asp = deets["prestige_level"].to_string().unquote().parse::<u8>().unwrap_or_else(|_| {0});
                            let kill_count = deets["kills"]["all_time"].to_string().unquote().parse::<u32>().unwrap_or_else(|_| {1});
                            let death_count = deets["weapon_deaths"]["value_forever"].to_string().unquote().parse::<u32>().unwrap_or_else(|_| {1});
                            ratio = kill_count as f32/ death_count as f32;
                       } else {
                        println!("no data.");
                       }
                    }

                    //Assemble it all and save.
                    let event = Event {
                        kind: event_type,
                        faction: faction,
                        br: br,
                        asp: asp,
                        class: class,
                        name: name,
                        weapon: weapon_name,
                        headshot: json["payload"]["is_headshot"].to_string().unquote().parse::<u8>().unwrap_or_else(|_| {0}) > 0,
                        kdr: ratio,
                        timestamp: timestamp,
                        vehicle: vehicle,
                        datetime: formatted_time,
                    };

                    let mut session_list_rw = session_list.write().unwrap();
                    if let Some(current_session) = session_list_rw.last_mut() {
                        current_session.log_event(event);
                        ui_frame.request_repaint();
                    }


/////////////////////

                } else if json["payload"]["event_name"].eq("PlayerLogout") {
                    println!("offline!");
                    let timestamp = json["payload"]["timestamp"].to_string().unquote().parse::<i64>().unwrap();
                    let _res = ws_out.send(
                            Message::Text("{\"service\":\"event\",\"action\":\"clearSubscribe\",\"eventNames\":[\"Death\"]}"
                                .to_string())).await;
                    let mut session_list_rw = session_list.write().unwrap();
                    if let Some(current_session) = session_list_rw.last_mut() {
                        current_session.end(timestamp);
                    }
                    ui_frame.request_repaint();
                }else {
                    println!("+{}", json);
                }

            },
            None => {
                println!("DOH!");
                parsing = false;
            },
        }

    }
}
