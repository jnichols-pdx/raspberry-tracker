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
}

impl Session {
    pub fn match_player_id(&self, to_match: &String) -> bool {
        to_match.eq(&self.character.character_id)
    }

    pub fn new(character: Character, br: u8, asp: u8) -> Self {
        let character = FullCharacter::new(&character, br, asp);
        Session {
            character: character,
            events: EventList::new(),
            weapons: WeaponStatsList::new(),
        }
    }

    pub fn new_from_full(character: FullCharacter) -> Self {
        Session {
            character: character,
            events: EventList::new(),
            weapons: WeaponStatsList::new(),
        }
    }

}

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
    kind: EventType,
    faction: Faction,
    br: u8,
    asp: u8,
    class: Class,
    name: String,
    weapon_id: String,
    headshot: bool,
    kdr: f32,
    timestamp: u64,
    vehicle: Option<Vehicle>,

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
