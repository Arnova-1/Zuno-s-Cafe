use rand::random_range;

const ACTION: [Action; 3] = [Action::Serve, Action::StartCooking, Action::Wait];

#[derive(Debug, Copy, Clone)]
pub enum Action {
    StartCooking,
    Wait,
    Serve
}

#[derive(Default)]
pub struct Agent {

}

impl Agent {
    pub fn choose_action(&mut self) -> Action {
        let random = random_range(0..3);

        ACTION[random]
    }
}