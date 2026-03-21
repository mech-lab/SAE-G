# GSAE Project Development Guide

This guide exists to keep the repository mathematically faithful to the theory and to prevent quiet regression into flat, Euclidean, convenience-first implementations.

It is a development constitution for the repo generated in `PROJECT.md`.

---

## 1. Primary rule

**Implement the math that is actually defined. Do not replace it with a familiar special case, a training convenience, a proxy baseline, or a low-dimensional intuition pump unless the code marks that object explicitly as a baseline, harness, or test fixture.**

The most important failure mode is silent substitution.

Examples of silent substitution that are forbidden in core theory crates:

- replacing the state-manifold metric `G_M(x)` with the identity without an explicit theorem-level assumption
- replacing the pullback metric `D^* G_M` with `J^T J`
- replacing intrinsic geodesic distance with latent Euclidean `L2`
- replacing chart overlap diffeomorphisms with affine alignment
- replacing transport with cosine matching or nearest-neighbor correspondence
- replacing higher-order interaction with additive or pairwise-only terms
- replacing chart validity/domain semantics with a reconstruction threshold heuristic
- replacing atlas-local semantics with a single global basis or fixed global latent ontology

---

## 2. Source-of-truth hierarchy

When there is any ambiguity, the resolution order is:

1. the mathematical note / paper definitions
2. this guide
3. crate-level docs and trait contracts
4. implementation details
5. experiments, notebooks, demos, benchmarks

**Code is not allowed to redefine the theory by accident.**

If the code and the note disagree, the code is wrong unless the note is formally revised.

---

## 3. The exact mathematical objects the repo is supposed to contain

The repo reflects the theory exactly by separating:

- **Foundational crates**: mathematical substrate
- **Witness crates**: smallest non-vacuous executable object for each claim
- **Scale-up crates**: orchestration, packaging, performance, and broader coverage

### Foundational crates

- `gsae-core-types`
- `gsae-linalg`
- `gsae-state-geometry`
- `gsae-autodiff`
- `gsae-chart-core`
- `gsae-pullback-metric`
- `gsae-geodesics`
- `gsae-objective`

### Witness crates

- `gsae-1-chart`
- `gsae-2-metric`
- `gsae-3-gauge`
- `gsae-4-transport`
- `gsae-5-hypergraph`
- `gsae-6-hyperpath`

### Scale-up crates

- atlas routing / management
- multilayer runtime
- training runtime
- artifacts / serialization
- benchmarks
- CLI / Python bindings

The repo must preserve the invariant:

**Foundational -> Witness -> Scale-up**

and never the reverse.

---

## 4. Core anti-degeneration rule: no Euclidean contamination

The project must **not** quietly collapse into “Euclidean 2D with nice words around it.”

### 4.1 Allowed uses of 2D or Euclidean examples

2D or Euclidean examples are allowed only in these roles:

- analytic harnesses
- visualization fixtures
- executable sanity tests
- local counterexamples or theorem illustrations
- baseline comparisons explicitly labeled as such

### 4.2 Forbidden uses of 2D or Euclidean examples

2D or Euclidean examples must **not** become:

- the implicit ontology of the repo
- the default target state geometry in foundational crates
- the assumed metric for pullback operations
- the default notion of distance in witness crates
- the conceptual model used in crate docs to define the actual theory

### 4.3 Required labeling rule

Any code path that uses a Euclidean or 2D simplification must be named and documented as one of:

- `*_baseline`
- `*_fixture`
- `*_toy`
- `*_sanity`
- `*_analytic_harness`

It must never be presented as the core object.

---

## 5. Foundational crate contracts

Each foundational crate has a mathematical burden. That burden is not optional.

### 5.1 `gsae-core-types`

Purpose:

- shared ontology across the whole workspace

Must contain only stable mathematical data types and identities:

- `StatePoint`
- `CodePoint`
- `Tangent`
- `ChartId`
- `LayerId`
- `HyperedgeId`
- `MetricTensor` or equivalent matrix/tensor wrappers

Must not contain:

- training convenience helpers
- benchmark-specific labels
- routing policies
- notebook-oriented formatting logic

### 5.2 `gsae-linalg`

Purpose:

- exact numerical substrate for differential geometry

Must contain:

- SPD factorizations
- stable solves
- tensor contractions
- determinant / logdet
- local matrix/tensor routines needed by geometry

Must not contain:

- geometry semantics
- chart assumptions
- Euclidean defaults hidden behind convenience constructors

### 5.3 `gsae-state-geometry`

Purpose:

- target manifold geometry

This crate is where the state-manifold metric lives.

Core trait should look like:

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

Non-negotiable rules:

- the pullback metric may only be computed through this crate’s metric object
- no foundational crate may assume `G_M(x) = I` unless the specific `StateMetric` instance says so
- target-space covariant derivatives must come from `StateMetric`, not from Euclidean shortcuts

### 5.4 `gsae-autodiff`

Purpose:

- exact derivative access for encoder, decoder, and state metric when required

Must support, at minimum:

- Jacobians
- Hessians
- derivative consistency checks

Must not let finite-difference fallback silently become the permanent production path in theory crates.

Finite differences may exist only as:

- test oracles
- debugging tools
- fallback diagnostics explicitly marked as approximate

### 5.5 `gsae-chart-core`

Purpose:

- the sparse chart primitive

A chart is not “an SAE with a nicer name.” It is the local object:

- nonlinear encoder `E_alpha`
- nonlinear decoder `D_alpha`
- explicit sparsifier
- explicit chart validity / domain semantics
- inverse-consistency diagnostics

Required trait boundary:

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

Forbidden chart degradations:

- linear encoder-decoder treated as the default chart ontology
- validity reduced to one reconstruction threshold without explicit theory justification
- support size used as the only notion of sparse semantics
- global chart assumptions hidden in APIs

### 5.6 `gsae-pullback-metric`

Purpose:

- exact chart-local pullback geometry

This crate must compute:

```math
g_alpha(z) = J_{D_alpha}(z)^T G_M(D_alpha(z)) J_{D_alpha}(z)
```

and **not** default to:

```math
J^T J
```

unless that is the explicit special case induced by the chosen `StateMetric`.

Required operations:

- `g(z)`
- `g^{-1}(z)`
- `∂g(z)`
- chart Levi-Civita connection
- curvature helpers
- tangent projector using `G_M`
- normal projector using `G_M`

Required formula discipline:

- derivative terms must include derivatives of the target metric field, not only decoder Hessian terms
- second fundamental form must use the target Levi-Civita connection, not raw Euclidean second derivatives
- any Gauss-equation route must include ambient/state curvature unless the chosen state metric is actually flat

### 5.7 `gsae-geodesics`

Purpose:

- intrinsic geometry on chart coordinates

Must implement:

- path energy
- path length
- geodesic IVP
- geodesic BVP
- exponential map
- logarithm map
- intrinsic distance
- parallel transport along solved paths

Forbidden degradations:

- latent Euclidean norm as the default distance
- straight-line interpolation relabeled as geodesic
- nearest-neighbor path approximation silently substituting for a geodesic solver

Approximate solvers are allowed only if the approximation is solver-level, not ontology-level.

### 5.8 `gsae-objective`

Purpose:

- unified mathematical loss decomposition, not training runtime

Must encode the math-only objective pieces:

- reconstruction
- sparsity
- metric regularity
- gauge/isometry residual
- transport residual
- hypergraph interaction residual

Must not absorb:

- optimizer choice
- checkpoint logic
- dataloader orchestration
- experiment bookkeeping

---

## 6. Witness crate contracts

Each witness crate must instantiate the **smallest non-vacuous object** for one claim.

### 6.1 `gsae-1-chart`

Witnesses:

- one nonlinear sparse chart on one layer

Must show:

- local sparse coordinates
- local reconstruction
- nontrivial validity domain

Must not cheat by:

- using a linear chart as the primary witness
- using a dense autoencoder and calling it sparse because of small coefficients

### 6.2 `gsae-2-metric`

Witnesses:

- one chart with pointwise pullback metric and geodesic solver

Must show:

- intrinsic geometry differs from flat latent `L2`
- geodesic machinery is operational
- the metric field is genuinely pointwise, not constant by default

Must not cheat by:

- substituting `J^T J` without an explicit state-metric assumption
- comparing only local quadratic forms while calling it geodesic distance

### 6.3 `gsae-3-gauge`

Witnesses:

- two overlapping same-layer charts
- transition diffeomorphism on the overlap

Must show:

- actual overlap
- an invertible transition map
- metric pullback compatibility
- decoding commutation or equivalent semantic consistency

Must not cheat by:

- using an affine regression and calling it a transition diffeomorphism
- comparing charts with no overlap
- using cross-layer maps to witness gauge

### 6.4 `gsae-4-transport`

Witnesses:

- one adjacent-layer chart pair
- one transport map / discrete connection

Must show:

- cross-layer movement of a feature section
- metric-preserving or connection-consistent transport behavior

Must not cheat by:

- using cosine matching or nearest-neighbor matching and renaming it transport
- using same-layer overlap to witness cross-layer transport

### 6.5 `gsae-5-hypergraph`

Witnesses:

- one decoder hyperedge
- one nonlinear interaction gate

Must show:

- an effect not reducible to additive singleton features

Must not cheat by:

- representing a hyperedge but evaluating only additive terms
- using a linear gate and calling it higher-order interaction

### 6.6 `gsae-6-hyperpath`

Witnesses:

- one transported section across adjacent layers
- one downstream hyperedge deformation

Must show:

- a mechanism better explained by transport + hyperedge than by isolated atoms

Must not cheat by:

- using transport alone
- using hypergraph interaction alone
- omitting the actual downstream deformation step

---

## 7. Scale-up crate rules

Scale-up crates are allowed to improve:

- coverage
- ergonomics
- orchestration
- speed
- serialization
- reproducibility
- benchmark reach

They are **not** allowed to redefine any mathematical object from lower layers.

That means:

- `atlas-router` must not redefine what a chart is
- `atlas-manager` must not redefine what overlap means
- `training` must not redefine what the objective is
- `cli` and `py` must not introduce convenience defaults that alter mathematical semantics

### Downward-leak prohibition

These dependencies are forbidden:

- foundational crates depending on scale-up crates
- witness crates depending on atlas-manager or runtime when not strictly required
- any theory crate depending on benchmark-specific assumptions

---

## 8. Explicit formulas that must remain exact

These formulas should appear in crate docs and implementation comments in near-verbatim form.

### 8.1 Pullback metric

```math
g_alpha(z) = J_{D_alpha}(z)^T G_M(D_alpha(z)) J_{D_alpha}(z)
```

### 8.2 Metric derivative

```math
∂_k g_{ij}
=
(∂_{ki} D^a) G_{ab} (∂_j D^b)
+
(∂_i D^a) G_{ab} (∂_{kj} D^b)
+
(∂_i D^a) (∂_c G_{ab})(D(z)) (∂_k D^c) (∂_j D^b)
```

### 8.3 Chart Levi-Civita connection

```math
Γ^k_{ij}
=
\frac{1}{2} g^{k\ell}
(∂_i g_{j\ell} + ∂_j g_{i\ell} - ∂_\ell g_{ij})
```

### 8.4 Intrinsic distance

```math
d_g(z_0, z_1) = \inf_{\gamma(0)=z_0,\gamma(1)=z_1} L(\gamma)
```

### 8.5 Gauge transition

```math
τ_{αβ} = φ_β \circ φ_α^{-1}
```

### 8.6 Metric compatibility of overlap map

```math
τ_{αβ}^* g_β \approx g_α
```

### 8.7 Transport compatibility

Use the precise transport law defined in the theory. Do not replace it with feature similarity matching.

### 8.8 Hyperedge interaction

The hyperedge contribution must be genuinely nonlinear in the participating coordinates.

---

## 9. Review checklist for every PR

Every PR should answer all of these.

### 9.1 Object identity

- What exact mathematical object from the theory is this code implementing?
- In what crate does that object belong?
- Is the object foundational, witness, or scale-up?

### 9.2 Assumption disclosure

- What assumptions are introduced here?
- Are they theorem-level assumptions, solver assumptions, or harness assumptions?
- Are any of them special cases of the actual theory?
- If yes, are they explicitly labeled as such?

### 9.3 Anti-substitution check

- Does this PR replace a general object with a familiar special case?
- Does it default to Euclidean identity anywhere?
- Does it smuggle in affine or additive structure where the theory requires diffeomorphic or higher-order structure?
- Does it turn an intrinsic object into an extrinsic convenience?

### 9.4 Layer discipline
n
- Does a scale-up crate depend downward on a witness or foundational crate only?
- Does a foundational crate stay free of router/runtime/CLI logic?
- Does a witness crate instantiate a claim without dragging in global orchestration?

### 9.5 Naming discipline

- Are 2D or Euclidean fixtures clearly named as baselines or harnesses?
- Are core implementations named in a way that implies full theory semantics only when that is actually true?

### 9.6 Test discipline

- What claim does this test witness?
- What formula is the test checking?
- Could the test still pass if the implementation had quietly collapsed to a Euclidean special case?
- If yes, the test is not strong enough.

---

## 10. Required test families

### 10.1 Foundational invariant tests

For foundational crates, require:

- SPD checks for metric tensors on valid points
- symmetry checks
- inverse consistency checks
- Christoffel torsion-free checks
- metric compatibility checks where applicable
- projector idempotence and range checks

### 10.2 Pullback-specific consistency tests

Require at least:

- `g = J^T G J` exactly for the chosen state metric
- derivative checks against autodiff or analytic oracle
- tangent projector uses `G`, not Euclidean identity
- second fundamental form uses target connection when the target metric is non-flat

### 10.3 Geodesic tests

Require at least:

- energy/length consistency on solved geodesics
- exp/log approximate inversion in valid neighborhoods
- BVP endpoint residual checks
- parallel transport norm preservation under the chart metric

### 10.4 Gauge tests

Require at least:

- overlap non-emptiness
- transition invertibility
- local metric pullback compatibility
- decoding commutation residual
- cocycle tests once triple overlaps are added

### 10.5 Transport tests

Require at least:

- adjacent-layer section tracking
- residual comparison against flat matching baseline
- transport consistency with local chart geometry

### 10.6 Hypergraph tests

Require at least:

- ablation of the hyperedge changes output in a way that singleton ablations cannot match
- nonlinear gate actually changes the composition law

### 10.7 Hyperpath tests

Require at least:

- transported section plus downstream hyperedge outperforms atomistic explanation

---

## 11. Development workflow

### Step 1: identify the theorem object

Before writing code, write one sentence:

- “This PR implements Definition X / Proposition Y / Stage Z witness.”

### Step 2: write the exact formula contract

Add the exact formula in the module docs before implementation.

### Step 3: write the forbidden simplifications list

For the module, list what would count as invalid substitution.

### Step 4: implement the narrowest exact version

Prefer a narrow but exact implementation over a broad but drifted one.

### Step 5: add invariant tests first

Write the tests that would fail if the code silently collapsed to a Euclidean or additive special case.

### Step 6: only then add harnesses and baselines

Harnesses are allowed after the exact object exists.

---

## 12. Documentation style rules

Every foundational and witness crate should begin with:

1. the mathematical object it implements
2. the exact formula or definition
3. the assumptions it takes as inputs
4. the forbidden substitutions
5. the specific claim or witness role it supports

Avoid vague prose like:

- “metric-like”
- “geometry-inspired”
- “approximately transport-like”
- “hypergraph-style synergy”

Use exact statements instead.

---

## 13. What the current `PROJECT.md` should be read as

The current flattened `PROJECT.md` is a **theory-shaped minimal executable realization**.

That means:

- it is correctly organized by witness coverage
- its crate sequence mirrors the paper sequence
- it is the right structural base for the theory

But any analytic low-dimensional fixtures inside it must be treated as:

- witness harnesses
- sanity fixtures
- executable proof-of-coverage artifacts

not as the final ontology of the theory.

If a contributor starts treating a 2D witness harness as the actual semantic manifold model, the project has already drifted.

---

## 14. Merge gate: red-flag phrases

If any PR description, comment, or code doc relies on language like this in a core crate, block it until clarified:

- “for simplicity, assume Euclidean”
- “we can just use `J^T J` here”
- “use straight-line interpolation as a geodesic approximation”
- “we can fit an affine map for the gauge transform”
- “cosine similarity is good enough for transport”
- “represent hyperedges as summed pairwise terms”
- “use reconstruction error as the chart domain”
- “just visualize it in 2D and treat that as the geometry”

Those may be fine for explicit baselines, but never as silent defaults in core theory code.

---

## 15. One-line constitution

**The repo exists to implement the theory exactly enough that each major claim has a non-vacuous executable witness, and every simplification must remain visibly a simplification rather than becoming the ontology by accident.**
