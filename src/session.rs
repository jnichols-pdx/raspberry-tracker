#![allow(unused_variables)]
use tokio_tungstenite::tungstenite::protocol::Message;


use tokio::sync::{mpsc};
use crate::common::*;
use crate::events::{*, Event};
use crate::character::*;
use crate::weapons::*;
use eframe::{egui, epi};
use egui::*;
use egui_extras::{TableBuilder, Size};
use time::OffsetDateTime;
use time_tz::{OffsetDateTimeExt,Tz};
use std::collections::BTreeMap;

#[allow(dead_code)]
pub struct Session {
    character: FullCharacter,
    events: EventList,
    weapons_initial: BTreeMap<String, WeaponInitial>,
    weapons: WeaponSet,

    start_time: i64,
    end_time: Option<i64>,

    kill_count: u32,
    death_count: u32,
    headshot_kills: u32,
    headshot_deaths: u32,
    vehicles_destroyed: u32,
    vehicles_lost: u32,
    vehicle_kills: u32, //Killed someone using a vehicle
    vehicle_deaths: u32, //killed by someone else in a vehicle
    //accuracy: f32,
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
}

#[allow(dead_code)]
impl Session {
    pub fn match_player_id(&self, to_match: &String) -> bool {
        to_match.eq(&self.character.character_id)
    }

    pub fn get_player_name(&self) -> String {
        self.character.full_name.to_owned()
    }

    pub fn new(character: Character, br: u8, asp: u8, start: i64) -> Self {
        let local_tz_q = time_tz::system::get_timezone();
        let local_tz;
        match local_tz_q {
            Ok(local) => local_tz = local,
            Err(e) => {println!("Error finding system timezone: {}", e);
                            std::process::exit(-2);
            },
        }
        let character = FullCharacter::new(&character, br, asp);
        Session {
            character: character,
            events: EventList::new(),
            weapons_initial: BTreeMap::new(),
            weapons: WeaponSet::new(),
            start_time: start,
            end_time: None,

            kill_count: 0,
            death_count: 0,
            headshot_kills: 0,
            headshot_deaths: 0,
            vehicles_destroyed: 0,
            vehicles_lost: 0,
            vehicle_kills: 0,
            vehicle_deaths: 0,

            time_zone: local_tz,

            initial_kills_total: 0,
            initial_actual_deaths_total: 0,
            initial_revived_deaths_total: 0,
            initial_vehicles_destroyed: 0,
            initial_shots_fired: 0,
            initial_shots_hit: 0,
            initial_headshot_kills: 0,

            latest_api_kills: 0,
            latest_api_revived_deaths: 0,
            latest_api_shots_fired: 0,
            latest_api_shots_hit: 0,
            latest_api_headshots: 0,
        }
    }

    pub fn new_from_full(character: FullCharacter, start: i64) -> Self {
        let local_tz_q = time_tz::system::get_timezone();
        let local_tz;
        match local_tz_q {
            Ok(local) => local_tz = local,
            Err(e) => {println!("Error finding system timezone: {}", e);
                            std::process::exit(-2);
            },
        }


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
                let stat_history = &details["single_character_by_id_list"][0]["stats"]["stat_history"];
                let stat_block = details["single_character_by_id_list"][0]["stats"]["stat"].as_array().unwrap(); //TODO: more care to be taken
                //let stat_by_faction = &details["single_character_by_id_list"][0]["stats"]["stat_by_faction"];
                let weapon_stat = details["single_character_by_id_list"][0]["stats"]["weapon_stat"].as_array().unwrap();
                let weapon_stat_by_faction = details["single_character_by_id_list"][0]["stats"]["weapon_stat_by_faction"].as_array().unwrap();

                init_kills = stat_history["kills"]["all_time"].to_string().unquote().parse::<u64>().unwrap();
                println!("Found lifetime kills: {}",init_kills);

                init_revived_deaths = stat_history["deaths"]["all_time"].to_string().unquote().parse::<u64>().unwrap();
                println!("Found lifetime deaths - revives: {}",init_revived_deaths);

                for stat in stat_block {
                    match stat["stat_name"].as_str() {
                       Some("weapon_hit_count") => {
                           init_hit = stat["value_forever"].to_string().unquote().parse::<u64>().unwrap();
                           },
                       Some("weapon_fire_count") => {
                           init_shot = stat["value_forever"].to_string().unquote().parse::<u64>().unwrap();
                           },
                       Some("weapon_deaths") => {
                           init_actual_deaths = stat["value_forever"].to_string().unquote().parse::<u64>().unwrap();
                           },
                       _ => {},
                    }
                }
                println!("Found lifetime deaths: {}",init_actual_deaths);
                println!("Found lifetime fired: {}",init_shot);
                println!("Found lifetime hit: {}",init_hit);

                for stat in weapon_stat {
                    let weapon_id = stat["item_id"].as_str().unwrap().to_owned();
                    if weapon_id == "0" {  //skip vehicles? does recursion track roadkills in per weapon stats?
                        continue
                    }

                    if ! weapons_initial.contains_key(&weapon_id) {
                        let wi = WeaponInitial::new();
                        weapons_initial.insert(weapon_id.clone(), wi);
                    }
                    if let Some(ws) = weapons_initial.get_mut(&weapon_id) {
                        match stat["stat_name"].as_str() {
                            Some("weapon_hit_count") => {
                                ws.hits = stat["value"].to_string().unquote().parse::<u64>().unwrap();
                            },
                            Some("weapon_fire_count") => {
                                ws.fired = stat["value"].to_string().unquote().parse::<u64>().unwrap();
                            },
                            Some(_) | None => {},
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
                            let vs_val = stat["value_vs"].to_string().unquote().parse::<u64>().unwrap();
                            let nc_val = stat["value_nc"].to_string().unquote().parse::<u64>().unwrap();
                            let tr_val = stat["value_tr"].to_string().unquote().parse::<u64>().unwrap();

                            vs_hs += vs_val;
                            nc_hs += nc_val;
                            tr_hs += tr_val;


                            let weapon_id = stat["item_id"].as_str().unwrap().to_owned();
                            if weapon_id == "0" {  //skip vehicles? does recursion track roadkills in per weapon stats?
                                continue
                            }
                            if ! weapons_initial.contains_key(&weapon_id) {
                                let wi = WeaponInitial::new();
                                weapons_initial.insert(weapon_id.clone(), wi);
                            }
                            if let Some(ws) = weapons_initial.get_mut(&weapon_id) {
                                ws.headshots += vs_val + nc_val + tr_val;
                            }
                        },
                        Some("weapon_vehicle_kills") => {
                            vs_veh_destroy += stat["value_vs"].to_string().unquote().parse::<u64>().unwrap();
                            nc_veh_destroy += stat["value_nc"].to_string().unquote().parse::<u64>().unwrap();
                            tr_veh_destroy += stat["value_tr"].to_string().unquote().parse::<u64>().unwrap();
                        },
                        Some("weapon_kills") => {
                            let weapon_id = stat["item_id"].as_str().unwrap().to_owned();
                            if weapon_id == "0" {  //skip vehicles? does recursion track roadkills in per weapon stats?
                                continue
                            }
                            if ! weapons_initial.contains_key(&weapon_id) {
                                let wi = WeaponInitial::new();
                                weapons_initial.insert(weapon_id.clone(), wi);
                            }
                            if let Some(ws) = weapons_initial.get_mut(&weapon_id) {
                                ws.kills += stat["value_vs"].to_string().unquote().parse::<u64>().unwrap();
                                ws.kills += stat["value_nc"].to_string().unquote().parse::<u64>().unwrap();
                                ws.kills += stat["value_tr"].to_string().unquote().parse::<u64>().unwrap();
                            }
                        },
                        _ => {},
                    }
                }

                init_headshots =  vs_hs + nc_hs + tr_hs;
                println!("headshots: VS {}, NC {}, TR {}, Total: {}", vs_hs, nc_hs, tr_hs, init_headshots);
                init_destroyed =  vs_veh_destroy + nc_veh_destroy + tr_veh_destroy;
                println!("vehicle destroys : VS {}, NC {}, TR {}, Total: {}", vs_veh_destroy, nc_veh_destroy, tr_veh_destroy, init_destroyed);

            }
        }

        Session {
            character: character,
            events: EventList::new(),
            weapons_initial,
            weapons:WeaponSet::new(),
            start_time: start,
            end_time: None,

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
        }
    }

    pub fn current_character(&self) -> FullCharacter {
        self.character.clone()
    }

    pub fn get_list_name(&self) -> String {
        if let Some(end_time) = self.end_time {
            format!("{} {}-{}", self.character.full_name, self.start_time, end_time )
        } else {
            format!("{} {}-Active", self.character.full_name, self.start_time)
        }
    }

    pub fn log_event(&mut self, event: Event) {
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
            },
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
                if event.weapon_id != "0" { //skip suicides
                    if self.weapons.contains(&event.weapon_id) {
                       self.weapons.add_kill(&event.weapon_id, event.headshot);
                    } else {
                        let initial;
                        match self.weapons_initial.remove(&event.weapon_id) {
                            Some(retrieved) => initial = retrieved,
                            None => initial = WeaponInitial::new(),
                        }

                        let mut new_stat = WeaponStats::new(&event.weapon, &event.weapon_id, initial);
                        new_stat.add_kill(event.headshot);
                        self.weapons.push(new_stat);
                    }
                }
            },
            EventType::DestroyVehicle => {
                //Update weapon stats also on vehicle destroys, without upping kill count.
                if event.weapon_id != "0" { //skip suicides
                    if ! self.weapons.contains(&event.weapon_id){
                        let initial;
                        match self.weapons_initial.remove(&event.weapon_id) {
                            Some(retrieved) => initial = retrieved,
                            None => initial = WeaponInitial::new(),
                        }

                        let new_stat = WeaponStats::new(&event.weapon, &event.weapon_id, initial);
                        self.weapons.push(new_stat);
                    }
                }

                self.vehicles_destroyed += 1
            },
            EventType::LoseVehicle => { self.vehicles_lost += 1},
            EventType::LoseVehicleFF => { self.vehicles_lost += 1},
            _ => {},
        };
        self.events.push(event);
    }

    pub fn ui(&self, ctx: &egui::Context) {
        self.events.ui(&ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel is the region left after adding TopPanel's and SidePanel's
            //ui.heading(format!("{} Stats", new_char_name));
            let formatter = time::format_description::parse("[hour repr:12]:[minute]:[second] [period]",).unwrap();
            let start_time = OffsetDateTime::from_unix_timestamp(self.start_time).unwrap_or_else(|_| {OffsetDateTime::now_utc()}).to_timezone(self.time_zone); //TODO: cleanup
            let formatted_start_time;
            if let Ok(tstamp) = start_time.format(&formatter) {
                formatted_start_time = tstamp;
            } else {
                formatted_start_time = "?-?-? ?:?:?".to_owned();
            }


            ui.horizontal(|ui| {
                ui.heading(format!("{}", self.character.full_name));

                if let Some(end_time_i) = self.end_time {
                    let end_time= OffsetDateTime::from_unix_timestamp(end_time_i).unwrap_or_else(|_| {OffsetDateTime::now_utc()}).to_timezone(self.time_zone); //TODO: cleanup
                    let formatted_end_time;
                    if let Ok(tstamp) = end_time.format(&formatter) {
                        formatted_end_time= tstamp;
                    } else {
                        formatted_end_time = "?-?-? ?:?:?".to_owned();
                    }

                    ui.label(format!("  {} - {}", formatted_start_time, formatted_end_time ));
                } else {
                    let now_time = OffsetDateTime::now_utc();
                    let session_duration = now_time - start_time;
                    let hours = session_duration.whole_hours();
                    let minutes = session_duration.whole_minutes() % 60;
                    let seconds = session_duration.whole_seconds() % 60;
                    let millis = session_duration.subsec_milliseconds() /10;
                    ui.label(format!("  {},  {:02}:{:02}:{:02}.{:02}",
                        formatted_start_time,
                        hours, minutes, seconds, millis
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
                    ui.end_row();
                    if self.death_count > 0 {
                        ui.label(format!("KDR {:.3}", self.kill_count as f32 / self.death_count as f32));
                    } else {
                        ui.label("KDR -");
                    }
                    if self.kill_count > 0 {
                        ui.label(format!("HSR {:.3}%", (self.headshot_kills as f32 / self.kill_count as f32) * 100.0 ));
                    } else {
                        ui.label("HSR -");
                    }
                    //accuracy needed
                    ui.label(format!("Accuracy {:.3}%", 0.0));

            });

            ui.separator();
            egui::Grid::new("lifetime_stats")
                .min_col_width(10.0)
                .show(ui, |ui| {
                    ui.label("Lifetime:");
                    ui.end_row();
                    let kills_current = self.initial_kills_total + self.kill_count as u64;
                    let deaths_current = self.initial_actual_deaths_total + self.death_count as u64;
                    let headshots_current = self.initial_headshot_kills + self.headshot_kills as u64;

                    ui.label(format!("Kills {}", kills_current));
                    ui.label(format!("Vehicles destroyed {}", self.initial_vehicles_destroyed +  self.vehicles_destroyed as u64));
                    ui.end_row();
                    ui.label(format!("Deaths (true) {}", deaths_current));
                    if self.initial_actual_deaths_total> 0 {

                        let current_kdr = kills_current as f64 / deaths_current as f64;
                        let init_kdr = if self.initial_actual_deaths_total > 0 {
                            self.initial_kills_total as f64 / self.initial_actual_deaths_total as f64 
                        } else { 0.0 };

                        ui.label(format!("KDR (true) {:.3} ({:+.3})", current_kdr, current_kdr - init_kdr));

                    } else {
                        ui.label("KDR (true) -");
                    }
                    ui.end_row();
                    ui.label(format!("Deaths (rezzed) {}", self.latest_api_revived_deaths));
                    if self.initial_revived_deaths_total> 0 {
                        let current_kdr = self.latest_api_kills as f64 / self.latest_api_revived_deaths as f64;
                        let init_kdr = if self.initial_revived_deaths_total > 0 {
                                self.initial_kills_total as f64 / self.initial_revived_deaths_total as f64
                        } else { 0.0 };
                        ui.label(format!("KDR (rezzed) {:.3} ({:+.3})", current_kdr, current_kdr - init_kdr));
                    } else {
                        ui.label("KDR (rezzed) -");
                    }
                    ui.end_row();
                    if kills_current > 0 {
                        let current_hsr =  (headshots_current as f64 / kills_current as f64) * 100.0;
                        let init_hsr = if self.initial_kills_total > 0 { 
                            (self.initial_headshot_kills as f64 / self.initial_kills_total as f64) * 100.0
                        } else {0.0};
                        ui.label(format!("HSR {:.3}% ({:+.3})", current_hsr, current_hsr - init_hsr));
                    } else {
                        ui.label("HSR -");
                    }
                    if self.initial_shots_fired> 0 {
                        let current_acc = (self.latest_api_shots_hit as f64 / self.latest_api_shots_fired as f64) * 100.0;
                        let init_acc = (self.initial_shots_hit as f64 / self.initial_shots_fired as f64) * 100.0;
                        ui.label(format!("Acc {:.3}% ({:+.3})", current_acc, current_acc - init_acc));
                    } else {
                        ui.label("Acc -");
                    }

            });

            TableBuilder::new(ui)
                .column(Size::RemainderMinimum(100.0)) //weapon name
                .column(Size::Absolute(25.0)) //kills
                .column(Size::Absolute(80.0)) //HS%
                .column(Size::Absolute(80.0)) //Acc
                .column(Size::Absolute(25.0)) //HS count
                .column(Size::Absolute(25.0)) //Fired
                .column(Size::Absolute(25.0)) //Hits
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
                    header.col_clip(|ui| {
                        ui.label(egui::RichText::new("Accuracy").small());
                    });
                    header.col_clip(|ui| {
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

    pub fn update_historical_stats(&mut self) {
        //println!("At historical update, session end_time is {:?}",self.end_time);
        if self.end_time.is_none() {
            match lookup_full_stats(&self.character.character_id) {
                Err(whut) => println!("Failed getting lifetime stats data:\n{}", whut),
                Ok(details) => {
                    let stat_history = &details["single_character_by_id_list"][0]["stats"]["stat_history"];
                    let stat_block = details["single_character_by_id_list"][0]["stats"]["stat"].as_array().unwrap(); //TODO: more care to be taken
                    let weapon_stat = details["single_character_by_id_list"][0]["stats"]["weapon_stat"].as_array().unwrap();
                    let weapon_stat_by_faction = details["single_character_by_id_list"][0]["stats"]["weapon_stat_by_faction"].as_array().unwrap();

                    self.latest_api_kills = stat_history["kills"]["all_time"].to_string().unquote().parse::<u64>().unwrap();
                    println!("Updated lifetime kills: {}",self.latest_api_kills);

                    self.latest_api_revived_deaths = stat_history["deaths"]["all_time"].to_string().unquote().parse::<u64>().unwrap();
                    println!("Updated lifetime deaths - revives: {}",self.latest_api_revived_deaths);

                    for stat in stat_block {
                        match stat["stat_name"].as_str() {
                           Some("weapon_hit_count") => {
                               self.latest_api_shots_hit= stat["value_forever"].to_string().unquote().parse::<u64>().unwrap();
                               },
                           Some("weapon_fire_count") => {
                               self.latest_api_shots_fired = stat["value_forever"].to_string().unquote().parse::<u64>().unwrap();
                               },
                           _ => {},
                        }
                    }
                    println!("Updated lifetime fired: {}", self.latest_api_shots_fired);
                    println!("Updated lifetime hit: {}",self.latest_api_shots_hit);

                    let mut vs_hs = 0;
                    let mut nc_hs = 0;
                    let mut tr_hs = 0;

                    for stat in weapon_stat {
                        let weapon_id = stat["item_id"].as_str().unwrap().to_owned();
                        if weapon_id == "0" {  //skip vehicles? does recursion track roadkills in per weapon stats?
                            continue
                        }

                        if self.weapons.contains(&weapon_id) {
                            match stat["stat_name"].as_str() {
                                Some("weapon_hit_count") => {
                                    let hits = stat["value"].to_string().unquote().parse::<u64>().unwrap();
                                    self.weapons.update_latest_hits(&weapon_id, hits);
                                },
                                Some("weapon_fire_count") => {
                                    let fired = stat["value"].to_string().unquote().parse::<u64>().unwrap();
                                    self.weapons.update_latest_fired(&weapon_id, fired);
                                },
                                Some(_) | None => {},
                            }
                        }
                    }


                    for stat in weapon_stat_by_faction {
                        match stat["stat_name"].as_str() {
                            Some("weapon_headshots") => {
                                vs_hs += stat["value_vs"].to_string().unquote().parse::<u64>().unwrap();
                                nc_hs += stat["value_nc"].to_string().unquote().parse::<u64>().unwrap();
                                tr_hs += stat["value_tr"].to_string().unquote().parse::<u64>().unwrap();
                            },
                            _ => {},
                        }
                    }

                    self.latest_api_headshots =  vs_hs + nc_hs + tr_hs;
                    println!("Updated headshots: VS {}, NC {}, TR {}, Total: {}", vs_hs, nc_hs, tr_hs, self.latest_api_headshots);

                }
            }
        } else {
            println!("Session Not active when wanted to update latest stats from Census API. Ignoring timer trigger.");
        }

    }
    
    pub fn end(&mut self, time: i64)
    {
        self.end_time = Some(time);
    }

}
