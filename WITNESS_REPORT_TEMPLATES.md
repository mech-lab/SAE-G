# WITNESS_REPORT_TEMPLATES.md

Canonical report templates for the six witness crates:

- `gsae-1-chart`
- `gsae-2-metric`
- `gsae-3-gauge`
- `gsae-4-transport`
- `gsae-5-hypergraph`
- `gsae-6-hyperpath`

These templates are not presentation suggestions. They are phase-gate artifacts.
A witness is not considered complete unless its report is emitted in the required structure, with all required fields filled, assumptions declared, baselines explicitly labeled, and anti-substitution attestation included.

---

# 1. Global witness reporting rules

## 1.1 Purpose

Each report must make one mathematical claim executable and auditable.

A report is valid only if it answers all of the following:

1. Which formal claim is being witnessed?
2. Which mathematical object realizes that claim?
3. Which assumptions are used?
4. Which baseline is used, if any?
5. What exact evidence shows the claim is non-vacuously instantiated?
6. What exact evidence shows the implementation did not silently collapse to a Euclidean / affine / cosine / additive special case?

## 1.2 Required naming

All witness reports must be written to a deterministic path of the form:

```text
artifacts/witness/<crate-name>/<run-id>/report.md
```

Required companion machine-readable artifact:

```text
artifacts/witness/<crate-name>/<run-id>/report.json
```

Where `<crate-name>` is one of:

- `gsae-1-chart`
- `gsae-2-metric`
- `gsae-3-gauge`
- `gsae-4-transport`
- `gsae-5-hypergraph`
- `gsae-6-hyperpath`

## 1.3 Required front matter

Every report must begin with this YAML front matter:

```yaml
crate: gsae-X-...
run_id: <deterministic-or-recorded-id>
date_utc: <ISO-8601>
git_commit: <full-sha>
status: pass | fail | partial
formal_target:
  definitions: [<Definition IDs>]
  propositions: [<Proposition IDs>]
  stages: [<Stage IDs>]
assumptions:
  theorem_level:
    - <explicit theorem-level assumptions>
  numerical:
    - <solver tolerances / discretization assumptions>
  baseline_only:
    - <anything only true for baseline comparisons>
forbidden_substitutions_checked:
  euclidean_pullback_shortcut: true | false
  affine_gauge_shortcut: true | false
  cosine_transport_shortcut: true | false
  additive_hypergraph_shortcut: true | false
baseline_artifacts:
  - <path or none>
fixture_artifacts:
  - <path or none>
```

## 1.4 Required baseline labeling

Any Euclidean, 2D, affine, cosine, or additive comparison path must be labeled using one of these suffixes:

- `_baseline`
- `_fixture`
- `_toy`
- `_sanity`
- `_analytic_harness`

If a baseline exists but is not labeled this way, the report is invalid.

## 1.5 Required anti-substitution attestation block

Every report must contain this exact section:

## Anti-Substitution Attestation

and explicitly state:

- whether `J^T J` appears anywhere in the witness path
- whether latent Euclidean `L2` appears anywhere other than labeled baseline comparison
- whether any same-layer overlap map is affine rather than diffeomorphic
- whether any cross-layer transport comparison relies on cosine as more than a baseline
- whether any hyperedge witness is reducible to additive singleton terms

## 1.6 Required comparison rule

A witness must compare against the collapsed special case when that collapsed case is one of the claims being displaced:

- chart → compare against global/flat or dense/non-sparse failure mode where relevant
- metric → compare against latent Euclidean baseline
- gauge → compare against naive matching / no-transition handling if relevant
- transport → compare against cosine / flat matching baseline
- hypergraph → compare against additive singleton or matched linear baseline
- hyperpath → compare against transport-only and hyperedge-only explanations

## 1.7 Required machine-readable schema keys

All JSON reports must include at least:

```json
{
  "crate": "gsae-X-...",
  "run_id": "...",
  "status": "pass|fail|partial",
  "formal_target": {
    "definitions": [],
    "propositions": [],
    "stages": []
  },
  "assumptions": {
    "theorem_level": [],
    "numerical": [],
    "baseline_only": []
  },
  "metrics": {},
  "thresholds": {},
  "anti_substitution": {},
  "artifacts": {},
  "pass_conditions": {},
  "failure_reasons": []
}
```

---

# 2. Template: gsae-1-chart

## 2.1 Formal target

This witnesses:

- local sparse chart replaces global flat basis
- existence of a nontrivial valid local coordinate patch

## 2.2 Required report structure

```markdown
# Witness Report — gsae-1-chart

## Claim
<state the exact chart-locality claim>

## Formal Target
- Definitions: <...>
- Propositions: <... or none>
- Stages: <...>

## Mathematical Object
- Chart: (phi_alpha, E_alpha, D_alpha)
- Domain semantics: U_alpha and/or validity function
- Sparsifier: <type>

## Assumptions
### Theorem-level
- ...

### Numerical
- ...

### Baseline-only
- ...

## Dataset / Layer / Slice
- Model:
- Layer:
- Token / slice definition:
- Sample count:

## Validity Evidence
- Valid region size:
- Validity threshold:
- In-domain reconstruction summary:
- Out-of-domain rejection summary:

## Sparsity Evidence
- Mean support size:
- Median support size:
- Max support size:
- Fraction exactly zero:
- Evidence that representation is not merely dense low-magnitude:

## Local Coordinate Evidence
- Reconstruction error on valid region:
- Inverse-consistency diagnostic:
- Jacobian availability:
- Hessian availability:

## Baselines
- <explicitly labeled baseline paths>

## Anti-Substitution Attestation
- Pullback shortcut used? yes/no
- Euclidean latent norm used outside labeled baselines? yes/no
- Dense local autoencoder substituted for sparse chart? yes/no
- Global basis assumption present? yes/no

## Pass / Fail Decision
- Pass conditions:
- Observed values:
- Final decision:

## Artifact Paths
- report.json:
- raw metrics:
- plots:
- fixtures:
```

## 2.3 Required JSON metrics

```json
{
  "metrics": {
    "valid_region_fraction": 0.0,
    "reconstruction_error_valid_mean": 0.0,
    "reconstruction_error_invalid_mean": 0.0,
    "support_size_mean": 0.0,
    "support_size_median": 0.0,
    "support_size_max": 0.0,
    "zero_fraction": 0.0,
    "inverse_consistency_error": 0.0
  },
  "thresholds": {
    "valid_region_fraction_min": 0.0,
    "support_size_max_allowed": 0.0,
    "reconstruction_error_valid_max": 0.0,
    "inverse_consistency_error_max": 0.0
  }
}
```

## 2.4 Required pass conditions

The report must demonstrate all of:

1. nontrivial valid region
2. explicit sparsity, not dense low-magnitude coding
3. accurate local reconstruction on valid points
4. chart-local inverse consistency or explicit diagnostic thereof

---

# 3. Template: gsae-2-metric

## 3.1 Formal target

This witnesses:

- intrinsic chart geometry replaces latent Euclidean geometry
- pointwise pullback metric and geodesic distance are non-vacuous

## 3.2 Required report structure

```markdown
# Witness Report — gsae-2-metric

## Claim
<state the exact intrinsic-geometry claim>

## Formal Target
- Definitions:
- Propositions:
- Stages:

## Mathematical Object
- Chart:
- State metric G_M:
- Pullback metric g = J^T G_M J
- Geodesic solver:

## Assumptions
### Theorem-level
- ...

### Numerical
- ...

### Baseline-only
- ...

## Metric Diagnostics
- Symmetry error:
- SPD check on valid region:
- Condition number summary:
- dg consistency:
- Christoffel sanity:

## Geodesic Diagnostics
- IVP zero-velocity behavior:
- BVP convergence summary:
- exp/log local inverse:
- path subdivision sanity:

## Intrinsic vs Euclidean Comparison
- Baseline path: <explicitly labeled latent_l2_baseline>
- Comparison target:
- Metric difference summary:
- Correlation / ranking / separation evidence:

## Anti-Substitution Attestation
- Pullback implemented as J^T G_M J? yes/no
- Any J^T J special case in witness path? yes/no
- Any latent Euclidean norm outside labeled baseline? yes/no
- Any constant Gram substitution for pointwise field? yes/no

## Pass / Fail Decision
- Pass conditions:
- Observed values:
- Final decision:
```

## 3.3 Required JSON metrics

```json
{
  "metrics": {
    "metric_symmetry_error": 0.0,
    "spd_failure_fraction": 0.0,
    "condition_number_mean": 0.0,
    "dg_consistency_error": 0.0,
    "ivp_zero_velocity_error": 0.0,
    "exp_log_local_inverse_error": 0.0,
    "distance_vs_latent_l2_difference": 0.0,
    "intrinsic_signal_gain_over_baseline": 0.0
  },
  "thresholds": {
    "metric_symmetry_error_max": 0.0,
    "spd_failure_fraction_max": 0.0,
    "ivp_zero_velocity_error_max": 0.0,
    "exp_log_local_inverse_error_max": 0.0,
    "intrinsic_signal_gain_over_baseline_min": 0.0
  }
}
```

## 3.4 Required pass conditions

The report must demonstrate all of:

1. pointwise pullback field actually implemented
2. geodesic machinery numerically operational
3. intrinsic distance behavior differs meaningfully from latent Euclidean baseline
4. no silent collapse to flat local metric in the witness path

---

# 4. Template: gsae-3-gauge

## 4.1 Formal target

This witnesses:

- basis instability is gauge variance
- identity is recovered through chart transition diffeomorphism on overlap

## 4.2 Required report structure

```markdown
# Witness Report — gsae-3-gauge

## Claim
<state the exact gauge claim>

## Formal Target
- Definitions:
- Propositions:
- Stages:

## Mathematical Object
- Chart alpha:
- Chart beta:
- Overlap region:
- Transition diffeomorphism tau_{alpha,beta}:

## Assumptions
### Theorem-level
- ...

### Numerical
- ...

### Baseline-only
- ...

## Overlap Evidence
- Overlap size:
- Overlap validity criteria:
- Overlap support diagnostics:

## Transition Evidence
- Forward transition error:
- Inverse transition error:
- Jacobian invertibility summary:
- Decode commutation residual:

## Metric Compatibility
- Pullback compatibility residual: ||tau^* g_beta - g_alpha||
- Local isometry residual:
- Optional cocycle diagnostics if available:

## Baselines
- <explicitly labeled naive_matching_baseline>

## Anti-Substitution Attestation
- Transition affine-only? yes/no
- Overlap empty or vacuous? yes/no
- Same-layer claim accidentally implemented cross-layer? yes/no
- Euclidean metric compatibility used outside baseline? yes/no

## Pass / Fail Decision
- Pass conditions:
- Observed values:
- Final decision:
```

## 4.3 Required JSON metrics

```json
{
  "metrics": {
    "overlap_fraction": 0.0,
    "transition_forward_error": 0.0,
    "transition_inverse_error": 0.0,
    "decode_commutation_error": 0.0,
    "metric_pullback_compatibility_error": 0.0,
    "local_isometry_residual": 0.0
  },
  "thresholds": {
    "overlap_fraction_min": 0.0,
    "transition_forward_error_max": 0.0,
    "transition_inverse_error_max": 0.0,
    "decode_commutation_error_max": 0.0,
    "metric_pullback_compatibility_error_max": 0.0
  }
}
```

## 4.4 Required pass conditions

The report must demonstrate all of:

1. nonempty same-layer overlap
2. invertible transition diffeomorphism on overlap
3. decode commutation on overlap
4. metric pullback compatibility on overlap

---

# 5. Template: gsae-4-transport

## 5.1 Formal target

This witnesses:

- cross-layer evolution is transport, not same-layer gauge or flat matching

## 5.2 Required report structure

```markdown
# Witness Report — gsae-4-transport

## Claim
<state the exact transport claim>

## Formal Target
- Definitions:
- Propositions:
- Stages:

## Mathematical Object
- Source chart:
- Target chart:
- Adjacent layer pair:
- Transport map T_{l,alpha->beta}:
- Section or tracked feature object:

## Assumptions
### Theorem-level
- ...

### Numerical
- ...

### Baseline-only
- ...

## Transport Evidence
- Source/target validity:
- Transported code error:
- Section tracking error:
- Decode transport consistency:

## Metric Preservation
- Transport metric residual:
- Parallel transport / connection diagnostics if applicable:

## Baselines
- <explicitly labeled cosine_transport_baseline>
- <optionally labeled affine_transport_baseline>

## Anti-Substitution Attestation
- Same-layer gauge reused as cross-layer transport? yes/no
- Cosine matching used outside baseline? yes/no
- Affine flat CLT shortcut used in witness path? yes/no

## Pass / Fail Decision
- Pass conditions:
- Observed values:
- Final decision:
```

## 5.3 Required JSON metrics

```json
{
  "metrics": {
    "transport_code_error": 0.0,
    "section_tracking_error": 0.0,
    "decode_transport_consistency_error": 0.0,
    "metric_preservation_residual": 0.0,
    "improvement_over_cosine_baseline": 0.0
  },
  "thresholds": {
    "transport_code_error_max": 0.0,
    "metric_preservation_residual_max": 0.0,
    "improvement_over_cosine_baseline_min": 0.0
  }
}
```

## 5.4 Required pass conditions

The report must demonstrate all of:

1. actual adjacent-layer transport object
2. transport beats or materially differs from cosine/flat matching baseline
3. metric-preservation residual is explicitly measured
4. transport is not merely same-layer gauge renamed

---

# 6. Template: gsae-5-hypergraph

## 6.1 Formal target

This witnesses:

- semantic composition is intrinsically higher-order, not merely additive

## 6.2 Required report structure

```markdown
# Witness Report — gsae-5-hypergraph

## Claim
<state the exact hypergraph claim>

## Formal Target
- Definitions:
- Propositions:
- Stages:

## Mathematical Object
- Hyperedge id:
- Participating coordinates:
- Nonlinear gate rho_e:
- Decoder location:

## Assumptions
### Theorem-level
- ...

### Numerical
- ...

### Baseline-only
- ...

## Hyperedge Evidence
- Mixed derivative or equivalent nonlinearity diagnostic:
- Hyperedge activation statistics:
- Hyperedge ablation effect:
- Singleton / additive comparison effect:

## Baselines
- <explicitly labeled additive_singleton_baseline>
- <optionally labeled matched_linear_combination_baseline>

## Anti-Substitution Attestation
- Hyperedge gate linear? yes/no
- Witness reducible to additive singleton terms? yes/no
- Hypergraph represented only as metadata without decoder effect? yes/no

## Pass / Fail Decision
- Pass conditions:
- Observed values:
- Final decision:
```

## 6.3 Required JSON metrics

```json
{
  "metrics": {
    "mixed_derivative_signal": 0.0,
    "hyperedge_activation_mean": 0.0,
    "hyperedge_ablation_delta": 0.0,
    "singleton_baseline_delta": 0.0,
    "nonadditive_gain_over_baseline": 0.0
  },
  "thresholds": {
    "mixed_derivative_signal_min": 0.0,
    "nonadditive_gain_over_baseline_min": 0.0
  }
}
```

## 6.4 Required pass conditions

The report must demonstrate all of:

1. at least one real decoder hyperedge with nonlinear gate
2. measurable non-additive effect
3. hyperedge witness outperforms additive singleton explanation
4. hypergraph is not merely a data structure detached from decoder semantics

---

# 7. Template: gsae-6-hyperpath

## 7.1 Formal target

This witnesses:

- mechanism is a parallel-transported hyperpath
- Proposition 2 requires both transport across layers and higher-order deformation within the decoder

## 7.2 Required report structure

```markdown
# Witness Report — gsae-6-hyperpath

## Claim
<state the exact hyperpath claim>

## Formal Target
- Definitions:
- Propositions:
- Stages:

## Mathematical Object
- Source section:
- Transport map:
- Target-layer hyperedge:
- Combined mechanism object:

## Assumptions
### Theorem-level
- ...

### Numerical
- ...

### Baseline-only
- ...

## Combined Evidence
- Transport-only explanation error:
- Hyperedge-only explanation error:
- Combined transport+hyperedge explanation error:
- Downstream causal delta:
- Mechanism alignment summary:

## Baselines
- <explicitly labeled transport_only_baseline>
- <explicitly labeled hyperedge_only_baseline>
- <optionally labeled singleton_only_baseline>

## Anti-Substitution Attestation
- Combined witness decomposes into transport-only? yes/no
- Combined witness decomposes into hyperedge-only? yes/no
- Downstream mechanism explained without joint object? yes/no

## Pass / Fail Decision
- Pass conditions:
- Observed values:
- Final decision:
```

## 7.3 Required JSON metrics

```json
{
  "metrics": {
    "transport_only_error": 0.0,
    "hyperedge_only_error": 0.0,
    "combined_hyperpath_error": 0.0,
    "combined_gain_over_transport_only": 0.0,
    "combined_gain_over_hyperedge_only": 0.0,
    "downstream_causal_delta": 0.0
  },
  "thresholds": {
    "combined_gain_over_transport_only_min": 0.0,
    "combined_gain_over_hyperedge_only_min": 0.0
  }
}
```

## 7.4 Required pass conditions

The report must demonstrate all of:

1. a real transported section
2. a real downstream hyperedge deformation
3. the joint object explains behavior better than either component alone
4. Proposition 2 is not reduced to isolated atom explanations

---

# 8. Coverage matrix template

Every phase-complete workspace must emit a cross-witness coverage matrix:

```markdown
# Coverage Matrix

| Claim / Definition / Proposition / Stage | gsae-1-chart | gsae-2-metric | gsae-3-gauge | gsae-4-transport | gsae-5-hypergraph | gsae-6-hyperpath |
|---|---:|---:|---:|---:|---:|---:|
| Definition 2 | yes | yes | no | no | no | no |
| Definition 3 | no | no | yes | no | no | no |
| Definition 4 | no | no | no | yes | no | yes |
| Definition 5 | no | no | no | no | yes | yes |
| Proposition 1 | no | no | yes | no | no | no |
| Proposition 2 | no | no | no | no | no | yes |
| Stage 1 | no | yes | no | no | no | no |
| Stage 2 | no | no | yes | no | no | no |
| Stage 3 | no | no | no | yes | no | no |
| Stage 4 | no | no | no | no | yes | no |
```

If any row has no `yes`, the workspace is not complete.

---

# 9. Failure conditions common to all witness reports

A witness report is automatically invalid if any of the following hold:

1. The formal target is omitted.
2. Assumptions are implicit rather than declared.
3. A Euclidean / affine / cosine / additive shortcut appears outside an explicitly labeled baseline.
4. A baseline exists but is not named with an allowed suffix.
5. Machine-readable JSON is missing.
6. Reported metrics lack thresholds.
7. The anti-substitution attestation section is missing.
8. The final pass/fail decision is not tied to explicit numerical evidence.

---

# 10. Minimal repo integration requirement

Each witness crate must export:

- a deterministic `run_witness(...)` entry point
- a `write_report(...)` function producing both Markdown and JSON
- a `validate_report(...)` function that fails if required sections or keys are missing

Suggested signature:

```rust
pub fn run_witness(cfg: &WitnessConfig) -> WitnessRun;
pub fn write_report(run: &WitnessRun, out_dir: &std::path::Path) -> std::io::Result<()>;
pub fn validate_report(report_path: &std::path::Path) -> anyhow::Result<()>;
```

This keeps witness emission as part of the executable contract rather than an afterthought.
