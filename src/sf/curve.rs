#[derive(Clone, Copy)]
pub enum Curve {
    Linear, Early, Late, EarlyLate,
}

impl Curve {
    pub fn curve_points(&self) -> Vec<(u8, u8)> {
        match self {
            //level to curve point / percentage
            Curve::Linear   => vec![(1, 0), (20, 100)], //curve 0 (0x08)
            Curve::Late     => vec![(1, 0), (10, 20), (14, 40), (20, 100)], //curve 1 (0x0A)
            Curve::Early     => vec![(1, 0), ( 4, 50), ( 6, 70), (10, 90), (14, 95), (20, 100)], //curve 3 (0x10)
            Curve::EarlyLate => vec![(1, 0), ( 4, 30), ( 6, 40), (14, 60), (16, 70), (20, 100)], //curve 2 (0x1A)
        }
    }
}

//todo: pre-compute curves