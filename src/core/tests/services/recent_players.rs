use heads_core::services::RecentPlayers;

fn uuid(n: usize) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&(n as u64).to_le_bytes());
    bytes
}

fn filled() -> RecentPlayers {
    let mut recent = RecentPlayers::new();
    for n in 0..RecentPlayers::CAPACITY {
        recent.note(uuid(n), &format!("x{n}"), &format!("p{n}"));
    }
    recent
}

#[test]
fn the_list_stops_at_240_and_evicts_the_oldest() {
    let mut recent = filled();
    assert_eq!(recent.len(), RecentPlayers::CAPACITY);

    recent.note(uuid(240), "x240", "p240");

    assert_eq!(recent.len(), RecentPlayers::CAPACITY);
    assert!(recent.iter().all(|(id, _, _)| id != uuid(0)));
    assert_eq!(recent.iter().next(), Some((uuid(1), "x1", "p1")));
    assert_eq!(recent.iter().last(), Some((uuid(240), "x240", "p240")));
}

#[test]
fn noting_a_known_player_again_updates_it_and_makes_it_the_newest() {
    let mut recent = filled();

    recent.note(uuid(0), "x0", "renamed");
    recent.note(uuid(240), "x240", "p240");

    assert_eq!(recent.len(), RecentPlayers::CAPACITY);
    assert!(recent.iter().all(|(id, _, _)| id != uuid(1)));
    assert!(recent.iter().any(|entry| entry == (uuid(0), "x0", "renamed")));
    assert!(recent.iter().all(|(_, _, name)| name != "p0"));
}

#[test]
fn a_removed_player_is_gone() {
    let mut recent = RecentPlayers::new();
    recent.note(uuid(1), "x1", "p1");
    recent.note(uuid(2), "x2", "p2");

    recent.remove(uuid(1));

    assert_eq!(recent.iter().collect::<Vec<_>>(), vec![(uuid(2), "x2", "p2")]);
}
