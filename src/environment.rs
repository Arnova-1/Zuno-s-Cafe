#[derive(Debug, Copy, Clone)]
pub struct Environment {
    pub money: i32,
    pub world_ticks: u32
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            money: 0,
            world_ticks: 30
        }
    }
}

impl Environment {
    pub fn run_tick(&mut self) {
        self.world_ticks = self.world_ticks.saturating_sub(1);
    }
}