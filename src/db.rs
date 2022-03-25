use crate::character::*;
use crate::character_list::*;
use crate::common::*;
use crate::events::Event;
use crate::session::*;
use crate::weapons::WeaponStats;
use futures_util::TryStreamExt;
use sqlx::sqlite::SqlitePool;
use sqlx::{Executor, Row};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Clone)]
pub struct DatabaseCore {
    pub conn: SqlitePool,
    weapons: BTreeMap<String, String>,
}

pub struct DatabaseSync {
    pub dbc: DatabaseCore,
    pub rt: Runtime,
}

impl DatabaseSync {
    pub fn save_new_char_sync(&self, char: &Character) -> bool {
        self.rt.block_on(self.dbc.save_new_char(char))
    }
    /*pub fn update_char_sync(&self, char: &Character) {
        self.rt.block_on(self.dbc.update_char(char));
    }*/
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
        sl: Arc<RwLock<Vec<Session>>>,
    ) -> CharacterList {
        match self.rt.block_on(self.dbc.get_character_list(ws_out, sl)) {
            Ok(c) => c,
            Err(e) => {
                println!(" Error getting character list:");
                println!("{:?}", e);
                std::process::exit(-5);
            }
        }
    }
    pub fn save_new_session_sync(&self, new_session: &mut Session) {
        self.rt.block_on(new_session.save_to_db(&self.dbc));
    }
    pub fn get_window_specs_sync(&self) -> (f32, f32) {
        //x_y_size {
        self.rt.block_on(self.dbc.get_window_specs())
    }
    pub fn set_window_specs_sync(&self, x: f64, y: f64) {
        self.rt.block_on(self.dbc.set_window_specs(x, y));
    }
    pub fn exist_or_download_image_sync(&mut self, name: &str, census_id: u32) -> bool {
        self.rt
            .block_on(self.dbc.exist_or_download_image(name, census_id))
    }
    pub fn get_image_sync(&self, name: &str) -> Option<Vec<u8>> {
        self.rt.block_on(self.dbc.get_image(name))
    }
    pub fn init_sync(&mut self) {
        self.rt.block_on(self.dbc.init());
    }
}

impl DatabaseCore {
    pub fn new(conn: SqlitePool) -> Self {
        DatabaseCore {
            conn,
            weapons: BTreeMap::new(),
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
        sl: Arc<RwLock<Vec<Session>>>,
    ) -> Result<CharacterList, sqlx::Error> {
        let mut characters = CharacterList::new(ws_out, sl);

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

    pub async fn get_window_specs(&self) -> (f32, f32) {
        //x_y_size {
        let x_size: f32;
        let y_size: f32;
        {
            match self
                .conn
                .fetch_one("SELECT * FROM windows WHERE name LIKE 'main' LIMIT 1;")
                .await
            {
                Ok(row) => {
                    x_size = row.get(1);
                    y_size = row.get(2);
                }
                Err(e) => {
                    println!("Error getting window size from DB:");
                    println!("{:?}", e);
                    std::process::exit(-6);
                }
            }
        }
        (x_size, y_size)
    }

    pub async fn set_window_specs(&self, x: f64, y: f64) {
        match sqlx::query("UPDATE windows SET width = ?, height = ? WHERE name LIKE 'main';")
            .bind(x)
            .bind(y)
            .execute(&self.conn)
            .await
        {
            Ok(_) => {}
            Err(e) => {
                println!("Error updating window size in DB:");
                println!("{:?}", e);
                std::process::exit(-7);
            }
        }
    }

    pub async fn get_weapon_name(&mut self, weapon_id: &str) -> String {
        let mut weapon_name;
        if weapon_id == "0" {
            weapon_name = "Suicide".to_owned(); //applies for crashing vehicles... but what of roadkills / fall damage?
        } else {
            match self.weapons.get(weapon_id) {
                Some(weapon) => weapon_name = weapon.replace('\\', ""), //Remove escape characters from API
                None => {
                    println!("Going to Census for {}", weapon_id);
                    match lookup_weapon_name(weapon_id) {
                        Err(whut) => {
                            println!("{}", whut);
                            weapon_name = format!("Error finding ({})", weapon_id);
                        }
                        Ok(weapon) => {
                            println!("with:");
                            println!("{:?}", weapon);
                            weapon_name =
                                weapon["item_list"][0]["name"]["en"].to_string().unquote();
                            if weapon_name == "ul" {
                                //"null" with the n and l removed by unquote.
                                //Census didn't actually return anything. Might be a new NSO
                                //weapon that isn't reporting correctly.
                                // Known ids that trigger this: 6011526, 6011563, 6011564.
                                weapon_name = format!("Missing ({})", weapon_id);
                            } else {
                                self.weapons
                                    .insert(weapon_id.to_owned(), weapon_name.to_owned());
                                match sqlx::query("INSERT INTO weapons VALUES (?, ?)")
                                    .bind(weapon_id)
                                    .bind(weapon_name.to_owned())
                                    .execute(&self.conn)
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
                }
            }
        }

        weapon_name
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
        while let Some(row) = cursor.try_next().await.unwrap() {
            println!(
                "Loading weapon: >{}< - >{}<",
                row.get::<String, usize>(0),
                row.get::<String, usize>(1)
            );
            self.weapons.insert(row.get(0), row.get(1));
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

    pub async fn update_session(&mut self, source: &Session) {}

    pub async fn record_event(&mut self, source: &Event, ordering: u32, session: u32) {}

    pub async fn record_weaponstats(&mut self, source: &WeaponStats, ordering: u32, session: u32) {}

    pub async fn update_weaponstats(&mut self, source: &WeaponStats, session: u32) {}
}
