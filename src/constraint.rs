use crate::state::{PossibilityState, EPSILON};

pub fn apply_bias(
    state: &PossibilityState,
    bias: &[f64],
) -> Result<PossibilityState, String> {
    if state.weights.len() != bias.len() {
        return Err("State and bias dimensions do not match".into());
    }

    let next: Vec<f64> = state
        .weights
        .iter()
        .zip(bias.iter())
        .map(|(p, b)| p + b)
        .collect();

    // PM-0.1 deliberately rejects invalid transitions rather than
    // silently clipping or renormalizing them.
    if next.iter().any(|&x| x < -EPSILON || x > 1.0 + EPSILON) {
        return Err("Bias would produce a weight outside [0, 1]".into());
    }

    let sum: f64 = next.iter().sum();
    if (sum - 1.0).abs() > EPSILON {
        return Err(format!("Transition violates conservation: sum = {sum}"));
    }

    let cleaned = next
        .into_iter()
        .map(|x| if x.abs() < EPSILON { 0.0 } else { x })
        .collect();

    PossibilityState::new(cleaned)
}
