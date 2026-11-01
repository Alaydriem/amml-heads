use std::collections::HashMap;

use crate::structs::{FaceImage, FaceKey, FaceUpdate};

struct Entry<T> {
    uuid: [u8; 16],
    actor: i64,
    generation: u32,
    face: Box<FaceImage>,
    pending: bool,
    texture: Option<T>,
    last_used: u64,
}

/// Faces by player, with the GPU texture each one owns once uploaded.
///
/// Holds at most `capacity` entries. Adding past capacity evicts the least recently used
/// entry and hands its texture back for release. Allocation happens only in `apply`.
pub struct FaceStore<T> {
    capacity: usize,
    uploads_per_frame: usize,
    uploaded_this_frame: usize,
    slots: Vec<Option<Entry<T>>>,
    free: Vec<usize>,
    by_uuid: HashMap<[u8; 16], usize>,
    by_actor: HashMap<i64, usize>,
    clock: u64,
    next_generation: u32,
}

impl<T> FaceStore<T> {
    pub fn new(capacity: usize, uploads_per_frame: usize) -> Self {
        Self {
            capacity,
            uploads_per_frame,
            uploaded_this_frame: 0,
            slots: (0..capacity).map(|_| None).collect(),
            free: (0..capacity).rev().collect(),
            by_uuid: HashMap::with_capacity(capacity),
            by_actor: HashMap::with_capacity(capacity),
            clock: 0,
            next_generation: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.by_actor.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_actor.is_empty()
    }

    /// Applies one update. Textures that are no longer wanted are pushed onto `released`.
    pub fn apply(&mut self, update: FaceUpdate, released: &mut Vec<T>) {
        match update {
            FaceUpdate::Add { uuid, actor, face } => self.add(uuid, actor, face, released),
            FaceUpdate::Skin { uuid, face } => {
                let Some(&slot) = self.by_uuid.get(&uuid) else { return };
                let generation = self.take_generation();
                if let Some(entry) = self.slots[slot].as_mut() {
                    if let Some(texture) = entry.texture.take() {
                        released.push(texture);
                    }
                    entry.face = face;
                    entry.pending = true;
                    entry.generation = generation;
                }
            }
            FaceUpdate::Remove { uuid } => {
                if let Some(&slot) = self.by_uuid.get(&uuid) {
                    self.free_slot(slot, released);
                }
            }
            FaceUpdate::Clear => {
                for slot in 0..self.slots.len() {
                    self.free_slot(slot, released);
                }
            }
        }
    }

    /// Starts a new frame and resets the upload budget.
    pub fn begin_frame(&mut self) {
        self.uploaded_this_frame = 0;
    }

    /// Whether `next_upload` would give a face now: one is waiting and the frame budget is not spent.
    pub fn has_pending_upload(&self) -> bool {
        self.uploaded_this_frame < self.uploads_per_frame && self.slots.iter().flatten().any(|entry| entry.pending)
    }

    /// Marks the face of `key` as waiting again, after its upload failed. A stale key does nothing.
    pub fn retry_upload(&mut self, key: FaceKey) {
        let Some(&slot) = self.by_actor.get(&key.actor) else { return };
        if let Some(entry) = self.slots[slot].as_mut()
            && entry.generation == key.generation
            && entry.texture.is_none()
        {
            entry.pending = true;
        }
    }

    /// The next face that needs a texture, until the upload budget of the frame is spent.
    pub fn next_upload(&mut self) -> Option<(FaceKey, &FaceImage)> {
        if self.uploaded_this_frame >= self.uploads_per_frame {
            return None;
        }
        let slot = self.slots.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.pending))?;
        self.uploaded_this_frame += 1;
        let entry = self.slots[slot].as_mut()?;
        entry.pending = false;
        Some((FaceKey::new(entry.actor, entry.generation), &entry.face))
    }

    /// Stores the texture made for `key`. A stale key hands the texture back.
    pub fn attach(&mut self, key: FaceKey, texture: T) -> Option<T> {
        let Some(&slot) = self.by_actor.get(&key.actor) else { return Some(texture) };
        match self.slots[slot].as_mut() {
            Some(entry) if entry.generation == key.generation && entry.texture.is_none() => {
                entry.texture = Some(texture);
                None
            }
            _ => Some(texture),
        }
    }

    /// The texture for an actor, marking the entry as recently used.
    pub fn texture_for(&mut self, actor: i64) -> Option<&T> {
        let slot = *self.by_actor.get(&actor)?;
        self.clock += 1;
        let entry = self.slots[slot].as_mut()?;
        entry.last_used = self.clock;
        entry.texture.as_ref()
    }

    fn add(&mut self, uuid: [u8; 16], actor: i64, face: Box<FaceImage>, released: &mut Vec<T>) {
        if self.capacity == 0 {
            return;
        }
        if let Some(&slot) = self.by_uuid.get(&uuid) {
            self.free_slot(slot, released);
        }
        if let Some(&slot) = self.by_actor.get(&actor) {
            self.free_slot(slot, released);
        }
        if self.free.is_empty() {
            self.evict_least_recent(released);
        }
        let Some(slot) = self.free.pop() else { return };
        self.clock += 1;
        let generation = self.take_generation();
        self.slots[slot] =
            Some(Entry { uuid, actor, generation, face, pending: true, texture: None, last_used: self.clock });
        self.by_uuid.insert(uuid, slot);
        self.by_actor.insert(actor, slot);
    }

    fn evict_least_recent(&mut self, released: &mut Vec<T>) {
        let oldest = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(slot, entry)| entry.as_ref().map(|entry| (slot, entry.last_used)))
            .min_by_key(|&(_, last_used)| last_used)
            .map(|(slot, _)| slot);
        if let Some(slot) = oldest {
            self.free_slot(slot, released);
        }
    }

    fn free_slot(&mut self, slot: usize, released: &mut Vec<T>) {
        let Some(entry) = self.slots[slot].take() else { return };
        self.by_uuid.remove(&entry.uuid);
        self.by_actor.remove(&entry.actor);
        self.free.push(slot);
        if let Some(texture) = entry.texture {
            released.push(texture);
        }
    }

    fn take_generation(&mut self) -> u32 {
        self.next_generation = self.next_generation.wrapping_add(1);
        self.next_generation
    }
}
