#[test]
fn pm01_specification_is_consistent() {
    let state = [0.20_f64, 0.50, 0.30];
    let bias = [0.10_f64, -0.05, -0.05];

    let next: Vec<f64> = state.iter().zip(bias.iter()).map(|(p, b)| p + b).collect();

    assert!((next.iter().sum::<f64>() - 1.0).abs() < 1e-9);
    assert_eq!(next, vec![0.30, 0.45, 0.25]);
}
