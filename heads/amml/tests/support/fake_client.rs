use alaydriem_heads::structs::ClientAccess;

use super::{ENTITY_VTABLE, FakeGame, FakeMemory};

const LOCATOR_BAR_OFFSET: usize = 0xEF0;
const GET_LOCAL_PLAYER_SLOT: usize = 3;
const GET_TEXTURE_GROUP_SLOT: usize = 5;

extern "C" fn get_local_player(client: usize) -> usize {
    // SAFETY: the fake client buffer holds the player address at +8.
    unsafe { ((client + 8) as *const usize).read() }
}

extern "C" fn get_texture_group(client: usize, out: *mut usize) -> *mut usize {
    let _ = client;
    let group = if FakeGame::is_group_available() { FakeGame::GROUP } else { 0 };
    // SAFETY: the render state passes a writable shared_ptr; the control block stays null.
    unsafe {
        out.write(group);
        out.add(1).write(0);
    }
    out
}

/// A fake `ClientInstance` with a vtable, and a `LocalPlayer` whose locator bar list starts at `list_head`.
pub struct FakeClient {
    client: FakeMemory,
    _vtable: FakeMemory,
    _player: FakeMemory,
}

impl FakeClient {
    pub fn new(list_head: usize) -> Self {
        let mut player = FakeMemory::new(0x1000);
        player.put(LOCATOR_BAR_OFFSET + 0x10, list_head);
        let mut vtable = FakeMemory::new(8 * 8);
        vtable.put(GET_LOCAL_PLAYER_SLOT * 8, get_local_player as *const () as usize);
        vtable.put(GET_TEXTURE_GROUP_SLOT * 8, get_texture_group as *const () as usize);
        let mut client = FakeMemory::new(0x20);
        client.put(0, vtable.addr());
        client.put(8, player.addr());
        Self { client, _vtable: vtable, _player: player }
    }

    pub fn addr(&self) -> usize {
        self.client.addr()
    }

    pub fn access(&self) -> ClientAccess {
        ClientAccess::new(GET_LOCAL_PLAYER_SLOT, GET_TEXTURE_GROUP_SLOT, LOCATOR_BAR_OFFSET, ENTITY_VTABLE)
    }
}
