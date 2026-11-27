use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

use heads_core::services::{ChatHeadPlacer, FaceExtractor, FaceGlyphs, FaceStore, IconSampler, PlayerDirectory};
use heads_core::structs::{ChatLine, FaceImage, FaceUpdate, GlyphCell, MessageKind, SkinView};

fn skin_bytes(side: usize) -> Vec<u8> {
    (0..side * side * 4).map(|i| (i % 251) as u8).collect()
}

fn extract(c: &mut Criterion, name: &str, side: u32) {
    let bytes = skin_bytes(side as usize);
    let skin = SkinView::new(side, side, 4, &bytes);
    c.bench_function(name, |b| b.iter(|| FaceExtractor::extract(black_box(&skin))));
}

fn extract_64(c: &mut Criterion) {
    extract(c, "extract_64", 64);
}

fn extract_256(c: &mut Criterion) {
    extract(c, "extract_256", 256);
}

fn sample_icon(c: &mut Criterion) {
    let face = FaceImage::new([77; 4096]);
    c.bench_function("sample_icon", |b| b.iter(|| IconSampler::sample(black_box(&face))));
}

fn texture_for_hit_128(c: &mut Criterion) {
    let mut store = FaceStore::<u32>::new(128, 128);
    let mut released = Vec::new();
    for n in 0..128u8 {
        let update = FaceUpdate::Add { uuid: [n; 16], actor: i64::from(n), face: Box::new(FaceImage::new([0; 4096])) };
        store.apply(update, &mut released);
    }
    while let Some((key, _)) = store.next_upload() {
        store.attach(key, u32::from(key.actor as u8));
    }
    c.bench_function("texture_for_hit_128", |b| b.iter(|| store.texture_for(black_box(64)).copied()));
}

fn full_world(players: usize) -> (FaceGlyphs, PlayerDirectory) {
    let mut glyphs = FaceGlyphs::new();
    let mut directory = PlayerDirectory::new();
    for n in 0..players {
        let uuid = [n as u8; 16];
        directory.record(uuid, &format!("x{n}"), &format!("Player{n:03}"));
        glyphs.set(uuid, GlyphCell(n as u8).codepoint());
    }
    (glyphs, directory)
}

fn place_chat_line(c: &mut Criterion) {
    let (glyphs, directory) = full_world(FaceGlyphs::CAPACITY);
    let line = ChatLine {
        kind: MessageKind::Chat,
        full: "<Player100> hello there",
        username: "Player100",
        author_xuid: "x100",
    };
    c.bench_function("place_chat_line", |b| b.iter(|| ChatHeadPlacer::place(black_box(&line), &glyphs, &directory)));
}

fn place_join_line_240_players(c: &mut Criterion) {
    let (glyphs, directory) = full_world(FaceGlyphs::CAPACITY);
    let line =
        ChatLine { kind: MessageKind::Localized, full: "Player200 joined the game", username: "", author_xuid: "" };
    c.bench_function("place_join_line_240_players", |b| {
        b.iter(|| ChatHeadPlacer::place(black_box(&line), &glyphs, &directory))
    });
}

fn place_edit_chat_line(c: &mut Criterion) {
    let (glyphs, directory) = full_world(FaceGlyphs::CAPACITY);
    let line = ChatLine {
        kind: MessageKind::Chat,
        full: "<Player100> hello there",
        username: "Player100",
        author_xuid: "x100",
    };
    c.bench_function("place_edit_chat_line", |b| {
        b.iter(|| ChatHeadPlacer::edit(black_box(&line), &glyphs, &directory))
    });
}

criterion_group!(
    heads,
    extract_64,
    extract_256,
    sample_icon,
    texture_for_hit_128,
    place_chat_line,
    place_join_line_240_players,
    place_edit_chat_line
);
criterion_main!(heads);
