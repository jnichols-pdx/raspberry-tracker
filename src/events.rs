
use crate::common::*;
use egui::Color32;
use egui_extras::{TableBuilder, Size};

pub struct Event {
    pub kind: EventType,
    pub faction: Faction,
    pub br : u8,
    pub asp: u8,
    pub class: Class,
    pub name: String,
    pub weapon: String,
    pub headshot: bool,
    pub kdr: f32,
    pub timestamp: i64,
    pub vehicle: Option<Vehicle>,
    pub datetime: String,

}

impl Event {
    pub fn ui(&self, body: &mut egui_extras::TableBody) {
        let img_size = (14.0,14.0);
        let bg_color;
        let text_color;
        match self.kind {
            EventType::Death => {
                bg_color = Color32::from_rgb(80,0,0);
                text_color = Color32::from_rgb(255,255,255);
            },
            EventType::TeamDeath => {
                bg_color = Color32::from_rgb(80,80,0);
                text_color = Color32::from_rgb(255,255,255);
            },
            EventType::Kill => {
                bg_color = Color32::from_rgb(0,80,0);
                text_color = Color32::from_rgb(255,255,255);
            },
            EventType::Suicide => {
                bg_color = Color32::from_rgb(0,0,80);
                text_color = Color32::from_rgb(255,255,255);
            },
            EventType::TeamKill => {
                bg_color = Color32::from_rgb(65,80,0);
                text_color = Color32::from_rgb(255,255,255);
            },
            EventType::LoseVehicle => {
                bg_color = Color32::from_rgb(80,0,0);
                text_color = Color32::from_rgb(200,200,200);
            },
            EventType::LoseVehicleFF => {
                bg_color = Color32::from_rgb(80,80,0);
                text_color = Color32::from_rgb(200,200,200);
            },
            EventType::DestroyVehicle => {
                bg_color = Color32::from_rgb(0,80,0);
                text_color = Color32::from_rgb(200,200,200);
            },
            EventType::DestroyVehicleFF=> {
                bg_color = Color32::from_rgb(0,0,80);
                text_color = Color32::from_rgb(200,200,200);
            },
            _ => {
                bg_color = Color32::from_rgb(80,80,80);
                text_color = Color32::from_rgb(255,255,255);
            },
        };

        body.row(17.0, Some(bg_color), |mut row| {
            row.col_clip(|ui| { //faction
                match ui.ctx().texture_by_name(&self.faction.to_string()) {
                    Some(image) => {ui.image(image.id(), img_size);},
                    None => {
                            ui.vertical(|ui| {
                                ui.add_space(1.5);
                                ui.label(egui::RichText::new(self.faction.to_string()).small().color(text_color));
                            });
                        },
                };
            });
            row.col_clip(|ui| { //BR
                ui.vertical(|ui| {
                    ui.add_space(1.5);
                    if self.asp > 0 {
                        ui.label(egui::RichText::new(format!("{}~{}", self.br, self.asp)).small().color(text_color));
                    } else {
                        ui.label(egui::RichText::new(format!("{}", self.br)).small().color(text_color));
                    }
                });
            });
            row.col_clip(|ui| { //Class
                match ui.ctx().texture_by_name(&self.class.to_string()) {
                    Some(image) => ui.image(image.id(), img_size),
                    None =>ui.label(""),// ui.label(egui::RichText::new(self.class.to_string()).small()),
                };
            });
            row.col_clip(|ui| { //Vehicle
                //Override for orbital strike direct kills (can't track when players die from falling
                //damage after being thrown airborn by orbital :( )
                if self.weapon == "Orbital Strike Uplink" {
                    if let Some(image) = ui.ctx().texture_by_name(&"Orbital") {
                        ui.image(image.id(), img_size);
                    };
                } else if let Some(vehicle) = self.vehicle {
                    match ui.ctx().texture_by_name(&vehicle.to_string()) {
                        Some(image) => {ui.image(image.id(), img_size);},
                        None => {
                            ui.vertical(|ui| {
                                ui.add_space(1.5);
                                ui.label(egui::RichText::new(vehicle.to_string()).small());
                            });
                        },
                    };
                }
            });
            row.col_clip(|ui| { //Player Name
                ui.vertical(|ui| {
                    ui.add_space(1.5);
                    ui.label(egui::RichText::new(&self.name).small().color(text_color));
                });
            });
            row.col_clip(|ui| { //Weapon
                ui.vertical(|ui| {
                    ui.add_space(1.5);
                    ui.label(egui::RichText::new(&self.weapon).small().color(text_color));
                });
            });
            row.col(|ui| { //Headshot
                if self.headshot {
                    match ui.ctx().texture_by_name("Headshot") {
                        Some(image) => {ui.image(image.id(), img_size);},
                        None => {
                            ui.vertical(|ui| {
                                ui.add_space(1.5);
                                ui.label(egui::RichText::new("HS!").small().color(text_color));
                            });
                        },
                    };
                }
            });
            row.col_clip(|ui| { //KD ratio
                ui.vertical(|ui| {
                    ui.add_space(1.5);
                    ui.label(egui::RichText::new(format!("{:.2}",self.kdr)).small().color(text_color));
                });
            });
            row.col_clip(|ui| { //Timestamp
                ui.vertical(|ui| {
                    ui.add_space(1.5);
                    ui.label(egui::RichText::new(&self.datetime).small().color(text_color));
                });
            });
        });

    }
}
pub struct EventList {
    events: Vec<Event>,
}

impl EventList {
    pub fn new() -> Self {
        EventList {
            events: Vec::new(),
        }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn ui(&self, ctx: &egui::Context) {
        egui::SidePanel::right("events_panel").min_width(387.0).show(ctx, |ui| {

            TableBuilder::new(ui)
                .striped(true)
                .column(Size::Absolute(20.0)) //Faction
                .column(Size::Absolute(30.0)) //BR
                .column(Size::Absolute(22.0)) //Class
                .column(Size::Absolute(20.0)) //Vehicle
                .column(Size::RemainderMinimum(100.0)) //playername
                .column(Size::RemainderMinimum(80.0)) //weapon
                .column(Size::Absolute(20.0)) //headshot
                .column(Size::Absolute(25.0)) //KD
                .column(Size::Absolute(70.0)) //Timestamp
                .header(12.0, |mut header| {
                    header.col(|ui| {
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
                    header.col_clip(|ui| {
                        ui.label(egui::RichText::new("Player").small());
                    });
                    header.col_clip(|ui| {
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
                .body(|mut body| {
                    for event in self.events.iter().rev() {
                        event.ui(&mut body);
                    }
                });

        });
    }
}
