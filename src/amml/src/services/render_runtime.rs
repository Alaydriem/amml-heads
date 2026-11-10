use std::sync::{Mutex, MutexGuard, PoisonError, TryLockError};

use heads_core::config::HeadsSettings;

use super::RenderState;
use crate::structs::IconChoice;

// the one render state, shared by the two draw detours; None until the mod enables
static STATE: Mutex<Option<RenderState>> = Mutex::new(None);

/// The process-wide holder of the `RenderState`. The lock is never held across the original render, because the
/// game calls the icon detour from inside it. `before` and `after` do call into the game (the client vtable, the
/// texture group) while they hold it; that cannot deadlock, because those calls never reach the icon detour, and
/// `icon` only try-locks.
pub struct RenderRuntime;

impl RenderRuntime {
    pub fn install(state: RenderState) {
        *Self::lock() = Some(state);
    }

    pub fn uninstall() -> Option<RenderState> {
        Self::lock().take()
    }

    pub fn is_installed() -> bool {
        Self::lock().is_some()
    }

    /// Applies new settings to the installed state.
    pub fn set_settings(settings: HeadsSettings) {
        if let Some(state) = Self::lock().as_mut() {
            state.set_settings(settings);
        }
    }

    /// Clears every face without going through the channel.
    pub fn request_clear() {
        if let Some(state) = Self::lock().as_mut() {
            state.request_clear();
        }
    }

    /// Starts a frame. False when the state is missing or busy, in which case the frame is not tagged.
    pub fn before(client: usize) -> bool {
        let mut guard = match STATE.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
            Err(TryLockError::WouldBlock) => return false,
        };
        match guard.as_mut() {
            Some(state) => {
                state.before_render(client);
                true
            }
            None => false,
        }
    }

    /// Ends a frame. Waits for the lock, because the waypoint colours must be put back.
    pub fn after() {
        if let Some(state) = Self::lock().as_mut() {
            state.after_render();
        }
    }

    /// What the icon blit should draw, and the scale for a face. Passes the colour through when the state is
    /// missing or busy.
    pub fn icon(packed: u32) -> (IconChoice, f32) {
        let mut guard = match STATE.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
            Err(TryLockError::WouldBlock) => return (IconChoice::Vanilla(packed), 1.0),
        };
        match guard.as_mut() {
            Some(state) => (state.icon_for(packed), state.scale()),
            None => (IconChoice::Vanilla(packed), 1.0),
        }
    }

    /// Uploads the chat glyph page with the client's texture group. False when the state is missing or busy, or the
    /// upload did not happen.
    pub fn upload_chat_page(client: usize, page: &[u8]) -> bool {
        let mut guard = match STATE.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
            Err(TryLockError::WouldBlock) => return false,
        };
        guard.as_mut().is_some_and(|state| state.upload_chat_page(client, page))
    }

    /// Puts the waypoint colours back after a panic in a detour.
    pub fn recover() {
        Self::after();
    }

    fn lock() -> MutexGuard<'static, Option<RenderState>> {
        STATE.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
