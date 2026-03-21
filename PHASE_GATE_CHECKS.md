# PHASE_GATE_CHECKS.md

Strict gate protocol for the minimal executable realization of GSAE.

This file is operational, not aspirational. Every phase gate is blocking. If a gate fails, the repo does **not** advance. The purpose of these checks is to preserve mathematical identity and prevent silent drift toward Euclidean, additive, affine, or otherwise flattened substitutes.

---

## 1. Gate policy

### 1.1 Advancement rule
A phase advances only when all of the following are true:

1. required crates for the phase compile
2. required tests pass
3. required evidence artifacts exist
4. anti-substitution audit passes
5. dependency discipline passes
6. no unresolved failures are reclassified as “known issues” unless they are outside the current phase boundary

### 1.2 Blocking rule
Any of the following blocks advancement:

- any foundational or witness crate test failure
- missing evidence artifact for a required witness
- unlabeled Euclidean or 2D simplification path in foundational or witness crates
- reintroduction of `J^T J` where `J^T G_M J` is required
- gauge implemented as a generic affine fit without explicit diffeomorphic semantics
- transport implemented as cosine matching or reused same-layer gauge map
- hypergraph interaction implemented as additive recombination
- downward dependency leak from scale-up crates into foundational or witness crates

### 1.3 Naming rule for non-core simplifications
Any Euclidean, 2D, flat, toy, sanity, or analytic convenience path must be named using one of:

- `*_baseline`
- `*_fixture`
- `*_toy`
- `*_sanity`
- `*_analytic_harness`

If it is not named this way, it is treated as an unlawful silent substitution.

---

## 2. Required evidence outputs

Each gate requires evidence. Evidence must be committed or emitted in a deterministic, reviewable form.

### 2.1 Evidence directory convention

```text
artifacts/gates/
  preflight/
  phase0-foundation/
  phase1-witness/
  phase2-scale-up/
```

### 2.2 Required evidence file types

- `*.txt` for command output logs
- `*.json` for structured diagnostics
- `*.md` for witness reports and human-readable summaries
- `*.svg` or `*.png` only if visual diagnostics are actually required

### 2.3 Minimum evidence contents
Every gate artifact set must include:

- command transcript
- pass/fail summary
- crate list covered by the gate
- explicit mention of any baseline or harness used
- explicit statement that no unlabeled Euclidean/2D path remains in gated crates

---

## 3. Standard command conventions

These commands assume a Rust workspace root.

### 3.1 Formatting and linting

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

### 3.2 Per-crate tests

```bash
cargo test -p <crate-name> -- --nocapture
```

### 3.3 Whole-workspace tests

```bash
cargo test --workspace -- --nocapture
```

### 3.4 Dependency tree checks

```bash
cargo tree -p <crate-name>
```

### 3.5 Audit grep checks
Use ripgrep if available.

```bash
rg -n "J\^T J|norm2|euclid|euclidean|identity metric|cosine similarity|affine gauge|additive hyperedge" crates/
```

Additional targeted searches:

```bash
rg -n "J\s*\^?T\s*J|transpose\(\).*matmul\(.*jacobian|jt\.matmul\(&j\)" crates/
rg -n "cosine|dot.*norm|nearest_neighbor.*cos" crates/
rg -n "affine|linear map|least squares overlap|procrustes" crates/gsae-3-gauge crates/gsae-4-transport
rg -n "sum of singleton|additive|linear combination" crates/gsae-5-hypergraph crates/gsae-6-hyperpath
```

These grep checks are **screening tools**, not proofs. Reviewers must inspect hits.

---

## 4. Pre-flight gate

This gate must pass before any phase work is accepted.

### 4.1 Commands

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings || true
rg -n "J\^T J|norm2|euclid|euclidean|identity metric|cosine similarity|affine gauge|additive hyperedge" crates/ || true
```

If the workspace is not yet complete enough for `clippy`, record that explicitly in the pre-flight artifact.

### 4.2 Required checks

- every foundational and witness crate has module-level docs
- every crate doc states:
  - object implemented
  - exact definition/formula
  - assumptions
  - forbidden substitutions
  - witness role if applicable
- all Euclidean/2D paths are explicitly labeled as baseline/fixture/toy/sanity/harness
- dependency direction is correct at the crate manifest level

### 4.3 Evidence required

```text
artifacts/gates/preflight/
  fmt.txt
  clippy.txt
  substitution-audit.txt
  crate-doc-audit.md
  dependency-audit.md
```

### 4.4 Pass condition
All items above exist and no unlabeled simplification path remains in foundational or witness crates.

### 4.5 Fail examples

- undocumented chart crate
- unlabeled `EuclideanMetric` used in a foundational metric path
- witness crate depends on CLI or benchmarks

---

## 5. Phase 0 gate — foundational crates

Phase 0 crates, in mandatory order:

1. `gsae-core-types`
2. `gsae-linalg`
3. `gsae-state-geometry`
4. `gsae-autodiff`
5. `gsae-chart-core`
6. `gsae-pullback-metric`
7. `gsae-geodesics`
8. `gsae-objective`

No later foundational crate is accepted until the earlier one passes.

---

### 5.1 Gate for `gsae-core-types`

#### Commands

```bash
cargo test -p gsae-core-types -- --nocapture
cargo tree -p gsae-core-types
```

#### Required evidence

```text
artifacts/gates/phase0-foundation/gsae-core-types/
  test.txt
  dep-tree.txt
  summary.md
```

#### Pass conditions

- core types exist:
  - `Scalar`
  - `StatePoint`
  - `CodePoint`
  - `Tangent`
  - `ChartId`
  - `LayerId`
  - `HyperedgeId`
- shared tensor/matrix aliases or wrappers compile
- no runtime, training, CLI, benchmark, or orchestration logic exists here
- deterministic equality and debuggability tests pass where relevant

#### Blockers

- training config type appears in this crate
- witness-specific logic appears here

---

### 5.2 Gate for `gsae-linalg`

#### Commands

```bash
cargo test -p gsae-linalg -- --nocapture
cargo tree -p gsae-linalg
```

#### Required evidence

```text
artifacts/gates/phase0-foundation/gsae-linalg/
  test.txt
  dep-tree.txt
  spd-report.md
```

#### Pass conditions

- SPD factorization works on analytic SPD inputs
- stable solve/inverse round-trip passes
- determinant/logdet checks pass
- tensor contraction and indexing sanity tests pass
- no geometry semantics or Euclidean defaults are embedded

#### Blockers

- linalg adds semantic assumptions like “distance means Euclidean norm”
- metric-specific shortcuts appear here

---

### 5.3 Gate for `gsae-state-geometry`

#### Commands

```bash
cargo test -p gsae-state-geometry -- --nocapture
cargo tree -p gsae-state-geometry
```

#### Required evidence

```text
artifacts/gates/phase0-foundation/gsae-state-geometry/
  test.txt
  dep-tree.txt
  analytic-metric-report.md
  curvature-report.json
```

#### Pass conditions

- `StateMetric` trait is implemented with:
  - `metric`
  - `metric_inv`
  - `metric_deriv`
  - `christoffel`
  - `curvature`
- at least one non-Euclidean analytic metric implementation exists
- any Euclidean/identity metric is labeled baseline or fixture
- tests confirm:
  - metric/inverse consistency
  - Christoffel symmetry where appropriate
  - nontrivial derivative and curvature for non-Euclidean case

#### Blockers

- only Euclidean metric exists
- non-Euclidean fixture has zero curvature because it secretly collapses to flat space

---

### 5.4 Gate for `gsae-autodiff`

#### Commands

```bash
cargo test -p gsae-autodiff -- --nocapture
cargo tree -p gsae-autodiff
```

#### Required evidence

```text
artifacts/gates/phase0-foundation/gsae-autodiff/
  test.txt
  dep-tree.txt
  derivative-report.md
```

#### Pass conditions

- exact Jacobian path exists
- exact Hessian path exists
- analytic nonlinear map derivative checks pass
- any finite-difference path is labeled diagnostics only

#### Blockers

- Hessians are only available via silent finite differences in foundational paths
- derivative API is coupled to witness or runtime crates

---

### 5.5 Gate for `gsae-chart-core`

#### Commands

```bash
cargo test -p gsae-chart-core -- --nocapture
cargo tree -p gsae-chart-core
```

#### Required evidence

```text
artifacts/gates/phase0-foundation/gsae-chart-core/
  test.txt
  dep-tree.txt
  chart-core-report.md
```

#### Pass conditions

- `SparseChart` trait implemented exactly as specified
- nonlinear encoder and decoder exist
- explicit sparsifier exists
- validity/domain semantics exist
- tests confirm:
  - local encode/decode reconstruction
  - explicit sparsity support
  - validity differs in-domain vs out-of-domain
  - Jacobian/Hessian availability
- inverse-consistency diagnostic exists or is emitted

#### Blockers

- dense autoencoder substituted for sparse chart
- chart validity replaced by generic confidence score without domain semantics
- only linear chart exists in core path

---

### 5.6 Gate for `gsae-pullback-metric`

#### Commands

```bash
cargo test -p gsae-pullback-metric -- --nocapture
cargo tree -p gsae-pullback-metric
rg -n "J\^T J|jt\.matmul\(&j\)" crates/gsae-pullback-metric crates/gsae-state-geometry crates/gsae-chart-core
```

#### Required evidence

```text
artifacts/gates/phase0-foundation/gsae-pullback-metric/
  test.txt
  dep-tree.txt
  substitution-audit.txt
  pullback-report.md
```

#### Pass conditions

- exact pullback implemented as:
  \[
  g = J^T G_{\mathcal M} J
  \]
- derivatives include target metric derivative terms
- tests confirm:
  - symmetry
  - SPD on valid region
  - exact formula match on analytic chart+metric
  - `dg` consistency
- no unlabeled `J^T J` fallback exists in core path

#### Blockers

- pullback metric silently reduces to Euclidean immersion formula
- target metric derivative omitted from `dg`

---

### 5.7 Gate for `gsae-geodesics`

#### Commands

```bash
cargo test -p gsae-geodesics -- --nocapture
cargo tree -p gsae-geodesics
```

#### Required evidence

```text
artifacts/gates/phase0-foundation/gsae-geodesics/
  test.txt
  dep-tree.txt
  geodesic-report.md
```

#### Pass conditions

- path energy and length implemented
- IVP implemented
- BVP implemented
- exp/log maps implemented
- tests confirm:
  - zero-velocity IVP stays fixed
  - exp/log are local inverses within tolerance
  - path length/energy subdivision sanity
  - intrinsic distance differs from latent Euclidean distance on a non-Euclidean fixture

#### Blockers

- geodesic distance implemented as latent `L2`
- BVP stubbed or replaced by straight-line interpolation without explicit harness labeling

---

### 5.8 Gate for `gsae-objective`

#### Commands

```bash
cargo test -p gsae-objective -- --nocapture
cargo tree -p gsae-objective
```

#### Required evidence

```text
artifacts/gates/phase0-foundation/gsae-objective/
  test.txt
  dep-tree.txt
  objective-report.md
```

#### Pass conditions

- separated terms exist:
  - reconstruction
  - sparsity
  - metric
  - gauge
  - transport
  - hypergraph
- no optimizer or training runtime logic exists
- tests confirm term separation and combination without orchestration leakage

#### Blockers

- optimizer schedules appear here
- witness logic is embedded into the objective crate

---

### 5.9 Foundation phase gate

#### Commands

```bash
cargo test -p gsae-core-types -- --nocapture
cargo test -p gsae-linalg -- --nocapture
cargo test -p gsae-state-geometry -- --nocapture
cargo test -p gsae-autodiff -- --nocapture
cargo test -p gsae-chart-core -- --nocapture
cargo test -p gsae-pullback-metric -- --nocapture
cargo test -p gsae-geodesics -- --nocapture
cargo test -p gsae-objective -- --nocapture
rg -n "J\^T J|norm2|euclid|euclidean|identity metric|cosine similarity|affine gauge|additive hyperedge" crates/gsae-core-types crates/gsae-linalg crates/gsae-state-geometry crates/gsae-autodiff crates/gsae-chart-core crates/gsae-pullback-metric crates/gsae-geodesics crates/gsae-objective || true
```

#### Required evidence

```text
artifacts/gates/phase0-foundation/
  ordered-test-run.txt
  substitution-audit.txt
  non-euclidean-fixture-span.md
```

#### Pass condition
The analytic non-Euclidean fixture spans:
- chart
- state metric
- pullback metric
- geodesics

and all foundational crates pass in order.

---

## 6. Phase 1 gate — witness crates

Mandatory order:

1. `gsae-1-chart`
2. `gsae-2-metric`
3. `gsae-3-gauge`
4. `gsae-4-transport`
5. `gsae-5-hypergraph`
6. `gsae-6-hyperpath`

---

### 6.1 Gate for `gsae-1-chart`

#### Commands

```bash
cargo test -p gsae-1-chart -- --nocapture
cargo tree -p gsae-1-chart
```

#### Required evidence

```text
artifacts/gates/phase1-witness/gsae-1-chart/
  test.txt
  dep-tree.txt
  witness-report.md
```

#### Witness report must include

- valid region description
- support size statistics
- reconstruction error summary
- sample sparse codes
- explicit note that the witness is nonlinear and local

#### Pass conditions

- nontrivial valid region exists
- sparse support is explicit, not merely dense with small magnitudes
- local reconstruction is accurate on the valid region

#### Blockers

- witness degenerates to dense AE
- “local” claim has no domain semantics

---

### 6.2 Gate for `gsae-2-metric`

#### Commands

```bash
cargo test -p gsae-2-metric -- --nocapture
cargo tree -p gsae-2-metric
```

#### Required evidence

```text
artifacts/gates/phase1-witness/gsae-2-metric/
  test.txt
  dep-tree.txt
  witness-report.md
  metric-diagnostics.json
```

#### Witness report must include

- intrinsic geodesic distance diagnostic
- explicit Euclidean latent baseline labeled as baseline
- statement of how the non-Euclidean fixture separates the two

#### Pass conditions

- geodesic distance differs meaningfully from Euclidean latent baseline on non-Euclidean fixture
- metric diagnostic confirms non-flat pointwise behavior or nontrivial path geometry

#### Blockers

- witness compares only quadratic forms at one point and not geodesic behavior
- baseline is unlabeled or reused as core path

---

### 6.3 Gate for `gsae-3-gauge`

#### Commands

```bash
cargo test -p gsae-3-gauge -- --nocapture
cargo tree -p gsae-3-gauge
```

#### Required evidence

```text
artifacts/gates/phase1-witness/gsae-3-gauge/
  test.txt
  dep-tree.txt
  witness-report.md
  overlap-diagnostics.json
```

#### Witness report must include

- overlap validity statistics
- transition error
- decode commutation residual
- metric isometry residual

#### Pass conditions

- overlap is nonempty
- transition map is invertible/diffeomorphic in the intended local sense
- metric pullback compatibility passes within tolerance
- decoding approximately commutes with chart transition

#### Blockers

- gauge witness uses different layers
- transition is only an unlabeled affine fit
- overlap is vacuous

---

### 6.4 Gate for `gsae-4-transport`

#### Commands

```bash
cargo test -p gsae-4-transport -- --nocapture
cargo tree -p gsae-4-transport
```

#### Required evidence

```text
artifacts/gates/phase1-witness/gsae-4-transport/
  test.txt
  dep-tree.txt
  witness-report.md
  transport-diagnostics.json
```

#### Witness report must include

- transported code/section error
- baseline comparison
- metric-preservation residual
- explicit note that this is not same-layer gauge reuse

#### Pass conditions

- transport beats explicitly labeled baseline
- transport preserves metric structure within tolerance

#### Blockers

- “transport” is implemented by cosine nearest-neighbor matching
- same-layer gauge code is reused and renamed as transport

---

### 6.5 Gate for `gsae-5-hypergraph`

#### Commands

```bash
cargo test -p gsae-5-hypergraph -- --nocapture
cargo tree -p gsae-5-hypergraph
```

#### Required evidence

```text
artifacts/gates/phase1-witness/gsae-5-hypergraph/
  test.txt
  dep-tree.txt
  witness-report.md
  hyperedge-diagnostics.json
```

#### Witness report must include

- hyperedge id
- nonlinear gate description
- mixed derivative or equivalent non-additivity diagnostic
- hyperedge ablation delta
- singleton ablation comparison

#### Pass conditions

- hyperedge effect is genuinely nonlinear
- effect is not reducible to additive singleton contributions

#### Blockers

- hyperedge is just named grouping of additive terms
- witness lacks any non-additive differential or ablation signature

---

### 6.6 Gate for `gsae-6-hyperpath`

#### Commands

```bash
cargo test -p gsae-6-hyperpath -- --nocapture
cargo tree -p gsae-6-hyperpath
```

#### Required evidence

```text
artifacts/gates/phase1-witness/gsae-6-hyperpath/
  test.txt
  dep-tree.txt
  witness-report.md
  hyperpath-diagnostics.json
```

#### Witness report must include

- transported section summary
- downstream hyperedge deformation summary
- comparison against transport-only explanation
- comparison against hyperedge-only explanation
- comparison against singleton explanations

#### Pass conditions

- combined mechanism is required
- transport-only is insufficient
- hyperedge-only is insufficient
- singleton explanations are insufficient

#### Blockers

- witness is reducible to transport alone
- witness is reducible to hyperedge ablation alone

---

### 6.7 Witness phase gate

#### Commands

```bash
cargo test -p gsae-1-chart -- --nocapture
cargo test -p gsae-2-metric -- --nocapture
cargo test -p gsae-3-gauge -- --nocapture
cargo test -p gsae-4-transport -- --nocapture
cargo test -p gsae-5-hypergraph -- --nocapture
cargo test -p gsae-6-hyperpath -- --nocapture
rg -n "euclid|euclidean|2d|affine gauge|cosine|additive hyperedge" crates/gsae-1-chart crates/gsae-2-metric crates/gsae-3-gauge crates/gsae-4-transport crates/gsae-5-hypergraph crates/gsae-6-hyperpath || true
```

#### Required evidence

```text
artifacts/gates/phase1-witness/
  ordered-test-run.txt
  witness-report-index.md
  baseline-label-audit.txt
```

#### Pass condition
Every witness crate has a corresponding witness report and every major claim in the note has a non-vacuous executable witness.

---

## 7. Phase 2 gate — scale-up crates

Scale-up crates are only legal after foundational and witness gates are green.

Order:

1. `gsae-atlas-router`
2. `gsae-atlas-manager`
3. `gsae-multilayer-runtime`
4. `gsae-training`
5. `gsae-artifacts`
6. `gsae-benchmarks`
7. `gsae-cli`
8. `gsae-py`

These do not create new first-witness types. Their gates enforce non-corruption of lower-tier semantics.

---

### 7.1 Gate for `gsae-atlas-router`

#### Commands

```bash
cargo test -p gsae-atlas-router -- --nocapture
cargo tree -p gsae-atlas-router
```

#### Pass conditions

- routing uses existing chart semantics
- chart ontology is not redefined in router code

#### Evidence

```text
artifacts/gates/phase2-scale-up/gsae-atlas-router/
  test.txt
  dep-tree.txt
  routing-report.md
```

---

### 7.2 Gate for `gsae-atlas-manager`

#### Commands

```bash
cargo test -p gsae-atlas-manager -- --nocapture
cargo tree -p gsae-atlas-manager
```

#### Pass conditions

- registry/overlap/cocycle bookkeeping works
- no witness crate depends downward on manager

#### Evidence

```text
artifacts/gates/phase2-scale-up/gsae-atlas-manager/
  test.txt
  dep-tree.txt
  atlas-manager-report.md
```

---

### 7.3 Gate for `gsae-multilayer-runtime`

#### Commands

```bash
cargo test -p gsae-multilayer-runtime -- --nocapture
cargo tree -p gsae-multilayer-runtime
```

#### Pass conditions

- transport chains preserve witness transport semantics
- no new transport definition is introduced here

#### Evidence

```text
artifacts/gates/phase2-scale-up/gsae-multilayer-runtime/
  test.txt
  dep-tree.txt
  runtime-report.md
```

---

### 7.4 Gate for `gsae-training`

#### Commands

```bash
cargo test -p gsae-training -- --nocapture
cargo tree -p gsae-training
```

#### Pass conditions

- runtime uses `gsae-objective`
- math is not redefined in orchestration code

#### Evidence

```text
artifacts/gates/phase2-scale-up/gsae-training/
  test.txt
  dep-tree.txt
  training-report.md
```

---

### 7.5 Gate for `gsae-artifacts`

#### Commands

```bash
cargo test -p gsae-artifacts -- --nocapture
cargo tree -p gsae-artifacts
```

#### Pass conditions

- round-trip serialization preserves mathematical identity and witness reports

#### Evidence

```text
artifacts/gates/phase2-scale-up/gsae-artifacts/
  test.txt
  dep-tree.txt
  schema-roundtrip-report.md
```

---

### 7.6 Gate for `gsae-benchmarks`

#### Commands

```bash
cargo test -p gsae-benchmarks -- --nocapture
cargo tree -p gsae-benchmarks
```

#### Pass conditions

- benchmarks consume witnesses
- benchmarks do not define core assumptions

#### Evidence

```text
artifacts/gates/phase2-scale-up/gsae-benchmarks/
  test.txt
  dep-tree.txt
  benchmark-boundary-report.md
```

---

### 7.7 Gate for `gsae-cli`

#### Commands

```bash
cargo test -p gsae-cli -- --nocapture
cargo tree -p gsae-cli
```

#### Pass conditions

- CLI wraps existing APIs only
- no new semantics introduced

#### Evidence

```text
artifacts/gates/phase2-scale-up/gsae-cli/
  test.txt
  dep-tree.txt
  cli-contract-report.md
```

---

### 7.8 Gate for `gsae-py`

#### Commands

```bash
cargo test -p gsae-py -- --nocapture
cargo tree -p gsae-py
```

If Python-side parity tests exist, also run the appropriate Python test command.

#### Pass conditions

- Python bindings preserve Rust semantics
- no simplification of math at the binding layer

#### Evidence

```text
artifacts/gates/phase2-scale-up/gsae-py/
  test.txt
  dep-tree.txt
  py-parity-report.md
```

---

### 7.9 Scale-up phase gate

#### Commands

```bash
cargo test -p gsae-atlas-router -- --nocapture
cargo test -p gsae-atlas-manager -- --nocapture
cargo test -p gsae-multilayer-runtime -- --nocapture
cargo test -p gsae-training -- --nocapture
cargo test -p gsae-artifacts -- --nocapture
cargo test -p gsae-benchmarks -- --nocapture
cargo test -p gsae-cli -- --nocapture
cargo test -p gsae-py -- --nocapture
cargo test --workspace -- --nocapture
```

#### Required evidence

```text
artifacts/gates/phase2-scale-up/
  ordered-test-run.txt
  workspace-test-run.txt
  boundary-audit.md
```

#### Pass condition
All scale-up crates are green and none of them redefine foundational or witness mathematics.

---

## 8. PR gate

Every PR touching foundational or witness crates must include a review block answering these questions.

### 8.1 Required PR checklist

- [ ] Which definition, proposition, or stage does this implement?
- [ ] Which crate tier is affected: foundational, witness, or scale-up?
- [ ] What assumptions are theorem-level, solver-level, or baseline-only?
- [ ] Does any path reintroduce `J^T J`, Euclidean latent distance, affine gauge, cosine transport, or additive hyperedge semantics?
- [ ] Are all Euclidean/2D harnesses explicitly labeled?
- [ ] Are dependency directions preserved?
- [ ] Do tests fail if the implementation collapses to a flat/additive special case?

### 8.2 Blocking PR failures

- missing object-identity statement
- unlabeled baseline path
- no test covering collapse-to-flat regression

---

## 9. Definition-of-done gate

The repo reaches minimal executable realization only when all of the following are true:

- all foundational crates pass in order
- all witness crates pass in order
- every major claim in the note has a non-vacuous executable witness
- Euclidean/2D machinery is explicitly labeled and isolated
- no scale-up crate is required for first-witness claim coverage
- workspace tests pass

### 9.1 Final commands

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace -- --nocapture
rg -n "J\^T J|norm2|euclid|euclidean|identity metric|cosine similarity|affine gauge|additive hyperedge" crates/ || true
```

### 9.2 Final evidence

```text
artifacts/gates/
  final-summary.md
  workspace-test-run.txt
  final-substitution-audit.txt
  witness-coverage-matrix.md
```

The `witness-coverage-matrix.md` file must map:

- Definition 2 → `gsae-1-chart`, `gsae-2-metric`
- Definition 3 → `gsae-3-gauge`
- Definition 4 → `gsae-4-transport`
- Definition 5 → `gsae-5-hypergraph`
- Proposition 1 → `gsae-3-gauge`
- Proposition 2 → `gsae-6-hyperpath`
- Stage 1 → `gsae-2-metric`
- Stage 2 → `gsae-3-gauge`
- Stage 3 → `gsae-4-transport`
- Stage 4 → `gsae-5-hypergraph`

---

## 10. Minimal reviewer summary

If a reviewer needs a compressed rule set, use this:

1. Foundational crates define the math.
2. Witness crates instantiate the first executable proof of each claim.
3. Scale-up crates may extend coverage, never redefine primitives.
4. Any silent flattening to Euclidean, affine, cosine, or additive defaults is a gate failure.
5. No phase advances without evidence.
