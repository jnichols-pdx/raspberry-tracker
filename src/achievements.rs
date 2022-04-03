use crate::common::*;
use crate::db::*;
use crate::events::*;
use crate::experience::*;

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
        vehicle: Option<Vehicle>,
        weapon_id: &str,
        headshot: bool,
        their_kdr: f32,
        their_class: Class,
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
            if self.kdr_under_one == 20 {
                results.push(Event::achieved("Easy Mode", timestamp, datetime.to_owned()));
            }
        }
        if their_kdr >= 1.0 {
            self.kdr_over_one += 1;
        }
        if their_kdr >= 2.0 {
            self.kdr_over_two += 1;
        }
        if their_kdr >= 3.0 {
            self.kdr_over_three += 1;
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
        if weapon_id.eq("76359") {
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
