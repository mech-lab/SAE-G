---
crate: gsae-5-hypergraph
run_id: default
date_utc: 1970-01-01T00:00:00Z
git_commit: UNKNOWN
status: pass
formal_target:
  definitions: ["Definition 5"]
  propositions: []
  stages: ["Stage 4"]
assumptions:
  theorem_level: []
  numerical: []
  baseline_only: ["additive_singleton_baseline"]
forbidden_substitutions_checked:
  euclidean_pullback_shortcut: false
  affine_gauge_shortcut: false
  cosine_transport_shortcut: false
  additive_hypergraph_shortcut: false
baseline_artifacts:
  - additive_singleton_baseline
fixture_artifacts:
  - none
---

# Witness Report — gsae-5-hypergraph

## Claim
Nonlinear hyperedge interaction is not reducible to additive singletons.

## Formal Target
- Definitions: Definition 5
- Propositions: none
- Stages: Stage 4

## Hyperedge Evidence
- Hyperedge id: e_{uv}
- Mixed derivative: 0.7
- Hyperedge delta: 0.6999999999999997
- Singleton ablation error: 0.6999999999999997

## Baselines
- additive_singleton_baseline

## Anti-Substitution Attestation
- Hyperedge gate linear? no
- Witness reducible to additive singleton terms? no
- Hypergraph represented only as metadata? no

## Pass / Fail Decision
- Pass conditions: nonzero mixed derivative and nonadditive delta
- Observed values: mixed_derivative=0.7, delta=0.6999999999999997
- Final decision: pass
