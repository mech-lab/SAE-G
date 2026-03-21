use gsae_core_types::Scalar;

#[derive(Clone, Copy, Debug)]
pub struct ObjectiveWeights {
    pub recon: Scalar,
    pub sparse: Scalar,
    pub metric: Scalar,
    pub gauge: Scalar,
    pub transport: Scalar,
    pub hypergraph: Scalar,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ObjectiveTerms {
    pub recon: Scalar,
    pub sparse: Scalar,
    pub metric: Scalar,
    pub gauge: Scalar,
    pub transport: Scalar,
    pub hypergraph: Scalar,
}

pub trait UnifiedObjective {
    fn total(&self, weights: ObjectiveWeights) -> Scalar;
}

impl UnifiedObjective for ObjectiveTerms {
    fn total(&self, w: ObjectiveWeights) -> Scalar {
        w.recon * self.recon
            + w.sparse * self.sparse
            + w.metric * self.metric
            + w.gauge * self.gauge
            + w.transport * self.transport
            + w.hypergraph * self.hypergraph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn objective_terms_separate_and_sum() {
        let terms = ObjectiveTerms {
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
        assert!((terms.total(weights) - 21.0).abs() < 1e-12);
    }
}
