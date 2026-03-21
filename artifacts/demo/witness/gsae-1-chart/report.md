---
crate: gsae-1-chart
run_id: default
date_utc: 1970-01-01T00:00:00Z
git_commit: UNKNOWN
status: pass
formal_target:
  definitions: ["Definition 2"]
  propositions: []
  stages: []
assumptions:
  theorem_level: []
  numerical: []
  baseline_only: []
forbidden_substitutions_checked:
  euclidean_pullback_shortcut: false
  affine_gauge_shortcut: false
  cosine_transport_shortcut: false
  additive_hypergraph_shortcut: false
baseline_artifacts:
  - none
fixture_artifacts:
  - none
---

# Witness Report — gsae-1-chart

## Claim
Local sparse chart replaces global flat basis.

## Formal Target
- Definitions: Definition 2
- Propositions: none
- Stages: none

## Mathematical Object
- Chart: analytic sparse chart
- Domain semantics: validity function
- Sparsifier: support threshold

## Assumptions
### Theorem-level
- none

### Numerical
- none

### Baseline-only
- none

## Validity Evidence
- Valid: true
- Reconstruction error: 0

## Sparsity Evidence
- Support size: 1

## Local Coordinate Evidence
- Inverse-consistency error: 0

## Baselines
- none

## Anti-Substitution Attestation
- Pullback shortcut used? no
- Euclidean latent norm used outside labeled baselines? no
- Dense local autoencoder substituted for sparse chart? no
- Global basis assumption present? no

## Pass / Fail Decision
- Pass conditions: validity true, sparse support, low reconstruction error
- Observed values: valid=true, support_size=1, recon_error=0
- Final decision: pass

## Artifact Paths
- report.json: report.json
- raw metrics: none
- plots: none
- fixtures: none
