use crate::achievements::*;
use crate::character_list::*;
use crate::common::*;
use crate::db::*;
use crate::session_list::*;
use crate::vpack::*;
use eframe::egui;
use egui::*;
use image::io::Reader as ImageReader;
use std::io::Cursor;
use std::sync::Arc;
use tokio::sync::{oneshot, RwLock};

pub struct TrackerApp {
    pub in_character_ui: bool,
    pub char_list: Arc<RwLock<CharacterList>>,
    pub session_list: Arc<RwLock<SessionList>>,
    pub db: DatabaseSync,
    pub size_changed: bool,
    pub session_count: usize,
    pub images: Option<Vec<TextureHandle>>,
    pub event_list_mode: EventViewMode,
    filter_text: String,
    pub achievements: Arc<RwLock<AchievementEngine>>,
}

impl TrackerApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        char_list: Arc<RwLock<CharacterList>>,
        session_list: Arc<RwLock<SessionList>>,
        db: DatabaseSync,
        mut context_cb: Option<oneshot::Sender<egui::Context>>,
        achievements: Arc<RwLock<AchievementEngine>>,
    ) -> Self {
        let initial_count;
        {
            let session_list_rw = session_list.blocking_read();
            initial_count = session_list_rw.len();
        }

        let event_list_mode = db.get_event_modes_sync();

        let mut app_ui = Self {
            in_character_ui: true,
            char_list,
            session_list,
            db,
            size_changed: false,
            session_count: initial_count,
            images: None,
            event_list_mode,
            filter_text: "".to_string(),
            achievements,
        };

        if let Some(callback) = context_cb.take() {
            let _blah = callback.send(cc.egui_ctx.clone());
        }
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        app_ui.images = Some(Vec::new());

        for (name, census_id) in master_images() {
            match app_ui.db.exist_or_download_image_sync(&name, census_id) {
                true => {
                    if let Some(image_bytes) = app_ui.db.get_image_sync(&name) {
                        if let Ok(image) = ImageReader::with_format(
                            Cursor::new(image_bytes),
                            image::ImageFormat::Png,
                        )
                        .decode()
                        {
                            let size = [image.width() as usize, image.height() as usize];
                            let image_buffer = image.to_rgba8();
                            let pixels = image_buffer.as_flat_samples();
                            if let Some(list) = app_ui.images.as_mut() {
                                list.push(cc.egui_ctx.load_texture(
                                    name,
                                    ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
                                    egui::TextureFilter::Linear,
                                ));
                            }
                        };
                    };
                }
                false => println!("Unable to load image {}: {}", census_id, name),
            };
        }

        app_ui.load_image_bytes("Robit", include_bytes!("../Images/NSO.png"), &cc.egui_ctx);
        app_ui.load_image_bytes("Robit-VS", include_bytes!("../Images/NSO-VS.png"), &cc.egui_ctx);
        app_ui.load_image_bytes("Robit-NC", include_bytes!("../Images/NSO-NC.png"), &cc.egui_ctx);
        app_ui.load_image_bytes("Robit-TR", include_bytes!("../Images/NSO-TR.png"), &cc.egui_ctx);
        app_ui.load_image_bytes(
            "Headshot",
            include_bytes!("../Images/Headshot.png"),
            &cc.egui_ctx,
        );
        app_ui.load_image_bytes(
            "Pumpkin",
            include_bytes!("../Images/Pumpkin.png"),
            &cc.egui_ctx,
        );
        app_ui.load_image_bytes(
            "AV Mana Turret",
            include_bytes!("../Images/ManaAVTurret.png"),
            &cc.egui_ctx,
        );
        app_ui.load_image_bytes("Flail", include_bytes!("../Images/Flail.png"), &cc.egui_ctx);
        app_ui.load_image_bytes(
            "Glaive",
            include_bytes!("../Images/Glaive.png"),
            &cc.egui_ctx,
        );
        app_ui.load_image_bytes(
            "AI Tower (Construction)",
            include_bytes!("../Images/AITower.png"),
            &cc.egui_ctx,
        );
        app_ui.load_image_bytes(
            "AV Tower (Construction)",
            include_bytes!("../Images/AVTower.png"),
            &cc.egui_ctx,
        );
        app_ui.load_image_bytes(
            "AA Tower (Construction)",
            include_bytes!("../Images/AATower.png"),
            &cc.egui_ctx,
        );
        app_ui.load_image_bytes(
            "Bastion Fleet Carrier",
            include_bytes!("../Images/BastionFleetCarrier.png"),
            &cc.egui_ctx,
        );

        {
            let session_list_ro = app_ui.session_list.blocking_read();
            if session_list_ro.active_session().is_some() {
                app_ui.in_character_ui = false;
            }
        }

        app_ui
    }

    fn load_image_bytes(&mut self, image_name: &str, bytes: &[u8], ctx: &egui::Context) {
        if let Ok(image) =
            ImageReader::with_format(Cursor::new(bytes), image::ImageFormat::Png).decode()
        {
            let size = [image.width() as usize, image.height() as usize];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            if let Some(list) = self.images.as_mut() {
                list.push(ctx.load_texture(
                    image_name,
                    ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
                    egui::TextureFilter::Linear,
                ));
                println!("Readied Custom : {}", image_name);
            }
        };
    }
}

impl eframe::App for TrackerApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        {
            let session_list_ro = self.session_list.blocking_read();
            if self.session_count < session_list_ro.len() {
                self.in_character_ui = false;
                self.session_count = session_list_ro.len();
            }
        }

        egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
            // thin topmost panel for menubar
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if !self.in_character_ui {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.filter_text);
                        ui.label("Filter");
                    });
                }
                egui::menu::bar(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Quit").clicked() {
                                frame.close();
                            }
                        });
                        ui.menu_button("View", |ui| {
                            if self.event_list_mode.kills_deaths {
                                if ui.button("Hide Kills/Death Events").clicked() {
                                    self.event_list_mode.kills_deaths = false;
                                    self.db.set_event_modes_sync(self.event_list_mode);
                                }
                            } else if ui.button("Show Kills/Death Events").clicked() {
                                self.event_list_mode.kills_deaths = true;
                                self.db.set_event_modes_sync(self.event_list_mode);
                            }

                            if self.event_list_mode.experience {
                                if ui.button("Hide Experience Gain Events").clicked() {
                                    self.event_list_mode.experience = false;
                                    self.db.set_event_modes_sync(self.event_list_mode);
                                }
                            } else if ui.button("Show Experience Gain Events").clicked() {
                                self.event_list_mode.experience = true;
                                self.db.set_event_modes_sync(self.event_list_mode);
                            }

                            if self.event_list_mode.revives {
                                if ui.button("Hide Revive Events").clicked() {
                                    self.event_list_mode.revives = false;
                                    self.db.set_event_modes_sync(self.event_list_mode);
                                }
                            } else if ui.button("Show Revive Events").clicked() {
                                self.event_list_mode.revives = true;
                                self.db.set_event_modes_sync(self.event_list_mode);
                            }

                            if self.event_list_mode.vehicles {
                                if ui.button("Hide Vehicle Destroyed Events").clicked() {
                                    self.event_list_mode.vehicles = false;
                                    self.db.set_event_modes_sync(self.event_list_mode);
                                }
                            } else if ui.button("Show Vehicle Destroyed Events").clicked() {
                                self.event_list_mode.vehicles = true;
                                self.db.set_event_modes_sync(self.event_list_mode);
                            }

                            if self.event_list_mode.achievements {
                                if ui.button("Hide Achievements Events").clicked() {
                                    self.event_list_mode.achievements = false;
                                    self.db.set_event_modes_sync(self.event_list_mode);
                                }
                            } else if ui.button("Show Achievements Events").clicked() {
                                self.event_list_mode.achievements = true;
                                self.db.set_event_modes_sync(self.event_list_mode);
                            }
                        });
                        ui.menu_button("Sounds", |ui| {
                            let soundset_names;
                            let current_soundset;
                            {
                                let achievements_ro = self.achievements.blocking_read();
                                soundset_names = achievements_ro.list_soundsets();
                                current_soundset = achievements_ro.active_soundset_name();
                            }
                            for soundset_name in soundset_names {
                                let option_name;
                                if let Some(ref active_name) = current_soundset {
                                    if active_name.eq(&soundset_name) {
                                        option_name = format!("{} ✔", soundset_name);
                                    } else {
                                        option_name = soundset_name.clone();
                                    }
                                } else {
                                    option_name = soundset_name.clone();
                                }
                                if ui.button(option_name).clicked() {
                                    let mut achievements_rw = self.achievements.blocking_write();
                                    achievements_rw.set_soundset(Some(soundset_name));
                                }
                            }

                            let none_name = if current_soundset.is_none() {
                                "None ✔"
                            } else {
                                "None"
                            };
                            if ui.button(none_name).clicked() {
                                let mut achievements_rw = self.achievements.blocking_write();
                                achievements_rw.set_soundset(None);
                            }

                            ui.separator();

                            if ui.button("Add Voicepack...").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_file() {
                                    if let Ok((new_name, new_sounds)) =
                                        import_rtst_vpk(&mut self.db, path)
                                    {
                                        let mut achievements_rw =
                                            self.achievements.blocking_write();
                                        achievements_rw.add_soundset(new_name, new_sounds);
                                    }
                                }
                            }
                        });
                    });
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_strip").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Version 0.4.2");
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        egui::SidePanel::left("picker_panel").show(ctx, |ui| {
            ui.heading("Sessions");
            ui.separator();
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if ui.button("characters").clicked() {
                    let mut char_list_rw = self.char_list.blocking_write();
                    char_list_rw.message = None;
                    self.in_character_ui = !self.in_character_ui;
                }
                let mut session_list_rw = self.session_list.blocking_write();

                //returns true when the user clicks on a session in the list
                if session_list_rw.ui(ctx, ui) {
                    self.in_character_ui = false;
                }
            });
        });

        if self.in_character_ui {
            //Mutable write access required because UI code handles adding / removing characters to the list.
            let mut char_list_rw = self.char_list.blocking_write();
            char_list_rw.ui(ctx, &self.db);
        } else {
            {
                //Updating the list of visible events / event filters requires write access, and
                //should only result in lengthy operations when the visible options, filters, or
                //currently selected session change.
                let mut session_list_rw = self.session_list.blocking_write();
                if let Some(session) = session_list_rw.selected_mut() {
                    let filter_opt = if self.filter_text.is_empty() {
                        None
                    } else {
                        Some(self.filter_text.to_lowercase())
                    };
                    session.update_filters(self.event_list_mode, filter_opt);
                }
            }
            {
                //Displaying the actual list however is repeated each refresh and has the potential
                //to be a lengthy operation each time.
                let mut session_list_rw = self.session_list.blocking_write();
                if let Some(session) = session_list_rw.selected_mut() {
                    session.ui(ctx);
                }
            }
        }
    }
}

impl TextureLookup for egui::Context {
    fn texture_by_name(&self, name: &str) -> Option<egui::TextureHandle> {
        let mut new_handle = None;
        let manager = self.tex_manager();
        let manager_cloned = manager.clone();
        {
            let mut manager_rw = manager.write(); //uses egui::RwLock
            for (id, meta) in manager_rw.allocated() {
                if meta.name == name {
                    new_handle = Some(TextureHandle::new(manager_cloned, *id));
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
