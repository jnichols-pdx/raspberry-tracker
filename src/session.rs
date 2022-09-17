use crate::character::*;
use crate::common::*;
use crate::db::*;
use crate::events::{Event, *};
use crate::weapons::*;
use eframe::egui;
use egui_extras::{Size, TableBuilder};
use sqlx::Row;
use std::collections::BTreeMap;
use time::{Date, OffsetDateTime};
use time_tz::{OffsetDateTimeExt, TimeZone, Tz};
use tokio::runtime::Handle;

#[derive(Clone)]
pub struct Session {
    character: FullCharacter,
    events: EventList,
    weapons_initial: BTreeMap<String, WeaponInitial>,
    weapons: WeaponSet,

    start_time: i64,
    end_time: Option<i64>,

    pub team: Team,

    kill_count: u32,
    death_count: u32,
    headshot_kills: u32,
    headshot_deaths: u32,
    vehicles_destroyed: u32,
    vehicles_lost: u32,
    vehicle_kills: u32,  //Killed someone using a vehicle
    vehicle_deaths: u32, //killed by someone else in a vehicle
    time_zone: &'static Tz,

    initial_kills_total: u64,
    initial_actual_deaths_total: u64,
    initial_revived_deaths_total: u64,
    initial_vehicles_destroyed: u64,
    initial_shots_fired: u64,
    initial_shots_hit: u64,
    initial_headshot_kills: u64,

    latest_api_kills: u64,
    latest_api_revived_deaths: u64,
    latest_api_shots_fired: u64,
    latest_api_shots_hit: u64,
    latest_api_headshots: u64,

    latest_br: u8,
    latest_asp: u8,
    pre_asp_rankups: u8,

    db_id: Option<i64>,
    dirty: bool,
    db: DatabaseSync,
}

impl Session {
    pub fn match_player_id(&self, to_match: &str) -> bool {
        to_match.eq(&self.character.character_id)
    }

    pub fn new(character: FullCharacter, start: i64, db: DatabaseSync) -> Self {
        let local_tz_q = time_tz::system::get_timezone();
        let local_tz = match local_tz_q {
            Ok(local) => local,
            Err(e) => {
                println!("Error finding system timezone: {}", e);
                std::process::exit(-2);
            }
        };

        println!("Session TZ is >{}<", local_tz.name());

        let mut init_kills = 0;
        let mut init_actual_deaths = 0;
        let mut init_revived_deaths = 0;
        let mut init_destroyed = 0;
        let mut init_shot = 0;
        let mut init_hit = 0;
        let mut init_headshots = 0;
        let mut weapons_initial = BTreeMap::new();

        match lookup_full_stats(&character.character_id) {
            Err(whut) => println!("Failed getting lifetime stats data:\n{}", whut),
            Ok(details) => {
                //println!("/nFULLSTATS:/n{:?}", details); //FAR too much to dump all to console.
                let stat_history =
                    &details["single_character_by_id_list"][0]["stats"]["stat_history"];
                let stat_block = details["single_character_by_id_list"][0]["stats"]["stat"]
                    .as_array()
                    .unwrap(); //TODO: more care to be taken
                               //let stat_by_faction = &details["single_character_by_id_list"][0]["stats"]["stat_by_faction"];
                let weapon_stat = details["single_character_by_id_list"][0]["stats"]["weapon_stat"]
                    .as_array()
                    .unwrap();
                let weapon_stat_by_faction = details["single_character_by_id_list"][0]["stats"]
                    ["weapon_stat_by_faction"]
                    .as_array()
                    .unwrap();

                init_kills = stat_history["kills"]["all_time"]
                    .to_string()
                    .unquote()
                    .parse::<u64>()
                    .unwrap();
                println!("Found lifetime kills: {}", init_kills);

                init_revived_deaths = stat_history["deaths"]["all_time"]
                    .to_string()
                    .unquote()
                    .parse::<u64>()
                    .unwrap();
                println!("Found lifetime deaths - revives: {}", init_revived_deaths);

                for stat in stat_block {
                    match stat["stat_name"].as_str() {
                        Some("weapon_hit_count") => {
                            init_hit = stat["value_forever"]
                                .to_string()
                                .unquote()
                                .parse::<u64>()
                                .unwrap();
                        }
                        Some("weapon_fire_count") => {
                            init_shot = stat["value_forever"]
                                .to_string()
                                .unquote()
                                .parse::<u64>()
                                .unwrap();
                        }
                        Some("weapon_deaths") => {
                            init_actual_deaths = stat["value_forever"]
                                .to_string()
                                .unquote()
                                .parse::<u64>()
                                .unwrap();
                        }
                        _ => {}
                    }
                }
                println!("Found lifetime deaths: {}", init_actual_deaths);
                println!("Found lifetime fired: {}", init_shot);
                println!("Found lifetime hit: {}", init_hit);

                for stat in weapon_stat {
                    let weapon_id = stat["item_id"].as_str().unwrap().to_owned();
                    if weapon_id == "0" {
                        //skip vehicles? does recursion track roadkills in per weapon stats?
                        continue;
                    }

                    if !weapons_initial.contains_key(&weapon_id) {
                        let wi = WeaponInitial::new();
                        weapons_initial.insert(weapon_id.clone(), wi);
                    }

                    if let Some(ws) = weapons_initial.get_mut(&weapon_id) {
                        match stat["stat_name"].as_str() {
                            Some("weapon_hit_count") => {
                                ws.hits +=
                                    stat["value"].to_string().unquote().parse::<u64>().unwrap();
                            }
                            Some("weapon_fire_count") => {
                                ws.fired +=
                                    stat["value"].to_string().unquote().parse::<u64>().unwrap();
                            }
                            Some(_) | None => {}
                        }
                    }
                }

                let mut vs_hs = 0;
                let mut nc_hs = 0;
                let mut tr_hs = 0;

                let mut vs_veh_destroy = 0;
                let mut nc_veh_destroy = 0;
                let mut tr_veh_destroy = 0;

                for stat in weapon_stat_by_faction {
                    match stat["stat_name"].as_str() {
                        Some("weapon_headshots") => {
                            let vs_val = stat["value_vs"]
                                .to_string()
                                .unquote()
                                .parse::<u64>()
                                .unwrap();
                            let nc_val = stat["value_nc"]
                                .to_string()
                                .unquote()
                                .parse::<u64>()
                                .unwrap();
                            let tr_val = stat["value_tr"]
                                .to_string()
                                .unquote()
                                .parse::<u64>()
                                .unwrap();

                            vs_hs += vs_val;
                            nc_hs += nc_val;
                            tr_hs += tr_val;

                            let weapon_id = stat["item_id"].as_str().unwrap().to_owned();
                            if weapon_id == "0" {
                                //skip vehicles? does recursion track roadkills in per weapon stats?
                                continue;
                            }
                            if !weapons_initial.contains_key(&weapon_id) {
                                let wi = WeaponInitial::new();
                                weapons_initial.insert(weapon_id.clone(), wi);
                            }
                            if let Some(ws) = weapons_initial.get_mut(&weapon_id) {
                                ws.headshots += vs_val + nc_val + tr_val;
                            }
                        }
                        Some("weapon_vehicle_kills") => {
                            vs_veh_destroy += stat["value_vs"]
                                .to_string()
                                .unquote()
                                .parse::<u64>()
                                .unwrap();
                            nc_veh_destroy += stat["value_nc"]
                                .to_string()
                                .unquote()
                                .parse::<u64>()
                                .unwrap();
                            tr_veh_destroy += stat["value_tr"]
                                .to_string()
                                .unquote()
                                .parse::<u64>()
                                .unwrap();
                        }
                        Some("weapon_kills") => {
                            let weapon_id = stat["item_id"].as_str().unwrap().to_owned();
                            if weapon_id == "0" {
                                //skip vehicles? does recursion track roadkills in per weapon stats?
                                continue;
                            }
                            if !weapons_initial.contains_key(&weapon_id) {
                                let wi = WeaponInitial::new();
                                weapons_initial.insert(weapon_id.clone(), wi);
                            }
                            if let Some(ws) = weapons_initial.get_mut(&weapon_id) {
                                ws.kills += stat["value_vs"]
                                    .to_string()
                                    .unquote()
                                    .parse::<u64>()
                                    .unwrap();
                                ws.kills += stat["value_nc"]
                                    .to_string()
                                    .unquote()
                                    .parse::<u64>()
                                    .unwrap();
                                ws.kills += stat["value_tr"]
                                    .to_string()
                                    .unquote()
                                    .parse::<u64>()
                                    .unwrap();
                            }
                        }
                        _ => {}
                    }
                }

                init_headshots = vs_hs + nc_hs + tr_hs;
                println!(
                    "headshots: VS {}, NC {}, TR {}, Total: {}",
                    vs_hs, nc_hs, tr_hs, init_headshots
                );
                init_destroyed = vs_veh_destroy + nc_veh_destroy + tr_veh_destroy;
                println!(
                    "vehicle destroys : VS {}, NC {}, TR {}, Total: {}",
                    vs_veh_destroy, nc_veh_destroy, tr_veh_destroy, init_destroyed
                );
            }
        }

        let latest_br = character.br;
        let latest_asp = character.asp;

        let rt = db.rt.clone();

        let mut new_session = Session {
            character,
            events: EventList::new(),
            weapons_initial,
            weapons: WeaponSet::new(),
            start_time: start,
            end_time: None,
            team: Team::Unknown,

            kill_count: 0,
            death_count: 0,
            headshot_kills: 0,
            headshot_deaths: 0,
            vehicles_destroyed: 0,
            vehicles_lost: 0,
            vehicle_kills: 0,
            vehicle_deaths: 0,

            time_zone: local_tz,

            initial_kills_total: init_kills,
            initial_actual_deaths_total: init_actual_deaths,
            initial_revived_deaths_total: init_revived_deaths,
            initial_vehicles_destroyed: init_destroyed,
            initial_shots_fired: init_shot,
            initial_shots_hit: init_hit,
            initial_headshot_kills: init_headshots,

            latest_api_kills: init_kills,
            latest_api_revived_deaths: init_revived_deaths,
            latest_api_shots_fired: init_shot,
            latest_api_shots_hit: init_hit,
            latest_api_headshots: init_headshots,

            latest_br,
            latest_asp,
            pre_asp_rankups: 0,

            dirty: false,
            db_id: None,
            db,
        };
        if Handle::try_current().is_err() {
            rt.block_on(new_session.save_to_db());
        }
        new_session
    }

    pub async fn from_db_row(row: sqlx::sqlite::SqliteRow, db: DatabaseSync) -> Self {
        let db_id = row.get(0);
        let character = FullCharacter {
            full_name: row.get(1),
            lower_name: row.get(2),
            server: row.get::<i64, usize>(3).into(),
            outfit: row.get::<Option<String>, usize>(4),
            outfit_full: row.get::<Option<String>, usize>(5),
            character_id: row.get(6),
            faction: row.get::<i64, usize>(7).into(),
            br: row.get::<u8, usize>(8),
            asp: row.get::<u8, usize>(9),
        };

        let events = db.dbc.get_events_for_session(db_id).await.unwrap();
        let weapons = db.dbc.get_weaponstats_for_session(db_id).await.unwrap();
        let mut new_session = Session {
            character,
            events,
            weapons_initial: BTreeMap::new(),
            weapons,

            start_time: row.get::<i64, usize>(10),
            end_time: row.get::<Option<i64>, usize>(11),
            team: row.get::<i64, usize>(36).into(),


            kill_count: row.get::<u32, usize>(12),
            death_count: row.get::<u32, usize>(13),
            headshot_kills: row.get::<u32, usize>(14),
            headshot_deaths: row.get::<u32, usize>(15),
            vehicles_destroyed: row.get::<u32, usize>(16),
            vehicles_lost: row.get::<u32, usize>(17),
            vehicle_kills: row.get::<u32, usize>(18),
            vehicle_deaths: row.get::<u32, usize>(19),
            time_zone: time_tz::timezones::get_by_name(row.get(20)).unwrap(),

            initial_kills_total: row.get::<i64, usize>(21) as u64,
            initial_actual_deaths_total: row.get::<i64, usize>(22) as u64,
            initial_revived_deaths_total: row.get::<i64, usize>(23) as u64,
            initial_vehicles_destroyed: row.get::<i64, usize>(24) as u64,
            initial_shots_fired: row.get::<i64, usize>(25) as u64,
            initial_shots_hit: row.get::<i64, usize>(26) as u64,
            initial_headshot_kills: row.get::<i64, usize>(27) as u64,

            latest_api_kills: row.get::<i64, usize>(28) as u64,
            latest_api_revived_deaths: row.get::<i64, usize>(29) as u64,
            latest_api_shots_fired: row.get::<i64, usize>(30) as u64,
            latest_api_shots_hit: row.get::<i64, usize>(31) as u64,
            latest_api_headshots: row.get::<i64, usize>(32) as u64,

            latest_br: row.get::<u8, usize>(33),
            latest_asp: row.get::<u8, usize>(34),
            pre_asp_rankups: row.get::<u8, usize>(35),

            db_id: Some(db_id),
            dirty: false,
            db,
        };

        //Sessions retrieved from DB are, by definition, not active. Correct the lack of end time
        //caused by quitting / crashing with an in-progress session.
        if new_session.is_active() {
            if let Some(estimated_timestamp) = new_session.events.last_event_time() {
                new_session.end_time = Some(estimated_timestamp);
            } else {
                new_session.end_time = Some(new_session.start_time);
            }
            new_session.dirty = true;
            new_session.update_db_entry().await;
        }
        new_session
    }

    pub async fn new_async(character: FullCharacter, start: i64, db: DatabaseSync) -> Self {
        let mut new_session = Session::new(character, start, db);
        new_session.save_to_db().await;
        new_session
    }

    pub fn current_character(&self) -> FullCharacter {
        self.character.clone()
    }

    pub fn current_true_kdr(&self) -> f32 {
        let kills_current = self.initial_kills_total + self.kill_count as u64;
        let deaths_current = self.initial_actual_deaths_total + self.death_count as u64;
        (kills_current as f64 / deaths_current as f64) as f32
    }

    pub fn get_list_name(&self) -> String {
        if let Some(end_time) = self.end_time {
            format!(
                "{} {}-{}",
                self.character.full_name, self.start_time, end_time
            )
        } else {
            format!("{} {}-Active", self.character.full_name, self.start_time)
        }
    }

    pub fn duration_string(&self) -> String {
        if self.end_time.is_none() {
            "Active".to_owned()
        } else {
            let start_time = OffsetDateTime::from_unix_timestamp(self.start_time)
                .unwrap_or_else(|_| OffsetDateTime::now_utc())
                .to_timezone(self.time_zone); //TODO: cleanup
            let end_time = OffsetDateTime::from_unix_timestamp(self.end_time.unwrap())
                .unwrap_or_else(|_| OffsetDateTime::now_utc())
                .to_timezone(self.time_zone); //TODO: cleanup
            let session_duration = end_time - start_time;
            let hours = session_duration.whole_hours();
            let minutes = session_duration.whole_minutes() % 60;
            format!("{:02}:{:02}", hours, minutes)
        }
    }

    pub fn local_start_date(&self) -> Date {
        OffsetDateTime::from_unix_timestamp(self.start_time)
            .unwrap_or_else(|_| OffsetDateTime::now_utc())
            .to_timezone(self.time_zone)
            .date()
    }

    fn br_with_change(&self) -> String {
        let current_rank = if self.latest_asp > 0 {
            format!("{}~{}", self.latest_br, self.latest_asp)
        } else {
            format!("{}", self.latest_br)
        };

        if self.character.br == self.latest_br && self.character.asp == self.latest_asp {
            format!("{} (+0)", current_rank)
        } else {
            //Taking ASP resets your BR to 1, may only happen during BR 100-120 the first time, and
            //then again only at BR 100~1.
            //
            //As the first ASP be taken in a range of levels we cannot infer from current and
            //initial BR how many rankups have occurred in the session where the player takes their
            //first ASP reset. Instead we must track the number of BattleRankUp events we've
            //received prior to the ASP reset in that session using self.pre_asp_rankups. However
            //it is possible for a character at low battlerank to rank up multiple times in a
            //single large XP gain; such as a continent locking with xp boosts on a double XP
            //weekend for example. As such pre_asp_rankups will STOP incrementing after the ASP
            //reset is taken (this assumes that a BattleRankUP event is triggered upon taking ASP),
            //and only use this value in the case of an ASP 0 to ASP transition.
            //
            //We can reasonably trust that the pre_asp_rankups value will be accurate in such a
            //session as it is very unlikely [though not technically impossible] for a player to
            //rank from BR 1 to 100 and then ASP in a single session.
            //
            //In all other cases we determine the change in BR directly, without relying on the
            //potentially fallible number of BattleRankUp events received.

            if self.latest_asp == 1 && self.character.asp == 0 {
                //First ASP reset happened during this session
                let total_rankups = self.pre_asp_rankups + (self.latest_br - 1);
                format!("{} (+{} [~1])", current_rank, total_rankups)
            } else if self.latest_asp > self.character.asp {
                //2nd or later ASP reset happened during this session
                format!(
                    "{} (+{} [~{}])",
                    current_rank,
                    (self.latest_br - 1) + (100 - self.character.br),
                    self.latest_asp - self.character.asp
                )
            } else {
                //No ASP reset this session
                format!("{} (+{})", current_rank, self.latest_br - self.character.br)
            }
        }
    }

    pub fn log_rankup(&mut self, new_br: u8, new_asp: u8) {
        self.latest_br = new_br;
        self.latest_asp = new_asp;
        if new_asp == 0 {
            self.pre_asp_rankups += 1;
        }
    }

    pub async fn log_event(&mut self, event: Event) -> u32 {
        match event.kind {
            EventType::Death | EventType::TeamDeath | EventType::Suicide => {
                self.death_count += 1;
                if let Some(vehicle) = event.vehicle {
                    if vehicle.is_true_vehicle() {
                        self.vehicle_deaths += 1;
                    }
                }
                if event.headshot {
                    self.headshot_deaths += 1;
                }
            }
            EventType::Kill => {
                self.kill_count += 1;
                if let Some(vehicle) = event.vehicle {
                    if vehicle.is_true_vehicle() {
                        self.vehicle_kills += 1;
                    }
                }
                if event.headshot {
                    self.headshot_kills += 1;
                }

                //Update weapon stats.
                if event.weapon_id != "0" {
                    //skip suicides
                    if self.weapons.contains(&event.weapon_id) {
                        self.weapons.add_kill(&event.weapon_id, event.headshot);
                    } else {
                        let initial = match self.weapons_initial.remove(&event.weapon_id) {
                            Some(retrieved) => retrieved,
                            None => WeaponInitial::new(),
                        };

                        let mut new_stat = WeaponStats::new(
                            &event.weapon,
                            &event.weapon_id,
                            initial,
                            event.weapon_kind,
                        );
                        new_stat.add_kill(event.headshot);
                        new_stat
                            .save_to_db(self.weapons.len(), self.db_id.unwrap(), &self.db.dbc)
                            .await;
                        self.weapons.push(new_stat);
                    }
                }
            }
            EventType::DestroyVehicle => {
                //Update weapon stats also on vehicle destroys, without upping kill count.
                if event.weapon_id != "0" {
                    //skip suicides
                    if !self.weapons.contains(&event.weapon_id) {
                        let initial = match self.weapons_initial.remove(&event.weapon_id) {
                            Some(retrieved) => retrieved,
                            None => WeaponInitial::new(),
                        };

                        let new_stat = WeaponStats::new(
                            &event.weapon,
                            &event.weapon_id,
                            initial,
                            event.weapon_kind,
                        );
                        new_stat
                            .save_to_db(self.weapons.len(), self.db_id.unwrap(), &self.db.dbc)
                            .await;
                        self.weapons.push(new_stat);
                    }
                }

                self.vehicles_destroyed += 1
            }
            EventType::LoseVehicle => self.vehicles_lost += 1,
            EventType::LoseVehicleFF => self.vehicles_lost += 1,
            _ => {}
        };
        self.events.push(event);
        self.dirty = true;
        self.events.len()
    }

    pub fn get_id(&self) -> Option<i64> {
        self.db_id
    }

    pub fn is_active(&self) -> bool {
        self.end_time.is_none()
    }

    pub fn update_filters(&mut self, event_mode: EventViewMode, filter: Option<String>) {
        self.events.update_filters(event_mode, filter);
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        self.events.ui(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel is the region left after adding TopPanel's and SidePanel's
            //ui.heading(format!("{} Stats", new_char_name));
            let formatter =
                time::format_description::parse("[hour repr:12]:[minute]:[second] [period]")
                    .unwrap();
            let start_time = OffsetDateTime::from_unix_timestamp(self.start_time)
                .unwrap_or_else(|_| OffsetDateTime::now_utc())
                .to_timezone(self.time_zone); //TODO: cleanup
            let formatted_start_time = start_time
                .format(&formatter)
                .unwrap_or_else(|_| "?-?-? ?:?:?".into());

            ui.horizontal(|ui| {
                match ui
                    .ctx()
                    .texture_by_name(&self.character.faction.to_string())
                {
                    Some(image) => ui.image(image.id(), (28.0, 28.0)),
                    None => ui.label(self.character.faction.to_string()),
                };

                ui.heading(self.character.name_with_outfit());

                if let Some(end_time_i) = self.end_time {
                    let end_time = OffsetDateTime::from_unix_timestamp(end_time_i)
                        .unwrap_or_else(|_| OffsetDateTime::now_utc())
                        .to_timezone(self.time_zone);
                    let formatted_end_time = end_time
                        .format(&formatter)
                        .unwrap_or_else(|_| "?-?-? ?:?:?".into());

                    ui.label(format!(
                        "  {} - {}",
                        formatted_start_time, formatted_end_time
                    ));
                } else {
                    let now_time = OffsetDateTime::now_utc();
                    let session_duration = now_time - start_time;
                    let hours = session_duration.whole_hours();
                    let minutes = session_duration.whole_minutes() % 60;
                    let seconds = session_duration.whole_seconds() % 60;
                    let millis = session_duration.subsec_milliseconds() / 10;
                    ui.label(format!(
                        "  {},  {:02}:{:02}:{:02}.{:02}",
                        formatted_start_time, hours, minutes, seconds, millis
                    ));
                }
            });

            ui.separator();
            egui::Grid::new("session_stats")
                .min_col_width(10.0)
                .show(ui, |ui| {
                    ui.label("Session:");
                    ui.end_row();
                    ui.label(format!("Kills {}", self.kill_count));
                    ui.label(format!("by HS {}", self.headshot_kills));
                    ui.label(format!("Vehicle kills {}", self.vehicle_kills));
                    ui.end_row();
                    ui.label(format!("Deaths {}", self.death_count));
                    ui.label(format!("by HS {}", self.headshot_deaths));
                    ui.label(format!("Vehicle deaths {}", self.vehicle_deaths));
                    ui.end_row();
                    ui.label(format!("Vehicles destroyed {}", self.vehicles_destroyed));
                    ui.label(format!("Vehicles lost {}", self.vehicles_lost));
                    ui.label(format!("BR {}", self.br_with_change()));
                    ui.end_row();
                    if self.death_count > 0 {
                        ui.label(format!(
                            "KDR {:.5}",
                            self.kill_count as f32 / self.death_count as f32
                        ));
                    } else {
                        ui.label("KDR -");
                    }
                    if self.kill_count > 0 {
                        ui.label(format!(
                            "HSR {:.3}%",
                            (self.headshot_kills as f32 / self.kill_count as f32) * 100.0
                        ));
                    } else {
                        ui.label("HSR -");
                    }
                    //accuracy needed
                    ui.label(format!(
                        "Accuracy {:.3}%",
                        self.weapons.aggregate_accuracy()
                    ));
                });

            ui.separator();
            egui::Grid::new("lifetime_stats")
                .min_col_width(10.0)
                .show(ui, |ui| {
                    ui.label("Lifetime:");
                    ui.end_row();
                    let kills_current = self.initial_kills_total + self.kill_count as u64;
                    let deaths_current = self.initial_actual_deaths_total + self.death_count as u64;
                    let headshots_current =
                        self.initial_headshot_kills + self.headshot_kills as u64;

                    ui.label(format!("Kills {}", kills_current));
                    ui.label(format!(
                        "Vehicles destroyed {}",
                        self.initial_vehicles_destroyed + self.vehicles_destroyed as u64
                    ));
                    ui.end_row();
                    ui.label(format!("Deaths (true) {}", deaths_current));
                    if self.initial_actual_deaths_total > 0 {
                        let current_kdr = kills_current as f64 / deaths_current as f64;
                        let init_kdr = if self.initial_actual_deaths_total > 0 {
                            self.initial_kills_total as f64
                                / self.initial_actual_deaths_total as f64
                        } else {
                            0.0
                        };

                        ui.label(format!(
                            "KDR (true) {:.5} ({:+.5})",
                            current_kdr,
                            current_kdr - init_kdr
                        ));
                    } else {
                        ui.label("KDR (true) -");
                    }
                    ui.end_row();
                    ui.label(format!(
                        "Deaths (rezzed) {}",
                        self.latest_api_revived_deaths
                    ));
                    if self.initial_revived_deaths_total > 0 {
                        let current_r_kdr =
                            self.latest_api_kills as f64 / self.latest_api_revived_deaths as f64;
                        let init_r_kdr = if self.initial_revived_deaths_total > 0 {
                            self.initial_kills_total as f64
                                / self.initial_revived_deaths_total as f64
                        } else {
                            0.0
                        };
                        ui.label(format!(
                            "KDR (rezzed) {:.5} ({:+.5})",
                            current_r_kdr,
                            current_r_kdr - init_r_kdr
                        ));
                    } else {
                        ui.label("KDR (rezzed) -");
                    }
                    ui.end_row();
                    if kills_current > 0 {
                        let current_hsr = (headshots_current as f64 / kills_current as f64) * 100.0;
                        let init_hsr = if self.initial_kills_total > 0 {
                            (self.initial_headshot_kills as f64 / self.initial_kills_total as f64)
                                * 100.0
                        } else {
                            0.0
                        };
                        ui.label(format!(
                            "HSR {:.3}% ({:+.3})",
                            current_hsr,
                            current_hsr - init_hsr
                        ));
                    } else {
                        ui.label("HSR -");
                    }
                    if self.initial_shots_fired > 0 {
                        let current_acc = (self.latest_api_shots_hit as f64
                            / self.latest_api_shots_fired as f64)
                            * 100.0;
                        let init_acc = (self.initial_shots_hit as f64
                            / self.initial_shots_fired as f64)
                            * 100.0;
                        ui.label(format!(
                            "Acc {:.3}% ({:+.3})",
                            current_acc,
                            current_acc - init_acc
                        ));
                    } else {
                        ui.label("Acc -");
                    }
                });

            TableBuilder::new(ui)
                .column(Size::remainder()) //weapon name //formerly minimum 100
                .column(Size::exact(25.0)) //kills
                .column(Size::exact(80.0)) //HS%
                .column(Size::exact(80.0)) //Acc
                .column(Size::exact(25.0)) //HS count
                .column(Size::exact(25.0)) //Fired
                .column(Size::exact(25.0)) //Hits
                .header(12.0, |mut header| {
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Name").small());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Kills").small());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("HS%").small());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Accuracy").small());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("HS").small());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Fired").small());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Hits").small());
                    });
                })
                .body(|mut body| {
                    for weapon in self.weapons.iter().rev() {
                        weapon.ui(&mut body);
                    }
                });
        });
    }

    pub async fn update_historical_stats(&mut self) {
        //println!("At historical update, session end_time is {:?}",self.end_time);
        if self.end_time.is_none() {
            match lookup_full_stats(&self.character.character_id) {
                Err(whut) => println!("Failed getting lifetime stats data:\n{}", whut),
                Ok(details) => {
                    let stat_history =
                        &details["single_character_by_id_list"][0]["stats"]["stat_history"];
                    let stat_block = details["single_character_by_id_list"][0]["stats"]["stat"]
                        .as_array()
                        .unwrap(); //TODO: more care to be taken
                    let weapon_stat = details["single_character_by_id_list"][0]["stats"]
                        ["weapon_stat"]
                        .as_array()
                        .unwrap();
                    let weapon_stat_by_faction = details["single_character_by_id_list"][0]["stats"]
                        ["weapon_stat_by_faction"]
                        .as_array()
                        .unwrap();

                    self.latest_api_kills = stat_history["kills"]["all_time"]
                        .to_string()
                        .unquote()
                        .parse::<u64>()
                        .unwrap();
                    println!("Updated lifetime kills: {}", self.latest_api_kills);

                    self.latest_api_revived_deaths = stat_history["deaths"]["all_time"]
                        .to_string()
                        .unquote()
                        .parse::<u64>()
                        .unwrap();
                    println!(
                        "Updated lifetime deaths - revives: {}",
                        self.latest_api_revived_deaths
                    );

                    for stat in stat_block {
                        match stat["stat_name"].as_str() {
                            Some("weapon_hit_count") => {
                                self.latest_api_shots_hit = stat["value_forever"]
                                    .to_string()
                                    .unquote()
                                    .parse::<u64>()
                                    .unwrap();
                            }
                            Some("weapon_fire_count") => {
                                self.latest_api_shots_fired = stat["value_forever"]
                                    .to_string()
                                    .unquote()
                                    .parse::<u64>()
                                    .unwrap();
                            }
                            _ => {}
                        }
                    }
                    println!("Updated lifetime fired: {}", self.latest_api_shots_fired);
                    println!("Updated lifetime hit: {}", self.latest_api_shots_hit);

                    let mut vs_hs = 0;
                    let mut nc_hs = 0;
                    let mut tr_hs = 0;

                    for stat in weapon_stat {
                        let weapon_id = stat["item_id"].as_str().unwrap().to_owned();
                        if weapon_id == "0" {
                            //skip vehicles? does recursion track roadkills in per weapon stats?
                            continue;
                        }

                        if self.weapons.contains(&weapon_id) {
                            match stat["stat_name"].as_str() {
                                Some("weapon_hit_count") => {
                                    let hits =
                                        stat["value"].to_string().unquote().parse::<u64>().unwrap();
                                    self.weapons.accumulate_latest_hits(&weapon_id, hits);
                                }
                                Some("weapon_fire_count") => {
                                    let fired =
                                        stat["value"].to_string().unquote().parse::<u64>().unwrap();
                                    self.weapons.accumulate_latest_fired(&weapon_id, fired);
                                }
                                Some(_) | None => {}
                            }
                        }
                    }

                    self.weapons.update_from_accumulators();
                    self.weapons
                        .update_db_entries(&self.db.dbc, self.db_id.unwrap())
                        .await;

                    for stat in weapon_stat_by_faction {
                        if let Some("weapon_headshots") = stat["stat_name"].as_str() {
                            vs_hs += stat["value_vs"]
                                .to_string()
                                .unquote()
                                .parse::<u64>()
                                .unwrap();
                            nc_hs += stat["value_nc"]
                                .to_string()
                                .unquote()
                                .parse::<u64>()
                                .unwrap();
                            tr_hs += stat["value_tr"]
                                .to_string()
                                .unquote()
                                .parse::<u64>()
                                .unwrap();
                        }
                    }

                    self.latest_api_headshots = vs_hs + nc_hs + tr_hs;
                    println!(
                        "Updated headshots: VS {}, NC {}, TR {}, Total: {}",
                        vs_hs, nc_hs, tr_hs, self.latest_api_headshots
                    );

                    self.update_db_entry().await;
                }
            }
        } else {
            println!("Session Not active when wanted to update latest stats from Census API. Ignoring timer trigger.");
        }
    }

    pub fn needs_db_update(&mut self) -> bool {
        if self.dirty {
            self.dirty = false;
            true
        } else {
            false
        }
    }

    pub async fn save_to_db(&mut self) {
        match sqlx::query("INSERT INTO sessions VALUES (NULL,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?) returning id;")
            .bind(&self.character.full_name)
            .bind(&self.character.lower_name)
            .bind(self.character.server as i64)
            .bind(&self.character.outfit)
            .bind(&self.character.outfit_full)
            .bind(&self.character.character_id)
            .bind(self.character.faction as i64)
            .bind(self.character.br as i64)
            .bind(self.character.asp as i64)

            .bind(self.start_time as i64)
            .bind(self.end_time)

            .bind(self.kill_count as i64)
            .bind(self.death_count as i64)
            .bind(self.headshot_kills as i64)
            .bind(self.headshot_deaths as i64)
            .bind(self.vehicles_destroyed as i64)
            .bind(self.vehicles_lost as i64)
            .bind(self.vehicle_kills as i64)
            .bind(self.vehicle_deaths as i64)
            .bind(&self.time_zone.name())

            .bind(self.initial_kills_total as i64)
            .bind(self.initial_actual_deaths_total as i64)
            .bind(self.initial_revived_deaths_total as i64)
            .bind(self.initial_vehicles_destroyed as i64)
            .bind(self.initial_shots_fired as i64)
            .bind(self.initial_shots_hit as i64)
            .bind(self.initial_headshot_kills as i64)

            .bind(self.latest_api_kills as i64)
            .bind(self.latest_api_revived_deaths as i64)
            .bind(self.latest_api_shots_fired as i64)
            .bind(self.latest_api_shots_hit  as i64)
            .bind(self.latest_api_headshots as i64)

            .bind(self.latest_br as i64)
            .bind(self.latest_asp as i64)
            .bind(self.pre_asp_rankups as i64)
            .bind(self.team as i64)
            .fetch_one(&self.db.dbc.conn)
            .await
        {
            Ok(row) => {
                self.db_id = Some(row.get(0));
                println!("New session has ID: {}", self.db_id.unwrap());
            }
            Err(err) => {
                if let Some(db_err) = err.as_database_error() {
                    println!("Error saving new sessionin DB:");
                    println!("{:?}", db_err);
                    std::process::exit(-20);
                }
            }
        }
    }

    pub async fn update_db_entry(&mut self) {
        if self.dirty {
            match sqlx::query("UPDATE sessions set end_time = ?, kill_count = ?, death_count = ?, headshot_kills = ?, headshot_deaths = ?,
                               vehicles_destroyed = ?, vehicles_lost = ?, vehicle_kills = ?, vehicle_deaths = ?, la_kills = ?,
                               la_revived_deaths = ?, la_fired = ?, la_shots = ?, la_headshots = ?, l_br = ?, l_asp = ?, pa_rankups = ?,
                               team = ?
                               WHERE id IS ?;")
                .bind(self.end_time)

                .bind(self.kill_count as i64)
                .bind(self.death_count as i64)
                .bind(self.headshot_kills as i64)
                .bind(self.headshot_deaths as i64)
                .bind(self.vehicles_destroyed as i64)
                .bind(self.vehicles_lost as i64)
                .bind(self.vehicle_kills as i64)
                .bind(self.vehicle_deaths as i64)

                .bind(self.latest_api_kills as i64)
                .bind(self.latest_api_revived_deaths as i64)
                .bind(self.latest_api_shots_fired as i64)
                .bind(self.latest_api_shots_hit  as i64)
                .bind(self.latest_api_headshots as i64)

                .bind(self.latest_br as i64)
                .bind(self.latest_asp as i64)
                .bind(self.pre_asp_rankups as i64)
                .bind(self.team as i64)
                .bind(self.db_id)
                .execute(&self.db.dbc.conn)
                .await
            {
                Ok(_) => {
                }
                Err(err) => {
                    if let Some(db_err) = err.as_database_error() {
                        println!("Error updating new sessionin DB:");
                        println!("{:?}", db_err);
                        std::process::exit(-21);
                    }
                }
            }
            self.dirty = false;
        }

        self.weapons
            .update_db_entries(&self.db.dbc, self.db_id.unwrap())
            .await;
    }

    pub async fn end(&mut self, time: i64) {
        self.end_time = Some(time);
        self.dirty = true;
        self.update_db_entry().await;
    }
}
