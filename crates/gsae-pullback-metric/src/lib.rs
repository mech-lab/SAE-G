use gsae_chart_core::SparseChart;
use gsae_core_types::{CodePoint, Scalar, Tangent};
use gsae_linalg::{dot, Matrix, Tensor3};
use gsae_state_geometry::StateMetric;

pub trait PullbackMetricField {
    fn g(&self, z: &CodePoint) -> Matrix;
    fn g_inv(&self, z: &CodePoint) -> Matrix;
    fn dg(&self, z: &CodePoint) -> Tensor3; // [k,i,j]
    fn christoffel(&self, z: &CodePoint) -> Tensor3; // Γ^k_{ij}
    fn metric_inner(&self, z: &CodePoint, u: &Tangent, v: &Tangent) -> Scalar;
}

#[derive(Clone)]
pub struct ChartMetric<C, M> {
    pub chart: C,
    pub state_metric: M,
}

impl<C, M> ChartMetric<C, M>
where
    C: SparseChart + Clone,
    M: StateMetric + Clone,
{
    pub fn new(chart: C, state_metric: M) -> Self {
        Self { chart, state_metric }
    }
}

impl<C, M> PullbackMetricField for ChartMetric<C, M>
where
    C: SparseChart + Clone,
    M: StateMetric + Clone,
{
    fn g(&self, z: &CodePoint) -> Matrix {
        let x = self.chart.decode(z);
        let j = self.chart.jacobian_decode(z);
        let g_state = self.state_metric.metric(&x);
        j.transpose().matmul(&g_state).matmul(&j)
    }

    fn g_inv(&self, z: &CodePoint) -> Matrix {
        self.g(z).inverse_2x2()
    }

    fn dg(&self, z: &CodePoint) -> Tensor3 {
        let x = self.chart.decode(z);
        let j = self.chart.jacobian_decode(z); // a,i
        let d_g = self.state_metric.metric_deriv(&x); // c,a,b
        let mut out = Tensor3::zeros(2, 2, 2);
        for k in 0..2 {
            let h0 = self.chart.hessian_decode_component(z, 0);
            let h1 = self.chart.hessian_decode_component(z, 1);
            for i in 0..2 {
                for j2 in 0..2 {
                    let mut value = 0.0;
                    // Hessian terms
                    for a in 0..2 {
                        for b in 0..2 {
                            let h_ki_a = if a == 0 { h0[(k, i)] } else { h1[(k, i)] };
                            let h_kj_b = if b == 0 { h0[(k, j2)] } else { h1[(k, j2)] };
                            let g_ab = self.state_metric.metric(&x)[(a, b)];
                            value += h_ki_a * g_ab * j[(b, j2)];
                            value += j[(a, i)] * g_ab * h_kj_b;
                        }
                    }
                    // Derivative of target metric along decoder
                    for a in 0..2 {
                        for b in 0..2 {
                            for c in 0..2 {
                                value += j[(a, i)] * d_g[(c, a, b)] * j[(c, k)] * j[(b, j2)];
                            }
                        }
                    }
                    out[(k, i, j2)] = value;
                }
            }
        }
        out
    }

    fn christoffel(&self, z: &CodePoint) -> Tensor3 {
        let g_inv = self.g_inv(z);
        let dg = self.dg(z);
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

    fn metric_inner(&self, z: &CodePoint, u: &Tangent, v: &Tangent) -> Scalar {
        let g = self.g(z);
        let tmp = g.mul_vec(&v.data);
        dot(&u.data, &tmp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gsae_chart_core::default_chart;
    use gsae_state_geometry::ConformalPlaneMetric;

    #[test]
    fn metric_is_symmetric_and_spd() {
        let chart = default_chart();
        let metric = ConformalPlaneMetric { k: 0.6 };
        let field = ChartMetric::new(chart, metric);
        let z = CodePoint::new(vec![0.2, 0.1]);
        let g = field.g(&z);
        assert!((g[(0, 1)] - g[(1, 0)]).abs() < 1e-12);
        assert!(g.determinant_2x2() > 0.0);
    }

    #[test]
    fn pullback_matches_jtg_j() {
        let chart = default_chart();
        let metric = ConformalPlaneMetric { k: 0.6 };
        let field = ChartMetric::new(chart.clone(), metric.clone());
        let z = CodePoint::new(vec![0.1, -0.2]);
        let x = chart.decode(&z);
        let j = chart.jacobian_decode(&z);
        let g_state = metric.metric(&x);
        let manual = j.transpose().matmul(&g_state).matmul(&j);
        let g = field.g(&z);
        assert!(g.approx_eq(&manual, 1e-12));
    }
}
