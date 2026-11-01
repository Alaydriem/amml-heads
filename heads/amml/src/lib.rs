//! @alaydriem/heads as an AMML mod: reads player skins from packets and draws their faces on the locator bar.

#![deny(unsafe_code)]

pub mod detours;
pub mod entries;
pub mod heads_mod;
pub mod layouts;
pub mod services;
pub mod structs;
pub mod traits;
pub mod types;

pub use heads_mod::HeadsMod;

amml_sdk::amml_mod!(HeadsMod);
