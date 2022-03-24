use crate::character::*;
use crate::common::*;
use crate::db::DatabaseSync;
use crate::session::Session;
use egui::{Color32, ScrollArea};
use std::sync::{Arc, RwLock};
use time::OffsetDateTime;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::protocol::Message;

pub struct CharacterList {
    pub characters: Vec<Character>,
    pub new_char_name: String,
    pub message: Option<String>,
    pub websocket_out: mpsc::Sender<Message>,
    pub session_list: Arc<RwLock<Vec<Session>>>,
}

impl CharacterList {
    pub fn new(ws_out: mpsc::Sender<Message>, sl: Arc<RwLock<Vec<Session>>>) -> Self {
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
        if let Some(target) = self.find_character_by_id(target_id) {
            target.auto_track
        } else {
            false
        }
    }

    pub fn find_character_by_id(&self, target_id: String) -> Option<&Character> {
        self.characters
            .iter()
            .find(|&chara| chara.character_id.eq(&target_id))
    }

    pub fn update_entry_from_full(&mut self, newer_char: &FullCharacter) {
        if let Some(mut target) = self
            .characters
            .iter_mut()
            .find(|chara| (**chara).character_id.eq(&newer_char.character_id))
        {
            target.full_name = newer_char.full_name.to_owned();
            target.lower_name = newer_char.lower_name.to_owned();
            target.server = newer_char.server;
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
            target.character_id = newer_char.character_id.to_owned();
            target.faction = newer_char.faction;
        }
    }
}

impl ViewWithDB for CharacterList {
    fn ui(&mut self, ctx: &egui::Context, db: &DatabaseSync) {
        egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                        ui.label("Track Character: ");
                        ui.text_edit_singleline(&mut self.new_char_name);
                        if ui.button("Track").clicked() && !self.new_char_name.is_empty() {
                            match lookup_character_id(&self.new_char_name) {
                                Ok(None) => {println!("no results");
                                        self.message = Some(format!("Character \"{}\" Not Found", self.new_char_name));
                                },
                                Err(whut) => {println!("{}", whut);
                                        self.message = Some("Census Error".to_string());
                                },
                                Ok(Some(char_id)) => {
                                    println!("character_id: {}", char_id);
                                    match  lookup_new_char_details(&char_id) {
                                        Err(whut) => println!("{}", whut),
                                        Ok(details) => {
                                            println!("RAW: {:?}", details);
                                            let bob = Character::from_json(&details).unwrap();

    let _res = self.websocket_out.blocking_send(
        Message::Text(format!("{{\"service\":\"event\",\"action\":\"subscribe\",\"characters\":[\"{}\"],\"eventNames\":[\"PlayerLogin\",\"PlayerLogout\"]}}",
        bob.character_id)));

                                            if db.save_new_char_sync(&bob) {
                                                self.characters.push(bob);
                                            }
                                        },
                                    }
                                    self.message = None;
                                }
                            }

                            self.new_char_name = "".to_owned();
                        }
                });

                match &self.message {
                    Some(msg) => {
                        ui.label(egui::RichText::new(msg).color(Color32::from_rgb(200,0,0)));
                        ui.separator();
                    },
                    None => {ui.add(egui::Separator::default().spacing(20.0));},
                }

                let scroll_chars = ScrollArea::vertical().auto_shrink([false; 2]);

                for char in &mut self.characters {
                    if char.to_track {
                        match self.websocket_out
                            .blocking_send(
                                Message::Text(format!("{{\"service\":\"event\",\"action\":\"subscribe\",\"characters\":[\"{}\"],\"eventNames\":[\"Death\",\"VehicleDestroy\"]}}",
                                char.character_id).to_owned())) {
                            Err(e) => println!("dah {:?}",e),
                            Ok(_) => {
//HERE
                                match  lookup_new_char_details(&char.character_id) {
                                    Err(whut) => println!("{}", whut),
                                    Ok(details) => {

                                        let active_char = FullCharacter::from_json(&details).unwrap();

                                        db.update_char_with_full_sync(&active_char);

                                        char.full_name =  active_char.full_name.to_owned();
                                        char.lower_name =  active_char.lower_name.to_owned();
                                        char.server =  active_char.server;
                                        if let Some(outfit_alias) = &active_char.outfit {
                                            char.outfit = Some(outfit_alias.to_owned());
                                        } else {
                                            char.outfit = None;
                                        }
                                        if let Some(outfit_name) = &active_char.outfit {
                                            char.outfit_full = Some(outfit_name.to_owned());
                                        } else {
                                            char.outfit_full = None;
                                        }
                                        char.character_id =  active_char.character_id.to_owned();
                                        char.faction =  active_char.faction;

                                        {
                                            let mut session_list_rw = self.session_list.write().unwrap();
                                            session_list_rw.push(Session::new(active_char, OffsetDateTime::now_utc().unix_timestamp()));
                                        }
                                    },
                                }

                                },
                            }
                        char.to_track = false;
                    }
                    if char.changed_auto_track {

                        db.set_char_auto_track_sync(char);
                        char.changed_auto_track = false;
                    }
                }

                scroll_chars.show(ui, |ui| {
                        for char in &mut self.characters {
                            if char.to_remove {
                                db.remove_char_sync(char);
        let _res = self.websocket_out.blocking_send(
            Message::Text(format!("{{\"service\":\"event\",\"action\":\"clearSubscribe\",\"characters\":[\"{}\"]}}",
            char.character_id).to_owned()));
                            }
                        }

                        self.characters.retain(|char| !char.to_remove);
                        for char in &mut self.characters {
                                char.draw(ui);
                                ui.separator();
                        }
                });
        });
    }
    fn draw(&mut self, _ui: &mut egui::Ui) {}
}
