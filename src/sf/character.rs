use std::num::NonZeroU8;

use super::{stats::Id, rand::Rand};

pub enum StatGainMethod {
    Real,
    Target,
    Min,
    Max,
}

#[derive(Debug, Clone, Copy)]
pub struct Character {
    pub id: Id,
    pub level: u8,
    pub stats: [u8; 6], //attack, defense, agility, hp, mp, crit
    pub promoted_level: Option<NonZeroU8>,
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
            let target = self.calculate_total_stat_growth(x, self.promoted_level.is_some());
            let gain = self.calculate_stat_gain(target, x, rng, method);

            self.stats[x] += gain;
        }
    }

    fn calculate_total_stat_growth(&self, stat: usize, promoted: bool) -> u8 {
        self.get_base_stat(stat) + self.calculate_growth_target(stat, self.level, promoted)
    }

    pub fn get_base_stat(&self, stat: usize) -> u8 { //CalculatePromotedBaseStats
        let base_stat = self.id.get_base(stat);

        if let Some(promoted_level) = self.promoted_level {
            // promoted base stat: 85% of (base stat + target growth of promotion level)
            let promoted_base_stat = base_stat + self.calculate_growth_target(stat, promoted_level.into(), false);
            ((promoted_base_stat as u16 * 85) / 100) as u8
        } else { // get unpromoted base stat
            base_stat
        }
    }

    fn calculate_growth_target(&self, stat: usize, level: u8, promoted: bool) -> u8 {
        let growth_percent = self.calculate_growth_percent(stat, level, promoted);
        ((self.id.get_gain(stat, promoted) as u16 * growth_percent as u16) / 100) as u8
    }

    fn calculate_growth_percent(&self, stat: usize, level: u8, promoted: bool) -> u8 {
        if level == 20 {
            100
        } else {
            let mut curve_idx = 0;
            let curve_points = self.id.get_curve(stat, promoted).curve_points();

            for x in 0 .. curve_points.len() {
                if level <= curve_points[x + 1].0 {
                    curve_idx = x;
                    break;
                }
            }

            let inter = (level - curve_points[curve_idx].0) as u16;
            let inter2 = (curve_points[curve_idx + 1].0 - curve_points[curve_idx].0) as u16;
            let inter3 = (curve_points[curve_idx + 1].1 - curve_points[curve_idx].1) as u16;

            (((inter * inter3) / inter2) + curve_points[curve_idx].1 as u16) as u8
        }
    }

    fn calculate_stat_gain(&self, target: u8, stat: usize, rng: &mut Rand, method: &StatGainMethod) -> u8 {
        let stat_value = self.stats[stat];

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

    pub fn promote(&mut self) {
        if self.level < 10 {
            println!("cannot promote yet");
            return;
        }

        self.promoted_level = Some(NonZeroU8::new(self.level).unwrap());

        for x in 0 .. 6 {
            self.stats[x] = self.get_base_stat(x);
        }

        self.level = 1;
    }
}
