#[derive(Clone, Debug)]
pub struct BenchmarkSuite {
    pub names: Vec<String>,
}

impl BenchmarkSuite {
    pub fn new(names: Vec<String>) -> Self {
        Self { names }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn benchmark_suite_constructs() {
        let suite = BenchmarkSuite::new(vec!["default".to_string()]);
        assert_eq!(suite.names.len(), 1);
    }
}

