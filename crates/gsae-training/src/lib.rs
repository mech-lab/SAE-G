use gsae_objective::{ObjectiveTerms, ObjectiveWeights, UnifiedObjective};

#[derive(Clone, Debug)]
pub struct TrainingHarness;

impl TrainingHarness {
    pub fn score(&self, terms: ObjectiveTerms, weights: ObjectiveWeights) -> f64 {
        terms.total(weights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harness_scores_objective() {
        let harness = TrainingHarness;
        let terms = ObjectiveTerms::default();
        let weights = ObjectiveWeights {
            recon: 1.0,
            sparse: 1.0,
            metric: 1.0,
            gauge: 1.0,
            transport: 1.0,
            hypergraph: 1.0,
        };
        let score = harness.score(terms, weights);
        assert_eq!(score, 0.0);
    }
}

