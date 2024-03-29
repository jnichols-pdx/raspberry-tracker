use crate::common::WeaponType;
use crate::db::DatabaseCore;
use egui::*;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Weapon {
    pub name: String,
    pub category: WeaponType,
}

#[derive(Copy, Clone, Debug)]
pub struct WeaponInitial {
    pub fired: u64,
    pub hits: u64,
    pub kills: u64,
    pub headshots: u64,
}

impl WeaponInitial {
    pub fn new() -> Self {
        WeaponInitial {
            fired: 0,
            hits: 0,
            kills: 0,
            headshots: 0,
        }
    }
}

impl Default for WeaponInitial {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct WeaponStats {
    weapon_id: String,
    name: String,
    session_kills: u32,
    session_headshots: u32,
    initial: WeaponInitial,
    latest_hits: u64,
    latest_fired: u64,
    hit_accumulator: u64,
    fired_accumulator: u64,
    dirty: bool,
    kind: WeaponType,
}

impl WeaponStats {
    pub fn new(name: &str, id: &str, initial: WeaponInitial, kind: WeaponType) -> Self {
        WeaponStats {
            weapon_id: id.to_owned(),
            name: name.to_owned(),
            session_kills: 0,
            session_headshots: 0,
            initial,
            latest_hits: initial.hits,
            latest_fired: initial.fired,
            hit_accumulator: 0,
            fired_accumulator: 0,
            dirty: true,
            kind,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_historical(
        name: &str,
        id: &str,
        initial: WeaponInitial,
        kills: u32,
        headshots: u32,
        hits: u64,
        fired: u64,
        kind: WeaponType,
    ) -> Self {
        WeaponStats {
            weapon_id: id.to_owned(),
            name: name.to_owned(),
            session_kills: kills,
            session_headshots: headshots,
            initial,
            latest_hits: hits,
            latest_fired: fired,
            hit_accumulator: hits,
            fired_accumulator: fired,
            dirty: false,
            kind,
        }
    }

    pub fn matches_id(&self, other_id: &str) -> bool {
        self.weapon_id == *other_id
    }

    pub fn add_kill(&mut self, is_headshot: bool) {
        self.session_kills += 1;
        if is_headshot {
            self.session_headshots += 1;
        }
        self.dirty = true;
    }

    pub fn shots_fired(&self) -> u64 {
        self.latest_fired - self.initial.fired
    }

    pub fn shots_hit(&self) -> u64 {
        self.latest_hits - self.initial.hits
    }

    pub fn accumulate_latest_hits(&mut self, new_lifetime_hits: u64) {
        self.hit_accumulator += new_lifetime_hits;
        println!("w {} hits now: {}", self.name, self.hit_accumulator);
    }

    pub fn accumulate_latest_fired(&mut self, new_lifetime_fired: u64) {
        self.fired_accumulator += new_lifetime_fired;
        println!("w {} fired now: {}", self.name, self.fired_accumulator);
    }

    pub fn update_from_accumulators(&mut self) {
        if self.fired_accumulator > self.latest_fired {
            self.dirty = true;
            self.latest_fired = self.fired_accumulator;
        }
        if self.hit_accumulator > self.latest_hits {
            self.dirty = true;
            self.latest_hits = self.hit_accumulator;
        }
        self.hit_accumulator = 0;
        self.fired_accumulator = 0;
    }

    fn session_hsr(&self) -> f32 {
        if self.session_kills > 0 {
            (self.session_headshots as f32 / self.session_kills as f32) * 100.0
        } else {
            f32::NAN
        }
    }

    fn initial_hsr(&self) -> f32 {
        if self.initial.kills > 0 {
            (self.initial.headshots as f32 / self.initial.kills as f32) * 100.0
        } else {
            f32::NAN
        }
    }

    fn total_hsr(&self) -> f32 {
        let total_kills = self.session_kills as u64 + self.initial.kills;
        let total_headshots = self.session_headshots as u64 + self.initial.headshots;
        if total_kills > 0 {
            (total_headshots as f32 / total_kills as f32) * 100.0
        } else {
            f32::NAN
        }
    }

    fn session_accuracy(&self) -> f32 {
        let fired = self.shots_fired();
        if fired > 0 {
            (self.shots_hit() as f32 / fired as f32) * 100.0
        } else {
            f32::NAN
        }
    }

    fn initial_accuracy(&self) -> f32 {
        if self.initial.fired > 0 {
            (self.initial.hits as f32 / self.initial.fired as f32) * 100.0
        } else {
            f32::NAN
        }
    }

    fn total_accuracy(&self) -> f32 {
        if self.latest_fired > 0 {
            (self.latest_hits as f32 / self.latest_fired as f32) * 100.0
        } else {
            f32::NAN
        }
    }

    pub fn ui(&self, body: &mut egui_extras::TableBody) {
        let bg_color = Color32::from_rgb(60, 60, 60);
        let text_color = Color32::from_rgb(255, 255, 255);
        let red_color = Color32::from_rgb(255, 0, 0);
        let green_color = Color32::from_rgb(0, 255, 0);

        body.row(25.0, |mut row| {
            row.set_bg_color(bg_color);
            row.col(|ui| {
                //name
                ui.vertical(|ui| {
                    ui.add_space(5.0);
                    if self.kind.is_vehicle_mount() {
                        ui.label(
                            egui::RichText::new(format!("{} ({})", &self.name, &self.kind))
                                .small()
                                .color(text_color),
                        );
                    } else {
                        ui.label(egui::RichText::new(&self.name).small().color(text_color));
                    }
                });
            });
            row.col(|ui| {
                //kills
                ui.vertical(|ui| {
                    ui.add_space(5.0);
                    ui.label(
                        egui::RichText::new(format!("{}", &self.session_kills))
                            .small()
                            .color(text_color),
                    );
                });
            });
            row.col(|ui| {
                //HS%
                ui.vertical(|ui| {
                    let mut stat_color = text_color;
                    let total = self.total_hsr();
                    let delta = total - self.initial_hsr();
                    if delta < 0.0 {
                        stat_color = red_color;
                    } else if delta > 0.0 {
                        stat_color = green_color;
                    }
                    ui.label(
                        egui::RichText::new(format!("{:.3}%", self.session_hsr()))
                            .small()
                            .color(stat_color),
                    );
                    ui.label(
                        egui::RichText::new(format!("{total:.3}% {delta:+.3}%"))
                            .small()
                            .color(stat_color),
                    );
                });
            });

            row.col(|ui| {
                //ACC
                let mut stat_color = text_color;
                ui.vertical(|ui| {
                    let total = self.total_accuracy();
                    let delta = total - self.initial_accuracy();
                    if delta < 0.0 {
                        stat_color = red_color;
                    } else if delta > 0.0 {
                        stat_color = green_color;
                    }
                    ui.label(
                        egui::RichText::new(format!("{:.3}%", self.session_accuracy()))
                            .small()
                            .color(stat_color),
                    );
                    ui.label(
                        egui::RichText::new(format!("{total:.3}% {delta:+.3}%"))
                            .small()
                            .color(stat_color),
                    );
                });
            });

            row.col(|ui| {
                //HS count
                ui.vertical(|ui| {
                    ui.add_space(5.0);
                    ui.label(
                        egui::RichText::new(format!("{}", &self.session_headshots))
                            .small()
                            .color(text_color),
                    );
                });
            });
            row.col(|ui| {
                //Fired
                ui.vertical(|ui| {
                    ui.add_space(5.0);
                    ui.label(
                        egui::RichText::new(format!("{}", self.shots_fired()))
                            .small()
                            .color(text_color),
                    );
                });
            });
            row.col(|ui| {
                //Hits
                ui.vertical(|ui| {
                    ui.add_space(5.0);
                    ui.label(
                        egui::RichText::new(format!("{}", self.shots_hit()))
                            .small()
                            .color(text_color),
                    );
                });
            });
        });
    }

    pub async fn save_to_db(&self, ordering: u32, session: i64, db: &DatabaseCore) {
        match sqlx::query("INSERT INTO weaponstats VALUES (?,?,?,?,?,?,?,?,?,?,?,?);")
            .bind(session)
            .bind(ordering as i64)
            .bind(&self.weapon_id)
            .bind(&self.name)
            .bind(self.session_kills as i64)
            .bind(self.session_headshots as i64)
            .bind(self.latest_hits as i64)
            .bind(self.latest_fired as i64)
            .bind(self.initial.fired as i64)
            .bind(self.initial.hits as i64)
            .bind(self.initial.kills as i64)
            .bind(self.initial.headshots as i64)
            .execute(&db.conn)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                println!("Error saving new weaponstats in DB:");
                println!("{err:?}");
                std::process::exit(-30);
            }
        }
    }

    pub async fn update_db_entry(&mut self, session: i64, db: &DatabaseCore) {
        if self.dirty {
            match sqlx::query("UPDATE weaponstats SET kills = ?, headshots = ?, hits = ?, fired = ? WHERE session IS ? AND weapon_id IS ?;")
                .bind(self.session_kills as i64)
                .bind(self.session_headshots as i64)
                .bind(self.latest_hits as i64)
                .bind(self.latest_fired as i64)

                .bind(session)
                .bind(&self.weapon_id)
                .execute(&db.conn)
                .await
            {
                Ok(_) => {}
                Err(err) => {
                    println!("Error saving new weaponstats in DB:");
                    println!("{err:?}");
                    std::process::exit(-30);
                }
            }
        }
        self.dirty = false;
    }
}

#[derive(Clone)]
pub struct WeaponSet {
    weapons: BTreeMap<String, WeaponStats>,
    ordering: Vec<String>,
}

impl WeaponSet {
    pub fn new() -> Self {
        WeaponSet {
            weapons: BTreeMap::new(),
            ordering: Vec::new(),
        }
    }

    pub fn push(&mut self, new_stat: WeaponStats) {
        //Doesn't replace if already present
        if !self.weapons.contains_key(&new_stat.weapon_id) {
            self.ordering.push(new_stat.weapon_id.to_owned());
            self.weapons.insert(new_stat.weapon_id.to_owned(), new_stat);
        }
    }

    pub fn iter(&self) -> WeaponSetIter {
        WeaponSetIter {
            front_index: 0,
            back_index: self.ordering.len() as isize,
            set: self,
        }
    }

    pub fn accumulate_latest_hits(&mut self, target_id: &str, hit_count: u64) {
        if let Some(weapon) = self.weapons.get_mut(target_id) {
            weapon.accumulate_latest_hits(hit_count);
        }
    }

    pub fn accumulate_latest_fired(&mut self, target_id: &str, fire_count: u64) {
        if let Some(weapon) = self.weapons.get_mut(target_id) {
            weapon.accumulate_latest_fired(fire_count);
        }
    }

    pub fn update_from_accumulators(&mut self) {
        for (_, weapon) in self.weapons.iter_mut() {
            weapon.update_from_accumulators();
        }
    }

    pub fn contains(&self, query_id: &str) -> bool {
        self.weapons.contains_key(query_id)
    }

    pub fn get_by_index(&self, target: usize) -> Option<&WeaponStats> {
        if target > self.ordering.len() {
            None
        } else {
            self.weapons.get(&self.ordering[target])
        }
    }

    pub fn add_kill(&mut self, target_id: &str, is_headshot: bool) {
        if let Some(weapon) = self.weapons.get_mut(target_id) {
            weapon.add_kill(is_headshot);
        }
    }

    pub fn aggregate_accuracy(&self) -> f32 {
        let mut hits_total: u64 = 0;
        let mut fired_total: u64 = 0;

        for weapon in self.iter() {
            hits_total += weapon.shots_hit();
            fired_total += weapon.shots_fired();
        }

        (hits_total as f64 / fired_total as f64) as f32 * 100.0
    }

    pub fn len(&self) -> u32 {
        self.ordering.len() as u32
    }

    pub async fn update_db_entries(&mut self, db: &DatabaseCore, session: i64) {
        for (_, weapon) in self.weapons.iter_mut() {
            weapon.update_db_entry(session, db).await;
        }
    }
}

impl Default for WeaponSet {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a mut WeaponSet {
    type Item = &'a WeaponStats;
    type IntoIter = WeaponSetIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct WeaponSetIter<'a> {
    front_index: isize,
    back_index: isize,
    set: &'a WeaponSet,
}

impl<'a> Iterator for WeaponSetIter<'a> {
    type Item = &'a WeaponStats;
    fn next(&mut self) -> Option<&'a WeaponStats> {
        if self.front_index >= self.back_index {
            None
        } else {
            let target = self.front_index;
            self.front_index += 1;
            self.set.get_by_index(target.try_into().unwrap())
        }
    }
}

impl<'a> DoubleEndedIterator for WeaponSetIter<'a> {
    fn next_back(&mut self) -> Option<&'a WeaponStats> {
        if self.front_index >= self.back_index || self.back_index <= 0 {
            None
        } else {
            self.back_index -= 1;
            self.set.get_by_index(self.back_index.try_into().unwrap())
        }
    }
}

pub fn weapon_is_flare_gun(weapon_id: &str) -> bool {
    matches!(
        weapon_id,
        "74590"
            | "75516"
            | "75517"
            | "75518"
            | "75519"
            | "75520"
            | "75521"
            | "75522"
            | "76359"
            | "801971"
            | "803007"
            | "803008"
            | "803009"
            | "803010"
            | "803011"
            | "803012"
    )
}

pub fn weapon_is_commissioner(weapon_id: &str) -> bool {
    matches!(
        weapon_id,
        "2001" | "75063" | "75170" | "75172" | "803824" | "803825" | "803826"
    )
}

//The Explosive weapon category also includes tank mines and C-4, we have to identify anti-infantry
//proximity mines by weapon_id instead of relying on category alone.
pub fn weapon_is_proxy_mine(weapon_id: &str) -> bool {
    matches!(
        weapon_id,
        "429" | "1044" | "1045" | "6005243" | "6005422" | "6005963"
    )
}

pub fn weapon_is_lancer(weapon_id: &str) -> bool {
    matches!(weapon_id, "268" | "35002")
}

pub fn weapon_is_c4(weapon_id: &str) -> bool {
    matches!(weapon_id, "432" | "800623" | "6009782")
}

pub fn weapon_is_frag_grenade(weapon_id: &str) -> bool {
    matches!(
        weapon_id,
        "880" | "881" | "882" | "1095" | "44505" | "44605" | "44705" | "6005182" | "6005328"
    )
}

pub fn weapon_is_tank_mine(weapon_id: &str) -> bool {
    matches!(weapon_id, "650" | "6005961" | "6005962")
}

pub fn weapon_is_decimator(weapon_id: &str) -> bool {
    matches!(
        weapon_id,
        "50560" | "802299" | "802300" | "6003611" | "6004407"
    )
}

pub fn weapon_is_not_skyguard(weapon_id: &str) -> bool {
    !matches!(weapon_id, "3106" | "3107" | "3108" | "6005303")
}
