use std::sync::{Mutex, PoisonError};

use amml_api::{Logger, SkinRef};
use heads_core::structs::UvRect;

use crate::structs::FaceRead;

/// Shows where a skin stops being a face: one log line per player for the first `LOG_LIMIT` players of a session.
pub struct SkinDiagnostics {
    log: Logger,
    // the players already logged; holds at most LOG_LIMIT entries
    seen: Mutex<Vec<[u8; 16]>>,
}

impl SkinDiagnostics {
    pub const LOG_LIMIT: usize = 64;

    pub fn new(log: Logger) -> Self {
        Self { log, seen: Mutex::new(Vec::with_capacity(Self::LOG_LIMIT)) }
    }

    /// How many players were logged.
    pub fn logged(&self) -> usize {
        self.seen.lock().unwrap_or_else(PoisonError::into_inner).len()
    }

    /// Logs the skin of a player the first time it is seen, up to `LOG_LIMIT` players.
    pub fn record(&self, kind: &str, uuid: [u8; 16], actor: Option<i64>, skin: &SkinRef<'_>, read: &FaceRead) {
        if !self.claim(uuid) {
            return;
        }
        let actor = actor.map_or_else(|| "none".to_string(), |actor| actor.to_string());
        let mut fields = vec![
            ("kind", kind.to_string()),
            ("uuid", Self::short_id(&uuid)),
            ("actor", actor),
            ("outcome", read.outcome.clone()),
            ("head_source", read.head_source.to_string()),
            ("face_rect", Self::rect(read.head.face)),
            ("hat_rect", read.head.hat.map_or_else(|| "none".to_string(), Self::rect)),
            ("geometry_json_len", read.geometry_json_len.map_or_else(|| "none".to_string(), |len| len.to_string())),
        ];
        match skin.image() {
            Some(image) => {
                fields.push(("width", image.width.to_string()));
                fields.push(("height", image.height.to_string()));
                fields.push(("format", u32::from(image.format).to_string()));
                fields.push(("blob_size", image.pixels.len().to_string()));
            }
            None => fields.push(("unreadable", "true".to_string())),
        }
        fields.push(("is_persona", Self::flag(skin.is_persona())));
        fields.push(("is_premium", Self::flag(skin.is_premium())));
        fields.push(("default_geometry", skin.default_geometry_name().unwrap_or("<unavailable>").to_string()));
        fields.push(("resource_patch_len", skin.resource_patch().map_or(0, str::len).to_string()));
        let kinds: Vec<String> = skin.animated_images().map(|(kind, _)| u32::from(kind).to_string()).collect();
        fields.push(("animated_images", kinds.len().to_string()));
        fields.push(("animated_types", kinds.join(",")));
        let borrowed: Vec<(&str, &str)> = fields.iter().map(|(key, value)| (*key, value.as_str())).collect();
        self.log.info("heads skin read", &borrowed);
    }

    // records the player; false when it was seen before or the log is full
    fn claim(&self, uuid: [u8; 16]) -> bool {
        let mut seen = self.seen.lock().unwrap_or_else(PoisonError::into_inner);
        if seen.len() >= Self::LOG_LIMIT || seen.contains(&uuid) {
            return false;
        }
        seen.push(uuid);
        true
    }

    fn flag(flag: Option<bool>) -> String {
        flag.map_or_else(|| "unavailable".to_string(), |flag| flag.to_string())
    }

    fn rect(rect: UvRect) -> String {
        format!("{},{},{},{}", rect.x, rect.y, rect.w, rect.h)
    }

    // the first four bytes of the uuid as 8 hex digits
    fn short_id(uuid: &[u8; 16]) -> String {
        uuid[..4].iter().map(|byte| format!("{byte:02x}")).collect()
    }
}
