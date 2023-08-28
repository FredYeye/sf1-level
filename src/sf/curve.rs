#[derive(Clone, Copy)]
pub enum Curve {
    Linear, Fast, Slow, FastSlow,
}

impl Curve {
    pub fn curve_points(&self) -> Vec<(u8, u8)> {
        match self {
            //level to curve point / percentage
            Curve::Linear   => vec![(1, 0), (20, 100)], //curve 0 (0x08)
            Curve::Slow     => vec![(1, 0), (10, 20), (14, 40), (20, 100)], //curve 1 (0x0A)

            //todo: switch names on these two? am i dumb??
            Curve::Fast     => vec![(1, 0), ( 4, 50), ( 6, 70), (10, 90), (14, 95), (20, 100)], //curve 3 (0x10)
            Curve::FastSlow => vec![(1, 0), ( 4, 30), ( 6, 40), (14, 60), (16, 70), (20, 100)], //curve 2 (0x1A)
        }
    }
}
