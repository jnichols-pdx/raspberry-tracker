#![allow(unused_variables)]
use tokio_tungstenite::tungstenite::protocol::Message;


use tokio::sync::{mpsc};
use crate::common::*;
use eframe::{egui, epi};
use egui::*;

pub struct Session {
   character: FullCharacter,
   events: EventList,
   weapons: WeaponStatsList,
   start_time: u64,
   end_time: Option<u64>,
}

impl Session {
    pub fn match_player_id(&self, to_match: &String) -> bool {
        to_match.eq(&self.character.character_id)
    }

    pub fn new(character: Character, br: u8, asp: u8, start: u64) -> Self {
        let character = FullCharacter::new(&character, br, asp);
        Session {
            character: character,
            events: EventList::new(),
            weapons: WeaponStatsList::new(),
            start_time: start,
            end_time: None,
        }
    }

    pub fn new_from_full(character: FullCharacter, start: u64) -> Self {
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
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            //ui.heading(format!("{} Stats", new_char_name));
            ui.heading("<char> Stats");
                ui.label(self.get_list_name());
        });

            self.events.ui(&ctx);
         
    }
    
    pub fn end(&mut self, time: u64)
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
    pub timestamp: u64,
    pub vehicle: Option<Vehicle>,

}

impl Event {
    pub fn ui(&self, ui: &mut egui::Ui) {
        match self.kind {
            EventType::Death => ui.label(format!("{} killed you with {}.", self.name, self.weapon)),
            EventType::Kill => ui.label(format!("You killed {} with {}.", self.name, self.weapon)),
            _ => ui.label("other".to_owned()),
        };
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
