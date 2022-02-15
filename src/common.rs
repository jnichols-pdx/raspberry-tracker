//use eframe::{egui, epi};

use num_enum::FromPrimitive;

pub struct Action {
    pub val: u32,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, FromPrimitive, PartialEq)]
#[repr(u8)]
pub enum Faction {
    VS = 0x01,
    #[num_enum(default)]
    TR = 0x02,
    NC = 0x03,
    NSO = 0x04,
}

pub struct Character {
    pub full_name: String,
    pub lower_name: String,
    pub server: String,
    pub outfit: String,
    pub character_id: String,
    pub auto_track: bool,
    pub faction: Faction,
}

pub fn name_from_faction(faction: Faction) -> String
{
    match faction {
        Faction::VS => "VS".to_owned(),
        Faction::TR => "TR".to_owned(),
        Faction::NC => "NC".to_owned(),
        Faction::NSO => "Robit".to_owned(),
    }
}

impl Character {
    pub fn new(new_lower: String) -> Self
    {
        Character {
            full_name: new_lower.to_uppercase(),
            lower_name: new_lower,
            server: "memelad".to_owned(),
            outfit: "outfitters".to_owned(),
            character_id: "123454987954698".to_owned(),
            auto_track: true,
            faction: Faction::from(1),
        }
    }
}

pub struct CharacterList {
    pub characters: Vec<Character>,
    pub new_char_name: String,
}

impl CharacterList {
    pub fn new() -> Self
    {
        CharacterList {
            characters: Vec::new(),
            new_char_name: "".to_owned(),
        }
    }
}

pub trait View {
    fn ui(&mut self, ctx: &egui::CtxRef);// &egui::Context);//,  ui: &mut egui::Ui);
    fn draw(&mut self, ui: &mut egui::Ui);
}
