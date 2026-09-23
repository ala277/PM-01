
---

# مجلد `src`

## 3. `src/main.rs`

```rust
mod bias;
mod constraint;
mod engine;
mod history;
mod space;
mod state;
mod transition;

use bias::PossibilityBias;
use engine::Engine;
use space::PossibilitySpace;
use state::PossibilityState;

fn main() -> Result<(), String> {
    println!("PM-0.1 — Possibility Mechanics");
    println!("================================\n");

    // Initial possibility space.
    let space = PossibilitySpace::new(vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
    ])?;

    // P0 = [0.20, 0.50, 0.30]
    let state = PossibilityState::new(vec![0.20, 0.50, 0.30])?;

    let mut engine = Engine::new(space, state)?;

    println!("Initial state:");
    engine.print_state();

    // A bias of +0.10 toward A, compensated by -0.05 from B and C.
    let bias = PossibilityBias::new(vec![0.10, -0.05, -0.05])?;

    println!("\nApplying Possibility Bias toward A...\n");

    for step in 1..=3 {
        let transition = engine.step(&bias)?;

        println!("Step {step}");
        println!("-------");
        transition.print(&engine.space);
        println!();
    }

    println!("Final state:");
    engine.print_state();

    println!("\nPossibility Drift from initial state:");
    let drift = engine.total_drift_from_initial();
    engine.print_vector(&drift);

    println!("\nHistory length: {} transitions", engine.history.len());

    Ok(())
}
