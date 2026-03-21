use gsae_core_types::LayerId;

#[derive(Clone, Debug)]
pub struct MultilayerRuntime {
    pub layers: Vec<LayerId>,
}

impl MultilayerRuntime {
    pub fn new(layers: Vec<LayerId>) -> Self {
        Self { layers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_constructs() {
        let runtime = MultilayerRuntime::new(vec![LayerId(0), LayerId(1)]);
        assert_eq!(runtime.layers.len(), 2);
    }
}

