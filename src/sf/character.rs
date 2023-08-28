use super::{stats::Id, rand::Rand};

pub enum StatGainMethod {
    Real,
    Target,
    Min,
    Max,
}

pub enum Stats {
    Attack,
    Defense,
    Agility,
    Hp,
    Mp,
    Crit,
}

impl Stats {
    fn idx_to_stats(idx: usize) -> Self {
        match idx {
            0 => Stats::Attack,
            1 => Stats::Defense,
            2 => Stats::Agility,
            3 => Stats::Hp,
            4 => Stats::Mp,
            5 => Stats::Crit,

            _ => unreachable!(),
        }
    }

    pub fn stats_to_idx(&self) -> usize {
        match self {
            Stats::Attack  => 0,
            Stats::Defense => 1,
            Stats::Agility => 2,
            Stats::Hp      => 3,
            Stats::Mp      => 4,
            Stats::Crit    => 5,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Character {
    pub id: Id,
    pub level: u8,
    pub stats: [u8; 6], //attack, defense, agility, hp, mp, crit
    pub promoted: bool,
}

impl Character {
    pub fn increase_stats_on_level_up(&mut self, rng: &mut Rand, method: &StatGainMethod) {
        if self.level >= 20 {
            println!("already max level");
            return;
        }

        // if jogurt, don't increase level

        self.level += 1;

        for x in 0 .. 6 {
            let stat = Stats::idx_to_stats(x);

            let target = self.calculate_total_stat_growth(&stat);
            let gain = self.calculate_stat_gain(target, &stat, rng, method);

            self.stats[x] += gain;
        }
    }

    fn calculate_total_stat_growth(&self, stat: &Stats) -> u8 {
        self.get_base_stat(stat) + self.calculate_growth_target(stat)
    }

    fn get_base_stat(&self, stat: &Stats) -> u8 { //CalculatePromotedBaseStats
        if self.promoted {
            todo!()
        } else { // get unpromoted base stat
            self.id.get_base(stat)
        }
    }

    fn calculate_growth_target(&self, stat: &Stats) -> u8 {
        let growth_percent = self.calculate_growth_percent(stat);
        ((self.id.get_gain(stat) as u16 * growth_percent as u16) / 100) as u8
    }

    fn calculate_growth_percent(&self, stat: &Stats) -> u8 {
        if self.level == 20 {
            100
        } else {
            let mut curve_idx = 0;
            let curve_points = self.id.get_curve(stat).curve_points();

            for x in 0 .. curve_points.len() {
                if self.level <= curve_points[x + 1].0 {
                    curve_idx = x;
                    break;
                }
            }

            let inter = (self.level - curve_points[curve_idx].0) as u16;
            let inter2 = (curve_points[curve_idx + 1].0 - curve_points[curve_idx].0) as u16;
            let inter3 = (curve_points[curve_idx + 1].1 - curve_points[curve_idx].1) as u16;

            (((inter * inter3) / inter2) + curve_points[curve_idx].1 as u16) as u8
        }
    }

    fn calculate_stat_gain(&self, target: u8, stat: &Stats, rng: &mut Rand, method: &StatGainMethod) -> u8 {
        let stat_value = self.stats[stat.stats_to_idx()];

        if target == 0 {
            0
        } else if self.level <= 20 || stat_value >= 99 { //2481A
            let new_target = (target >> 2).min(5);

            let target2 = match method {
                StatGainMethod::Real   => target + rng.get(new_target) - rng.get(new_target),
                StatGainMethod::Target => target,
                StatGainMethod::Min    => target - new_target.saturating_sub(1),
                StatGainMethod::Max    => target + new_target.saturating_sub(1),
            };
            
            //todo: cap stat to 99 here

            target2.max(stat_value) - stat_value
        } else {
            todo!()
        }
    }
}
