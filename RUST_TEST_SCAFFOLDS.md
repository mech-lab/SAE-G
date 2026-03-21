# RUST_TEST_SCAFFOLDS.md

This file defines the **full test scaffold** for the GSAE repo.

It is aligned with:
- DEVELOPMENT_GUIDE.md (anti-substitution + math fidelity)
- BUILD_GUIDE.md (phase gates)
- WITNESS_REPORT_TEMPLATES.md (report outputs)

All tests are **crate-local**, **mathematically grounded**, and **gate-enforcing**.

---

# GLOBAL TEST RULES

## Anti-Substitution Enforcement (applies everywhere)

Every crate MUST include tests that fail if:

- JᵀJ is used when G_M ≠ I
- Euclidean norm is used as intrinsic distance
- cosine similarity replaces transport
- affine maps replace diffeomorphisms
- additive decoding replaces hyperedge interaction

## Required fixture

All geometry tests MUST use at least one:

```text
NonEuclideanFixture:
  - Non-constant metric G(x)
  - Non-zero Christoffel symbols
  - Non-zero curvature (preferred)
```

---

# PHASE 0 — FOUNDATIONAL TESTS

---

## gsae-core-types/tests/core_types.rs

```rust
#[test]
fn construct_core_types() {
    let s = StatePoint::zeros(4);
    let z = CodePoint::zeros(3);
    let t = Tangent::zeros(3);
    assert_eq!(s.dim(), 4);
    assert_eq!(z.dim(), 3);
    assert_eq!(t.dim(), 3);
}

#[test]
fn deterministic_equality() {
    let a = StatePoint::zeros(5);
    let b = StatePoint::zeros(5);
    assert_eq!(a, b);
}
```

---

## gsae-linalg/tests/spd.rs

```rust
#[test]
fn spd_inverse_roundtrip() {
    let g = random_spd(4);
    let g_inv = g.inverse().unwrap();
    let ident = g * g_inv;
    assert!(approx_identity(ident));
}

#[test]
fn logdet_consistency() {
    let g = random_spd(5);
    let logdet = g.logdet();
    assert!(logdet.is_finite());
}
```

---

## gsae-state-geometry/tests/metric.rs

```rust
#[test]
fn metric_inverse_consistency() {
    let m = NonEuclideanFixture::new();
    let x = m.sample_point();

    let g = m.metric(&x);
    let g_inv = m.metric_inv(&x);

    assert!(approx_identity(g * g_inv));
}

#[test]
fn christoffel_nontrivial() {
    let m = NonEuclideanFixture::new();
    let x = m.sample_point();

    let gamma = m.christoffel(&x);

    assert!(gamma.norm() > 1e-6); // must NOT be zero
}
```

---

## gsae-autodiff/tests/derivatives.rs

```rust
#[test]
fn jacobian_matches_analytic() {
    let f = analytic_nonlinear_map();
    let x = random_input();

    let j = f.jacobian(&x);
    let j_expected = analytic_jacobian(&x);

    assert!(approx_eq(j, j_expected));
}

#[test]
fn hessian_component_exists() {
    let f = analytic_nonlinear_map();
    let x = random_input();

    let h = f.hessian_component(&x, 0);
    assert!(h.norm() > 0.0);
}
```

---

## gsae-chart-core/tests/chart.rs

```rust
#[test]
fn encode_decode_reconstruction() {
    let chart = test_chart();
    let h = sample_state();

    let z = chart.encode(&h);
    let h_hat = chart.decode(&z);

    assert!(reconstruction_error(&h, &h_hat) < 1e-3);
}

#[test]
fn sparsity_enforced() {
    let chart = test_chart();
    let h = sample_state();

    let z = chart.encode(&h);

    assert!(z.nonzero_count() < z.dim());
}

#[test]
fn jacobian_exists() {
    let chart = test_chart();
    let h = sample_state();

    let j = chart.jacobian_encode(&h);
    assert!(j.norm() > 0.0);
}
```

---

## gsae-pullback-metric/tests/pullback.rs

```rust
#[test]
fn pullback_matches_definition() {
    let chart = test_chart();
    let metric = NonEuclideanFixture::new();

    let z = sample_code();

    let j = chart.decoder_jacobian(&z);
    let g_m = metric.metric(&chart.decode(&z));

    let g_expected = j.transpose() * g_m * j;
    let g_actual = pullback_metric(&chart, &metric, &z);

    assert!(approx_eq(g_actual, g_expected));
}

#[test]
fn spd_property() {
    let g = compute_pullback(sample_code());
    assert!(is_spd(g));
}
```

---

## gsae-geodesics/tests/geodesic.rs

```rust
#[test]
fn zero_velocity_ivp() {
    let z0 = sample_code();
    let v0 = Tangent::zeros(z0.dim());

    let path = geodesic_ivp(z0.clone(), v0, 1.0);

    assert!(approx_eq(path.last().unwrap(), &z0));
}

#[test]
fn exp_log_inverse() {
    let z0 = sample_code();
    let z1 = nearby_code();

    let v = log_map(&z0, &z1);
    let z1_recovered = exp_map(&z0, &v);

    assert!(distance(z1, z1_recovered) < 1e-3);
}

#[test]
fn intrinsic_not_euclidean() {
    let z0 = sample_code();
    let z1 = nearby_code();

    let d_geo = geodesic_distance(&z0, &z1);
    let d_l2 = (z0 - z1).norm2();

    assert!((d_geo - d_l2).abs() > 1e-4);
}
```

---

## gsae-objective/tests/objective.rs

```rust
#[test]
fn objective_terms_separate() {
    let obj = build_objective();

    let loss = obj.total();
    let parts = obj.components();

    assert!(approx_eq(loss, parts.sum()));
}
```

---

# PHASE 1 — WITNESS TESTS

---

## gsae-1-chart/tests/witness_chart.rs

```rust
#[test]
fn nontrivial_domain_exists() {
    let w = chart_witness();

    assert!(w.valid_region_size() > MIN_REGION);
}

#[test]
fn sparse_codes_not_dense_noise() {
    let z = w.sample_code();

    assert!(z.nonzero_count() << z.dim());
}
```

---

## gsae-2-metric/tests/witness_metric.rs

```rust
#[test]
fn geodesic_differs_from_l2() {
    let (z0, z1) = sample_pair();

    let d_geo = w.geodesic_distance(z0, z1);
    let d_l2 = (z0 - z1).norm2();

    assert!(relative_difference(d_geo, d_l2) > THRESHOLD);
}
```

---

## gsae-3-gauge/tests/witness_gauge.rs

```rust
#[test]
fn overlap_nonempty() {
    assert!(w.overlap_size() > 0);
}

#[test]
fn transition_invertible() {
    let z = sample_overlap_point();

    let z2 = w.forward(z.clone());
    let z_back = w.inverse(z2);

    assert!(distance(z, z_back) < 1e-3);
}
```

---

## gsae-4-transport/tests/witness_transport.rs

```rust
#[test]
fn transport_beats_baseline() {
    let err_transport = w.transport_error();
    let err_baseline = w.baseline_error();

    assert!(err_transport < err_baseline);
}
```

---

## gsae-5-hypergraph/tests/witness_hypergraph.rs

```rust
#[test]
fn hyperedge_is_non_additive() {
    let delta_hyper = w.hyperedge_effect();
    let delta_linear = w.linear_approx_effect();

    assert!(delta_hyper != delta_linear);
}
```

---

## gsae-6-hyperpath/tests/witness_hyperpath.rs

```rust
#[test]
fn mechanism_requires_transport_and_hyperedge() {
    let err_full = w.full_model_error();
    let err_transport_only = w.transport_only_error();
    let err_hyper_only = w.hyper_only_error();

    assert!(err_full < err_transport_only);
    assert!(err_full < err_hyper_only);
}
```

---

# SCALE-UP NON-INTERFERENCE TESTS

---

## gsae-atlas-router/tests/router.rs

```rust
#[test]
fn routing_does_not_change_chart() {
    let before = chart_behavior();
    let after = routed_chart_behavior();

    assert_eq!(before, after);
}
```

---

## gsae-training/tests/no_math_mutation.rs

```rust
#[test]
fn objective_not_modified() {
    let obj_before = build_objective();
    train_step();
    let obj_after = build_objective();

    assert_eq!(obj_before.definition(), obj_after.definition());
}
```

---

# PROPERTY TESTS (RECOMMENDED)

* symmetry of g
* positive definiteness across samples
* exp/log local inverses
* transition cocycle (if extended)
* transport composition consistency

---

# COMMANDS

```bash
cargo test --workspace
cargo test -p gsae-3-gauge
cargo test -p gsae-4-transport
```

---

# FINAL RULE

If any test passes with:

* Euclidean fallback
* affine substitution
* cosine transport
* additive hyperedge

then the test suite is incomplete.

The test suite must **fail loudly** under those substitutions.
