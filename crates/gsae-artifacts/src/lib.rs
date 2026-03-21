#[derive(Clone, Debug)]
pub struct ArtifactRegistry {
    pub entries: Vec<String>,
}

impl ArtifactRegistry {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_constructs() {
        let registry = ArtifactRegistry::new();
        assert!(registry.entries.is_empty());
    }
}

