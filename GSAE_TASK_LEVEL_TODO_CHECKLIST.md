# GSAE Task-Level TODO Checklist

Actionable checklist derived from `DEVELOPMENT_GUIDE.md` and `BUILD_GUIDE.md`.

**Execution order is mandatory:** Foundation → Witness → Scale-up.

---

## 0. Global pre-flight (must be done before any phase work)

- [ ] Add or verify module docs in each foundational and witness crate covering: object identity, exact formula/definition, assumptions, forbidden substitutions, witness role.
- [ ] Add or verify explicit labels for any Euclidean/2D simplification paths using one of: `*_baseline`, `*_fixture`, `*_toy`, `*_sanity`, `*_analytic_harness`.
- [ ] Audit for silent substitutions in foundational and witness crates. Search for: `J^T J`, `norm2`, `euclid`, `identity metric`, `cosine`. Fix or relabel as baseline.
- [ ] Confirm dependency direction: foundational crates do not depend on witness or scale-up crates; witness crates do not depend on scale-up crates.

---

## Phase 0 — Foundational crates (must pass in order)

### 1) `gsae-core-types`

- [ ] Ensure core types exist: `Scalar`, `StatePoint`, `CodePoint`, `Tangent`, `ChartId`, `LayerId`, `HyperedgeId`.
- [ ] Add matrix and tensor wrappers or aliases used across crates.
- [ ] Confirm no training, runtime, CLI, or benchmark logic exists here.
- [ ] Add tests: construction, debugability, deterministic equality where relevant.

### 2) `gsae-linalg`

- [ ] Implement SPD factorization and stable SPD solve/inverse.
- [ ] Implement determinant and `logdet` plus tensor contraction helpers.
- [ ] Add tests: SPD inversion round-trip, determinant consistency, tensor indexing sanity.
- [ ] Ensure no geometry semantics or Euclidean defaults are embedded.

### 3) `gsae-state-geometry`

- [ ] Implement full `StateMetric` trait: `metric`, `metric_inv`, `metric_deriv`, `christoffel`, `curvature`.
- [ ] Add at least one non-Euclidean analytic metric implementation.
- [ ] If Euclidean or identity metric exists, name and label it as baseline or fixture.
- [ ] Add tests: metric/inverse round-trip, Christoffel symmetry, nontrivial derivative and curvature for a non-Euclidean case.

### 4) `gsae-autodiff`

- [ ] Provide exact Jacobian and Hessian access paths.
- [ ] If finite differences exist, mark them as diagnostics only.
- [ ] Add tests: Jacobian on analytic nonlinear map, Hessian component check, optional FD-vs-exact comparison labeled as diagnostic.

### 5) `gsae-chart-core`

- [ ] Implement `SparseChart` trait exactly as specified, including `jacobian_encode`.
- [ ] Implement explicit sparsifier and validity/domain semantics.
- [ ] Add tests: local encode/decode reconstruction, sparsity support, validity on in-domain and out-of-domain points, Jacobian and Hessian availability.
- [ ] Add inverse-consistency diagnostics or report.

### 6) `gsae-pullback-metric`

- [ ] Implement exact pullback: `g = J^T G_M J` with no `J^T J` fallback.
- [ ] Include derivatives of the target metric in `dg` and Christoffel calculations.
- [ ] Add tests: exact formula check on analytic chart + metric, SPD on valid region, symmetry, `dg` consistency.

### 7) `gsae-geodesics`

- [ ] Implement path energy and path length, IVP, BVP, `exp` and `log` maps; optional parallel transport.
- [ ] Add tests: zero-velocity IVP, `exp/log` local inverse, length and energy sanity under subdivision, intrinsic distance differs from latent `L2` for a non-Euclidean metric.

### 8) `gsae-objective`

- [ ] Implement separated terms: reconstruction, sparsity, metric, gauge, transport, hypergraph.
- [ ] Ensure no optimizer or runtime logic exists here.
- [ ] Add tests validating term combination and separation, with no training leakage.

### Foundation Gate (must pass before witness phase)

- [ ] Run per-crate tests in order, matching `BUILD_GUIDE.md` section 7.1.
- [ ] Re-run contamination audit for forbidden substitutions in foundational crates.
- [ ] Verify the analytic non-Euclidean fixture spans chart + state metric + pullback + geodesics.

---

## Phase 1 — Witness crates (must pass in order)

### 9) `gsae-1-chart`

- [ ] Implement a nonlinear sparse chart witness.
- [ ] Provide a witness report: validity, support size, reconstruction error, sample code.
- [ ] Add tests: nontrivial valid region, explicit sparsity, accurate local reconstruction.

### 10) `gsae-2-metric`

- [ ] Implement a witness report with intrinsic geodesic distance and metric diagnostics.
- [ ] Compare explicitly against a latent Euclidean baseline that is clearly labeled.
- [ ] Add tests: geodesic distance differs meaningfully from Euclidean baseline on a non-Euclidean fixture.

### 11) `gsae-3-gauge`

- [ ] Implement two overlapping same-layer charts and a transition diffeomorphism.
- [ ] Add a witness report: overlap validity, transition error, decode commutation, isometry residual.
- [ ] Add tests: nonempty overlap, invertible transition, metric pullback compatibility.

### 12) `gsae-4-transport`

- [ ] Implement an adjacent-layer transport map, not a gauge reuse.
- [ ] Add a witness report: transported code error, baseline comparison, metric-preservation residual.
- [ ] Add tests: transport beats baseline, metric preservation within tolerance.

### 13) `gsae-5-hypergraph`

- [ ] Implement a nonlinear hyperedge gate in the decoder.
- [ ] Add a witness report: hyperedge ID, mixed derivative, hyperedge delta, singleton ablation error.
- [ ] Add tests: hyperedge effect is non-additive and nonlinear.

### 14) `gsae-6-hyperpath`

- [ ] Implement transported section plus downstream hyperedge deformation as a joint object.
- [ ] Add a witness report comparing transport + hyperedge against singleton explanations.
- [ ] Add tests: the combined mechanism is required; transport-only and hyperedge-only are insufficient.

### Witness Gate (must pass before scale-up phase)

- [ ] Run witness tests in order, matching `BUILD_GUIDE.md` section 7.2.
- [ ] Verify all witness reports include required diagnostics.
- [ ] Re-audit baseline labeling for any Euclidean or 2D harness used in witnesses.

---

## Phase 2 — Scale-up crates (only after witnesses are green)

### 15) `gsae-atlas-router`

- [ ] Implement chart routing without redefining chart ontology.
- [ ] Add tests: routing correctness without altering chart semantics.

### 16) `gsae-atlas-manager`

- [ ] Implement registry, overlap graph, and cocycle bookkeeping.
- [ ] Add tests: dependency direction is preserved, with no witness dependency on manager.

### 17) `gsae-multilayer-runtime`

- [ ] Implement transport chains across multiple layers without redefining primitives.
- [ ] Add tests: transport chain consistency with witness transport semantics.

### 18) `gsae-training`

- [ ] Implement optimizer and runtime orchestration using `gsae-objective` only.
- [ ] Add tests: objective usage without redefining the math.

### 19) `gsae-artifacts`

- [ ] Implement serialization preserving mathematical identities.
- [ ] Add tests: schema round-trip for witness reports and core objects.

### 20) `gsae-benchmarks`

- [ ] Implement benchmarks that consume, rather than define, witnesses.
- [ ] Add tests: benchmarks do not introduce core assumptions.

### 21) `gsae-cli`

- [ ] Implement CLI wrappers around existing APIs without new semantics.
- [ ] Add tests: CLI uses existing crate contracts.

### 22) `gsae-py`

- [ ] Implement Python bindings without redefining or simplifying the math.
- [ ] Add tests: parity with Rust APIs and semantics.

---

## PR Review Checklist (aligns with `DEVELOPMENT_GUIDE.md` 9.x)

- [ ] Object identity: which definition or claim is implemented, and in which crate tier.
- [ ] Assumptions: explicit and labeled as theorem-level, solver-level, or baseline-level.
- [ ] Anti-substitution: no silent `J^T J`, Euclidean distance, affine gauge, cosine transport, or additive hyperedge.
- [ ] Layer discipline: no downward dependency violations.
- [ ] Naming discipline: Euclidean and 2D harnesses are labeled with the required suffixes.
- [ ] Test discipline: tests fail if implementation collapses to Euclidean or additive special cases.

---

## Definition of Done (minimal executable realization)

- [ ] All foundational crates pass gates in order.
- [ ] All witness crates pass gates in order.
- [ ] Every claim in the note has a non-vacuous executable witness.
- [ ] Euclidean and 2D machinery are explicitly labeled and isolated.
- [ ] No scale-up crate is required for first-witness claim coverage.
