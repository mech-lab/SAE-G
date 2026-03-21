use gsae_core_types::Scalar;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Scalar>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tensor3 {
    pub n0: usize,
    pub n1: usize,
    pub n2: usize,
    pub data: Vec<Scalar>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tensor4 {
    pub n0: usize,
    pub n1: usize,
    pub n2: usize,
    pub n3: usize,
    pub data: Vec<Scalar>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self { rows, cols, data: vec![0.0; rows * cols] }
    }

    pub fn from_rows(rows: &[&[Scalar]]) -> Self {
        let r = rows.len();
        let c = rows[0].len();
        let mut out = Self::zeros(r, c);
        for i in 0..r {
            for j in 0..c {
                out[(i, j)] = rows[i][j];
            }
        }
        out
    }

    pub fn eye(n: usize) -> Self {
        let mut out = Self::zeros(n, n);
        for i in 0..n { out[(i, i)] = 1.0; }
        out
    }

    pub fn transpose(&self) -> Self {
        let mut out = Self::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                out[(j, i)] = self[(i, j)];
            }
        }
        out
    }

    pub fn matmul(&self, rhs: &Self) -> Self {
        assert_eq!(self.cols, rhs.rows);
        let mut out = Self::zeros(self.rows, rhs.cols);
        for i in 0..self.rows {
            for k in 0..self.cols {
                let a = self[(i, k)];
                for j in 0..rhs.cols {
                    out[(i, j)] += a * rhs[(k, j)];
                }
            }
        }
        out
    }

    pub fn mul_vec(&self, rhs: &[Scalar]) -> Vec<Scalar> {
        assert_eq!(self.cols, rhs.len());
        let mut out = vec![0.0; self.rows];
        for i in 0..self.rows {
            for j in 0..self.cols {
                out[i] += self[(i, j)] * rhs[j];
            }
        }
        out
    }

    pub fn add(&self, rhs: &Self) -> Self {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);
        let mut out = self.clone();
        for i in 0..out.data.len() { out.data[i] += rhs.data[i]; }
        out
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);
        let mut out = self.clone();
        for i in 0..out.data.len() { out.data[i] -= rhs.data[i]; }
        out
    }

    pub fn scale(&self, s: Scalar) -> Self {
        let mut out = self.clone();
        for v in &mut out.data { *v *= s; }
        out
    }

    pub fn determinant_2x2(&self) -> Scalar {
        assert_eq!(self.rows, 2);
        assert_eq!(self.cols, 2);
        self[(0,0)] * self[(1,1)] - self[(0,1)] * self[(1,0)]
    }

    pub fn cholesky(&self) -> Self {
        assert_eq!(self.rows, self.cols);
        let n = self.rows;
        let mut out = Self::zeros(n, n);
        for i in 0..n {
            for j in 0..=i {
                let mut sum = self[(i, j)];
                for k in 0..j {
                    sum -= out[(i, k)] * out[(j, k)];
                }
                if i == j {
                    assert!(sum > 0.0, "matrix not SPD");
                    out[(i, j)] = sum.sqrt();
                } else {
                    out[(i, j)] = sum / out[(j, j)];
                }
            }
        }
        out
    }

    pub fn solve_spd(&self, b: &[Scalar]) -> Vec<Scalar> {
        assert_eq!(self.rows, self.cols);
        assert_eq!(self.rows, b.len());
        let l = self.cholesky();
        let n = self.rows;
        let mut y = vec![0.0; n];
        for i in 0..n {
            let mut sum = b[i];
            for k in 0..i {
                sum -= l[(i, k)] * y[k];
            }
            y[i] = sum / l[(i, i)];
        }
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = y[i];
            for k in (i+1)..n {
                sum -= l[(k, i)] * x[k];
            }
            x[i] = sum / l[(i, i)];
        }
        x
    }

    pub fn logdet_spd(&self) -> Scalar {
        let l = self.cholesky();
        let mut sum = 0.0;
        for i in 0..l.rows {
            sum += l[(i, i)].ln();
        }
        2.0 * sum
    }

    pub fn inverse_2x2(&self) -> Self {
        assert_eq!(self.rows, 2);
        assert_eq!(self.cols, 2);
        let det = self.determinant_2x2();
        assert!(det.abs() > 1e-12, "singular 2x2 matrix");
        let mut out = Self::zeros(2, 2);
        out[(0,0)] = self[(1,1)] / det;
        out[(0,1)] = -self[(0,1)] / det;
        out[(1,0)] = -self[(1,0)] / det;
        out[(1,1)] = self[(0,0)] / det;
        out
    }

    pub fn frob_norm(&self) -> Scalar {
        self.data.iter().map(|x| x * x).sum::<Scalar>().sqrt()
    }

    pub fn approx_eq(&self, rhs: &Self, tol: Scalar) -> bool {
        self.rows == rhs.rows
            && self.cols == rhs.cols
            && self.data.iter().zip(&rhs.data).all(|(a, b)| (a - b).abs() <= tol)
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = Scalar;
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.data[idx.0 * self.cols + idx.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.data[idx.0 * self.cols + idx.1]
    }
}

impl Tensor3 {
    pub fn zeros(n0: usize, n1: usize, n2: usize) -> Self {
        Self { n0, n1, n2, data: vec![0.0; n0 * n1 * n2] }
    }
}

impl Tensor4 {
    pub fn zeros(n0: usize, n1: usize, n2: usize, n3: usize) -> Self {
        Self { n0, n1, n2, n3, data: vec![0.0; n0 * n1 * n2 * n3] }
    }
}

impl Index<(usize, usize, usize)> for Tensor3 {
    type Output = Scalar;
    fn index(&self, idx: (usize, usize, usize)) -> &Self::Output {
        &self.data[(idx.0 * self.n1 + idx.1) * self.n2 + idx.2]
    }
}

impl IndexMut<(usize, usize, usize)> for Tensor3 {
    fn index_mut(&mut self, idx: (usize, usize, usize)) -> &mut Self::Output {
        &mut self.data[(idx.0 * self.n1 + idx.1) * self.n2 + idx.2]
    }
}

impl Index<(usize, usize, usize, usize)> for Tensor4 {
    type Output = Scalar;
    fn index(&self, idx: (usize, usize, usize, usize)) -> &Self::Output {
        &self.data[((idx.0 * self.n1 + idx.1) * self.n2 + idx.2) * self.n3 + idx.3]
    }
}

impl IndexMut<(usize, usize, usize, usize)> for Tensor4 {
    fn index_mut(&mut self, idx: (usize, usize, usize, usize)) -> &mut Self::Output {
        &mut self.data[((idx.0 * self.n1 + idx.1) * self.n2 + idx.2) * self.n3 + idx.3]
    }
}

pub fn dot(a: &[Scalar], b: &[Scalar]) -> Scalar {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

pub fn norm2(a: &[Scalar]) -> Scalar {
    dot(a, a).sqrt()
}

pub fn vec_add(a: &[Scalar], b: &[Scalar]) -> Vec<Scalar> {
    a.iter().zip(b).map(|(x, y)| x + y).collect()
}

pub fn vec_sub(a: &[Scalar], b: &[Scalar]) -> Vec<Scalar> {
    a.iter().zip(b).map(|(x, y)| x - y).collect()
}

pub fn vec_scale(a: &[Scalar], s: Scalar) -> Vec<Scalar> {
    a.iter().map(|x| x * s).collect()
}

pub fn solve_2x2(a: &Matrix, b: &[Scalar]) -> Vec<Scalar> {
    let inv = a.inverse_2x2();
    inv.mul_vec(b)
}

pub fn tensor3_contract_left(t: &Tensor3, v: &[Scalar]) -> Matrix {
    assert_eq!(t.n0, v.len());
    let mut out = Matrix::zeros(t.n1, t.n2);
    for i in 0..t.n0 {
        for j in 0..t.n1 {
            for k in 0..t.n2 {
                out[(j, k)] += v[i] * t[(i, j, k)];
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cholesky_roundtrip_spd() {
        let a = Matrix::from_rows(&[
            &[4.0, 1.0],
            &[1.0, 3.0],
        ]);
        let l = a.cholesky();
        let lt = l.transpose();
        let recon = l.matmul(&lt);
        assert!(recon.approx_eq(&a, 1e-10));
    }

    #[test]
    fn solve_spd_matches_inverse() {
        let a = Matrix::from_rows(&[
            &[5.0, 2.0],
            &[2.0, 2.0],
        ]);
        let b = vec![1.0, -1.0];
        let x = a.solve_spd(&b);
        let inv = a.inverse_2x2();
        let x_ref = inv.mul_vec(&b);
        assert!((x[0] - x_ref[0]).abs() < 1e-10);
        assert!((x[1] - x_ref[1]).abs() < 1e-10);
    }

    #[test]
    fn logdet_spd_is_consistent() {
        let a = Matrix::from_rows(&[
            &[3.0, 1.0],
            &[1.0, 2.0],
        ]);
        let det = a.determinant_2x2();
        let logdet = a.logdet_spd();
        assert!((logdet - det.ln()).abs() < 1e-10);
    }

    #[test]
    fn tensor3_contract_left_matches_manual() {
        let mut t = Tensor3::zeros(2, 2, 2);
        t[(0, 0, 0)] = 1.0;
        t[(1, 0, 0)] = 2.0;
        t[(0, 1, 1)] = 3.0;
        t[(1, 1, 1)] = 4.0;
        let v = vec![2.0, 3.0];
        let m = tensor3_contract_left(&t, &v);
        assert!((m[(0, 0)] - (2.0 * 1.0 + 3.0 * 2.0)).abs() < 1e-12);
        assert!((m[(1, 1)] - (2.0 * 3.0 + 3.0 * 4.0)).abs() < 1e-12);
    }
}
