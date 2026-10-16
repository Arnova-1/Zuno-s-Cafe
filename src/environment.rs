#[derive(Debug, Copy, Clone)]
pub struct Environment {
    pub money: f32,
    pub world_ticks: u32,
    pub days: u32,
    pub epsilon: f32
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            money: 0.0,
            world_ticks: 30,
            days: 1,
            epsilon: 1.0
        }
    }
}

impl Environment {
    pub fn run_tick(&mut self) {
        self.world_ticks = self.world_ticks.saturating_sub(1);
    }
    pub fn earn(&mut self, reward: f32) {
        self.money += reward;
    }
    pub fn end_day(&mut self) {
        if self.world_ticks == 1 {
            self.days += 1;
            self.world_ticks = 30;
            self.epsilon = 0.05_f32.max(self.epsilon * 0.995);
        }
    }
}