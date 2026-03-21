use gsae_core_types::{Scalar, StatePoint};
use gsae_linalg::{Matrix, Tensor3, Tensor4};

pub trait StateMetric {
    fn state_dim(&self) -> usize;
    fn metric(&self, x: &StatePoint) -> Matrix;
    fn metric_inv(&self, x: &StatePoint) -> Matrix;
    fn metric_deriv(&self, x: &StatePoint) -> Tensor3; // [c,a,b] = ∂_c G_ab
    fn christoffel(&self, x: &StatePoint) -> Tensor3; // Γ^k_{ij}
    fn curvature(&self, x: &StatePoint) -> Tensor4; // R^i_{jkl}
}

#[derive(Clone, Debug)]
pub struct ConformalPlaneMetric {
    pub k: Scalar,
}

impl ConformalPlaneMetric {
    fn lambda(&self, x: &StatePoint) -> Scalar {
        (2.0 * self.k * x.data[0] * x.data[0]).exp()
    }
}

impl StateMetric for ConformalPlaneMetric {
    fn state_dim(&self) -> usize { 2 }

    fn metric(&self, x: &StatePoint) -> Matrix {
        let lam = self.lambda(x);
        Matrix::from_rows(&[
            &[lam, 0.0],
            &[0.0, lam],
        ])
    }

    fn metric_inv(&self, x: &StatePoint) -> Matrix {
        let lam = self.lambda(x);
        Matrix::from_rows(&[
            &[1.0 / lam, 0.0],
            &[0.0, 1.0 / lam],
        ])
    }

    fn metric_deriv(&self, x: &StatePoint) -> Tensor3 {
        let lam = self.lambda(x);
        let mut t = Tensor3::zeros(2, 2, 2);
        t[(0, 0, 0)] = 4.0 * self.k * x.data[0] * lam;
        t[(0, 1, 1)] = 4.0 * self.k * x.data[0] * lam;
        t
    }

    fn christoffel(&self, x: &StatePoint) -> Tensor3 {
        let g_inv = self.metric_inv(x);
        let dg = self.metric_deriv(x);
        let mut gamma = Tensor3::zeros(2, 2, 2);
        for k in 0..2 {
            for i in 0..2 {
                for j in 0..2 {
                    let mut sum = 0.0;
                    for l in 0..2 {
                        let term = dg[(i, j, l)] + dg[(j, i, l)] - dg[(l, i, j)];
                        sum += 0.5 * g_inv[(k, l)] * term;
                    }
                    gamma[(k, i, j)] = sum;
                }
            }
        }
        gamma
    }

    fn curvature(&self, x: &StatePoint) -> Tensor4 {
        let eps = 1e-5;
        let gamma = self.christoffel(x);
        let mut dgamma = Tensor4::zeros(2, 2, 2, 2);
        for m in 0..2 {
            let mut xp = x.clone();
            let mut xm = x.clone();
            xp.data[m] += eps;
            xm.data[m] -= eps;
            let gp = self.christoffel(&xp);
            let gm = self.christoffel(&xm);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        dgamma[(i, j, k, m)] = (gp[(i, j, k)] - gm[(i, j, k)]) / (2.0 * eps);
                    }
                }
            }
        }
        let mut r = Tensor4::zeros(2, 2, 2, 2);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    for l in 0..2 {
                        let mut value = dgamma[(i, j, l, k)] - dgamma[(i, j, k, l)];
                        for m in 0..2 {
                            value += gamma[(i, k, m)] * gamma[(m, j, l)]
                                - gamma[(i, l, m)] * gamma[(m, j, k)];
                        }
                        r[(i, j, k, l)] = value;
                    }
                }
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metric_inverse_round_trip() {
        let metric = ConformalPlaneMetric { k: 0.6 };
        let x = StatePoint::new(vec![0.4, -0.2]);
        let g = metric.metric(&x);
        let g_inv = metric.metric_inv(&x);
        let eye = g.matmul(&g_inv);
        let expected = Matrix::from_rows(&[
            &[1.0, 0.0],
            &[0.0, 1.0],
        ]);
        assert!(eye.approx_eq(&expected, 1e-8));
    }

    #[test]
    fn christoffel_symmetry_in_lower_indices() {
        let metric = ConformalPlaneMetric { k: 0.6 };
        let x = StatePoint::new(vec![0.3, 0.1]);
        let gamma = metric.christoffel(&x);
        for k in 0..2 {
            for i in 0..2 {
                for j in 0..2 {
                    assert!((gamma[(k, i, j)] - gamma[(k, j, i)]).abs() < 1e-10);
                }
            }
        }
    }

    #[test]
    fn curvature_is_nontrivial_off_axis() {
        let metric = ConformalPlaneMetric { k: 0.6 };
        let x = StatePoint::new(vec![0.5, 0.2]);
        let r = metric.curvature(&x);
        let mut max: Scalar = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    for l in 0..2 {
                        max = max.max(r[(i, j, k, l)].abs());
                    }
                }
            }
        }
        assert!(max > 1e-8);
    }
}
