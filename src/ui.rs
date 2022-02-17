#![allow(unused_variables)]
use eframe::{egui, epi};
use egui::*;
use tokio::sync::{mpsc};
use crate::common::*;

pub struct TrackerApp {
    pub from_main: mpsc::Receiver<Action>,
    pub in_character_ui: bool,
    pub char_list: CharacterList,
}

impl View for CharacterList {
    fn ui(&mut self, ctx: &egui::CtxRef) {
        egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                        ui.label("Track Character: ");
                        ui.text_edit_singleline(&mut self.new_char_name);
                        if ui.button("Track").clicked() {
                            if self.new_char_name != "".to_owned() {

                                match lookup_character_id(&self.new_char_name) {
                                    Ok(None) => println!("no results"),
                                    Err(whut) => println!("{}", whut),
                                    Ok(Some(char_id)) => {
                                        println!("character_id: {}", char_id);
                                        match  lookup_new_char_details(&char_id) {
                                            Err(whut) => println!("{}", whut),
                                            Ok(details) => {
                                                let new_char = &details["character_list"][0];
                                                println!("deets: {:?}", new_char);
                                                let faction_num = new_char["faction_id"].to_string().unquote().parse::<u8>().unwrap();
                                                let world_num = new_char["world_id"].to_string().unquote().parse::<u8>().unwrap();

                                                let mut bob = Character {
                                                    full_name: new_char["name"]["first"].to_string().unquote(),
                                                    lower_name: new_char["name"]["first_lower"].to_string().unquote(),
                                                    server: World::from(world_num),
                                                    outfit: None,
                                                    outfit_full: None,
                                                    character_id: new_char["character_id"].to_string().unquote(),
                                                    auto_track: true,
                                                    faction: Faction::from(faction_num),
                                                };

                                                if new_char["outfit"].is_object() {
                                                    bob.outfit = Some(new_char["outfit"]["alias"].to_string().unquote());
                                                    bob.outfit_full = Some(new_char["outfit"]["name"].to_string().unquote());
                                                }
                                                self.characters.push(bob);


                                            },
                                        }
                                    }
                                }


                                //self.characters.push(Character::new(self.new_char_name.to_owned()));
                                self.new_char_name = "".to_owned();
                            }
                        }
                });

                ui.add(egui::Separator::default().spacing(20.0));

                let scroll_chars = ScrollArea::vertical().auto_shrink([false; 2]);

                scroll_chars.show(ui, |ui| {
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
        });
        ui.horizontal(|ui| {
            ui.label(&self.character_id);
            ui.label(name_from_faction(self.faction));
            ui.checkbox(&mut self.auto_track, "Auto Track");
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
        let Self { in_character_ui, from_main, char_list} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

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
                self.in_character_ui = ! self.in_character_ui;
            }
            });


        });

        if self.in_character_ui {
            self.char_list.ui(&ctx);

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

