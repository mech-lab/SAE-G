pub type Scalar = f64;

#[derive(Clone, Debug, PartialEq)]
pub struct StatePoint {
    pub data: Vec<Scalar>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodePoint {
    pub data: Vec<Scalar>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tangent {
    pub data: Vec<Scalar>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChartId(pub &'static str);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LayerId(pub usize);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HyperedgeId(pub &'static str);

#[derive(Clone, Debug, PartialEq)]
pub struct Section {
    pub chart: ChartId,
    pub layer: LayerId,
    pub code: CodePoint,
}

pub type DenseMatrix = Vec<Vec<Scalar>>;
pub type DenseTensor3 = Vec<Vec<Vec<Scalar>>>;
pub type DenseTensor4 = Vec<Vec<Vec<Vec<Scalar>>>>;

impl StatePoint {
    pub fn new(data: Vec<Scalar>) -> Self { Self { data } }
    pub fn dim(&self) -> usize { self.data.len() }
    pub fn zeros(dim: usize) -> Self { Self { data: vec![0.0; dim] } }
}

impl CodePoint {
    pub fn new(data: Vec<Scalar>) -> Self { Self { data } }
    pub fn dim(&self) -> usize { self.data.len() }
    pub fn zeros(dim: usize) -> Self { Self { data: vec![0.0; dim] } }
}

impl Tangent {
    pub fn new(data: Vec<Scalar>) -> Self { Self { data } }
    pub fn dim(&self) -> usize { self.data.len() }
    pub fn zeros(dim: usize) -> Self { Self { data: vec![0.0; dim] } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_types_construct_and_debug() {
        let x = StatePoint::new(vec![0.1, 0.2]);
        let z = CodePoint::new(vec![0.0, 1.0]);
        let t = Tangent::new(vec![1.0, -1.0]);
        let _ = format!("{:?}{:?}{:?}", x, z, t);
        assert_eq!(x.dim(), 2);
        assert_eq!(z.dim(), 2);
        assert_eq!(t.dim(), 2);
    }
}
