#[derive(Debug)]
pub struct Rand {
    pub state: u16,
}

impl Rand {
    pub fn get(&mut self, upper_bound: u8) -> u8 {
        self.update();
        (self.state as u32 * upper_bound as u32).to_le_bytes()[2]
    }

    fn update(&mut self) {
        self.state = self.state.wrapping_mul(13).wrapping_add(7);
    }
}
