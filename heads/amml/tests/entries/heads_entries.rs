use std::collections::HashSet;

use alaydriem_heads::entries::HeadsEntries;

#[test]
fn entry_ids_are_distinct() {
    let ids: HashSet<&str> = HeadsEntries::ALL.iter().map(|id| id.as_str()).collect();
    assert_eq!(ids.len(), HeadsEntries::ALL.len());
    assert_eq!(ids.len(), 17);

    let chat: HashSet<&str> = HeadsEntries::CHAT.iter().map(|id| id.as_str()).collect();
    assert_eq!(chat.len(), HeadsEntries::CHAT.len());
    assert!(chat.is_disjoint(&ids));
}

#[test]
fn manifest_has_every_chat_entry_once() {
    let manifest: HashSet<&str> = HeadsEntries::MANIFEST.iter().map(|id| id.as_str()).collect();
    assert_eq!(manifest.len(), HeadsEntries::MANIFEST.len());
    for id in HeadsEntries::CHAT {
        assert!(manifest.contains(id.as_str()), "{} is not in the manifest", id.as_str());
    }
}
