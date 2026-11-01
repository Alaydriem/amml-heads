#![allow(unsafe_code)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock, PoisonError, RwLock};

use amm_client::link::ApiTable;
use amm_core::errors::LinkError;
use amml_sdk::{HookHandle, ModRuntime};
use heads_core::config::HeadsSettings;

use super::{ChatRuntime, HeadsLink, PacketObserver, RenderRuntime};
use crate::types::LinkState;

// What the hooks share: the manager link state, the installed hooks and the packet observer. The mod manager is
// reached on the first ClientInstance::update after enable, when every mod of the host has loaded; `hooks` holds
// at most HOOK_CAPACITY handles. Without the manager the mod runs with its default settings.
pub struct HeadsRuntime {
    config_dir: PathBuf,
    link_state: AtomicU8,
    link: Mutex<Option<HeadsLink>>,
    hooks: Mutex<Vec<HookHandle>>,
    observer: RwLock<Option<PacketObserver>>,
}

static RUNTIME: OnceLock<HeadsRuntime> = OnceLock::new();

impl HeadsRuntime {
    pub const HOOK_CAPACITY: usize = 8;

    pub fn new(config_dir: &Path) -> Self {
        Self {
            config_dir: config_dir.to_path_buf(),
            link_state: AtomicU8::new(LinkState::Waiting.to_u8()),
            link: Mutex::new(None),
            hooks: Mutex::new(Vec::with_capacity(Self::HOOK_CAPACITY)),
            observer: RwLock::new(None),
        }
    }

    // the runtime of this DLL; the first call wins
    pub fn install(config_dir: &Path) -> &'static HeadsRuntime {
        RUNTIME.get_or_init(|| Self::new(config_dir))
    }

    pub fn get() -> Option<&'static HeadsRuntime> {
        RUNTIME.get()
    }

    pub fn link_state(&self) -> LinkState {
        LinkState::from_u8(self.link_state.load(Ordering::SeqCst))
    }

    // enable: the next tick reaches the manager again
    pub fn enable_link(&self) {
        self.link_state.store(LinkState::Waiting.to_u8(), Ordering::SeqCst);
    }

    // once per ClientInstance::update on the game thread, after the original: reaches the manager on the first
    // call after enable, then applies settings changes
    pub fn tick(&self) {
        if self.link_state() == LinkState::Waiting {
            self.attach_with(ApiTable::resolve());
        }
        if self.link_state() == LinkState::Attached {
            self.poll_link();
        }
    }

    // registers with the manager the table belongs to; on failure the mod keeps its default settings and one line
    // is logged. Only a Waiting runtime takes the result: if disable ran meanwhile, a new link is dropped again
    pub fn attach_with(&self, table: Result<ApiTable, LinkError>) -> LinkState {
        let mut slot = self.link.lock().unwrap_or_else(PoisonError::into_inner);
        let link = table.and_then(|table| HeadsLink::new(table, &self.config_dir));
        let (state, link) = match link {
            Ok(link) => (LinkState::Attached, Some(link)),
            Err(error) => {
                Self::log_link_error(&error);
                (LinkState::Off, None)
            }
        };
        let waiting = LinkState::Waiting.to_u8();
        match self.link_state.compare_exchange(waiting, state.to_u8(), Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => {
                if let Some(link) = &link {
                    self.apply_settings(link.settings());
                }
                *slot = link;
                state
            }
            Err(current) => LinkState::from_u8(current),
        }
    }

    // reads the manager events and applies new settings to the render state
    pub fn poll_link(&self) {
        let mut slot = self.link.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(link) = slot.as_mut()
            && link.poll()
        {
            self.apply_settings(link.settings());
        }
    }

    // new settings reach the render state and the chat state
    fn apply_settings(&self, settings: HeadsSettings) {
        ChatRuntime::set_chat(settings.chat);
        RenderRuntime::set_settings(settings);
    }

    // disable: no tick reaches the manager until the next enable; dropping the link unregisters
    pub fn detach(&self) {
        self.link_state.store(LinkState::Disabled.to_u8(), Ordering::SeqCst);
        drop(self.link.lock().unwrap_or_else(PoisonError::into_inner).take());
    }

    pub fn set_observer(&self, observer: PacketObserver) {
        *self.observer.write().unwrap_or_else(PoisonError::into_inner) = Some(observer);
    }

    pub fn clear_observer(&self) {
        *self.observer.write().unwrap_or_else(PoisonError::into_inner) = None;
    }

    /// Sends the faces of a PlayerListPacket on. Skipped while the observer is being replaced.
    ///
    /// # Safety
    /// `packet` must be null or a live PlayerListPacket for the call, which the packet handler detour guarantees.
    pub unsafe fn observe_list(&self, packet: usize) {
        if let Ok(guard) = self.observer.try_read()
            && let Some(observer) = guard.as_ref()
        {
            // SAFETY: the caller guarantees the packet is live for the call.
            unsafe { observer.on_player_list(packet) };
        }
    }

    /// Sends the face of a PlayerSkinPacket on.
    ///
    /// # Safety
    /// `packet` must be null or a live PlayerSkinPacket for the call, which the packet handler detour guarantees.
    pub unsafe fn observe_skin(&self, packet: usize) {
        if let Ok(guard) = self.observer.try_read()
            && let Some(observer) = guard.as_ref()
        {
            // SAFETY: the caller guarantees the packet is live for the call.
            unsafe { observer.on_player_skin(packet) };
        }
    }

    // the world was left: every face and texture goes. A full channel cannot take the update, so the render state
    // is cleared directly
    pub fn clear_faces(&self) {
        let queued = match self.observer.try_read() {
            Ok(guard) => guard.as_ref().map(PacketObserver::send_clear),
            Err(_) => Some(false),
        };
        if queued == Some(false) {
            RenderRuntime::request_clear();
        }
    }

    // keeps an installed hook; false, keeping nothing, when HOOK_CAPACITY hooks are already kept
    pub fn keep_hook(&self, hook: HookHandle) -> bool {
        let mut hooks = self.hooks.lock().unwrap_or_else(PoisonError::into_inner);
        if hooks.len() >= Self::HOOK_CAPACITY {
            return false;
        }
        hooks.push(hook);
        true
    }

    // the installed hooks, newest first, leaving none behind
    pub fn take_hooks(&self) -> Vec<HookHandle> {
        self.take_hooks_since(0)
    }

    pub fn hook_count(&self) -> usize {
        self.hooks.lock().unwrap_or_else(PoisonError::into_inner).len()
    }

    // the hooks kept after the first `count`, newest first; the first `count` stay
    pub fn take_hooks_since(&self, count: usize) -> Vec<HookHandle> {
        let mut hooks = self.hooks.lock().unwrap_or_else(PoisonError::into_inner);
        let at = count.min(hooks.len());
        let mut taken = hooks.split_off(at);
        taken.reverse();
        taken
    }

    fn log_link_error(error: &LinkError) {
        let Some(runtime) = ModRuntime::get() else { return };
        match error {
            LinkError::ManagerMissing | LinkError::AbiMismatch { .. } => {
                runtime.logger().warn("amml_modmanager.dll (ABI 2.x) was not found; the heads mod uses its defaults");
            }
            other => {
                runtime
                    .logger()
                    .warn(&format!("@amml/modmanager refused @alaydriem/heads ({other}); using the defaults"));
            }
        }
    }
}
