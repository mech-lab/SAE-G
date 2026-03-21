use gsae_autodiff::{central_difference, ExactHessianComponent, ExactJacobian};
use gsae_linalg::Matrix;

struct AnalyticMap;

impl ExactJacobian<Vec<f64>> for AnalyticMap {
    fn jacobian_exact(&self, input: &Vec<f64>) -> Matrix {
        Matrix::from_rows(&[
            &[2.0 * input[0], 0.0],
            &[0.0, 2.0 * input[1]],
        ])
    }
}

impl ExactHessianComponent<Vec<f64>> for AnalyticMap {
    fn hessian_component_exact(&self, _input: &Vec<f64>, output_idx: usize) -> Matrix {
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
fn jacobian_matches_analytic() {
    let f = AnalyticMap;
    let x = vec![0.3, -0.2];
    let j = f.jacobian_exact(&x);
    let expected = Matrix::from_rows(&[
        &[0.6, 0.0],
        &[0.0, -0.4],
    ]);
    assert!(j.approx_eq(&expected, 1e-12));
}

#[test]
fn hessian_component_exists() {
    let f = AnalyticMap;
    let x = vec![0.3, -0.2];
    let h = f.hessian_component_exact(&x, 0);
    assert!(h[(0, 0)].abs() > 0.0);
}

#[test]
fn fd_matches_exact_jacobian() {
    let f = |x: &[f64]| x[0] * x[0];
    let grad = central_difference(f, &[0.3, -0.2], 1e-6);
    assert!((grad[0] - 0.6).abs() < 1e-4);
}
