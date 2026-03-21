use gsae_chart_core::{default_chart, inverse_consistency_error, SparseChart};
use gsae_core_types::StatePoint;

fn sample_state() -> StatePoint {
    let u = 0.2;
    let chart = default_chart();
    let y = chart.c * u * u;
    StatePoint::new(vec![u, y])
}

#[test]
fn encode_decode_reconstruction() {
    let chart = default_chart();
    let h = sample_state();
    let err = inverse_consistency_error(&chart, &h);
    assert!(err < 1e-6);
}

#[test]
fn sparsity_enforced() {
    let chart = default_chart();
    let h = sample_state();
    let z = chart.encode(&h);
    let nonzero = z.data.iter().filter(|v| v.abs() > 0.0).count();
    assert!(nonzero < z.dim());
}

#[test]
fn jacobian_exists() {
    let chart = default_chart();
    let h = sample_state();
    let j = chart.jacobian_encode(&h);
    assert!(j.frob_norm() > 0.0);
}

#[test]
fn validity_distinguishes_out_of_domain() {
    let chart = default_chart();
    let h = sample_state();
    let bad = StatePoint::new(vec![0.2, 100.0]);
    assert!(chart.validity(&h));
    assert!(!chart.validity(&bad));
}
