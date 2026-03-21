use gsae_1_chart::{run_witness as run_chart, write_report as write_chart};
use gsae_2_metric::{run_witness as run_metric, write_report as write_metric};
use gsae_3_gauge::{run_witness as run_gauge, write_report as write_gauge};
use gsae_4_transport::{run_witness as run_transport, write_report as write_transport};
use gsae_5_hypergraph::{run_witness as run_hypergraph, write_report as write_hypergraph};
use gsae_6_hyperpath::{run_witness as run_hyperpath, write_report as write_hyperpath};
use gsae_chart_core::{default_chart, SparseChart};
use gsae_core_types::{CodePoint, StatePoint};
use gsae_geodesics::{GeodesicSolver, ShootingGeodesics};
use gsae_pullback_metric::ChartMetric;
use gsae_state_geometry::ConformalPlaneMetric;
use std::fs;
use std::path::Path;
use std::process::Command;

fn report_dir(crate_name: &str) -> std::path::PathBuf {
    Path::new("artifacts").join("demo").join("witness").join(crate_name)
}

fn run_gpt2_demo() {
    let out_dir = Path::new("artifacts").join("demo").join("gpt2_small");
    let status = Command::new("python3")
        .arg("scripts/gpt2_small_demo.py")
        .arg("--out")
        .arg(&out_dir)
        .status()
        .or_else(|_| {
            Command::new("python")
                .arg("scripts/gpt2_small_demo.py")
                .arg("--out")
                .arg(&out_dir)
                .status()
        });

    match status {
        Ok(s) if s.success() => {
            println!("gpt2 demo written to artifacts/demo/gpt2_small/gpt2_small_demo.json");
        }
        Ok(s) => {
            println!("gpt2 demo failed with status: {}", s);
        }
        Err(e) => {
            println!("gpt2 demo not run: {}", e);
        }
    }
}

struct FrozenModel {
    chart_c: f64,
    chart_gamma: f64,
    metric_k: f64,
    gauge_sigma: f64,
    transport_sigma: f64,
}

impl FrozenModel {
    fn new() -> Self {
        let chart = default_chart();
        Self {
            chart_c: chart.c,
            chart_gamma: chart.gamma,
            metric_k: 0.6,
            gauge_sigma: 0.35,
            transport_sigma: 0.2,
        }
    }

    fn encode_decode(&self, x: &StatePoint) -> (CodePoint, StatePoint) {
        let chart = default_chart();
        let z = chart.encode(x);
        let x_hat = chart.decode(&z);
        (z, x_hat)
    }
}

fn write_frozen_model_artifact(model: &FrozenModel, out_dir: &Path) {
    fs::create_dir_all(out_dir).expect("create demo artifacts");
    let sample_points = vec![
        StatePoint::new(vec![0.2, model.chart_c * 0.2 * 0.2]),
        StatePoint::new(vec![0.4, model.chart_c * 0.4 * 0.4]),
        StatePoint::new(vec![0.6, model.chart_c * 0.6 * 0.6]),
    ];
    let mut samples = Vec::new();
    for x in sample_points {
        let (z, x_hat) = model.encode_decode(&x);
        let recon = ((x_hat.data[0] - x.data[0]).powi(2) + (x_hat.data[1] - x.data[1]).powi(2)).sqrt();
        samples.push((x.data, z.data, x_hat.data, recon));
    }
    let samples_json = samples
        .into_iter()
        .map(|(x, z, x_hat, recon)| {
            format!(
                r#"{{"x":[{},{}],"z":[{},{}],"x_hat":[{},{}],"recon":{}}}"#,
                x[0], x[1], z[0], z[1], x_hat[0], x_hat[1], recon
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let json = format!(
        r#"{{"frozen_model":{{"chart_c":{},"chart_gamma":{},"metric_k":{},"gauge_sigma":{},"transport_sigma":{}}},"samples":[{}]}}"#,
        model.chart_c,
        model.chart_gamma,
        model.metric_k,
        model.gauge_sigma,
        model.transport_sigma,
        samples_json
    );
    fs::write(out_dir.join("frozen_model.json"), json).expect("write frozen model");
}

fn main() {
    let chart = default_chart();
    let x = StatePoint::new(vec![0.2, chart.c * 0.2 * 0.2]);
    let z = chart.encode(&x);

    let metric = ConformalPlaneMetric { k: 0.6 };
    let field = ChartMetric::new(chart.clone(), metric);
    let solver = ShootingGeodesics { field };
    let z1 = CodePoint::new(vec![0.5, 0.0]);
    let d_geo = solver.distance(&z, &z1, 0.01, 12);

    println!("GSAE demo");
    println!("chart encode z = {:?}", z.data);
    println!("geodesic distance to z1 = {}", d_geo);

    let frozen = FrozenModel::new();
    write_frozen_model_artifact(&frozen, Path::new("artifacts").join("demo").as_path());
    println!("frozen model artifact written to artifacts/demo/frozen_model.json");
    run_gpt2_demo();

    let chart_run = run_chart();
    let metric_run = run_metric();
    let gauge_run = run_gauge();
    let transport_run = run_transport();
    let hypergraph_run = run_hypergraph();
    let hyperpath_run = run_hyperpath();

    let dir = report_dir("gsae-1-chart");
    fs::create_dir_all(&dir).expect("create report dir");
    write_chart(&chart_run, &dir).expect("write report");

    let dir = report_dir("gsae-2-metric");
    fs::create_dir_all(&dir).expect("create report dir");
    write_metric(&metric_run, &dir).expect("write report");

    let dir = report_dir("gsae-3-gauge");
    fs::create_dir_all(&dir).expect("create report dir");
    write_gauge(&gauge_run, &dir).expect("write report");

    let dir = report_dir("gsae-4-transport");
    fs::create_dir_all(&dir).expect("create report dir");
    write_transport(&transport_run, &dir).expect("write report");

    let dir = report_dir("gsae-5-hypergraph");
    fs::create_dir_all(&dir).expect("create report dir");
    write_hypergraph(&hypergraph_run, &dir).expect("write report");

    let dir = report_dir("gsae-6-hyperpath");
    fs::create_dir_all(&dir).expect("create report dir");
    write_hyperpath(&hyperpath_run, &dir).expect("write report");

    println!("witness reports written to artifacts/demo/witness/*");
}
