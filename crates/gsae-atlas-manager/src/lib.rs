use gsae_core_types::ChartId;

#[derive(Clone, Debug)]
pub struct AtlasManager {
    pub charts: Vec<ChartId>,
}

impl AtlasManager {
    pub fn new(charts: Vec<ChartId>) -> Self {
        Self { charts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manager_constructs() {
        let manager = AtlasManager::new(vec![ChartId("alpha")]);
        assert_eq!(manager.charts.len(), 1);
    }
}

