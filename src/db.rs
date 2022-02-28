

use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::sync::{mpsc};
use tokio::runtime::Runtime;
use sqlx::sqlite::SqlitePool;
use sqlx::{Pool, Executor, Sqlite, Row};
use crate::common::*;
use crate::session::FullCharacter;
use futures_util::TryStreamExt;


#[derive(Clone)]
pub struct DatabaseCore {
    pub conn: SqlitePool,
}

pub struct DatabaseSync {
    pub dbc: DatabaseCore,
    pub rt: Runtime,
}

impl DatabaseSync {
    pub fn save_new_char_sync(&self, char: &Character) -> bool {
        self.rt.block_on(self.dbc.save_new_char(char))
    }
    pub fn update_char_sync(&self, char: &Character) {
        self.rt.block_on(self.dbc.update_char(char));
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
    pub fn get_character_list_sync(&self, ws_out: mpsc::Sender<Message>) -> CharacterList {
        match self.rt.block_on(self.dbc.get_character_list(ws_out)) {
            Ok(c) => c,
            Err(e) => {
                println!(" Error getting character list:");
                println!("{:?}", e);
                std::process::exit(-5);
            },
        }
    }
    pub fn get_window_specs_sync(&self) -> (f64, f64) { //x_y_size {
        self.rt.block_on(self.dbc.get_window_specs())
    }
    pub fn set_window_specs_sync(&self, x: f64, y: f64) {
        self.rt.block_on(self.dbc.set_window_specs(x,y));
    }
    pub fn init_sync(&self) {
        self.rt.block_on(self.dbc.init());
    }

}

impl DatabaseCore {
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
            .execute(&self.conn).await {
                Ok(_) => {},
                Err(err) => {
                    result = false;
                    if let Some(db_err) = err.as_database_error() {
                        if db_err.message() != "UNIQUE constraint failed: characters.id".to_string() {
                            println!("Error saving new character in DB:");
                            println!("{:?}", db_err);
                            std::process::exit(-10);
                        }
                    }
                },
        }
        result
    }

    pub async fn update_char(&self, char: &Character) {}

    pub async fn update_char_with_full(&self, char: &FullCharacter) {}

    pub async fn set_char_auto_track(&self, char: &Character) {
        match sqlx::query("UPDATE characters SET auto_track = ? WHERE id IS ?;")
            .bind(char.auto_track as i64)
            .bind(&char.character_id)
            .execute(&self.conn).await {
                Ok(_) => {},
                Err(e) => {
                    println!("Error setting auto track for character in DB:");
                    println!("{:?}", e);
                    std::process::exit(-9);
                },
        }
    }

    pub async fn remove_char(&self, char: &Character) {
        match sqlx::query("DELETE FROM characters WHERE id IS ?;")
            .bind(&char.character_id)
            .execute(&self.conn).await {
                Ok(_) => {},
                Err(e) => {
                    println!("Error removing character from DB:");
                    println!("{:?}", e);
                    std::process::exit(-8);
                },
        }
    }

    pub async fn get_character_list(&self, ws_out: mpsc::Sender<Message>) -> Result<CharacterList,sqlx::Error> {
        let mut characters = CharacterList::new(ws_out);

        let mut cursor = self.conn.fetch("SELECT * FROM characters;");
        while let Some(row) = cursor.try_next().await? {
            let achar = Character {
                full_name: row.get(0),
                lower_name: row.get(1),
                outfit: row.get::<Option<String>,usize>(2),
                outfit_full: row.get::<Option<String>,usize>(3),
                character_id: row.get(4),
                auto_track: row.get(5),
                server: row.get::<i64,usize>(6).into(),
                faction: row.get::<i64,usize>(7).into(),
                to_remove: false,
                confirm_visible: false,
                to_track: false,
                changed_auto_track: false,
            };
            characters.push(achar);
        }

        Ok(characters)
    }

    pub async fn get_window_specs(&self) -> (f64,f64) { //x_y_size {
        let mut x_size:f64 = 800.0;
        let mut y_size:f64 = 480.0;
        {
            match  self.conn.fetch_one("SELECT * FROM windows WHERE name LIKE 'main' LIMIT 1;").await {
                Ok(row) => {
                    x_size = row.get(1);
                    y_size = row.get(2);
                },
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
            .execute(&self.conn).await {
                Ok(_) => {},
                Err(e) => {
                    println!("Error updating window size in DB:");
                    println!("{:?}", e);
                    std::process::exit(-7);
                },
        }
    }

    pub async fn init(&self) {
        match self.conn.execute("SELECT version FROM raspberrytracker LIMIT 1;").await {
            Err(err) => {
                if let Some(db_err) = err.as_database_error() {
                    if db_err.message()== "no such table: raspberrytracker".to_string() {
                        print!("Setting up local database...");
                        match self.conn.execute("CREATE TABLE raspberrytracker (version NUMBER);
                              INSERT INTO raspberrytracker VALUES (0.1); 
                              CREATE TABLE windows (name TEXT, width NUMBER, height NUMBER);
                              INSERT INTO windows VALUES ('main', 800.0, 480.0);
                              CREATE TABLE characters (name TEXT, lower_name TEXT, outfit TEXT, outfit_full TEXT, id TEXT NOT NULL, auto_track INTEGER, server INTEGER, faction INTEGER, PRIMARY KEY (id));
                              CREATE TABLE weapons (name TEXT, id TEXT);
                              ",).await {
                            Ok(_) => println!(" finished"),
                            Err(e) => {
                                println!(" Error:");
                                println!("{:?}", e);
                                std::process::exit(-3);
                            },
                        }
                    } else {
                        println!("sqlhuh? {:?}", db_err.message());
                    }
                }
                },
            Ok(_) => {},
        };

        match  self.conn.fetch_one("SELECT version FROM raspberrytracker LIMIT 1;").await {
            Ok(row) => println!("db ver = {}", row.get::<f64, usize>(0) ),
            Err(e) => {
                                println!("Error checking DB version:");
                                println!("{:?}", e);
                                std::process::exit(-4);
            }
        }


    }

}
