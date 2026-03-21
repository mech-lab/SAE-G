use gsae_4_transport::witness_transport_report;

#[test]
fn transport_beats_baseline() {
    let w = witness_transport_report();
    assert!(w.transported_code_error < w.identity_baseline_error);
}

#[test]
fn transport_preserves_metric() {
    let w = witness_transport_report();
    assert!(w.metric_preservation_error < 1e-6);
}
