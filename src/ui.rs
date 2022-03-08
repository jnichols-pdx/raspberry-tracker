#![allow(unused_variables)]
use eframe::{egui, epi};
use egui::*;
use tokio::sync::{mpsc, oneshot};
use crate::common::*;
use crate::session::*;
use crate::db::*;
use sqlx::sqlite::SqlitePool;
use std::sync::{Arc, RwLock};
use tokio_tungstenite::tungstenite::protocol::Message;
use image::io::Reader as ImageReader;
use std::io::Cursor;

pub struct TrackerApp {
    //pub from_main: mpsc::Receiver<Action>,
    pub in_character_ui: bool,
    pub char_list: Arc<RwLock<CharacterList>>,
    pub session_list: Arc<RwLock<Vec<Session>>>,
    pub db: DatabaseSync,
    pub lastx: f32,
    pub lasty: f32,
    pub size_changed: bool,
    pub ws_messages: mpsc::Receiver<serde_json::Value>,
    pub ws_out: mpsc::Sender<Message>,
    pub frame_cb: Option<oneshot::Sender<epi::Frame>>,
    pub session_count: usize,
    pub images: Option<Vec<TextureHandle>>,
}

impl ViewWithDB for CharacterList {
    fn ui(&mut self, ctx: &egui::Context, db: &DatabaseSync) {
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
                                                let bob = character_from_json(&details).unwrap();

        let _res = self.websocket_out.blocking_send(
            Message::Text(format!("{{\"service\":\"event\",\"action\":\"subscribe\",\"characters\":[\"{}\"],\"eventNames\":[\"PlayerLogin\",\"PlayerLogout\"]}}",
            bob.character_id).to_owned()));

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
                    if char.changed_auto_track {

                        db.set_char_auto_track_sync(&char);
                        char.changed_auto_track = false;
                    }
                }

                scroll_chars.show(ui, |ui| {
                        for char in &mut self.characters {
                            if char.to_remove {
                                db.remove_char_sync(&char);
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
    fn draw(&mut self, _ui: &mut egui::Ui) {
    }
}

impl View for Character {
    fn ui(&mut self, _ctx: &egui::Context) {
    }
    fn draw(&mut self, ui: &mut egui::Ui){
        egui::Grid::new(format!("charctrs{}", self.character_id))
            .min_col_width(10.0)
            .show(ui, |ui| {
            match ui.ctx().texture_by_name(&self.faction.to_string()) {
                Some(image) => ui.image(image.id(), (28.0,28.0)),
                None => ui.label(self.faction.to_string()),
            };
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if let Some(outfit_name) = &self.outfit {
                        ui.label(format!("[{}] {}", outfit_name, self.full_name));
                    } else {
                        ui.label(&self.full_name);
                    }
                    ui.label(self.server.to_string());
                });
                ui.horizontal(|ui| {
                    ui.label(&self.character_id);
                });
            });
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                        ui.label(""); //hold space open
                });
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut self.auto_track, "Auto Track").clicked() {
                        self.changed_auto_track = true;
                    }
                    if !self.auto_track {
                        if ui.button("Start Session").clicked() {
                            self.to_track = true;
                        }
                    }
                });
            });
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if self.confirm_visible {
                        ui.label(egui::RichText::new("Actually remove this character?".to_owned()).color(Color32::from_rgb(200,0,0)));
                    } else {
                        ui.label(""); //hold space open
                    }
                });
                ui.horizontal(|ui| {
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
            });
            ui.end_row();
        });
    }
}

impl TrackerApp {

    fn load_image_bytes(&mut self, image_name: &str, bytes: &[u8], ctx: &egui::Context) {
        match  ImageReader::with_format(Cursor::new(bytes), image::ImageFormat::Png)
            .decode() {
                Ok(image) => {
                    let size = [image.width() as usize, image.height() as usize];
                    let image_buffer = image.to_rgba8();
                    let pixels = image_buffer.as_flat_samples();
                    match self.images.as_mut() {
                        Some(list) => {
                            list.push(ctx.load_texture(image_name, ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())));
                            println!("Readied Custom : {}", image_name);
                        },
                        None => {},
                    }
                },
                Err(e) => {},
        }
    }
}

impl epi::App for TrackerApp {
    fn name(&self) -> &str {
        "Raspberry Tracker"
    }
   

    /// Called once before UI first renders
    fn setup(
        &mut self,
        ctx: &egui::Context,
        frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(callback) = self.frame_cb.take() {
            let _blah = callback.send(frame.clone());
        }
        ctx.set_visuals(egui::Visuals::dark()); 

        self.images = Some(Vec::new());



        
        for (name, census_id) in master_images() {
            match self.db.exist_or_download_image_sync(&name, census_id) {
                true => {
                        match self.db.get_image_sync(&name) {
                            Some(image_bytes) => {
                                match  ImageReader::with_format(Cursor::new(image_bytes), image::ImageFormat::Png)
                                    .decode() {
                                        Ok(image) => {
                                            let size = [image.width() as usize, image.height() as usize];
                                            let image_buffer = image.to_rgba8();
                                            let pixels = image_buffer.as_flat_samples();
                                            match self.images.as_mut() {
                                                Some(list) => {
                                                    list.push(ctx.load_texture(name, ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())));
                                                    //println!("Readied {}: {}", census_id, name);
                                                },
                                                None => {},
                                            }
                                        },
                                        Err(e) => {

                                        },
                                }
                            },
                            None => {},
                        }
                    },
                false => println!("Unable to load image {}: {}", census_id, name),
            };
        }

        self.load_image_bytes("Robit", include_bytes!("../Images/NSO.png"), ctx);
        self.load_image_bytes("Headshot", include_bytes!("../Images/Headshot.png"), ctx);
        self.load_image_bytes("Pumpkin", include_bytes!("../Images/Pumpkin.png"), ctx);
        self.load_image_bytes("ManaAVTurret", include_bytes!("../Images/ManaAVTurret.png"), ctx);
        self.load_image_bytes("Flail", include_bytes!("../Images/Flail.png"), ctx);
        self.load_image_bytes("Glaive", include_bytes!("../Images/Glaive.png"), ctx);
        self.load_image_bytes("BastionFleetCarrier", include_bytes!("../Images/BastionFleetCarrier.png"), ctx);


    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        //let Self { in_character_ui, from_main, char_list, db, lastx, lasty, size_changed} = self;

        match self.ws_messages.try_recv() {
            Err(mpsc::error::TryRecvError::Empty) => {},
            Err(_) => {println!("x");},
            Ok(json) => {
                println!("j");
                }
        }

        {
            let session_list_ro = self.session_list.read().unwrap();
            if self.session_count < session_list_ro.len() {
                self.in_character_ui = false;
                self.session_count = session_list_ro.len();
            }
        }
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
            self.db.set_window_specs_sync(self.lastx as f64, self.lasty as f64);
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
                let mut char_list_rw = self.char_list.write().unwrap();
                char_list_rw.message = None;
                self.in_character_ui = ! self.in_character_ui;
            }
            });


        });

        if self.in_character_ui {
            //Mutable write access required because UI code handles adding / removing characters to the list.
            let mut char_list_rw = self.char_list.write().unwrap();
            char_list_rw.ui(&ctx, &self.db);

        }
        else
        {
        
            let session_list_ro = self.session_list.read().unwrap();
            if let Some(session) = session_list_ro.last() {
                session.ui(&ctx);
            }
            
        }

    }

}

impl TextureLookup for egui::Context {
    fn texture_by_name(&self, name: &str) -> Option<egui::TextureHandle>{
        let mut new_handle = None;
        let manager = self.tex_manager();
        let manager_cloned = manager.clone();
        {
            let mut manager_rw = manager.write();
            for (id, meta) in manager_rw.allocated() {
                if meta.name == name {
                    new_handle = Some(TextureHandle::new(manager_cloned, id.clone()));
                    break;
                }
            }
            if let Some(ref nh) = new_handle {
                 manager_rw.retain(nh.id());
            }
        }
        new_handle
    }
}

