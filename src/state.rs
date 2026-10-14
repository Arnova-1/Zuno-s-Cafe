use rand::random_range;
use crate::agent::Action;
use crate::environment::Environment;

const SPAWN_RATE:f32 = 0.3;

#[derive(Debug, Copy, Clone)]
pub enum Machine {
    Idle,
    Cooking(u32),
    Done
}

#[derive(Debug, Copy, Clone)]
pub enum Customer {
    Arrived,
    Waiting(u32),
    Absent
}

#[derive(Debug)]
pub enum Outcome {
    Served,
    Left
}

#[derive(Debug, Copy, Clone)]
pub struct State {
    pub customer: Customer,
    pub machine: Machine
}

impl State {
    pub fn step (&mut self, environment: &mut Environment, action: Action) -> Option<(Outcome, f32)> {
        let outcome = self.try_serve(action);
        match (self.machine, action) {
            (Machine::Idle, Action::StartCooking) => {
                self.machine = Machine::Cooking(2);
            }
            (Machine::Cooking(1), _) => {
                self.machine = Machine::Done;
            }
            (Machine::Cooking(p ), _) => {
                self.machine = Machine::Cooking(p.saturating_sub(1));
            }
            _ => {}
        }
        match self.customer {
            Customer::Arrived => {
                self.customer = Customer::Waiting(10);
            }
            Customer::Waiting(1) => {
                    self.customer = Customer::Absent;
                    return Some((Outcome::Left, -10.0))
                }
            Customer::Waiting(p) => {
                self.customer = Customer::Waiting(p.saturating_sub(1));
            }
            _ => {}
        }

        if let Customer::Absent = self.customer {
            if random_range(0.0..1.0) < SPAWN_RATE {
                self.customer = Customer::Arrived
            }
        }

        environment.run_tick();
        outcome
    }
    fn try_serve(&mut self, action: Action) -> Option<(Outcome, f32)> {
        if let (Machine::Done, &Customer::Waiting(p), Action::Serve) = (&self.machine, &self.customer, &action) {
            self.customer = Customer::Absent;
            self.machine = Machine::Idle;
            return Some((Outcome::Served, 5.0 + p as f32))
        }
        None
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            machine: Machine::Idle,
            customer: Customer::Absent
        }
    }
}