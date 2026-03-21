use gsae_core_types::{CodePoint, Scalar, Tangent};
use gsae_linalg::{norm2, solve_2x2, vec_add, vec_scale, vec_sub, Matrix};
use gsae_pullback_metric::PullbackMetricField;

pub trait GeodesicSolver {
    fn path_energy(&self, path: &[CodePoint]) -> Scalar;
    fn path_length(&self, path: &[CodePoint]) -> Scalar;
    fn geodesic_ivp(&self, z0: &CodePoint, v0: &Tangent, t1: Scalar, dt: Scalar) -> Vec<CodePoint>;
    fn exp_map(&self, z0: &CodePoint, v0: &Tangent, dt: Scalar) -> CodePoint;
    fn log_map(&self, z0: &CodePoint, z1: &CodePoint, dt: Scalar, max_iter: usize) -> Tangent;
    fn geodesic_bvp(&self, z0: &CodePoint, z1: &CodePoint, dt: Scalar, max_iter: usize) -> Vec<CodePoint>;
    fn distance(&self, z0: &CodePoint, z1: &CodePoint, dt: Scalar, max_iter: usize) -> Scalar;
}

#[derive(Clone)]
pub struct ShootingGeodesics<F> {
    pub field: F,
}

impl<F: PullbackMetricField + Clone> ShootingGeodesics<F> {
    fn acceleration(&self, z: &CodePoint, v: &Tangent) -> Tangent {
        let gamma = self.field.christoffel(z);
        let mut out = vec![0.0; 2];
        for k in 0..2 {
            let mut s = 0.0;
            for i in 0..2 {
                for j in 0..2 {
                    s += gamma[(k, i, j)] * v.data[i] * v.data[j];
                }
            }
            out[k] = -s;
        }
        Tangent::new(out)
    }

    fn rk4_step(&self, z: &CodePoint, v: &Tangent, dt: Scalar) -> (CodePoint, Tangent) {
        let a1 = self.acceleration(z, v);
        let z2 = CodePoint::new(vec_add(&z.data, &vec_scale(&v.data, 0.5 * dt)));
        let v2 = Tangent::new(vec_add(&v.data, &vec_scale(&a1.data, 0.5 * dt)));
        let a2 = self.acceleration(&z2, &v2);

        let z3 = CodePoint::new(vec_add(&z.data, &vec_scale(&v2.data, 0.5 * dt)));
        let v3 = Tangent::new(vec_add(&v.data, &vec_scale(&a2.data, 0.5 * dt)));
        let a3 = self.acceleration(&z3, &v3);

        let z4 = CodePoint::new(vec_add(&z.data, &vec_scale(&v3.data, dt)));
        let v4 = Tangent::new(vec_add(&v.data, &vec_scale(&a3.data, dt)));
        let a4 = self.acceleration(&z4, &v4);

        let mut z_next = vec![0.0; 2];
        let mut v_next = vec![0.0; 2];
        for i in 0..2 {
            z_next[i] = z.data[i] + dt * (v.data[i] + 2.0 * v2.data[i] + 2.0 * v3.data[i] + v4.data[i]) / 6.0;
            v_next[i] = v.data[i] + dt * (a1.data[i] + 2.0 * a2.data[i] + 2.0 * a3.data[i] + a4.data[i]) / 6.0;
        }
        (CodePoint::new(z_next), Tangent::new(v_next))
    }

    fn endpoint_jacobian(&self, z0: &CodePoint, v0: &Tangent, dt: Scalar) -> Matrix {
        let eps = 1e-5;
        let mut j = Matrix::zeros(2, 2);
        for i in 0..2 {
            let mut vp = v0.clone();
            let mut vm = v0.clone();
            vp.data[i] += eps;
            vm.data[i] -= eps;
            let ep = self.exp_map(z0, &vp, dt);
            let em = self.exp_map(z0, &vm, dt);
            j[(0, i)] = (ep.data[0] - em.data[0]) / (2.0 * eps);
            j[(1, i)] = (ep.data[1] - em.data[1]) / (2.0 * eps);
        }
        j
    }
}

impl<F: PullbackMetricField + Clone> GeodesicSolver for ShootingGeodesics<F> {
    fn path_energy(&self, path: &[CodePoint]) -> Scalar {
        if path.len() < 2 { return 0.0; }
        let dt = 1.0 / ((path.len() - 1) as Scalar);
        let mut total = 0.0;
        for pair in path.windows(2) {
            let z = &pair[0];
            let dz = Tangent::new(vec_scale(&vec_sub(&pair[1].data, &pair[0].data), 1.0 / dt));
            total += self.field.metric_inner(z, &dz, &dz) * dt;
        }
        total
    }

    fn path_length(&self, path: &[CodePoint]) -> Scalar {
        if path.len() < 2 { return 0.0; }
        let dt = 1.0 / ((path.len() - 1) as Scalar);
        let mut total = 0.0;
        for pair in path.windows(2) {
            let z = &pair[0];
            let dz = Tangent::new(vec_scale(&vec_sub(&pair[1].data, &pair[0].data), 1.0 / dt));
            total += self.field.metric_inner(z, &dz, &dz).sqrt() * dt;
        }
        total
    }

    fn geodesic_ivp(&self, z0: &CodePoint, v0: &Tangent, t1: Scalar, dt: Scalar) -> Vec<CodePoint> {
        let steps = (t1 / dt).ceil() as usize;
        let mut z = z0.clone();
        let mut v = v0.clone();
        let mut path = Vec::with_capacity(steps + 1);
        path.push(z.clone());
        for _ in 0..steps {
            let (zn, vn) = self.rk4_step(&z, &v, dt);
            z = zn;
            v = vn;
            path.push(z.clone());
        }
        path
    }

    fn exp_map(&self, z0: &CodePoint, v0: &Tangent, dt: Scalar) -> CodePoint {
        self.geodesic_ivp(z0, v0, 1.0, dt).last().unwrap().clone()
    }

    fn log_map(&self, z0: &CodePoint, z1: &CodePoint, dt: Scalar, max_iter: usize) -> Tangent {
        let mut v = Tangent::new(vec_sub(&z1.data, &z0.data));
        for _ in 0..max_iter {
            let end = self.exp_map(z0, &v, dt);
            let residual = vec_sub(&end.data, &z1.data);
            if norm2(&residual) < 1e-8 {
                return v;
            }
            let j = self.endpoint_jacobian(z0, &v, dt);
            let delta = solve_2x2(&j, &residual);
            v = Tangent::new(vec_sub(&v.data, &delta));
        }
        v
    }

    fn geodesic_bvp(&self, z0: &CodePoint, z1: &CodePoint, dt: Scalar, max_iter: usize) -> Vec<CodePoint> {
        let v0 = self.log_map(z0, z1, dt, max_iter);
        self.geodesic_ivp(z0, &v0, 1.0, dt)
    }

    fn distance(&self, z0: &CodePoint, z1: &CodePoint, dt: Scalar, max_iter: usize) -> Scalar {
        let path = self.geodesic_bvp(z0, z1, dt, max_iter);
        self.path_length(&path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gsae_chart_core::default_chart;
    use gsae_pullback_metric::ChartMetric;
    use gsae_state_geometry::ConformalPlaneMetric;

    #[test]
    fn zero_velocity_ivp_stays_fixed() {
        let chart = default_chart();
        let metric = ConformalPlaneMetric { k: 0.6 };
        let field = ChartMetric::new(chart, metric);
        let solver = ShootingGeodesics { field };
        let z0 = CodePoint::new(vec![0.1, 0.0]);
        let v0 = Tangent::new(vec![0.0, 0.0]);
        let path = solver.geodesic_ivp(&z0, &v0, 1.0, 0.05);
        for z in path {
            assert!((z.data[0] - z0.data[0]).abs() < 1e-10);
            assert!((z.data[1] - z0.data[1]).abs() < 1e-10);
        }
    }

    #[test]
    fn exp_log_local_inverse() {
        let chart = default_chart();
        let metric = ConformalPlaneMetric { k: 0.6 };
        let field = ChartMetric::new(chart, metric);
        let solver = ShootingGeodesics { field };
        let z0 = CodePoint::new(vec![0.2, 0.0]);
        let v0 = Tangent::new(vec![0.1, 0.02]);
        let z1 = solver.exp_map(&z0, &v0, 0.02);
        let v1 = solver.log_map(&z0, &z1, 0.02, 10);
        assert!((v0.data[0] - v1.data[0]).abs() < 1e-3);
        assert!((v0.data[1] - v1.data[1]).abs() < 1e-3);
    }

    #[test]
    fn intrinsic_distance_differs_from_latent_l2() {
        let chart = default_chart();
        let metric = ConformalPlaneMetric { k: 0.6 };
        let field = ChartMetric::new(chart, metric);
        let solver = ShootingGeodesics { field: field.clone() };
        let z0 = CodePoint::new(vec![0.0, 0.0]);
        let z1 = CodePoint::new(vec![0.5, 0.0]);
        let d_geo = solver.distance(&z0, &z1, 0.01, 12);
        let d_l2 = norm2(&[z1.data[0] - z0.data[0], z1.data[1] - z0.data[1]]);
        assert!((d_geo - d_l2).abs() > 1e-3);
    }
}
