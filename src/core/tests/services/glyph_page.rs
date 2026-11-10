use heads_core::services::GlyphPage;
use heads_core::structs::{FaceImage, GlyphCell};

const ROW_BYTES: usize = 256 * 4;

fn row0() -> Vec<u8> {
    (0..ROW_BYTES * 16).map(|n| (n % 251) as u8 | 1).collect()
}

fn face() -> FaceImage {
    let mut pixels = [0u8; 4096];
    for y in 0..32u32 {
        for x in 0..32u32 {
            let at = ((y * 32 + x) * 4) as usize;
            pixels[at] = x as u8;
            pixels[at + 1] = y as u8;
            pixels[at + 2] = 7;
            pixels[at + 3] = 255;
        }
    }
    FaceImage::new(pixels)
}

fn pixel(page: &[u8], x: u32, y: u32) -> [u8; 4] {
    let at = ((y * GlyphPage::SIDE + x) * 4) as usize;
    [page[at], page[at + 1], page[at + 2], page[at + 3]]
}

#[test]
fn a_new_page_keeps_row_0_and_is_dirty_once() {
    let row0 = row0();
    let mut page = GlyphPage::new(&row0);
    let first = page.take_dirty().expect("first frame uploads");
    assert_eq!(first.len(), ROW_BYTES * 256);
    assert_eq!(&first[..row0.len()], &row0[..]);
    assert!(first[row0.len()..].iter().all(|b| *b == 0));
    assert!(page.take_dirty().is_none());
}

#[test]
fn a_row_0_of_the_wrong_length_leaves_row_0_transparent() {
    let mut page = GlyphPage::new(&[1, 2, 3]);
    let first = page.take_dirty().expect("first frame uploads");
    assert!(first.iter().all(|b| *b == 0));
}

#[test]
fn a_face_lands_at_10x10_on_the_icon_baseline_of_its_cell() {
    let face = face();
    for index in [0u8, 15, 16, 239] {
        let mut page = GlyphPage::new(&row0());
        page.take_dirty();
        page.write_face(GlyphCell(index), &face);
        let data = page.take_dirty().expect("dirty after write");
        let cell_x = u32::from(index % 16) * 16;
        let cell_y = (u32::from(index / 16) + 1) * 16;
        for y in 0..16 {
            for x in 0..16 {
                let expected = if x < 10 && (2..12).contains(&y) {
                    face.pixel(x * 32 / 10, (y - 2) * 32 / 10)
                } else {
                    [0, 0, 0, 0]
                };
                assert_eq!(pixel(data, cell_x + x, cell_y + y), expected, "cell {index} at {x},{y}");
            }
        }
    }
}

#[test]
fn writing_a_face_does_not_touch_row_0_or_other_cells() {
    let row0 = row0();
    let mut page = GlyphPage::new(&row0);
    page.take_dirty();
    page.write_face(GlyphCell(17), &face());
    let data = page.take_dirty().expect("dirty after write");
    assert_eq!(&data[..row0.len()], &row0[..]);
    for y in 16..256u32 {
        for x in 0..256u32 {
            let in_cell = (16..32).contains(&x) && (32..48).contains(&y);
            if !in_cell {
                assert_eq!(pixel(data, x, y), [0, 0, 0, 0], "{x},{y}");
            }
        }
    }
}

#[test]
fn take_dirty_returns_the_page_once_per_change() {
    let mut page = GlyphPage::new(&row0());
    assert!(page.take_dirty().is_some());
    assert!(page.take_dirty().is_none());
    page.write_face(GlyphCell(3), &face());
    page.write_face(GlyphCell(4), &face());
    assert!(page.take_dirty().is_some());
    assert!(page.take_dirty().is_none());
}

#[test]
fn a_page_marked_dirty_after_a_failed_upload_is_taken_again() {
    let mut page = GlyphPage::new(&row0());
    page.take_dirty();
    assert!(page.take_dirty().is_none());

    page.mark_dirty();

    assert!(page.take_dirty().is_some());
    assert!(page.take_dirty().is_none());
}
