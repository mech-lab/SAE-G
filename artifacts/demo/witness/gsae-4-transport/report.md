---
crate: gsae-4-transport
run_id: default
date_utc: 1970-01-01T00:00:00Z
git_commit: UNKNOWN
status: pass
formal_target:
  definitions: ["Definition 4"]
  propositions: []
  stages: ["Stage 3"]
assumptions:
  theorem_level: []
  numerical: []
  baseline_only: ["identity_baseline"]
forbidden_substitutions_checked:
  euclidean_pullback_shortcut: false
  affine_gauge_shortcut: false
  cosine_transport_shortcut: false
  additive_hypergraph_shortcut: false
baseline_artifacts:
  - identity_baseline
fixture_artifacts:
  - none
---

# Witness Report — gsae-4-transport

## Claim
Cross-layer evolution is transport, not flat matching.

## Formal Target
- Definitions: Definition 4
- Propositions: none
- Stages: Stage 3

## Transport Evidence
- Transported code error: 0
- Identity baseline error: 0.018000000000000016

## Metric Preservation
- Metric preservation error: 0.00000000000000015700924586837752

## Baselines
- identity_baseline

## Anti-Substitution Attestation
- Same-layer gauge reused as transport? no
- Cosine matching used outside baseline? no
- Affine shortcut used in witness path? no

## Pass / Fail Decision
- Pass conditions: transport improves over baseline and preserves metric
- Observed values: transported_error=0, baseline_error=0.018000000000000016
- Final decision: pass
