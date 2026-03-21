# GSAE Task-Level TODO Checklist

Actionable checklist derived from `DEVELOPMENT_GUIDE.md` and `BUILD_GUIDE.md`.
Order is mandatory: **Foundation → Witness → Scale-up**.

---

## 0. Global pre-flight (must be done before any phase work)

- [ ] Add/verify module docs in each foundational & witness crate: object, exact formula/definition, assumptions, forbidden substitutions, witness role.
- [ ] Add/verify explicit labels for any Euclidean/2D simplification paths using one of: `*_baseline`, `*_fixture`, `*_toy`, `*_sanity`, `*_analytic_harness`.
- [ ] Audit for silent substitutions in foundational/witness crates (search `J^T J`, `norm2`, `euclid`, `identity metric`, `cosine`); fix or label as baseline.
- [ ] Confirm dependency direction: foundational crates do not depend on witness/scale-up crates; witness crates do not depend on scale-up crates.

---

## Phase 0 — Foundational crates (must pass in order)

### 1) `gsae-core-types`
- [ ] Ensure core types exist: `Scalar`, `StatePoint`, `CodePoint`, `Tangent`, `ChartId`, `LayerId`, `HyperedgeId`.
- [ ] Add matrix/tensor wrappers or aliases used across crates.
- [ ] Confirm no training/runtime/benchmark logic exists here.
- [ ] Add tests: construction, debugability, deterministic equality where relevant.

### 2) `gsae-linalg`
- [ ] Implement SPD factorization and stable SPD solve/inverse.
- [ ] Implement determinant / logdet and tensor contraction helpers.
- [ ] Add tests: SPD inversion round-trip, determinant consistency, tensor indexing sanity.
- [ ] Ensure no geometry semantics or Euclidean defaults are embedded.

### 3) `gsae-state-geometry`
- [ ] Implement full `StateMetric` trait: `metric`, `metric_inv`, `metric_deriv`, `christoffel`, `curvature`.
- [ ] Add at least one non-Euclidean analytic metric implementation.
- [ ] If Euclidean/identity metric exists, name and label as baseline/fixture.
- [ ] Add tests: metric/inverse round-trip, Christoffel symmetry, nontrivial derivative/curvature for non-Euclidean case.

### 4) `gsae-autodiff`
- [ ] Provide exact Jacobian and Hessian access paths.
- [ ] If finite differences exist, mark as diagnostics only.
- [ ] Add tests: Jacobian on analytic nonlinear map, Hessian component check, optional FD vs exact (labeled).

### 5) `gsae-chart-core`
- [ ] Implement `SparseChart` trait exactly as specified (including `jacobian_encode`).
- [ ] Implement explicit sparsifier and validity/domain semantics.
- [ ] Add tests: local encode/decode reconstruction, sparsity support, validity on in/out-of-domain points, Jacobian/Hessian availability.
- [ ] Add inverse-consistency diagnostics or report.

### 6) `gsae-pullback-metric`
- [ ] Implement exact pullback: `g = J^T G_M J` (no `J^T J` fallback).
- [ ] Include derivatives of target metric in `dg` and Christoffel.
- [ ] Add tests: exact formula check on analytic chart+metric, SPD on valid region, symmetry, `dg` consistency.

### 7) `gsae-geodesics`
- [ ] Implement path energy/length, IVP, BVP, exp/log maps; optional parallel transport.
- [ ] Add tests: zero-velocity IVP, exp/log local inverse, length/energy sanity under subdivision, intrinsic distance differs from latent `L2` for non-Euclidean metric.

### 8) `gsae-objective`
- [ ] Implement separated terms: recon, sparse, metric, gauge, transport, hypergraph.
- [ ] Ensure no optimizer/runtime logic exists here.
- [ ] Add tests to validate term combination and separation (no training leakage).

### Foundation Gate (must pass before witness phase)
- [ ] Run per-crate tests in order (see `BUILD_GUIDE.md` section 7.1).
- [ ] Re-run contamination audit for forbidden substitutions in foundational crates.
- [ ] Verify analytic non-Euclidean fixture spans chart + state metric + pullback + geodesics.

---

## Phase 1 — Witness crates (must pass in order)

### 9) `gsae-1-chart`
- [ ] Implement a nonlinear sparse chart witness.
- [ ] Provide a witness report: validity, support size, reconstruction error, sample code.
- [ ] Add tests: nontrivial valid region, explicit sparsity (not dense low-magnitude), accurate local reconstruction.

### 10) `gsae-2-metric`
- [ ] Implement witness report with intrinsic geodesic distance and metric diagnostic.
- [ ] Compare against latent Euclidean baseline explicitly labeled.
- [ ] Add tests: geodesic distance differs meaningfully from Euclidean baseline on non-Euclidean fixture.

### 11) `gsae-3-gauge`
- [ ] Implement two overlapping same-layer charts and transition diffeomorphism.
- [ ] Add witness report: overlap validity, transition error, decode commutation, isometry residual.
- [ ] Add tests: nonempty overlap, invertible transition, metric pullback compatibility.

### 12) `gsae-4-transport`
- [ ] Implement adjacent-layer transport map (not gauge reuse).
- [ ] Add witness report: transported code error, baseline comparison, metric preservation residual.
- [ ] Add tests: transport beats baseline, metric preservation within tolerance.

### 13) `gsae-5-hypergraph`
- [ ] Implement nonlinear hyperedge gate in decoder.
- [ ] Add witness report: hyperedge id, mixed derivative, hyperedge delta, singleton ablation error.
- [ ] Add tests: hyperedge effect non-additive and non-linear.

### 14) `gsae-6-hyperpath`
- [ ] Implement transported section + downstream hyperedge deformation as a joint object.
- [ ] Add witness report comparing transport+hyperedge vs singleton explanations.
- [ ] Add tests: combined mechanism required; transport-only and hyperedge-only are insufficient.

### Witness Gate (must pass before scale-up phase)
- [ ] Run witness tests in order (see `BUILD_GUIDE.md` section 7.2).
- [ ] Verify all witness reports include required diagnostics.
- [ ] Re-audit baseline labeling for any Euclidean/2D harness used in witnesses.

---

## Phase 2 — Scale-up crates (only after witnesses are green)

### 15) `gsae-atlas-router`
- [ ] Implement chart routing without redefining chart ontology.
- [ ] Add tests: routing correctness without altering chart semantics.

### 16) `gsae-atlas-manager`
- [ ] Implement registry/overlap graph/cocycle bookkeeping.
- [ ] Add tests: dependency direction preserved (no witness dependency on manager).

### 17) `gsae-multilayer-runtime`
- [ ] Implement transport chains across multiple layers without redefining primitives.
- [ ] Add tests: transport chain consistency with witness transport semantics.

### 18) `gsae-training`
- [ ] Implement optimizer/runtime orchestration using `gsae-objective` only.
- [ ] Add tests: objective usage without redefining math.

### 19) `gsae-artifacts`
- [ ] Implement serialization preserving mathematical identities.
- [ ] Add tests: schema round-trip for witness reports and core objects.

### 20) `gsae-benchmarks`
- [ ] Implement benchmarks that consume, not define, witnesses.
- [ ] Add tests: benchmarks do not introduce core assumptions.

### 21) `gsae-cli`
- [ ] Implement CLI wrappers around existing APIs without new semantics.
- [ ] Add tests: CLI uses existing crate contracts.

### 22) `gsae-py`
- [ ] Implement Python bindings without redefining or simplifying math.
- [ ] Add tests: parity with Rust APIs and semantics.

---

## PR Review Checklist (aligns with Development Guide 9.x)

- [ ] Object identity: which definition/claim is implemented and in which crate tier.
- [ ] Assumptions: explicit and labeled (theorem-level vs solver vs baseline).
- [ ] Anti-substitution: no silent `J^T J`, Euclidean distance, affine gauge, cosine transport, additive hyperedge.
- [ ] Layer discipline: no downward dependency violations.
- [ ] Naming discipline: Euclidean/2D harnesses labeled with required suffixes.
- [ ] Test discipline: tests fail if implementation collapses to Euclidean/additive special cases.

---

## Definition of Done (minimal executable realization)

- [ ] All foundational crates pass gates in order.
- [ ] All witness crates pass gates in order.
- [ ] Every claim in the note has a non-vacuous executable witness.
- [ ] Euclidean/2D machinery is explicitly labeled and isolated.
- [ ] No scale-up crate is required for first-witness claim coverage.

---

## Gate Evidence & Artifacts (from `PHASE_GATE_CHECKS.md`)

### Evidence directory convention

```text
artifacts/gates/
  preflight/
  phase0-foundation/
  phase1-witness/
  phase2-scale-up/
```

### Minimum evidence contents per gate

- [ ] Command transcript (`*.txt`)
- [ ] Pass/fail summary (`*.md`)
- [ ] Crate list covered by the gate
- [ ] Explicit mention of any baseline/harness used
- [ ] Explicit statement that no unlabeled Euclidean/2D path remains

### Pre-flight gate (must pass before Phase 0)

- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings || true` (record if not yet possible)
- [ ] Substitution audit via `rg` for Euclidean/affine/cosine/additive shortcuts
- [ ] Verify module docs in all foundational/witness crates contain:
  - object implemented
  - exact formula/definition
  - assumptions
  - forbidden substitutions
  - witness role (if applicable)

---

## Standard Commands & Audits (from `PHASE_GATE_CHECKS.md`)

- [ ] Per-crate tests: `cargo test -p <crate> -- --nocapture`
- [ ] Workspace tests: `cargo test --workspace -- --nocapture`
- [ ] Dependency checks: `cargo tree -p <crate>`
- [ ] Substitution audit:
  - `rg -n "J\\^T J|norm2|euclid|euclidean|identity metric|cosine similarity|affine gauge|additive hyperedge" crates/`
  - Review hits and label or remove as required

---

## Witness Report Requirements (from `WITNESS_REPORT_TEMPLATES.md`)

### Required report paths

- [ ] `artifacts/witness/<crate-name>/<run-id>/report.md`
- [ ] `artifacts/witness/<crate-name>/<run-id>/report.json`

### Required front matter (YAML)

- [ ] `crate`, `run_id`, `date_utc`, `git_commit`, `status`
- [ ] `formal_target` (definitions, propositions, stages)
- [ ] `assumptions` (theorem_level, numerical, baseline_only)
- [ ] `forbidden_substitutions_checked` flags
- [ ] `baseline_artifacts` and `fixture_artifacts`

### Required anti-substitution attestation section

- [ ] Explicitly state whether `J^T J` appears in the witness path
- [ ] Explicitly state whether latent Euclidean `L2` appears outside labeled baselines
- [ ] Explicitly state whether gauge transition is affine rather than diffeomorphic
- [ ] Explicitly state whether transport relies on cosine beyond baseline
- [ ] Explicitly state whether hyperedge is reducible to additive singletons

### Required JSON keys

- [ ] `crate`, `run_id`, `status`, `formal_target`, `assumptions`
- [ ] `metrics`, `thresholds`, `anti_substitution`, `artifacts`, `pass_conditions`, `failure_reasons`

### Required comparisons (baseline vs. claim)

- [ ] Chart: compare against global/flat or dense/non-sparse failure mode where relevant
- [ ] Metric: compare against labeled latent Euclidean baseline
- [ ] Gauge: compare against naive matching/no-transition baseline
- [ ] Transport: compare against cosine/flat matching baseline
- [ ] Hypergraph: compare against additive singleton or matched linear baseline
- [ ] Hyperpath: compare against transport-only and hyperedge-only explanations

---

## Coverage Matrix (from `WITNESS_REPORT_TEMPLATES.md`)

- [ ] Emit `witness-coverage-matrix.md` mapping claims to witness crates.
- [ ] Fail the phase if any claim row has no `yes`.

---

## Witness Report Integration Requirement

Each witness crate must export:

- [ ] `run_witness(...)`
- [ ] `write_report(...)` producing both Markdown and JSON
- [ ] `validate_report(...)` that fails if required sections/keys are missing
