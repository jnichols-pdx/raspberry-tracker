use crate::character_list::*;
use crate::common::*;
use crate::db::*;
use crate::session::*;
use eframe::{egui, epi};
use egui::*;
use image::io::Reader as ImageReader;
use std::io::Cursor;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::tungstenite::protocol::Message;

pub struct TrackerApp {
    //pub from_main: mpsc::Receiver<Action>,
    pub in_character_ui: bool,
    pub char_list: Arc<RwLock<CharacterList>>,
    pub session_list: Arc<RwLock<Vec<Session>>>,
    pub db: DatabaseSync,
    pub lastx: f32,
    pub lasty: f32,
    pub size_changed: bool,
    pub ws_out: mpsc::Sender<Message>,
    pub frame_cb: Option<oneshot::Sender<epi::Frame>>,
    pub session_count: usize,
    pub images: Option<Vec<TextureHandle>>,
}

impl TrackerApp {
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
                ));
                println!("Readied Custom : {}", image_name);
            }
        };
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
        _glcx: &Rc<epi::glow::Context>,
    ) {
        if let Some(callback) = self.frame_cb.take() {
            let _blah = callback.send(frame.clone());
        }
        ctx.set_visuals(egui::Visuals::dark());

        self.images = Some(Vec::new());

        for (name, census_id) in master_images() {
            match self.db.exist_or_download_image_sync(&name, census_id) {
                true => {
                    if let Some(image_bytes) = self.db.get_image_sync(&name) {
                        if let Ok(image) = ImageReader::with_format(
                            Cursor::new(image_bytes),
                            image::ImageFormat::Png,
                        )
                        .decode()
                        {
                            let size = [image.width() as usize, image.height() as usize];
                            let image_buffer = image.to_rgba8();
                            let pixels = image_buffer.as_flat_samples();
                            if let Some(list) = self.images.as_mut() {
                                list.push(ctx.load_texture(
                                    name,
                                    ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
                                ));
                                //println!("Readied {}: {}", census_id, name);
                            }
                        };
                    };
                }
                false => println!("Unable to load image {}: {}", census_id, name),
            };
        }

        self.load_image_bytes("Robit", include_bytes!("../Images/NSO.png"), ctx);
        self.load_image_bytes("Headshot", include_bytes!("../Images/Headshot.png"), ctx);
        self.load_image_bytes("Pumpkin", include_bytes!("../Images/Pumpkin.png"), ctx);
        self.load_image_bytes(
            "ManaAVTurret",
            include_bytes!("../Images/ManaAVTurret.png"),
            ctx,
        );
        self.load_image_bytes("Flail", include_bytes!("../Images/Flail.png"), ctx);
        self.load_image_bytes("Glaive", include_bytes!("../Images/Glaive.png"), ctx);
        self.load_image_bytes(
            "BastionFleetCarrier",
            include_bytes!("../Images/BastionFleetCarrier.png"),
            ctx,
        );
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
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
        if self.size_changed && !newchange {
            self.size_changed = false;
            self.db
                .set_window_specs_sync(self.lastx as f64, self.lasty as f64);
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
                    self.in_character_ui = !self.in_character_ui;
                }
            });
        });

        if self.in_character_ui {
            //Mutable write access required because UI code handles adding / removing characters to the list.
            let mut char_list_rw = self.char_list.write().unwrap();
            char_list_rw.ui(ctx, &self.db);
        } else {
            let session_list_ro = self.session_list.read().unwrap();
            if let Some(session) = session_list_ro.last() {
                session.ui(ctx);
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
            let mut manager_rw = manager.write();
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
