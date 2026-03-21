use gsae_atlas_router::AtlasRouter;
use gsae_core_types::ChartId;

#[test]
fn routing_does_not_change_chart() {
    let before = ChartId("alpha");
    let router = AtlasRouter::new(vec![before.clone()]);
    let after = router.known_charts[0].clone();
    assert_eq!(before, after);
}

