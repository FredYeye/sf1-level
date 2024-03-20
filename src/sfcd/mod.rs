use self::{character::Character, rand::Rand};

mod rand;
mod character;

pub struct Sfcd {
    rng: Rand,
}

impl Sfcd {
    pub fn init(seed: u16) -> Self {
        Self { rng: Rand { state: seed } }
    }

    pub fn calculate_stat_gain(&mut self, level: u8) -> u8 {
        let current_stat = 8;

        let stat = Character::Nick.get_stats(1, false);

        if level < 20 {
            let gain_segments = [ // gain rates, 1 rate per 5 levels
                [0, 0, 0, 0],
                [5, 5, 2, 2], // EEFF
                [3, 3, 2, 2], // DDFF
                [4, 4, 3, 3], // CCDD
                [6, 6, 4, 4], // BBCC
                [2, 4, 6, 6], // FCBB
                [7, 6, 3, 1], // ABDG
                [2, 6, 4, 1], // FBCG
            ];

            let curve_segment = gain_segments[stat[0] as usize][(level / 5) as usize];

            // 4.4 fixed point
            //base gain, upper bound
            let gain_rate = [
                // gains, sorted by low to high:
                // 0 1 2 5 3 4 6 7

                [0x00, 0x00], // 0      ± 0
                [0x03, 0x0A], // 0.1875 ± 0.5625 | 1: 15 / 100
                [0x07, 0x06], // 0.4375 ± 0.3125 | 1: 15 / 36
                [0x0E, 0x0C], // 0.875  ± 0.7333 | 1: 126 / 144, 2: 3 / 144
                [0x17, 0x06], // 1.4375 ± 0.3125 | 1: 21 / 36, 2: 15 / 36
                [0x0A, 0x06], // 0.625  ± 0.3125 | 1: 30 / 36
                [0x25, 0x04], // 2.3125 ± 0.1875 | 2: 15/16, 3: 1/16
                [0x2B, 0x08], // 2.6875 ± 0.4375 | 2: 10/64, 3: 54/64
            ];

            // add and subtract 0 .. upper_bound, creating a "bell curve" effect
            let rng_add = self.rng.get(gain_rate[curve_segment as usize][1]);
            let rng_sub = self.rng.get(gain_rate[curve_segment as usize][1]);

            let res_fp = (gain_rate[curve_segment as usize][0] + rng_add).wrapping_sub(rng_sub);

            let res = if res_fp as i8 <= 0 {
                0
            } else {
                (res_fp + 8) >> 4 // 8 = 0.5 in 4.4fp | round up >= .5
            };

            return res;
        } else { // level 20
            let level_max_current_diff = 99 - level;
            let stat_max_current_diff = stat[1] - current_stat;
            
            let mut stat_diff_4_4 = stat_max_current_diff << 4;
            stat_diff_4_4 /= level_max_current_diff; // level diff is essentially 16 times smaller here
            
            // ± "bell curve" 3/16
            stat_diff_4_4 += self.rng.get(4);
            stat_diff_4_4 = stat_diff_4_4.saturating_sub(self.rng.get(4));

            (stat_diff_4_4 + 8) >> 4 // 8 = 0.5 in 4.4fp | round up >= .5
        }
    }
}

// characters
// Nick
// Ruce
// Shade
// Sig
// Wendy
// Apis




// spells
// 0A egress
// 0D, 4D, 8D, CD bolt 1-4

//nick spells
// 01 0A
// 10 0D
// 14 4D
// 19 8D
// 1E CD
