---
crate: gsae-3-gauge
run_id: default
date_utc: 1970-01-01T00:00:00Z
git_commit: UNKNOWN
status: pass
formal_target:
  definitions: ["Definition 3"]
  propositions: ["Proposition 1"]
  stages: ["Stage 2"]
assumptions:
  theorem_level: []
  numerical: []
  baseline_only: ["naive_matching_baseline"]
forbidden_substitutions_checked:
  euclidean_pullback_shortcut: false
  affine_gauge_shortcut: false
  cosine_transport_shortcut: false
  additive_hypergraph_shortcut: false
baseline_artifacts:
  - naive_matching_baseline
fixture_artifacts:
  - none
---

# Witness Report — gsae-3-gauge

## Claim
Same-layer overlap realizes gauge transition.

## Formal Target
- Definitions: Definition 3
- Propositions: Proposition 1
- Stages: Stage 2

## Mathematical Object
- Charts: alpha, beta
- Transition diffeomorphism

## Assumptions
### Theorem-level
- none

### Numerical
- none

### Baseline-only
- naive_matching_baseline

## Overlap Evidence
- Transition error: 0
- Decode commutation error: 0

## Metric Compatibility
- Isometry error: 0.0000000000000004611102534756203

## Baselines
- naive_matching_baseline error: 0.021874999999999978

## Anti-Substitution Attestation
- Transition affine-only? no
- Overlap empty or vacuous? no
- Same-layer claim accidentally implemented cross-layer? no
- Euclidean metric compatibility used outside baseline? no

## Pass / Fail Decision
- Pass conditions: low transition and isometry errors
- Observed values: transition=0, isometry=0.0000000000000004611102534756203
- Final decision: pass
