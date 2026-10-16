mod agent;
mod environment;
mod state;

use crate::agent::{Action, Agent};
use crate::environment::Environment;
use crate::state::State;

const ACTION: [Action; 3] = [Action::Serve, Action::StartCooking, Action::Wait];

fn main() {
    let mut state = State::default();
    let mut environment = Environment::default();
    let mut agent = Agent::new();

    while environment.days <= 1000 {
        let prev_state = state.clone();

        let action_idx = agent.choose_action(state, environment.epsilon);

        let (outcome, reward) = state.step(&mut environment, ACTION[action_idx]);

        environment.earn(reward);

        agent.learn(reward, ACTION[action_idx], state, prev_state);

        println!("Days: {:?} | Ticks: {:?} | Money: {:?}", environment.days, environment.world_ticks,
                 environment.money);
        println!("Before: ({:?}, {:?}) | Action: {:?} | After: ({:?}, {:?}) | Outcome: ({:?}, {})",
                 prev_state.customer, prev_state.machine, ACTION[action_idx], state.customer, state.machine,
                 outcome, reward);

        environment.end_day()
    }
}