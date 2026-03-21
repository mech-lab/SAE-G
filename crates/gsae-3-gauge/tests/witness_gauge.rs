use gsae_3_gauge::witness_gauge_report;

#[test]
fn overlap_nonempty() {
    let w = witness_gauge_report();
    assert!(w.transition_error < w.naive_matching_baseline_error);
}

#[test]
fn transition_invertible() {
    let w = witness_gauge_report();
    assert!(w.transition_error < 1e-8);
}

#[test]
fn metric_compatibility_small() {
    let w = witness_gauge_report();
    assert!(w.isometry_error < 1e-6);
}
