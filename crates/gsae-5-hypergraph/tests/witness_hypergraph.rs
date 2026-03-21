use gsae_5_hypergraph::witness_hypergraph_report;

#[test]
fn hyperedge_is_non_additive() {
    let w = witness_hypergraph_report();
    assert!(w.singleton_fit_error.abs() > 1e-9);
}

#[test]
fn mixed_derivative_nonzero() {
    let w = witness_hypergraph_report();
    assert!(w.mixed_second_derivative.abs() > 1e-9);
}
