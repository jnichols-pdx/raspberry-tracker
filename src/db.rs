use crate::character::*;
use crate::character_list::*;
use crate::common::*;
use crate::events::*;
use crate::session::*;
use crate::session_list::*;
use crate::weapons::*;
use futures_util::TryStreamExt;
use sqlx::sqlite::SqlitePool;
use sqlx::{Executor, Row};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Clone)]
pub struct DatabaseCore {
    pub conn: SqlitePool,
    weapons: Arc<RwLock<BTreeMap<String, Weapon>>>,
}

#[derive(Clone)]
pub struct DatabaseSync {
    pub dbc: DatabaseCore,
    pub rt: Handle,
}

impl DatabaseSync {
    pub fn save_new_char_sync(&self, char: &Character) -> bool {
        self.rt.block_on(self.dbc.save_new_char(char))
    }
    pub fn update_char_with_full_sync(&self, char: &FullCharacter) {
        self.rt.block_on(self.dbc.update_char_with_full(char));
    }
    pub fn set_char_auto_track_sync(&self, char: &Character) {
        self.rt.block_on(self.dbc.set_char_auto_track(char));
    }
    pub fn remove_char_sync(&self, char: &Character) {
        self.rt.block_on(self.dbc.remove_char(char));
    }
    pub fn get_character_list_sync(
        &self,
        ws_out: mpsc::Sender<Message>,
        sl: Arc<RwLock<SessionList>>,
        ws_logout_out: mpsc::Sender<Message>,
    ) -> CharacterList {
        match self
            .rt
            .block_on(self.dbc.get_character_list(ws_out, sl, ws_logout_out))
        {
            Ok(c) => c,
            Err(e) => {
                println!(" Error getting character list:");
                println!("{:?}", e);
                std::process::exit(-5);
            }
        }
    }
    pub fn exist_or_download_image_sync(&mut self, name: &str, census_id: u32) -> bool {
        self.rt
            .block_on(self.dbc.exist_or_download_image(name, census_id))
    }
    pub fn get_image_sync(&self, name: &str) -> Option<Vec<u8>> {
        self.rt.block_on(self.dbc.get_image(name))
    }
    pub fn get_sessions_sync(&self) -> SessionList {
        self.rt
            .block_on(self.dbc.get_sessions(self.rt.clone()))
            .unwrap()
    }
    pub fn get_event_modes_sync(&self) -> EventViewMode {
        self.rt.block_on(self.dbc.get_event_modes())
    }
    pub fn set_event_modes_sync(&mut self, new_mode: EventViewMode) {
        self.rt.block_on(self.dbc.set_event_modes(new_mode));
    }
    pub fn get_soundset_sync(&mut self) -> Option<String> {
        self.rt.block_on(self.dbc.get_soundset())
    }
    pub fn set_soundset_sync(&mut self, new_set_name: String) {
        self.rt.block_on(self.dbc.set_soundset(new_set_name));
    }
    pub fn clear_soundset_sync(&mut self) {
        self.rt.block_on(self.dbc.clear_soundset());
    }
    pub fn store_voicepack_sync(
        &mut self,
        name: String,
        filename: String,
        author: String,
        description: String,
        keys_to_sounds_names: HashMap<String, Vec<(Vec<u8>, String)>>,
    ) {
        self.rt.block_on(self.dbc.store_voicepack(
            name,
            filename,
            author,
            description,
            keys_to_sounds_names,
        ));
    }
    pub fn load_soundsets_sync(&mut self) -> BTreeMap<String, HashMap<String, Vec<Vec<u8>>>> {
        self.rt.block_on(self.dbc.load_soundsets())
    }
    pub fn init_sync(&mut self) {
        self.rt.block_on(self.dbc.init());
    }
}

impl DatabaseCore {
    pub fn new(conn: SqlitePool) -> Self {
        DatabaseCore {
            conn,
            weapons: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub async fn save_new_char(&self, char: &Character) -> bool {
        let mut result = true;
        //characters (name TEXT, lower_name TEXT, outfit TEXT, outfit_full TEXT, id TEXT NOT NULL, auto_track INTEGER, server INTEGER, faction INTEGER)
        match sqlx::query("INSERT INTO characters VALUES (?,?,?,?,?,?,?,?);")
            .bind(&char.full_name)
            .bind(&char.lower_name)
            .bind(&char.outfit)
            .bind(&char.outfit_full)
            .bind(&char.character_id)
            .bind(char.auto_track as i64)
            .bind(char.server as i64)
            .bind(char.faction as i64)
            .execute(&self.conn)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                result = false;
                if let Some(db_err) = err.as_database_error() {
                    if db_err.message() != "UNIQUE constraint failed: characters.id" {
                        println!("Error saving new character in DB:");
                        println!("{:?}", db_err);
                        std::process::exit(-10);
                    }
                }
            }
        }
        result
    }

    pub async fn update_char_with_full(&self, char: &FullCharacter) {
        match sqlx::query("UPDATE characters SET name = ?, lower_name = ?, outfit = ?, outfit_full = ? WHERE id IS ?;")
        .bind(&char.full_name)
        .bind(&char.lower_name)
        .bind(&char.outfit)
        .bind(&char.outfit_full)
        .bind(&char.character_id)
            .execute(&self.conn).await {
                Ok(_) => {},
                Err(err) => {
                    println!("Error updating names for character in DB:");
                    println!("{:?}", err);
                    std::process::exit(-9);
                },
        }
    }

    pub async fn set_char_auto_track(&self, char: &Character) {
        match sqlx::query("UPDATE characters SET auto_track = ? WHERE id IS ?;")
            .bind(char.auto_track as i64)
            .bind(&char.character_id)
            .execute(&self.conn)
            .await
        {
            Ok(_) => {}
            Err(e) => {
                println!("Error setting auto track for character in DB:");
                println!("{:?}", e);
                std::process::exit(-9);
            }
        }
    }

    pub async fn remove_char(&self, char: &Character) {
        match sqlx::query("DELETE FROM characters WHERE id IS ?;")
            .bind(&char.character_id)
            .execute(&self.conn)
            .await
        {
            Ok(_) => {}
            Err(e) => {
                println!("Error removing character from DB:");
                println!("{:?}", e);
                std::process::exit(-8);
            }
        }
    }

    pub async fn get_character_list(
        &self,
        ws_out: mpsc::Sender<Message>,
        sl: Arc<RwLock<SessionList>>,
        ws_logout_out: mpsc::Sender<Message>,
    ) -> Result<CharacterList, sqlx::Error> {
        let mut characters = CharacterList::new(ws_out, sl, ws_logout_out);

        let mut cursor = self.conn.fetch("SELECT * FROM characters;");
        while let Some(row) = cursor.try_next().await? {
            let achar = Character {
                full_name: row.get(0),
                lower_name: row.get(1),
                outfit: row.get::<Option<String>, usize>(2),
                outfit_full: row.get::<Option<String>, usize>(3),
                character_id: row.get(4),
                auto_track: row.get(5),
                server: row.get::<i64, usize>(6).into(),
                faction: row.get::<i64, usize>(7).into(),
                to_remove: false,
                confirm_visible: false,
                to_track: false,
                changed_auto_track: false,
            };
            characters.push(achar);
        }

        Ok(characters)
    }

    pub async fn get_weapon_category(&mut self, weapon_id: &str) -> WeaponType {
        let weapon_cat;
        if weapon_id == "0" {
            weapon_cat = WeaponType::Unknown;
        } else {
            let weapons_rw = self.weapons.write().await;
            match weapons_rw.get(weapon_id) {
                Some(weapon) => weapon_cat = weapon.category,
                None => {
                    weapon_cat =
                        DatabaseCore::retrieve_and_store_weapon(weapon_id, weapons_rw, &self.conn)
                            .await
                            .category
                }
            }
        }

        weapon_cat
    }

    pub async fn get_weapon_name(&mut self, weapon_id: &str) -> String {
        let weapon_name;
        if weapon_id == "0" {
            weapon_name = "Suicide".to_owned();
        } else {
            let weapons_rw = self.weapons.write().await;
            match weapons_rw.get(weapon_id) {
                Some(weapon) => weapon_name = weapon.name.to_owned(),
                None => {
                    weapon_name =
                        DatabaseCore::retrieve_and_store_weapon(weapon_id, weapons_rw, &self.conn)
                            .await
                            .name
                }
            }
        }

        weapon_name.replace('\\', "") //Remove escape characters from API
    }

    async fn retrieve_and_store_weapon(
        weapon_id: &str,
        mut weapons_rw: tokio::sync::RwLockWriteGuard<'_, BTreeMap<String, Weapon>>,
        conn: &SqlitePool,
    ) -> Weapon {
        println!("Going to Census for {}", weapon_id);
        let mut weapon = Weapon {
            name: "".to_owned(),
            category: WeaponType::Unknown,
        };
        match lookup_weapon_name(weapon_id) {
            Err(whut) => {
                println!("{}", whut);
                weapon.name = format!("Error finding ({})", weapon_id);
            }
            Ok(weapon_details) => {
                println!("with:");
                println!("{:?}", weapon_details);
                weapon.name = weapon_details["item_list"][0]["name"]["en"]
                    .to_string()
                    .unquote();
                if weapon.name == "ul" {
                    //"null" with the n and l removed by unquote.
                    //Census didn't actually return anything. Might be a new NSO
                    //weapon that isn't reporting correctly.
                    // Known ids that trigger this: 6011526, 6011563, 6011564.
                    weapon.name = format!("Missing ({})", weapon_id);
                } else {
                    weapon.category = WeaponType::from(
                        weapon_details["item_list"][0]["item_category_id"]
                            .to_string()
                            .unquote()
                            .parse::<i64>()
                            .unwrap_or(0),
                    );
                    weapons_rw.insert(weapon_id.to_owned(), weapon.clone());
                    match sqlx::query("INSERT INTO weapons VALUES (?, ?, ?)")
                        .bind(weapon_id)
                        .bind(weapon.name.to_owned())
                        .bind(weapon.category as i64)
                        .execute(conn)
                        .await
                    {
                        Ok(_) => {}
                        Err(err) => {
                            println!("Error saving new weapon in DB:");
                            println!("{:?}", err);
                            std::process::exit(-10);
                        }
                    }
                }
            }
        }
        weapon
    }

    pub async fn init(&mut self) {
        match sqlx::migrate!().run(&self.conn).await {
            Ok(_) => {}
            Err(e) => {
                println!("Error running DB migration:");
                println!("{:?}", e);
                std::process::exit(10);
            }
        }

        match self
            .conn
            .fetch_one("SELECT version FROM raspberrytracker LIMIT 1;")
            .await
        {
            Ok(row) => println!("db ver = {}", row.get::<f64, usize>(0)),
            Err(e) => {
                println!("Error checking DB version:");
                println!("{:?}", e);
                std::process::exit(-4);
            }
        }

        let mut cursor = self.conn.fetch("SELECT * FROM weapons;");
        let mut weapons_rw = self.weapons.write().await;
        while let Some(row) = cursor.try_next().await.unwrap() {
            let weapon_id = row.get::<String, usize>(0);
            let mut category_id = row.get::<i64, usize>(2);

            println!(
                "Loading weapon: >{}< - >{}<: >{}<",
                weapon_id,
                row.get::<String, usize>(1),
                category_id
            );

            if category_id == 0 {
                println!("Going to Census for {} category", weapon_id);
                match lookup_weapon_name(&weapon_id) {
                    Err(whut) => {
                        println!("{}", whut);
                    }
                    Ok(weapon_details) => {
                        category_id = weapon_details["item_list"][0]["item_category_id"]
                            .to_string()
                            .unquote()
                            .parse::<i64>()
                            .unwrap_or(0);
                        println!("Setting category to {}", category_id);
                        match sqlx::query("UPDATE weapons SET category = ? WHERE id =  ?;")
                            .bind(category_id)
                            .bind(weapon_id)
                            .execute(&self.conn)
                            .await
                        {
                            Ok(_) => {}
                            Err(err) => {
                                println!("Error updating weapon category in DB:");
                                println!("{:?}", err);
                                std::process::exit(-52);
                            }
                        }
                    }
                }
            }

            let weapon_entry = Weapon {
                name: row.get(1),
                category: WeaponType::from(category_id),
            };

            weapons_rw.insert(row.get(0), weapon_entry);
        }
    }

    pub async fn exist_or_download_image(&mut self, name: &str, census_id: u32) -> bool {
        match sqlx::query("SELECT census_id FROM images WHERE name IS ? LIMIT 1;")
            .bind(name)
            .fetch_one(&self.conn)
            .await
        {
            Ok(_) => true,
            Err(sqlx::Error::RowNotFound) => match download_census_image(census_id) {
                Ok(response) => match response {
                    Some(image_bytes) => {
                        println!("Found image for {}", name);
                        match sqlx::query("INSERT INTO images VALUES (?,?,?);")
                            .bind(name)
                            .bind(census_id)
                            .bind(&image_bytes)
                            .execute(&self.conn)
                            .await
                        {
                            Ok(_) => true,
                            Err(err) => {
                                println!("Error saving new image in DB:");
                                println!("{:?}", err);
                                std::process::exit(-12);
                            }
                        }
                    }
                    None => false,
                },
                Err(e) => {
                    println!("{:?}", e);
                    false
                }
            },
            Err(e) => {
                println!("Error pulling image from DB:");
                println!("{:?}", e);
                std::process::exit(-11);
            }
        }
    }

    pub async fn get_image(&self, name: &str) -> Option<Vec<u8>> {
        match sqlx::query("SELECT img FROM images WHERE name IS ? LIMIT 1;")
            .bind(name)
            .fetch_one(&self.conn)
            .await
        {
            Ok(row) => Some(row.get(0)),
            Err(sqlx::Error::RowNotFound) => None,
            Err(e) => {
                println!("Error pulling image from DB:");
                println!("{:?}", e);
                std::process::exit(-13);
            }
        }
    }

    pub async fn record_event(&mut self, source: &Event, ordering: u32, session: i64) {
        let mut vehicle = None;
        if let Some(vehicle_enum) = source.vehicle {
            vehicle = Some(vehicle_enum as i64);
        }
        match sqlx::query("INSERT INTO events VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);")
            .bind(session)
            .bind(ordering as i64)
            .bind(source.kind as i64)
            .bind(source.faction as i64)
            .bind(source.br as i64)
            .bind(source.asp as i64)
            .bind(source.class as i64)
            .bind(&source.name)
            .bind(&source.weapon)
            .bind(&source.weapon_id)
            .bind(source.headshot)
            .bind(source.kdr)
            .bind(source.timestamp as i64)
            .bind(vehicle)
            .bind(&source.datetime) //text
            .execute(&self.conn)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                println!("Error saving new event in DB:");
                println!("{:?}", err);
                std::process::exit(-21);
            }
        }
    }

    pub async fn get_events_for_session(&self, session_id: i64) -> Result<EventList, sqlx::Error> {
        let mut events = EventList::new();

        let mut cursor =
            sqlx::query("SELECT * FROM events WHERE session IS ? ORDER BY ordering ASC;")
                .bind(session_id)
                .fetch(&self.conn);

        while let Some(row) = cursor.try_next().await? {
            let new_event = Event {
                kind: row.get::<i64, usize>(2).into(),
                faction: row.get::<i64, usize>(3).into(),
                br: row.get::<u8, usize>(4),
                asp: row.get::<u8, usize>(5),
                class: row.get::<i64, usize>(6).into(),
                name: row.get(7),
                weapon: row.get(8),
                weapon_id: row.get(9),
                headshot: row.get::<bool, usize>(10),
                kdr: row.get::<f32, usize>(11),
                timestamp: row.get::<i64, usize>(12),
                vehicle: row.get::<Option<i64>, usize>(13).map(|v| v.into()),
                datetime: row.get(14),
            };
            events.push(new_event);
        }
        Ok(events)
    }

    pub async fn get_weaponstats_for_session(
        &self,
        session_id: i64,
    ) -> Result<WeaponSet, sqlx::Error> {
        let mut weapon_set = WeaponSet::new();

        let mut cursor =
            sqlx::query("SELECT * FROM weaponstats WHERE session IS ? ORDER BY ordering ASC;")
                .bind(session_id)
                .fetch(&self.conn);

        while let Some(row) = cursor.try_next().await? {
            //SQLITE driver in sqlx doesn't support u64 - sqlite internally considers all Numeric
            //to be signed. While we choose u64 for our internal representations of counts that
            //make no sense to be negative this exceeds the available precision in sqlite. How
            //does the Census API internally represent these counts?
            //
            //POTENTIAL REFACTOR: it is doubtful that even the oldest active accounts will
            //accumulate more then i64::MAX shots fired. Consider using i64 internally?
            let new_initial = WeaponInitial {
                fired: row.get::<i64, usize>(8) as u64,
                hits: row.get::<i64, usize>(9) as u64,
                kills: row.get::<i64, usize>(10) as u64,
                headshots: row.get::<i64, usize>(11) as u64,
            };

            let new_weaponstat = WeaponStats::new_historical(
                row.get(3), //name
                row.get(2), //weapon_id
                new_initial,
                row.get::<u32, usize>(4),        //kills
                row.get::<u32, usize>(5),        //headshots
                row.get::<i64, usize>(6) as u64, //hits
                row.get::<i64, usize>(7) as u64, //shots fired
            );

            weapon_set.push(new_weaponstat);
        }
        Ok(weapon_set)
    }

    pub async fn get_sessions(&self, rt: Handle) -> Result<SessionList, sqlx::Error> {
        let new_sync_db = DatabaseSync {
            dbc: self.clone(),
            rt,
        };
        let mut sessions = Vec::<Session>::new();
        let mut cursor = self.conn.fetch("SELECT * FROM sessions ORDER BY id ASC;");
        while let Some(row) = cursor.try_next().await? {
            let session = Session::from_db_row(row, new_sync_db.clone()).await;
            sessions.push(session);
        }
        Ok(SessionList::new_from_vec(sessions))
    }

    pub async fn get_event_modes(&self) -> EventViewMode {
        match self
            .conn
            .fetch_one("SELECT * FROM raspberrytracker LIMIT 1;")
            .await
        {
            Ok(row) => EventViewMode {
                kills_deaths: row.get::<bool, usize>(1),
                experience: row.get::<bool, usize>(2),
                revives: row.get::<bool, usize>(3),
                vehicles: row.get::<bool, usize>(4),
                achievements: row.get::<bool, usize>(5),
            },
            Err(e) => {
                println!("Error loading Event List View Modes:");
                println!("{:?}", e);
                std::process::exit(-55);
            }
        }
    }

    pub async fn set_event_modes(&mut self, new_mode: EventViewMode) {
        match sqlx::query("UPDATE raspberrytracker SET event_kills_death = ?, event_experience = ?, event_revives = ?, event_vehicles = ?, event_achievements = ?;")
            .bind(new_mode.kills_deaths)
            .bind(new_mode.experience)
            .bind(new_mode.revives)
            .bind(new_mode.vehicles)
            .bind(new_mode.achievements)
            .execute(&self.conn)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                println!("Error updating Event List View Modes in DB:");
                println!("{:?}", err);
                std::process::exit(-56);
            }
        }
    }

    pub async fn get_soundset(&mut self) -> Option<String> {
        match sqlx::query("SELECT current_soundset FROM raspberrytracker")
            .fetch_one(&self.conn)
            .await
        {
            Ok(row) => row.get::<Option<String>, usize>(0),
            Err(err) => {
                println!("Error setting current soundset in DB:");
                println!("{:?}", err);
                std::process::exit(-63);
            }
        }
    }

    pub async fn set_soundset(&mut self, new_set_name: String) {
        match sqlx::query("UPDATE raspberrytracker SET current_soundset = ?;")
            .bind(new_set_name)
            .execute(&self.conn)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                println!("Error setting current soundset in DB:");
                println!("{:?}", err);
                std::process::exit(-57);
            }
        }
    }

    pub async fn clear_soundset(&mut self) {
        match sqlx::query("UPDATE raspberrytracker SET current_soundset = NULL;")
            .execute(&self.conn)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                println!("Error saving 'no soundset' in DB:");
                println!("{:?}", err);
                std::process::exit(-58);
            }
        }
    }

    pub async fn load_soundsets(&mut self) -> BTreeMap<String, HashMap<String, Vec<Vec<u8>>>> {
        let mut sets = BTreeMap::new();
        let mut set_cursor = self.conn.fetch("SELECT id, name FROM soundsets;");
        while let Some(set_row) = set_cursor.try_next().await.unwrap() {
            let set_id: i64 = set_row.get(0);
            let set_name: String = set_row.get(1);
            let mut sounds = HashMap::new();

            let mut cursor =
                sqlx::query("SELECT achievement, audio FROM sounds WHERE parent_set IS ?;")
                    .bind(set_id)
                    .fetch(&self.conn);
            while let Some(row) = cursor.try_next().await.unwrap() {
                let achievement: String = row.get(0);
                let sound_data: Vec<u8> = row.get(1);
                let list = sounds.entry(achievement).or_insert_with(Vec::new);
                list.push(sound_data);
            }
            sets.insert(set_name, sounds);
        }
        sets
    }

    pub async fn store_voicepack(
        &mut self,
        name: String,
        filename: String,
        author: String,
        description: String,
        keys_to_sounds_names: HashMap<String, Vec<(Vec<u8>, String)>>,
    ) {
        match sqlx::query("INSERT INTO soundsets VALUES (NULL,?,?,?,?) returning id;")
            .bind(&name)
            .bind(&filename)
            .bind(&author)
            .bind(&description)
            .fetch_one(&self.conn)
            .await
        {
            Ok(row) => {
                let set_id = row.get::<i64, usize>(0);
                println!("New soundset has DB ID: {}", set_id);

                for (key, list) in keys_to_sounds_names {
                    for (sound_data, filename) in list {
                        match sqlx::query("INSERT INTO sounds VALUES (?,?,?,?);")
                            .bind(set_id)
                            .bind(filename)
                            .bind(key.clone())
                            .bind(sound_data)
                            .execute(&self.conn)
                            .await
                        {
                            Ok(_) => {}
                            Err(err) => {
                                println!("Error saving sound data in DB:");
                                println!("{:?}", err);
                                std::process::exit(-62);
                            }
                        }
                    }
                }
            }
            Err(err) => {
                if let Some(db_err) = err.as_database_error() {
                    println!("Error saving new soundset metadata in DB:");
                    println!("{:?}", db_err);
                    std::process::exit(-61);
                }
            }
        }
    }
}
