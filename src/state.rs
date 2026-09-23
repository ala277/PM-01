pub const EPSILON: f64 = 1e-9;

#[derive(Debug, Clone)]
pub struct PossibilityState {
    pub weights: Vec<f64>,
}

impl PossibilityState {
    pub fn new(weights: Vec<f64>) -> Result<Self, String> {
        let state = Self { weights };
        state.validate()?;
        Ok(state)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.weights.is_empty() {
            return Err("PossibilityState cannot be empty".into());
        }

        if self.weights.iter().any(|&x| !x.is_finite()) {
            return Err("State contains a non-finite value".into());
        }

        if self.weights.iter().any(|&x| x < -EPSILON || x > 1.0 + EPSILON) {
            return Err("Each possibility weight must be in [0, 1]".into());
        }

        let sum: f64 = self.weights.iter().sum();
        if (sum - 1.0).abs() > EPSILON {
            return Err(format!("State weights must sum to 1. Current sum = {sum}"));
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.weights.len()
    }
}
