# PM-0.1 — Possibility Mechanics

PM-0.1 is the first executable mathematical prototype of the fictional/proposed
"Possibility Mechanics" concept.

It does **not** claim to establish a new physical law. It tests whether the
concept can be represented as a consistent computational model.

## PM-0.1 rules

A possibility state is a vector:

P = [p1, p2, ..., pn]

with:

- 0 <= pi <= 1
- sum(pi) = 1

A Possibility Bias is a delta vector:

B = [b1, b2, ..., bn]

with:

- sum(bi) = 0

The transition rule is:

P(t+1) = P(t) + B(t)

The model records:

- previous state
- applied bias
- next state
- transition drift: Delta P = P(t+1) - P(t)

Invalid transitions are rejected rather than silently clipped or normalized.

## Run

Install Rust, then from this directory:

```bash
cargo run
