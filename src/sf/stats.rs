use super::{character::Character, curve::Curve};

#[derive(Debug, Clone, Copy)]
pub enum Id {
    Max,
    Mae, Pelle, Ken, Vankar, Earnest, Arthur,
    Gort, Luke,
    Guntz,
    Anri, Alef, Tao,
    Domingo,
    Lowe, Khris, Torasu,
    Gong,
    Diane, Hans,
    Lyle,
    Amon, Balbaroy,
    Kokichi,
    Bleu,
    Adam,
    Zylo,
    Musashi,
    Hanzou,
}

struct StatGain {
    pub gain: u8,
    pub curve_type: Curve,
}

pub struct BaseStats {
    pub starting_level: u8,
    base: [u8; 6], //attack, defense, agility, hp, mp, crit
    stat_gain: [[StatGain; 6]; 2],
}

impl Id {
    pub fn get_ids() -> Vec<Self> {
        vec![
            Self::Max,
            Self::Mae, Self::Pelle, Self::Ken, Self::Vankar, Self::Earnest, Self::Arthur,
            Self::Gort, Self::Luke,
            Self::Guntz,
            Self::Anri, Self::Alef, Self::Tao,
            Self::Domingo,
            Self::Lowe, Self::Khris, Self::Torasu,
            Self::Gong,
            Self::Diane, Self::Hans,
            Self::Lyle,
            Self::Amon, Self::Balbaroy,
            Self::Kokichi,
            Self::Bleu,
            Self::Adam,
            Self::Zylo,
            Self::Musashi,
            Self::Hanzou,
        ]
    }

    pub fn base_stats(&self) -> BaseStats {
        match self {
            Id::Max => BaseStats {
                starting_level: 1,
                base: [6, 4, 4, 12, 8, 3],
                stat_gain: [ [
                    StatGain { gain: 17, curve_type: Curve::EarlyLate },
                    StatGain { gain: 16, curve_type: Curve::EarlyLate },
                    StatGain { gain: 14, curve_type: Curve::EarlyLate },
                    StatGain { gain: 23, curve_type: Curve::Linear    },
                    StatGain { gain:  4, curve_type: Curve::EarlyLate },
                    StatGain { gain:  4, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 23, curve_type: Curve::Linear    },
                    StatGain { gain: 48, curve_type: Curve::Linear    },
                    StatGain { gain: 35, curve_type: Curve::Early     },
                    StatGain { gain: 46, curve_type: Curve::Linear    },
                    StatGain { gain:  2, curve_type: Curve::EarlyLate },
                    StatGain { gain:  8, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Mae => BaseStats {
                starting_level: 2,
                base: [5, 5, 7, 11, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 15, curve_type: Curve::Linear    },
                    StatGain { gain: 15, curve_type: Curve::Early     },
                    StatGain { gain: 14, curve_type: Curve::Linear    },
                    StatGain { gain: 24, curve_type: Curve::Late      },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 18, curve_type: Curve::Linear    },
                    StatGain { gain: 35, curve_type: Curve::Linear    },
                    StatGain { gain: 45, curve_type: Curve::Linear    },
                    StatGain { gain: 43, curve_type: Curve::Early     },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  4, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Pelle => BaseStats {
                starting_level: 8,
                base: [6, 7, 7, 10, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 13, curve_type: Curve::Early     },
                    StatGain { gain: 16, curve_type: Curve::Early     },
                    StatGain { gain: 16, curve_type: Curve::EarlyLate },
                    StatGain { gain: 18, curve_type: Curve::EarlyLate },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 17, curve_type: Curve::EarlyLate },
                    StatGain { gain: 37, curve_type: Curve::Linear    },
                    StatGain { gain: 46, curve_type: Curve::Late      },
                    StatGain { gain: 42, curve_type: Curve::EarlyLate },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  3, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Ken => BaseStats {
                starting_level: 1,
                base: [7, 6, 5, 8, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 14, curve_type: Curve::Linear    },
                    StatGain { gain: 15, curve_type: Curve::Late      },
                    StatGain { gain: 14, curve_type: Curve::Linear    },
                    StatGain { gain: 23, curve_type: Curve::Early     },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 21, curve_type: Curve::Late      },
                    StatGain { gain: 37, curve_type: Curve::Late      },
                    StatGain { gain: 40, curve_type: Curve::Linear    },
                    StatGain { gain: 42, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  5, curve_type: Curve::EarlyLate },
                ] ],
            },

            Id::Vankar => BaseStats {
                starting_level: 8,
                base: [7, 6, 6, 10, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 11, curve_type: Curve::EarlyLate },
                    StatGain { gain: 14, curve_type: Curve::Linear    },
                    StatGain { gain: 14, curve_type: Curve::Linear    },
                    StatGain { gain: 24, curve_type: Curve::Early     },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 17, curve_type: Curve::EarlyLate },
                    StatGain { gain: 34, curve_type: Curve::EarlyLate },
                    StatGain { gain: 43, curve_type: Curve::Linear    },
                    StatGain { gain: 42, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  2, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Earnest => BaseStats {
                starting_level: 8,
                base: [7, 7, 7, 10, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 12, curve_type: Curve::EarlyLate },
                    StatGain { gain: 11, curve_type: Curve::EarlyLate },
                    StatGain { gain: 15, curve_type: Curve::Linear    },
                    StatGain { gain: 20, curve_type: Curve::EarlyLate },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 20, curve_type: Curve::EarlyLate },
                    StatGain { gain: 34, curve_type: Curve::EarlyLate },
                    StatGain { gain: 46, curve_type: Curve::Linear    },
                    StatGain { gain: 41, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  2, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Arthur => BaseStats {
                starting_level: 4,
                base: [5, 6, 6, 8, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 20, curve_type: Curve::Late      },
                    StatGain { gain: 16, curve_type: Curve::Late      },
                    StatGain { gain: 19, curve_type: Curve::Late      },
                    StatGain { gain: 29, curve_type: Curve::Late      },
                    StatGain { gain: 10, curve_type: Curve::Late      },
                    StatGain { gain:  2, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 20, curve_type: Curve::Late      },
                    StatGain { gain: 42, curve_type: Curve::Late      },
                    StatGain { gain: 47, curve_type: Curve::Late      },
                    StatGain { gain: 47, curve_type: Curve::Late      },
                    StatGain { gain: 22, curve_type: Curve::Linear    },
                    StatGain { gain:  6, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Gort => BaseStats {
                starting_level: 2,
                base: [8, 7, 4, 12, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 17, curve_type: Curve::Linear    },
                    StatGain { gain: 18, curve_type: Curve::EarlyLate },
                    StatGain { gain: 13, curve_type: Curve::Linear    },
                    StatGain { gain: 16, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  4, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 19, curve_type: Curve::Linear    },
                    StatGain { gain: 51, curve_type: Curve::Linear    },
                    StatGain { gain: 33, curve_type: Curve::EarlyLate },
                    StatGain { gain: 42, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  8, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Luke => BaseStats {
                starting_level: 1,
                base: [9, 7, 4, 9, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 14, curve_type: Curve::Linear    },
                    StatGain { gain: 19, curve_type: Curve::Linear    },
                    StatGain { gain: 12, curve_type: Curve::Linear    },
                    StatGain { gain: 16, curve_type: Curve::EarlyLate },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  4, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 26, curve_type: Curve::Linear    },
                    StatGain { gain: 53, curve_type: Curve::Linear    },
                    StatGain { gain: 35, curve_type: Curve::EarlyLate },
                    StatGain { gain: 40, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  8, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Guntz => BaseStats {
                starting_level: 8,
                base: [10, 11, 4, 9, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 15, curve_type: Curve::Linear    },
                    StatGain { gain: 19, curve_type: Curve::Linear    },
                    StatGain { gain: 11, curve_type: Curve::Early     },
                    StatGain { gain: 15, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  2, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 19, curve_type: Curve::Linear    },
                    StatGain { gain: 60, curve_type: Curve::Linear    },
                    StatGain { gain: 33, curve_type: Curve::EarlyLate },
                    StatGain { gain: 38, curve_type: Curve::EarlyLate },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  6, curve_type: Curve::EarlyLate },
                ] ],
            },

            Id::Anri => BaseStats {
                starting_level: 3,
                base: [4, 3, 7, 8, 6, 3],
                stat_gain: [ [
                    StatGain { gain:  9, curve_type: Curve::Linear    },
                    StatGain { gain: 12, curve_type: Curve::Linear    },
                    StatGain { gain: 13, curve_type: Curve::Linear    },
                    StatGain { gain: 10, curve_type: Curve::Linear    },
                    StatGain { gain: 34, curve_type: Curve::EarlyLate },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 11, curve_type: Curve::EarlyLate },
                    StatGain { gain: 28, curve_type: Curve::Early     },
                    StatGain { gain: 43, curve_type: Curve::EarlyLate },
                    StatGain { gain: 35, curve_type: Curve::Linear    },
                    StatGain { gain: 44, curve_type: Curve::Linear    },
                    StatGain { gain:  2, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Alef => BaseStats {
                starting_level: 15,
                base: [4, 4, 7, 8, 6, 3],
                stat_gain: [ [
                    StatGain { gain:  8, curve_type: Curve::Linear    },
                    StatGain { gain: 10, curve_type: Curve::Linear    },
                    StatGain { gain: 15, curve_type: Curve::Linear    },
                    StatGain { gain: 12, curve_type: Curve::Linear    },
                    StatGain { gain: 36, curve_type: Curve::EarlyLate },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 10, curve_type: Curve::Linear    },
                    StatGain { gain: 27, curve_type: Curve::Linear    },
                    StatGain { gain: 45, curve_type: Curve::Linear    },
                    StatGain { gain: 36, curve_type: Curve::Linear    },
                    StatGain { gain: 47, curve_type: Curve::Linear    },
                    StatGain { gain:  2, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Tao => BaseStats {
                starting_level: 1,
                base: [4, 4, 6, 10, 7, 3],
                stat_gain: [ [
                    StatGain { gain:  7, curve_type: Curve::Linear    },
                    StatGain { gain: 11, curve_type: Curve::Linear    },
                    StatGain { gain: 18, curve_type: Curve::Linear    },
                    StatGain { gain: 12, curve_type: Curve::Linear    },
                    StatGain { gain: 34, curve_type: Curve::EarlyLate },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 12, curve_type: Curve::Linear    },
                    StatGain { gain: 29, curve_type: Curve::Linear    },
                    StatGain { gain: 46, curve_type: Curve::Early     },
                    StatGain { gain: 38, curve_type: Curve::Late      },
                    StatGain { gain: 47, curve_type: Curve::Linear    },
                    StatGain { gain:  2, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Domingo => BaseStats {
                starting_level: 1,
                base: [10, 15, 18, 15, 15, 4],
                stat_gain: [ [
                    StatGain { gain: 20, curve_type: Curve::Linear    },
                    StatGain { gain: 33, curve_type: Curve::Linear    },
                    StatGain { gain: 22, curve_type: Curve::Linear    },
                    StatGain { gain: 40, curve_type: Curve::Linear    },
                    StatGain { gain: 35, curve_type: Curve::Linear    },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Lowe => BaseStats {
                starting_level: 1,
                base: [6, 5, 5, 11, 10, 3],
                stat_gain: [ [
                    StatGain { gain: 10, curve_type: Curve::Linear    },
                    StatGain { gain: 10, curve_type: Curve::Linear    },
                    StatGain { gain: 13, curve_type: Curve::Linear    },
                    StatGain { gain: 15, curve_type: Curve::Linear    },
                    StatGain { gain: 25, curve_type: Curve::EarlyLate },
                    StatGain { gain:  2, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 13, curve_type: Curve::Linear    },
                    StatGain { gain: 30, curve_type: Curve::Linear    },
                    StatGain { gain: 40, curve_type: Curve::Early     },
                    StatGain { gain: 40, curve_type: Curve::Linear    },
                    StatGain { gain: 38, curve_type: Curve::Linear    },
                    StatGain { gain:  6, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Khris => BaseStats {
                starting_level: 2,
                base: [6, 5, 4, 10, 7, 3],
                stat_gain: [ [
                    StatGain { gain: 10, curve_type: Curve::Linear    },
                    StatGain { gain: 11, curve_type: Curve::Linear    },
                    StatGain { gain: 13, curve_type: Curve::Linear    },
                    StatGain { gain: 15, curve_type: Curve::Linear    },
                    StatGain { gain: 25, curve_type: Curve::EarlyLate },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 14, curve_type: Curve::Linear    },
                    StatGain { gain: 31, curve_type: Curve::Early     },
                    StatGain { gain: 38, curve_type: Curve::Linear    },
                    StatGain { gain: 37, curve_type: Curve::Linear    },
                    StatGain { gain: 38, curve_type: Curve::Linear    },
                    StatGain { gain:  4, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Torasu => BaseStats {
                starting_level: 16,
                base: [6, 5, 4, 10, 7, 3],
                stat_gain: [ [
                    StatGain { gain:  9, curve_type: Curve::Linear    },
                    StatGain { gain: 11, curve_type: Curve::Linear    },
                    StatGain { gain: 14, curve_type: Curve::Linear    },
                    StatGain { gain: 16, curve_type: Curve::Linear    },
                    StatGain { gain: 26, curve_type: Curve::EarlyLate },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 12, curve_type: Curve::Linear    },
                    StatGain { gain: 30, curve_type: Curve::Late      },
                    StatGain { gain: 38, curve_type: Curve::Linear    },
                    StatGain { gain: 38, curve_type: Curve::Linear    },
                    StatGain { gain: 44, curve_type: Curve::Linear    },
                    StatGain { gain:  4, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Gong => BaseStats {
                starting_level: 1,
                base: [11, 4, 6, 11, 8, 3],
                stat_gain: [ [
                    StatGain { gain: 29, curve_type: Curve::Linear    },
                    StatGain { gain: 16, curve_type: Curve::EarlyLate },
                    StatGain { gain: 11, curve_type: Curve::EarlyLate },
                    StatGain { gain: 19, curve_type: Curve::Linear    },
                    StatGain { gain: 16, curve_type: Curve::Linear    },
                    StatGain { gain:  4, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 26, curve_type: Curve::Linear    },
                    StatGain { gain: 31, curve_type: Curve::Linear    },
                    StatGain { gain: 39, curve_type: Curve::Linear    },
                    StatGain { gain: 42, curve_type: Curve::Linear    },
                    StatGain { gain: 28, curve_type: Curve::EarlyLate },
                    StatGain { gain: 20, curve_type: Curve::Late      },
                ] ],
            },

            Id::Diane => BaseStats {
                starting_level: 6,
                base: [6, 4, 6, 10, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 11, curve_type: Curve::Linear    },
                    StatGain { gain: 13, curve_type: Curve::Linear    },
                    StatGain { gain: 13, curve_type: Curve::EarlyLate },
                    StatGain { gain: 16, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  2, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 12, curve_type: Curve::Linear    },
                    StatGain { gain: 30, curve_type: Curve::Linear    },
                    StatGain { gain: 42, curve_type: Curve::Linear    },
                    StatGain { gain: 37, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  5, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Hans => BaseStats {
                starting_level: 1,
                base: [6, 5, 6, 12, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 12, curve_type: Curve::Late      },
                    StatGain { gain: 13, curve_type: Curve::Late      },
                    StatGain { gain: 12, curve_type: Curve::EarlyLate },
                    StatGain { gain: 18, curve_type: Curve::Late      },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  4, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 14, curve_type: Curve::Linear    },
                    StatGain { gain: 31, curve_type: Curve::Linear    },
                    StatGain { gain: 41, curve_type: Curve::Linear    },
                    StatGain { gain: 39, curve_type: Curve::Late      },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  9, curve_type: Curve::Late      },
                ] ],
            },

            Id::Lyle => BaseStats {
                starting_level: 8,
                base: [8, 4, 5, 9, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 15, curve_type: Curve::Linear    },
                    StatGain { gain: 12, curve_type: Curve::Linear    },
                    StatGain { gain: 12, curve_type: Curve::Linear    },
                    StatGain { gain: 15, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 16, curve_type: Curve::Linear    },
                    StatGain { gain: 28, curve_type: Curve::Linear    },
                    StatGain { gain: 40, curve_type: Curve::Linear    },
                    StatGain { gain: 37, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  3, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Amon => BaseStats {
                starting_level: 5,
                base: [5, 6, 7, 12, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 15, curve_type: Curve::Linear    },
                    StatGain { gain: 12, curve_type: Curve::Linear    },
                    StatGain { gain: 17, curve_type: Curve::Linear    },
                    StatGain { gain: 18, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 15, curve_type: Curve::Linear    },
                    StatGain { gain: 33, curve_type: Curve::Linear    },
                    StatGain { gain: 44, curve_type: Curve::Linear    },
                    StatGain { gain: 41, curve_type: Curve::Late      },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  4, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Balbaroy => BaseStats {
                starting_level: 5,
                base: [6, 7, 7, 10, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 12, curve_type: Curve::EarlyLate },
                    StatGain { gain: 13, curve_type: Curve::EarlyLate },
                    StatGain { gain: 14, curve_type: Curve::EarlyLate },
                    StatGain { gain: 16, curve_type: Curve::EarlyLate },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  3, curve_type: Curve::EarlyLate },
                ], [
                    StatGain { gain: 20, curve_type: Curve::Linear    },
                    StatGain { gain: 33, curve_type: Curve::Linear    },
                    StatGain { gain: 42, curve_type: Curve::Linear    },
                    StatGain { gain: 37, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  3, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Kokichi => BaseStats {
                starting_level: 7,
                base: [7, 5, 6, 10, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 12, curve_type: Curve::EarlyLate },
                    StatGain { gain: 12, curve_type: Curve::EarlyLate },
                    StatGain { gain: 13, curve_type: Curve::EarlyLate },
                    StatGain { gain: 18, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  1, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 18, curve_type: Curve::Linear    },
                    StatGain { gain: 33, curve_type: Curve::Linear    },
                    StatGain { gain: 41, curve_type: Curve::Linear    },
                    StatGain { gain: 40, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  3, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Bleu => BaseStats {
                starting_level: 9,
                base: [10, 8, 4, 12, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 35, curve_type: Curve::Linear    },
                    StatGain { gain: 22, curve_type: Curve::Linear    },
                    StatGain { gain: 13, curve_type: Curve::Linear    },
                    StatGain { gain: 18, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  4, curve_type: Curve::Late      },
                ], [
                    StatGain { gain: 37, curve_type: Curve::Linear    },
                    StatGain { gain: 55, curve_type: Curve::Linear    },
                    StatGain { gain: 37, curve_type: Curve::Linear    },
                    StatGain { gain: 58, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  2, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Adam => BaseStats {
                starting_level: 10,
                base: [10, 14, 5, 15, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 25, curve_type: Curve::Linear    },
                    StatGain { gain: 21, curve_type: Curve::Linear    },
                    StatGain { gain: 15, curve_type: Curve::Linear    },
                    StatGain { gain: 18, curve_type: Curve::Early     },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  2, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain: 41, curve_type: Curve::Linear    },
                    StatGain { gain: 53, curve_type: Curve::Linear    },
                    StatGain { gain: 37, curve_type: Curve::Linear    },
                    StatGain { gain: 44, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  3, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Zylo => BaseStats {
                starting_level: 9,
                base: [14, 7, 7, 9, 0, 3],
                stat_gain: [ [
                    StatGain { gain: 26, curve_type: Curve::EarlyLate },
                    StatGain { gain: 17, curve_type: Curve::Linear    },
                    StatGain { gain: 16, curve_type: Curve::Linear    },
                    StatGain { gain: 16, curve_type: Curve::Early     },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  4, curve_type: Curve::EarlyLate },
                ], [
                    StatGain { gain: 46, curve_type: Curve::Linear    },
                    StatGain { gain: 38, curve_type: Curve::Linear    },
                    StatGain { gain: 42, curve_type: Curve::Linear    },
                    StatGain { gain: 46, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  8, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Musashi => BaseStats {
                starting_level: 10,
                base: [18, 17, 16, 28, 0, 7],
                stat_gain: [ [
                    StatGain { gain: 22, curve_type: Curve::Linear    },
                    StatGain { gain: 53, curve_type: Curve::Linear    },
                    StatGain { gain: 24, curve_type: Curve::EarlyLate },
                    StatGain { gain: 52, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain: 18, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                ] ],
            },

            Id::Hanzou => BaseStats {
                starting_level: 10,
                base: [16, 16, 19, 23, 10, 5],
                stat_gain: [ [
                    StatGain { gain: 19, curve_type: Curve::Linear    },
                    StatGain { gain: 44, curve_type: Curve::Linear    },
                    StatGain { gain: 41, curve_type: Curve::Linear    },
                    StatGain { gain: 42, curve_type: Curve::Linear    },
                    StatGain { gain: 30, curve_type: Curve::Linear    },
                    StatGain { gain:  8, curve_type: Curve::Linear    },
                ], [
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                    StatGain { gain:  0, curve_type: Curve::Linear    },
                ] ],
            },
        }
    }

    pub fn get_base(&self, stat: usize) -> u8 {
        self.base_stats().base[stat]
    }

    pub fn get_gain(&self, stat: usize, promoted: bool) -> u8 {
        self.base_stats().stat_gain[promoted as usize][stat].gain
    }

    pub fn get_curve(&self, stat: usize, promoted: bool) -> Curve {
        self.base_stats().stat_gain[promoted as usize][stat].curve_type
    }

    pub fn character_from_id(&self) -> Character {
        let base_stats = self.base_stats();

        Character {
            id: self.clone(),
            level: base_stats.starting_level,
            stats: base_stats.base,
            promoted_level: None,
        }
    }
}
