//! @alaydriem/heads as an AMML mod: reads player skins from the loader's network events and shows each player's face
//! on the locator bar and in chat.

#![deny(unsafe_code)]

#[forbid(unsafe_code)]
pub mod heads_mod;
#[forbid(unsafe_code)]
pub mod services;
#[forbid(unsafe_code)]
pub mod structs;

pub use heads_mod::HeadsMod;

amml_api::amml_mod!(HeadsMod);
