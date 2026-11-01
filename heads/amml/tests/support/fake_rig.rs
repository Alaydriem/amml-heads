use alaydriem_heads::services::{RenderState, TextureBridge};
use crossbeam_channel::{Sender, bounded};
use heads_core::config::HeadsSettings;
use heads_core::services::FaceStore;
use heads_core::structs::{FaceImage, FaceUpdate};

use super::{FakeClient, FakeGame, FakeWaypoints, WaypointSpec};

/// A render state wired to fake game memory: a client, its locator bar list and fake texture functions.
pub struct FakeRig {
    pub state: RenderState,
    pub sender: Sender<FaceUpdate>,
    pub waypoints: FakeWaypoints,
    pub client: FakeClient,
}

impl FakeRig {
    pub fn new(specs: &[WaypointSpec], settings: HeadsSettings) -> Self {
        let waypoints = FakeWaypoints::new(specs);
        let client = FakeClient::new(waypoints.head());
        let (sender, receiver) = bounded(256);
        let state = RenderState::new(
            FaceStore::new(128, 128),
            receiver,
            settings,
            TextureBridge::new(FakeGame::texture_fns()),
            client.access(),
            Some(FakeGame::heap()),
        );
        Self { state, sender, waypoints, client }
    }

    pub fn face(pixel: [u8; 4]) -> Box<FaceImage> {
        let mut pixels = [0u8; 4096];
        for chunk in pixels.chunks_mut(4) {
            chunk.copy_from_slice(&pixel);
        }
        Box::new(FaceImage::new(pixels))
    }

    pub fn add_face(&self, n: u8, actor: i64, pixel: [u8; 4]) {
        self.sender.send(FaceUpdate::Add { uuid: [n; 16], actor, face: Self::face(pixel) }).unwrap();
    }
}
