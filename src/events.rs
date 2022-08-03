use crate::common::*;
use egui::Color32;
use egui_extras::{Size, TableBuilder};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Event {
    pub kind: EventType,
    pub faction: Faction,
    pub br: u8,
    pub asp: u8,
    pub class: Class,
    pub name: String,
    pub weapon: String,
    pub weapon_id: String,
    pub weapon_kind: WeaponType,
    pub headshot: bool,
    pub kdr: f32,
    pub timestamp: i64,
    pub vehicle: Option<Vehicle>,
    pub datetime: String,
}

impl Event {
    pub fn achieved(text: &str, timestamp: i64, datetime: String) -> Event {
        Event {
            kind: EventType::Achievement,
            faction: Faction::Unknown,
            br: 0,
            asp: 0,
            class: Class::Unknown,
            name: text.to_owned(),
            weapon: "".to_owned(),
            weapon_id: "0".to_owned(),
            weapon_kind: WeaponType::Unknown,
            headshot: false,
            kdr: 0.0,
            timestamp,
            vehicle: None,
            datetime,
        }
    }

    pub fn ui(&self, row: &mut egui_extras::TableRow) {
        let img_size = (15.0, 15.0);
        let bg_color;
        let text_color;
        let mut minimal = false;
        match self.kind {
            EventType::Death => {
                bg_color = Color32::from_rgb(80, 0, 0);
                text_color = Color32::from_rgb(255, 255, 255);
            }
            EventType::TeamDeath => {
                bg_color = Color32::from_rgb(80, 80, 0);
                text_color = Color32::from_rgb(255, 255, 255);
            }
            EventType::Kill => {
                bg_color = Color32::from_rgb(0, 80, 0);
                text_color = Color32::from_rgb(255, 255, 255);
            }
            EventType::Suicide => {
                bg_color = Color32::from_rgb(0, 0, 0);
                text_color = Color32::from_rgb(255, 255, 255);
            }
            EventType::TeamKill => {
                bg_color = Color32::from_rgb(65, 80, 0);
                text_color = Color32::from_rgb(255, 255, 255);
            }
            EventType::LoseVehicle => {
                bg_color = Color32::from_rgb(80, 0, 0);
                text_color = Color32::from_rgb(200, 200, 200);
            }
            EventType::LoseVehicleFF => {
                bg_color = Color32::from_rgb(80, 80, 0);
                text_color = Color32::from_rgb(200, 200, 200);
            }
            EventType::DestroyVehicle => {
                bg_color = Color32::from_rgb(0, 80, 0);
                text_color = Color32::from_rgb(200, 200, 200);
            }
            EventType::DestroyVehicleFF => {
                bg_color = Color32::from_rgb(0, 0, 80);
                text_color = Color32::from_rgb(200, 200, 200);
            }
            EventType::ExperienceTick => {
                bg_color = Color32::from_rgb(0, 80, 80);
                text_color = Color32::from_rgb(255, 255, 255);
                minimal = true;
            }
            EventType::Achievement => {
                bg_color = Color32::from_rgb(80, 0, 80);
                text_color = Color32::from_rgb(255, 255, 255);
                minimal = true;
            }
            EventType::Revived => {
                bg_color = Color32::from_rgb(200, 200, 200);
                text_color = Color32::from_rgb(0, 0, 0);
            }
            _ => {
                bg_color = Color32::from_rgb(80, 80, 80);
                text_color = Color32::from_rgb(255, 255, 255);
            }
        };

        row.set_bg_color(bg_color);
        if minimal {
            row.col(|_ui| {});
            row.col(|_ui| {});
            row.col(|_ui| {});
            row.col(|_ui| {});
        } else {
            row.col(|ui| {
                //faction
                ui.vertical(|ui| {
                    match ui.ctx().texture_by_name(&self.faction.to_string()) {
                        Some(image) => {
                            ui.horizontal(|ui| {
                                ui.add_space(2.0);
                                ui.image(image.id(), img_size);
                            });
                        }
                        None => {
                            ui.add_space(3.5);
                            ui.label(
                                egui::RichText::new(self.faction.to_string())
                                    .small()
                                    .color(text_color),
                            );
                        }
                    };
                });
            });
            row.col(|ui| {
                //BR
                ui.vertical(|ui| {
                    ui.add_space(3.5);
                    if self.asp > 0 {
                        ui.label(
                            egui::RichText::new(format!("{}~{}", self.br, self.asp))
                                .small()
                                .color(text_color),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new(format!("{}", self.br))
                                .small()
                                .color(text_color),
                        );
                    }
                });
            });
            row.col(|ui| {
                //Class
                ui.horizontal(|ui| {
                    ui.add_space(3.0);
                    ui.vertical(|ui| {
                        ui.add_space(1.0);
                        match ui.ctx().texture_by_name(&self.class.to_string()) {
                            Some(image) => ui.image(image.id(), img_size),
                            None => ui.label(""), // ui.label(egui::RichText::new(self.class.to_string()).small()),
                        };
                    });
                });
            });
            row.col(|ui| {
                //Vehicle
                ui.horizontal(|ui| {
                    ui.add_space(2.0);
                    ui.vertical(|ui| {
                        //Override for orbital strike direct kills (can't track when players die from falling
                        //damage after being thrown airborn by orbital :( )
                        if self.weapon == "Orbital Strike Uplink" {
                            if let Some(image) = ui.ctx().texture_by_name("Orbital") {
                                ui.add_space(1.0);
                                ui.image(image.id(), img_size);
                            };
                        } else if let Some(vehicle) = self.vehicle {
                            match ui.ctx().texture_by_name(&vehicle.to_string()) {
                                Some(image) => {
                                    ui.add_space(1.0);
                                    ui.image(image.id(), img_size).on_hover_ui(|ui| {
                                        ui.horizontal(|ui| {
                                            //Screenspace is in Points, which aren't 1 to 1 with pixels
                                            //in my desktop environment the Sunderer icon, which is
                                            //18x32 pixels content in a 32x32 png, renders at that
                                            //actual screen resolution when the image size is 21x22
                                            //points. This MAY NOT HOLD across other user's desktops.
                                            //
                                            //In theory we could divide the image dimensions by
                                            //Context.pixels_per_point() to get 'correct' on screen
                                            //dimensions, however in testing this stretched icons
                                            //horizontally by 1 pixel.
                                            ui.image(image.id(), (21.0, 22.0));
                                            ui.label(vehicle.to_string());
                                        });
                                    });
                                }
                                None => {
                                    ui.add_space(1.5);
                                    ui.label(egui::RichText::new(vehicle.to_string()).small())
                                        .on_hover_text(vehicle.to_string());
                                }
                            };
                        }
                    });
                });
            });
        }
        row.col(|ui| {
            //Player Name
            ui.vertical(|ui| {
                ui.add_space(3.5);
                ui.label(egui::RichText::new(&self.name).small().color(text_color))
                    .on_hover_text(&self.name);
            });
        });
        row.col(|ui| {
            //Weapon
            ui.vertical(|ui| {
                ui.add_space(3.5);
                ui.label(egui::RichText::new(&self.weapon).small().color(text_color))
                    .on_hover_text(&self.weapon);
            });
        });
        if minimal {
            row.col(|_ui| {});
            row.col(|_ui| {});
        } else {
            row.col(|ui| {
                //Headshot
                ui.horizontal(|ui| {
                    ui.add_space(2.0);
                    ui.vertical(|ui| {
                        ui.add_space(1.5);
                        if self.headshot {
                            match ui.ctx().texture_by_name("Headshot") {
                                Some(image) => {
                                    ui.image(image.id(), img_size);
                                }
                                None => {
                                    ui.label(egui::RichText::new("HS!").small().color(text_color));
                                }
                            };
                        }
                    });
                });
            });
            row.col(|ui| {
                //KD ratio
                ui.vertical(|ui| {
                    ui.add_space(3.5);
                    ui.label(
                        egui::RichText::new(format!("{:.2}", self.kdr))
                            .small()
                            .color(text_color),
                    );
                });
            });
        }
        row.col(|ui| {
            //Timestamp
            ui.vertical(|ui| {
                ui.add_space(3.5);
                ui.label(
                    egui::RichText::new(&self.datetime)
                        .small()
                        .color(text_color),
                );
            });
        });
    }
}

#[derive(Clone)]
pub struct EventList {
    events: Vec<Event>,
    visible_events: VecDeque<usize>,
    last_view_mode: EventViewMode,
    last_filter: Option<String>,
}

impl EventList {
    pub fn new() -> Self {
        EventList {
            events: Vec::new(),
            visible_events: VecDeque::new(),
            last_view_mode: EventViewMode::default(),
            last_filter: None,
        }
    }

    pub fn push(&mut self, event: Event) {
        let shown = match event.kind {
            EventType::Death
            | EventType::Kill
            | EventType::TeamKill
            | EventType::TeamDeath
            | EventType::Suicide => self.last_view_mode.kills_deaths,
            EventType::DestroyVehicle
            | EventType::LoseVehicle
            | EventType::DestroyVehicleFF
            | EventType::LoseVehicleFF => self.last_view_mode.vehicles,
            EventType::ExperienceTick => self.last_view_mode.experience,
            EventType::Achievement => self.last_view_mode.achievements,
            EventType::Revived => self.last_view_mode.revives,
            EventType::Unknown => true,
        };
        let not_filtered = if let Some(ref filter_text) = self.last_filter {
            event.weapon.to_lowercase().contains(filter_text)
                || event.name.to_lowercase().contains(filter_text)
                || event.class.to_string().to_lowercase().contains(filter_text)
                || event
                    .vehicle
                    .unwrap_or(Vehicle::NoVehicle)
                    .to_string()
                    .to_lowercase()
                    .contains(filter_text)
        } else {
            true
        };
        self.events.push(event);
        if shown && not_filtered {
            self.visible_events.push_front(self.events.len() - 1);
        }
    }

    pub fn len(&self) -> u32 {
        self.events.len() as u32
    }

    pub fn last_event_time(&self) -> Option<i64> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.last().unwrap().timestamp)
        }
    }

    pub fn update_filters(&mut self, event_mode: EventViewMode, filter: Option<String>) {
        if event_mode != self.last_view_mode || filter != self.last_filter {
            self.last_view_mode = event_mode;
            self.last_filter = filter;
            self.compute_visible();
        }
    }

    fn compute_visible(&mut self) {
        self.visible_events.clear();
        for (index, event) in self.events.iter().enumerate() {
            let shown = match event.kind {
                EventType::Death
                | EventType::Kill
                | EventType::TeamKill
                | EventType::TeamDeath
                | EventType::Suicide => self.last_view_mode.kills_deaths,
                EventType::DestroyVehicle
                | EventType::LoseVehicle
                | EventType::DestroyVehicleFF
                | EventType::LoseVehicleFF => self.last_view_mode.vehicles,
                EventType::ExperienceTick => self.last_view_mode.experience,
                EventType::Achievement => self.last_view_mode.achievements,
                EventType::Revived => self.last_view_mode.revives,
                EventType::Unknown => true,
            };
            let not_filtered = if let Some(ref filter_text) = self.last_filter {
                event.weapon.to_lowercase().contains(filter_text)
                    || event.name.to_lowercase().contains(filter_text)
                    || event.class.to_string().to_lowercase().contains(filter_text)
                    || event
                        .vehicle
                        .unwrap_or(Vehicle::NoVehicle)
                        .to_string()
                        .to_lowercase()
                        .contains(filter_text)
            } else {
                true
            };

            if shown && not_filtered {
                self.visible_events.push_front(index);
            }
        }
    }

    pub fn ui(&self, ctx: &egui::Context) {
        egui::SidePanel::right("events_panel")
            .min_width(387.0)
            .show(ctx, |ui| {
                TableBuilder::new(ui)
                    .column(Size::exact(20.0)) //Faction
                    .column(Size::exact(30.0)) //BR
                    .column(Size::exact(22.0)) //Class
                    .column(Size::exact(20.0)) //Vehicle
                    .column(Size::remainder()) //playername //formerly minimum 100
                    .column(Size::remainder()) //weapon //formerly minimum 80
                    .column(Size::exact(20.0)) //headshot
                    .column(Size::exact(25.0)) //KD
                    .column(Size::exact(70.0)) //Timestamp
                    .header(12.0, |mut header| {
                        header.col(|_ui| { //No header for first column
                        });
                        header.col(|ui| {
                            ui.label(egui::RichText::new("BR").small());
                        });
                        header.col(|ui| {
                            ui.label(egui::RichText::new("Class").small());
                        });
                        header.col(|ui| {
                            ui.label(egui::RichText::new("Veh.").small());
                        });
                        header.col(|ui| {
                            ui.label(egui::RichText::new("Player").small());
                        });
                        header.col(|ui| {
                            ui.label(egui::RichText::new("Method").small());
                        });
                        header.col(|ui| {
                            ui.label(egui::RichText::new("HS").small());
                        });
                        header.col(|ui| {
                            ui.label(egui::RichText::new("KD").small());
                        });
                        header.col(|ui| {
                            ui.label(egui::RichText::new("Time").small());
                        });
                    })
                    .body(|body| {
                        body.rows(17.0, self.visible_events.len(), |row_index, mut row| {
                            self.events[self.visible_events[row_index]].ui(&mut row);
                        });
                    });

                //Claim the available space left over from the table - prevents the right side panel
                //from shrinking down toward minimum table width with each redraw.
                let rect = ui.available_size();
                ui.allocate_space(rect);
            });
    }
}

impl Default for EventList {
    fn default() -> Self {
        Self::new()
    }
}
