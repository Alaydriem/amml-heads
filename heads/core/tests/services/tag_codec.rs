use heads_core::services::TagCodec;
use heads_core::structs::TagColor;

#[test]
fn round_trips_every_index() {
    for index in (0..=255u16).chain([65535]) {
        assert_eq!(TagCodec::decode(TagCodec::pack(TagCodec::encode(index))), Some(index), "index {index}");
    }
}

#[test]
fn round_trips_every_lane_value() {
    for index in 0..=65535u16 {
        assert_eq!(TagCodec::decode(TagCodec::pack(TagCodec::encode(index))), Some(index), "index {index}");
    }
}

#[test]
fn vanilla_white_is_not_a_tag() {
    assert_eq!(TagCodec::decode(TagCodec::pack(TagColor::new(1.0, 1.0, 1.0, 1.0))), None);
}

#[test]
fn translucent_tag_colour_is_not_a_tag() {
    let packed = TagCodec::pack(TagCodec::encode(5)) & 0x00FF_FFFF;
    assert_eq!(TagCodec::decode(packed), None);
}

#[test]
fn pack_matches_vanilla_rounding() {
    assert_eq!(TagCodec::pack(TagColor::new(1.0, 1.0, 1.0, 1.0)), 0xFFFF_FFFF);
    assert_eq!(TagCodec::pack(TagColor::new(0.5, 0.25, 0.999, 1.0)), 0xFF7F_3FFE);
    assert_eq!(TagCodec::pack(TagColor::new(0.0, 0.0, 0.0, 0.0)), 0x0000_0000);
}

#[test]
fn pack_clamps_out_of_range() {
    assert_eq!(TagCodec::pack(TagColor::new(-0.5, 2.0, 0.5, 1.0)), 0xFF00_FF7F);
}
