# GSAE Project Build Guide

Ordered build plan with phased test gates for the minimal executable realization.

This guide is intentionally **theory-first**. A phase is complete only when its mathematical exit gates pass. The order is designed to preserve the dependency discipline:

**Foundational -> Witness -> Scale-up**

and to prevent quiet collapse into flat Euclidean substitutions or convenience-driven rewrites.

---

## 0. Global rules before any build work

### 0.1 Non-substitution rule

No phase may introduce any of the following in foundational or witness crates unless the code path is explicitly labeled as a baseline or fixture:

- `G_M(x) = I` as an unmarked default
- `g = J^T J` in place of `g = J^T G_M J`
- latent Euclidean `L2` in place of intrinsic geodesic distance
- affine overlap maps in place of transition diffeomorphisms
- cosine matching in place of transport
- additive decoder structure in place of nonlinear hyperedge interaction

### 0.2 Build discipline

Every phase must produce all three:

1. compileable crate interfaces
2. deterministic unit tests
3. witness-facing diagnostics or reports

### 0.3 Phase advancement rule

Do not advance to the next phase unless all gates in the current phase pass.

If a later phase exposes a flaw in an earlier phase, fix the earlier phase and rerun all downstream gates.

---

## 1. Workspace order

### Phase 0 — Foundation substrate

Build in this exact order:

1. `gsae-core-types`
2. `gsae-linalg`
3. `gsae-state-geometry`
4. `gsae-autodiff`
5. `gsae-chart-core`
6. `gsae-pullback-metric`
7. `gsae-geodesics`
8. `gsae-objective`

### Phase 1 — Witness sequence

Build in this exact order:

9. `gsae-1-chart`
10. `gsae-2-metric`
11. `gsae-3-gauge`
12. `gsae-4-transport`
13. `gsae-5-hypergraph`
14. `gsae-6-hyperpath`

### Phase 2 — Scale-up only after witnesses are green

15. `gsae-atlas-router`
16. `gsae-atlas-manager`
17. `gsae-multilayer-runtime`
18. `gsae-training`
19. `gsae-artifacts`
20. `gsae-benchmarks`
21. `gsae-cli`
22. `gsae-py`

---

## 2. Phase 0 gates: foundational crates

# 2.1 `gsae-core-types`

## Purpose
Shared mathematical ontology.

## Required contents

- `Scalar`
- `StatePoint`
- `CodePoint`
- `Tangent`
- `ChartId`
- `LayerId`
- `HyperedgeId`
- dense matrix / tensor wrappers or type aliases used everywhere else

## Exit gates

### Gate A — ontology compiles
All foundational crates can depend on these types without redefining local copies.

### Gate B — no semantic leakage
This crate contains no benchmark labels, routing logic, training policy, or CLI concerns.

### Gate C — deterministic equality / debugability
Core types support debug output and stable construction for tests.

## Suggested commands

```bash
cargo test -p gsae-core-types
```

## Fail conditions

- duplicate point/tensor types appear in downstream crates
- chart/layer IDs are represented inconsistently
- mathematical data types depend on runtime/training logic

---

# 2.2 `gsae-linalg`

## Purpose
Numerical substrate for geometry.

## Required contents

- SPD factorization
- inverse / solve for SPD matrices
- determinant / logdet
- tensor contraction helpers
- small stable routines for Jacobian/Hessian/Christoffel code

## Exit gates

### Gate A — SPD contract
For SPD inputs, Cholesky succeeds and inverse/solve are numerically consistent.

### Gate B — tensor indexing contract
Tensor indexing and contraction APIs are stable and unambiguous.

### Gate C — no geometry semantics
No code in this crate assumes Euclidean state geometry, chart semantics, or witness logic.

## Suggested commands

```bash
cargo test -p gsae-linalg
```

## Required tests

- SPD inversion round-trip
- determinant consistency on small analytic matrices
- tensor contraction sanity checks

## Fail conditions

- silent fallback from SPD solve to pseudo-inverse without explicit labeling
- geometry-specific assumptions embedded in helpers
- matrix layout ambiguity between crates

---

# 2.3 `gsae-state-geometry`

## Purpose
Target manifold metric and its derivatives.

## Required trait

```rust
pub trait StateMetric {
    fn state_dim(&self) -> usize;
    fn metric(&self, x: &StatePoint) -> Matrix;
    fn metric_inv(&self, x: &StatePoint) -> Matrix;
    fn metric_deriv(&self, x: &StatePoint) -> Tensor3;
    fn christoffel(&self, x: &StatePoint) -> Tensor3;
    fn curvature(&self, x: &StatePoint) -> Tensor4;
}
```

## Exit gates

### Gate A — metric coherence
For every provided `StateMetric`, `metric_inv` is the inverse of `metric` up to tolerance.

### Gate B — Levi-Civita coherence
Returned Christoffels match the metric derivative contract.

### Gate C — non-Euclidean witness exists
At least one non-Euclidean analytic metric implementation exists and is used in tests.

### Gate D — Euclidean case is explicit
If an identity metric exists, it is explicitly named Euclidean and used only as a baseline or fixture.

## Suggested commands

```bash
cargo test -p gsae-state-geometry
```

## Required tests

- metric/inverse round-trip
- Christoffel symmetry in lower indices
- curvature zero for explicit flat baseline metric if included
- nontrivial curvature or nontrivial metric derivatives for at least one non-Euclidean example

## Fail conditions

- state metric defaults silently to identity
- target-space covariant structure is absent
- Euclidean helper becomes the only tested implementation

---

# 2.4 `gsae-autodiff`

## Purpose
Exact derivative access for chart and state geometry code.

## Required contents

- Jacobian support
- Hessian support
- derivative consistency checks
- clear separation between exact and approximate derivative paths

## Exit gates

### Gate A — exact derivative path exists
Core chart and metric code can obtain Jacobians and Hessians without relying on finite differences.

### Gate B — approximation paths are labeled
If finite differences exist, they are marked as debugging/test-only tools.

### Gate C — derivative agreement tests
Exact derivative routines agree with analytic derivatives on known fixtures.

## Suggested commands

```bash
cargo test -p gsae-autodiff
```

## Required tests

- Jacobian for a known nonlinear map
- Hessian component test on an analytic function
- optional FD vs exact comparison, labeled as diagnostic only

## Fail conditions

- finite differences silently power foundational geometry in production paths
- Hessian access is missing, forcing Euclidean simplifications later

---

# 2.5 `gsae-chart-core`

## Purpose
Actual sparse chart primitive.

## Required trait boundary

```rust
pub trait SparseChart {
    fn chart_id(&self) -> ChartId;
    fn layer_id(&self) -> LayerId;
    fn state_dim(&self) -> usize;
    fn latent_dim(&self) -> usize;

    fn encode(&self, x: &StatePoint) -> CodePoint;
    fn decode(&self, z: &CodePoint) -> StatePoint;
    fn validity(&self, x: &StatePoint) -> f64;

    fn jacobian_encode(&self, x: &StatePoint) -> Matrix;
    fn jacobian_decode(&self, z: &CodePoint) -> Matrix;
    fn hessian_decode_component(&self, z: &CodePoint, a: usize) -> Matrix;
}
```

## Exit gates

### Gate A — nonlinear chart exists
At least one actual nonlinear sparse chart implementation exists.

### Gate B — explicit sparsifier exists
Sparsity is not merely incidental; the chart has an explicit sparsifier or equivalent mechanism.

### Gate C — local-domain semantics exist
`validity(x)` is present and tested.

### Gate D — inverse-consistency diagnostics exist
Chart code exposes or computes local coordinate consistency diagnostics.

## Suggested commands

```bash
cargo test -p gsae-chart-core
```

## Required tests

- encode/decode local reconstruction on an analytic region
- sparsity support test
- validity behavior on in-domain and out-of-domain points
- Jacobian/Hessian availability for decoder

## Fail conditions

- chart degenerates into a dense autoencoder without explicit sparse semantics
- chart is linear-only without being labeled as a fixture or baseline
- validity is reduced to an unprincipled reconstruction threshold and nothing else

---

# 2.6 `gsae-pullback-metric`

## Purpose
Exact pointwise pullback geometry of the chart.

## Required identity

\[
g_\alpha(z) = J_{D_\alpha}(z)^\top\,G_\mathcal{M}(D_\alpha(z))\,J_{D_\alpha}(z)
\]

## Required outputs

- `g(z)`
- `g_inv(z)`
- `dg(z)`
- chart Levi-Civita connection
- optional curvature helpers

## Exit gates

### Gate A — no Euclidean substitution
There is no unmarked use of `J^T J` in core pullback code.

### Gate B — state metric is actually used
The pullback implementation depends on `gsae-state-geometry`, not on ambient identity assumptions.

### Gate C — SPD on valid chart points
Metric is SPD on the chart’s valid region.

### Gate D — derivative coherence
`dg` matches analytic or autodiff-backed expectations.

## Suggested commands

```bash
cargo test -p gsae-pullback-metric
```

## Required tests

- direct formula test against analytic chart + analytic state metric
- SPD test over valid chart samples
- symmetry of metric tensor
- derivative consistency test for `dg`

## Fail conditions

- pullback field is implemented as a constant Gram matrix by default
- target metric derivatives are ignored
- chart metric quietly assumes flat ambient geometry

---

# 2.7 `gsae-geodesics`

## Purpose
Intrinsic geometry solver layer.

## Required contents

- path energy
- path length
- geodesic IVP
- geodesic BVP
- exp/log maps
- optional parallel transport along a solved path

## Exit gates

### Gate A — geodesics are driven by the pullback field
No latent Euclidean fallback is used in the main solver path.

### Gate B — IVP solver works on analytic chart metrics
Known analytic cases produce stable solutions.

### Gate C — BVP or log/exp coherence exists
There is a tested route to intrinsic distance between two chart points.

### Gate D — reparameterization sanity
Path length behaves consistently under refinement or reparameterization tests.

## Suggested commands

```bash
cargo test -p gsae-geodesics
```

## Required tests

- zero-velocity IVP stability
- short-time exp/log consistency
- path energy/path length monotonic sanity under subdivision
- metric-based distance differs from flat latent norm on a non-Euclidean witness fixture

## Fail conditions

- “distance” is actually latent Euclidean norm
- geodesic solver is present but unused by witness crates
- solver is only tested against Euclidean fixtures

---

# 2.8 `gsae-objective`

## Purpose
Mathematical objective decomposition.

## Required contents

- reconstruction term
- sparsity term
- metric term
- gauge/isometry term
- transport term
- hypergraph interaction term

## Exit gates

### Gate A — terms are separated by object type
Loss pieces align with the theory’s primitive objects.

### Gate B — no runtime leakage
This crate does not implement optimizer orchestration.

### Gate C — witness crates can depend on individual terms without importing training machinery.

## Suggested commands

```bash
cargo test -p gsae-objective
```

## Fail conditions

- objective crate becomes a general training runtime
- mathematical losses are entangled with CLI/runtime concerns

---

## 3. Foundation phase gate

Advance to witness crates only when all foundational crates pass and the following workspace-level invariants hold.

### Foundation Gate F1 — dependency integrity
No foundational crate depends on any witness or scale-up crate.

### Foundation Gate F2 — anti-Euclidean audit
Search results for likely contamination sites are reviewed.

Suggested audit commands:

```bash
grep -R "J\.transpose().*J\|J.t().*J\|norm2\|euclid\|identity metric\|cosine" crates/
```

Any hit in foundational crates must either:
- be part of an explicitly named baseline or fixture, or
- be removed.

### Foundation Gate F3 — exact derivative path audit
Foundational code paths use exact/autodiff derivative access where required.

### Foundation Gate F4 — analytic harness coverage
At least one analytic non-Euclidean fixture spans:
- chart
- state metric
- pullback metric
- geodesic solver

---

## 4. Phase 1 gates: witness crates

# 4.1 `gsae-1-chart`

## Purpose
Witness for: local sparse chart replaces global flat basis.

## Required witness object

- one nonlinear sparse chart on one layer
- local sparse coordinates
- local reconstruction on a nontrivial valid region

## Exit gates

### Gate A — nontrivial valid domain
The witness chart has a region where validity is high and reconstruction is locally accurate.

### Gate B — explicit sparsity
The witness code is genuinely sparse, not merely low-magnitude dense.

### Gate C — witness report exists
The crate exports a witness report with at least:
- validity
- support size
- reconstruction error
- local code sample

## Suggested commands

```bash
cargo test -p gsae-1-chart
```

## Fail conditions

- witness chart is linear-only without explicit fixture labeling
- no sparse support semantics are demonstrated

---

# 4.2 `gsae-2-metric`

## Purpose
Witness for: intrinsic geometry replaces Euclidean latent distance.

## Required witness object

- one chart
- pointwise pullback field
- geodesic solver
- comparison against flat latent distance baseline

## Exit gates

### Gate A — intrinsic distance is actually computed
Witness report includes geodesic or intrinsic metric distance, not only Euclidean norm.

### Gate B — nontrivial separation from flat baseline
On a non-Euclidean fixture, intrinsic distance differs from latent `L2` in a meaningful way.

### Gate C — metric diagnostics exist
Witness report includes at least one metric diagnostic such as determinant, curvature proxy, or SPD status.

## Suggested commands

```bash
cargo test -p gsae-2-metric
```

## Fail conditions

- witness crate compares two Euclidean quantities and calls one “intrinsic”
- pointwise field exists but geodesic machinery is unused

---

# 4.3 `gsae-3-gauge`

## Purpose
Witness for: basis instability is gauge variance.

## Required witness object

- two overlapping same-layer charts
- transition diffeomorphism `tau_{alpha beta}`
- local isometry residual
- decode-commutation residual

## Exit gates

### Gate A — actual overlap exists
The two charts share a nonempty valid overlap region.

### Gate B — actual transition map exists
A diffeomorphic transition map is implemented and exercised.

### Gate C — gauge diagnostics exist
Witness report includes at least:
- overlap size or overlap validity
- local isometry residual
- inverse or cycle consistency residual
- decode commutation residual

## Suggested commands

```bash
cargo test -p gsae-3-gauge
```

## Fail conditions

- overlap is assumed rather than tested
- transition map is affine by convenience and unmarked as a baseline
- isometry is evaluated in flat latent space rather than through pullback metrics

---

# 4.4 `gsae-4-transport`

## Purpose
Witness for: cross-layer evolution is transport, not flat feature matching.

## Required witness object

- one adjacent-layer chart pair
- one transport map
- section tracking residual
- metric preservation residual

## Exit gates

### Gate A — actual adjacent-layer witness exists
Transport is implemented between two layer-local chart objects.

### Gate B — transport is not gauge reuse
The crate distinguishes same-layer chart transition from cross-layer transport.

### Gate C — comparison baseline exists
Witness report compares transport against a flat matching baseline such as cosine or direct nearest match.

### Gate D — metric preservation is measured
Transport witness reports a metric-preservation or pullback compatibility residual.

## Suggested commands

```bash
cargo test -p gsae-4-transport
```

## Fail conditions

- transport is just reused affine overlap logic
- no baseline comparison is present
- transport is not tested against metric compatibility

---

# 4.5 `gsae-5-hypergraph`

## Purpose
Witness for: semantic composition is higher-order, not additive.

## Required witness object

- one decoder hyperedge
- one nonlinear gate `rho_e`
- one ablation showing non-additive effect

## Exit gates

### Gate A — hyperedge is genuinely higher-order
The hyperedge depends on multiple coordinates.

### Gate B — gate is nonlinear
Linear reparameterization does not count.

### Gate C — ablation witness exists
Report shows a difference between ablating the hyperedge and ablating matched singleton/additive terms.

## Suggested commands

```bash
cargo test -p gsae-5-hypergraph
```

## Fail conditions

- hypergraph is just a data structure with no nonlinear decoder effect
- “hyperedge” is reducible to additive singleton terms

---

# 4.6 `gsae-6-hyperpath`

## Purpose
Witness for: mechanism is a parallel-transported hyperpath.

## Required witness object

- one transported section across adjacent layers
- one downstream hyperedge deformation
- one causal or behavioral report comparing transport+hyperedge vs isolated atoms

## Exit gates

### Gate A — transport and hypergraph are both actually used
This crate must depend on both witness types conceptually, not merely by import.

### Gate B — proposition-level report exists
Report shows that the combined explanation outperforms isolated-atom explanation on the witness fixture.

### Gate C — path semantics are explicit
The crate exposes a transported-path or mechanism-path object, not just disconnected reports.

## Suggested commands

```bash
cargo test -p gsae-6-hyperpath
```

## Fail conditions

- crate merely concatenates outputs from transport and hypergraph witnesses without a joint object
- no comparative mechanism report exists

---

## 5. Witness phase gate

Advance to scale-up crates only when all witness crates pass and the following hold.

### Witness Gate W1 — claim coverage table is green

| Claim | Witness crate | Gate |
|---|---|---|
| Local sparse chart replaces global flat basis | `gsae-1-chart` | passing |
| Intrinsic distance replaces Euclidean latent distance | `gsae-2-metric` | passing |
| Basis instability is gauge variance | `gsae-3-gauge` | passing |
| Cross-layer evolution is transport | `gsae-4-transport` | passing |
| Semantic composition is higher-order | `gsae-5-hypergraph` | passing |
| Mechanism is a parallel-transported hyperpath | `gsae-6-hyperpath` | passing |

### Witness Gate W2 — no witness crate depends on scale-up crates

Suggested check:

```bash
cargo metadata --no-deps --format-version 1
```

Review dependency graph.

### Witness Gate W3 — Euclidean baseline isolation
All Euclidean or 2D harnesses used by witness crates are explicitly labeled as fixtures, baselines, or analytic harnesses.

### Witness Gate W4 — theory sequence preserved
Witness crates build and pass in paper order:
- 1-chart
- 2-metric
- 3-gauge
- 4-transport
- 5-hypergraph
- 6-hyperpath

---

## 6. Phase 2 gates: scale-up crates

These crates are only allowed after all witness gates are green.

# 6.1 `gsae-atlas-router`

## Purpose
Routing among many charts.

## Gate
Does not change chart ontology. Only extends coverage and selection.

## Fail condition
Introduces global-basis semantics or leaks routing assumptions into `gsae-chart-core`.

---

# 6.2 `gsae-atlas-manager`

## Purpose
Registry, overlap graph, cocycle bookkeeping.

## Gate
Depends on witness-proven gauge semantics, not the other way around.

## Fail condition
Witness crates become dependent on atlas-manager for their first executable claim.

---

# 6.3 `gsae-multilayer-runtime`

## Purpose
Longer transport chains, many layers, mechanism tracing depth.

## Gate
Extends transport/hyperpath realism but introduces no new primitive.

## Fail condition
Cross-layer transport witness becomes impossible to run without this crate.

---

# 6.4 `gsae-training`

## Purpose
Optimizer/runtime orchestration.

## Gate
Uses `gsae-objective`; does not redefine math objects.

## Fail condition
Foundational crates begin depending on training runtime.

---

# 6.5 `gsae-artifacts`

## Purpose
Serialization and reproducibility.

## Gate
Artifact schemas preserve mathematical identities and witness reports.

## Fail condition
Artifact schema mutates or flattens distinct object types into ambiguous blobs.

---

# 6.6 `gsae-benchmarks`

## Purpose
Broader evaluation harness.

## Gate
Benchmarks consume witness objects; they do not define them.

## Fail condition
Benchmark shortcuts become foundational assumptions.

---

# 6.7 `gsae-cli` and `gsae-py`

## Purpose
Exposure layers.

## Gate
Only wrap existing crate APIs.

## Fail condition
Notebook or CLI ergonomics force mathematical shortcuts into foundational crates.

---

## 7. Workspace-level command sequence

Use this order when `cargo` is available locally.

### 7.1 Foundational sweep

```bash
cargo test -p gsae-core-types
cargo test -p gsae-linalg
cargo test -p gsae-state-geometry
cargo test -p gsae-autodiff
cargo test -p gsae-chart-core
cargo test -p gsae-pullback-metric
cargo test -p gsae-geodesics
cargo test -p gsae-objective
```

### 7.2 Witness sweep in theory order

```bash
cargo test -p gsae-1-chart
cargo test -p gsae-2-metric
cargo test -p gsae-3-gauge
cargo test -p gsae-4-transport
cargo test -p gsae-5-hypergraph
cargo test -p gsae-6-hyperpath
```

### 7.3 Full minimal workspace gate

```bash
cargo test --workspace
```

### 7.4 Optional audit checks

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo metadata --no-deps --format-version 1
```

---

## 8. Required PR checklist

Every PR touching foundational or witness crates must answer these explicitly.

### 8.1 Mathematical fidelity
- What definition/proposition/stage does this change implement?
- Does this introduce any new Euclidean or affine special-case assumption?
- If yes, is it baseline-labeled and isolated?

### 8.2 Dependency discipline
- Does any lower-tier crate now depend on a higher-tier crate?
- If yes, the PR is blocked.

### 8.3 Test gate effect
- Which phase gate does this PR change?
- Which downstream witness crates must be rerun?

### 8.4 Anti-degeneration audit
- Did any path replace `D^*G_M` with `J^T J`?
- Did any path replace geodesic distance with latent Euclidean norm?
- Did any path replace diffeomorphism with affine alignment?
- Did any path replace transport with cosine matching?
- Did any path replace nonlinear hyperedge interaction with additive decoding?

If any answer is yes and not explicitly baseline-scoped, the PR is blocked.

---

## 9. Definition of done for the minimal executable realization

The minimal executable realization is complete when:

1. all foundational crates pass their gates,
2. all witness crates `gsae-1-chart` through `gsae-6-hyperpath` pass in order,
3. every major claim in the note has an executable witness,
4. Euclidean or 2D machinery is quarantined to explicit baselines/fixtures,
5. no scale-up crate is required for first-witness claim coverage.

That is the build-complete state for the theory-exact minimal repo.
