#![allow(unused_variables)]
use tokio_tungstenite::tungstenite::protocol::Message;


use tokio::sync::{mpsc};
use crate::common::*;
use crate::events::{*, Event};
use crate::character::*;
use eframe::{egui, epi};
use egui::*;
use egui_extras::{TableBuilder, Size};
use time::OffsetDateTime;
use time_tz::{OffsetDateTimeExt,Tz};

#[allow(dead_code)]
pub struct Session {
    character: FullCharacter,
    events: EventList,
    weapons: WeaponStatsList,
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
            weapons: WeaponStatsList::new(),
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
        Session {
            character: character,
            events: EventList::new(),
            weapons: WeaponStatsList::new(),
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
            },
            EventType::DestroyVehicle => {self.vehicles_destroyed += 1},
            EventType::LoseVehicle => { self.vehicles_lost += 1},
            EventType::LoseVehicleFF => { self.vehicles_lost += 1},
            _ => {},
        };
        self.events.push(event);
    }

    pub fn ui(&self, ctx: &egui::Context) {
        self.events.ui(&ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
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
                        ui.label(format!("HSR {:.3}", self.headshot_kills as f32 / self.kill_count as f32));
                    } else {
                        ui.label("HSR -");
                    }
                    //accuracy needed
                    ui.label(format!("Accuracy {:.3}", 0.0));

            });

            ui.separator();
            egui::Grid::new("lifetime_stats")
                .min_col_width(10.0)
                .show(ui, |ui| {
                    ui.label("Lifetime:");
                    ui.end_row();
                  /*  ui.label(format!("Kills {}", self.kill_count));
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
                        ui.label(format!("HSR {:.3}", self.headshot_kills as f32 / self.kill_count as f32));
                    } else {
                        ui.label("HSR -");
                    }
                    //accuracy needed*/

            });

        });

         
    }
    
    pub fn end(&mut self, time: i64)
    {
        self.end_time = Some(time);
    }

}



#[allow(dead_code)]
pub struct WeaponStats {
    weapon_id: String,
    name: String,
    kills: u32,
    headshots: u32,
    fired: u64,
    hits: u64,
    lifetime_accuracy: f32,
    lifetime_hsr: f32,
    starting_accuracy: f32,
    starting_hsr: f32,
}

#[allow(dead_code)]
pub struct WeaponStatsList {
    weapons: Vec<WeaponStats>,
}

impl WeaponStatsList {
    pub fn new() -> Self {
        WeaponStatsList{
            weapons: Vec::new(),
        }
    }
}
