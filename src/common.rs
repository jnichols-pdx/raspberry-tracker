#![allow(unused_variables)]
#![allow(dead_code)]


//use eframe::{egui, epi};

use tokio_tungstenite::tungstenite::protocol::Message;
use num_enum::FromPrimitive;
use tokio::sync::{mpsc};
use crate::session::*;
use crate::db::*;
use sqlx::sqlite::SqlitePool;
use std::io::Read;
use std::sync::{Arc, RwLock};

/*pub struct Action {
    pub val: u32,
}*/

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
#[repr(i64)]
pub enum Faction {
    VS = 1,
    NC = 2,
    TR = 3,
    NSO = 4,

    #[num_enum(default)]
    Unknown = 0,
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

impl std::fmt::Display for Faction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Faction::VS => write!(f,"VS"),
            Faction::NC => write!(f,"NC"),
            Faction::TR => write!(f,"TR"),
            Faction::NSO => write!(f,"Robit"),
            Faction::Unknown => write!(f,"???"),
        }
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

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            World::CN => write!(f,"Connery"),
            World::EM => write!(f,"Emerald"),
            World::ML => write!(f,"Miller"),
            World::CB => write!(f,"Cobalt"),
            World::JA => write!(f,"Jaeger"),
            World::ST => write!(f,"SolTech"),
        }
    }
}

pub fn lookup_character_id(new_char: &String) -> Result<Option<String>, ureq::Error> {
    let resp: serde_json::Value = ureq::get(&*format!("http://census.daybreakgames.com/s:raspberrytracker/get/ps2/character/?name.first_lower={}&c:show=character_id", new_char.to_lowercase()))
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
        "http://census.daybreakgames.com/s:raspberrytracker/get/ps2/character/?character_id={}&c:hide=battle_rank.percent_to_next,certs,profile_id,times,title_id,daily_ribbon&c:join=outfit_member_extended^show:name'alias^inject_at:outfit,characters_stat^terms:stat_name=weapon_deaths^show:value_forever^inject_at:weapon_deaths,characters_stat_history^terms:stat_name=kills^show:all_time^inject_at:kills&c:resolve=world",
        new_id))
        .call()?
        .into_json()?;

    Ok(resp)
}

pub fn lookup_weapon_name(new_id: &String) -> Result<serde_json::Value, ureq::Error> {
    let resp = ureq::get(&*format!(
        "http://census.daybreakgames.com/s:raspberrytracker/get/ps2/item/?item_id={}", new_id)).call()?.into_json()?;

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

pub fn download_census_image(census_id: u32) -> Result<Option<Vec<u8>>, ureq::Error> {
    let resp = ureq::get(&*format!("http://census.daybreakgames.com/files/ps2/images/static/{}.png", census_id))
                .call()?;
    if resp.status() == 200 {
        println!("{:?}",resp); 
        let mut image_bytes: Vec<u8> = Vec::with_capacity(1024);
        resp.into_reader().take(5242880).read_to_end(&mut image_bytes)?;
        Ok(Some(image_bytes))
    } else {
        Ok(None)
    }

}

pub fn is_online(char_id: &String) -> Result<bool, ureq::Error> {
    let resp = ureq::get(&*format!("http://census.daybreakgames.com/s:raspberrytracker/get/ps2/characters_online_status/?character_id={}", char_id))
        .call()?;
    if resp.status() == 200 {
        let json: serde_json::Value = resp.into_json()?;
        //println!("Online check: {:?}", json);
        let status = &json["characters_online_status_list"][0]["online_status"];
        Ok(status.is_string() && status != "0")
    } else {
        Ok(false)
    }
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
    pub session_list: Arc<RwLock<Vec<Session>>>,
}

impl CharacterList {
    pub fn new(ws_out: mpsc::Sender<Message>, sl: Arc<RwLock<Vec<Session>>>) -> Self
    {
        CharacterList {
            characters: Vec::new(),
            new_char_name: "".to_owned(),
            message: None,
            websocket_out: ws_out,
            session_list: sl,
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
    fn ui(&mut self, ctx: &egui::Context, db: &DatabaseSync);
    fn draw(&mut self, ui: &mut egui::Ui);
}

pub trait View {
    fn ui(&mut self, ctx: &egui::Context);
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

pub trait TextureLookup {
    fn texture_by_name(&self, name: &str) -> Option<egui::TextureHandle>;
}


pub enum EventType {
    Death,
    Kill,
    TeamKill,
    TeamDeath,
    Suicide,
    DestroyVehicle,
    LoseVehicle,
    DestroyVehicleFF,
    LoseVehicleFF,
    Unknown,
}

#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
#[repr(i64)]
pub enum Vehicle {
    Flash = 1,
    Javelin = 2033,
    Harasserr = 12,
    Sunderer = 2,
    Lightning = 3,
    Prowler = 6,
    Vanguard = 5,
    Magrider = 4,
    Chimera = -0x10,  //Currently unknown (not listed by census Vehicles collection)
    Colossus = 2007,
    Ant = 15,
    Deliverer = 2039,

    DropPod = 13,
    Mosquito = 9,
    Scythe = 7,
    Reaver = 8,
    Dervish = 2136,
    Valkyrie = 14,
    Wasp = 2040,
    Liberator = 10,
    Galaxy = 11,
    Lodestar = 2140,//found by empirical testing.
    BastionFleetCarrier = 2019,

    AIPhalanxTurret = 100,
    ManaAITurret = 101,
    ManaAVTurret = 102,
    Spitfire = 103, //WORKS for standard spitty.
    SpitfireAlt1 = 104, //is one of these aux spitty?
    SpitfireAlt2 = 105,
    AAPhalanxTurret = 150,
    AVPhalanxTurret = 151,
    AVBuilderTower = 160, //THIS appears to be correct, these are the towers
    AABuilderTower = 161,
    AIBuilderTower = 162, 
    
    Glaive = 163,
    AVPhalanxTurretAlt = 2006,
    DropPodALt = 2008,
    AIPhalanxTurretAlt = 2009,
    PocketFlash = 2010,
    Flail = 2021,
    
    Pumpkin = 2036,

    MosquitoInterceptor = 2122,
    ReaverInterceptor = 2123,
    ScytheInterceptor = 2124,
    JavelinAlt1 = 2125,
    SpitfireALt3 = 2128,
    JavelinAlt2 = 2129,

    ReclaimedSunderer = 2130,
    ReclaimedGalaxy = 2131,
    ReclaimedValkyrie = 2132,
    ReclaimedMagrider = 2133,
    ReclaimedVanguard = 2134,
    ReclaimedProwler = 2135,

    #[num_enum(default)]
    NoVehicle= 0,

    Unknown = -1,
}


impl std::fmt::Display for Vehicle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vehicle::Flash => write!(f,"Flash"),
            Vehicle::Javelin => write!(f,"Javelin"),
            Vehicle::Harasserr => write!(f,"Harasser"),
            Vehicle::Sunderer => write!(f,"Sunderer"),
            Vehicle::Lightning => write!(f,"Lightning"),
            Vehicle::Prowler => write!(f,"Prowler"),
            Vehicle::Vanguard => write!(f,"Vanguard"),
            Vehicle::Magrider => write!(f,"Magrider"),
            Vehicle::Chimera => write!(f,"Chimera"),
            Vehicle::Colossus => write!(f,"Colossus"),
            Vehicle::Ant => write!(f,"Ant"),
            Vehicle::Deliverer => write!(f,"Deliverer"),

            Vehicle::DropPod => write!(f,"DropPod"),
            Vehicle::Mosquito => write!(f,"Mosquito"),
            Vehicle::Scythe => write!(f,"Scythe"),
            Vehicle::Reaver => write!(f,"Reaver"),
            Vehicle::Dervish => write!(f,"Dervish"),
            Vehicle::Valkyrie => write!(f,"Valkyrie"),
            Vehicle::Wasp => write!(f,"Wasp"),
            Vehicle::Liberator => write!(f,"Liberator"),
            Vehicle::Galaxy => write!(f,"Galaxy"),
            Vehicle::Lodestar => write!(f,"Lodestar"),
            Vehicle::BastionFleetCarrier => write!(f,"BastionFleetCarrier"),

            Vehicle::AIPhalanxTurret => write!(f,"AIPhalanxTurret"),
            Vehicle::ManaAITurret => write!(f,"ManaAITurret"),
            Vehicle::ManaAVTurret => write!(f,"ManaAVTurret"),
            Vehicle::Spitfire => write!(f,"Spitfire"),
            Vehicle::SpitfireAlt1 => write!(f,"Spitfire1"),
            Vehicle::SpitfireAlt2 => write!(f,"Spitfire2"),
            Vehicle::AAPhalanxTurret => write!(f,"AAPhalanxTurret"),
            Vehicle::AVPhalanxTurret => write!(f,"AVPhalanxTurret"),
            Vehicle::AVBuilderTower => write!(f,"AVBuilderTower"),
            Vehicle::AABuilderTower => write!(f,"AABuilderTower"),
            Vehicle::AIBuilderTower => write!(f,"AIBuilderTower"),

            Vehicle::Glaive => write!(f,"Glaive"),
            Vehicle::AVPhalanxTurretAlt => write!(f,"AVPhalanxTurret1"),
            Vehicle::DropPodALt => write!(f,"Droppod1"),
            Vehicle::AIPhalanxTurretAlt => write!(f,"AIPhalanxTurret1"),
            Vehicle::PocketFlash => write!(f,"PocketFlash"),
            Vehicle::Flail => write!(f,"Flail"),

            Vehicle::MosquitoInterceptor => write!(f,"Mossy-Interceptor"),
            Vehicle::ReaverInterceptor => write!(f,"Reaver-Interceptor"),
            Vehicle::ScytheInterceptor => write!(f,"Scythe-Interceptor"),
            Vehicle::JavelinAlt1 => write!(f,"Javelin1"),
            Vehicle::SpitfireALt3 => write!(f,"Spitfire3"),
            Vehicle::JavelinAlt2 => write!(f,"Javelin2"),

            Vehicle::ReclaimedSunderer => write!(f,"Sunderer-Reclaimed"),
            Vehicle::ReclaimedGalaxy => write!(f,"Galaxy-Reclaimed"),
            Vehicle::ReclaimedValkyrie => write!(f,"Valkyrie-Reclaimed"),
            Vehicle::ReclaimedMagrider => write!(f,"Magrider-Reclaimed"),
            Vehicle::ReclaimedVanguard => write!(f,"Vanguard-Reclaimed"),
            Vehicle::ReclaimedProwler => write!(f,"Prowler-Reclaimed"),

            Vehicle::Pumpkin => write!(f,"Pumpkin"),

            Vehicle::NoVehicle=> write!(f,"NONE"),
            Vehicle::Unknown => write!(f,"???"),
        }
    }
}

impl Vehicle {

    pub fn is_true_vehicle(&self) -> bool{
       match self {
            Vehicle::Flash  |
            Vehicle::Javelin  |
            Vehicle::Harasserr  |
            Vehicle::Sunderer  |
            Vehicle::Lightning  |
            Vehicle::Prowler  |
            Vehicle::Vanguard  |
            Vehicle::Magrider  |
            Vehicle::Chimera  |
            Vehicle::Colossus  |
            Vehicle::Ant  |
            Vehicle::Deliverer  |

            Vehicle::DropPod  |
            Vehicle::Mosquito  |
            Vehicle::Scythe  |
            Vehicle::Reaver  |
            Vehicle::Dervish  |
            Vehicle::Valkyrie  |
            Vehicle::Wasp  |
            Vehicle::Liberator  |
            Vehicle::Galaxy  |
            Vehicle::Lodestar  |
            Vehicle::BastionFleetCarrier  |
            Vehicle::PocketFlash  |
            Vehicle::MosquitoInterceptor  |
            Vehicle::ReaverInterceptor  |
            Vehicle::ScytheInterceptor  |
            Vehicle::JavelinAlt1  |
            Vehicle::SpitfireALt3  |
            Vehicle::JavelinAlt2  |

            Vehicle::ReclaimedSunderer  |
            Vehicle::ReclaimedGalaxy  |
            Vehicle::ReclaimedValkyrie  |
            Vehicle::ReclaimedMagrider  |
            Vehicle::ReclaimedVanguard  |
            Vehicle::ReclaimedProwler => true,

            _ => false



       }

    }

}


#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
#[repr(i64)]
pub enum Class {
    NCInfiltrator = 1,
    NCLightAssault = 3,
    NCMedic = 4,
    NCEngineer = 5,
    NCHeavyAssault = 6,
    NCMax = 7,

    TRInfiltrator = 8,
    TRLightAssault = 10,
    TRMedic = 11,
    TREngineer = 12,
    TRHeavyAssault = 13,
    TRMax = 14,

    VSInfiltrator = 15,
    VSLightAssault = 17,
    VSMedic = 18,
    VSEngineer = 19,
    VSHeavyAssault = 20,
    VSMax = 21,

    NSOInfiltrator = 28,
    NSOLightAssault = 29,
    NSOMedic = 30,
    NSOEngineer = 31,
    NSOHeavyAssault = 32,
    NSOMax = 45,

    #[num_enum(default)]
    Unknown = 0,
}


impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::NCInfiltrator => write!(f, "Infiltrator"),
            Class::NCLightAssault => write!(f, "LightAssault"),
            Class::NCMedic => write!(f, "Medic"),
            Class::NCEngineer => write!(f, "Engineer"),
            Class::NCHeavyAssault => write!(f, "HeavyAssault"),
            Class::NCMax => write!(f, "MAX"),

            Class::TRInfiltrator => write!(f, "Infiltrator"),
            Class::TRLightAssault => write!(f, "LightAssault"),
            Class::TRMedic => write!(f, "Medic"),
            Class::TREngineer => write!(f, "Engineer"),
            Class::TRHeavyAssault => write!(f, "HeavyAssault"),
            Class::TRMax => write!(f, "MAX"),

            Class::VSInfiltrator => write!(f, "Infiltrator"),
            Class::VSLightAssault => write!(f, "LightAssault"),
            Class::VSMedic => write!(f, "Medic"),
            Class::VSEngineer => write!(f, "Engineer"),
            Class::VSHeavyAssault => write!(f, "HeavyAssault"),
            Class::VSMax => write!(f, "MAX"),

            Class::NSOInfiltrator => write!(f, "Infiltrator"),
            Class::NSOLightAssault => write!(f, "LightAssault"),
            Class::NSOMedic => write!(f, "Medic"),
            Class::NSOEngineer => write!(f, "Engineer"),
            Class::NSOHeavyAssault => write!(f, "HeavyAssault"),
            Class::NSOMax => write!(f, "MAX"),

            Class::Unknown => write!(f, "???"),
        }
    }
}

pub fn master_images() -> std::array::IntoIter<(String, u32), 31> {
    [("NC".to_owned(), 12),
    ("TR".into(), 18),
    ("VS".into(), 94),
    ("HeavyAssault".into(), 59),
    ("LightAssault".into(), 62),
    ("Medic".into(), 65),
    ("Engineer".into(), 201),
    ("Infiltrator".into(), 204),
    ("MAX".into(), 207),
    ("Galaxy".into(), 256),
    ("Liberator".into(), 257),
    ("Lightning".into(), 258),
    ("Magrider".into(), 259),
    ("Mosquito".into(), 260),
    ("Prowler".into(), 261),
    ("Flash".into(), 262),
    ("Reaver".into(), 263),
    ("Sunderer".into(), 264),
    ("Vanguard".into(), 265),
    ("Scythe".into(), 266),
    ("Harasser".into(), 8852),
    ("DropPod".into(), 12259),
    ("Valkyrie".into(), 79711),
    ("Spitfire".into(), 82143),
    ("Ant".into(), 84726),
    ("Javelin".into(), 92332),
    ("Colossus".into(), 92799),
    ("Chimera".into(), 93602),
    ("Dervish".into(), 93605),
    ("ManaAITurret".into(), 12260),
    ("Orbital".into(), 86740),
    /*
    ("".into(), ),
    */
    ].into_iter()
}
