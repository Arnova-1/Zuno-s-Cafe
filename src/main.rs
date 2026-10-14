mod agent;
mod environment;
mod state;

use crate::agent::Agent;
use crate::environment::Environment;
use crate::state::State;

fn main() {
    let mut state = State::default();
    let mut environment = Environment::default();
    let mut agent = Agent::default();

    let mut x = 0;

    while x < 30 {
        let prev_state = state.clone();

        let action = agent.choose_action();

        let outcome = state.step(&mut environment, action);

        println!("Before: ({:?}, {:?}) | Action: {:?} | After: ({:?}, {:?}) | Outcome: {:?}",
                 prev_state.customer, prev_state.machine, action, state.customer, state.machine, outcome);

        x += 1;
    }
}