use heads_core::services::QuadScaler;
use heads_core::structs::IconRect;

#[test]
fn scale_keeps_centre() {
    let scaled = QuadScaler::scale(IconRect::new(10.0, 20.0, 7.0, 7.0), 2.0);
    assert_eq!(scaled, IconRect::new(6.5, 16.5, 14.0, 14.0));
}

#[test]
fn scale_clamps_factor() {
    let rect = IconRect::new(10.0, 20.0, 7.0, 7.0);
    assert_eq!(QuadScaler::scale(rect, 3.0), QuadScaler::scale(rect, 2.0));
    assert_eq!(QuadScaler::scale(rect, 0.5), rect);
}
