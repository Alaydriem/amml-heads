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

    let layout: HashSet<&str> = HeadsEntries::LAYOUT.iter().map(|id| id.as_str()).collect();
    assert_eq!(layout.len(), HeadsEntries::LAYOUT.len());
    assert!(layout.is_disjoint(&ids));
    assert!(layout.is_disjoint(&chat));

    let optional: Vec<&str> = HeadsEntries::DIAGNOSTICS
        .iter()
        .chain(HeadsEntries::GEOMETRY.iter())
        .chain(HeadsEntries::PERSONA.iter())
        .map(|id| id.as_str())
        .collect();
    let optional_set: HashSet<&str> = optional.iter().copied().collect();
    assert_eq!(optional_set.len(), optional.len());
    assert!(optional_set.is_disjoint(&ids));
    assert!(optional_set.is_disjoint(&chat));
    assert!(optional_set.is_disjoint(&layout));
}

#[test]
fn manifest_has_every_chat_and_layout_entry_once() {
    let manifest: HashSet<&str> = HeadsEntries::MANIFEST.iter().map(|id| id.as_str()).collect();
    assert_eq!(manifest.len(), HeadsEntries::MANIFEST.len());
    for id in HeadsEntries::CHAT
        .iter()
        .chain(HeadsEntries::LAYOUT.iter())
        .chain(HeadsEntries::DIAGNOSTICS.iter())
        .chain(HeadsEntries::GEOMETRY.iter())
        .chain(HeadsEntries::PERSONA.iter())
    {
        assert!(manifest.contains(id.as_str()), "{} is not in the manifest", id.as_str());
    }
}
