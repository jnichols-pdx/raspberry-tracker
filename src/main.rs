mod character;
mod character_list;
mod common;
mod db;
mod events;
mod experience;
mod session;
mod session_list;
mod ui;
mod weapons;

//use std::env;
use crate::character::*;
use crate::character_list::*;
use crate::common::*;
use crate::db::*;
use crate::events::*;
use crate::experience::*;
use crate::session::*;
use crate::session_list::*;
use futures_util::{SinkExt, StreamExt};
use image::io::Reader as ImageReader;
use sqlx::sqlite::SqlitePool;
use std::io::Cursor;
use std::sync::Arc;
use time::OffsetDateTime;
use time_tz::OffsetDateTimeExt;
use tokio::runtime::Runtime;
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};

//EGUI offers both native and web assembly compilation targets, I don't intend to use WASM.
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let rt = Runtime::new().unwrap();
    let rth = rt.handle();

    let (tx_to_websocket, rx_from_app) = mpsc::channel::<Message>(32);
    let (tx_context_to_ws, rx_context_from_ui) = oneshot::channel::<egui::Context>();

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

    let mut sync_db = DatabaseSync {
        dbc: db,
        rt: rth.clone(),
    };

    sync_db.init_sync();

    let session_list = Arc::new(RwLock::new(sync_db.get_sessions_sync()));

    let mut char_to_track = None;
    let character_list = Arc::new(RwLock::new(
        sync_db.get_character_list_sync(tx_to_websocket.clone(), session_list.clone()),
    ));
    {
        let mut char_list_rw = character_list.blocking_write();
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
                        let mut session_list_rw = session_list.blocking_write();
                        session_list_rw.push(Session::new(
                            active_char,
                            OffsetDateTime::now_utc().unix_timestamp(),
                            sync_db.clone(),
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
        x: x_size,
        y: y_size,
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
        rx_context_from_ui,
        sync_db.clone(),
    ));

    eframe::run_native(
        "Raspberry Tracker",
        native_options,
        Box::new(move |cc| {
            Box::new(ui::TrackerApp::new(
                cc,
                character_list,
                session_list,
                sync_db,
                x_size,
                y_size,
                tx_to_websocket,
                Some(tx_context_to_ws),
            ))
        }),
    );
}

async fn websocket_threads(
    rx_from_app: mpsc::Receiver<Message>,
    ws_out: mpsc::Sender<Message>,
    char_list: Arc<RwLock<CharacterList>>,
    session_list: Arc<RwLock<SessionList>>,
    rx_ui_context: oneshot::Receiver<egui::Context>,
    db: DatabaseSync,
) {
    let ws_url = url::Url::parse(
        "wss://push.planetside2.com/streaming?environment=ps2&service-id=s:raspberrytracker",
    )
    .unwrap();
    let (ws_str, _) = connect_async(ws_url).await.unwrap(); //.expect("failed to connect to streaming api");
                                                            //println!("{:?}", ws_str);
    let (ws_write, ws_read) = ws_str.split();
    let (report_to_parser, ws_messages) = mpsc::channel::<serde_json::Value>(64);
    let ui_context = rx_ui_context.await.unwrap();
    let out_task = tokio::spawn(ws_outgoing(rx_from_app, ws_write));
    let in_task = tokio::spawn(ws_incoming(ws_read, report_to_parser));
    let parse_task = tokio::spawn(parse_messages(
        ws_messages,
        char_list,
        ws_out.clone(),
        session_list.clone(),
        ui_context.clone(),
        db.clone(),
    ));
    let ticker_task = tokio::spawn(ticker(ui_context));
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
    session_list: Arc<RwLock<SessionList>>,
    ui_context: egui::Context,
    mut db: DatabaseSync,
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
    let mut json = serde_json::json!(null);
    while parsing {
        match ws_messages.try_recv() {
            Ok(incoming_json) => json = incoming_json,
            Err(mpsc::error::TryRecvError::Empty) => {
                //Nothing outstanding to process, now is a decent time to save session state to the
                //database.
                {
                    let mut session_list_rw = session_list.write().await;
                    if let Some(current_session) = session_list_rw.active_session_mut() {
                        current_session.update_db_entry().await;
                    }
                }

                //Wait for next message / string of messages to arrive
                match ws_messages.recv().await {
                    Some(incoming_json) => json = incoming_json,
                    None => {
                        println!("DOH!");
                        parsing = false;
                    }
                }
            }
            Err(mpsc::error::TryRecvError::Disconnected) => {
                println!("DAH!");
                parsing = false;
            }
        }
        if parsing {
            //Parsing will be false once the MPSC channel closes, don't double parse the last JSON
            //message received.
            if json["type"].eq("heartbeat") {
                println!(".");
            } else if json["payload"]["event_name"].eq("PlayerLogin") {
                println!("online!");
                let is_tracked;
                {
                    let char_list_ro = char_list.read().await;
                    is_tracked = char_list_ro
                        .has_auto_tracked(json["payload"]["character_id"].to_string().unquote());
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
                                let mut char_list_rw = char_list.write().await;
                                char_list_rw.update_entry_from_full(&bob);
                                //println!("did update");
                            }

                            db.dbc.update_char_with_full(&bob).await;

                            {
                                let mut session_list_rw = session_list.write().await;
                                session_list_rw.push(
                                    Session::new_async(
                                        bob,
                                        json["payload"]["timestamp"]
                                            .to_string()
                                            .unquote()
                                            .parse::<i64>()
                                            .unwrap(),
                                        db.clone(),
                                    )
                                    .await,
                                );
                                ui_context.request_repaint();
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
                let weapon_name = db.dbc.get_weapon_name(&weapon_id).await;
                let timestamp = json["payload"]["timestamp"]
                    .to_string()
                    .unquote()
                    .parse::<i64>()
                    .unwrap_or(0);
                let datetime = OffsetDateTime::from_unix_timestamp(timestamp)
                    .unwrap_or_else(|_| OffsetDateTime::now_utc())
                    .to_timezone(local_tz);
                let formatter =
                    time::format_description::parse("[hour repr:12]:[minute]:[second] [period]")
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
                    let session_list_ro = session_list.read().await;
                    if let Some(current_session) = session_list_ro.active_session() {
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

                    let session_list_ro = session_list.read().await;
                    if let Some(session) = session_list_ro.active_session() {
                        ratio = session.current_true_kdr();
                    } else {
                        ratio = -1.0;
                    }
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

                let mut current_session_id = None;
                let mut event_ordering = 0;
                {
                    let mut session_list_rw = session_list.write().await;
                    if let Some(current_session) = session_list_rw.active_session_mut() {
                        event_ordering = current_session.log_event(event.clone()).await;
                        ui_context.request_repaint();
                        current_session_id = current_session.get_id();
                    }
                }

                if let Some(sess_id) = current_session_id {
                    db.dbc.record_event(&event, event_ordering, sess_id).await;
                }
            } else if json["payload"]["event_name"].eq("PlayerLogout") {
                println!("offline!");
                let timestamp = json["payload"]["timestamp"]
                    .to_string()
                    .unquote()
                    .parse::<i64>()
                    .unwrap();
                let _res = ws_out
                    .send(Message::Text(clear_subscribe_session_string()))
                    .await;
                let mut session_list_rw = session_list.write().await;
                if let Some(current_session) = session_list_rw.active_session_mut() {
                    current_session.end(timestamp).await;
                }
                ui_context.request_repaint();
            } else if json["payload"]["event_name"].eq("BattleRankUp") {
                println!("Found rankup!");
                if let Ok(latest_asp) =
                    lookup_character_asp(&json["payload"]["character_id"].to_string().unquote())
                {
                    let latest_br = json["payload"]["battle_rank"]
                        .to_string()
                        .unquote()
                        .parse::<u8>()
                        .unwrap_or(0);

                    let mut session_list_rw = session_list.write().await;
                    if let Some(current_session) = session_list_rw.active_session_mut() {
                        current_session.log_rankup(latest_br, latest_asp);
                    }
                }
            } else if json["payload"]["event_name"].eq("GainExperience") {
                let xp_id = json["payload"]["experience_id"]
                    .to_string()
                    .unquote()
                    .parse::<i64>()
                    .unwrap_or(0);
                let xp_type = ExperienceType::from(xp_id);
                println!("+{}", json);

                let player_id;
                {
                    let session_list_ro = session_list.read().await;
                    if let Some(current_session) = session_list_ro.active_session() {
                       player_id = current_session.current_character().character_id;
                    } else {
                        continue;
                    }
                }
                let timestamp = json["payload"]["timestamp"]
                    .to_string()
                    .unquote()
                    .parse::<i64>()
                    .unwrap_or(0);
                let datetime = OffsetDateTime::from_unix_timestamp(timestamp)
                    .unwrap_or_else(|_| OffsetDateTime::now_utc())
                    .to_timezone(local_tz);
                let formatter =
                    time::format_description::parse("[hour repr:12]:[minute]:[second] [period]")
                        .unwrap();
                let formatted_time = datetime
                    .format(&formatter)
                    .unwrap_or_else(|_| "?-?-? ?:?:?".into());

                let new_event;

                if !json["payload"]["character_id"].to_string().unquote().eq(&player_id) {
                    //Some events we do care about have our player as the target in 'other_id'
                    //rather than as the reveiver of the XP itself. Check for things like being
                    //revived.
                    if json["payload"]["other_id"].to_string().unquote().eq(&player_id) {
                        if xp_type == ExperienceType::Revive {
                            //WE have been revived. Log a Revived event instead.
                            let mut name =
                                format!("Unknown ({})", json["payload"]["character_id"]
                                    .to_string()
                                    .unquote()
                                );
                            let mut br = 0;
                            let mut asp = 0;
                            let mut kdr = 0.0;
                            let mut faction = Faction::Unknown;
                            let mut class = Class::Unknown;
                            match lookup_new_char_details(
                                &json["payload"]["character_id"].to_string().unquote(),
                            ) {
                                Err(whut) => println!("{}", whut),
                                Ok(details) => {
                                    println!("YOUR LIFE SAVER:");
                                    println!("{:?}", details);
                                    let faction_num = details["character_list"][0]["faction_id"]
                                        .to_string()
                                        .unquote()
                                        .parse::<i64>()
                                        .unwrap_or(0);
                                    faction = Faction::from(faction_num);
                                    class = Class::from(
                                        json["payload"]["character_loadout_id"]
                                            .to_string()
                                            .unquote()
                                            .parse::<i64>()
                                            .unwrap_or(0),
                                    );
                                    let player_name = details["character_list"][0]["name"]["first"].to_string().unquote();
                                    if details["character_list"][0]["outfit"].is_object() {
                                        let outfit_alias = details["character_list"][0]["outfit"]["alias"].to_string().unquote();
                                        let outfit_name = details["character_list"][0]["outfit"]["name"].to_string().unquote();
                                        if outfit_alias.is_empty() {
                                            name = format!("[{}] {}", outfit_name, player_name);
                                        } else {
                                            name = format!("[{}] {}", outfit_alias, player_name);
                                        }
                                    } else {
                                        name = player_name;
                                    }
                                    br = details["character_list"][0]["battle_rank"]["value"]
                                        .to_string()
                                        .unquote()
                                        .parse::<u8>()
                                        .unwrap_or(0);
                                    asp = details["character_list"][0]["prestige_level"]
                                        .to_string()
                                        .unquote()
                                        .parse::<u8>()
                                        .unwrap_or(0);
                                    let kill_count = details["character_list"][0]["kills"]["all_time"]
                                        .to_string()
                                        .unquote()
                                        .parse::<u32>()
                                        .unwrap_or(1);
                                    let death_count = details["character_list"][0]["weapon_deaths"]["value_forever"]
                                        .to_string()
                                        .unquote()
                                        .parse::<u32>()
                                        .unwrap_or(1);
                                    kdr = kill_count as f32 / death_count as f32;
                                }
                            }

                            new_event = Event {
                                kind: EventType::Revived,
                                faction,
                                br,
                                asp,
                                class,
                                name,
                                weapon: "Revived You".to_owned(),
                                weapon_id: "0".to_owned(),
                                headshot: false,
                                kdr,
                                timestamp,
                                vehicle: None,
                                datetime: formatted_time,
                            };
                        } else {
                            //Was someone else's XP pertaining to us, but of a type we don't care
                            //about.
                            println!("XP {} - {} - OTHER GUYS?? SKIP", xp_id, xp_type);
                            continue;
                        }
                    } else {
                        println!("XP {} - {} - Doesn't concern us, why did the API send us this XP tick??? ", xp_id, xp_type);
                        continue;
                    }
                } else {
                    println!("XP {} - {}", xp_id, xp_type);
                    let xp_amount = json["payload"]["amount"].to_string().unquote();


                    new_event = Event {
                        kind: EventType::ExperienceTick,
                        faction: Faction::from(0),
                        br: 0,
                        asp: 0,
                        class: Class::from(0),
                        name: xp_type.to_string(),
                        weapon: format!("+{}",xp_amount),
                        weapon_id: "0".to_owned(),
                        headshot: false,
                        kdr: 0.0,
                        timestamp,
                        vehicle: None,
                        datetime: formatted_time,
                    };
                }

                let mut current_session_id = None;
                let mut event_ordering = 0;

                let mut session_list_rw = session_list.write().await;
                if let Some(current_session) = session_list_rw.active_session_mut() {
                    event_ordering = current_session.log_event(new_event.clone()).await;
                    ui_context.request_repaint();
                    current_session_id = current_session.get_id();
                }
                if let Some(sess_id) = current_session_id {
                    db.dbc.record_event(&new_event, event_ordering, sess_id).await;
                }
            } else {
                println!("+{}", json);
            }
        }
    }
}

async fn ticker(ui_context: egui::Context) {
    loop {
        sleep(Duration::from_millis(100)).await;
        ui_context.request_repaint();
    }
}

async fn session_historical_update(session_list: Arc<RwLock<SessionList>>) {
    loop {
        sleep(Duration::from_secs(300)).await;
        let mut session_list_rw = session_list.write().await;
        if let Some(current_session) = session_list_rw.active_session_mut() {
            current_session.update_historical_stats().await;
        }
    }
}
