use heads_core::services::FaceStore;
use heads_core::structs::{FaceImage, FaceKey, FaceUpdate};

fn face() -> Box<FaceImage> {
    Box::new(FaceImage::new([0; 4096]))
}

fn uuid(n: u8) -> [u8; 16] {
    [n; 16]
}

fn add(store: &mut FaceStore<u32>, n: u8, actor: i64, released: &mut Vec<u32>) {
    store.apply(FaceUpdate::Add { uuid: uuid(n), actor, face: face() }, released);
}

fn next_key(store: &mut FaceStore<u32>) -> Option<FaceKey> {
    store.next_upload().map(|(key, _)| key)
}

// Adds a face, uploads it on a fresh frame and attaches the texture.
fn add_with_texture(store: &mut FaceStore<u32>, n: u8, actor: i64, texture: u32, released: &mut Vec<u32>) {
    add(store, n, actor, released);
    store.begin_frame();
    let key = next_key(store).unwrap();
    assert_eq!(key.actor, actor);
    assert_eq!(store.attach(key, texture), None);
}

#[test]
fn evicts_least_recent_past_capacity() {
    let mut store = FaceStore::new(2, 4);
    let mut released = Vec::new();
    add_with_texture(&mut store, 1, 1, 11, &mut released);
    add_with_texture(&mut store, 2, 2, 12, &mut released);
    assert!(released.is_empty());
    add_with_texture(&mut store, 3, 3, 13, &mut released);

    assert_eq!(store.len(), 2);
    assert_eq!(released, vec![11]);
    assert!(store.texture_for(1).is_none());
    assert_eq!(store.texture_for(2), Some(&12));
    assert_eq!(store.texture_for(3), Some(&13));
}

#[test]
fn texture_for_refreshes_recency() {
    let mut store = FaceStore::new(2, 4);
    let mut released = Vec::new();
    add_with_texture(&mut store, 1, 1, 11, &mut released);
    add_with_texture(&mut store, 2, 2, 12, &mut released);
    assert_eq!(store.texture_for(1), Some(&11));
    add_with_texture(&mut store, 3, 3, 13, &mut released);

    assert_eq!(released, vec![12]);
    assert!(store.texture_for(2).is_none());
    assert_eq!(store.texture_for(1), Some(&11));
}

#[test]
fn skin_update_bumps_generation_and_releases_old_texture() {
    let mut store = FaceStore::new(4, 4);
    let mut released = Vec::new();
    add(&mut store, 1, 7, &mut released);
    store.begin_frame();
    let first = next_key(&mut store).unwrap();
    assert_eq!(store.attach(first, 70), None);

    store.apply(FaceUpdate::Skin { uuid: uuid(1), face: face() }, &mut released);
    assert_eq!(released, vec![70]);
    assert!(store.texture_for(7).is_none());

    store.begin_frame();
    let second = next_key(&mut store).unwrap();
    assert_eq!(second.actor, 7);
    assert_ne!(second.generation, first.generation);
    assert_eq!(store.attach(second, 71), None);
    assert_eq!(store.texture_for(7), Some(&71));
}

#[test]
fn stale_attach_is_returned() {
    let mut store = FaceStore::new(4, 4);
    let mut released = Vec::new();
    add(&mut store, 1, 7, &mut released);
    store.begin_frame();
    let stale = next_key(&mut store).unwrap();

    store.apply(FaceUpdate::Skin { uuid: uuid(1), face: face() }, &mut released);
    assert_eq!(store.attach(stale, 99), Some(99));
    assert!(store.texture_for(7).is_none());
}

#[test]
fn remove_by_uuid_releases() {
    let mut store = FaceStore::new(4, 4);
    let mut released = Vec::new();
    add_with_texture(&mut store, 1, 7, 70, &mut released);
    store.apply(FaceUpdate::Remove { uuid: uuid(1) }, &mut released);

    assert_eq!(released, vec![70]);
    assert_eq!(store.len(), 0);
    assert!(store.texture_for(7).is_none());
}

#[test]
fn clear_releases_everything() {
    let mut store = FaceStore::new(4, 4);
    let mut released = Vec::new();
    add_with_texture(&mut store, 1, 1, 11, &mut released);
    add_with_texture(&mut store, 2, 2, 12, &mut released);
    store.apply(FaceUpdate::Clear, &mut released);

    released.sort();
    assert_eq!(released, vec![11, 12]);
    assert_eq!(store.len(), 0);
    assert!(store.texture_for(1).is_none());
}

#[test]
fn upload_budget_per_frame() {
    let mut store = FaceStore::new(8, 4);
    let mut released = Vec::new();
    for n in 1..=6u8 {
        add(&mut store, n, i64::from(n), &mut released);
    }

    let mut first = 0;
    while store.next_upload().is_some() {
        first += 1;
    }
    assert_eq!(first, 4);

    store.begin_frame();
    let mut second = 0;
    while store.next_upload().is_some() {
        second += 1;
    }
    assert_eq!(second, 2);
}

#[test]
fn skin_for_unknown_uuid_is_ignored() {
    let mut store = FaceStore::<u32>::new(4, 4);
    let mut released = Vec::new();
    store.apply(FaceUpdate::Skin { uuid: uuid(9), face: face() }, &mut released);

    assert_eq!(store.len(), 0);
    assert!(released.is_empty());
    assert!(store.next_upload().is_none());
}

#[test]
fn pending_upload_follows_queue_and_budget() {
    let mut store = FaceStore::new(8, 1);
    let mut released = Vec::new();
    assert!(!store.has_pending_upload());

    add(&mut store, 1, 1, &mut released);
    add(&mut store, 2, 2, &mut released);
    assert!(store.has_pending_upload());

    assert!(store.next_upload().is_some());
    assert!(!store.has_pending_upload());

    store.begin_frame();
    assert!(store.has_pending_upload());
    assert!(store.next_upload().is_some());
    store.begin_frame();
    assert!(!store.has_pending_upload());
}

#[test]
fn failed_upload_is_retried_until_the_skin_changes() {
    let mut store = FaceStore::<u32>::new(4, 4);
    let mut released = Vec::new();
    add(&mut store, 1, 7, &mut released);
    let key = next_key(&mut store).unwrap();
    assert!(!store.has_pending_upload());

    store.retry_upload(key);
    assert!(store.has_pending_upload());
    assert_eq!(next_key(&mut store), Some(key));

    store.apply(FaceUpdate::Skin { uuid: uuid(1), face: face() }, &mut released);
    store.begin_frame();
    let newer = next_key(&mut store).unwrap();
    store.retry_upload(key);
    assert!(!store.has_pending_upload());
    assert_eq!(store.attach(newer, 5), None);
    store.retry_upload(newer);
    assert!(!store.has_pending_upload());
}
