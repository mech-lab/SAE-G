use gsae_1_chart::witness_chart_report;

#[test]
fn nontrivial_domain_exists() {
    let w = witness_chart_report();
    assert!(w.valid);
}

#[test]
fn sparse_codes_not_dense_noise() {
    let w = witness_chart_report();
    assert!(w.support_size < w.code.dim());
}

