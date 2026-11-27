use heads_core::services::FaceGlyphs;

fn uuid(n: usize) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&(n as u64).to_le_bytes());
    bytes
}

#[test]
fn glyphs_stop_at_240_and_a_known_player_keeps_its_place() {
    let mut glyphs = FaceGlyphs::new();
    for n in 0..FaceGlyphs::CAPACITY {
        assert!(glyphs.set(uuid(n), char::from_u32(0xE110 + n as u32).unwrap()));
    }
    assert!(!glyphs.set(uuid(240), '\u{E1FF}'));
    assert_eq!(glyphs.glyph_of(uuid(240)), None);
    assert!(glyphs.set(uuid(0), '\u{E1FE}'));
    assert_eq!(glyphs.glyph_of(uuid(0)), Some('\u{E1FE}'));
    assert_eq!(glyphs.len(), FaceGlyphs::CAPACITY);
}

#[test]
fn full_is_reported_once_and_only_after_a_refusal() {
    let mut glyphs = FaceGlyphs::new();
    for n in 0..FaceGlyphs::CAPACITY {
        glyphs.set(uuid(n), 'a');
    }
    assert!(!glyphs.full_reported());
    glyphs.set(uuid(240), 'a');
    assert!(glyphs.full_reported());
    assert!(!glyphs.full_reported());
}

#[test]
fn clear_forgets_every_player_and_opens_the_table_again() {
    let mut glyphs = FaceGlyphs::new();
    for n in 0..=FaceGlyphs::CAPACITY {
        glyphs.set(uuid(n), 'a');
    }

    glyphs.clear();

    assert!(glyphs.is_empty());
    assert_eq!(glyphs.glyph_of(uuid(0)), None);
    assert!(glyphs.set(uuid(240), 'b'));
    assert!(!glyphs.full_reported());
}
