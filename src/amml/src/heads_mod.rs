use std::sync::{Arc, Mutex, PoisonError};

use amml_api::amml_abi::{
    ChatMessageAdded, ClientTick, InjectedInputReleased, InputLeaseLost, InputLeaseResumed, InputLeaseSuspended,
    LocatorBarIconDraw, ModsLoaded, PlayerListUpdate, PlayerSkinChanged, ProviderGone, ProviderReady, ScreenshotTaken,
    ScreenshotWritten, TexturesReloaded, WorldLeft,
};
use amml_api::{AmmlMod, ApiError, Area, Ctx, DisableCtx, Uses};

use crate::services::{HeadsLink, HeadsListeners, HeadsRuntime};

// what the enabled mod owns. `disable` takes it, which drops the listeners and closes the link
static RUNNING: Mutex<Option<HeadsListeners>> = Mutex::new(None);

// the AMML entry point: enable starts the manager link and the listeners, disable removes them and leaves the mod
// manager
pub struct HeadsMod;

impl HeadsMod {
    // the side features of a skin and of the chat, with the name `features_off` gives them
    const SIDE_FEATURES: [(&'static str, Area); 4] = [
        ("chat", Area::UI_INLINE_ICONS),
        ("persona", Area::NETWORK_SKIN_ANIMATED),
        ("geometry", Area::NETWORK_SKIN_GEOMETRY),
        ("diagnostics", Area::NETWORK_SKIN_DETAILS),
    ];

    // the side features the loader does not serve, comma separated; empty when it serves all of them
    fn features_off(ctx: &Ctx) -> String {
        let off: Vec<&str> =
            Self::SIDE_FEATURES.iter().filter(|(_, area)| !ctx.supports(*area)).map(|(name, _)| *name).collect();
        off.join(",")
    }
}

impl AmmlMod for HeadsMod {
    const NAMESPACE: &'static str = "alaydriem";
    const NAME: &'static str = "heads";
    const USES: Uses = amml_api::amml_uses! {
        feature "faces" {
            required PlayerListUpdate;
            required PlayerSkinChanged;
            required LocatorBarIconDraw;
            required WorldLeft;
            required TexturesReloaded;
            required area Area::UI_TEXTURES;
            required area Area::NETWORK_SKIN;
        }
        feature "settings" {
            required ClientTick;
            required ProviderReady;
            required ProviderGone;
            required ModsLoaded;
            // The link listens to these when the loader serves them
            optional InputLeaseSuspended;
            optional InputLeaseResumed;
            optional InputLeaseLost;
            optional InjectedInputReleased;
            optional ScreenshotTaken;
            optional ScreenshotWritten;
            optional area Area::INPUT_LEASE;
        }
        feature "chat" {
            required ChatMessageAdded;
            required area Area::UI_INLINE_ICONS;
        }
        feature "persona" {
            required area Area::NETWORK_SKIN_ANIMATED;
        }
        feature "geometry" {
            required area Area::NETWORK_SKIN_GEOMETRY;
        }
        feature "diagnostics" {
            required area Area::NETWORK_SKIN_DETAILS;
        }
    };

    fn enable(ctx: &Ctx) -> Result<(), ApiError> {
        let log = ctx.log();
        let link = match HeadsLink::start(ctx) {
            Ok(link) => Some(link),
            Err(error) => {
                log.warn("heads mod manager not reached; defaults in use", &[("error", &error.to_string())]);
                None
            }
        };
        let runtime = Arc::new(HeadsRuntime::new(log, ctx.textures(), ctx.chat()));
        runtime.attach(link);
        let listeners = HeadsListeners::start(ctx, runtime).inspect_err(|error| {
            log.error("heads off", &[("error", &error.to_string())]);
        })?;
        *RUNNING.lock().unwrap_or_else(PoisonError::into_inner) = Some(listeners);
        log.info("heads enabled", &[("features_off", &Self::features_off(ctx))]);
        Ok(())
    }

    fn disable(_ctx: &Ctx, _how: &DisableCtx) {
        let running = RUNNING.lock().unwrap_or_else(PoisonError::into_inner).take();
        if let Some(listeners) = running {
            listeners.stop();
        }
    }
}
