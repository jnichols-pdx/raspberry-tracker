use crate::db::*;
use num_enum::FromPrimitive;
use std::io::Read;
use std::ops::Sub;
use time::{Date, Duration};

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
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

impl std::fmt::Display for Faction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Faction::VS => write!(f, "VS"),
            Faction::NC => write!(f, "NC"),
            Faction::TR => write!(f, "TR"),
            Faction::NSO => write!(f, "Robit"),
            Faction::Unknown => write!(f, "???"),
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
            World::CN => write!(f, "Connery"),
            World::EM => write!(f, "Emerald"),
            World::ML => write!(f, "Miller"),
            World::CB => write!(f, "Cobalt"),
            World::JA => write!(f, "Jaeger"),
            World::ST => write!(f, "SolTech"),
        }
    }
}

pub fn lookup_character_id(new_char: &str) -> Result<Option<String>, ureq::Error> {
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

pub fn lookup_character_asp(char_id: &str) -> Result<u8, ureq::Error> {
    let resp: serde_json::Value = ureq::get(&*format!("http://census.daybreakgames.com/s:raspberrytracker/get/ps2/character/?character_id={}&c:hide=battle_rank.percent_to_next,certs,profile_id,times,title_id,daily_ribbon,battle_rank,name,faction_id,head_id",
        char_id))
        .call()?
        .into_json()?;

    Ok(resp["character_list"][0]["prestige_level"]
        .to_string()
        .unquote()
        .parse::<u8>()
        .unwrap_or(0))
}

pub fn lookup_new_char_details(new_id: &str) -> Result<serde_json::Value, ureq::Error> {
    let resp = ureq::get(&*format!(
        "http://census.daybreakgames.com/s:raspberrytracker/get/ps2/character/?character_id={}&c:hide=battle_rank.percent_to_next,certs,profile_id,times,title_id,daily_ribbon&c:join=outfit_member_extended^show:name'alias^inject_at:outfit,characters_stat^terms:stat_name=weapon_deaths^show:value_forever^inject_at:weapon_deaths,characters_stat_history^terms:stat_name=kills^show:all_time^inject_at:kills&c:resolve=world",
        new_id))
        .call()?
        .into_json()?;

    Ok(resp)
}

pub fn lookup_full_stats(new_id: &str) -> Result<serde_json::Value, ureq::Error> {
    let resp = ureq::get(&*format!(
        "http://census.daybreakgames.com/s:raspberrytracker/get/ps2/single_character_by_id?character_id={}", new_id)).call()?.into_json()?;

    Ok(resp)
}

pub fn subscribe_session_string(character_id: &str) -> String {
    format!("{{\"service\":\"event\",\"action\":\"subscribe\",\"characters\":[{}],\"eventNames\":[\"Death\",\"VehicleDestroy\",\"BattleRankUp\",\"GainExperience\"]}}",
        character_id)
}

pub fn clear_subscribe_session_string() -> String {
    "{\"service\":\"event\",\"action\":\"clearSubscribe\",\"eventNames\":[\"Death\",\"VehicleDestroy\",\"BattleRankUp\",\"GainExperience\"]}".to_owned()
}

pub fn lookup_weapon_name(new_id: &str) -> Result<serde_json::Value, ureq::Error> {
    let resp = ureq::get(&*format!(
        "http://census.daybreakgames.com/s:raspberrytracker/get/ps2/item/?item_id={}",
        new_id
    ))
    .call()?
    .into_json()?;

    Ok(resp)
}

pub fn download_census_image(census_id: u32) -> Result<Option<Vec<u8>>, ureq::Error> {
    let resp = ureq::get(&*format!(
        "http://census.daybreakgames.com/files/ps2/images/static/{}.png",
        census_id
    ))
    .call()?;
    if resp.status() == 200 {
        println!("{:?}", resp);
        let mut image_bytes: Vec<u8> = Vec::with_capacity(1024);
        resp.into_reader()
            .take(5242880)
            .read_to_end(&mut image_bytes)?;
        Ok(Some(image_bytes))
    } else {
        Ok(None)
    }
}

pub fn is_online(char_id: &str) -> Result<bool, ureq::Error> {
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

pub fn relative_date_string(then: &Date, today: &Date) -> String {
    let week_ago = today.sub(Duration::days(7));
    if today == then {
        "Today".to_owned()
    } else if today.previous_day().unwrap() == *then {
        "Yesterday".to_owned()
    } else if *then > week_ago {
        then.weekday().to_string()
    } else {
        small_date_format(then)
    }
}

pub fn small_date_format(then: &Date) -> String {
    let format = time::format_description::parse("[year]-[month]-[day]").unwrap();
    then.format(&format).unwrap()
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
        self[1..self.len() - 1].to_owned()
    }
}

pub trait TextureLookup {
    fn texture_by_name(&self, name: &str) -> Option<egui::TextureHandle>;
}

#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
#[repr(i64)]
pub enum EventType {
    Death = 1,
    Kill = 2,
    TeamKill = 3,
    TeamDeath = 4,
    Suicide = 5,
    DestroyVehicle = 6,
    LoseVehicle = 7,
    DestroyVehicleFF = 8,
    LoseVehicleFF = 9,

    #[num_enum(default)]
    Unknown = 0,
}

#[allow(clippy::enum_variant_names)]
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
    Chimera = 2137, //found by empirical testing
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
    Lodestar = 2140, //found by empirical testing.
    BastionFleetCarrier = 2019,

    AIPhalanxTurret = 100,
    ManaAITurret = 101,
    ManaAVTurret = 102,
    Spitfire = 103,     //WORKS for standard spitty.
    SpitfireAlt1 = 104, //is one of these aux spitty?
    SpitfireAlt2 = 105,
    AAPhalanxTurret = 150,
    AVPhalanxTurret = 151, //connfirmed to be prebuilt base turret
    AVBuilderTower = 160,  //THIS appears to be correct, these are the towers
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
    NoVehicle = 0,

    Unknown = -1,
}

impl std::fmt::Display for Vehicle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vehicle::Flash => write!(f, "Flash"),
            Vehicle::Javelin => write!(f, "Javelin"),
            Vehicle::Harasserr => write!(f, "Harasser"),
            Vehicle::Sunderer => write!(f, "Sunderer"),
            Vehicle::Lightning => write!(f, "Lightning"),
            Vehicle::Prowler => write!(f, "Prowler"),
            Vehicle::Vanguard => write!(f, "Vanguard"),
            Vehicle::Magrider => write!(f, "Magrider"),
            Vehicle::Chimera => write!(f, "Chimera"),
            Vehicle::Colossus => write!(f, "Colossus"),
            Vehicle::Ant => write!(f, "Ant"),
            Vehicle::Deliverer => write!(f, "Deliverer"),

            Vehicle::DropPod => write!(f, "DropPod"),
            Vehicle::Mosquito => write!(f, "Mosquito"),
            Vehicle::Scythe => write!(f, "Scythe"),
            Vehicle::Reaver => write!(f, "Reaver"),
            Vehicle::Dervish => write!(f, "Dervish"),
            Vehicle::Valkyrie => write!(f, "Valkyrie"),
            Vehicle::Wasp => write!(f, "Wasp"),
            Vehicle::Liberator => write!(f, "Liberator"),
            Vehicle::Galaxy => write!(f, "Galaxy"),
            Vehicle::Lodestar => write!(f, "Lodestar"),
            Vehicle::BastionFleetCarrier => write!(f, "BastionFleetCarrier"),

            Vehicle::AIPhalanxTurret => write!(f, "AIPhalanxTurret"),
            Vehicle::ManaAITurret => write!(f, "ManaAITurret"),
            Vehicle::ManaAVTurret => write!(f, "ManaAVTurret"),
            Vehicle::Spitfire => write!(f, "Spitfire"),
            Vehicle::SpitfireAlt1 => write!(f, "Spitfire1"),
            Vehicle::SpitfireAlt2 => write!(f, "Spitfire2"),
            Vehicle::AAPhalanxTurret => write!(f, "AAPhalanxTurret"),
            Vehicle::AVPhalanxTurret => write!(f, "AVPhalanxTurret"),
            Vehicle::AVBuilderTower => write!(f, "AVBuilderTower"),
            Vehicle::AABuilderTower => write!(f, "AABuilderTower"),
            Vehicle::AIBuilderTower => write!(f, "AIBuilderTower"),

            Vehicle::Glaive => write!(f, "Glaive"),
            Vehicle::AVPhalanxTurretAlt => write!(f, "AVPhalanxTurret1"),
            Vehicle::DropPodALt => write!(f, "Droppod1"),
            Vehicle::AIPhalanxTurretAlt => write!(f, "AIPhalanxTurret1"),
            Vehicle::PocketFlash => write!(f, "PocketFlash"),
            Vehicle::Flail => write!(f, "Flail"),

            Vehicle::MosquitoInterceptor => write!(f, "Mossy-Interceptor"),
            Vehicle::ReaverInterceptor => write!(f, "Reaver-Interceptor"),
            Vehicle::ScytheInterceptor => write!(f, "Scythe-Interceptor"),
            Vehicle::JavelinAlt1 => write!(f, "Javelin1"),
            Vehicle::SpitfireALt3 => write!(f, "Spitfire3"),
            Vehicle::JavelinAlt2 => write!(f, "Javelin2"),

            Vehicle::ReclaimedSunderer => write!(f, "Sunderer-Reclaimed"),
            Vehicle::ReclaimedGalaxy => write!(f, "Galaxy-Reclaimed"),
            Vehicle::ReclaimedValkyrie => write!(f, "Valkyrie-Reclaimed"),
            Vehicle::ReclaimedMagrider => write!(f, "Magrider-Reclaimed"),
            Vehicle::ReclaimedVanguard => write!(f, "Vanguard-Reclaimed"),
            Vehicle::ReclaimedProwler => write!(f, "Prowler-Reclaimed"),

            Vehicle::Pumpkin => write!(f, "Pumpkin"),

            Vehicle::NoVehicle => write!(f, "NONE"),
            Vehicle::Unknown => write!(f, "???"),
        }
    }
}

impl Vehicle {
    pub fn is_true_vehicle(&self) -> bool {
        matches!(
            self,
            Vehicle::Flash
                | Vehicle::Javelin
                | Vehicle::Harasserr
                | Vehicle::Sunderer
                | Vehicle::Lightning
                | Vehicle::Prowler
                | Vehicle::Vanguard
                | Vehicle::Magrider
                | Vehicle::Chimera
                | Vehicle::Colossus
                | Vehicle::Ant
                | Vehicle::Deliverer
                | Vehicle::DropPod
                | Vehicle::Mosquito
                | Vehicle::Scythe
                | Vehicle::Reaver
                | Vehicle::Dervish
                | Vehicle::Valkyrie
                | Vehicle::Wasp
                | Vehicle::Liberator
                | Vehicle::Galaxy
                | Vehicle::Lodestar
                | Vehicle::BastionFleetCarrier
                | Vehicle::PocketFlash
                | Vehicle::MosquitoInterceptor
                | Vehicle::ReaverInterceptor
                | Vehicle::ScytheInterceptor
                | Vehicle::JavelinAlt1
                | Vehicle::SpitfireALt3
                | Vehicle::JavelinAlt2
                | Vehicle::ReclaimedSunderer
                | Vehicle::ReclaimedGalaxy
                | Vehicle::ReclaimedValkyrie
                | Vehicle::ReclaimedMagrider
                | Vehicle::ReclaimedVanguard
                | Vehicle::ReclaimedProwler
        )
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
    [
        ("NC".to_owned(), 12),
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
    ]
    .into_iter()
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
#[repr(i64)]
pub enum WeaponType {
    Knife = 2,
    Pistol = 3,
    Shotgun = 4,
    SMG = 5,
    LMG = 6,
    AR = 7,
    Carbine = 8,
    AV_MAX_L = 9,
    AI_MAX_L = 10,
    Sniper_Rifle = 11,
    Scout_Rifle = 12,
    Rocket_Launcher = 13,
    Heavy_Weapon = 14,
    MAX_Flamethrower = 15,
    MAX_Flak = 16,
    Grenade = 17,
    Explosive = 18,
    Battle_Rifle = 19,
    AA_MAX_R = 20,
    AV_MAX_R = 21,
    AI_MAX_R = 22,
    AA_MAX_L = 23,
    Crossbow = 24,
    Flash_Primary = 109,
    Galaxy_L = 110,
    Galaxzxy_Tail = 111,
    Galaxy_R = 112,
    Galaxy_Top = 113,
    Harasser_Top = 114,
    Liberator_Belly = 115,
    Liberator_Nose = 116,
    Liberator_Tail = 117,
    Lightning_Primary = 118,
    Magrider_Top = 119,
    Magrider_Primary = 120,
    Mosquito_Nose = 121,
    Mosquito_Wingmount = 122,
    Prowler_Top = 123,
    Prowler_Primary = 124,
    Reaver_Nose = 125,
    Reaver_Wingmoutn = 126,
    Scythe_Nose = 127,
    Scythe_Wingmount = 128,
    Sunderer_Front = 129,
    Sunderer_Rear = 130,
    Vanguard_Top = 131,
    Vanguard_Primary = 132,
    Valkyrie_Nose = 138,
    Ant_Top = 144,
    Rocklet_Rifle = 147,

    //Kuwa Only - hybrid in that it can be used by Engineers, Medics AND heavy assaults.
    Hybrid_Rifle = 157,

    Bastion_AA_Turret = 208,
    Bastion_Pilot_Bombard = 209,
    Bastion_Missiles = 210,
    Colossus_Primary = 211,
    ColossuS_FR = 212,
    ColossuS_FL = 213,
    ColossuS_RR = 214,
    ColossuS_RL = 215,

    //Not available in Census: Dervish, Chimera, Javelin specific weapon Category IDs.
    #[num_enum(default)]
    Unknown = 0,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
#[repr(i64)]
pub enum ExperienceType {
    Kill_Player = 1,
    Kill_Player_Assist = 2,
    Kill_Player_Spawn_Assist = 3,
    Heal_Player = 4,
    Heal_Assist = 5,
    MAX_Repair = 6,
    Revive = 7,
    Kill_Streak = 8,
    Domination_Kill = 10,
    Revenge_Kill = 11,
    Control_Point_Defend = 15,
    Control_Point_Attack = 16,
    Facility_Captured_Not_Used = 19,
    Destroy_Secondary_Objective = 21,
    Destroy_SecondaryObjectiveAssist = 22,
    Vehicle_Destruction_Flash = 24,
    Multiple_Kill = 25,
    Vehicle_RoadKill = 26,
    Squad_Repair_Flash = 28,
    Kill_Player_Class_MAX = 29,
    Transport_Assist = 30,
    Vehicle_Repair_Flash = 31,
    Nemesis_Kill = 32,
    Resupply_Player = 34,
    Spot_Kill = 36,
    Headshot = 37,
    Stop_Kill_Streak = 38,
    Meta_Game_Event_reward = 47,
    Squad_Heal = 51,
    Squad_Revive = 53,
    Squad_Spot_Kill = 54,
    Squad_Resupply = 55,
    Squad_Spawn = 56,
    Destroy_Engineer_Turret = 57,
    Vehicle_Destruction_Phalanx = 58,
    Vehicle_Destruction_Drop_Pod = 59,
    Vehicle_Destruction_Galaxy = 60,
    Vehicle_Destruction_Liberator = 61,
    Vehicle_Destruction_Lightning = 62,
    Vehicle_Destruction_Magrider = 63,
    Vehicle_Destruction_Mosquito = 64,
    Vehicle_Destruction_Prowler = 65,
    Vehicle_Destruction_Reaver = 66,
    Vehicle_Destruction_Scythe = 67,
    Vehicle_Destruction_Sunderer = 68,
    Vehicle_Destruction_Vanguard = 69,
    Vehicle_Ram_Bonus = 72,
    Vehicle_Ram_Kill_Engi_Turret = 73,
    Vehicle_Ram_Kill_Phalanx = 74,
    Vehicle_Ram_Kill_Drop_Pod = 75,
    Vehicle_Ram_Kill_Galaxy = 76,
    Vehicle_Ram_Kill_Liberator = 77,
    Vehicle_Ram_Kill_Lightning = 78,
    Vehicle_Ram_Kill_Magrider = 79,
    Vehicle_Ram_Kill_Mosquito = 80,
    Vehicle_Ram_Kill_Prowler = 81,
    Vehicle_Ram_Kill_Reaver = 82,
    Vehicle_Ram_Kill_Scythe = 83,
    Vehicle_Ram_Kill_Sunderer = 84,
    Vehicle_Ram_Kill_Vanguard = 85,
    Explosive_Destruction = 86,
    Secondary_Facility_Object_Repair = 87,
    Vehicle_Repair_Engi_Turret = 88,
    Vehicle_Repair_Phalanx = 89,
    Vehicle_Repair_Drop_Pod = 90,
    Vehicle_Repair_Galaxy = 91,
    Vehicle_Repair_Liberator = 92,
    Vehicle_Repair_Lightning = 93,
    Vehicle_Repair_Magrider = 94,
    Vehicle_Repair_Mosquito = 95,
    Vehicle_Repair_Prowler = 96,
    Vehicle_Repair_Reaver = 97,
    Vehicle_Repair_Scythe = 98,
    Vehicle_Repair_Sunderer = 99,
    Vehicle_Repair_Vanguard = 100,
    Kill_Assist_Flash = 101,
    Kill_Assist_Engi_Turret = 102,
    Kill_Assist_Phalanx = 103,
    Kill_Assist_Drop_Pod = 104,
    Kill_Assist_Galaxy = 105,
    Kill_Assist_Liberator = 106,
    Kill_Assist_Lightning = 107,
    Kill_Assist_Magrider = 108,
    Kill_Assist_Mosquito = 109,
    Kill_Assist_Prowler = 110,
    Kill_Assist_Reaver = 111,
    Kill_Assist_Scythe = 112,
    Kill_Assist_Sunderer = 113,
    Kill_Assist_Vanguard = 114,
    Squad_Repair_Engi_Turret = 129,
    Squad_Repair_Phalanx = 130,
    Squad_Repair_Drop_Pod = 131,
    Squad_Repair_Galaxy = 132,
    Squad_Repair_Liberator = 133,
    Squad_Repair_Lightning = 134,
    Squad_Repair_Magrider = 135,
    Squad_Repair_Mosquito = 136,
    Squad_Repair_Prowler = 137,
    Squad_Repair_Reaver = 138,
    Squad_Repair_Scythe = 139,
    Squad_Repair_Sunderer = 140,
    Squad_Repair_Vanguard = 141,
    Squad_MAX_Repair = 142,
    Drop_Pod_Kill = 143,
    Player_Kill_by_Sunderer_Gunner = 146,
    Player_Kill_by_Magrider_Gunner = 148,
    Player_Kill_by_Vanguard_Gunner = 149,
    Player_Kill_by_Prowler_Gunner = 150,
    Player_Kill_by_Liberator_Gunner = 154,
    Player_Kill_by_Galaxy_Gunner = 155,
    Flash_Kill_by_Sunderer_Gunner = 159,
    Sunderer_Kill_by_Sunderer_Gunner = 160,
    Lightning_Kill_by_Sunderer_Gunner = 161,
    Magrider_Kill_by_Sunderer_Gunner = 162,
    Vanguard_Kill_by_Sunderer_Gunner = 163,
    Prowler_Kill_by_Sunderer_Gunner = 164,
    Scythe_Kill_by_Sunderer_Gunner = 165,
    Reaver_Kill_by_Sunderer_Gunner = 166,
    Mosquito_Kill_by_Sunderer_Gunner = 167,
    Lib_Kill_by_Sunderer = 168,
    Galaxy_Kill_by_Sunderer_Gunner = 169,
    Flash_Kill_by_Magrider_Gunner = 181,
    Sunderer_Kill_by_Magrider_Gunner = 182,
    Lightning_Kill_by_Magrider_Gunner = 183,
    Vanguard_Kill_by_Magrider_Gunner = 184,
    Prowler_Kill_by_Magrider_Gunner = 185,
    Reaver_Kill_by_Magrider_Gunner = 186,
    Mosquito_Kill_by_Magrider_Gunner = 187,
    Lib_Kill_by_Magrider = 188,
    Galaxy_Kill_by_Magrider_Gunner = 189,
    Flash_Kill_by_Vanguard_Gunner = 190,
    Sunderer_Kill_by_Vanguard_Gunner = 191,
    Lightning_Kill_by_Vanguard_Gunner = 192,
    Magrider_Kill_by_Vanguard_Gunner = 193,
    Prowler_Kill_by_Vanguard_Gunner = 195,
    Scythe_Kill_by_Vanguard_Gunner = 196,
    Mosquito_Kill_by_Vanguard_Gunner = 197,
    Lib_Kill_by_Vanguard = 198,
    Galaxy_Kill_by_Vanguard_Gunner = 199,
    Flash_Kill_by_Prowler_Gunner = 200,
    Galaxy_Spawn_Bonus = 201,
    Sunderer_Kill_by_Prowler_Gunner = 202,
    Lightning_Kill_by_Prowler_Gunner = 203,
    Magrider_Kill_by_Prowler_Gunner = 204,
    Vanguard_Kill_by_Prowler_Gunner = 205,
    Scythe_Kill_by_Prowler_Gunner = 207,
    Reaver_Kill_by_Prowler_Gunner = 208,
    Liberator_Kill_by_Prowler_Gunner = 209,
    Galaxy_Kill_by_Prowler_Gunner = 210,
    Flash_Kill_by_Liberator_Gunner = 211,
    Sunderer_Kill_by_Lib_Gunner = 212,
    Lightning_Kill_by_Liberator_Gunner = 213,
    Magrider_Kill_by_Lib_Gunner = 214,
    Vanguard_Kill_by_Lib_Gunner = 215,
    Prowler_Kill_by_Liberator_Gunner = 216,
    Scythe_Kill_by_Liberator_Gunner = 217,
    Reaver_Kill_by_Liberator_Gunner = 218,
    Mosquito_Kill_by_Lib_Gunner = 219,
    Lib_Kill_by_Liberator = 220,
    Galaxy_Kill_by_Liberator_Gunner = 221,
    Flash_Kill_by_Galaxy_Gunner = 222,
    Sunderer_Kill_by_Galaxy_Gunner = 223,
    Lightning_Kill_by_Galaxy_Gunner = 224,
    Magrider_Kill_by_Galaxy_Gunner = 225,
    Vanguard_Kill_by_Galaxy_Gunner = 226,
    Prowler_Kill_by_Galaxy_Gunner = 227,
    Scythe_Kill_by_Galaxy_Gunner = 228,
    Reaver_Kill_by_Galaxy_Gunner = 229,
    Mosquito_Kill_by_Galaxy_Gunner = 230,
    LibKill_by_Galaxy_Gunner = 231,
    Galaxy_Kill_by_Galaxy_Gunner = 232,
    Sunderer_Spawn_Bonus = 233,
    Facility_placed_bomb = 234,
    Facility_defused_bomb = 235,
    Facility_Terminal_Hack = 236,
    Facility_Turret_Hack = 237,
    Vehicle_Resupply = 240,
    Squad_Vehicle_Resupply = 241,
    Spot_Kill_Flash = 242,
    Spot_Kill_Engi_Turret = 243,
    Spot_Kill_Phalanx = 244,
    Spot_Kill_Drop_Pod = 245,
    Spot_Kill_Galaxy = 246,
    Spot_Kill_Liberator = 247,
    Spot_Kill_Lightning = 248,
    Spot_Kill_Magrider = 249,
    Spot_Kill_Mosquito = 250,
    Spot_Kill_Prowler = 251,
    Spot_Kill_Reaver = 252,
    Spot_Kill_Scythe = 253,
    Spot_Kill_Sunderer = 254,
    Spot_Kill_Vanguard = 255,
    Squad_Spot_Kill_Flash = 256,
    Squad_Spot_Kill_Engi_Turret = 257,
    Squad_Spot_Kill_Phalanx = 258,
    Squad_Spot_Kill_Drop_Pod = 259,
    Squad_Spot_Kill_Galaxy = 260,
    Squad_Spot_Kill_Liberator = 261,
    Squad_Spot_Kill_Lightning = 262,
    Squad_Spot_Kill_Magrider = 263,
    Squad_Spot_Kill_Mosquito = 264,
    Squad_Spot_Kill_Prowler = 265,
    Squad_Spot_Kill_Reaver = 266,
    Squad_Spot_Kill_Scythe = 267,
    Squad_Spot_Kill_Sunderer = 268,
    Squad_Spot_Kill_Vanguard = 269,
    Squad_Spawn_Beacon_Kill = 270,
    Convert_Capture_Point = 272,
    Terminal_Kill = 275,
    Terminal_Repair = 276,
    Spawn_Kill = 277,
    Priority_Kill = 278,
    High_Priority_Kill = 279,
    Lightning_Damage_Infantry_vs_Vehicle = 280,
    Prowler_Damage_Infantry_vs_Vehicle = 281,
    Galaxy_Damage = 283,
    Liberator_Damage = 284,
    Magrider_Damage_Infantry_vs_Vehicle = 285,
    Mosquito_Damage = 286,
    Reaver_Damage = 287,
    Scythe_Damage = 288,
    Sunderer_Damage_Infantry_vs_Vehicle = 289,
    Vanguard_Damage_Infantry_vs_Vehicle = 290,
    Ribbon_Experience = 291,
    Motion_Detect = 293,
    Squad_Motion_Spot = 294,
    Vehicle_Ram_Kill_Harasser = 300,
    Vehicle_Destruction_Harasser = 301,
    Squad_Repair_Harasser = 302,
    Vehicle_Repair_Harasser = 303,
    Kill_Assist_Harasser = 304,
    Spot_Kill_Harasser = 306,
    Squad_Spot_Kill_Harasser = 307,
    Harasser_Kill_by_Sunderer_Gunner = 308,
    Harasser_Kill_by_Magrider_Gunner = 309,
    Harasser_Kill_by_Vanguard_Gunner = 310,
    Harasser_Kill_by_Prowler_Gunner = 311,
    Harasser_Kill_by_Liberator_Gunner = 312,
    Harasser_Kill_by_Galaxy_Gunner = 313,
    Player_Kill_by_Harasser_Gunner = 314,
    Flash_Kill_by_Harasser_Gunner = 315,
    Sunderer_Kill_by_Harasser_Gunner = 316,
    Lightning_Kill_by_Harasser_Gunner = 317,
    Vanguard_Kill_by_Harasser_Gunner = 318,
    Prowler_Kill_by_Harasser_Gunner = 319,
    Reaver_Kill_by_Harasser_Gunner = 320,
    Mosquito_Kill_by_Harasser_Gunner = 321,
    Lib_Kill_by_Harasser = 322,
    Galaxy_Kill_by_Harasser_Gunner = 323,
    Harasser_Kill_by_Harasser_Gunner = 324,
    Magrider_Kill_by_Harasser_Gunner = 325,
    Scythe_Kill_by_Harasser_Gunner = 326,
    Tank_Mine_Despawn_or_Defusal = 327,
    Alert_Reward = 328,
    Tank_Hunter_Bonus_Prowler_Kill = 329,
    Tank_Hunter_Bonus_Magrider_Kil = 330,
    Dogfighter_Bonus_Mosquito_Kill = 331,
    Dogfighter_Bonus_Reaver_Kill = 332,
    Dogfighter_Bonus_Scythe_Kill = 333,
    Tank_Hunter_Bonus_Vanguard_Kil = 334,
    Savior_Kill_Non_MAX = 335,
    Saved = 336,
    Holiday_Event_NPC_Kill = 337,
    Holiday_Event_NPC_Gold_Kill = 338,
    Snowman_Kill_by_Sunderer_Gunner = 339,
    Snowman_Kill_by_Magrider_Gunner = 340,
    Snowman_Kill_by_Vanguard_Gunner = 341,
    Snowman_Kill_by_Prowler_Gunner = 342,
    Snowman_Kill_by_Liberator_Gunner = 343,
    Snowman_Kill_by_Galaxy_Gunner = 344,
    Snowman_Kill_by_Harasser_Gunner = 345,
    GSnowman_Kill_by_Sunder_Gunner = 346,
    GSnowman_Kill_by_Mag_Gunner = 347,
    GSnowman_Kill_by_Vang_Gunner = 348,
    GSnowman_Kill_by_Prow_Gunner = 349,
    GSnowman_Kill_by_Lib_Gunner = 350,
    GSnowman_Kill_by_Gal_Gunner = 351,
    GSnowman_Kill_by_Haras_Gunner = 352,
    Scout_Radar_Detect = 353,
    Squad_Scout_Radar_Detect = 354,
    Squad_Vehicle_Spawn_Bonus = 355,
    Vehicle_Ram_Kill_R_Drone = 356,
    Vehicle_Destruction_R_Drone = 357,
    Squad_Repair_R_Drone = 358,
    Vehicle_Repair_R_Drone = 359,
    Kill_Assist_R_Drone = 360,
    R_Drone_Kill_by_Harasser_Gunner = 362,
    R_Drone_Kill_by_Sunderer_Gunner = 363,
    R_Drone_Kill_by_Magrider_Gunner = 364,
    R_Drone_Kill_by_Prowler_Gunner = 365,
    R_Drone_Kill_by_Lib_Gunner = 366,
    R_Drone_Kill_by_Galaxy_Gunner = 367,
    Spot_Kill_R_Drone = 368,
    Squad_Spot_Kill_R_Drone = 369,
    Motion_Sensor_Spotter_Kill = 370,
    Kill_Player_Priority_Assist = 371,
    Kill_Player_High_Priority_Assist = 372,
    Shield_Regen_Tool_Kill = 437,
    Shield_Repair = 438,
    Squad_Shield_Repair = 439,
    Chain_Expl_Assist_Infantry = 440,
    Chain_Expl_Assist_Flash = 441,
    Vehicle_Destruction_Valkyrie = 501,
    Vehicle_Ram_Kill_Valkyrie = 502,
    Vehicle_Repair_Valkyrie = 503,
    Kill_Assist_Valkyrie = 504,
    Squad_Repair_Valkyrie = 505,
    Spot_Kill_Valkyrie = 506,
    Squad_Spot_Kill_Valkyrie = 507,
    Valkyrie_Damage = 508,
    Valkyrie_Kill_by_Sunderer_Gunner = 509,
    Valkyrie_Kill_by_Magrider_Gunner = 510,
    Valkyrie_Kill_by_Vanguard_Gunner = 511,
    Valkyrie_Kill_by_Prowler_Gunner = 512,
    Valkyrie_Kill_by_Liberator_Gunner = 513,
    Valkyrie_Kill_by_Galaxy_Gunner = 514,
    Player_Kill_by_Valkyrie_Gunner = 515,
    Flash_Kill_by_Valkyrie_Gunner = 520,
    Sunderer_Kill_by_Valkyrie_Gunner = 521,
    Lightning_Kill_by_Valkyrie_Gunner = 522,
    Vanguard_Kill_by_Valkyrie_Gunner = 523,
    Prowler_Kill_by_Valkyrie_Gunner = 524,
    Reaver_Kill_by_Valkyrie_Gunner = 525,
    Mosquito_Kill_by_Valkyrie_Gunner = 526,
    Lib_Kill_by_Valkyrie = 527,
    Galaxy_Kill_by_Valkyrie_Gunner = 528,
    Magrider_Kill_by_Valkyrie_Gunner = 529,
    Scythe_Kill_by_Valkyrie_Gunner = 530,
    Snowman_Kill_by_Valkyrie_Gunner = 531,
    R_Drone_Kill_by_Valkyrie_Gunner = 532,
    Valkyrie_Kill_by_Valkyrie_Gunner = 533,
    Chain_Expl_Assist_Phalanx = 534,
    Chain_Expl_Assist_Drop_Pod = 535,
    Chain_Expl_Assist_Galaxy = 536,
    Chain_Expl_Assist_Liberator = 537,
    Chain_Expl_Assist_Lightning = 538,
    Chain_Expl_Assist_Magrider = 539,
    Chain_Expl_Assist_Mosquito = 540,
    Chain_Expl_Assist_Prowler = 541,
    Chain_Expl_Assist_Reaver = 542,
    Chain_Expl_Assist_Scythe = 543,
    Chain_Expl_Assist_Sunderer = 544,
    Chain_Expl_Assist_Vanguard = 545,
    Chain_Expl_Assist_Harasser = 546,
    Chain_Expl_Assist_R_Drone = 547,
    Chain_Expl_Assist_Valkyrie = 548,
    Concussion_Grenade_Assist = 550,
    Concussion_Grenade_Squad_Assist = 551,
    EMP_Grenade_Assist = 552,
    EMP_Grenade_Squad_Assist = 553,
    Flashbang_Assist = 554,
    Flashbang_Squad_Assist = 555,
    Objective_Pulse_Defend = 556,
    Objective_Pulse_Capture = 557,
    Halloween_Event_NPC_GreatP_Kill = 558,
    Pumpkin_Kill_by_Valkyrie_Gunner = 559,
    Pumpkin_Kill_by_Sunderer_Gunner = 560,
    Pumpkin_Kill_by_Magrider_Gunner = 561,
    Pumpkin_Kill_by_Vanguard_Gunner = 562,
    Pumpkin_Kill_by_Prowler_Gunner = 563,
    Pumpkin_Kill_by_Liberator_Gunner = 564,
    Pumpkin_Kill_by_Galaxy_Gunner = 565,
    Pumpkin_Kill_by_Harasser_Gunner = 566,
    GPumpkin_Kill_by_Sunder_Gunner = 567,
    GPumpkin_Kill_by_Mag_Gunner = 568,
    GPumpkin_Kill_by_Vang_Gunner = 569,
    GPumpkin_Kill_by_Prow_Gunner = 570,
    GPumpkin_Kill_by_Lib_Gunner = 571,
    GPumpkin_Kill_by_Gal_Gunner = 572,
    GPumpkin_Kill_by_Haras_Gunner = 573,
    Halloween_Event_NPC_Kill = 574,
    Harasser_Kill_by_Valkyrie_Gunner = 575,
    Valkyrie_Kill_by_Harasser_Gunner = 576,
    Snowman_kill_by_squad_member = 577,
    Gsnowman_kill_by_squad_member = 578,
    Destroy_Spitfire_Turret = 579,
    Vehicle_Ram_Kill_Spitfire_Turret = 580,
    Vehicle_Repair_Spitfire_Turret = 581,
    Kill_Assist_Spitfire_Turret = 582,
    Squad_Repair_Spitfire_Turret = 584,
    Spot_Kill_Spitfire_Turret = 585,
    Squad_Spot_Kill_Spitfire_Turret = 586,
    Kill_Dummy_NPC = 591,
    Savior_Kill_MAX = 592,
    Bounty_Kill_Bonus = 593,
    Bounty_Kill_Cashed_In = 594,
    Bounty_Kill_Streak = 595,
    Membership_bonus_xp = 596,
    Victory_Point_XP = 597,
    Continent_Capture_XP = 598,
    Victory_Point_XP25_percent_XP = 599,
    Victory_Point_XP50_percent_XP = 600,
    Victory_Point_XP75_percent_XP = 601,
    Victory_Point_XP_plus_50_percent_XP = 602,
    Vehicle_Ram_Kill_Construction_Med = 603,
    Kill_or_Destroy_Construction_Med = 604,
    Squad_Repair_Construction_Med = 605,
    Repair_Construction_Med = 606,
    KillAssist_Construction_Med = 607,
    Spot_Kill_Construction_Med = 609,
    Squad_Spot_Kill_Construction_Med = 610,
    Vehicle_Ram_Kill_Construction_Small = 615,
    Kill_or_Destroy_Construction_Small = 616,
    Squad_Repair_Construction_Small = 617,
    Repair_Construction_Small = 618,
    KillAssist_Construction_Small = 619,
    Spot_Kill_Construction_Small = 621,
    Squad_Spot_Kill_Construction_Small = 622,
    Vehicle_Ram_Kill_Construction_Large = 627,
    Kill_or_Destroy_Construction_Large = 628,
    Squad_Repair_Construction_Large = 629,
    Repair_Construction_Large = 630,
    KillAssist_Construction_Large = 631,
    Spot_Kill_Construction_Large = 633,
    Squad_Spot_Kill_Construction_Large = 634,
    Vehicle_Ram_Kill_Construction_Core = 639,
    Kill_or_Destroy_Construction_Core = 640,
    Squad_Repair_Construction_Core = 641,
    Repair_Construction_Core = 642,
    KillAssist_Construction_Core = 643,
    Spot_Kill_Construction_Core = 645,
    Squad_Spot_Kill_Construction_Core = 646,
    Vehicle_Destruction_ANT = 651,
    Vehicle_Ram_Kill_ANT = 652,
    Vehicle_Repair_ANT = 653,
    Kill_Assist_ANT = 654,
    Squad_Repair_ANT = 656,
    ANT_Kill_by_ANT_Gunner = 657,
    ANT_Kill_by_Magrider_Gunner = 658,
    ANT_Kill_by_Vanguard_Gunner = 659,
    ANT_Kill_by_Prowler_Gunner = 660,
    ANT_Kill_by_Lib_Gunner = 661,
    ANT_Kill_by_Galaxy_Gunner = 662,
    Spot_Kill_ANT = 663,
    Squad_Spot_Kill_ANT = 664,
    ANT_Damage_Infantry_vs_Vehicle = 665,
    ANT_Kill_by_Harasser_Gunner = 666,
    ANT_Kill_by_Valkyrie_Gunner = 671,
    Chain_Expl_Assist_ANT = 672,
    Bounty_Kill_Cashed_In_Alt = 673,
    Cortium_Harvest = 674,
    Cortium_Deposit = 675,
    Flash_Kill_by_ANT_Gunner = 676,
    Galaxy_Kill_by_ANT_Gunner = 677,
    Harasser_Kill_by_ANT_Gunner = 678,
    Magrider_Kill_by_ANT_Gunner = 679,
    Mosquito_Kill_by_ANT_Gunner = 680,
    Player_Kill_by_ANT_Gunner = 681,
    Prowler_Kill_by_ANT_Gunner = 682,
    Pumpkin_Kill_by_ANT_Gunner = 683,
    R_Drone_Kill_by_ANT_Gunner = 684,
    Reaver_Kill_by_ANT_Gunner = 685,
    Scythe_Kill_by_ANT_Gunner = 686,
    Snowman_Kill_by_ANT_Gunner = 687,
    Sunderer_Kill_by_ANT_Gunner = 688,
    Valkyrie_Kill_by_ANT_Gunner = 689,
    Vanguard_Kill_by_ANT_Gunner = 690,
    /*
    Kill_Player_HIVE_XP_Source = 693,
    Kill_Player_HIVE_XP_Target = 694,
    Kill_Player_Assist_HIVE_Source = 695,
    Kill_Spawn_Ass_HIVE_Source = 696,
    Heal_Player_HIVE_Source = 697,
    MAX_Repair_HIVE_Source = 698,
    Revive_HIVE_Source = 699,
    Destroy_SecObj_HIVE_Source = 701,
    Destroy_SecAss_HIVE_Source = 702,
    Vehicle_Dest_Flash_HIVE_Source = 703,
    Transport_Assist_HIVE_Source = 705,
    Vehicle_Repair_Flash_HIVE_Source = 706,
    Resupply_Player_HIVE_Source = 707,
    Kill_Player_Assist_HIVE_XP_Target = 708,
    Kill_Spawn_Ass_HIVE_XP_Target = 709,
    Heal_Player_HIVE_XP_Target = 710,
    MAX_Repair_HIVE_XP_Target = 711,
    Revive_HIVE_XP_Target = 712,
    Destroy_SecObj_HIVE_XP_Target = 714,
    Destroy_SecAss_HIVE_XP_Target = 715,
    Vehicle_Dest_Flash_HIVE_XP_Target = 716,
    Transport_Assist_HIVE_XP_Target = 718,
    Vehicle_Repair_Flash_HIVE_XP_Target = 719,
    Resupply_Player_HIVE_XP_Target = 720,
    Destroy_Engineer_Turret_HIVE_XP_Source = 721,
    Vehicle_Destruction_Phalanx_HIVE_XP_Source = 722,
    Vehicle_Destruction_Drop_Pod_HIVE_XP_Source = 723,
    Vehicle_Destruction_Galaxy_HIVE_XP_Source = 724,
    Vehicle_Destruction_Liberator_HIVE_XP_Source = 725,
    Vehicle_Destruction_Lightning_HIVE_XP_Source = 726,
    Vehicle_Destruction_Magrider_HIVE_XP_Source = 727,
    Vehicle_Destruction_Mosquito_HIVE_XP_Source = 728,
    Vehicle_Destruction_Prowler_HIVE_XP_Source = 729,
    Vehicle_Destruction_Reaver_HIVE_XP_Source = 730,
    Vehicle_Destruction_Scythe_HIVE_XP_Source = 731,
    Vehicle_Destruction_Sunderer_HIVE_XP_Source = 732,
    Vehicle_Destruction_Vanguard_HIVE_XP_Source = 733,
    Vehicle_Repair_Engi_Turret_HIVE_XP_Source = 734,
    Vehicle_Repair_Phalanx_HIVE_XP_Source = 735,
    Vehicle_Repair_Drop_Pod_HIVE_XP_Source = 736,
    Vehicle_Repair_Galaxy_HIVE_XP_Source = 737,
    Vehicle_Repair_Liberator_HIVE_XP_Source = 738,
    Vehicle_Repair_Lightning_HIVE_XP_Source = 739,
    Vehicle_Repair_Magrider_HIVE_XP_Source = 740,
    Destroy_Engineer_Turret_HIVE_XP_Target = 741,
    Vehicle_Destruction_Phalanx_HIVE_XP_Target = 742,
    Vehicle_Destruction_Drop_Pod_HIVE_XP_Target = 743,
    Vehicle_Destruction_Galaxy_HIVE_XP_Target = 744,
    Vehicle_Destruction_Liberator_HIVE_XP_Target = 745,
    Vehicle_Destruction_Lightning_HIVE_XP_Target = 746,
    Vehicle_Destruction_Magrider_HIVE_XP_Target = 747,
    Vehicle_Destruction_Mosquito_HIVE_XP_Target = 748,
    Vehicle_Destruction_Prowler_HIVE_XP_Target = 749,
    Vehicle_Destruction_Reaver_HIVE_XP_Target = 750,
    Vehicle_Destruction_Scythe_HIVE_XP_Target = 751,
    Vehicle_Destruction_Sunderer_HIVE_XP_Target = 752,
    Vehicle_Destruction_Vanguard_HIVE_XP_Target = 753,
    Vehicle_Repair_Engi_Turret_HIVE_XP_Target = 754,
    Vehicle_Repair_Phalanx_HIVE_XP_Target = 755,
    Vehicle_Repair_Drop_Pod_HIVE_XP_Target = 756,
    Vehicle_Repair_Galaxy_HIVE_XP_Target = 757,
    Vehicle_Repair_Liberator_HIVE_XP_Target = 758,
    Vehicle_Repair_Lightning_HIVE_XP_Target = 759,
    Vehicle_Repair_Magrider_HIVE_XP_Target = 760,
    Vehicle_Repair_Mosquito_HIVE_XP_Source = 761,
    Vehicle_Repair_Prowler_HIVE_XP_Source = 762,
    Vehicle_Repair_Reaver_HIVE_XP_Source = 763,
    Vehicle_Repair_Scythe_HIVE_XP_Source = 764,
    Vehicle_Repair_Sunderer_HIVE_XP_Source = 765,
    Vehicle_Repair_Vanguard_HIVE_XP_Source = 766,
    Kill_Assist_Flash_HIVE_XP_Source = 767,
    Kill_Assist_Engi_Turret_HIVE_XP_Source = 768,
    Kill_Assist_Phalanx_HIVE_XP_Source = 769,
    Kill_Assist_Drop_Pod_HIVE_XP_Source = 770,
    Kill_Assist_Galaxy_HIVE_XP_Source = 771,
    Kill_Assist_Liberator_HIVE_XP_Source = 772,
    Kill_Assist_Lightning_HIVE_XP_Source = 773,
    Kill_Assist_Magrider_HIVE_XP_Source = 774,
    Kill_Assist_Mosquito_HIVE_XP_Source = 775,
    Kill_Assist_Prowler_HIVE_XP_Source = 776,
    Kill_Assist_Reaver_HIVE_XP_Source = 777,
    Kill_Assist_Scythe_HIVE_XP_Source = 778,
    Kill_Assist_Sunderer_HIVE_XP_Source = 779,
    Kill_Assist_Vanguard_HIVE_XP_Source = 780,
    Vehicle_Repair_Mosquito_HIVE_XP_Target = 781,
    Vehicle_Repair_Prowler_HIVE_XP_Target = 782,
    Vehicle_Repair_Reaver_HIVE_XP_Target = 783,
    Vehicle_Repair_Scythe_HIVE_XP_Target = 784,
    Vehicle_Repair_Sunderer_HIVE_XP_Target = 785,
    Vehicle_Repair_Vanguard_HIVE_XP_Target = 786,
    Kill_Assist_Flash_HIVE_XP_Target = 787,
    Kill_Assist_Engi_Turret_HIVE_XP_Target = 788,
    Kill_Assist_Phalanx_HIVE_XP_Target = 789,
    Kill_Assist_Drop_Pod_HIVE_XP_Target = 790,
    Kill_Assist_Galaxy_HIVE_XP_Target = 791,
    Kill_Assist_Liberator_HIVE_XP_Target = 792,
    Kill_Assist_Lightning_HIVE_XP_Target = 793,
    Kill_Assist_Magrider_HIVE_XP_Target = 794,
    Kill_Assist_Mosquito_HIVE_XP_Target = 795,
    Kill_Assist_Prowler_HIVE_XP_Target = 796,
    Kill_Assist_Reaver_HIVE_XP_Target = 797,
    Kill_Assist_Scythe_HIVE_XP_Target = 798,
    Kill_Assist_Sunderer_HIVE_XP_Target = 799,
    Kill_Assist_Vanguard_HIVE_XP_Target = 800,
    Drop_Pod_Kill_HIVE_XP_Source = 815,
    Player_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 816,
    Player_Kill_by_Magrider_Gunner_HIVE_XP_Source = 817,
    Player_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 818,
    Player_Kill_by_Prowler_Gunner_HIVE_XP_Source = 819,
    Player_Kill_by_Liberator_Gunner_HIVE_XP_Source = 820,
    Drop_Pod_Kill_HIVE_XP_Target = 835,
    Player_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 836,
    Player_Kill_by_Magrider_Gunner_HIVE_XP_Target = 837,
    Player_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 838,
    Player_Kill_by_Prowler_Gunner_HIVE_XP_Target = 839,
    Player_Kill_by_Liberator_Gunner_HIVE_XP_Target = 840,
    Player_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 841,
    Flash_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 842,
    Sunderer_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 843,
    Lightning_Kill_by_Sunderer_Gunne_HIVE_XP_Source = 844,
    Magrider_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 845,
    Vanguard_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 846,
    Prowler_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 847,
    Scythe_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 848,
    Reaver_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 849,
    Mosquito_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 850,
    Lib_Kill_by_Sunderer_HIVE_XP_Source = 851,
    Galaxy_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 852,
    Flash_Kill_by_Magrider_Gunner_HIVE_XP_Source = 853,
    Sunderer_Kill_by_Magrider_Gunner_HIVE_XP_Source = 854,
    Lightning_Kill_by_Magrider_Gunne_HIVE_XP_Source = 855,
    Vanguard_Kill_by_Magrider_Gunner_HIVE_XP_Source = 856,
    Prowler_Kill_by_Magrider_Gunner_HIVE_XP_Source = 857,
    Reaver_Kill_by_Magrider_Gunner_HIVE_XP_Source = 858,
    Mosquito_Kill_by_Magrider_Gunner_HIVE_XP_Source = 859,
    Lib_Kill_by_Magrider_HIVE_XP_Source = 860,
    Player_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 861,
    Flash_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 862,
    Sunderer_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 863,
    Lightning_Kill_by_Sunderer_Gunne_HIVE_XP_Target = 864,
    Magrider_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 865,
    Vanguard_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 866,
    Prowler_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 867,
    Scythe_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 868,
    Reaver_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 869,
    Mosquito_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 870,
    Lib_Kill_by_Sunderer_HIVE_XP_Target = 871,
    Galaxy_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 872,
    Flash_Kill_by_Magrider_Gunner_HIVE_XP_Target = 873,
    Sunderer_Kill_by_Magrider_Gunner_HIVE_XP_Target = 874,
    Lightning_Kill_by_Magrider_Gunne_HIVE_XP_Target = 875,
    Vanguard_Kill_by_Magrider_Gunner_HIVE_XP_Target = 876,
    Prowler_Kill_by_Magrider_Gunner_HIVE_XP_Target = 877,
    Reaver_Kill_by_Magrider_Gunner_HIVE_XP_Target = 878,
    Mosquito_Kill_by_Magrider_Gunner_HIVE_XP_Target = 879,
    Lib_Kill_by_Magrider_HIVE_XP_Target = 880,
    Galaxy_Kill_by_Magrider_Gunner_HIVE_XP_Source = 881,
    Flash_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 882,
    Sunderer_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 883,
    Lightning_Kill_by_Vanguard_Gunne_HIVE_XP_Source = 884,
    Magrider_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 885,
    Prowler_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 886,
    Scythe_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 887,
    Mosquito_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 888,
    Lib_Kill_by_Vanguard_HIVE_XP_Source = 889,
    Galaxy_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 890,
    Flash_Kill_by_Prowler_Gunner_HIVE_XP_Source = 891,
    Sunderer_Kill_by_Prowler_Gunner_HIVE_XP_Source = 892,
    Lightning_Kill_by_Prowler_Gunner_HIVE_XP_Source = 893,
    Magrider_Kill_by_Prowler_Gunner_HIVE_XP_Source = 894,
    Vanguard_Kill_by_Prowler_Gunner_HIVE_XP_Source = 895,
    Scythe_Kill_by_Prowler_Gunner_HIVE_XP_Source = 896,
    Reaver_Kill_by_Prowler_Gunner_HIVE_XP_Source = 897,
    Liberator_Kill_by_Prowler_Gunner_HIVE_XP_Source = 898,
    Galaxy_Kill_by_Prowler_Gunner_HIVE_XP_Source = 899,
    Galaxy_Kill_by_Magrider_Gunner_HIVE_XP_Target = 900,
    Flash_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 901,
    Sunderer_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 902,
    Lightning_Kill_by_Vanguard_Gunne_HIVE_XP_Target = 903,
    Magrider_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 904,
    Prowler_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 905,
    Scythe_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 906,
    Mosquito_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 907,
    Lib_Kill_by_Vanguard_HIVE_XP_Target = 908,
    Galaxy_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 909,
    Flash_Kill_by_Prowler_Gunner_HIVE_XP_Target = 910,
    Sunderer_Kill_by_Prowler_Gunner_HIVE_XP_Target = 911,
    Lightning_Kill_by_Prowler_Gunner_HIVE_XP_Target = 912,
    Magrider_Kill_by_Prowler_Gunner_HIVE_XP_Target = 913,
    Vanguard_Kill_by_Prowler_Gunner_HIVE_XP_Target = 914,
    Scythe_Kill_by_Prowler_Gunner_HIVE_XP_Target = 915,
    Reaver_Kill_by_Prowler_Gunner_HIVE_XP_Target = 916,
    Liberator_Kill_by_Prowler_Gunner_HIVE_XP_Target = 917,
    Galaxy_Kill_by_Prowler_Gunner_HIVE_XP_Target = 918,
    Flash_Kill_by_Liberator_Gunner_HIVE_XP_Source = 919,
    Sunderer_Kill_by_Lib_Gunner_HIVE_XP_Source = 920,
    Lightning_Kill_by_Liberator_Gunn_HIVE_XP_Source = 921,
    Magrider_Kill_by_Lib_Gunner_HIVE_XP_Source = 922,
    Vanguard_Kill_by_Lib_Gunner_HIVE_XP_Source = 923,
    Prowler_Kill_by_Liberator_Gunner_HIVE_XP_Source = 924,
    Scythe_Kill_by_Liberator_Gunner_HIVE_XP_Source = 925,
    Reaver_Kill_by_Liberator_Gunner_HIVE_XP_Source = 926,
    Mosquito_Kill_by_Lib_Gunner_HIVE_XP_Source = 927,
    Lib_Kill_by_Liberator_HIVE_XP_Source = 928,
    Galaxy_Kill_by_Liberator_Gunner_HIVE_XP_Source = 929,
    Flash_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 930,
    Sunderer_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 931,
    Lightning_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 932,
    Magrider_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 933,
    Vanguard_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 934,
    Prowler_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 935,
    Scythe_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 936,
    Reaver_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 937,
    Mosquito_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 938,
    Flash_Kill_by_Liberator_Gunner_HIVE_XP_Target = 939,
    Sunderer_Kill_by_Lib_Gunner_HIVE_XP_Target = 940,
    Lightning_Kill_by_Liberator_Gunn_HIVE_XP_Target = 941,
    Magrider_Kill_by_Lib_Gunner_HIVE_XP_Target = 942,
    Vanguard_Kill_by_Lib_Gunner_HIVE_XP_Target = 943,
    Prowler_Kill_by_Liberator_Gunner_HIVE_XP_Target = 944,
    Scythe_Kill_by_Liberator_Gunner_HIVE_XP_Target = 945,
    Reaver_Kill_by_Liberator_Gunner_HIVE_XP_Target = 946,
    Mosquito_Kill_by_Lib_Gunner_HIVE_XP_Target = 947,
    Lib_Kill_by_Liberator_HIVE_XP_Target = 948,
    Galaxy_Kill_by_Liberator_Gunner_HIVE_XP_Target = 949,
    Flash_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 950,
    Sunderer_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 951,
    Lightning_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 952,
    Magrider_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 953,
    Vanguard_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 954,
    Prowler_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 955,
    Scythe_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 956,
    Reaver_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 957,
    Mosquito_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 958,
    LibKill_by_Galaxy_Gunner_HIVE_XP_Source = 959,
    Galaxy_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 960,
    Terminal_Kill_HIVE_XP_Source = 961,
    Terminal_Repair_HIVE_XP_Source = 962,
    Galaxy_Damage_HIVE_XP_Source = 966,
    Liberator_Damage_HIVE_XP_Source = 967,
    Mosquito_Damage_HIVE_XP_Source = 969,
    Reaver_Damage_HIVE_XP_Source = 970,
    Scythe_Damage_HIVE_XP_Source = 971,
    Vehicle_Repair_Harasser_HIVE_XP_Source = 974,
    Kill_Assist_Harasser_HIVE_XP_Source = 975,
    Spot_Kill_Harasser_HIVE_XP_Source = 977,
    LibKill_by_Galaxy_Gunner_HIVE_XP_Target = 978,
    Galaxy_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 979,
    Terminal_Kill_HIVE_XP_Target = 980,
    Terminal_Repair_HIVE_XP_Target = 981,
    Galaxy_Damage_HIVE_XP_Target = 985,
    Liberator_Damage_HIVE_XP_Target = 986,
    Mosquito_Damage_HIVE_XP_Target = 988,
    Reaver_Damage_HIVE_XP_Target = 989,
    Scythe_Damage_HIVE_XP_Target = 990,
    Vehicle_Repair_Harasser_HIVE_XP_Target = 993,
    Kill_Assist_Harasser_HIVE_XP_Target = 994,
    Spot_Kill_Harasser_HIVE_XP_Target = 996,
    Harasser_Kill_by_Sunderer_Gunne_HIVE_XP_Source = 997,
    Harasser_Kill_by_Magrider_Gunne_HIVE_XP_Source = 998,
    Harasser_Kill_by_Vanguard_Gunne_HIVE_XP_Source = 999,
    Harasser_Kill_by_Prowler_Gunner_HIVE_XP_Source = 1000,
    Harasser_Kill_by_Liberator_Gunn_HIVE_XP_Source = 1001,
    Harasser_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 1002,
    Player_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1003,
    Flash_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1004,
    Sunderer_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1005,
    Lightning_Kill_by_Harasser_Gunne_HIVE_XP_Source = 1006,
    Vanguard_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1007,
    Prowler_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1008,
    Reaver_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1009,
    Mosquito_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1010,
    Lib_Kill_by_Harasser_HIVE_XP_Source = 1011,
    Galaxy_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1012,
    Harasser_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1013,
    Magrider_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1014,
    Scythe_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1015,
    Tank_Mine_Despawn_or_Defusal_HIVE_XP_Source = 1016,
    Harasser_Kill_by_Sunderer_Gunne_HIVE_XP_Target = 1017,
    Harasser_Kill_by_Magrider_Gunne_HIVE_XP_Target = 1018,
    Harasser_Kill_by_Vanguard_Gunne_HIVE_XP_Target = 1019,
    Harasser_Kill_by_Prowler_Gunner_HIVE_XP_Target = 1020,
    Harasser_Kill_by_Liberator_Gunn_HIVE_XP_Target = 1021,
    Harasser_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 1022,
    Player_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1023,
    Flash_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1024,
    Sunderer_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1025,
    Lightning_Kill_by_Harasser_Gunne_HIVE_XP_Target = 1026,
    Vanguard_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1027,
    Prowler_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1028,
    Reaver_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1029,
    Mosquito_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1030,
    Lib_Kill_by_Harasser_HIVE_XP_Target = 1031,
    Galaxy_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1032,
    Harasser_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1033,
    Magrider_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1034,
    Scythe_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1035,
    Tank_Mine_Despawn_or_Defusal_HIVE_XP_Target = 1036,
    Vehicle_Gunner_Kill_Share_Infantry_HIVE_XP_Source = 1037,
    Vehicle_Gunner_Kill_Share_Flash_HIVE_XP_Source = 1038,
    Vehicle_Gunner_Kill_Share_Engineer_Turret_HIVE_XP_Source = 1039,
    Vehicle_Gunner_Kill_Share_Phalanx_HIVE_XP_Source = 1040,
    Vehicle_Gunner_Kill_Share_Drop_Pod_HIVE_XP_Source = 1041,
    Vehicle_Gunner_Kill_Share_Galaxy_HIVE_XP_Source = 1042,
    Vehicle_Gunner_Kill_Share_Liberator_HIVE_XP_Source = 1043,
    Vehicle_Gunner_Kill_Share_Lightning_HIVE_XP_Source = 1044,
    Vehicle_Gunner_Kill_Share_Magrider_HIVE_XP_Source = 1045,
    Vehicle_Gunner_Kill_Share_Mosquito_HIVE_XP_Source = 1046,
    Vehicle_Gunner_Kill_Share_Prowler_HIVE_XP_Source = 1047,
    Vehicle_Gunner_Kill_Share_Reaver_HIVE_XP_Source = 1048,
    Vehicle_Gunner_Kill_Share_Scythe_HIVE_XP_Source = 1049,
    Vehicle_Gunner_Kill_Share_Sunderer_HIVE_XP_Source = 1050,
    Vehicle_Gunner_Kill_Share_Vanguard_HIVE_XP_Source = 1051,
    Vehicle_Gunner_Kill_Share_Harasser_HIVE_XP_Source = 1052,
    Vehicle_Gunner_Kill_Assist_Share_Infantry_HIVE_XP_Source = 1053,
    Vehicle_Gunner_Kill_Assist_Share_Flash_HIVE_XP_Source = 1054,
    Vehicle_Gunner_Kill_Assist_Share_Engi_Turret_HIVE_XP_Source = 1055,
    Vehicle_Gunner_Kill_Assist_Share_Phalanx_HIVE_XP_Source = 1056,
    Vehicle_Gunner_Kill_Share_Infantry_HIVE_XP_Target = 1057,
    Vehicle_Gunner_Kill_Share_Flash_HIVE_XP_Target = 1058,
    Vehicle_Gunner_Kill_Share_Engineer_Turret_HIVE_XP_Target = 1059,
    Vehicle_Gunner_Kill_Share_Phalanx_HIVE_XP_Target = 1060,
    Vehicle_Gunner_Kill_Share_Drop_Pod_HIVE_XP_Target = 1061,
    Vehicle_Gunner_Kill_Share_Galaxy_HIVE_XP_Target = 1062,
    Vehicle_Gunner_Kill_Share_Liberator_HIVE_XP_Target = 1063,
    Vehicle_Gunner_Kill_Share_Lightning_HIVE_XP_Target = 1064,
    Vehicle_Gunner_Kill_Share_Magrider_HIVE_XP_Target = 1065,
    Vehicle_Gunner_Kill_Share_Mosquito_HIVE_XP_Target = 1066,
    Vehicle_Gunner_Kill_Share_Prowler_HIVE_XP_Target = 1067,
    Vehicle_Gunner_Kill_Share_Reaver_HIVE_XP_Target = 1068,
    Vehicle_Gunner_Kill_Share_Scythe_HIVE_XP_Target = 1069,
    Vehicle_Gunner_Kill_Share_Sunderer_HIVE_XP_Target = 1070,
    Vehicle_Gunner_Kill_Share_Vanguard_HIVE_XP_Target = 1071,
    Vehicle_Gunner_Kill_Share_Harasser_HIVE_XP_Target = 1072,
    Vehicle_Gunner_Kill_Assist_Share_Infantry_HIVE_XP_Target = 1073,
    Vehicle_Gunner_Kill_Assist_Share_Flash_HIVE_XP_Target = 1074,
    Vehicle_Gunner_Kill_Assist_Share_Engi_Turret_HIVE_XP_Target = 1075,
    Vehicle_Gunner_Kill_Assist_Share_Phalanx_HIVE_XP_Target = 1076,
    Vehicle_Gunner_Kill_Assist_Share_Drop_Pod_HIVE_XP_Source = 1077,
    Vehicle_Gunner_Kill_Assist_Share_Galaxy_HIVE_XP_Source = 1078,
    Vehicle_Gunner_Kill_Assist_Share_Liberator_HIVE_XP_Source = 1079,
    Vehicle_Gunner_Kill_Assist_Share_Lightning_HIVE_XP_Source = 1080,
    Vehicle_Gunner_Kill_Assist_Share_Magrider_HIVE_XP_Source = 1081,
    Vehicle_Gunner_Kill_Assist_Share_Mosquito_HIVE_XP_Source = 1082,
    Vehicle_Gunner_Kill_Assist_Share_Prowler_HIVE_XP_Source = 1083,
    Vehicle_Gunner_Kill_Assist_Share_Reaver_HIVE_XP_Source = 1084,
    Vehicle_Gunner_Kill_Assist_Share_Scythe_HIVE_XP_Source = 1085,
    Vehicle_Gunner_Kill_Assist_Share_Sunderer_HIVE_XP_Source = 1086,
    Vehicle_Gunner_Kill_Assist_Share_Vanguard_HIVE_XP_Source = 1087,
    Vehicle_Gunner_Kill_Assist_Share_Harasser_HIVE_XP_Source = 1088,
    Vehicle_Passenger_Kill_Share_Infantry_HIVE_XP_Source = 1089,
    Vehicle_Passenger_Kill_Share_Flash_HIVE_XP_Source = 1090,
    Vehicle_Passenger_Kill_Share_Engineer_Turret_HIVE_XP_Source = 1091,
    Vehicle_Passenger_Kill_Share_Phalanx_HIVE_XP_Source = 1092,
    Vehicle_Passenger_Kill_Share_Drop_Pod_HIVE_XP_Source = 1093,
    Vehicle_Passenger_Kill_Share_Galaxy_HIVE_XP_Source = 1094,
    Vehicle_Passenger_Kill_Share_Liberator_HIVE_XP_Source = 1095,
    Vehicle_Passenger_Kill_Share_Lightning_HIVE_XP_Source = 1096,
    Vehicle_Gunner_Kill_Assist_Share_Drop_Pod_HIVE_XP_Target = 1097,
    Vehicle_Gunner_Kill_Assist_Share_Galaxy_HIVE_XP_Target = 1098,
    Vehicle_Gunner_Kill_Assist_Share_Liberator_HIVE_XP_Target = 1099,
    Vehicle_Gunner_Kill_Assist_Share_Lightning_HIVE_XP_Target = 1100,
    Vehicle_Gunner_Kill_Assist_Share_Magrider_HIVE_XP_Target = 1101,
    Vehicle_Gunner_Kill_Assist_Share_Mosquito_HIVE_XP_Target = 1102,
    Vehicle_Gunner_Kill_Assist_Share_Prowler_HIVE_XP_Target = 1103,
    Vehicle_Gunner_Kill_Assist_Share_Reaver_HIVE_XP_Target = 1104,
    Vehicle_Gunner_Kill_Assist_Share_Scythe_HIVE_XP_Target = 1105,
    Vehicle_Gunner_Kill_Assist_Share_Sunderer_HIVE_XP_Target = 1106,
    Vehicle_Gunner_Kill_Assist_Share_Vanguard_HIVE_XP_Target = 1107,
    Vehicle_Gunner_Kill_Assist_Share_Harasser_HIVE_XP_Target = 1108,
    Vehicle_Passenger_Kill_Share_Infantry_HIVE_XP_Target = 1109,
    Vehicle_Passenger_Kill_Share_Flash_HIVE_XP_Target = 1110,
    Vehicle_Passenger_Kill_Share_Engineer_Turret_HIVE_XP_Target = 1111,
    Vehicle_Passenger_Kill_Share_Phalanx_HIVE_XP_Target = 1112,
    Vehicle_Passenger_Kill_Share_Drop_Pod_HIVE_XP_Target = 1113,
    Vehicle_Passenger_Kill_Share_Galaxy_HIVE_XP_Target = 1114,
    Vehicle_Passenger_Kill_Share_Liberator_HIVE_XP_Target = 1115,
    Vehicle_Passenger_Kill_Share_Lightning_HIVE_XP_Target = 1116,
    Vehicle_Passenger_Kill_Share_Magrider_HIVE_XP_Source = 1117,
    Vehicle_Passenger_Kill_Share_Mosquito_HIVE_XP_Source = 1118,
    Vehicle_Passenger_Kill_Share_Prowler_HIVE_XP_Source = 1119,
    Vehicle_Passenger_Kill_Share_Reaver_HIVE_XP_Source = 1120,
    Vehicle_Passenger_Kill_Share_Scythe_HIVE_XP_Source = 1121,
    Vehicle_Passenger_Kill_Share_Sunderer_HIVE_XP_Source = 1122,
    Vehicle_Passenger_Kill_Share_Vanguard_HIVE_XP_Source = 1123,
    Vehicle_Passenger_Kill_Share_Harasser_HIVE_XP_Source = 1124,
    Vehicle_Driver_Kill_Assist_Share_Infantry_HIVE_XP_Source = 1125,
    Vehicle_Driver_Kill_Assist_Share_Flash_HIVE_XP_Source = 1126,
    Vehicle_Driver_Kill_Assist_Share_Engi_Turret_HIVE_XP_Source = 1127,
    Vehicle_Driver_Kill_Assist_Share_Phalanx_HIVE_XP_Source = 1128,
    Vehicle_Driver_Kill_Assist_Share_Drop_Pod_HIVE_XP_Source = 1129,
    Vehicle_Driver_Kill_Assist_Share_Galaxy_HIVE_XP_Source = 1130,
    Vehicle_Driver_Kill_Assist_Share_Liberator_HIVE_XP_Source = 1131,
    Vehicle_Driver_Kill_Assist_Share_Lightning_HIVE_XP_Source = 1132,
    Vehicle_Driver_Kill_Assist_Share_Magrider_HIVE_XP_Source = 1133,
    Vehicle_Driver_Kill_Assist_Share_Mosquito_HIVE_XP_Source = 1134,
    Vehicle_Driver_Kill_Assist_Share_Prowler_HIVE_XP_Source = 1135,
    Vehicle_Driver_Kill_Assist_Share_Reaver_HIVE_XP_Source = 1136,
    Vehicle_Passenger_Kill_Share_Magrider_HIVE_XP_Target = 1137,
    Vehicle_Passenger_Kill_Share_Mosquito_HIVE_XP_Target = 1138,
    Vehicle_Passenger_Kill_Share_Prowler_HIVE_XP_Target = 1139,
    Vehicle_Passenger_Kill_Share_Reaver_HIVE_XP_Target = 1140,
    Vehicle_Passenger_Kill_Share_Scythe_HIVE_XP_Target = 1141,
    Vehicle_Passenger_Kill_Share_Sunderer_HIVE_XP_Target = 1142,
    Vehicle_Passenger_Kill_Share_Vanguard_HIVE_XP_Target = 1143,
    Vehicle_Passenger_Kill_Share_Harasser_HIVE_XP_Target = 1144,
    Vehicle_Driver_Kill_Assist_Share_Infantry_HIVE_XP_Target = 1145,
    Vehicle_Driver_Kill_Assist_Share_Flash_HIVE_XP_Target = 1146,
    Vehicle_Driver_Kill_Assist_Share_Engi_Turret_HIVE_XP_Target = 1147,
    Vehicle_Driver_Kill_Assist_Share_Phalanx_HIVE_XP_Target = 1148,
    Vehicle_Driver_Kill_Assist_Share_Drop_Pod_HIVE_XP_Target = 1149,
    Vehicle_Driver_Kill_Assist_Share_Galaxy_HIVE_XP_Target = 1150,
    Vehicle_Driver_Kill_Assist_Share_Liberator_HIVE_XP_Target = 1151,
    Vehicle_Driver_Kill_Assist_Share_Lightning_HIVE_XP_Target = 1152,
    Vehicle_Driver_Kill_Assist_Share_Magrider_HIVE_XP_Target = 1153,
    Vehicle_Driver_Kill_Assist_Share_Mosquito_HIVE_XP_Target = 1154,
    Vehicle_Driver_Kill_Assist_Share_Prowler_HIVE_XP_Target = 1155,
    Vehicle_Driver_Kill_Assist_Share_Reaver_HIVE_XP_Target = 1156,
    Vehicle_Driver_Kill_Assist_Share_Scythe_HIVE_XP_Source = 1157,
    Vehicle_Driver_Kill_Assist_Share_Sunderer_HIVE_XP_Source = 1158,
    Vehicle_Driver_Kill_Assist_Share_Vanguard_HIVE_XP_Source = 1159,
    Vehicle_Driver_Kill_Assist_Share_Harasser_HIVE_XP_Source = 1160,
    Valkyrie_Kill_by_Sunderer_Gunner_HIVE_XP_Source = 1161,
    Valkyrie_Kill_by_Magrider_Gunner_HIVE_XP_Source = 1162,
    Valkyrie_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 1163,
    Valkyrie_Kill_by_Prowler_Gunner_HIVE_XP_Source = 1164,
    Valkyrie_Kill_by_Liberator_Gunner_HIVE_XP_Source = 1165,
    Valkyrie_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 1166,
    Player_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1167,
    Vehicle_Gunner_Kill_Share_Valkyrie_HIVE_XP_Source = 1168,
    Vehicle_Gunner_Kill_Assist_Share_Valkyrie_HIVE_XP_Source = 1169,
    Vehicle_Passenger_Kill_Share_Valkyrie_HIVE_XP_Source = 1170,
    Vehicle_Driver_Kill_Assist_Share_Valkyrie_HIVE_XP_Source = 1171,
    Flash_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1172,
    Sunderer_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1173,
    Lightning_Kill_by_Valkyrie_Gunne_HIVE_XP_Source = 1174,
    Vanguard_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1175,
    Prowler_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1176,
    Vehicle_Driver_Kill_Assist_Share_Scythe_HIVE_XP_Target = 1177,
    Vehicle_Driver_Kill_Assist_Share_Sunderer_HIVE_XP_Target = 1178,
    Vehicle_Driver_Kill_Assist_Share_Vanguard_HIVE_XP_Target = 1179,
    Vehicle_Driver_Kill_Assist_Share_Harasser_HIVE_XP_Target = 1180,
    Valkyrie_Kill_by_Sunderer_Gunner_HIVE_XP_Target = 1181,
    Valkyrie_Kill_by_Magrider_Gunner_HIVE_XP_Target = 1182,
    Valkyrie_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 1183,
    Valkyrie_Kill_by_Prowler_Gunner_HIVE_XP_Target = 1184,
    Valkyrie_Kill_by_Liberator_Gunner_HIVE_XP_Target = 1185,
    Valkyrie_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 1186,
    Player_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1187,
    Vehicle_Gunner_Kill_Share_Valkyrie_HIVE_XP_Target = 1188,
    Vehicle_Gunner_Kill_Assist_Share_Valkyrie_HIVE_XP_Target = 1189,
    Vehicle_Passenger_Kill_Share_Valkyrie_HIVE_XP_Target = 1190,
    Vehicle_Driver_Kill_Assist_Share_Valkyrie_HIVE_XP_Target = 1191,
    Flash_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1192,
    Sunderer_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1193,
    Lightning_Kill_by_Valkyrie_Gunne_HIVE_XP_Target = 1194,
    Vanguard_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1195,
    Prowler_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1196,
    Reaver_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1197,
    Mosquito_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1198,
    Lib_Kill_by_Valkyrie_HIVE_XP_Source = 1199,
    Galaxy_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1200,
    Magrider_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1201,
    Scythe_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1202,
    Snowman_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1203,
    Valkyrie_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1204,
    Harasser_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1205,
    Valkyrie_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1206,
    Destroy_Spitfire_Turret_HIVE_XP_Source = 1207,
    Vehicle_Gunner_Kill_Share_Spitfire_HIVE_XP_Source = 1208,
    Vehicle_Gunner_Kill_Assist_Share_Spitfire_HIVE_XP_Source = 1209,
    Vehicle_Passenger_Kill_Share_Spitfire_HIVE_XP_Source = 1210,
    Vehicle_Driver_Kill_Assist_Share_Engi_Turret_HIVE_XP_Source = 1211,
    Kill_or_Destroy_Construction_Med_HIVE_XP_Source = 1212,
    Repair_Construction_Med_HIVE_XP_Source = 1214,
    KillAssist_Construction_Med_HIVE_XP_Source = 1215,
    Reaver_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1217,
    Mosquito_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1218,
    Lib_Kill_by_Valkyrie_HIVE_XP_Target = 1219,
    Galaxy_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1220,
    Magrider_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1221,
    Scythe_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1222,
    Snowman_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1223,
    Valkyrie_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1224,
    Harasser_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1225,
    Valkyrie_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1226,
    Destroy_Spitfire_Turret_HIVE_XP_Target = 1227,
    Vehicle_Gunner_Kill_Share_Spitfire_HIVE_XP_Target = 1228,
    Vehicle_Gunner_Kill_Assist_Share_Spitfire_HIVE_XP_Target = 1229,
    Vehicle_Passenger_Kill_Share_Spitfire_HIVE_XP_Target = 1230,
    Vehicle_Driver_Kill_Assist_Share_Engi_Turret_HIVE_XP_Target = 1231,
    Kill_or_Destroy_Construction_Med_HIVE_XP_Target = 1232,
    Repair_Construction_Med_HIVE_XP_Target = 1234,
    KillAssist_Construction_Med_HIVE_XP_Target = 1235,
    Spot_Kill_Construction_Med__HIVE_XP_Source = 1237,
    Gunner_Kill_Share_Construction_Med_HIVE_XP_Source = 1238,
    Gunner_Kill_Assist_Share_Construction_Med_HIVE_XP_Source = 1239,
    Vehicle_Passenger_Kill_Share_Construction_Med_HIVE_XP_Source = 1240,
    Driver_Kill_Aassist_Share_Construction_Med_HIVE_XP_Source = 1241,
    Vehicle_Ram_Kill_Construction_Small_HIVE_XP_Source = 1242,
    Kill_or_Destroy_Construction_Small_HIVE_XP_Source = 1243,
    Repair_Construction_Small_HIVE_XP_Source = 1245,
    KillAssist_Construction_Small_HIVE_XP_Source = 1246,
    Spot_Kill_Construction_Small__HIVE_XP_Source = 1248,
    Gunner_Kill_Share_Construction_Small_HIVE_XP_Source = 1249,
    Gunner_Kill_Assist_Share_Construction_Small_HIVE_XP_Source = 1250,
    Vehicle_Passenger_Kill_Share_Construction_Small_HIVE_XP_Source = 1251,
    Driver_Kill_Aassist_Share_Construction_Small_HIVE_XP_Source = 1252,
    Vehicle_Ram_Kill_Construction_Large_HIVE_XP_Source = 1253,
    Kill_or_Destroy_Construction_Large_HIVE_XP_Source = 1254,
    Spot_Kill_Construction_Med__HIVE_XP_Target = 1255,
    Gunner_Kill_Share_Construction_Med_HIVE_XP_Target = 1256,
    Gunner_Kill_Assist_Share_Construction_Med_HIVE_XP_Target = 1257,
    Vehicle_Passenger_Kill_Share_Construction_Med_HIVE_XP_Target = 1258,
    Driver_Kill_Aassist_Share_Construction_Med_HIVE_XP_Target = 1259,
    Vehicle_Ram_Kill_Construction_Small_HIVE_XP_Target = 1260,
    Kill_or_Destroy_Construction_Small_HIVE_XP_Target = 1261,
    Repair_Construction_Small_HIVE_XP_Target = 1263,
    KillAssist_Construction_Small_HIVE_XP_Target = 1264,
    Spot_Kill_Construction_Small__HIVE_XP_Target = 1266,
    Gunner_Kill_Share_Construction_Small_HIVE_XP_Target = 1267,
    Gunner_Kill_Assist_Share_Construction_Small_HIVE_XP_Target = 1268,
    Vehicle_Passenger_Kill_Share_Construction_Small_HIVE_XP_Target = 1269,
    Driver_Kill_Aassist_Share_Construction_Small_HIVE_XP_Target = 1270,
    Vehicle_Ram_Kill_Construction_Large_HIVE_XP_Target = 1271,
    Kill_or_Destroy_Construction_Large_HIVE_XP_Target = 1272,
    Repair_Construction_Large_HIVE_XP_Source = 1273,
    KillAssist_Construction_Large_HIVE_XP_Source = 1274,
    Spot_Kill_Construction_Large__HIVE_XP_Source = 1276,
    Gunner_Kill_Share_Construction_Large_HIVE_XP_Source = 1277,
    Gunner_Kill_Assist_Share_Construction_Large_HIVE_XP_Source = 1278,
    Vehicle_Passenger_Kill_Share_Construction_Large_HIVE_XP_Source = 1279,
    Driver_Kill_Aassist_Share_Construction_Large_HIVE_XP_Source = 1280,
    Vehicle_Ram_Kill_Construction_Core_HIVE_XP_Source = 1281,
    Kill_or_Destroy_Construction_Core_HIVE_XP_Source = 1282,
    Repair_Construction_Core_HIVE_XP_Source = 1283,
    KillAssist_Construction_Core_HIVE_XP_Source = 1284,
    Spot_Kill_Construction_Core__HIVE_XP_Source = 1286,
    Gunner_Kill_Share_Construction_Core_HIVE_XP_Source = 1287,
    Gunner_Kill_Assist_Share_Construction_Core_HIVE_XP_Source = 1288,
    Repair_Construction_Large_HIVE_XP_Target = 1289,
    KillAssist_Construction_Large_HIVE_XP_Target = 1290,
    Spot_Kill_Construction_Large__HIVE_XP_Target = 1292,
    Gunner_Kill_Share_Construction_Large_HIVE_XP_Target = 1293,
    Gunner_Kill_Assist_Share_Construction_Large_HIVE_XP_Target = 1294,
    Vehicle_Passenger_Kill_Share_Construction_Large_HIVE_XP_Target = 1295,
    Driver_Kill_Aassist_Share_Construction_Large_HIVE_XP_Target = 1296,
    Vehicle_Ram_Kill_Construction_Core_HIVE_XP_Target = 1297,
    Kill_Destroy_Construction_Core_HIVE_XP_Target = 1298,
    Repair_Construction_Core_HIVE_XP_Target = 1299,
    KillAssist_Construction_Core_HIVE_XP_Target = 1300,
    Spot_Kill_Construction_Core__HIVE_XP_Target = 1302,
    Gunner_Kill_Share_Construction_Core_HIVE_XP_Target = 1303,
    Gunner_Kill_Assist_Share_Construction_Core_HIVE_XP_Target = 1304,
    Vehicle_Passenger_Kill_Share_Construction_Core_HIVE_XP_Source = 1305,
    Driver_Kill_Aassist_Share_Construction_Core_HIVE_XP_Source = 1306,
    Vehicle_Destruction_ANT_HIVE_XP_Source = 1307,
    Vehicle_Ram_Kill_ANT_HIVE_XP_Source = 1308,
    Vehicle_Repair_ANT_HIVE_XP_Source = 1309,
    Kill_Assist_ANT_HIVE_XP_Source = 1310,
    ANT_Kill_by_ANT_Gunner_HIVE_XP_Source = 1312,
    ANT_Kill_by_Magrider_Gunner_HIVE_XP_Source = 1313,
    ANT_Kill_by_Vanguard_Gunner_HIVE_XP_Source = 1314,
    ANT_Kill_by_Prowler_Gunner_HIVE_XP_Source = 1315,
    ANT_Kill_by_Lib_Gunner_HIVE_XP_Source = 1316,
    ANT_Kill_by_Galaxy_Gunner_HIVE_XP_Source = 1317,
    Spot_Kill_ANT_HIVE_XP_Source = 1318,
    ANT_Kill_by_Harasser_Gunner_HIVE_XP_Source = 1320,
    Vehicle_Gunner_Kill_Share_ANT_HIVE_XP_Source = 1321,
    Vehicle_Gunner_Kill_Assist_Share_ANT_HIVE_XP_Source = 1322,
    Vehicle_Passenger_Kill_Share_ANT_HIVE_XP_Source = 1323,
    Vehicle_Driver_Kill_Assist_Share_ANT_HIVE_XP_Source = 1324,
    Vehicle_Passenger_Kill_Share_Construction_Core_HIVE_XP_Target = 1325,
    Driver_Kill_Aassist_Share_Construction_Core_HIVE_XP_Target = 1326,
    Vehicle_Destruction_ANT_HIVE_XP_Target = 1327,
    Vehicle_Ram_Kill_ANT_HIVE_XP_Target = 1328,
    Vehicle_Repair_ANT_HIVE_XP_Target = 1329,
    Kill_Assist_ANT_HIVE_XP_Target = 1330,
    ANT_Kill_by_ANT_Gunner_HIVE_XP_Target = 1332,
    ANT_Kill_by_Magrider_Gunner_HIVE_XP_Target = 1333,
    ANT_Kill_by_Vanguard_Gunner_HIVE_XP_Target = 1334,
    ANT_Kill_by_Prowler_Gunner_HIVE_XP_Target = 1335,
    ANT_Kill_by_Lib_Gunner_HIVE_XP_Target = 1336,
    ANT_Kill_by_Galaxy_Gunner_HIVE_XP_Target = 1337,
    Spot_Kill_ANT_HIVE_XP_Target = 1338,
    ANT_Kill_by_Harasser_Gunner_HIVE_XP_Target = 1340,
    Vehicle_Gunner_Kill_Share_ANT_HIVE_XP_Target = 1341,
    Vehicle_Gunner_Kill_Assist_Share_ANT_HIVE_XP_Target = 1342,
    Vehicle_Passenger_Kill_Share_ANT_HIVE_XP_Target = 1343,
    Vehicle_Driver_Kill_Assist_Share_ANT_HIVE_XP_Target = 1344,
    ANT_Kill_by_Valkyrie_Gunner_HIVE_XP_Source = 1345,
    Flash_Kill_by_ANT_Gunner_HIVE_XP_Source = 1346,
    Galaxy_Kill_by_ANT_Gunner_HIVE_XP_Source = 1347,
    Harasser_Kill_by_ANT_Gunner_HIVE_XP_Source = 1348,
    Magrider_Kill_by_ANT_Gunner_HIVE_XP_Source = 1349,
    Mosquito_Kill_by_ANT_Gunner_HIVE_XP_Source = 1350,
    Player_Kill_by_ANT_Gunner_HIVE_XP_Source = 1351,
    Prowler_Kill_by_ANT_Gunner_HIVE_XP_Source = 1352,
    Pumpkin_Kill_by_ANT_Gunner_HIVE_XP_Source = 1353,
    Reaver_Kill_by_ANT_Gunner_HIVE_XP_Source = 1354,
    Scythe_Kill_by_ANT_Gunner_HIVE_XP_Source = 1355,
    Sunderer_Kill_by_ANT_Gunner_HIVE_XP_Source = 1356,
    Valkyrie_Kill_by_ANT_Gunner_HIVE_XP_Source = 1357,
    Vanguard_Kill_by_ANT_Gunner_HIVE_XP_Source = 1358,
    ANT_Kill_by_Valkyrie_Gunner_HIVE_XP_Target = 1359,
    Flash_Kill_by_ANT_Gunner_HIVE_XP_Target = 1360,
    Galaxy_Kill_by_ANT_Gunner_HIVE_XP_Target = 1361,
    Harasser_Kill_by_ANT_Gunner_HIVE_XP_Target = 1362,
    Magrider_Kill_by_ANT_Gunner_HIVE_XP_Target = 1363,
    Mosquito_Kill_by_ANT_Gunner_HIVE_XP_Target = 1364,
    Player_Kill_by_ANT_Gunner_HIVE_XP_Target = 1365,
    Prowler_Kill_by_ANT_Gunner_HIVE_XP_Target = 1366,
    Reaver_Kill_by_ANT_Gunner_HIVE_XP_Target = 1368,
    Scythe_Kill_by_ANT_Gunner_HIVE_XP_Target = 1369,
    Sunderer_Kill_by_ANT_Gunner_HIVE_XP_Target = 1370,
    Valkyrie_Kill_by_ANT_Gunner_HIVE_XP_Target = 1371,
    Vanguard_Kill_by_ANT_Gunner_HIVE_XP_Target = 1372,
    Destroy_Hardlight_Barrier = 1373,
    Vehicle_Ram_Kill_Hardlight_Barrier = 1374,
    Vehicle_Repair_Hardlight_Barrier = 1375,
    Kill_Assist_Hardlight_Barrier = 1376,
    Squad_Repair_Hardlight_Barrier = 1378,
    Destroy_Hardlight_Barrier_HIVE_XP_Source = 1383,
    Vehicle_Gunner_Kill_Share_Hardlight_Barrier_HIVE_XP_Source = 1384,
    Vehicle_Gunner_Kill_Assist_Share_Hardlight_Barrier_HIVE_XP_Source = 1385,
    Vehicle_Passenger_Kill_Share_Hardlight_Barrier_HIVE_XP_Source = 1386,
    Vehicle_Driver_Kill_Assist_Share_Hardlight_Barrier_HIVE_XP_Source = 1387,
    Destroy_Hardlight_Barrier_HIVE_XP_Target = 1388,
    Vehicle_Gunner_Kill_Share_Hardlight_Barrier_HIVE_XP_Target = 1389,
    Vehicle_Gunner_Kill_Assist_Share_Hardlight_Barrier_HIVE_XP_Target = 1390,
    Vehicle_Passenger_Kill_Share_Hardlight_Barrier_HIVE_XP_Target = 1391,
    Vehicle_Driver_Kill_Assist_Share_Hardlight_Barrier_HIVE_XP_Target = 1392,
    */
    Hardlight_Cover_Blocking_Exp_placeholder_until_code_is_done = 1393,
    Draw_Fire_Award = 1394,
    Flash_Damage_Infantry_vs_Vehicle = 1395,
    Harasser_Damage_Infantry_vs_Vehicle = 1396,
    Router_Kill = 1409,
    Generic_Npc_Spawn = 1410,
    Event_Anomaly_Defense_Pulse = 1420,
    EQ20_Collectible = 1430,
    EQ20_Nightshade = 1440,
    Vehicle_Destruction_Colossus = 1449,
    Vehicle_Ram_Kill_Colossus = 1450,
    Vehicle_Repair_Colossus = 1451,
    Squad_Repair_Colossus = 1452,
    Kill_Assist_Colossus = 1453,
    Tank_Hunter_Bonus_Colossus_Kill = 1454,
    Chain_Expl_Assist_Colossus = 1455,
    Squad_Spot_Kill_Colossus = 1456,
    Spot_Kill_Colossus = 1457,
    Colossus_Damage_Infantry_vs_Vehicle = 1458,
    Flash_Kill_by_Colossus_Gunner = 1459,
    Vanguard_Kill_by_Colossus_Gunner = 1460,
    ANT_Kill_by_Colossus_Gunner = 1461,
    Galaxy_Kill_by_Colossus_Gunner = 1462,
    Harasser_Kill_by_Colossus_Gunner = 1463,
    Magrider_Kill_by_Colossus_Gunner = 1464,
    Mosquito_Kill_by_Colossus_Gunner = 1465,
    Player_Kill_by_Colossus_Gunner = 1466,
    Prowler_Kill_by_Colossus_Gunner = 1467,
    Pumpkin_Kill_by_Colossus_Gunner = 1468,
    R_Drone_Kill_by_Colossus_Gunner = 1469,
    Reaver_Kill_by_Colossus_Gunner = 1470,
    Scythe_Kill_by_Colossus_Gunner = 1471,
    Snowman_Kill_by_Colossus_Gunner = 1472,
    Sunderer_Kill_by_Colossus_Gunner = 1473,
    Valkyrie_Kill_by_Colossus_Gunner = 1474,
    Colossus_Kill_by_Colossus_Gunner = 1475,
    Vehicle_Destruction_Javelin = 1480,
    Squad_Repair_Javelin = 1481,
    Vehicle_Repair_Javelin = 1482,
    Vehicle_Ram_Bonus_Javelin = 1483,
    Kill_Assist_Javelin = 1484,
    Javelin_Kill_by_Sunderer_Gunner = 1486,
    Javelin_Kill_by_Magrider_Gunner = 1487,
    Javelin_Kill_by_Vanguard_Gunner = 1488,
    Javelin_Kill_by_Prowler_Gunner = 1489,
    Javelin_Kill_by_Liberator_Gunner = 1490,
    Javelin_Kill_by_Galaxy_Gunner = 1491,
    Spot_Kill_Javelin = 1492,
    Squad_Spot_Kill_Javelin = 1493,
    Javelin_Kill_by_Harasser_Gunner = 1494,
    Chain_Expl_Assist_Javelin = 1499,
    Javelin_Kill_by_Valkyrie_Gunner = 1500,
    Javelin_Kill_by_ANT_Gunner = 1501,
    Javelin_Damage_Infantry_vs_Vehicle = 1502,
    ANT_Kill_by_Bastion_Gunner = 1503,
    Colossus_Kill_by_Bastion_Gunner = 1504,
    Flash_Kill_by_Bastion_Gunner = 1505,
    Galaxy_Kill_by_Bastion_Gunner = 1506,
    Harasser_Kill_by_Bastion_Gunner = 1507,
    Magrider_Kill_by_Bastion_Gunner = 1508,
    Mosquito_Kill_by_Bastion_Gunner = 1509,
    Player_Kill_by_Bastion_Gunner = 1510,
    Prowler_Kill_by_Bastion_Gunner = 1511,
    Pumpkin_Kill_by_Bastion_Gunner = 1512,
    R_Drone_Kill_by_Bastion_Gunner = 1513,
    Reaver_Kill_by_Bastion_Gunner = 1514,
    Scythe_Kill_by_Bastion_Gunner = 1515,
    Snowman_Kill_by_Bastion_Gunner = 1516,
    Sunderer_Kill_by_Bastion_Gunner = 1517,
    Valkyrie_Kill_by_Bastion_Gunner = 1518,
    Vanguard_Kill_by_Bastion_Gunner = 1519,
    Vehicle_Component_Destruction_Bastion = 1520,
    Kill_Assist_Bastion_Component = 1521,
    Bastion_Component_Kill_by_Sunderer_Gunner = 1526,
    Bastion_Component_Kill_by_Vanguard_Gunner = 1527,
    Bastion_Component_Kill_by_Prowler_Gunner = 1528,
    Bastion_Component_Kill_by_Lib_Gunner = 1529,
    Bastion_Component_Kill_by_Galaxy_Gunner = 1530,
    Bastion_Component_Kill_by_Harasser_Gunner = 1531,
    Bastion_Component_Kill_by_Valkyrie_Gunner = 1532,
    Bastion_Component_Kill_by_ANT_Gunner = 1533,
    Bastion_Component_Kill_by_Colossus_Gunner = 1534,
    Bastion_Component_Kill_by_Bastion_Gunner = 1535,
    Bastion_Component_Damage = 1536,
    Destroy_Lightning_Arrester = 1537,
    Lightning_Arrester_Absorb_Xp = 1538,
    Convoy_Periodic_Update = 1539,
    Convoy_Convoy_Complete = 1540,
    Sunderer_Campaign_Spawn_Bonus = 1542,
    Galaxy_Campaign_Spawn_Bonus = 1543,
    tutorial2_grantBr1 = 1544,
    Door_Lock_Repair = 1545,
    Destroy_Shield_Door_Lock = 1546,
    Containment_Site_Gate_Shield_Gen_Destroy = 1547,
    Containment_Site_Gate_Shield_Gen_Destroy_Assist = 1548,
    Containment_Site_Gate_Shield_Gen_Repair = 1549,

    #[num_enum(default)]
    Unknown = 0,
}

#[allow(dead_code)]
impl ExperienceType {
    pub fn is_repair(&self) -> bool {
        matches!(
            self,
            ExperienceType::MAX_Repair
                | ExperienceType::Squad_Repair_Flash
                | ExperienceType::Vehicle_Repair_Flash
                | ExperienceType::Secondary_Facility_Object_Repair
                | ExperienceType::Vehicle_Repair_Engi_Turret
                | ExperienceType::Vehicle_Repair_Phalanx
                | ExperienceType::Vehicle_Repair_Drop_Pod
                | ExperienceType::Vehicle_Repair_Galaxy
                | ExperienceType::Vehicle_Repair_Liberator
                | ExperienceType::Vehicle_Repair_Lightning
                | ExperienceType::Vehicle_Repair_Magrider
                | ExperienceType::Vehicle_Repair_Mosquito
                | ExperienceType::Vehicle_Repair_Prowler
                | ExperienceType::Vehicle_Repair_Reaver
                | ExperienceType::Vehicle_Repair_Scythe
                | ExperienceType::Vehicle_Repair_Sunderer
                | ExperienceType::Vehicle_Repair_Vanguard
                | ExperienceType::Squad_Repair_Engi_Turret
                | ExperienceType::Squad_Repair_Phalanx
                | ExperienceType::Squad_Repair_Drop_Pod
                | ExperienceType::Squad_Repair_Galaxy
                | ExperienceType::Squad_Repair_Liberator
                | ExperienceType::Squad_Repair_Lightning
                | ExperienceType::Squad_Repair_Magrider
                | ExperienceType::Squad_Repair_Mosquito
                | ExperienceType::Squad_Repair_Prowler
                | ExperienceType::Squad_Repair_Reaver
                | ExperienceType::Squad_Repair_Scythe
                | ExperienceType::Squad_Repair_Sunderer
                | ExperienceType::Squad_Repair_Vanguard
                | ExperienceType::Terminal_Repair
                | ExperienceType::Squad_MAX_Repair
                | ExperienceType::Squad_Repair_Harasser
                | ExperienceType::Vehicle_Repair_Harasser
                | ExperienceType::Squad_Repair_R_Drone
                | ExperienceType::Vehicle_Repair_R_Drone
                | ExperienceType::Shield_Repair
                | ExperienceType::Squad_Shield_Repair
                | ExperienceType::Vehicle_Repair_Valkyrie
                | ExperienceType::Squad_Repair_Valkyrie
                | ExperienceType::Vehicle_Repair_Spitfire_Turret
                | ExperienceType::Squad_Repair_Spitfire_Turret
                | ExperienceType::Squad_Repair_Construction_Med
                | ExperienceType::Repair_Construction_Med
                | ExperienceType::Squad_Repair_Construction_Small
                | ExperienceType::Repair_Construction_Small
                | ExperienceType::Squad_Repair_Construction_Large
                | ExperienceType::Repair_Construction_Large
                | ExperienceType::Squad_Repair_Construction_Core
                | ExperienceType::Repair_Construction_Core
                | ExperienceType::Vehicle_Repair_ANT
                | ExperienceType::Squad_Repair_ANT
                | ExperienceType::Vehicle_Repair_Colossus
                | ExperienceType::Squad_Repair_Colossus
                | ExperienceType::Vehicle_Repair_Javelin
                | ExperienceType::Squad_Repair_Javelin
                | ExperienceType::Door_Lock_Repair
                | ExperienceType::Containment_Site_Gate_Shield_Gen_Repair
        )
    }
}

