#![allow(unsafe_code)]

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError, TryLockError};

use amml_msvc::MsvcHeap;
use heads_core::services::CellFlags;

use super::{ChatFontCheck, ChatState, RenderRuntime};
use crate::layouts::GuiMessageLayout;

// the one chat state, shared by the packet thread and the game thread; None until chat heads is installed
static STATE: Mutex<Option<ChatState>> = Mutex::new(None);
// the flags the font width detour reads without the lock; fixed for the life of the process
static FLAGS: OnceLock<Arc<CellFlags>> = OnceLock::new();
// the game functions and heap the chat line edit uses; they come from the game image, so fixed for the process
static FONT_CHECK: OnceLock<ChatFontCheck> = OnceLock::new();
static HEAP: OnceLock<MsvcHeap> = OnceLock::new();
// the last font check result, and the frames since it ran
static FONT_IS_BITMAP: AtomicBool = AtomicBool::new(false);
static FONT_FRAMES: AtomicU32 = AtomicU32::new(0);
// set when the world is left; the next page flush takes it and offers the page again
static PAGE_STALE: AtomicBool = AtomicBool::new(false);

/// The process-wide holder of the `ChatState`. Every access except `install`, `suspend`, `set_chat` and `with_state_blocking` try-locks,
/// so the game thread never waits for the packet thread.
pub struct ChatRuntime;

impl ChatRuntime {
    /// The font check runs on the first frame and then once every this many frames.
    pub const FONT_CHECK_FRAMES: u32 = 64;

    /// The flags of the process. A `ChatState` is built with these so `has_face` sees its cells.
    pub fn flags() -> Arc<CellFlags> {
        Arc::clone(FLAGS.get_or_init(|| Arc::new(CellFlags::new())))
    }

    /// Starts chat heads. The first call keeps `state` for the life of the process; a later call resumes that state
    /// and drops `state`, so a cell never moves to another player.
    pub fn install(state: ChatState, flags: Arc<CellFlags>) {
        let _ = FLAGS.get_or_init(|| flags);
        let mut guard = Self::lock();
        match guard.as_mut() {
            Some(kept) => kept.resume(),
            None => *guard = Some(state),
        }
    }

    /// Stops chat heads. The state stays, ignoring packets and lines, until the next `install`.
    pub fn suspend() {
        if let Some(state) = Self::lock().as_mut() {
            state.suspend();
        }
    }

    /// Runs `f` on the state, waiting for the lock. Only the packet feed may use this: the game thread must use
    /// `with_state`.
    pub fn with_state_blocking<R>(f: impl FnOnce(&mut ChatState) -> R) -> Option<R> {
        Self::lock().as_mut().map(f)
    }

    /// Runs `f` on the state. `None` when there is no state or it is busy; never waits.
    pub fn with_state<R>(f: impl FnOnce(&mut ChatState) -> R) -> Option<R> {
        let mut guard = match STATE.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
            Err(TryLockError::WouldBlock) => return None,
        };
        guard.as_mut().map(f)
    }

    /// Whether the cell of a code point holds a face. False before the flags exist.
    pub fn has_face(codepoint: u32) -> bool {
        FLAGS.get().is_some_and(|flags| flags.has_face(codepoint))
    }

    /// Applies the live `chat` setting to the installed state.
    pub fn set_chat(chat: bool) {
        if let Some(state) = Self::lock().as_mut() {
            state.set_chat(chat);
        }
    }

    /// Uploads the glyph page when it changed. A frame with no change costs a `try_lock` and a flag check.
    pub fn flush_page(client: usize) {
        Self::flush_page_with(|page| RenderRuntime::upload_chat_page(client, page));
    }

    /// As `flush_page`, with `upload` in place of the texture upload.
    pub fn flush_page_with(upload: impl FnOnce(&[u8]) -> bool) {
        Self::with_state(|state| {
            if PAGE_STALE.swap(false, Ordering::SeqCst) {
                state.on_world_exit();
            }
            state.flush_page(upload)
        });
    }

    /// Marks the glyph page for upload again, as the world was left. Never waits: the next page flush applies it.
    pub fn on_world_exit() {
        PAGE_STALE.store(true, Ordering::SeqCst);
    }

    /// Gives the chat line edit its font check and the game heap. The next frame runs the font check.
    pub fn set_game_access(font_check: ChatFontCheck, heap: MsvcHeap) {
        let _ = FONT_CHECK.set(font_check);
        let _ = HEAP.set(heap);
        FONT_FRAMES.store(0, Ordering::Relaxed);
    }

    /// Runs the font check on the first frame and every `FONT_CHECK_FRAMES` frames after; other frames cost one
    /// atomic add.
    ///
    /// # Safety
    /// `client` must be null or the live `ClientInstance`, called on the game thread.
    pub unsafe fn refresh_font(client: usize) {
        if !FONT_FRAMES.fetch_add(1, Ordering::Relaxed).is_multiple_of(Self::FONT_CHECK_FRAMES) {
            return;
        }
        let Some(check) = FONT_CHECK.get() else { return };
        // SAFETY: the caller guarantees the client is live and this is the game thread.
        let bitmap = unsafe { check.is_bitmap(client) };
        FONT_IS_BITMAP.store(bitmap, Ordering::Relaxed);
    }

    /// Writes the full string and the filtered full string of a new chat line as `ChatState::glyph_line` gives
    /// them. Nothing changes while the font is not a `BitmapFont`, the state is busy, or a string stays as it is.
    ///
    /// # Safety
    /// `message` must be null or a `GuiMessage` its constructor has just built, with strings on the game heap, that
    /// no other thread touches during the call.
    pub unsafe fn edit_message(message: usize) {
        if message == 0 || !FONT_IS_BITMAP.load(Ordering::Relaxed) {
            return;
        }
        let Some(heap) = HEAP.get() else { return };
        // SAFETY: the caller guarantees a live, fully built GuiMessage that only this thread uses.
        let message = unsafe { &mut *(message as *mut GuiMessageLayout) };
        let lines = Self::with_state(|state| {
            let full = message.line().and_then(|line| state.glyph_line(&line));
            let filtered = message.filtered_line().and_then(|line| state.glyph_line(&line));
            (full, filtered)
        });
        let Some((full, filtered)) = lines else { return };
        if let Some(full) = full {
            // SAFETY: the constructor built the string with the game heap, which `heap` is.
            unsafe { message.replace_full(heap, &full) };
        }
        if let Some(filtered) = filtered {
            // SAFETY: as above.
            unsafe { message.replace_filtered_full(heap, &filtered) };
        }
    }

    fn lock() -> MutexGuard<'static, Option<ChatState>> {
        STATE.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
