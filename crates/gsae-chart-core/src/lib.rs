use gsae_core_types::{ChartId, CodePoint, HyperedgeId, LayerId, Scalar, StatePoint};
use gsae_linalg::Matrix;

pub trait SparseChart {
    fn chart_id(&self) -> ChartId;
    fn layer_id(&self) -> LayerId;
    fn latent_dim(&self) -> usize;
    fn state_dim(&self) -> usize;

    fn encode(&self, x: &StatePoint) -> CodePoint;
    fn decode(&self, z: &CodePoint) -> StatePoint;
    fn validity(&self, x: &StatePoint) -> bool;
    fn support_size(&self, z: &CodePoint) -> usize;

    fn jacobian_encode(&self, x: &StatePoint) -> Matrix;
    fn jacobian_decode(&self, z: &CodePoint) -> Matrix;
    fn hessian_decode_component(&self, z: &CodePoint, output_idx: usize) -> Matrix;
}

pub trait LatentDiffeomorphism {
    fn forward(&self, z: &CodePoint) -> CodePoint;
    fn inverse(&self, z: &CodePoint) -> CodePoint;
    fn jacobian_forward(&self, z: &CodePoint) -> Matrix;
    fn jacobian_inverse(&self, z: &CodePoint) -> Matrix;
    fn hessian_inverse_component(&self, z: &CodePoint, output_idx: usize) -> Matrix;
}

#[derive(Clone, Debug)]
pub struct AnalyticSparseChart {
    pub chart: ChartId,
    pub layer: LayerId,
    pub c: Scalar,
    pub gamma: Scalar,
    pub sparsity_threshold: Scalar,
    pub validity_vmax: Scalar,
}

impl AnalyticSparseChart {
    pub fn decode_without_hyperedge(&self, z: &CodePoint) -> StatePoint {
        let u = z.data[0];
        let v = z.data[1];
        StatePoint::new(vec![u, v + self.c * u * u])
    }

    pub fn hyperedge_value(&self, z: &CodePoint) -> Scalar {
        self.gamma * z.data[0] * z.data[1]
    }

    pub fn mixed_second_derivative_y(&self) -> Scalar {
        self.gamma
    }
}

impl SparseChart for AnalyticSparseChart {
    fn chart_id(&self) -> ChartId { self.chart.clone() }
    fn layer_id(&self) -> LayerId { self.layer }
    fn latent_dim(&self) -> usize { 2 }
    fn state_dim(&self) -> usize { 2 }

    fn encode(&self, x: &StatePoint) -> CodePoint {
        let u = x.data[0];
        let denom = 1.0 + self.gamma * u;
        let v = (x.data[1] - self.c * u * u) / denom;
        CodePoint::new(vec![u, v])
    }

    fn decode(&self, z: &CodePoint) -> StatePoint {
        let u = z.data[0];
        let v = z.data[1];
        StatePoint::new(vec![u, v + self.c * u * u + self.gamma * u * v])
    }

    fn validity(&self, x: &StatePoint) -> bool {
        let u = x.data[0];
        let denom = 1.0 + self.gamma * u;
        if denom.abs() < 1e-6 {
            return false;
        }
        let z = self.encode(x);
        z.data[1].abs() <= self.validity_vmax
    }

    fn support_size(&self, z: &CodePoint) -> usize {
        z.data.iter().filter(|v| v.abs() > self.sparsity_threshold).count()
    }

    fn jacobian_encode(&self, x: &StatePoint) -> Matrix {
        let u = x.data[0];
        let y = x.data[1];
        let denom = 1.0 + self.gamma * u;
        let v_num = y - self.c * u * u;
        let dv_du = ((-2.0 * self.c * u) * denom - v_num * self.gamma) / (denom * denom);
        let dv_dy = 1.0 / denom;
        Matrix::from_rows(&[
            &[1.0, 0.0],
            &[dv_du, dv_dy],
        ])
    }

    fn jacobian_decode(&self, z: &CodePoint) -> Matrix {
        let u = z.data[0];
        let v = z.data[1];
        Matrix::from_rows(&[
            &[1.0, 0.0],
            &[2.0 * self.c * u + self.gamma * v, 1.0 + self.gamma * u],
        ])
    }

    fn hessian_decode_component(&self, _z: &CodePoint, output_idx: usize) -> Matrix {
        let mut out = Matrix::zeros(2, 2);
        if output_idx == 1 {
            out[(0, 0)] = 2.0 * self.c;
            out[(0, 1)] = self.gamma;
            out[(1, 0)] = self.gamma;
        }
        out
    }
}

#[derive(Clone, Debug)]
pub struct QuadraticShear {
    pub sigma: Scalar,
}

impl LatentDiffeomorphism for QuadraticShear {
    fn forward(&self, z: &CodePoint) -> CodePoint {
        let u = z.data[0];
        let v = z.data[1] + self.sigma * z.data[0] * z.data[0];
        CodePoint::new(vec![u, v])
    }

    fn inverse(&self, z: &CodePoint) -> CodePoint {
        let u = z.data[0];
        let v = z.data[1] - self.sigma * z.data[0] * z.data[0];
        CodePoint::new(vec![u, v])
    }

    fn jacobian_forward(&self, z: &CodePoint) -> Matrix {
        let u = z.data[0];
        Matrix::from_rows(&[
            &[1.0, 0.0],
            &[2.0 * self.sigma * u, 1.0],
        ])
    }

    fn jacobian_inverse(&self, z: &CodePoint) -> Matrix {
        let u = z.data[0];
        Matrix::from_rows(&[
            &[1.0, 0.0],
            &[-2.0 * self.sigma * u, 1.0],
        ])
    }

    fn hessian_inverse_component(&self, z: &CodePoint, output_idx: usize) -> Matrix {
        let mut out = Matrix::zeros(2, 2);
        if output_idx == 1 {
            out[(0, 0)] = -2.0 * self.sigma;
        }
        let _ = z;
        out
    }
}

#[derive(Clone, Debug)]
pub struct ReparameterizedChart {
    pub chart: ChartId,
    pub layer: LayerId,
    pub base: AnalyticSparseChart,
    pub diff: QuadraticShear,
}

impl SparseChart for ReparameterizedChart {
    fn chart_id(&self) -> ChartId { self.chart.clone() }
    fn layer_id(&self) -> LayerId { self.layer }
    fn latent_dim(&self) -> usize { 2 }
    fn state_dim(&self) -> usize { 2 }

    fn encode(&self, x: &StatePoint) -> CodePoint {
        let z = self.base.encode(x);
        self.diff.inverse(&z)
    }

    fn decode(&self, z: &CodePoint) -> StatePoint {
        let z_base = self.diff.forward(z);
        self.base.decode(&z_base)
    }

    fn validity(&self, x: &StatePoint) -> bool {
        self.base.validity(x)
    }

    fn support_size(&self, z: &CodePoint) -> usize {
        let z_base = self.diff.forward(z);
        self.base.support_size(&z_base)
    }

    fn jacobian_encode(&self, x: &StatePoint) -> Matrix {
        let z_base = self.base.encode(x);
        let j_base = self.base.jacobian_encode(x);
        let j_inv = self.diff.jacobian_inverse(&z_base);
        j_inv.matmul(&j_base)
    }

    fn jacobian_decode(&self, z: &CodePoint) -> Matrix {
        let z_base = self.diff.forward(z);
        let j_base = self.base.jacobian_decode(&z_base);
        let j_diff = self.diff.jacobian_forward(z);
        j_base.matmul(&j_diff)
    }

    fn hessian_decode_component(&self, z: &CodePoint, output_idx: usize) -> Matrix {
        let z_base = self.diff.forward(z);
        let j_base = self.base.jacobian_decode(&z_base);
        let h_base = self.base.hessian_decode_component(&z_base, output_idx);
        let j_inv = self.diff.jacobian_forward(z);
        let h_inv = self.diff.hessian_inverse_component(z, output_idx);
        let mut out = Matrix::zeros(2, 2);
        for i in 0..2 {
            for j in 0..2 {
                let mut value = 0.0;
                for p in 0..2 {
                    for q in 0..2 {
                        value += h_base[(p, q)] * j_inv[(p, i)] * j_inv[(q, j)];
                    }
                }
                if output_idx == 1 {
                    value += j_base[(1, 1)] * h_inv[(i, j)];
                }
                out[(i, j)] = value;
            }
        }
        out
    }
}

pub fn default_chart() -> AnalyticSparseChart {
    AnalyticSparseChart {
        chart: ChartId("alpha"),
        layer: LayerId(0),
        c: 0.4,
        gamma: 0.7,
        sparsity_threshold: 0.15,
        validity_vmax: 1.5,
    }
}

pub fn gauge_chart() -> ReparameterizedChart {
    ReparameterizedChart {
        chart: ChartId("beta"),
        layer: LayerId(0),
        base: default_chart(),
        diff: QuadraticShear { sigma: 0.35 },
    }
}

pub fn transport_chart() -> ReparameterizedChart {
    ReparameterizedChart {
        chart: ChartId("gamma"),
        layer: LayerId(1),
        base: default_chart(),
        diff: QuadraticShear { sigma: -0.2 },
    }
}

pub fn canonical_hyperedge() -> HyperedgeId {
    HyperedgeId("e_{uv}")
}

pub fn inverse_consistency_error<C: SparseChart>(chart: &C, x: &StatePoint) -> Scalar {
    let z = chart.encode(x);
    let x_hat = chart.decode(&z);
    ((x_hat.data[0] - x.data[0]).powi(2) + (x_hat.data[1] - x.data[1]).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gsae_autodiff::central_difference;

    #[test]
    fn encode_decode_local_reconstruction() {
        let chart = default_chart();
        let x = StatePoint::new(vec![0.2, 0.3]);
        let err = inverse_consistency_error(&chart, &x);
        assert!(err < 1e-10);
    }

    #[test]
    fn jacobian_encode_matches_finite_difference() {
        let chart = default_chart();
        let x = StatePoint::new(vec![0.4, -0.1]);
        let j = chart.jacobian_encode(&x);
        let eps = 1e-6;
        for out in 0..2 {
            let f = |xp: &[Scalar]| {
                let x = StatePoint::new(vec![xp[0], xp[1]]);
                chart.encode(&x).data[out]
            };
            let grad = central_difference(f, &[x.data[0], x.data[1]], eps);
            assert!((j[(out, 0)] - grad[0]).abs() < 1e-5);
            assert!((j[(out, 1)] - grad[1]).abs() < 1e-5);
        }
    }

    #[test]
    fn sparsity_support_is_explicit() {
        let chart = default_chart();
        let x = StatePoint::new(vec![0.2, chart.c * 0.2 * 0.2]);
        let z = chart.encode(&x);
        assert_eq!(chart.support_size(&z), 1);
    }

    #[test]
    fn validity_distinguishes_out_of_domain() {
        let chart = default_chart();
        let x = StatePoint::new(vec![0.2, chart.c * 0.2 * 0.2]);
        let bad = StatePoint::new(vec![0.2, chart.c * 0.2 * 0.2 + 10.0]);
        assert!(chart.validity(&x));
        assert!(!chart.validity(&bad));
    }
}
