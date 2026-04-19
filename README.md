# Geometric Sparse Autoencoder (GSAE) — Theory and Rust Implementation

## Abstractish

This repository is a **mathematically-faithful, executable witness** for the Geometric Sparse Autoencoder (GSAE) theory. It is intentionally minimal: every crate and test exists to **realize a specific mathematical object or claim**. The code is not a training runtime or an experimental stack; it is a **proof‑of‑coverage implementation** that enforces the theory’s semantics and rejects  substitutions (Euclidean shortcuts, affine reparametrizations, cosine transport, or additive hyperedges) unless explicitly labeled as baselines.

The repository is organized as:

1. **Foundational crates**: exact math primitives
2. **Witness crates**: minimal executable instantiations of each theoretical claim
3. **Scale‑up stubs**: placeholders that preserve dependency discipline without redefining core math

---

## The Theory (High‑Level)

### 1) Sparse Charts and Local Coordinates

GSAE treats representations as **local charts** on a (possibly curved) state manifold. A chart is a nonlinear encoder/decoder pair with explicit sparsity and domain validity:

- **Encoder**: \(E_\alpha: \mathcal{M} \to \mathbb{R}^d\)
- **Decoder**: \(D_\alpha: \mathbb{R}^d \to \mathcal{M}\)
- **Validity**: a chart is only trusted on its explicit local domain
- **Sparsity**: supported coordinates are part of the mathematical object, not just a heuristic

This repo includes an analytic nonlinear chart with explicit sparsity semantics and a validity function.

### 2) Pullback Geometry

The intrinsic geometry of the chart is obtained by pulling back the **state‑manifold metric** \(G_\mathcal{M}(x)\):

\[
g_\alpha(z) = J_{D_\alpha}(z)^\top \, G_\mathcal{M}(D_\alpha(z)) \, J_{D_\alpha}(z)
\]

This is **not** replaced by \(J^\top J\) unless the chosen metric is explicitly Euclidean. Derivatives of the metric include derivatives of the ambient metric field \(G_\mathcal{M}\).

### 3) Geodesics and Intrinsic Distance

Distances are defined intrinsically on the chart via geodesics:

- path energy / length are defined under \(g_\alpha\)
- geodesic IVP/BVP are solved via metric‑aware dynamics
- intrinsic distance differs meaningfully from latent Euclidean norm on a non‑Euclidean fixture

### 4) Gauge (Overlap) Semantics

Overlapping charts on the **same layer** are related by a **transition diffeomorphism**:

\[
\tau_{\alpha\beta} = \phi_\beta \circ \phi_\alpha^{-1}
\]

Gauge compatibility is expressed by metric pullback consistency on the overlap:

\[
\tau_{\alpha\beta}^* g_\beta \approx g_\alpha
\]

The witness verifies overlap, invertibility, and metric compatibility.

### 5) Transport Across Layers

Cross‑layer evolution is not the same as same‑layer gauge. GSAE models adjacent layer transport with a discrete connection/transport map and checks metric‑preservation‑like residuals.

### 6) Hypergraph Interaction

Decoder interactions can be genuinely higher‑order: hyperedges are **nonlinear**, not reducible to additive singleton contributions. The witness includes explicit nonlinear hyperedge effects and ablation tests.

### 7) Hyperpath Mechanism

The full mechanism is a **transported section** followed by **downstream hyperedge deformation**, not explained by either alone. The witness compares the combined mechanism against transport‑only and hyperedge‑only baselines.

---

## Faithful Rust Implementation

### Foundational Crates (Exact Math Substrate)

- `gsae-core-types`: shared mathematical types (points, tangents, IDs, dense aliases)
- `gsae-linalg`: matrix/tensor primitives, SPD solves, logdet, contractions
- `gsae-state-geometry`: state‑manifold metrics, Christoffels, curvature
- `gsae-autodiff`: exact derivative interfaces with diagnostics
- `gsae-chart-core`: chart interfaces, Jacobians/Hessians, validity and sparsity
- `gsae-pullback-metric`: pullback metric and its derivatives
- `gsae-geodesics`: intrinsic solver layer
- `gsae-objective`: math‑only objective decomposition

These crates enforce the **anti‑substitution rule**: no silent Euclidean shortcuts or affine replacements in core math.

### Witness Crates (Minimal Executable Claims)

- `gsae-1-chart`: nonlinear sparse chart + validity + sparsity witness
- `gsae-2-metric`: pullback metric + intrinsic geodesics vs Euclidean baseline
- `gsae-3-gauge`: same‑layer overlap + transition diffeomorphism + metric compatibility
- `gsae-4-transport`: adjacent‑layer transport + metric preservation residual
- `gsae-5-hypergraph`: nonlinear hyperedge gate + non‑additivity test
- `gsae-6-hyperpath`: transport + hyperedge combined mechanism witness

Each witness crate exposes report emission hooks (`run_witness`, `write_report`, `validate_report`) that align with the report templates and the gate protocol.

### Scale‑Up Crates (Non‑Interference Stubs)

Stubs exist for routing, orchestration, training, artifacts, benchmarks, CLI, and Python exposure. They are deliberately minimal and **must not redefine** foundational or witness semantics.

---

## Anti‑Substitution Discipline

The code and tests explicitly fail if:

- \(J^\top J\) is used where \(G_\mathcal{M}\) is non‑Euclidean
- latent Euclidean norms replace intrinsic distance
- affine overlap replaces diffeomorphic transition
- cosine similarity replaces transport
- additive decoding replaces hyperedge interaction

If a baseline uses one of these simplifications, it **must** be labeled with:

`*_baseline`, `*_fixture`, `*_toy`, `*_sanity`, or `*_analytic_harness`.

---

## Tests and Gates

This repo implements **crate‑local, gate‑enforcing** tests. The test scaffold is documented in `RUST_TEST_SCAFFOLDS.md`.

Run the full suite:

```bash
cargo test --workspace -- --nocapture
```

---

## Demo / Toy Example (End‑to‑End)

The demo runs a minimal end‑to‑end path through chart encoding, pullback geometry, intrinsic geodesics, and all six witness reports. It also writes witness report artifacts into `artifacts/demo/witness/*`.

```bash
cargo run -p gsae-demo
```

This demo will also attempt to download and run **GPT‑2 small** via a Python helper. The helper pins a specific commit for reproducibility. If you want to run it manually:

```bash
python3 scripts/gpt2_small_demo.py --out artifacts/demo/gpt2_small
# Optional override:
# python3 scripts/gpt2_small_demo.py --out artifacts/demo/gpt2_small --revision <commit-sha>
```

Dependencies for the GPT‑2 helper:

```bash
pip install torch transformers
```

---

## What This Repo Is (and Isn’t)

**Is:**
- A minimal, executable witness of the GSAE theory
- A test‑enforced, anti‑substitution implementation
- A structural base for future scale‑up

**Isn’t:**
- A training runtime
- A large‑scale experimental codebase
- A substitute for the theory itself

---

## Entry Points

To explore the theory through code:

- Start with `gsae-chart-core` and `gsae-state-geometry`
- Trace `gsae-pullback-metric` → `gsae-geodesics`
- Follow witness crates in paper order

The repository exists to make the mathematical definitions **executable and auditable**.
