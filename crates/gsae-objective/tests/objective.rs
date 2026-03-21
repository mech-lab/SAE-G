use gsae_objective::{ObjectiveTerms, ObjectiveWeights, UnifiedObjective};

#[test]
fn objective_terms_separate() {
    let obj = ObjectiveTerms {
        recon: 1.0,
        sparse: 2.0,
        metric: 3.0,
        gauge: 4.0,
        transport: 5.0,
        hypergraph: 6.0,
    };
    let weights = ObjectiveWeights {
        recon: 1.0,
        sparse: 1.0,
        metric: 1.0,
        gauge: 1.0,
        transport: 1.0,
        hypergraph: 1.0,
    };
    let loss = obj.total(weights);
    assert!((loss - 21.0).abs() < 1e-12);
}

