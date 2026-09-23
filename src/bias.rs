use crate::state::EPSILON;

#[derive(Debug, Clone)]
pub struct PossibilityBias {
    pub delta: Vec<f64>,
}

impl PossibilityBias {
    pub fn new(delta: Vec<f64>) -> Result<Self, String> {
        let bias = Self { delta };
        bias.validate()?;
        Ok(bias)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.delta.is_empty() {
            return Err("PossibilityBias cannot be empty".into());
        }

        if self.delta.iter().any(|&x| !x.is_finite()) {
            return Err("Bias contains a non-finite value".into());
        }

        // Conservation rule for PM-0.1:
        // the bias must redistribute possibility rather than create/destroy it.
        let sum: f64 = self.delta.iter().sum();
        if sum.abs() > EPSILON {
            return Err(format!(
                "Bias must sum to 0 in PM-0.1. Current sum = {sum}"
            ));
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.delta.len()
    }
}
