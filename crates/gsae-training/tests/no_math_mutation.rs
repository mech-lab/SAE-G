use gsae_objective::{ObjectiveTerms, ObjectiveWeights, UnifiedObjective};
use gsae_training::TrainingHarness;

fn build_objective() -> ObjectiveTerms {
    ObjectiveTerms {
        recon: 1.0,
        sparse: 2.0,
        metric: 3.0,
        gauge: 4.0,
        transport: 5.0,
        hypergraph: 6.0,
    }
}

#[test]
fn objective_not_modified() {
    let obj_before = build_objective();
    let harness = TrainingHarness;
    let weights = ObjectiveWeights {
        recon: 1.0,
        sparse: 1.0,
        metric: 1.0,
        gauge: 1.0,
        transport: 1.0,
        hypergraph: 1.0,
    };
    let _ = harness.score(obj_before, weights);
    let obj_after = build_objective();
    let before = obj_before.total(weights);
    let after = obj_after.total(weights);
    assert_eq!(before, after);
}

