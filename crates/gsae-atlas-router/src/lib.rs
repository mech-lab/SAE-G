use gsae_core_types::ChartId;

#[derive(Clone, Debug)]
pub struct AtlasRouter {
    pub known_charts: Vec<ChartId>,
}

impl AtlasRouter {
    pub fn new(known_charts: Vec<ChartId>) -> Self {
        Self { known_charts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_constructs() {
        let router = AtlasRouter::new(vec![ChartId("alpha")]);
        assert_eq!(router.known_charts.len(), 1);
    }
}

