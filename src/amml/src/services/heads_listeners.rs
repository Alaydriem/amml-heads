use std::sync::Arc;

use amml_api::amml_abi::{
    ChatMessageAdded, ClientTick, LocatorBarIconDraw, PlayerListUpdate, PlayerSkinChanged, TexturesReloaded, WorldLeft,
};
use amml_api::{ApiError, Ctx, HookOrder, Listener};

use crate::services::HeadsRuntime;

// the loader listeners of the mod; each only hands its event to the runtime. Dropping them removes them
pub struct HeadsListeners {
    listeners: Vec<Listener>,
    runtime: Arc<HeadsRuntime>,
}

impl HeadsListeners {
    // every listen is required: a loader that cannot serve one fails the enable
    pub fn start(ctx: &Ctx, runtime: Arc<HeadsRuntime>) -> Result<Self, ApiError> {
        let events = ctx.events();
        let mut listeners = Vec::with_capacity(7);
        let listing = Arc::clone(&runtime);
        listeners.push(events.listen::<PlayerListUpdate>(HookOrder::NORMAL, move |event, _| {
            listing.on_player_list(event);
        })?);
        let skins = Arc::clone(&runtime);
        listeners.push(events.listen::<PlayerSkinChanged>(HookOrder::NORMAL, move |event, _| {
            skins.on_player_skin(event);
        })?);
        let drawing = Arc::clone(&runtime);
        listeners.push(events.listen::<LocatorBarIconDraw>(HookOrder::NORMAL, move |event, _| {
            drawing.icon(event);
        })?);
        let chatting = Arc::clone(&runtime);
        listeners.push(events.listen::<ChatMessageAdded>(HookOrder::NORMAL, move |event, _| {
            chatting.chat_line(event);
        })?);
        let reloading = Arc::clone(&runtime);
        listeners.push(events.listen::<TexturesReloaded>(HookOrder::NORMAL, move |event, _| {
            reloading.textures_reloaded(event);
        })?);
        let leaving = Arc::clone(&runtime);
        listeners.push(events.listen::<WorldLeft>(HookOrder::NORMAL, move |_, _| leaving.world_left())?);
        let ticking = Arc::clone(&runtime);
        listeners.push(events.listen::<ClientTick>(HookOrder::NORMAL, move |_, _| ticking.tick())?);
        Ok(Self { listeners, runtime })
    }

    // the listeners go first, so no listener runs while the runtime lets go of the link
    pub fn stop(self) {
        let Self { listeners, runtime } = self;
        drop(listeners);
        runtime.detach();
    }
}
