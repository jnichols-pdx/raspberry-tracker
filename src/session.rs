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

pub struct FullCharacter {
    full_name: String,
    pub lower_name: String,
    server: World,
    outfit: Option<String>,
    outfit_full: Option<String>,
    character_id: String,
    faction: Faction,
    br: u8,
    asp: u8,
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

