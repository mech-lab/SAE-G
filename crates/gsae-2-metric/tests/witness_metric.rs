use gsae_2_metric::witness_metric_report;

#[test]
fn geodesic_differs_from_l2() {
    let w = witness_metric_report();
    let diff = (w.geodesic_distance - w.euclidean_distance).abs();
    assert!(diff > 1e-4);
}

