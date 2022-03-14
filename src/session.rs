#![allow(unused_variables)]
use tokio_tungstenite::tungstenite::protocol::Message;


use tokio::sync::{mpsc};
use crate::common::*;
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
            if let Some(end_time_i) = self.end_time {
                let end_time= OffsetDateTime::from_unix_timestamp(end_time_i).unwrap_or_else(|_| {OffsetDateTime::now_utc()}).to_timezone(self.time_zone); //TODO: cleanup
                let formatted_end_time;
                if let Ok(tstamp) = end_time.format(&formatter) {
                    formatted_end_time= tstamp;
                } else {
                    formatted_end_time = "?-?-? ?:?:?".to_owned();
                }

                ui.label(format!("{}  {} - {}", self.character.full_name, formatted_start_time, formatted_end_time ));
            } else {
                let now_time = OffsetDateTime::now_utc();
                let session_duration = now_time - start_time;
                let hours = session_duration.whole_hours();
                let minutes = session_duration.whole_minutes() % 60;
                let seconds = session_duration.whole_seconds() % 60;
                let millis = session_duration.subsec_milliseconds() /10;
                ui.label(format!("{}  {},  {:02}:{:02}:{:02}.{:02}",
                    self.character.full_name,
                    formatted_start_time,
                    hours, minutes, seconds, millis
                ));
            }

            //TODO - current session Duration display.

            ui.label(format!("Kills {}", self.kill_count));
            ui.label(format!("by HS {}", self.headshot_kills));
            ui.label(format!("Vehicle kills {}", self.vehicle_kills));
            ui.label(format!("Deaths {}", self.death_count));
            ui.label(format!("by HS {}", self.headshot_deaths));
            ui.label(format!("Vehicle deaths {}", self.vehicle_deaths));
            ui.label(format!("Vehicles destroyed {}", self.vehicles_destroyed));
            ui.label(format!("Vehicles lost {}", self.vehicles_lost));
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

            /*TableBuilder::new(ui)
                .column(Size::Absolute(40.0))
                .column(Size::RemainderMinimum(100.0))
                .column(Size::Absolute(50.0))
                .header(15.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Fixed-S");
                    });
                    header.col(|ui| {
                        ui.heading("Grows");
                    });
                    header.col(|ui| {
                        ui.heading("Fixed-L");
                    });
                })
                .body(|mut body| {
                    body.row(20.0, |mut row| {
                        row.col_clip(|ui| {
                            ui.label("smol");
                        });
                        row.col(|ui| {
                            ui.label("stretechyalkajd lakdjflasdkj flasdkfj a");
                        });
                        row.col_clip(|ui| {
                            ui.label("beeg");
                        });
                    });
                });*/

        });

         
    }
    
    pub fn end(&mut self, time: i64)
    {
        self.end_time = Some(time);
    }

}

#[derive(Clone)]
pub struct FullCharacter {
    pub full_name: String,
    pub lower_name: String,
    pub server: World,
    pub outfit: Option<String>,
    pub outfit_full: Option<String>,
    pub character_id: String,
    pub faction: Faction,
    pub br: u8,
    pub asp: u8,
}

impl FullCharacter {
    pub fn new(source: &Character, br: u8, asp: u8) -> Self {
        let mut new_char = FullCharacter {
            full_name: source.full_name.to_owned(),
            lower_name: source.lower_name.to_owned(),
            server: source.server,
            outfit: None,
            outfit_full: None,
            character_id: source.character_id.to_owned(),
            faction: source.faction,
            br: br,
            asp: asp,
        };

        if let Some(outfit) = &source.outfit {
            new_char.outfit = Some(outfit.to_owned());
        }
        if let Some(outfit_full) = &source.outfit_full {
            new_char.outfit_full = Some(outfit_full.to_owned());
        }

        new_char
    }
}

pub struct Event {
    pub kind: EventType,
    pub faction: Faction,
    pub br : u8,
    pub asp: u8,
    pub class: Class,
    pub name: String,
    pub weapon: String,
    pub headshot: bool,
    pub kdr: f32,
    pub timestamp: i64,
    pub vehicle: Option<Vehicle>,
    pub datetime: String,

}

impl Event {
    pub fn ui(&self, body: &mut egui_extras::TableBody) {
        let img_size = (14.0,14.0);
        body.row(20.0, |mut row| {
            row.col_clip(|ui| { //faction
                match ui.ctx().texture_by_name(&self.faction.to_string()) {
                    Some(image) => ui.image(image.id(), img_size),
                    None => ui.label(egui::RichText::new(self.faction.to_string()).small()),
                };
            });
            row.col_clip(|ui| { //BR
                if self.asp > 0 {
                    ui.label(egui::RichText::new(format!("{}~{}", self.br, self.asp)).small());
                } else {
                    ui.label(egui::RichText::new(format!("{}", self.br)).small());
                }
            });
            row.col_clip(|ui| { //Class
                match ui.ctx().texture_by_name(&self.class.to_string()) {
                    Some(image) => ui.image(image.id(), img_size),
                    None =>ui.label(""),// ui.label(egui::RichText::new(self.class.to_string()).small()),
                };
            });
            row.col_clip(|ui| { //Vehicle
                //Override for orbital strike direct kills (can't track when players die from falling
                //damage after being thrown airborn by orbital :( )
                if self.weapon == "Orbital Strike Uplink" {
                    if let Some(image) = ui.ctx().texture_by_name(&"Orbital") {
                        ui.image(image.id(), img_size);
                    };
                } else if let Some(vehicle) = self.vehicle {
                    match ui.ctx().texture_by_name(&vehicle.to_string()) {
                        Some(image) => ui.image(image.id(), img_size),
                        None => ui.label(egui::RichText::new(vehicle.to_string()).small()),
                    };
                }
            });
            row.col(|ui| { //Player Name
                let bg_color;
                match self.kind {
                    EventType::Death => bg_color = Color32::from_rgb(80,0,0),
                    EventType::TeamDeath => bg_color = Color32::from_rgb(80,80,0),
                    EventType::Kill => bg_color = Color32::from_rgb(0,80,0),
                    EventType::Suicide => bg_color = Color32::from_rgb(0,0,80),
                    EventType::TeamKill => bg_color = Color32::from_rgb(65,80,0),
                    EventType::LoseVehicle => bg_color = Color32::from_rgb(80,0,0),
                    EventType::LoseVehicleFF => bg_color = Color32::from_rgb(80,80,0),
                    EventType::DestroyVehicle => bg_color = Color32::from_rgb(0,80,0),
                    EventType::DestroyVehicleFF=> bg_color = Color32::from_rgb(0,0,80),
                    _ => bg_color = Color32::from_rgb(80,80,80),
                };
                ui.label(egui::RichText::new(&self.name).small().background_color(bg_color).color(Color32::from_rgb(255,255,255)));
            });
            row.col_clip(|ui| { //Weapon
                ui.label(egui::RichText::new(&self.weapon).small());
            });
            row.col_clip(|ui| { //Headshot
                if self.headshot {
                    match ui.ctx().texture_by_name("Headshot") {
                        Some(image) => ui.image(image.id(), img_size),
                        None => ui.label(egui::RichText::new("HS!").small()),
                    };
                }
            });
            row.col_clip(|ui| { //KD ratio
                ui.label(egui::RichText::new(format!("{:.2}",self.kdr)).small());
            });
            row.col_clip(|ui| { //Timestamp
                ui.label(egui::RichText::new(&self.datetime).small());
            });
        });

/*

            match self.kind {
                EventType::Death => ui.label(format!("{} {}{} killed You with {}. {:.2} {}", br,vehicle_str, self.name, self.weapon, self.kdr, self.datetime)),
                EventType::TeamDeath => ui.label(format!("{} {}{} TEAMkilled You with {}. {:.2} {}", br,vehicle_str, self.name, self.weapon, self.kdr, self.datetime)),
                EventType::Kill => ui.label(format!("{} {}You killed {} with {}. {:.2} {}", br,vehicle_str, self.name, self.weapon, self.kdr, self.datetime)),
                EventType::Suicide => ui.label(format!("{} {}You killed {} with {}. {:.2} {}", br,vehicle_str, self.name, self.weapon, self.kdr, self.datetime)),
                EventType::TeamKill => ui.label(format!("{} {}You TEAMkilled {} with {}. {:.2} {}", br,vehicle_str, self.name, self.weapon, self.kdr, self.datetime)),
                _ => ui.label("other".to_owned()),
            };



        });*/
    }
}
pub struct EventList {
    events: Vec<Event>,
}

impl EventList {
    pub fn new() -> Self {
        EventList {
            events: Vec::new(),
        }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn ui(&self, ctx: &egui::Context) {
        egui::SidePanel::right("events_panel").min_width(400.0).show(ctx, |ui| {

            TableBuilder::new(ui)
                .column(Size::Absolute(25.0)) //Faction
                .column(Size::Absolute(30.0)) //BR
                .column(Size::Absolute(25.0)) //Class
                .column(Size::Absolute(25.0)) //Vehicle
                .column(Size::RemainderMinimum(100.0)) //playername
                .column(Size::RemainderMinimum(80.0)) //weapon
                .column(Size::Absolute(25.0)) //headshot
                .column(Size::Absolute(30.0)) //KD
                .column(Size::Absolute(80.0)) //Timestamp
                .header(15.0, |mut header| {
                    header.col(|ui| {
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("BR").small());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Class").small());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Veh.").small());
                    });
                    header.col_clip(|ui| {
                        ui.label(egui::RichText::new("Player").small());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Method").small());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("HS").small());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("KD").small());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Time").small());
                    });
                })
                .body(|mut body| {
                    for event in self.events.iter().rev() {
                        event.ui(&mut body);
                    }
                });

/*
            let row_height = ui.text_style_height(&text_style);
            ScrollArea::vertical().show_rows( //is an stick_to_bottom, but no stick_to_top ...
                ui,
                row_height,
                self.events.len(),
                |ui, row_range| {
                    let rev = self.events.iter().rev();
                    let length = row_range.end - row_range.start;
                    let mut shown = 0;
                    for event in  rev.skip(row_range.start) {
                        event.ui(ui);
                        shown += 1;
                        if shown > length {
                            break;
                        }
                    }


                }
            );
            */
        });
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
