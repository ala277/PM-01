use crate::{
    bias::PossibilityBias,
    constraint::apply_bias,
    history::History,
    space::PossibilitySpace,
    state::PossibilityState,
    transition::Transition,
};

pub struct Engine {
    pub space: PossibilitySpace,
    pub initial_state: PossibilityState,
    pub state: PossibilityState,
    pub history: History,
}

impl Engine {
    pub fn new(
        space: PossibilitySpace,
        state: PossibilityState,
    ) -> Result<Self, String> {
        if space.len() != state.len() {
            return Err("PossibilitySpace and state dimensions do not match".into());
        }

        Ok(Self {
            space,
            initial_state: state.clone(),
            state,
            history: History::new(),
        })
    }

    pub fn step(&mut self, bias: &PossibilityBias) -> Result<Transition, String> {
        if bias.len() != self.state.len() {
            return Err("Bias dimension does not match current state".into());
        }

        let previous = self.state.clone();
        let next = apply_bias(&previous, &bias.delta)?;

        let transition = Transition::new(
            self.history.len() + 1,
            previous,
            bias.clone(),
            next.clone(),
        );

        self.state = next;
        self.history.push(transition.clone());

        Ok(transition)
    }

    pub fn total_drift_from_initial(&self) -> Vec<f64> {
        self.state
            .weights
            .iter()
            .zip(self.initial_state.weights.iter())
            .map(|(current, initial)| current - initial)
            .collect()
    }

    pub fn print_state(&self) {
        self.print_vector(&self.state.weights);
    }

    pub fn print_vector(&self, values: &[f64]) {
        for (label, value) in self.space.labels.iter().zip(values.iter()) {
            println!("  {label} = {value:+.4} ({:.2}%)", value, value * 100.0);
        }
    }
}
