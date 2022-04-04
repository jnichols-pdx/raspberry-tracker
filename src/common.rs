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

    ExperienceTick = 10,
    Achievement = 11,
    Revived = 12,

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
    DropPodAlt = 2008,
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
            Vehicle::DropPodAlt => write!(f, "Droppod1"),
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

impl Class {
    pub fn is_max(&self) -> bool {
        matches!(
            self,
            Class::NCMax | Class::TRMax | Class::VSMax | Class::NSOMax
        )
    }

    pub fn is_infil(&self) -> bool {
        matches!(
            self,
            Class::NCInfiltrator
                | Class::TRInfiltrator
                | Class::VSInfiltrator
                | Class::NSOInfiltrator
        )
    }
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

pub fn big_print_num(number: &str) {
    println!();
    for slice in 0..=7 {
        for numeral in number.chars() {
            match numeral {
                '0' => print_0(slice),
                '1' => print_1(slice),
                '2' => print_2(slice),
                '3' => print_3(slice),
                '4' => print_4(slice),
                '5' => print_5(slice),
                '6' => print_6(slice),
                '7' => print_7(slice),
                '8' => print_8(slice),
                '9' => print_9(slice),
                ' ' => print!("       "),
                '.' => print_dot(slice),
                _ => print!(" █████ "),
            }
        }
        println!();
    }
    println!();

    fn print_0(slice: usize) {
        match slice {
            0 => print!("  ###  "),
            1 => print!(" #   # "),
            2 => print!(" #   # "),
            3 => print!(" # | # "),
            4 => print!(" # | # "),
            5 => print!(" #   # "),
            6 => print!(" #   # "),
            7 => print!("  ###  "),
            _ => {}
        }
    }

    fn print_1(slice: usize) {
        match slice {
            0 => print!("   #   "),
            1 => print!("  ##   "),
            2 => print!("   #   "),
            3 => print!("   #   "),
            4 => print!("   #   "),
            5 => print!("   #   "),
            6 => print!("   #   "),
            7 => print!("  ###  "),
            _ => {}
        }
    }

    fn print_2(slice: usize) {
        match slice {
            0 => print!("  ###  "),
            1 => print!(" #   # "),
            2 => print!("     # "),
            3 => print!("    #  "),
            4 => print!("   #   "),
            5 => print!("  #    "),
            6 => print!(" #     "),
            7 => print!(" ##### "),
            _ => {}
        }
    }

    fn print_3(slice: usize) {
        match slice {
            0 => print!("  ###  "),
            1 => print!(" #   # "),
            2 => print!("     # "),
            3 => print!("   ##  "),
            4 => print!("     # "),
            5 => print!("     # "),
            6 => print!(" #   # "),
            7 => print!("  ###  "),
            _ => {}
        }
    }

    fn print_4(slice: usize) {
        match slice {
            0 => print!("    #  "),
            1 => print!("   ##  "),
            2 => print!("  # #  "),
            3 => print!(" #  #  "),
            4 => print!(" ##### "),
            5 => print!("    #  "),
            6 => print!("    #  "),
            7 => print!("    #  "),
            _ => {}
        }
    }

    fn print_5(slice: usize) {
        match slice {
            0 => print!(" ##### "),
            1 => print!(" #     "),
            2 => print!(" #     "),
            3 => print!(" ####  "),
            4 => print!("     # "),
            5 => print!("     # "),
            6 => print!(" #   # "),
            7 => print!("  ###  "),
            _ => {}
        }
    }

    fn print_6(slice: usize) {
        match slice {
            0 => print!("   ##  "),
            1 => print!("  #    "),
            2 => print!(" #     "),
            3 => print!(" ####  "),
            4 => print!(" #   # "),
            5 => print!(" #   # "),
            6 => print!(" #   # "),
            7 => print!("  ###  "),
            _ => {}
        }
    }

    fn print_7(slice: usize) {
        match slice {
            0 => print!(" ##### "),
            1 => print!("     # "),
            2 => print!("    #  "),
            3 => print!("    #  "),
            4 => print!("   #   "),
            5 => print!("   #   "),
            6 => print!("  #    "),
            7 => print!("  #    "),
            _ => {}
        }
    }

    fn print_8(slice: usize) {
        match slice {
            0 => print!("  ###  "),
            1 => print!(" #   # "),
            2 => print!(" #   # "),
            3 => print!("  ###  "),
            4 => print!(" #   # "),
            5 => print!(" #   # "),
            6 => print!(" #   # "),
            7 => print!("  ###  "),
            _ => {}
        }
    }

    fn print_9(slice: usize) {
        match slice {
            0 => print!("  ###  "),
            1 => print!(" #   # "),
            2 => print!(" #   # "),
            3 => print!(" #   # "),
            4 => print!("  #### "),
            5 => print!("     # "),
            6 => print!("    #  "),
            7 => print!("  ##   "),
            _ => {}
        }
    }

    fn print_dot(slice: usize) {
        match slice {
            0 => print!("    "),
            1 => print!("    "),
            2 => print!("    "),
            3 => print!("    "),
            4 => print!("    "),
            5 => print!("    "),
            6 => print!(" ## "),
            7 => print!(" ## "),
            _ => {}
        }
    }
}

#[derive(Copy, Clone)]
pub struct EventViewMode {
    pub kills_deaths: bool,
    pub experience: bool,
    pub revives: bool,
    pub vehicles: bool,
    pub achievements: bool,
}
