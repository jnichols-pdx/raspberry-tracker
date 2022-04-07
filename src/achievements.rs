use crate::common::*;
use crate::db::*;
use crate::events::*;
use crate::experience::*;
use crate::weapons::*;
use std::collections::HashMap;

const COMBO_LIMIT: i64 = 5;
const COMBO_MEND_LIMIT: i64 = 61;
const COMBO_RESUPPLY_LIMIT: i64 = 91;
const RAGE_LIMIT: i64 = 10;

pub struct AchievementEngine {
    login_time: i64,
    db: DatabaseCore,
    killstreak: u32,
    combo_kills: u32,
    last_victim: String,
    last_kill_time: i64,
    deathstreak: u32,
    last_killer: String,
    last_death_time: i64,
    headshots_consecutive: u32,
    last_explosive_suicide_time: i64,
    last_explosive_kill_time: i64,
    last_revived_time: i64,
    bad_revive_streak: i64,
    knife_kills_consecutive: u32,
    team_kills: u32,
    high_roller: u32,
    kdr_under_one: u32,
    kdr_over_one: u32,
    kdr_over_two: u32,
    kdr_over_three: u32,
    rocket_kills: u32,
    iced_kills: u32,
    fire_kills: u32,
    pistol_kills: u32,
    mana_ai_kills: u32,
    mana_av_kills: u32,
    phalanx_ai_kills: u32,
    commissioner_kills: u32,
    max_suit_kills: u32,
    max_melee_kills: u32,
    roadkills: u32,
    flash_roadkills: u32,
    proxy_mine_kills: u32,
    lancer_kills: u32,
    last_c4_kill_time: i64,
    same_time_c4_kills: u32,
    last_frag_time: i64,
    same_time_frag_kills: u32,
    non_vehicle_kills: u32,
    revive_count: u32,
    revive_no_kills_count: u32,
    explosives_destroyed: u32,
    assist_count: u32,
    savior_kills: u32,
    last_revive_time: i64,
    combo_revives: u32,
    last_heal_time: i64,
    last_repair_time: i64,
    last_resupply_time: i64,
    last_reshield_time: i64,
    combo_heal_xp: u32,
    combo_repair_xp: u32,
    combo_resupply_xp: u32,
    combo_reshield_xp: u32,
    last_vehicle_destroy_time: i64,
    last_vehicle_destroy_weapon: String,
    last_vehicle_destroy_kind: Vehicle,
    pizza_awarded_time: i64,
    last_fighter_pilot_id: String,
    ground_vehicle_kills: u32,
    opponents: HashMap<String, PlayerInteraction>,
    tank_mines_defused: u32,
    last_radar_kill_time: i64,
    last_radar_kill_id: String,
    radar_kills: u32,
    air_to_ground_kills: u32,
    last_air_to_air_time: i64,
    air_to_air_destructions: u32,
}

struct PlayerInteraction {
    latest_death_time: i64,
    player_kills: u32,
    deaths_to_player: u32,
}

impl PlayerInteraction {
    pub fn new() -> Self {
        Self {
            latest_death_time: 0,
            player_kills: 0,
            deaths_to_player: 0,
        }
    }
}

#[allow(dead_code, unused_variables)]
impl AchievementEngine {
    pub fn new(db: DatabaseCore) -> Self {
        AchievementEngine {
            login_time: 0,
            db,
            killstreak: 0,
            combo_kills: 0,
            last_victim: "".to_owned(),
            last_kill_time: 0,
            deathstreak: 0,
            last_killer: "".to_owned(),
            last_death_time: 0,
            headshots_consecutive: 0,
            last_explosive_suicide_time: 0,
            last_explosive_kill_time: 0,
            last_revived_time: 0,
            bad_revive_streak: 0,
            knife_kills_consecutive: 0,
            team_kills: 0,
            high_roller: 0,
            kdr_under_one: 0,
            kdr_over_one: 0,
            kdr_over_two: 0,
            kdr_over_three: 0,
            rocket_kills: 0,
            iced_kills: 0,
            fire_kills: 0,
            pistol_kills: 0,
            mana_ai_kills: 0,
            mana_av_kills: 0,
            phalanx_ai_kills: 0,
            commissioner_kills: 0,
            max_suit_kills: 0,
            max_melee_kills: 0,
            roadkills: 0,
            flash_roadkills: 0,
            proxy_mine_kills: 0,
            lancer_kills: 0,
            last_c4_kill_time: 0,
            same_time_c4_kills: 0,
            last_frag_time: 0,
            same_time_frag_kills: 0,
            non_vehicle_kills: 0,
            revive_count: 0,
            revive_no_kills_count: 0,
            explosives_destroyed: 0,
            assist_count: 0,
            savior_kills: 0,
            last_revive_time: 0,
            combo_revives: 0,
            last_heal_time: 0,
            last_repair_time: 0,
            last_resupply_time: 0,
            last_reshield_time: 0,
            combo_heal_xp: 0,
            combo_repair_xp: 0,
            combo_resupply_xp: 0,
            combo_reshield_xp: 0,
            last_vehicle_destroy_time: 0,
            last_vehicle_destroy_weapon: "".to_owned(),
            last_vehicle_destroy_kind: Vehicle::Unknown,
            pizza_awarded_time: 0,
            last_fighter_pilot_id: "".to_owned(),
            ground_vehicle_kills: 0,
            opponents: HashMap::new(),
            tank_mines_defused: 0,
            last_radar_kill_time: 0,
            last_radar_kill_id: "".to_owned(),
            radar_kills: 0,
            air_to_ground_kills: 0,
            last_air_to_air_time: 0,
            air_to_air_destructions: 0,
        }
    }
    pub fn reset(&mut self, start_time: i64) {
        self.login_time = start_time;
        self.killstreak = 0;
        self.combo_kills = 0;
        self.last_victim = "".to_owned();
        self.last_kill_time = 0;
        self.deathstreak = 0;
        self.last_killer = "".to_owned();
        self.last_death_time = 0;
        self.headshots_consecutive = 0;
        self.last_explosive_suicide_time = 0;
        self.last_explosive_kill_time = 0;
        self.last_revived_time = 0;
        self.bad_revive_streak = 0;
        self.knife_kills_consecutive = 0;
        self.team_kills = 0;
        self.high_roller = 0;
        self.kdr_under_one = 0;
        self.kdr_over_one = 0;
        self.kdr_over_two = 0;
        self.kdr_over_three = 0;
        self.rocket_kills = 0;
        self.iced_kills = 0;
        self.fire_kills = 0;
        self.pistol_kills = 0;
        self.mana_ai_kills = 0;
        self.mana_av_kills = 0;
        self.phalanx_ai_kills = 0;
        self.commissioner_kills = 0;
        self.max_suit_kills = 0;
        self.max_melee_kills = 0;
        self.roadkills = 0;
        self.flash_roadkills = 0;
        self.proxy_mine_kills = 0;
        self.lancer_kills = 0;
        self.last_c4_kill_time = 0;
        self.same_time_c4_kills = 0;
        self.last_frag_time = 0;
        self.same_time_frag_kills = 0;
        self.non_vehicle_kills = 0;
        self.revive_count = 0;
        self.revive_no_kills_count = 0;
        self.explosives_destroyed = 0;
        self.assist_count = 0;
        self.savior_kills = 0;
        self.last_revive_time = 0;
        self.combo_revives = 0;
        self.last_heal_time = 0;
        self.last_repair_time = 0;
        self.last_resupply_time = 0;
        self.last_reshield_time = 0;
        self.combo_heal_xp = 0;
        self.combo_repair_xp = 0;
        self.combo_resupply_xp = 0;
        self.combo_reshield_xp = 0;
        self.last_vehicle_destroy_time = 0;
        self.last_vehicle_destroy_weapon = "".to_owned();
        self.last_vehicle_destroy_kind = Vehicle::Unknown;
        self.pizza_awarded_time = 0;
        self.last_fighter_pilot_id = "".to_owned();
        self.ground_vehicle_kills = 0;
        self.opponents.clear();
        self.tank_mines_defused = 0;
        self.last_radar_kill_time = 0;
        self.last_radar_kill_id = "".to_owned();
        self.radar_kills = 0;
        self.air_to_ground_kills = 0;
        self.last_air_to_air_time = 0;
        self.air_to_air_destructions = 0;
    }

    pub fn tally_xp_tick(
        &mut self,
        kind: ExperienceType,
        amount: u32,
        other_id: String,
        timestamp: i64,
        datetime: &str,
    ) -> Option<Vec<Event>> {
        let mut results = Vec::new();
        match kind {
            ExperienceType::Revive => {
                self.revive_count += 1;
                match self.revive_count {
                    5 => results.push(Event::achieved(
                        "Cautious Practicioner",
                        timestamp,
                        datetime.to_owned(),
                    )),
                    15 => results.push(Event::achieved(
                        "Master Medic",
                        timestamp,
                        datetime.to_owned(),
                    )),
                    30 => results.push(Event::achieved(
                        "Shadow Healer",
                        timestamp,
                        datetime.to_owned(),
                    )),
                    _ => {}
                }
                self.revive_no_kills_count += 1;
                if self.revive_no_kills_count == 40 {
                    results.push(Event::achieved(
                        "Do No Harm",
                        timestamp,
                        datetime.to_owned(),
                    ));
                }

                if timestamp - self.last_revive_time < COMBO_LIMIT {
                    self.combo_revives += 1;
                    match self.combo_revives {
                        2 => results.push(Event::achieved(
                            "Sustaining Force",
                            timestamp,
                            datetime.to_owned(),
                        )),
                        4 => results.push(Event::achieved(
                            "Rapid Fire Revival",
                            timestamp,
                            datetime.to_owned(),
                        )),
                        6 => results.push(Event::achieved(
                            "Miracle Worker",
                            timestamp,
                            datetime.to_owned(),
                        )),
                        8 => results.push(Event::achieved(
                            "Zombie Summoner",
                            timestamp,
                            datetime.to_owned(),
                        )),
                        _ => {}
                    }
                } else {
                    self.combo_revives = 1;
                }

                self.last_revive_time = timestamp;
            }
            ExperienceType::Explosive_Destruction => {
                self.explosives_destroyed += 1;
                if self.explosives_destroyed % 3 == 0 {
                    results.push(Event::achieved(
                        "Mine Sweeper",
                        timestamp,
                        datetime.to_owned(),
                    ));
                }
            }
            ExperienceType::Kill_Player_Assist
            | ExperienceType::Kill_Player_Priority_Assist
            | ExperienceType::Kill_Player_High_Priority_Assist => {
                self.assist_count += 1;
                if self.assist_count % 8 == 0 {
                    results.push(Event::achieved("Side Kick", timestamp, datetime.to_owned()));
                }
            }
            //TODO: verify how these interact - do we ever see both for the same kill?
            ExperienceType::Savior_Kill_Non_MAX | ExperienceType::Savior_Kill_MAX => {
                self.savior_kills += 1;
                if self.savior_kills % 3 == 0 {
                    results.push(Event::achieved("Overwatch", timestamp, datetime.to_owned()));
                }
            }
            ExperienceType::Heal_Player | ExperienceType::Squad_Heal => {
                if timestamp - self.last_heal_time < COMBO_MEND_LIMIT {
                    let new_total = self.combo_heal_xp + amount;
                    if self.combo_heal_xp < 100 && new_total > 100 {
                        results.push(Event::achieved(
                            "Main Healer",
                            timestamp,
                            datetime.to_owned(),
                        ));
                    } else if self.combo_heal_xp < 250 && new_total > 250 {
                        results.push(Event::achieved(
                            "Heals Are Real",
                            timestamp,
                            datetime.to_owned(),
                        ));
                    } else if self.combo_heal_xp < 500 && new_total > 500 {
                        results.push(Event::achieved(
                            "Life Force",
                            timestamp,
                            datetime.to_owned(),
                        ));
                    }
                    self.combo_heal_xp = new_total;
                } else {
                    self.combo_heal_xp = amount;
                }
            }
            ExperienceType::Resupply_Player
            | ExperienceType::Squad_Resupply
            | ExperienceType::Vehicle_Resupply
            | ExperienceType::Squad_Vehicle_Resupply => {
                if timestamp - self.last_resupply_time < COMBO_RESUPPLY_LIMIT {
                    let new_total = self.combo_resupply_xp + amount;
                    if self.combo_resupply_xp < 500 && new_total > 500 {
                        results.push(Event::achieved(
                            "Supply The Demand",
                            timestamp,
                            datetime.to_owned(),
                        ));
                    } else if self.combo_resupply_xp < 1000 && new_total > 1000 {
                        results.push(Event::achieved(
                            "Arms Dealer",
                            timestamp,
                            datetime.to_owned(),
                        ));
                    } else if self.combo_resupply_xp < 2000 && new_total > 2000 {
                        results.push(Event::achieved(
                            "Merchant Of Death",
                            timestamp,
                            datetime.to_owned(),
                        ));
                    }
                    self.combo_resupply_xp = new_total;
                } else {
                    self.combo_resupply_xp = amount;
                }
            }
            ExperienceType::Shield_Repair | ExperienceType::Squad_Shield_Repair => {
                if timestamp - self.last_reshield_time < COMBO_MEND_LIMIT {
                    let new_total = self.combo_reshield_xp + amount;
                    if self.combo_reshield_xp < 250 && new_total > 250 {
                        results.push(Event::achieved("Bastion", timestamp, datetime.to_owned()));
                    } else if self.combo_reshield_xp < 500 && new_total > 500 {
                        results.push(Event::achieved("Energizer", timestamp, datetime.to_owned()));
                    } else if self.combo_reshield_xp < 1000 && new_total > 1000 {
                        results.push(Event::achieved(
                            "Mend And Defend",
                            timestamp,
                            datetime.to_owned(),
                        ));
                    }
                    self.combo_reshield_xp = new_total;
                } else {
                    self.combo_reshield_xp = amount;
                }
            }
            ExperienceType::Tank_Mine_Despawn_or_Defusal => {
                self.tank_mines_defused += 1;
                if self.tank_mines_defused % 3 == 0 {
                    results.push(Event::achieved(
                        "Counter Terrorists",
                        timestamp,
                        datetime.to_owned(),
                    ));
                }
            }
            ExperienceType::Motion_Detect => {
                if self.last_kill_time == timestamp
                    && self.last_victim.eq(&other_id)
                    && (self.last_radar_kill_time != timestamp
                        || !self.last_radar_kill_id.eq(&other_id))
                {
                    self.radar_kills += 1;
                    self.last_radar_kill_time = timestamp;
                    self.last_radar_kill_id = other_id;
                    if self.radar_kills % 10 == 0 {
                        results.push(Event::achieved(
                            "Interlinked",
                            timestamp,
                            datetime.to_owned(),
                        ));
                    }
                }
            }
            _ => {}
        }

        if kind.is_repair() {
            if timestamp - self.last_repair_time < COMBO_MEND_LIMIT {
                let new_total = self.combo_repair_xp + amount;
                if self.combo_repair_xp < 500 && new_total > 500 {
                    results.push(Event::achieved(
                        "Patchworker",
                        timestamp,
                        datetime.to_owned(),
                    ));
                } else if self.combo_repair_xp < 1000 && new_total > 1000 {
                    results.push(Event::achieved(
                        "Mechanized Mending",
                        timestamp,
                        datetime.to_owned(),
                    ));
                } else if self.combo_repair_xp < 2000 && new_total > 2000 {
                    results.push(Event::achieved(
                        "Nanitesmith",
                        timestamp,
                        datetime.to_owned(),
                    ));
                }
                self.combo_repair_xp = new_total;
            } else {
                self.combo_repair_xp = amount;
            }
        }

        if !results.is_empty() {
            Some(results)
        } else {
            None
        }
    }

    pub fn tally_death(
        &mut self,
        timestamp: i64,
        datetime: &str,
        attacker_id: String,
        vehicle: Option<Vehicle>,
        weapon_id: &str,
    ) -> Option<Vec<Event>> {
        let mut results = Vec::new();
        self.killstreak = 0;
        self.combo_kills = 0;
        self.deathstreak += 1;
        self.last_death_time = timestamp;
        self.last_killer = attacker_id.clone();
        self.headshots_consecutive = 0;
        self.knife_kills_consecutive = 0;
        self.team_kills = 0;
        self.high_roller = 0;
        self.kdr_under_one = 0;
        self.kdr_over_one = 0;
        self.kdr_over_two = 0;
        self.kdr_over_three = 0;
        self.rocket_kills = 0;
        self.iced_kills = 0;
        self.fire_kills = 0;
        self.pistol_kills = 0;
        self.mana_ai_kills = 0;
        self.mana_av_kills = 0;
        self.phalanx_ai_kills = 0;
        self.commissioner_kills = 0;
        self.max_suit_kills = 0;
        self.max_melee_kills = 0;
        self.proxy_mine_kills = 0;
        self.lancer_kills = 0;
        self.last_c4_kill_time = 0;
        self.same_time_c4_kills = 0;
        self.last_frag_time = 0;
        self.same_time_frag_kills = 0;
        self.non_vehicle_kills = 0;
        self.revive_count = 0;
        self.revive_no_kills_count = 0;
        self.explosives_destroyed = 0;
        self.assist_count = 0;
        self.savior_kills = 0;
        self.last_revive_time = 0;
        self.combo_revives = 0;
        self.last_heal_time = 0;
        self.last_repair_time = 0;
        self.last_resupply_time = 0;
        self.last_reshield_time = 0;
        self.combo_heal_xp = 0;
        self.combo_repair_xp = 0;
        self.combo_resupply_xp = 0;
        self.combo_reshield_xp = 0;
        self.last_vehicle_destroy_time = 0;
        self.last_vehicle_destroy_weapon = "".to_owned();
        self.last_vehicle_destroy_kind = Vehicle::Unknown;
        self.pizza_awarded_time = 0;
        self.last_fighter_pilot_id = "".to_owned();
        self.ground_vehicle_kills = 0;
        self.tank_mines_defused = 0;
        self.last_radar_kill_time = 0;
        self.last_radar_kill_id = "".to_owned();
        self.radar_kills = 0;
        self.air_to_ground_kills = 0;
        self.last_air_to_air_time = 0;
        self.air_to_air_destructions = 0;

        let opponent = self
            .opponents
            .entry(attacker_id)
            .or_insert_with(PlayerInteraction::new);
        opponent.deaths_to_player = 0;
        opponent.player_kills += 1;

        //Mutual Kill, here the opponent was logged as dying before the player.
        let delta = self.last_death_time - self.last_kill_time;
        if (delta == 0 || delta == 1) && self.last_killer.eq(&self.last_victim) {
            results.push(Event::achieved("Mutual", timestamp, datetime.to_owned()));
        }

        //Death streaks - N deaths without getting a kill.
        //Repeats on each death after 10 in a row.
        match self.deathstreak {
            6 => results.push(Event::achieved(
                "Death Streak",
                timestamp,
                datetime.to_owned(),
            )),
            7 => results.push(Event::achieved(
                "Being Farmed",
                timestamp,
                datetime.to_owned(),
            )),
            _ => {}
        }
        if self.deathstreak > 10 {
            results.push(Event::achieved(
                &format!("Death Insult {}", self.deathstreak),
                timestamp,
                datetime.to_owned(),
            ));
        }

        //Bad revive streaks - dying immediately after being revived is often bad.
        let revive_delta = self.last_death_time - self.last_revived_time;

        if revive_delta < COMBO_LIMIT {
            self.bad_revive_streak += 1;
            match self.bad_revive_streak {
                2 => results.push(Event::achieved(
                    "Poor Choices",
                    timestamp,
                    datetime.to_owned(),
                )),
                3 => results.push(Event::achieved(
                    "Malpractice",
                    timestamp,
                    datetime.to_owned(),
                )),
                _ => {}
            }
        } else {
            self.bad_revive_streak = 0;
        }

        if !results.is_empty() {
            Some(results)
        } else {
            None
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn tally_kill(
        &mut self,
        timestamp: i64,
        datetime: &str,
        victim_id: String,
        maybe_vehicle: Option<Vehicle>,
        weapon_id: &str,
        headshot: bool,
        their_kdr: f32,
        their_class: Class,
        your_class: Class,
        br: u8,
        asp: u8,
    ) -> Option<Vec<Event>> {
        println!("In tally kill");
        let mut results = Vec::new();
        self.killstreak += 1;
        self.deathstreak = 0;
        self.revive_no_kills_count = 0;
        self.last_victim = victim_id.clone();
        if headshot {
            self.headshots_consecutive += 1;
        } else {
            self.headshots_consecutive = 0;
        }
        self.bad_revive_streak = 0;
        self.team_kills = 0;

        let login_delta = timestamp - self.login_time;
        if login_delta <= 60 {
            self.login_time = 0; //Prevent triggering more than once per session
            results.push(Event::achieved(
                "First Blood",
                timestamp,
                datetime.to_owned(),
            ));
        } else if login_delta <= 90 {
            self.login_time = 0; //Prevent triggering more than once per session
            results.push(Event::achieved(
                "Insant Action",
                timestamp,
                datetime.to_owned(),
            ));
        }

        //Per player killstreak and Revenge achievements
        let opponent = self
            .opponents
            .entry(victim_id.clone())
            .or_insert_with(PlayerInteraction::new);
        if opponent.player_kills >= 3 {
            results.push(Event::achieved("Revenge", timestamp, datetime.to_owned()));
        }
        opponent.player_kills = 0;
        opponent.deaths_to_player += 1;
        opponent.latest_death_time = timestamp;
        match opponent.deaths_to_player {
            0 | 1 | 2 => {}
            3 => results.push(Event::achieved(
                "Repeat Customer",
                timestamp,
                datetime.to_owned(),
            )),
            4 => results.push(Event::achieved(
                "Think They'd Learn!",
                timestamp,
                datetime.to_owned(),
            )),
            5 => results.push(Event::achieved(
                "Domination",
                timestamp,
                datetime.to_owned(),
            )),
            6 | 7 | 8 | 9 => {
                results.push(Event::achieved("Recursion", timestamp, datetime.to_owned()))
            }
            //At least 10:
            _ => results.push(Event::achieved(
                "Recursive Recursion",
                timestamp,
                datetime.to_owned(),
            )),
        }
        println!(
            "Opponent {} now at {} deaths to player",
            victim_id, opponent.deaths_to_player
        );

        //Combo kills - where each previous kill was only moments before the next.
        if timestamp - self.last_kill_time < COMBO_LIMIT {
            self.combo_kills += 1;
            //May be overzealous. Might need a way to collapse these in the Event list?
            match self.combo_kills {
                0 | 1 => {}
                2 => results.push(Event::achieved(
                    "Double Kill",
                    timestamp,
                    datetime.to_owned(),
                )),
                3 => results.push(Event::achieved(
                    "Triple Kill",
                    timestamp,
                    datetime.to_owned(),
                )),
                4 => results.push(Event::achieved(
                    "Multi Kill",
                    timestamp,
                    datetime.to_owned(),
                )),
                5 => results.push(Event::achieved("Mega Kill", timestamp, datetime.to_owned())),
                6 => results.push(Event::achieved(
                    "Ultra Kill",
                    timestamp,
                    datetime.to_owned(),
                )),
                7 => results.push(Event::achieved(
                    "Monster Kill",
                    timestamp,
                    datetime.to_owned(),
                )),
                8 => results.push(Event::achieved(
                    "Luidcrous Kill",
                    timestamp,
                    datetime.to_owned(),
                )),
                //At least 9:
                _ => results.push(Event::achieved("Holy Shit", timestamp, datetime.to_owned())),
            }
        } else {
            self.combo_kills = 1;
        }

        self.last_kill_time = timestamp;

        //Mutual Kill, here the player was logged as dying before the opponent.
        let delta = self.last_kill_time - self.last_death_time;
        if (delta == 0 || delta == 1) && self.last_killer.eq(&self.last_victim) {
            results.push(Event::achieved("Mutual", timestamp, datetime.to_owned()))
        }

        //Suicide bomber (kill self and 1+ enemy with an Explosive like C-4 or landmine)
        //In this case the player was considered to have died before the opponent.
        let weapon_category = self.db.get_weapon_category(weapon_id).await;
        if weapon_category == WeaponType::Explosive {
            self.last_explosive_kill_time = timestamp;
            let delta = timestamp - self.last_explosive_suicide_time;
            if delta == 0 || delta == 1 {
                results.push(Event::achieved(
                    "Suicide Bomber",
                    timestamp,
                    datetime.to_owned(),
                ));
            }
        }

        //C-4 Simultaneous Kills achievement
        if weapon_is_c4(weapon_id) {
            if timestamp == self.last_c4_kill_time {
                self.same_time_c4_kills += 1;
                if self.same_time_c4_kills == 4 {
                    results.push(Event::achieved(
                        "Terrorists Win",
                        timestamp,
                        datetime.to_owned(),
                    ));
                }
            } else {
                self.last_c4_kill_time = timestamp;
                self.same_time_c4_kills = 1;
            }
        }

        //Frag grenade simultaneous kills achievement
        if weapon_is_frag_grenade(weapon_id) {
            if timestamp == self.last_frag_time {
                self.same_time_frag_kills += 1;
                match self.same_time_frag_kills {
                    3 => results.push(Event::achieved(
                        "Explosive Efficiency",
                        timestamp,
                        datetime.to_owned(),
                    )),
                    5 => results.push(Event::achieved("Fragasm", timestamp, datetime.to_owned())),
                    _ => {}
                }
            } else {
                self.last_frag_time = timestamp;
                self.same_time_frag_kills = 1;
            }
        }

        //Kill streaks - N kills without dying.
        match self.killstreak {
            5 => results.push(Event::achieved(
                "Killing Spree",
                timestamp,
                datetime.to_owned(),
            )),
            10 => results.push(Event::achieved(
                "Dominating",
                timestamp,
                datetime.to_owned(),
            )),
            20 => results.push(Event::achieved(
                "Unstoppable",
                timestamp,
                datetime.to_owned(),
            )),
            25 => results.push(Event::achieved(
                "Wicked Sick",
                timestamp,
                datetime.to_owned(),
            )),
            30 => results.push(Event::achieved("God Like", timestamp, datetime.to_owned())),
            48 => results.push(Event::achieved(
                "One Platoon",
                timestamp,
                datetime.to_owned(),
            )),
            100 => results.push(Event::achieved(
                "One Man Empire",
                timestamp,
                datetime.to_owned(),
            )),
            250 => results.push(Event::achieved(
                "One Man Mission",
                timestamp,
                datetime.to_owned(),
            )),
            500 => results.push(Event::achieved(
                "One Man Farm",
                timestamp,
                datetime.to_owned(),
            )),
            _ => {}
        }

        //Headshot -- will be used for audio callout later, but this isn't strictly necessary to
        //make a separate achievement 'event' in the event list. The kill line in the event list
        //already includes the headshot indicator.
        /*if headshot {

        }*/

        //Max down
        if their_class.is_max() {
            results.push(Event::achieved("Max Down", timestamp, datetime.to_owned()));
        }

        //Knife kills, individually and 3 in a row
        if weapon_category == WeaponType::Knife {
            self.knife_kills_consecutive += 1;
            results.push(Event::achieved(
                "Humiliation",
                timestamp,
                datetime.to_owned(),
            ));
            if self.knife_kills_consecutive == 3 {
                results.push(Event::achieved(
                    "Knife Fight",
                    timestamp,
                    datetime.to_owned(),
                ));
                //consider resetting the consecutive count here - so every 3rd triggers this
                //achievement?
            }
        }

        //Consecutive Headshot kills
        match self.headshots_consecutive {
            3 => results.push(Event::achieved("Accuracy", timestamp, datetime.to_owned())),
            6 => results.push(Event::achieved(
                "Impressive",
                timestamp,
                datetime.to_owned(),
            )),
            9 => results.push(Event::achieved(
                "Sharp Shooter",
                timestamp,
                datetime.to_owned(),
            )),
            12 => results.push(Event::achieved("Marksman", timestamp, datetime.to_owned())),
            16 => results.push(Event::achieved(
                "Head Hunter",
                timestamp,
                datetime.to_owned(),
            )),
            _ => {}
        }
        if self.headshots_consecutive >= 21 {
            results.push(Event::achieved(
                "Lobotomist",
                timestamp,
                datetime.to_owned(),
            ));
        }

        //High Roller - kill 2x 'top BR' players in a row
        //Originally top BR was 100, recursion's achievemnt notes imply they check BR 120 instead.
        //However there are three 'top BRs': at BR 120/ASP0, BR100/ASP1, and BR100/ASP2. A player may
        //*choose* to stop leveling at the first two until they take an ASP level. We will consider
        //a player at any of these levels as valid for the High roller achievemnt.
        if br == 120 || (asp > 0 && br == 100) {
            self.high_roller += 1;
            if self.high_roller > 1 {
                results.push(Event::achieved(
                    "High Roller",
                    timestamp,
                    datetime.to_owned(),
                ));
            }
        } else {
            self.high_roller = 0;
        }

        //KDR kill streaks
        if their_kdr < 1.0 {
            self.kdr_under_one += 1;
            self.kdr_over_one = 0;
            self.kdr_over_two = 0;
            self.kdr_over_three = 0;
            if self.kdr_under_one == 20 {
                results.push(Event::achieved("Easy Mode", timestamp, datetime.to_owned()));
            }
        }
        if their_kdr >= 3.0 {
            self.kdr_under_one = 0;
            self.kdr_over_one += 1;
            self.kdr_over_two += 1;
            self.kdr_over_three += 1;
        } else if their_kdr >= 2.0 {
            self.kdr_under_one = 0;
            self.kdr_over_one += 1;
            self.kdr_over_two += 1;
            self.kdr_over_three = 0;
        } else if their_kdr >= 1.0 {
            self.kdr_under_one = 0;
            self.kdr_over_one += 1;
            self.kdr_over_two = 0;
            self.kdr_over_three = 0;
        }
        if self.kdr_over_three >= 3 {
            results.push(Event::achieved(
                "Impress Myself",
                timestamp,
                datetime.to_owned(),
            ));
        }
        if self.kdr_over_two >= 5 {
            results.push(Event::achieved("Nightmare", timestamp, datetime.to_owned()));
        }
        if self.kdr_over_one >= 7 {
            results.push(Event::achieved("Hard Mode", timestamp, datetime.to_owned()));
        }

        //Rocket killstreaks
        if weapon_category == WeaponType::Rocket_Launcher {
            self.rocket_kills += 1;
            match self.rocket_kills {
                4 => results.push(Event::achieved(
                    "Rocket Primary",
                    timestamp,
                    datetime.to_owned(),
                )),
                8 => results.push(Event::achieved(
                    "Master Juggler",
                    timestamp,
                    datetime.to_owned(),
                )),
                _ => {}
            }
        }

        //Iced -- 3 kills with the NS Deep Freeze
        if weapon_id.eq("76358") {
            self.iced_kills += 1;
            if self.iced_kills > 0 && self.iced_kills % 3 == 0 {
                results.push(Event::achieved("Iced", timestamp, datetime.to_owned()));
            }
        }

        //Fire -- 3 kills with a flare pistol
        if weapon_is_flare_gun(weapon_id) {
            self.fire_kills += 1;
            if self.fire_kills > 0 && self.fire_kills % 3 == 0 {
                results.push(Event::achieved("Fire", timestamp, datetime.to_owned()));
            }
        }

        //Pistol killstreaks
        if weapon_category == WeaponType::Pistol {
            self.pistol_kills += 1;
            match self.pistol_kills {
                4 => results.push(Event::achieved(
                    "Pistol Whipped",
                    timestamp,
                    datetime.to_owned(),
                )),
                8 => results.push(Event::achieved(
                    "Run and Handgun",
                    timestamp,
                    datetime.to_owned(),
                )),
                12 => results.push(Event::achieved(
                    "Sidearm Slayer",
                    timestamp,
                    datetime.to_owned(),
                )),
                _ => {}
            }
        }

        //Commisioiner killstreak / 'Executions' (kill infil with the commie)
        if weapon_is_commissioner(weapon_id) {
            self.commissioner_kills += 1;
            if self.commissioner_kills == 3 {
                results.push(Event::achieved(
                    "Commissioner",
                    timestamp,
                    datetime.to_owned(),
                ));
            }
            if their_class.is_infil() {
                results.push(Event::achieved(
                    "Executioner",
                    timestamp,
                    datetime.to_owned(),
                ));
            }
        }

        //Lancer killstreak
        if weapon_is_lancer(weapon_id) {
            self.lancer_kills += 1;
            if self.lancer_kills == 7 {
                results.push(Event::achieved("Care Bear", timestamp, datetime.to_owned()));
            }
        }

        //Max killstreak / melee achievements
        if your_class.is_max() {
            self.max_suit_kills += 1;
            if self.db.get_weapon_category(weapon_id).await == WeaponType::Knife {
                self.max_melee_kills += 1;
                if self.max_melee_kills == 3 {
                    results.push(Event::achieved(
                        "Exploding Fist",
                        timestamp,
                        datetime.to_owned(),
                    ));
                }
            }
            match self.max_suit_kills {
                15 => results.push(Event::achieved(
                    "Juggernaught",
                    timestamp,
                    datetime.to_owned(),
                )),
                30 => results.push(Event::achieved(
                    "Maximum Damage",
                    timestamp,
                    datetime.to_owned(),
                )),
                40 => {
                    match your_class {
                        Class::NCMax => results.push(Event::achieved(
                            "BOOM-Sticks",
                            timestamp,
                            datetime.to_owned(),
                        )),
                        Class::TRMax => results.push(Event::achieved(
                            "DAKKA DAKKA",
                            timestamp,
                            datetime.to_owned(),
                        )),
                        Class::VSMax => results.push(Event::achieved(
                            "Z.O.E. ZOE",
                            timestamp,
                            datetime.to_owned(),
                        )),
                        //Why limit ourselves to what Recursion has?
                        Class::NSOMax => results.push(Event::achieved(
                            "METAL is superior",
                            timestamp,
                            datetime.to_owned(),
                        )),
                        //Future max types?
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        //Killed a new player
        if br <= 1 && asp == 0 {
            results.push(Event::achieved(
                "Welcome to Planetside",
                timestamp,
                datetime.to_owned(),
            ));
        }

        //Mines vs Infantry kills
        if weapon_is_proxy_mine(weapon_id) {
            self.proxy_mine_kills += 1;
            match self.proxy_mine_kills {
                2 => results.push(Event::achieved(
                    "Present For Ya",
                    timestamp,
                    datetime.to_owned(),
                )),
                4 => results.push(Event::achieved(
                    "Watch Your Step",
                    timestamp,
                    datetime.to_owned(),
                )),
                _ => {}
            }
        }

        //Tank mine kills - Only trigger on maxes / vehicle destruction resulting in occupant
        //death, however we only want to trigger once even if the vehicle held multiple players.
        //This still isn't perfect, as a vehicle being blown up with infantry standing in range to
        //also be blown up, while not in the vehicle, will be counted.
        if weapon_is_tank_mine(weapon_id) && self.pizza_awarded_time != timestamp {
            if self.last_vehicle_destroy_time == timestamp {
                if weapon_is_tank_mine(&self.last_vehicle_destroy_weapon) {
                    results.push(Event::achieved(
                        "Pizza Delivery",
                        timestamp,
                        datetime.to_owned(),
                    ));
                    self.pizza_awarded_time = timestamp;
                }
            } else if their_class.is_max() {
                results.push(Event::achieved(
                    "Pizza Delivery - MAX",
                    timestamp,
                    datetime.to_owned(),
                ));
                self.pizza_awarded_time = timestamp;
            }
        }

        //Decimation - Shoot down an ESF with the Decimator rocket launcher, killing the pilot.
        if weapon_is_decimator(weapon_id)
            && timestamp == self.last_vehicle_destroy_time
            && victim_id.eq(&self.last_fighter_pilot_id)
        {
            results.push(Event::achieved(
                "Decimation",
                timestamp,
                datetime.to_owned(),
            ));
        }

        //Vehicular achievements
        if let Some(vehicle) = maybe_vehicle {
            self.non_vehicle_kills = 0;

            //MANA AI turret killstreak
            if vehicle == Vehicle::ManaAITurret {
                self.mana_ai_kills += 1;
                match self.mana_ai_kills {
                    6 => results.push(Event::achieved("Lawnmower", timestamp, datetime.to_owned())),
                    15 => {
                        results.push(Event::achieved("Harvester", timestamp, datetime.to_owned()))
                    }
                    _ => {}
                }
            }

            //MANA AV turret killstreak
            if vehicle == Vehicle::ManaAVTurret {
                self.mana_av_kills += 1;
                if self.mana_av_kills == 12 {
                    results.push(Event::achieved("Precipice", timestamp, datetime.to_owned()));
                }
            }

            //Phalanx / Builder AI turret killstreak
            if vehicle == Vehicle::AIPhalanxTurret || vehicle == Vehicle::AIBuilderTower {
                self.phalanx_ai_kills += 1;
                if self.phalanx_ai_kills == 6 {
                    results.push(Event::achieved("Lawnmower", timestamp, datetime.to_owned()));
                }
            }

            //Steel Rain (drop pod smashed something)
            if vehicle == Vehicle::DropPod || vehicle == Vehicle::DropPodAlt {
                results.push(Event::achieved(
                    "Steel Rain",
                    timestamp,
                    datetime.to_owned(),
                ));
            }

            //Roadkills
            if weapon_id.eq("0") {
                self.roadkills += 1;
                if vehicle.is_flash() {
                    self.flash_roadkills += 1;
                    results.push(Event::achieved(
                        "Quad Damage",
                        timestamp,
                        datetime.to_owned(),
                    ));
                } else {
                    results.push(Event::achieved("Roadkill", timestamp, datetime.to_owned()));
                }
                if self.roadkills == 4 {
                    results.push(Event::achieved("Road Rage", timestamp, datetime.to_owned()));
                }
            }

            let weapon_cat = self.db.get_weapon_category(weapon_id).await;

            //Tank vs Aircraft
            if weapon_cat.is_tank_primary()
                && weapon_is_not_skyguard(weapon_id)
                && timestamp == self.last_vehicle_destroy_time
                && victim_id.eq(&self.last_fighter_pilot_id)
            {
                results.push(Event::achieved(
                    "Flyswatter",
                    timestamp,
                    datetime.to_owned(),
                ));
            }

            //Ground vehicle based killstreaks
            if vehicle.is_ground_vehicle() {
                self.ground_vehicle_kills += 1;
                match self.ground_vehicle_kills {
                    15 => results.push(Event::achieved(
                        "Armored Assault",
                        timestamp,
                        datetime.to_owned(),
                    )),
                    30 => results.push(Event::achieved(
                        "Blitzkrieg",
                        timestamp,
                        datetime.to_owned(),
                    )),
                    _ => {}
                }
            }

            //Air to Ground killstreaks
            if vehicle.is_aircraft()
                && (timestamp != self.last_vehicle_destroy_time
                    || !self.last_vehicle_destroy_kind.is_aircraft())
            {
                self.air_to_ground_kills += 1;
                match self.air_to_ground_kills {
                    15 => results.push(Event::achieved(
                        "Death From Above",
                        timestamp,
                        datetime.to_owned(),
                    )),
                    40 => results.push(Event::achieved(
                        "Bombardier",
                        timestamp,
                        datetime.to_owned(),
                    )),
                    60 => results.push(Event::achieved(
                        "Scourge Of the Skies",
                        timestamp,
                        datetime.to_owned(),
                    )),
                    _ => {}
                }
            }

            //Air to Air vehicle destroy streaks
            if vehicle.is_aircraft()
                && timestamp == self.last_vehicle_destroy_time
                && self.last_vehicle_destroy_kind.is_aircraft()
                && timestamp != self.last_air_to_air_time
            {
                self.air_to_air_destructions += 1;
                self.last_air_to_air_time = timestamp;
                match self.air_to_air_destructions {
                    3 => results.push(Event::achieved("Ace", timestamp, datetime.to_owned())),
                    8 => results.push(Event::achieved("Top Gun", timestamp, datetime.to_owned())),
                    20 => results.push(Event::achieved("Superman", timestamp, datetime.to_owned())),
                    _ => {}
                }
            }
        } else {
            self.non_vehicle_kills += 1;
            match self.non_vehicle_kills {
                40 => results.push(Event::achieved("Batman", timestamp, datetime.to_owned())),
                50 => results.push(Event::achieved("Cheater", timestamp, datetime.to_owned())),
                _ => {}
            }

            //Fatality achievement - Weapon id 0 and not a vehicle roadkill
            if weapon_id.eq("0") {
                results.push(Event::achieved("Fatality", timestamp, datetime.to_owned()));
            }
        }

        if !results.is_empty() {
            Some(results)
        } else {
            None
        }
    }

    pub fn tally_teamkill(&mut self, timestamp: i64, datetime: &str) -> Option<Vec<Event>> {
        let mut results = Vec::new();
        //Should teamkills RESET beneficial streak counts?
        self.team_kills += 1;

        //Consecutive teamkills - BAD Planetman!
        match self.team_kills {
            3 => results.push(Event::achieved(
                "Team Killer",
                timestamp,
                datetime.to_owned(),
            )),
            6 => results.push(Event::achieved(
                "Prevent Friendly Fire!",
                timestamp,
                datetime.to_owned(),
            )),
            9 => results.push(Event::achieved(
                "Whose Side?",
                timestamp,
                datetime.to_owned(),
            )),
            _ => {}
        }
        if !results.is_empty() {
            Some(results)
        } else {
            None
        }
    }

    pub fn tally_teamdeath(&mut self) -> Option<Vec<Event>> {
        //Should teamdeaths RESET beneficial streak counts?
        None
    }

    pub async fn tally_suicide(
        &mut self,
        weapon_id: &str,
        timestamp: i64,
        datetime: &str,
    ) -> Option<Vec<Event>> {
        self.killstreak = 0;
        self.combo_kills = 0;
        self.high_roller = 0;
        self.headshots_consecutive = 0;
        self.knife_kills_consecutive = 0;
        self.kdr_under_one = 0;
        self.kdr_over_one = 0;
        self.kdr_over_two = 0;
        self.kdr_over_three = 0;
        self.rocket_kills = 0;
        self.iced_kills = 0;
        self.fire_kills = 0;
        self.pistol_kills = 0;
        self.mana_ai_kills = 0;
        self.mana_av_kills = 0;
        self.phalanx_ai_kills = 0;
        self.commissioner_kills = 0;
        self.max_suit_kills = 0;
        self.max_melee_kills = 0;
        self.roadkills = 0;
        self.flash_roadkills = 0;
        self.proxy_mine_kills = 0;
        self.lancer_kills = 0;
        self.last_c4_kill_time = 0;
        self.same_time_c4_kills = 0;
        self.last_frag_time = 0;
        self.same_time_frag_kills = 0;
        self.non_vehicle_kills = 0;
        self.revive_count = 0;
        self.revive_no_kills_count = 0;
        self.explosives_destroyed = 0;
        self.assist_count = 0;
        self.savior_kills = 0;
        self.last_revive_time = 0;
        self.combo_revives = 0;
        self.last_heal_time = 0;
        self.last_repair_time = 0;
        self.last_resupply_time = 0;
        self.last_reshield_time = 0;
        self.combo_heal_xp = 0;
        self.combo_repair_xp = 0;
        self.combo_resupply_xp = 0;
        self.combo_reshield_xp = 0;
        self.last_vehicle_destroy_time = 0;
        self.last_vehicle_destroy_weapon = "".to_owned();
        self.last_vehicle_destroy_kind = Vehicle::Unknown;
        self.pizza_awarded_time = 0;
        self.last_fighter_pilot_id = "".to_owned();
        self.ground_vehicle_kills = 0;
        self.tank_mines_defused = 0;
        self.last_radar_kill_time = 0;
        self.last_radar_kill_id = "".to_owned();
        self.radar_kills = 0;
        self.air_to_ground_kills = 0;
        self.last_air_to_air_time = 0;
        self.air_to_air_destructions = 0;

        //Suicide bomber (kill self and 1+ enemy with an Explosive like C-4 or Mine)
        //In this case the opponent was considered to have died before the player
        match self.db.get_weapon_category(weapon_id).await {
            WeaponType::Explosive => {
                self.last_explosive_suicide_time = timestamp;
                let delta = timestamp - self.last_explosive_kill_time;
                if delta == 0 || delta == 1 {
                    let bomber = Event::achieved("Suicide Bomber", timestamp, datetime.to_owned());
                    Some(vec![bomber])
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn tally_revive(&mut self, timestamp: i64) {
        //Apparently revives do NOT break deathstreaks in recursion!
        //self.deathstreak = 0;
        self.last_revived_time = timestamp;
    }

    pub async fn tally_vehicle_destroy(
        &mut self,
        weapon_id: &str,
        our_vehicle: Option<Vehicle>,
        their_vehicle: Vehicle,
        driver_id: String,
        timestamp: i64,
        datetime: &str,
    ) -> Option<Vec<Event>> {
        let mut results = Vec::new();
        self.last_vehicle_destroy_time = timestamp;
        self.last_vehicle_destroy_weapon = weapon_id.to_owned();
        self.last_vehicle_destroy_kind = their_vehicle;

        if their_vehicle == Vehicle::DropPod || their_vehicle == Vehicle::DropPodAlt {
            results.push(Event::achieved(
                "Shoot The Needle",
                timestamp,
                datetime.to_owned(),
            ));
        }

        if their_vehicle.is_esf() {
            self.last_fighter_pilot_id = driver_id;
        } else {
            self.last_fighter_pilot_id = "".to_owned();
        }

        if !results.is_empty() {
            Some(results)
        } else {
            None
        }
    }

    pub fn tally_logout(
        &mut self,
        character_id: String,
        timestamp: i64,
        datetime: &str,
    ) -> Option<Event> {
        let mut rage_event = None;
        if let Some(opponent) = self.opponents.get(&character_id) {
            println!(
                "{} had {} deaths_to_player at logout",
                character_id, opponent.deaths_to_player
            );
            if opponent.deaths_to_player > 0 {
                let delta = timestamp - opponent.latest_death_time;
                println!("{} death to logout delta: {} seconds", character_id, delta);
                if delta <= RAGE_LIMIT {
                    let name: String;
                    match lookup_new_char_details(&character_id) {
                        Err(whut) => {
                            println!("{}", whut);
                            name = format!("missing: {}", character_id);
                        }
                        Ok(details) => {
                            let player_name = details["character_list"][0]["name"]["first"]
                                .to_string()
                                .unquote();
                            if details["character_list"][0]["outfit"].is_object() {
                                let outfit_alias = details["character_list"][0]["outfit"]["alias"]
                                    .to_string()
                                    .unquote();
                                let outfit_name = details["character_list"][0]["outfit"]["name"]
                                    .to_string()
                                    .unquote();
                                if outfit_alias.is_empty() {
                                    name = format!("[{}] {}", outfit_name, player_name);
                                } else {
                                    name = format!("[{}] {}", outfit_alias, player_name);
                                }
                            } else {
                                name = player_name;
                            }
                        }
                    }

                    let rage_message = if opponent.deaths_to_player >= 3 {
                        format!("Submission ({})", name)
                    } else {
                        format!("Rage Quit ({})", name)
                    };
                    println!("push here: {}", rage_message);
                    rage_event = Some(Event::achieved(
                        &rage_message,
                        timestamp,
                        datetime.to_owned(),
                    ));
                }
            }
            self.opponents.remove(&character_id);
        } else {
            println!("{} not in oppo list at logout", character_id)
        }
        rage_event
    }
}
