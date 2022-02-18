#![allow(unused_variables)]


//use eframe::{egui, epi};

use num_enum::FromPrimitive;
use sqlite::State;

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
}

#[derive(Debug)]
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
}

pub fn name_from_faction(faction: Faction) -> String
{
    match faction {
        Faction::VS => "VS".to_owned(),
        Faction::NC => "NC".to_owned(),
        Faction::TR => "TR".to_owned(),
        Faction::NSO => "Robit".to_owned(),
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
    println!("results: {}", resp["returned"]);
println!("found: {}", resp["character_list"][0]["character_id"]);
if resp["character_list"][0]["character_id"].is_string() {
    println!("yup");
}
    if resp["returned"] == 0 {
        Ok(None)
    } else {
        let quoted = resp["character_list"][0]["character_id"].to_string();

        Ok(Some(quoted.unquote()))
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

pub fn db_remove_char(char: &Character, db: &sqlite::Connection) {
    let mut statement = db.prepare("DELETE FROM characters WHERE id LIKE ?;").unwrap();
    statement.bind(1,char.character_id.as_str()).unwrap();
    let _ = statement.next();
}

impl Character {
    pub fn new(new_lower: String) -> Self
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

    pub fn push(&mut self, new_char: Character) {
        self.characters.push(new_char);
    }
}

pub trait ViewWithDB {
    fn ui(&mut self, ctx: &egui::CtxRef, db: &sqlite::Connection);// &egui::Context);//,  ui: &mut egui::Ui);
    fn draw(&mut self, ui: &mut egui::Ui);
}

pub trait View {
    fn ui(&mut self, ctx: &egui::CtxRef);// &egui::Context);//,  ui: &mut egui::Ui);
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

