use gsae_chart_core::{default_chart, SparseChart};
use gsae_pullback_metric::{ChartMetric, PullbackMetricField};
use gsae_state_geometry::{ConformalPlaneMetric, StateMetric};
use gsae_core_types::CodePoint;

fn sample_code() -> CodePoint {
    CodePoint::new(vec![0.1, -0.2])
}

#[test]
fn pullback_matches_definition() {
    let chart = default_chart();
    let metric = ConformalPlaneMetric { k: 0.6 };
    let field = ChartMetric::new(chart.clone(), metric.clone());
    let z = sample_code();
    let j = chart.jacobian_decode(&z);
    let g_m = metric.metric(&chart.decode(&z));
    let g_expected = j.transpose().matmul(&g_m).matmul(&j);
    let g_actual = field.g(&z);
    assert!(g_actual.approx_eq(&g_expected, 1e-10));
}

#[test]
fn spd_property() {
    let chart = default_chart();
    let metric = ConformalPlaneMetric { k: 0.6 };
    let field = ChartMetric::new(chart, metric);
    let g = field.g(&sample_code());
    assert!(g.determinant_2x2() > 0.0);
}

#[test]
fn dg_is_nontrivial() {
    let chart = default_chart();
    let metric = ConformalPlaneMetric { k: 0.6 };
    let field = ChartMetric::new(chart, metric);
    let dg = field.dg(&sample_code());
    let mut max: f64 = 0.0;
    for i in 0..dg.n0 {
        for j in 0..dg.n1 {
            for k in 0..dg.n2 {
                max = max.max(dg[(i, j, k)].abs());
            }
        }
    }
    assert!(max > 1e-8);
}
