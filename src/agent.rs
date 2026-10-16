use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::state::State;

const LEARNING_RATE: f32 = 0.1;
const DISCOUNT: f32 = 0.8;
const ACTION: [Action; 3] = [Action::Serve, Action::StartCooking, Action::Wait];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    StartCooking,
    Wait,
    Serve
}

#[derive(Default, Clone)]
pub struct Agent {
    pub brain: HashMap<State, [f32; 3]>,
}


impl Agent {
    pub fn new() -> Self {
        Self {
            brain: Default::default(),
        }
    }
    pub fn choose_action(&mut self, current_state: State, epsilon: f32) -> usize {
        let random_action_index = rand::random_range(0..3);
        let random_number = rand::random_range(0.0..1.0);

        if random_number > epsilon {
            self.best(current_state).1
        } else {
            random_action_index
        }
    }

    pub fn best(&self, current_state: State) -> (f32, usize) {
        let current_array = self.brain.get(&current_state).unwrap_or(&[0.0; 3]).clone();
        let combined: Vec<_> = current_array.into_iter().zip(0..ACTION.len()).collect();

        let best = combined.into_iter().fold(
            (f32::NEG_INFINITY, 0), | best, current | {
                if current.0 > best.0 {
                    current
                } else {
                    best
                }
            }
        );

        best
    }
    pub fn learn(&mut self, reward: f32, next_action: Action, current_state: State, prev_state: State) {
        let action_idx = ACTION.iter().position(|&x| x == next_action).unwrap();

        let (best, _) = self.best(current_state);
        
        let q_values = self.brain.entry(prev_state).or_insert([0.0; 3]);
        let current_q = q_values[action_idx];

        q_values[action_idx] = current_q + LEARNING_RATE * (reward + DISCOUNT * best - current_q);
    }
}