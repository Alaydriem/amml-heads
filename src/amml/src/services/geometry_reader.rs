#![allow(unsafe_code)]

use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use amml_msvc::{MsvcHeap, MsvcString};
use heads_core::services::GeometryHeadLocator;

use super::{MemoryReader, StdStringReader};
use crate::structs::{GeometryRead, PacketOffsets};

type ToStyledStringFn = unsafe extern "C" fn(usize, *mut MsvcString) -> *mut MsvcString;

/// Reads the geometry of a skin, to find where its head is.
///
/// It asks the game for the geometry JSON (`Json::Value::toStyledString` on `mGeometryData`), copies the text and
/// frees the game string at once.
pub struct GeometryReader {
    to_styled_string: usize,
    heap: MsvcHeap,
    reads: AtomicU64,
    slowest_nanos: AtomicU64,
}

impl GeometryReader {
    /// Longest geometry text read; a longer one is freed unread.
    pub const MAX_JSON: usize = 1024 * 1024;
    const MAX_NAME: usize = 256;
    // the slowest read is logged once, after this many reads
    const LOG_AFTER: u64 = 64;

    pub fn new(to_styled_string: usize, heap: MsvcHeap) -> Self {
        Self { to_styled_string, heap, reads: AtomicU64::new(0), slowest_nanos: AtomicU64::new(0) }
    }

    /// The slowest read so far, in nanoseconds.
    pub fn slowest_nanos(&self) -> u64 {
        self.slowest_nanos.load(Ordering::Relaxed)
    }

    /// The geometry of the skin behind a `SerializedSkinRef`, with the head of its animated face mesh when
    /// `animated` is set. `None` when a link is null.
    ///
    /// # Safety
    /// `skin_ref` must be null or the address of a live `shared_ptr` to a `SerializedSkinImpl`, and
    /// `to_styled_string` must be the game `Json::Value::toStyledString`. The game thread calls this inside the
    /// packet detour that holds the skin.
    pub unsafe fn read(&self, skin_ref: usize, offsets: &PacketOffsets, animated: bool) -> Option<GeometryRead> {
        // SAFETY: the caller guarantees `skin_ref` is null or a live shared_ptr, whose first word is the element.
        let element: usize = unsafe { MemoryReader::read(skin_ref)? };
        if element == 0 {
            return None;
        }
        let started = Instant::now();
        let name = offsets.skin.default_geometry_name.map(|offset| {
            // SAFETY: the name is a std::string member of the live SerializedSkinImpl.
            unsafe { StdStringReader::text(element + offset, Self::MAX_NAME) }.unwrap_or_default()
        });
        // SAFETY: mGeometryData is a Json::Value member of the live SerializedSkinImpl.
        let json = unsafe { self.styled_string(element.checked_add(offsets.skin.geometry_data)?) };
        // without the name offset the geometry head locator is off, and the skin keeps its classic head
        let head =
            json.as_deref().zip(name.as_deref()).and_then(|(json, name)| GeometryHeadLocator::locate(json, name));
        let animated_head = json.as_deref().filter(|_| animated).and_then(GeometryHeadLocator::locate_animated_face);
        self.note(started.elapsed());
        Some(GeometryRead { head, animated_head, json })
    }

    // The JSON of a Json::Value, or None when it is longer than MAX_JSON or not sane. The string the game builds
    // is freed on the game heap exactly once, whatever happens to its text.
    unsafe fn styled_string(&self, value: usize) -> Option<String> {
        let mut out = MaybeUninit::<MsvcString>::zeroed();
        // SAFETY: `to_styled_string` is the resolved Json::Value::toStyledString: this in rcx, the address of an
        // uninitialised std::string in rdx, which it constructs and returns.
        unsafe {
            std::mem::transmute::<usize, ToStyledStringFn>(self.to_styled_string)(value, out.as_mut_ptr());
        }
        let address = out.as_ptr() as usize;
        // SAFETY: the game just built a std::string at `address`.
        let text = unsafe { StdStringReader::text(address, Self::MAX_JSON) };
        // SAFETY: the string was built by the game with its allocator; it is released here once and not used again.
        unsafe { out.assume_init_mut().release(&self.heap) };
        text
    }

    fn note(&self, elapsed: Duration) {
        let nanos = u64::try_from(elapsed.as_nanos()).unwrap_or(u64::MAX);
        self.slowest_nanos.fetch_max(nanos, Ordering::Relaxed);
        if self.reads.fetch_add(1, Ordering::Relaxed) + 1 == Self::LOG_AFTER {
            curia::info!("slowest skin geometry read", { nanos: self.slowest_nanos(), reads: Self::LOG_AFTER });
        }
    }
}
