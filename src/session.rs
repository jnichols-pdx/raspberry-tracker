#![allow(unused_variables)]
use tokio_tungstenite::tungstenite::protocol::Message;


use tokio::sync::{mpsc};
use crate::common::*;
use eframe::{egui, epi};
use egui::*;

#[allow(dead_code)]
pub struct Session {
   character: FullCharacter,
   events: EventList,
   weapons: WeaponStatsList,
   start_time: i64,
   end_time: Option<i64>,
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
        let character = FullCharacter::new(&character, br, asp);
        Session {
            character: character,
            events: EventList::new(),
            weapons: WeaponStatsList::new(),
            start_time: start,
            end_time: None,
        }
    }

    pub fn new_from_full(character: FullCharacter, start: i64) -> Self {
        Session {
            character: character,
            events: EventList::new(),
            weapons: WeaponStatsList::new(),
            start_time: start,
            end_time: None,
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
        self.events.push(event);
    }

    pub fn ui(&self, ctx: &egui::Context) {
        self.events.ui(&ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            //ui.heading(format!("{} Stats", new_char_name));
            ui.heading("<char> Stats");
                ui.label(self.get_list_name());
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
    pub fn ui(&self, ui: &mut egui::Ui) {
        /*let headshot_str ;
        if self.headshot {
            headshot_str = " HS!";
        } else {
            headshot_str = "";
        }*/

        ui.horizontal(|ui| {
            match ui.ctx().texture_by_name(&self.faction.to_string()) {
                Some(image) => ui.image(image.id(), (18.0,18.0)),
                None => ui.label(self.faction.to_string()),
            };

            match ui.ctx().texture_by_name(&self.class.to_string()) {
                Some(image) => ui.image(image.id(), (18.0,18.0)),
                None => ui.label(self.faction.to_string()),
            };
            
            let mut vehicle_str;
            if let Some(vehicle) = self.vehicle {
                match ui.ctx().texture_by_name(&vehicle.to_string()) {
                    Some(image) => {
                        ui.image(image.id(), (18.0,18.0));
                        vehicle_str = "".to_owned();
                    },
                    None => vehicle_str = vehicle.to_string().to_owned(),
                };
            } else {
                vehicle_str = "".to_owned();
            }
            //Override for orbital strike direct kills (can't track when players die from falling
            //damage after being thrown airborn by orbital :( )
            if self.weapon == "Orbital Strike Uplink" {
                if let Some(image) = ui.ctx().texture_by_name(&"Orbital") {
                    ui.image(image.id(), (18.0,18.0));
                    vehicle_str = "".to_owned();
                };
            }

            let br;
            if self.asp > 0 {
                br = format!("{}~{}", self.br, self.asp);
            } else {
                br = format!("{}",self.br);
            }


            match self.kind {
                EventType::Death => ui.label(format!("{} {}{} killed You with {}. {:.2} {}", br,vehicle_str, self.name, self.weapon, self.kdr, self.datetime)),
                EventType::TeamDeath => ui.label(format!("{} {}{} TEAMkilled You with {}. {:.2} {}", br,vehicle_str, self.name, self.weapon, self.kdr, self.datetime)),
                EventType::Kill => ui.label(format!("{} {}You killed {} with {}. {:.2} {}", br,vehicle_str, self.name, self.weapon, self.kdr, self.datetime)),
                EventType::Suicide => ui.label(format!("{} {}You killed {} with {}. {:.2} {}", br,vehicle_str, self.name, self.weapon, self.kdr, self.datetime)),
                EventType::TeamKill => ui.label(format!("{} {}You TEAMkilled {} with {}. {:.2} {}", br,vehicle_str, self.name, self.weapon, self.kdr, self.datetime)),
                _ => ui.label("other".to_owned()),
            };

            if self.headshot {
                match ui.ctx().texture_by_name("Headshot") {
                    Some(image) => ui.image(image.id(), (18.0,18.0)),
                    None => ui.label("HS!"),
                };
            }


        });
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
        egui::SidePanel::right("events_panel").min_width(250.0).show(ctx, |ui| {
            ui.heading("Event feed");
            let text_style = TextStyle::Body;
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
