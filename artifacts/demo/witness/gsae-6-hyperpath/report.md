---
crate: gsae-6-hyperpath
run_id: default
date_utc: 1970-01-01T00:00:00Z
git_commit: UNKNOWN
status: pass
formal_target:
  definitions: ["Definition 4","Definition 5"]
  propositions: ["Proposition 2"]
  stages: []
assumptions:
  theorem_level: []
  numerical: []
  baseline_only: ["transport_only_baseline","hyperedge_only_baseline"]
forbidden_substitutions_checked:
  euclidean_pullback_shortcut: false
  affine_gauge_shortcut: false
  cosine_transport_shortcut: false
  additive_hypergraph_shortcut: false
baseline_artifacts:
  - transport_only_baseline
  - hyperedge_only_baseline
fixture_artifacts:
  - none
---

# Witness Report — gsae-6-hyperpath

## Claim
Mechanism requires transport plus downstream hyperedge deformation.

## Formal Target
- Definitions: Definition 4, Definition 5
- Propositions: Proposition 2
- Stages: none

## Combined Evidence
- Transport error: 0
- Hyperedge delta after transport: 0.504
- Best singleton ablation error: 0.1280000000000001
- Transport-only error: 0
- Hyperedge-only error: 0.19968000000000008

## Baselines
- transport_only_baseline
- hyperedge_only_baseline

## Anti-Substitution Attestation
- Combined witness decomposes into transport-only? no
- Combined witness decomposes into hyperedge-only? no
- Downstream mechanism explained without joint object? no

## Pass / Fail Decision
- Pass conditions: combined mechanism required
- Observed values: transport_error=0, hyperedge_delta=0.504
- Final decision: pass
