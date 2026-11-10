use std::sync::Arc;

use heads_core::services::{CellFlags, GlyphCells};
use heads_core::structs::GlyphCell;

fn uuid(n: usize) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&(n as u64).to_le_bytes());
    bytes
}

fn cells() -> GlyphCells {
    GlyphCells::new(Arc::new(CellFlags::new()))
}

#[test]
fn cells_stop_at_240_and_never_move() {
    let mut cells = cells();
    for n in 0..GlyphCells::CAPACITY {
        assert_eq!(cells.cell_for(uuid(n)), Some(GlyphCell(n as u8)));
    }
    assert_eq!(cells.cell_for(uuid(240)), None);
    assert_eq!(cells.cell_for(uuid(0)), Some(GlyphCell(0)));
    assert_eq!(cells.cell_of(uuid(240)), None);
    assert!(cells.full_reported());
    assert!(!cells.full_reported());
}

#[test]
fn full_is_not_reported_before_a_refusal() {
    let mut cells = cells();
    for n in 0..GlyphCells::CAPACITY {
        cells.cell_for(uuid(n));
    }
    assert!(!cells.full_reported());
}

#[test]
fn a_cell_has_a_face_only_after_mark_face() {
    let mut cells = cells();
    let cell = cells.cell_for(uuid(0)).unwrap();
    assert!(!cells.has_face(0xE110));
    cells.mark_face(cell);
    assert!(cells.has_face(0xE110));
    assert!(!cells.has_face(0xE111));
}

#[test]
fn has_face_is_false_outside_the_face_range() {
    let mut cells = cells();
    for n in 0..GlyphCells::CAPACITY {
        let cell = cells.cell_for(uuid(n)).unwrap();
        cells.mark_face(cell);
    }
    assert!(cells.has_face(0xE1FF));
    for outside in [0xE10F, 0xE200, 'A' as u32] {
        assert!(!cells.has_face(outside), "{outside:#x}");
    }
}

#[test]
fn codepoint_round_trips_for_every_cell() {
    for index in 0..GlyphCells::CAPACITY {
        let cell = GlyphCell(index as u8);
        assert_eq!(GlyphCell::from_codepoint(cell.codepoint() as u32), Some(cell));
    }
}
