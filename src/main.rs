mod character;
mod character_list;
mod common;
mod db;
mod events;
mod session;
mod ui;
mod weapons;

//use std::env;
use crate::character::*;
use crate::character_list::*;
use crate::common::*;
use crate::db::*;
use crate::events::*;
use crate::session::*;
use futures_util::{SinkExt, StreamExt};
use image::io::Reader as ImageReader;
use sqlx::sqlite::SqlitePool;
use std::io::Cursor;
use std::sync::{Arc, RwLock};
use time::OffsetDateTime;
use time_tz::OffsetDateTimeExt;
use tokio::runtime::Runtime;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};

//EGUI offers both native and web assembly compilation targets, I don't intend to use WASM.
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let rt = Runtime::new().unwrap();

    let session_list = Arc::new(RwLock::new(Vec::<Session>::new()));
    let (tx_to_websocket, rx_from_app) = mpsc::channel::<Message>(32);
    let (tx_frame_to_ws, rx_frame_from_ui) = oneshot::channel::<epi::Frame>();

    //let mut dbo: Option<sqlite::Connection> = None ;
    let db_url;
    if let Some(dir_gen) =
        directories_next::ProjectDirs::from("com", "JTNBrickWorks", "Raspberry Tracker")
    {
        let data_dir = dir_gen.data_dir().to_path_buf();
        if let Ok(()) = std::fs::create_dir_all(&data_dir) {
            let db_path = data_dir.join("tracker_data.sqlite");
            db_url = format!("sqlite:{}?mode=rwc", db_path.to_string_lossy());
        } else {
            println!("couldn't find/make db directory, using ram instead.");
            db_url = "sqlite::memory:".to_owned();
        }
    } else {
        println!("unable to determine platform directories, using ram instead.");
        db_url = "sqlite::memory:".to_owned();
    }

    let db_pool = match rt.block_on(SqlitePool::connect(&db_url)) {
        Ok(pool) => pool,
        Err(err) => {
            println!("DB OPEN ERRROR: {:?}", err);
            std::process::exit(-2);
        }
    };

    let db = DatabaseCore::new(db_pool);

    let mut sync_db = DatabaseSync { dbc: db, rt };

    sync_db.init_sync();

    let mut char_to_track = None;
    let character_list = Arc::new(RwLock::new(
        sync_db.get_character_list_sync(tx_to_websocket.clone(), session_list.clone()),
    ));
    {
        let mut char_list_rw = character_list.write().unwrap();
        for achar in &char_list_rw.characters {
            let _res = tx_to_websocket.blocking_send(
                Message::Text(
                    format!("{{\"service\":\"event\",\"action\":\"subscribe\",\"characters\":[\"{}\"],\"eventNames\":[\"PlayerLogin\",\"PlayerLogout\"]}}",
                    achar.character_id)
                .to_owned()));

            if achar.auto_track {
                match is_online(&achar.character_id) {
                    Ok(true) => {
                        println!("{} is already online!", achar.full_name);
                        char_to_track = Some(achar.character_id.to_owned());
                    }
                    Ok(false) => {}
                    Err(e) => println!(
                        "Failed to get online status for {}:\n {}",
                        &achar.character_id, e
                    ),
                }
            }
        }
        if let Some(active_char_id) = char_to_track {
            match lookup_new_char_details(&active_char_id) {
                Err(whut) => println!("{}", whut),
                Ok(details) => {
                    let active_char = FullCharacter::from_json(&details).unwrap();

                    sync_db.update_char_with_full_sync(&active_char);

                    char_list_rw.update_entry_from_full(&active_char);
                    let _res = tx_to_websocket.blocking_send(Message::Text(
                        subscribe_session_string(&active_char.character_id),
                    ));
                    {
                        let mut session_list_rw = session_list.write().unwrap();
                        session_list_rw.push(Session::new(
                            active_char,
                            OffsetDateTime::now_utc().unix_timestamp(),
                        ));
                    }
                }
            }
        }
    }

    let mut native_options = eframe::NativeOptions::default();
    let (x_size, y_size) = sync_db.get_window_specs_sync();

    println!("setting window as {} x {} ", x_size, y_size);
    native_options.initial_window_size = Some(egui::Vec2 {
        x: x_size as f32,
        y: y_size as f32,
    });

    if let Ok(image) = ImageReader::with_format(
        Cursor::new(include_bytes!("../Images/RaspberryTrackerIcon.png")),
        image::ImageFormat::Png,
    )
    .decode()
    {
        let image_buffer = image.to_rgba8();
        native_options.icon_data = Some(eframe::epi::IconData {
            rgba: image_buffer.into_raw(),
            width: image.width(),
            height: image.height(),
        });
    };

    sync_db.rt.spawn(websocket_threads(
        rx_from_app,
        tx_to_websocket.clone(),
        character_list.clone(),
        session_list.clone(),
        rx_frame_from_ui,
        sync_db.dbc.clone(),
    ));

    let app_ui = ui::TrackerApp {
        in_character_ui: true,
        char_list: character_list,
        session_list,
        db: sync_db,
        lastx: x_size as f32,
        lasty: y_size as f32,
        size_changed: false,
        ws_out: tx_to_websocket,
        frame_cb: Some(tx_frame_to_ws),
        session_count: 0,
        images: None,
    };

    eframe::run_native(Box::new(app_ui), native_options);
}

async fn websocket_threads(
    rx_from_app: mpsc::Receiver<Message>,
    ws_out: mpsc::Sender<Message>,
    char_list: Arc<RwLock<CharacterList>>,
    session_list: Arc<RwLock<Vec<Session>>>,
    rx_ui_frame: oneshot::Receiver<epi::Frame>,
    db: DatabaseCore,
) {
    let ws_url = url::Url::parse(
        "wss://push.planetside2.com/streaming?environment=ps2&service-id=s:raspberrytracker",
    )
    .unwrap();
    let (ws_str, _) = connect_async(ws_url).await.unwrap(); //.expect("failed to connect to streaming api");
                                                            //println!("{:?}", ws_str);
    let (ws_write, ws_read) = ws_str.split();
    let (report_to_parser, ws_messages) = mpsc::channel::<serde_json::Value>(64);
    let ui_frame = rx_ui_frame.await.unwrap();
    let out_task = tokio::spawn(ws_outgoing(rx_from_app, ws_write));
    let in_task = tokio::spawn(ws_incoming(ws_read, report_to_parser));
    let parse_task = tokio::spawn(parse_messages(
        ws_messages,
        char_list,
        ws_out.clone(),
        session_list.clone(),
        ui_frame.clone(),
        db.clone(),
    ));
    let ticker_task = tokio::spawn(ticker(ui_frame));
    let session_long_update_task = tokio::spawn(session_historical_update(session_list.clone()));

    tokio::select! {
        _ = out_task => {},
        _ = in_task => {},
        _ = parse_task => {},
        _ = ticker_task => {},
        _ = session_long_update_task => {},
    }
}

async fn ws_outgoing(
    mut rx_from_app: mpsc::Receiver<Message>,
    mut ws_out: futures_util::stream::SplitSink<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        Message,
    >,
) {
    let mut looking = true;
    while looking {
        match rx_from_app.recv().await {
            Some(msg) => {
                println!("Want to send {}", msg);
                let _result = ws_out.send(msg).await;
            }
            None => {
                println!("DOH!");
                looking = false;
            }
        }
    }
}

async fn ws_incoming(
    ws_in: futures_util::stream::SplitStream<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    >,
    report_to_parser: mpsc::Sender<serde_json::Value>,
) {
    let print_task = ws_in.for_each(|message| async {
        match message.unwrap().into_text() {
            Ok(msg) => {
                //println!("{}", msg);
                let json: serde_json::Value = serde_json::from_str(&msg).expect("parse JSON fail");
                if json["payload"].is_object() {
                    let _ignore = report_to_parser.send(json).await;
                } else if json["type"].is_string() {
                    let _ignore = report_to_parser.send(json).await;
                } else {
                    println!("-{}", json);
                }
            }
            Err(e) => {
                println!("DIH {:?}", e);
            }
        }
    });

    tokio::select! {
        _ = print_task => {},
    }
}

async fn parse_messages(
    mut ws_messages: mpsc::Receiver<serde_json::Value>,
    char_list: Arc<RwLock<CharacterList>>,
    ws_out: mpsc::Sender<Message>,
    session_list: Arc<RwLock<Vec<Session>>>,
    ui_frame: epi::Frame,
    mut db: DatabaseCore,
) {
    let mut parsing = true;
    let local_tz_q = time_tz::system::get_timezone();
    let local_tz = match local_tz_q {
        Ok(local) => local,
        Err(e) => {
            println!("Error finding system timezone: {}", e);
            std::process::exit(-2);
        }
    };
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
                        is_tracked = char_list_ro.has_auto_tracked(
                            json["payload"]["character_id"].to_string().unquote(),
                        );
                    }
                    if is_tracked {
                        if let Err(e) = ws_out
                            .send(Message::Text(subscribe_session_string(
                                &json["payload"]["character_id"].to_string(),
                            )))
                            .await
                        {
                            println!("dah {:?}", e);
                        };

                        match lookup_new_char_details(
                            &json["payload"]["character_id"].to_string().unquote(),
                        ) {
                            Err(whut) => println!("{}", whut),
                            Ok(details) => {
                                let bob = FullCharacter::from_json(&details).unwrap();
                                {
                                    let mut char_list_rw = char_list.write().unwrap();
                                    char_list_rw.update_entry_from_full(&bob);
                                    //println!("did update");
                                }

                                db.update_char_with_full(&bob).await;

                                {
                                    //HERE
                                    let mut session_list_rw = session_list.write().unwrap();
                                    session_list_rw.push(Session::new(
                                        bob,
                                        json["payload"]["timestamp"]
                                            .to_string()
                                            .unquote()
                                            .parse::<i64>()
                                            .unwrap(),
                                    ));
                                    ui_frame.request_repaint();
                                }
                            }
                        }
                    } else {
                        println!("Unknown or not auto-tracked, ignoring.");
                    }
                } else if json["payload"]["event_name"].eq("Death")
                    || json["payload"]["event_name"].eq("VehicleDestroy")
                {
                    let vehicle_destroyed = if json["payload"]["event_name"].eq("VehicleDestroy") {
                        println!("Found a VehicleDestroy");
                        true
                    } else {
                        println!("Found a death");
                        false
                    };
                    println!("{:?}", json);
                    let weapon_id = json["payload"]["attacker_weapon_id"].to_string().unquote();
                    let weapon_name = db.get_weapon_name(&weapon_id).await;
                    let timestamp = json["payload"]["timestamp"]
                        .to_string()
                        .unquote()
                        .parse::<i64>()
                        .unwrap_or(0);
                    let datetime = OffsetDateTime::from_unix_timestamp(timestamp)
                        .unwrap_or_else(|_| OffsetDateTime::now_utc())
                        .to_timezone(local_tz);
                    let formatter = time::format_description::parse(
                        "[hour repr:12]:[minute]:[second] [period]",
                    )
                    .unwrap();
                    let formatted_time = datetime
                        .format(&formatter)
                        .unwrap_or_else(|_| "?-?-? ?:?:?".into());

                    let vehicle_num = json["payload"]["attacker_vehicle_id"]
                        .to_string()
                        .unquote()
                        .parse::<i64>()
                        .unwrap_or(-1);
                    let vehicle = if vehicle_num <= 0 {
                        None
                    } else {
                        Some(Vehicle::from(vehicle_num))
                    };

                    let mut attacker = false;
                    let mut some_player_char: Option<FullCharacter> = None;
                    {
                        let session_list_ro = session_list.read().unwrap();
                        if let Some(current_session) = session_list_ro.last() {
                            if current_session.match_player_id(
                                &json["payload"]["attacker_character_id"]
                                    .to_string()
                                    .unquote(),
                            ) {
                                if !vehicle_destroyed {
                                    println!("You killed someone!");
                                } else {
                                    println!("You destroyed something!");
                                }
                                attacker = true;
                            } else {
                                if !vehicle_destroyed {
                                    println!("You died!!!!");
                                } else {
                                    println!("You lost something!");
                                }
                                attacker = false;
                            }
                            some_player_char = Some(current_session.current_character());
                            //May be incomplete if KD ratio has shifted during session?
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
                    let mut faction = Faction::Unknown;

                    //Suicide
                    if json["payload"]["character_id"] == json["payload"]["attacker_character_id"] {
                        event_type = EventType::Suicide;
                        if let Some(outfit_alias) = player_char.outfit {
                            if outfit_alias.is_empty() {
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
                        class = Class::from(
                            json["payload"]["character_loadout_id"]
                                .to_string()
                                .unquote()
                                .parse::<i64>()
                                .unwrap_or(0),
                        );
                        br = player_char.br;
                        asp = player_char.asp;
                        faction = player_char.faction;
                        ratio = 0.5; //TODO - get from current player details.
                    } else {
                        let mut deets = None;
                        if attacker {
                            //Player character's ID was the attacker
                            //Killed other player
                            match lookup_new_char_details(
                                &json["payload"]["character_id"].to_string().unquote(),
                            ) {
                                Err(whut) => println!("{}", whut),
                                Ok(details) => {
                                    if !vehicle_destroyed {
                                        println!("YOUR VICTIM:");
                                    } else {
                                        println!("YOUR TARGET'S OWNER:");
                                    }
                                    println!("{:?}", details);
                                    let faction_num = details["character_list"][0]["faction_id"]
                                        .to_string()
                                        .unquote()
                                        .parse::<i64>()
                                        .unwrap_or(0);
                                    faction = Faction::from(faction_num);
                                    if faction == player_char.faction {
                                        event_type = EventType::TeamKill;
                                    } else {
                                        event_type = EventType::Kill;
                                    }
                                    class = Class::from(
                                        json["payload"]["character_loadout_id"]
                                            .to_string()
                                            .unquote()
                                            .parse::<i64>()
                                            .unwrap_or(0),
                                    );
                                    deets = Some(details["character_list"][0].clone());
                                }
                            }
                        } else {
                            //Unrecognized character ID was attacker
                            //Died to other player
                            match lookup_new_char_details(
                                &json["payload"]["attacker_character_id"]
                                    .to_string()
                                    .unquote(),
                            ) {
                                Err(whut) => println!("{}", whut),
                                Ok(details) => {
                                    if !vehicle_destroyed {
                                        println!("YOUR KILLER:");
                                    } else {
                                        println!("YOUR RIDE'S DESTROYER:");
                                    }
                                    println!("{:?}", details);
                                    let faction_num = details["character_list"][0]["faction_id"]
                                        .to_string()
                                        .unquote()
                                        .parse::<i64>()
                                        .unwrap_or(0);
                                    faction = Faction::from(faction_num);
                                    if faction == player_char.faction {
                                        event_type = EventType::TeamDeath;
                                    } else {
                                        event_type = EventType::Death;
                                    }
                                    class = Class::from(
                                        json["payload"]["attacker_loadout_id"]
                                            .to_string()
                                            .unquote()
                                            .parse::<i64>()
                                            .unwrap_or(0),
                                    );
                                    deets = Some(details["character_list"][0].clone());
                                }
                            }
                        }
                        //Pull relevant data from the Census details for the attacker/victim.
                        if let Some(deets) = deets {
                            let player_name = deets["name"]["first"].to_string().unquote();
                            if deets["outfit"].is_object() {
                                let outfit_alias = deets["outfit"]["alias"].to_string().unquote();
                                let outfit_name = deets["outfit"]["name"].to_string().unquote();
                                if outfit_alias.is_empty() {
                                    name = format!("[{}] {}", outfit_name, player_name);
                                } else {
                                    name = format!("[{}] {}", outfit_alias, player_name);
                                }
                            } else {
                                name = player_name;
                            }
                            br = deets["battle_rank"]["value"]
                                .to_string()
                                .unquote()
                                .parse::<u8>()
                                .unwrap_or(0);
                            asp = deets["prestige_level"]
                                .to_string()
                                .unquote()
                                .parse::<u8>()
                                .unwrap_or(0);
                            let kill_count = deets["kills"]["all_time"]
                                .to_string()
                                .unquote()
                                .parse::<u32>()
                                .unwrap_or(1);
                            let death_count = deets["weapon_deaths"]["value_forever"]
                                .to_string()
                                .unquote()
                                .parse::<u32>()
                                .unwrap_or(1);
                            ratio = kill_count as f32 / death_count as f32;
                        } else {
                            println!("no data.");
                        }
                    }

                    if vehicle_destroyed {
                        let materiel_num = json["payload"]["vehicle_id"]
                            .to_string()
                            .unquote()
                            .parse::<i64>()
                            .unwrap_or(-1);
                        let material = Vehicle::from(materiel_num);
                        if material.is_true_vehicle() {
                            if materiel_num > 0 {
                                name = format!("{}({})", Vehicle::from(materiel_num), name);
                            }

                            event_type = match event_type {
                                EventType::Death => EventType::LoseVehicle,
                                EventType::TeamDeath => EventType::LoseVehicleFF,
                                EventType::Suicide => EventType::LoseVehicleFF,
                                EventType::Kill => EventType::DestroyVehicle,
                                EventType::TeamKill => EventType::DestroyVehicleFF,
                                _ => EventType::Unknown,
                            };
                        } else {
                            //Not a mobile vehicle (spitty, mana turret, base turret etc.) don't count it as a vehicle destroyed event.
                            println!("Supressing non-really-a-vehicle destruction.");
                            continue;
                        }
                    }

                    //Assemble it all and save.
                    let event = Event {
                        kind: event_type,
                        faction,
                        br,
                        asp,
                        class,
                        name,
                        weapon: weapon_name,
                        weapon_id,
                        headshot: json["payload"]["is_headshot"]
                            .to_string()
                            .unquote()
                            .parse::<u8>()
                            .unwrap_or(0)
                            > 0,
                        kdr: ratio,
                        timestamp,
                        vehicle,
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
                    let timestamp = json["payload"]["timestamp"]
                        .to_string()
                        .unquote()
                        .parse::<i64>()
                        .unwrap();
                    let _res = ws_out.send(
                            Message::Text("{\"service\":\"event\",\"action\":\"clearSubscribe\",\"eventNames\":[\"Death\",\"VehicleDestroy\"]}"
                                .to_string())).await;
                    let mut session_list_rw = session_list.write().unwrap();
                    if let Some(current_session) = session_list_rw.last_mut() {
                        current_session.end(timestamp);
                    }
                    ui_frame.request_repaint();
                } else {
                    println!("+{}", json);
                }
            }
            None => {
                println!("DOH!");
                parsing = false;
            }
        }
    }
}

async fn ticker(ui_frame: epi::Frame) {
    loop {
        sleep(Duration::from_millis(100)).await;
        ui_frame.request_repaint();
    }
}

async fn session_historical_update(session_list: Arc<RwLock<Vec<Session>>>) {
    loop {
        sleep(Duration::from_secs(300)).await;
        let mut session_list_rw = session_list.write().unwrap();
        if let Some(current_session) = session_list_rw.last_mut() {
            current_session.update_historical_stats();
        }
    }
}
