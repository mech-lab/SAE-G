#[derive(Clone, Debug)]
pub struct PyBindings {
    pub enabled: bool,
}

impl PyBindings {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_construct() {
        let bindings = PyBindings::new(true);
        assert!(bindings.enabled);
    }
}

