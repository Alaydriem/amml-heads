use heads_core::config::HeadsSettings;

#[test]
fn settings_default_and_clamp() {
    let defaults = HeadsSettings::default();
    assert!(defaults.enabled);
    assert_eq!(defaults.scale(), 1.0);

    assert!(defaults.chat);

    let without_chat = serde_json::from_str::<HeadsSettings>(r#"{"enabled": true}"#).unwrap();
    assert!(without_chat.chat);

    let too_big = HeadsSettings::new(true, 5.0, true);
    assert_eq!(too_big.scale(), 2.0);
    let too_small = HeadsSettings::new(true, 0.25, true);
    assert_eq!(too_small.scale(), 1.0);
    let inside = HeadsSettings::new(false, 1.5, true);
    assert_eq!(inside.scale(), 1.5);
}

#[test]
fn a_scale_that_is_not_finite_is_the_smallest_scale() {
    for scale in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(HeadsSettings::new(true, scale, true).scale(), 1.0);
    }
}
