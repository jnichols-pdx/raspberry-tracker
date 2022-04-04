use crate::common::*;
use crate::db::*;
use crate::events::*;
use crate::experience::*;
use crate::weapons::*;

pub struct AchievementEngine {
    db: DatabaseCore,
    killstreak: u32,
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
}

#[allow(dead_code, unused_variables)]
impl AchievementEngine {
    pub fn new(db: DatabaseCore) -> Self {
        AchievementEngine {
            db,
            killstreak: 0,
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
        }
    }
    pub fn reset(&mut self) {
        self.killstreak = 0;
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
    }

    pub fn tally_xp_tick(&mut self, kind: ExperienceType, amount: u32) -> Option<Vec<Event>> {
        None
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
        self.deathstreak += 1;
        self.last_death_time = timestamp;
        self.last_killer = attacker_id;
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

        //Mutual Kill, here the opponent was logged as dying before the player.
        let delta = self.last_death_time - self.last_kill_time;
        if delta == 0 || delta == 1 && self.last_killer.eq(&self.last_victim) {
            results.push(Event::achieved("Mutual", timestamp, datetime.to_owned()));
        }

        //Death streaks - N deaths without reviving or getting a kill.
        //Repeats on each death after 10 in a row.
        match self.deathstreak {
            6 => results.push(Event::achieved( "Death Streak", timestamp, datetime.to_owned())),
            7 => results.push(Event::achieved("Being Farmed", timestamp, datetime.to_owned())),
            _ => {}
        }
        if self.deathstreak > 10 {
            results.push(Event::achieved(&format!("Death Insult {}", self.deathstreak), timestamp, datetime.to_owned()));
        }

        //Bad revive streaks - dying immediately after being revived is often bad.
        let revive_delta = self.last_death_time - self.last_revived_time;

        if revive_delta < 4 {
            self.bad_revive_streak += 1;
            match self.bad_revive_streak {
                2 => results.push(Event::achieved("Poor Choices", timestamp, datetime.to_owned())),
                3 => results.push(Event::achieved("Malpractice", timestamp, datetime.to_owned())),
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
        self.last_kill_time = timestamp;
        self.last_victim = victim_id.clone();
        if headshot {
            self.headshots_consecutive += 1;
        } else {
            self.headshots_consecutive = 0;
        }
        self.bad_revive_streak = 0;
        self.team_kills = 0;

        //Mutual Kill, here the player was logged as dying before the opponent.
        let delta = self.last_kill_time - self.last_death_time;
        if delta == 0 || delta == 1 && self.last_killer.eq(&self.last_victim) {
            results.push(Event::achieved("Mutual", timestamp, datetime.to_owned()))
        }

        //Suicide bomber (kill self and 1+ enemy with an Explosive like C-4 or landmine)
        //In this case the player was considered to have died before the opponent.
        let weapon_category = self.db.get_weapon_category(weapon_id).await;
        if weapon_category == WeaponType::Explosive {
            self.last_explosive_kill_time = timestamp;
            let delta = timestamp - self.last_explosive_suicide_time;
            if delta == 0 || delta == 1 {
                results.push(Event::achieved("Suicide Bomber", timestamp, datetime.to_owned()));
            }
        }

        //C-4 Simultaneous Kills achievement
        if weapon_is_c4(weapon_id) {
            if timestamp == self.last_c4_kill_time {
                self.same_time_c4_kills += 1;
                if self.same_time_c4_kills == 4 {
                    results.push(Event::achieved("Terrorists Win", timestamp, datetime.to_owned()));
                }
            } else {
                self.last_c4_kill_time = timestamp;
                self.same_time_c4_kills = 1;
            }
        }

        //Kill streaks - N kills without dying.
        match self.killstreak {
            5   => results.push(Event::achieved("Killing Spree", timestamp, datetime.to_owned())),
            10  => results.push(Event::achieved("Dominating", timestamp, datetime.to_owned())),
            20  => results.push(Event::achieved("Unstoppable", timestamp, datetime.to_owned())),
            25  => results.push(Event::achieved("Wicked Sick", timestamp, datetime.to_owned())),
            30  => results.push(Event::achieved("God Like", timestamp, datetime.to_owned())),
            48  => results.push(Event::achieved("One Platoon", timestamp, datetime.to_owned())),
            100 => results.push(Event::achieved("One Man Empire", timestamp, datetime.to_owned())),
            250 => results.push(Event::achieved("One Man Mission", timestamp, datetime.to_owned())),
            500 => results.push(Event::achieved("One Man Farm", timestamp, datetime.to_owned())),
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
            results.push(Event::achieved("Humiliation", timestamp, datetime.to_owned()));
            if self.knife_kills_consecutive == 3 {
                results.push(Event::achieved("Knife Fight", timestamp, datetime.to_owned()));
                //consider resetting the consecutive count here - so every 3rd triggers this
                //achievement?
            }
        }

        //Consecutive Headshot kills
        match self.headshots_consecutive {
            3  => results.push(Event::achieved("Accuracy", timestamp, datetime.to_owned())),
            6  => results.push(Event::achieved("Impressive", timestamp, datetime.to_owned())),
            9  => results.push(Event::achieved("Sharp Shooter", timestamp, datetime.to_owned())),
            12 => results.push(Event::achieved("Marksman", timestamp, datetime.to_owned())),
            16 => results.push(Event::achieved("Head Hunter", timestamp, datetime.to_owned())),
            _ => {}
        }
        if self.headshots_consecutive >= 21 {
            results.push(Event::achieved("Lobotomist", timestamp, datetime.to_owned()));
        }

        //High Roller - kill 2x 'top BR' players in a row
        //Originally top BR was 100, recursion's achievemnt notes imply they check BR 120 instead.
        //However there are three 'top BRs': at BR 120/ASP0, BR100/ASP1, and BR100/ASP2. A player may
        //*choose* to stop leveling at the first two until they take an ASP level. We will consider
        //a player at any of these levels as valid for the High roller achievemnt.
        if br == 120 || (asp > 0 && br == 100) {
            self.high_roller += 1;
            if self.high_roller > 1 {
                results.push(Event::achieved("High Roller", timestamp, datetime.to_owned()));
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
            results.push(Event::achieved("Impress Myself", timestamp, datetime.to_owned()));
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
                4 => results.push(Event::achieved("Rocket Primary", timestamp, datetime.to_owned())),
                8 => results.push(Event::achieved("Master Juggler", timestamp, datetime.to_owned())),
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
                4  => results.push(Event::achieved("Pistol Whipped", timestamp, datetime.to_owned())),
                8  => results.push(Event::achieved("Run and Handgun", timestamp, datetime.to_owned())),
                12 => results.push(Event::achieved("Sidearm Slayer", timestamp, datetime.to_owned())),
                _ => {}
            }
        }

        //Commisioiner killstreak / 'Executions' (kill infil with the commie)
        if weapon_is_commissioner(weapon_id) {
            self.commissioner_kills += 1;
            if self.commissioner_kills == 3 {
                results.push(Event::achieved("Commissioner", timestamp, datetime.to_owned()));
            }
            if their_class.is_infil() {
                results.push(Event::achieved("Executioner", timestamp, datetime.to_owned()));
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
                    results.push(Event::achieved("Exploding Fist", timestamp, datetime.to_owned()));
                }
            }
            match self.max_suit_kills {
                15 => results.push(Event::achieved("Juggernaught", timestamp, datetime.to_owned())),
                30 => results.push(Event::achieved("Maximum Damage", timestamp, datetime.to_owned())),
                40 => {
                    match your_class {
                        Class::NCMax => results.push(Event::achieved("BOOM-Sticks", timestamp, datetime.to_owned())),
                        Class::TRMax => results.push(Event::achieved("DAKKA DAKKA", timestamp, datetime.to_owned())),
                        Class::VSMax => results.push(Event::achieved("Z.O.E. ZOE", timestamp, datetime.to_owned())),
                        //Why limit ourselves to what Recursion has?
                        Class::NSOMax => results.push(Event::achieved("METAL is superior", timestamp, datetime.to_owned())),
                        //Future max types?
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        //Killed a new player
        if br <= 1 && asp == 0 {
            results.push(Event::achieved("Welcome to Planetside", timestamp, datetime.to_owned()));
        }

        //Mines vs Infantry kills
        if weapon_is_proxy_mine(weapon_id) {
            self.proxy_mine_kills += 1;
            match self.proxy_mine_kills {
                2 => results.push(Event::achieved("Present For Ya", timestamp, datetime.to_owned())),
                4 => results.push(Event::achieved("Watch Your Step", timestamp, datetime.to_owned())),
                _ => {}
             }
        }

        //Vehicular achievements
        if let Some(vehicle) = maybe_vehicle {
            //MANA AI turret killstreak
            if vehicle == Vehicle::ManaAITurret {
                self.mana_ai_kills += 1;
                match self.mana_ai_kills {
                    6  => results.push(Event::achieved("Lawnmower", timestamp, datetime.to_owned())),
                    15 => results.push(Event::achieved("Harvester", timestamp, datetime.to_owned())),
                    _  => {}
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
            if vehicle == Vehicle::AIPhalanxTurret || vehicle == Vehicle::AIBuilderTower
            {
                self.phalanx_ai_kills += 1;
                if self.phalanx_ai_kills == 6 {
                    results.push(Event::achieved("Lawnmower", timestamp, datetime.to_owned()));
                }
            }

            //Steel Rain (drop pod smashed something)
            if vehicle == Vehicle::DropPod || vehicle == Vehicle::DropPodAlt {
                results.push(Event::achieved("Steel Rain", timestamp, datetime.to_owned()));
            }

            //Roadkills
            if weapon_id.eq("0") {
                self.roadkills += 1;
                if vehicle.is_flash() {
                    self.flash_roadkills += 1;
                    results.push(Event::achieved("Quad Damage", timestamp, datetime.to_owned()));
                } else {
                    results.push(Event::achieved("Roadkill", timestamp, datetime.to_owned()));
                }
                if self.roadkills == 4 {
                    results.push(Event::achieved("Road Rage", timestamp, datetime.to_owned()));
                }
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
            3  => results.push(Event::achieved("Team Killer", timestamp, datetime.to_owned())),
            6  => results.push(Event::achieved("Prevent Friendly Fire!", timestamp, datetime.to_owned())),
            9  => results.push(Event::achieved("Whose Side?", timestamp, datetime.to_owned())),
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

    pub fn tally_destroy_vehicle(&mut self) -> Option<Vec<Event>> {
        None
    }
}
