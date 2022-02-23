#![allow(unused_variables)]


//use eframe::{egui, epi};

use tokio_tungstenite::tungstenite::protocol::Message;
use num_enum::FromPrimitive;
use tokio::sync::{mpsc};
use sqlite::State;
use crate::session::*;

pub struct Action {
    pub val: u32,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
#[repr(i64)]
pub enum Faction {
    VS = 0x01,
    #[num_enum(default)]
    NC = 0x02,
    TR = 0x03,
    NSO = 0x04,

    UNK = 0x00,
}

#[derive(Debug, Clone)]
pub struct Character {
    pub full_name: String,
    pub lower_name: String,
    pub server: World,
    pub outfit: Option<String>,
    pub outfit_full: Option<String>,
    pub character_id: String,
    pub auto_track: bool,
    pub faction: Faction,
    pub to_remove: bool,
    pub confirm_visible: bool,
    pub to_track: bool,
    pub changed_auto_track: bool,
}

pub fn name_from_faction(faction: Faction) -> String
{
    match faction {
        Faction::VS => "VS".to_owned(),
        Faction::NC => "NC".to_owned(),
        Faction::TR => "TR".to_owned(),
        Faction::NSO => "Robit".to_owned(),
        Faction::UNK => "???".to_owned(),
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
#[repr(i64)]
pub enum World {
    CN = 1,
    #[num_enum(default)]
    EM = 17,
    ML = 10,
    CB = 13,
    JA = 19,
    ST = 40,
}

pub fn name_from_world(world: World) -> String
{
    match world{
        World::CN => "Connery".to_owned(),
        World::EM => "Emerald".to_owned(),
        World::ML => "Miller".to_owned(),
        World::CB => "Cobalt".to_owned(),
        World::JA => "Jaeger".to_owned(),
        World::ST => "SolTech".to_owned(),
    }
}

pub fn lookup_character_id(new_char: &String) -> Result<Option<String>, ureq::Error> {
    let resp: serde_json::Value = ureq::get(&*format!("http://census.daybreakgames.com/s:example/get/ps2/character/?name.first_lower={}&c:show=character_id", new_char.to_lowercase()))
                .call()?
                .into_json()?;

    if resp["error"].is_null() {
        println!("found: {}", resp["character_list"][0]["character_id"]);
        if resp["returned"] == 0 {
            Ok(None)
        } else {
            let quoted = resp["character_list"][0]["character_id"].to_string();

            Ok(Some(quoted.unquote()))
        }
    } else {
        println!("ERROR: {:?}", resp["error"]);
        Ok(None)
    }
}

pub fn lookup_new_char_details(new_id: &String) -> Result<serde_json::Value, ureq::Error> {
    let resp = ureq::get(&*format!(
        "http://census.daybreakgames.com/s.example/get/ps2/character/?character_id={}&c:hide=battle_rank.percent_to_next,certs,profile_id,times,title_id,daily_ribbon&c:join=outfit_member_extended^show:name'alias^inject_at:outfit,characters_stat^terms:stat_name=weapon_deaths^show:value_forever^inject_at:weapon_deaths,characters_stat_history^terms:stat_name=kills^show:all_time^inject_at:kills&c:resolve=world",
        new_id))
        .call()?
        .into_json()?;

    Ok(resp)
}

pub fn character_from_json(json: &serde_json::Value) -> Result<Character, String> {

    let new_char = &json["character_list"][0];
    println!("deets: {:?}", new_char);
    let faction_num = new_char["faction_id"].to_string().unquote().parse::<i64>().unwrap();
    let world_num = new_char["world_id"].to_string().unquote().parse::<i64>().unwrap();

    let mut bob = Character {
        full_name: new_char["name"]["first"].to_string().unquote(),
        lower_name: new_char["name"]["first_lower"].to_string().unquote(),
        server: World::from(world_num),
        outfit: None,
        outfit_full: None,
        character_id: new_char["character_id"].to_string().unquote(),
        auto_track: true,
        faction: Faction::from(faction_num),
        to_remove: false,
        confirm_visible: false,
        to_track: false,
        changed_auto_track: false,
    };

    if new_char["outfit"].is_object() {
        bob.outfit = Some(new_char["outfit"]["alias"].to_string().unquote());
        bob.outfit_full = Some(new_char["outfit"]["name"].to_string().unquote());
    }
    Ok(bob)

}

pub fn full_character_from_json(json: &serde_json::Value) -> Result<FullCharacter, String> {
    let bob = character_from_json(json).unwrap();
    let biff = FullCharacter::new(&bob, 
            json["character_list"][0]["battle_rank"]["value"].to_string().unquote().parse::<u8>().unwrap(), 
            json["character_list"][0]["prestige_level"].to_string().unquote().parse::<u8>().unwrap());
    Ok(biff)
}

//characters (name TEXT, lower_name TEXT, outfit TEXT, outfit_full TEXT, id TEXT NOT NULL, auto_track INTEGER, server INTEGER, faction INTEGER)
pub fn db_save_new_char(char: &Character, db: &sqlite::Connection) -> bool {
            //println!("In save");
            let mut statement = db
                .prepare("INSERT INTO characters VALUES (?,?,?,?,?,?,?,?);").unwrap();
            statement.bind(1,char.full_name.as_str()).unwrap();
            statement.bind(2,&*char.lower_name).unwrap();
            match &char.outfit {
                Some(outfit_alias) => statement.bind(3,outfit_alias.as_str()).unwrap(),
                None => statement.bind(3,()).unwrap(),
            };
            match &char.outfit_full {
                Some(outfit_name) => statement.bind(4,outfit_name.as_str()).unwrap(),
                None => statement.bind(4,()).unwrap(),
            }
            statement.bind(5,&*char.character_id).unwrap();
            statement.bind(6,char.auto_track as i64).unwrap();
            statement.bind(7,char.server as i64).unwrap();
            statement.bind(8,char.faction as i64).unwrap();

            //println!("{:?}", statement);
            // while let State::Row= statement.next().unwrap() {};
            match statement.next() {
                Ok(_) => true,
                Err(_) => false,
            }
}

pub fn db_update_char(char: &Character, db: &sqlite::Connection) {}

pub fn db_update_char_with_full(char: &FullCharacter, db: &sqlite::Connection) {}

pub fn db_set_char_auto_track(char: &Character, db: &sqlite::Connection) {
            let mut statement = db
                .prepare("UPDATE characters SET auto_track = ? WHERE id IS ?;").unwrap();
            statement.bind(1,char.auto_track as i64).unwrap();
            statement.bind(2,&*char.character_id).unwrap();

            while let State::Row= statement.next().unwrap() {};
}

pub fn db_remove_char(char: &Character, db: &sqlite::Connection) {
    let mut statement = db.prepare("DELETE FROM characters WHERE id LIKE ?;").unwrap();
    statement.bind(1,char.character_id.as_str()).unwrap();
    let _ = statement.next();
}

impl Character {
    /*pub fn new(new_lower: String) -> Self
    {
        Character {
            full_name: new_lower.to_uppercase(),
            lower_name: new_lower,
            server: World::EM,
            outfit: Some("OTFS".to_owned()),
            outfit_full: Some("OUTFITTERS".to_owned()),
            character_id: "123454987954698".to_owned(),
            auto_track: true,
            faction: Faction::VS,
            to_remove: false,
            confirm_visible: false,
            to_track: false,
        }
    }*/
}

pub struct CharacterList {
    pub characters: Vec<Character>,
    pub new_char_name: String,
    pub message: Option<String>,
    pub websocket_out: mpsc::Sender<Message>,
}

impl CharacterList {
    pub fn new(ws_out: mpsc::Sender<Message>) -> Self
    {
        CharacterList {
            characters: Vec::new(),
            new_char_name: "".to_owned(),
            message: None,
            websocket_out: ws_out,
        }
    }

    pub fn push(&mut self, new_char: Character) {
        self.characters.push(new_char);
    }

    pub fn has_auto_tracked(&self, target_id: String) -> bool {
        println!("track check for >{}<", target_id);
        if let Some(target) = &self.characters.iter().find(|&chara| chara.character_id.eq(&target_id))  {
            target.auto_track
        } else{
            false
        }
    }

    pub fn find_character_by_id(&self, target_id: String) -> Option<&Character> {
        if let Some(target) = &self.characters.iter().find(|&chara| chara.character_id.eq(&target_id))  {
            Some(&target)
        } else{
            None
        }
    }

    pub fn update_entry_from_full(&mut self, newer_char: &FullCharacter) {
       if let Some(mut target) = self.characters.iter_mut().find(|chara| (**chara).character_id.eq(&newer_char.character_id))  {
            target.full_name =  newer_char.full_name.to_owned();
            target.lower_name =  newer_char.lower_name.to_owned();
            target.server =  newer_char.server;
            if let Some(outfit_alias) = &newer_char.outfit {
                target.outfit = Some(outfit_alias.to_owned());
            } else {
                target.outfit = None;
            }
            if let Some(outfit_name) = &newer_char.outfit {
                target.outfit_full = Some(outfit_name.to_owned());
            } else {
                target.outfit_full = None;
            }
            target.character_id =  newer_char.character_id.to_owned();
            target.faction =  newer_char.faction;
       }
    }

}

pub trait ViewWithDB {
    fn ui(&mut self, ctx: &egui::Context, db: &sqlite::Connection);// &egui::Context);//,  ui: &mut egui::Ui);
    fn draw(&mut self, ui: &mut egui::Ui);
}

pub trait View {
    fn ui(&mut self, ctx: &egui::Context);// &egui::Context);//,  ui: &mut egui::Ui);
    fn draw(&mut self, ui: &mut egui::Ui);
}


pub trait StripQuote {
    fn unquote(&self) -> String;
}

impl StripQuote for String {
    fn unquote(&self) -> String {
        self[1..self.len() -1].to_owned()
    }
}


pub enum EventType {
    Death,
    Kill,
    TeamKill,
    Suicide,
    DestroyVehicle,
    LoseVehicle,
}

pub enum Vehicle {
    Flash,
    Javelin,
    Harasser,
    Sunderer,
    Lightning,
    Prowler,
    Vanguard,
    Magrider,
    Chimera,
    Colossus,
    Ant,
    Deliverer,

    DropPod,
    Mosquito,
    Scythe,
    Reaver,
    Dervish,
    Valkyrie,
    Wasp,
    Liberator,
    Galaxy,
    Lodestar,
    BastionFleetCarrier,

    Unknown,
}

pub enum Class {
    LightAssault,
    Medic,
    Engineer,
    HeavyAssault,
    Infiltrator,
    Max,

    Unknown,
}

