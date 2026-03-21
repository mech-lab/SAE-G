use gsae_linalg::Matrix;

fn random_spd(n: usize) -> Matrix {
    let mut a = Matrix::zeros(n, n);
    for i in 0..n {
        for j in 0..n {
            a[(i, j)] = (i + 1) as f64 + (j + 1) as f64 * 0.3;
        }
    }
    let at = a.transpose();
    let mut g = at.matmul(&a);
    for i in 0..n {
        g[(i, i)] += 1.0;
    }
    g
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

#[test]
fn spd_inverse_roundtrip() {
    let g = random_spd(2);
    let g_inv = g.inverse_2x2();
    let ident = g.matmul(&g_inv);
    assert!(approx_identity(ident, 1e-8));
}

#[test]
fn spd_solve_matches_inverse() {
    let g = random_spd(2);
    let b = vec![1.0, -0.5];
    let x = g.solve_spd(&b);
    let x_ref = g.inverse_2x2().mul_vec(&b);
    assert!((x[0] - x_ref[0]).abs() < 1e-8);
    assert!((x[1] - x_ref[1]).abs() < 1e-8);
}

#[test]
fn logdet_consistency() {
    let g = random_spd(2);
    let logdet = g.logdet_spd();
    assert!(logdet.is_finite());
}
