use gsae_core_types::{CodePoint, StatePoint, Tangent};

#[test]
fn construct_core_types() {
    let s = StatePoint::zeros(4);
    let z = CodePoint::zeros(3);
    let t = Tangent::zeros(3);
    assert_eq!(s.dim(), 4);
    assert_eq!(z.dim(), 3);
    assert_eq!(t.dim(), 3);
}

#[test]
fn deterministic_equality() {
    let a = StatePoint::zeros(5);
    let b = StatePoint::zeros(5);
    assert_eq!(a, b);
}

