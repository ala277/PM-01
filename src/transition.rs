use crate::{bias::PossibilityBias, space::PossibilitySpace, state::PossibilityState};

#[derive(Debug, Clone)]
pub struct Transition {
    pub step: usize,
    pub previous: PossibilityState,
    pub bias: PossibilityBias,
    pub next: PossibilityState,
    pub delta: Vec<f64>,
}

impl Transition {
    pub fn new(
        step: usize,
        previous: PossibilityState,
        bias: PossibilityBias,
        next: PossibilityState,
    ) -> Self {
        let delta = next
            .weights
            .iter()
            .zip(previous.weights.iter())
            .map(|(new, old)| new - old)
            .collect();

        Self {
            step,
            previous,
            bias,
            next,
            delta,
        }
    }

    pub fn print(&self, space: &PossibilitySpace) {
        for (label, value) in space.labels.iter().zip(self.next.weights.iter()) {
            println!("  {label} = {:.4} ({:.2}%)", value, value * 100.0);
        }

        println!("  Drift:");
        for (label, value) in space.labels.iter().zip(self.delta.iter()) {
            println!("    {label}: {:+.4}", value);
        }
    }
}
