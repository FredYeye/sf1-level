use std::collections::BTreeMap;

use self::{character::{Character, StatGainMethod}, rand::Rand, stats::Id};

pub mod rand;
pub mod character;
pub mod stats;
mod curve;

#[derive(Debug)]
pub struct Sf {
    character: Character, //todo: should probably not be a part of this struct
    rng: Rand,
}

impl Sf {
    pub fn init(id: Id, seed: u16) -> Self {
        Self { character: id.character_from_id(), rng: Rand { state: seed } }
    }

    pub fn log_stats(&mut self, method: &StatGainMethod) {
        let mut stats = [[0; 6]; 20];

        //todo: make this if-else a function and use elsewhere
        if self.character.promoted_level.is_some() {
            self.character.level = 1;
            for x in 0..6 {
                stats[0][x] = self.character.get_base_stat(x);
            }
        } else {
            self.character.level = 0;
        }

        for x in self.character.level .. 20 {
            self.character.increase_stats_on_level_up(&mut self.rng, method);
            stats[x as usize] = self.character.stats;
        }

        println!("{:?}", self.character.id);
        let stat_names = ["Attack:", "Defense:", "Agility:", "Hp:", "Mp:", "Crit:"];

        for (idx, &stat) in stat_names.iter().enumerate() {
            let asd: Vec<u8> = stats.iter().map(|x| x[idx]).collect();
            println!("{:8} {:2?}", stat, asd);
        }
    }

    pub fn starting_level_min_max(&mut self) {
        let mut stats_min = [0xFF; 6];
        let mut stats_max = [0; 6];

        for x in 0 .. 0x10000 {
            self.rng.state = x as u16;
            self.character = self.character.id.character_from_id();
            self.character.level = 0;

            for _ in 0 .. self.character.id.base_stats().starting_level {
                self.character.increase_stats_on_level_up(&mut self.rng, &character::StatGainMethod::Real);
            }

            for x in 0 .. 6 {
                stats_min[x] = stats_min[x].min(self.character.stats[x]);
                stats_max[x] = stats_max[x].max(self.character.stats[x]);
            }
        }

        println!("{:?}", self.character.id);
        let stat_names = ["Attack:", "Defense:", "Agility:", "Hp:", "Mp:", "Crit:"];

        for (idx, &stat) in stat_names.iter().enumerate() {
            println!("{:8} {:2}-{:2}", stat, stats_min[idx], stats_max[idx]);
        }
    }

    pub fn all_levels_min_max(&mut self) {
        let mut stats_min = [[0xFF; 6]; 20];
        let mut stats_max = [[0   ; 6]; 20];

        self.character = self.character.id.character_from_id();
        self.character.level = 0;

        for x in 0 .. 20 {
            self.character.increase_stats_on_level_up(&mut self.rng, &character::StatGainMethod::Min);

            for y in 0 .. 6 {
                stats_min[x][y] = stats_min[x][y].min(self.character.stats[y]);
            }
        }

        self.character = self.character.id.character_from_id();
        self.character.level = 0;

        for x in 0 .. 20 {
            self.character.increase_stats_on_level_up(&mut self.rng, &character::StatGainMethod::Max);

            for y in 0 .. 6 {
                stats_max[x][y] = stats_max[x][y].max(self.character.stats[y]);
            }
        }

        println!("{:?}", self.character.id);
        let stat_names = ["Attack:", "Defense:", "Agility:", "Hp:", "Mp:", "Crit:"];

        for y in 0..6 {
            print!("{:8} ", stat_names[y]);

            for x in 0..20 {
                print!("{:2}-{:2}, ", stats_min[x][y], stats_max[x][y]);
            }
            println!();
        }
    }

    pub fn promote_test(&mut self, level: u8) {
        self.character.level = 0;

        for x in 0 .. level {
            self.character.increase_stats_on_level_up(&mut self.rng, &character::StatGainMethod::Target);
        }

        self.character.promote();

        self.log_stats(&StatGainMethod::Target);
    }

    pub fn speed_test(&mut self) {
        let mut colle: BTreeMap<u8, u16> = BTreeMap::new();

        let agi = 6;
        let variance = ((agi as u16 * 51 + 128) / 256) as u8;

        let min = agi - variance - 1;
        let max = agi + variance + 1;

        println!("{}-{} ({})\n", min, max, variance);

        let count = 65535;

        for _ in 0 .. count {
            let mut result = agi - variance;
            result += self.rng.get(variance + 1) + self.rng.get(variance + 1);
            result = (result as i8 + (self.rng.get(3) as i8 - 1)) as u8;

            colle.entry(result).and_modify(|counter| *counter += 1).or_insert(1);
        }

        for (k, v) in colle {
            let percent = v as f32 / (count as f32 / 100.0);
            println!("{}: {:.2}", k, percent);
        }
    }    
} 
