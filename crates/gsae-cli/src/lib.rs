#[derive(Clone, Debug)]
pub struct CliEntry {
    pub args: Vec<String>,
}

impl CliEntry {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_constructs() {
        let cli = CliEntry::new(vec!["--help".to_string()]);
        assert_eq!(cli.args.len(), 1);
    }
}

