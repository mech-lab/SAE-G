use gsae_chart_core::default_chart;
use gsae_geodesics::{GeodesicSolver, ShootingGeodesics};
use gsae_pullback_metric::ChartMetric;
use gsae_state_geometry::ConformalPlaneMetric;
use gsae_core_types::{CodePoint, Tangent};

fn sample_code() -> CodePoint {
    CodePoint::new(vec![0.1, 0.0])
}

fn nearby_code() -> CodePoint {
    CodePoint::new(vec![0.12, 0.01])
}

#[test]
fn zero_velocity_ivp() {
    let chart = default_chart();
    let metric = ConformalPlaneMetric { k: 0.6 };
    let field = ChartMetric::new(chart, metric);
    let solver = ShootingGeodesics { field };
    let z0 = sample_code();
    let v0 = Tangent::zeros(z0.dim());
    let path = solver.geodesic_ivp(&z0, &v0, 1.0, 0.05);
    assert_eq!(path.last().unwrap(), &z0);
}

#[test]
fn exp_log_inverse() {
    let chart = default_chart();
    let metric = ConformalPlaneMetric { k: 0.6 };
    let field = ChartMetric::new(chart, metric);
    let solver = ShootingGeodesics { field };
    let z0 = sample_code();
    let z1 = nearby_code();
    let v = solver.log_map(&z0, &z1, 0.02, 10);
    let z1_recovered = solver.exp_map(&z0, &v, 0.02);
    let diff = (z1_recovered.data[0] - z1.data[0]).abs() + (z1_recovered.data[1] - z1.data[1]).abs();
    assert!(diff < 1e-3);
}

#[test]
fn intrinsic_not_euclidean() {
    let chart = default_chart();
    let metric = ConformalPlaneMetric { k: 0.6 };
    let field = ChartMetric::new(chart, metric);
    let solver = ShootingGeodesics { field };
    let z0 = sample_code();
    let z1 = CodePoint::new(vec![0.5, 0.0]);
    let d_geo = solver.distance(&z0, &z1, 0.01, 12);
    let d_l2 = ((z1.data[0] - z0.data[0]).powi(2) + (z1.data[1] - z0.data[1]).powi(2)).sqrt();
    assert!((d_geo - d_l2).abs() > 1e-4);
}

