use heads_core::services::TagCodec;
use heads_core::structs::TagColor;

use super::FakeMemory;

pub const ENTITY_VTABLE: usize = 0xF13DFD0;
const PLAIN_VTABLE: usize = 0xF114660;
const WAYPOINT_SIZE: usize = 0x98;
const NODE_SIZE: usize = 0x28;

#[derive(Clone, Copy)]
pub struct WaypointSpec {
    pub visible: bool,
    pub color: Option<[f32; 4]>,
    pub actor: Option<i64>,
}

impl WaypointSpec {
    pub fn plain(color: Option<[f32; 4]>) -> Self {
        Self { visible: true, color, actor: None }
    }

    pub fn player(actor: i64, color: Option<[f32; 4]>) -> Self {
        Self { visible: true, color, actor: Some(actor) }
    }

    pub fn hidden() -> Self {
        Self { visible: false, color: Some([0.5; 4]), actor: None }
    }
}

/// A circular `ClientLocatorBar` list of fake waypoints around a sentinel node.
pub struct FakeWaypoints {
    sentinel: FakeMemory,
    nodes: Vec<FakeMemory>,
    waypoints: Vec<FakeMemory>,
}

impl FakeWaypoints {
    pub fn new(specs: &[WaypointSpec]) -> Self {
        let mut list = Self { sentinel: FakeMemory::new(NODE_SIZE), nodes: Vec::new(), waypoints: Vec::new() };
        for (i, spec) in specs.iter().enumerate() {
            list.add(i as u8, *spec);
        }
        list
    }

    fn add(&mut self, seed: u8, spec: WaypointSpec) {
        let mut waypoint = FakeMemory::new(WAYPOINT_SIZE);
        waypoint.fill_pattern(seed);
        waypoint.put(0, if spec.actor.is_some() { ENTITY_VTABLE } else { PLAIN_VTABLE });
        waypoint.put(0x60, u8::from(spec.visible));
        match spec.color {
            Some(color) => {
                waypoint.put(0x18, color);
                waypoint.put(0x28, 1u8);
            }
            None => waypoint.put(0x28, 0u8),
        }
        if let Some(actor) = spec.actor {
            waypoint.put(0x78, actor);
        }
        let mut node = FakeMemory::new(NODE_SIZE);
        node.put(0x20, waypoint.addr());
        self.waypoints.push(waypoint);
        self.nodes.push(node);
        self.relink();
    }

    fn relink(&mut self) {
        let sentinel = self.sentinel.addr();
        let addrs: Vec<usize> = self.nodes.iter().map(FakeMemory::addr).collect();
        self.sentinel.put(0, addrs.first().copied().unwrap_or(sentinel));
        for (i, node) in self.nodes.iter_mut().enumerate() {
            node.put(0, addrs.get(i + 1).copied().unwrap_or(sentinel));
        }
    }

    pub fn head(&self) -> usize {
        self.sentinel.addr()
    }

    pub fn len(&self) -> usize {
        self.waypoints.len()
    }

    pub fn bytes(&self, index: usize) -> Vec<u8> {
        self.waypoints[index].bytes()
    }

    /// The colour packed the way the game packs it, from the bytes now in the waypoint.
    pub fn current_packed(&self, index: usize) -> u32 {
        let color: [f32; 4] = self.waypoints[index].get(0x18);
        TagCodec::pack(TagColor::new(color[0], color[1], color[2], color[3]))
    }
}
