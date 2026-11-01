use heads_core::services::PlayerDirectory;

fn uuid(n: usize) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&(n as u64).to_le_bytes());
    bytes
}

#[test]
fn a_player_is_found_by_xuid_and_by_name() {
    let mut directory = PlayerDirectory::new();
    assert!(directory.record(uuid(1), "2535", "Steve"));
    assert_eq!(directory.by_xuid("2535"), Some(uuid(1)));
    assert_eq!(directory.by_name("Steve"), Some(uuid(1)));
    assert_eq!(directory.by_name("steve"), None);
    assert_eq!(directory.names().collect::<Vec<_>>(), vec![("Steve", uuid(1))]);
}

#[test]
fn an_empty_xuid_finds_nothing() {
    let mut directory = PlayerDirectory::new();
    directory.record(uuid(1), "", "Steve");
    assert_eq!(directory.by_xuid(""), None);
}

#[test]
fn two_players_with_one_name_give_no_match() {
    let mut directory = PlayerDirectory::new();
    directory.record(uuid(1), "1", "Alex");
    directory.record(uuid(2), "2", "Alex");
    assert_eq!(directory.by_name("Alex"), None);
    assert_eq!(directory.by_xuid("1"), Some(uuid(1)));
    assert_eq!(directory.by_xuid("2"), Some(uuid(2)));
}

#[test]
fn a_new_name_for_the_same_player_replaces_the_old_one() {
    let mut directory = PlayerDirectory::new();
    directory.record(uuid(1), "1", "Old");
    directory.record(uuid(1), "1", "New");
    assert_eq!(directory.by_name("Old"), None);
    assert_eq!(directory.by_name("New"), Some(uuid(1)));
    assert_eq!(directory.names().count(), 1);
}

#[test]
fn the_directory_stops_at_240() {
    let mut directory = PlayerDirectory::new();
    for n in 0..PlayerDirectory::CAPACITY {
        assert!(directory.record(uuid(n), &format!("x{n}"), &format!("p{n}")));
    }
    assert!(!directory.record(uuid(240), "x240", "p240"));
    assert_eq!(directory.by_xuid("x240"), None);
    assert_eq!(directory.by_name("p240"), None);
    assert!(directory.record(uuid(0), "x0", "renamed"));
    assert_eq!(directory.by_name("renamed"), Some(uuid(0)));
    assert_eq!(directory.names().count(), PlayerDirectory::CAPACITY);
}

#[test]
fn a_xuid_shared_by_two_players_gives_no_match() {
    let mut directory = PlayerDirectory::new();
    directory.record(uuid(1), "same", "Steve");
    assert_eq!(directory.by_xuid("same"), Some(uuid(1)));
    directory.record(uuid(2), "same", "Alex");
    assert_eq!(directory.by_xuid("same"), None);
    assert_eq!(directory.by_name("Alex"), Some(uuid(2)));
}

#[test]
fn a_noted_player_makes_a_shared_name_and_xuid_ambiguous() {
    let mut directory = PlayerDirectory::new();
    directory.record(uuid(1), "1", "Alex");
    directory.note(uuid(2), "1", "Alex");
    assert_eq!(directory.by_name("Alex"), None);
    assert_eq!(directory.by_xuid("1"), None);
    assert_eq!(directory.names().count(), 2);
}

#[test]
fn a_noted_player_alone_is_found_by_name_and_xuid() {
    let mut directory = PlayerDirectory::new();
    directory.note(uuid(2), "2", "Bob");
    assert_eq!(directory.by_name("Bob"), Some(uuid(2)));
    assert_eq!(directory.by_xuid("2"), Some(uuid(2)));
}

#[test]
fn noting_a_recorded_player_updates_it_and_adds_no_entry() {
    let mut directory = PlayerDirectory::new();
    directory.record(uuid(1), "1", "Old");
    directory.note(uuid(1), "1", "New");
    assert_eq!(directory.by_name("Old"), None);
    assert_eq!(directory.by_name("New"), Some(uuid(1)));
    assert_eq!(directory.names().count(), 1);
}

#[test]
fn recording_a_noted_player_keeps_one_entry() {
    let mut directory = PlayerDirectory::new();
    directory.note(uuid(1), "1", "Alex");
    assert!(directory.record(uuid(1), "1", "Alex"));
    assert_eq!(directory.by_name("Alex"), Some(uuid(1)));
    assert_eq!(directory.names().count(), 1);
}
