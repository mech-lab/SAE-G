use gsae_6_hyperpath::witness_hyperpath_report;

#[test]
fn mechanism_requires_transport_and_hyperedge() {
    let w = witness_hyperpath_report();
    assert!(w.hyperedge_delta_after_transport.abs() > 1e-9);
    assert!(w.best_singleton_ablation_error > 1e-6);
}

#[test]
fn transport_error_small() {
    let w = witness_hyperpath_report();
    assert!(w.transport_error < 1e-8);
}
