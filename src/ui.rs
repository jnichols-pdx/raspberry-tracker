#![allow(unused_variables)]
use eframe::{egui, epi};
use egui::*;
use tokio::sync::{mpsc};
use crate::common::*;
use sqlite::State;
use tokio_tungstenite::tungstenite::protocol::Message;

pub struct TrackerApp {
    pub from_main: mpsc::Receiver<Action>,
    pub in_character_ui: bool,
    pub char_list: CharacterList,
    pub db: sqlite::Connection,
    pub lastx: f32,
    pub lasty: f32,
    pub size_changed: bool,
}

impl ViewWithDB for CharacterList {
    fn ui(&mut self, ctx: &egui::CtxRef, db: &sqlite::Connection) {
        egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                        ui.label("Track Character: ");
                        ui.text_edit_singleline(&mut self.new_char_name);
                        if ui.button("Track").clicked() {
                            if self.new_char_name != "".to_owned() {

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
                                                let new_char = &details["character_list"][0];
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
                                                };

                                                if new_char["outfit"].is_object() {
                                                    bob.outfit = Some(new_char["outfit"]["alias"].to_string().unquote());
                                                    bob.outfit_full = Some(new_char["outfit"]["name"].to_string().unquote());
                                                }

                                                if db_save_new_char(&bob, db) {
                                                    self.characters.push(bob);
                                                }
                                            },
                                        }
                                        self.message = None;
                                    }
                                }

                                self.new_char_name = "".to_owned();
                            }
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
                                Message::Text(format!("{{\"service\":\"event\",\"action\":\"subscribe\",\"characters\":[\"{}\"],\"eventNames\":[\"Death\"]}}",
                                char.character_id).to_owned())) {
                            Err(e) => println!("dah {:?}",e),
                            Ok(_) => {},
                            }
                        char.to_track = false;
                    }
                }

                scroll_chars.show(ui, |ui| {
                        for char in &mut self.characters {
                            if char.to_remove {
                                db_remove_char(&char, db);
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
    fn draw(&mut self, _ui: &mut egui::Ui) {
    }
}

impl View for Character {
    fn ui(&mut self, _ctx: &egui::CtxRef) {
    }
    fn draw(&mut self, ui: &mut egui::Ui){
        ui.horizontal(|ui| {
            ui.label(&self.full_name);
            if let Some(outfit_name) = &self.outfit {
                ui.label(outfit_name);
            } else {
                ui.label("<no outfit>");
            }
            ui.label(name_from_world(self.server));
            if self.confirm_visible {
                ui.label(egui::RichText::new("Actually remove this character?".to_owned()).color(Color32::from_rgb(200,0,0)));
            }
        });
        ui.horizontal(|ui| {
            ui.label(&self.character_id);
            ui.label(name_from_faction(self.faction));
            ui.checkbox(&mut self.auto_track, "Auto Track");
            if !self.auto_track {
                if ui.button("Start Session").clicked() {
                    self.to_track = true;
                }
            }
            if !self.confirm_visible {
                if ui.button("remove").clicked() {
                    self.confirm_visible= true;
                }
            } else {
                if ui.button(" cancel ").clicked() {
                    self.confirm_visible = false;
                }
                if ui.button("confirm").clicked() {
                    self.to_remove = true;
                }

            }
        });
    }
}


impl epi::App for TrackerApp {
    fn name(&self) -> &str {
        "Raspberry Tracker"
    }
   
   /*
    /// Called once before UI first renders
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: *epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
            //load previous apps tate as held by egui.. won't be using this?
    }*/

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        //let Self { in_character_ui, from_main, char_list, db, lastx, lasty, size_changed} = self;

        //can access "window size" via ctx.available_rect();
        let mut newchange = false;
        //println!("{:?}",ctx.available_rect());
        let thisrect = ctx.available_rect();
        if self.lastx != thisrect.max.x {
            self.size_changed = true;
            newchange = true;
            self.lastx = thisrect.max.x
        }
        if self.lasty != thisrect.max.y {
            self.size_changed = true;
            newchange = true;
            self.lasty = thisrect.max.y
        }
        if self.size_changed && !newchange  {
            self.size_changed = false;
            //println!("bing!");

            let mut statement = self.db
                .prepare("UPDATE windows SET width = ?, height = ? WHERE name LIKE 'main';").unwrap();
            statement.bind(1,self.lastx as f64).unwrap();
            statement.bind(2,self.lasty as f64).unwrap();
            while let State::Row = statement.next().unwrap() {};

        }


        egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
            // thin topmost panel for menubar
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_strip").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Version 0.10");
                egui::warn_if_debug_build(ui);
                });
            });
        });

        egui::SidePanel::left("picker_panel").show(ctx, |ui| {
            ui.heading("Sessions");
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {

            if ui.button("characters").clicked() {
                self.char_list.message = None;
                self.in_character_ui = ! self.in_character_ui;
            }
            });


        });

        if self.in_character_ui {
            self.char_list.ui(&ctx, &self.db);

        }
        else
        {

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            //ui.heading(format!("{} Stats", new_char_name));
            ui.heading("<char> Stats");
        });

        egui::SidePanel::right("events_panel").show(ctx, |ui| {
            ui.heading("Event feed");
        });
        }

    }
}

