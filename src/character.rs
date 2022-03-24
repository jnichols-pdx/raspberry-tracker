use crate::common::*;
use egui::Color32;

#[derive(Debug, Clone)]
pub struct Character {
    pub full_name: String,
    pub lower_name: String,
    pub server: World,
    pub outfit: Option<String>,
    pub outfit_full: Option<String>,
    pub character_id: String,
    pub auto_track: bool,
    pub faction: Faction,
    pub to_remove: bool,
    pub confirm_visible: bool,
    pub to_track: bool,
    pub changed_auto_track: bool,
}

impl Character {
    pub fn from_json(json: &serde_json::Value) -> Result<Character, String> {
        let new_char = &json["character_list"][0];
        println!("deets: {:?}", new_char);
        let faction_num = new_char["faction_id"]
            .to_string()
            .unquote()
            .parse::<i64>()
            .unwrap();
        let world_num = new_char["world_id"]
            .to_string()
            .unquote()
            .parse::<i64>()
            .unwrap();

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
            changed_auto_track: false,
        };

        if new_char["outfit"].is_object() {
            bob.outfit = Some(new_char["outfit"]["alias"].to_string().unquote());
            bob.outfit_full = Some(new_char["outfit"]["name"].to_string().unquote());
        }
        Ok(bob)
    }

    pub fn name_with_outfit(&self) -> String {
        if let Some(outfit_alias) = &self.outfit {
            if outfit_alias.is_empty() {
                if let Some(outfit_name) = &self.outfit_full {
                    format!("[{}] {}", outfit_name, self.full_name)
                } else {
                    self.full_name.to_owned()
                }
            } else {
                format!("[{}] {}", outfit_alias, self.full_name)
            }
        } else {
            self.full_name.to_owned()
        }
    }
}

impl View for Character {
    fn ui(&mut self, _ctx: &egui::Context) {}
    fn draw(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new(format!("characters{}", self.character_id))
            .min_col_width(10.0)
            .show(ui, |ui| {
                match ui.ctx().texture_by_name(&self.faction.to_string()) {
                    Some(image) => ui.image(image.id(), (28.0, 28.0)),
                    None => ui.label(self.faction.to_string()),
                };
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(self.name_with_outfit());
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
                        if !self.auto_track && ui.button("Start Session").clicked() {
                            self.to_track = true;
                        }
                    });
                });
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if self.confirm_visible {
                            ui.label(
                                egui::RichText::new("Actually remove this character?".to_owned())
                                    .color(Color32::from_rgb(200, 0, 0)),
                            );
                        } else {
                            ui.label(""); //hold space open
                        }
                    });
                    ui.horizontal(|ui| {
                        if !self.confirm_visible {
                            if ui.button("remove").clicked() {
                                self.confirm_visible = true;
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

#[derive(Clone)]
pub struct FullCharacter {
    pub full_name: String,
    pub lower_name: String,
    pub server: World,
    pub outfit: Option<String>,
    pub outfit_full: Option<String>,
    pub character_id: String,
    pub faction: Faction,
    pub br: u8,
    pub asp: u8,
}

impl FullCharacter {
    pub fn new(source: &Character, br: u8, asp: u8) -> Self {
        let mut new_char = FullCharacter {
            full_name: source.full_name.to_owned(),
            lower_name: source.lower_name.to_owned(),
            server: source.server,
            outfit: None,
            outfit_full: None,
            character_id: source.character_id.to_owned(),
            faction: source.faction,
            br,
            asp,
        };

        if let Some(outfit) = &source.outfit {
            new_char.outfit = Some(outfit.to_owned());
        }
        if let Some(outfit_full) = &source.outfit_full {
            new_char.outfit_full = Some(outfit_full.to_owned());
        }

        new_char
    }

    pub fn from_json(json: &serde_json::Value) -> Result<FullCharacter, String> {
        let bob = Character::from_json(json).unwrap();
        let biff = FullCharacter::new(
            &bob,
            json["character_list"][0]["battle_rank"]["value"]
                .to_string()
                .unquote()
                .parse::<u8>()
                .unwrap(),
            json["character_list"][0]["prestige_level"]
                .to_string()
                .unquote()
                .parse::<u8>()
                .unwrap(),
        );
        Ok(biff)
    }

    pub fn name_with_outfit(&self) -> String {
        if let Some(outfit_alias) = &self.outfit {
            if outfit_alias.is_empty() {
                if let Some(outfit_name) = &self.outfit_full {
                    format!("[{}] {}", outfit_name, self.full_name)
                } else {
                    self.full_name.to_owned()
                }
            } else {
                format!("[{}] {}", outfit_alias, self.full_name)
            }
        } else {
            self.full_name.to_owned()
        }
    }
}
