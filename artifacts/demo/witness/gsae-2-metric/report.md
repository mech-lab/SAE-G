---
crate: gsae-2-metric
run_id: default
date_utc: 1970-01-01T00:00:00Z
git_commit: UNKNOWN
status: pass
formal_target:
  definitions: ["Definition 2"]
  propositions: []
  stages: ["Stage 1"]
assumptions:
  theorem_level: []
  numerical: []
  baseline_only: ["latent_l2_baseline"]
forbidden_substitutions_checked:
  euclidean_pullback_shortcut: false
  affine_gauge_shortcut: false
  cosine_transport_shortcut: false
  additive_hypergraph_shortcut: false
baseline_artifacts:
  - latent_l2_baseline
fixture_artifacts:
  - none
---

# Witness Report — gsae-2-metric

## Claim
Intrinsic chart geometry replaces latent Euclidean distance.

## Formal Target
- Definitions: Definition 2
- Propositions: none
- Stages: Stage 1

## Mathematical Object
- Chart: analytic sparse chart
- State metric G_M
- Pullback metric g = J^T G_M J
- Geodesic solver

## Assumptions
### Theorem-level
- none

### Numerical
- geodesic step size and max iterations

### Baseline-only
- latent_l2_baseline

## Metric Diagnostics
- Determinant at start: 1

## Geodesic Diagnostics
- Geodesic distance: 0.5359574711951903
- Euclidean distance baseline: 0.5

## Intrinsic vs Euclidean Comparison
- Baseline: latent_l2_baseline
- Difference: 0.03595747119519033

## Anti-Substitution Attestation
- Pullback implemented as J^T G_M J? yes
- Any J^T J special case in witness path? no
- Any latent Euclidean norm outside labeled baseline? no
- Any constant Gram substitution? no

## Pass / Fail Decision
- Pass conditions: geodesic distance differs from baseline
- Observed values: diff=0.03595747119519033
- Final decision: pass
