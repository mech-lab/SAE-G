use gsae_state_geometry::{ConformalPlaneMetric, StateMetric};
use gsae_core_types::StatePoint;
use gsae_linalg::Matrix;
use gsae_linalg::Tensor4;

struct NonEuclideanFixture {
    metric: ConformalPlaneMetric,
}

impl NonEuclideanFixture {
    fn new() -> Self {
        Self { metric: ConformalPlaneMetric { k: 0.6 } }
    }

    fn sample_point(&self) -> StatePoint {
        StatePoint::new(vec![0.5, -0.2])
    }

    fn metric(&self, x: &StatePoint) -> Matrix {
        self.metric.metric(x)
    }

    fn metric_inv(&self, x: &StatePoint) -> Matrix {
        self.metric.metric_inv(x)
    }

    fn christoffel(&self, x: &StatePoint) -> gsae_linalg::Tensor3 {
        self.metric.christoffel(x)
    }
}

fn approx_identity(m: Matrix, tol: f64) -> bool {
    for i in 0..m.rows {
        for j in 0..m.cols {
            let expected = if i == j { 1.0 } else { 0.0 };
            if (m[(i, j)] - expected).abs() > tol {
                return false;
            }
        }
    }
    true
}

fn tensor3_norm(t: &gsae_linalg::Tensor3) -> f64 {
    let mut sum = 0.0;
    for i in 0..t.n0 {
        for j in 0..t.n1 {
            for k in 0..t.n2 {
                let v = t[(i, j, k)];
                sum += v * v;
            }
        }
    }
    sum.sqrt()
}

fn tensor4_norm(t: &Tensor4) -> f64 {
    let mut sum = 0.0;
    for i in 0..t.n0 {
        for j in 0..t.n1 {
            for k in 0..t.n2 {
                for l in 0..t.n3 {
                    let v = t[(i, j, k, l)];
                    sum += v * v;
                }
            }
        }
    }
    sum.sqrt()
}

#[test]
fn metric_inverse_consistency() {
    let m = NonEuclideanFixture::new();
    let x = m.sample_point();
    let g = m.metric(&x);
    let g_inv = m.metric_inv(&x);
    let ident = g.matmul(&g_inv);
    assert!(approx_identity(ident, 1e-6));
}

#[test]
fn christoffel_nontrivial() {
    let m = NonEuclideanFixture::new();
    let x = m.sample_point();
    let gamma = m.christoffel(&x);
    assert!(tensor3_norm(&gamma) > 1e-6);
}

#[test]
fn curvature_nontrivial() {
    let m = NonEuclideanFixture::new();
    let x = m.sample_point();
    let r = m.metric.curvature(&x);
    assert!(tensor4_norm(&r) > 1e-8);
}
