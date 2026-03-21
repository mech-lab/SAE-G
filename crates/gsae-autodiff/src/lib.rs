use gsae_core_types::Scalar;
use gsae_linalg::Matrix;

pub trait ExactJacobian<I> {
    fn jacobian_exact(&self, input: &I) -> Matrix;
}

pub trait ExactHessianComponent<I> {
    fn hessian_component_exact(&self, input: &I, output_idx: usize) -> Matrix;
}

pub fn central_difference<F>(f: F, x: &[Scalar], eps: Scalar) -> Vec<Scalar>
where
    F: Fn(&[Scalar]) -> Scalar,
{
    let n = x.len();
    let mut out = vec![0.0; n];
    for i in 0..n {
        let mut xp = x.to_vec();
        let mut xm = x.to_vec();
        xp[i] += eps;
        xm[i] -= eps;
        out[i] = (f(&xp) - f(&xm)) / (2.0 * eps);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use gsae_linalg::Matrix;

    struct Quadratic;

    impl ExactJacobian<Vec<Scalar>> for Quadratic {
        fn jacobian_exact(&self, input: &Vec<Scalar>) -> Matrix {
            Matrix::from_rows(&[
                &[2.0 * input[0], 0.0],
                &[0.0, 2.0 * input[1]],
            ])
        }
    }

    impl ExactHessianComponent<Vec<Scalar>> for Quadratic {
        fn hessian_component_exact(&self, _input: &Vec<Scalar>, output_idx: usize) -> Matrix {
            let mut out = Matrix::zeros(2, 2);
            if output_idx == 0 {
                out[(0, 0)] = 2.0;
            } else if output_idx == 1 {
                out[(1, 1)] = 2.0;
            }
            out
        }
    }

    #[test]
    fn jacobian_matches_analytic_quadratic() {
        let f = Quadratic;
        let x = vec![0.3, -0.2];
        let j = f.jacobian_exact(&x);
        assert!((j[(0, 0)] - 0.6).abs() < 1e-12);
        assert!((j[(1, 1)] + 0.4).abs() < 1e-12);
    }

    #[test]
    fn hessian_component_matches_analytic() {
        let f = Quadratic;
        let x = vec![0.3, -0.2];
        let h0 = f.hessian_component_exact(&x, 0);
        let h1 = f.hessian_component_exact(&x, 1);
        assert!((h0[(0, 0)] - 2.0).abs() < 1e-12);
        assert!((h1[(1, 1)] - 2.0).abs() < 1e-12);
    }

    #[test]
    fn central_difference_is_consistent() {
        let f = |x: &[Scalar]| x[0] * x[0] + x[1] * x[1];
        let grad = central_difference(f, &[0.3, -0.2], 1e-6);
        assert!((grad[0] - 0.6).abs() < 1e-4);
        assert!((grad[1] + 0.4).abs() < 1e-4);
    }
}
