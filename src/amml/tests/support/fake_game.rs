use std::cell::{Cell, RefCell};

use alaydriem_heads::layouts::{ImageDescriptionLayout, PathViewLayout, ResourceLocationLayout, TexturePtrLayout};
use alaydriem_heads::structs::ResolvedTextureFns;
use amml_msvc::MsvcHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameCall {
    Upload { group: usize, slot: u64, pixel: [u8; 4] },
    Get { group: usize, slot: u64 },
    Unload { group: usize, slot: u64 },
    PageUpload { group: usize, bytes: usize, pixel: [u8; 4] },
}

thread_local! {
    static LOG: RefCell<Vec<GameCall>> = const { RefCell::new(Vec::new()) };
    static FAIL_GET: Cell<bool> = const { Cell::new(false) };
    static BAD_BUFFER: Cell<bool> = const { Cell::new(false) };
    static GROUP_AVAILABLE: Cell<bool> = const { Cell::new(true) };
    static LOCATIONS_MADE: Cell<usize> = const { Cell::new(0) };
}

/// Fake texture group functions with the 26.60 signatures, recording each call on the current thread.
pub struct FakeGame;

impl FakeGame {
    pub const GROUP: usize = 0x5000;
    pub const GET_GROUP_OFFSET: usize = 0x18;
    pub const PAGE_SLOT: u64 = u64::MAX;

    pub fn texture_fns() -> ResolvedTextureFns {
        ResolvedTextureFns {
            resource_location_ctor: location_ctor as *const () as usize,
            image_buffer_ctor: image_buffer_ctor as *const () as usize,
            upload: upload as *const () as usize,
            get: get as *const () as usize,
            unload: unload as *const () as usize,
        }
    }

    /// A heap whose functions are never reached: the fake locations hold no heap strings.
    pub fn heap() -> MsvcHeap {
        MsvcHeap::new(heap_new, heap_delete)
    }

    /// The code addresses of the heap functions, as the recipe entries resolve them.
    pub fn heap_addresses() -> (usize, usize) {
        (heap_new as *const () as usize, heap_delete as *const () as usize)
    }

    pub fn take_log() -> Vec<GameCall> {
        LOG.with(|log| std::mem::take(&mut *log.borrow_mut()))
    }

    /// How many resource locations the fake constructor built on this thread.
    pub fn locations_made() -> usize {
        LOCATIONS_MADE.with(Cell::get)
    }

    pub fn fail_get(fail: bool) {
        FAIL_GET.with(|flag| flag.set(fail));
    }

    pub fn bad_buffer(bad: bool) {
        BAD_BUFFER.with(|flag| flag.set(bad));
    }

    pub fn group_available(available: bool) {
        GROUP_AVAILABLE.with(|flag| flag.set(available));
    }

    pub fn is_group_available() -> bool {
        GROUP_AVAILABLE.with(Cell::get)
    }

    fn record(call: GameCall) {
        LOG.with(|log| log.borrow_mut().push(call));
    }
}

extern "C" fn location_ctor(
    out: *mut ResourceLocationLayout,
    view: *const PathViewLayout,
) -> *mut ResourceLocationLayout {
    // SAFETY: the bridge passes a live path view and a writable location buffer.
    unsafe {
        let view = &*view;
        let text = std::str::from_utf8(std::slice::from_raw_parts(view.data, view.size)).unwrap();
        LOCATIONS_MADE.with(|made| made.set(made.get() + 1));
        (*out).path_hash = match text.strip_prefix("heads/slot_") {
            Some(slot) => slot.parse().unwrap(),
            None => {
                assert_eq!(text, "font/glyph_E1");
                FakeGame::PAGE_SLOT
            }
        };
    }
    out
}

extern "C" fn image_buffer_ctor(buf: *mut usize, desc: *const ImageDescriptionLayout) -> *mut usize {
    // SAFETY: the bridge passes a live description and a writable 0x30 byte buffer.
    unsafe {
        let side = (*desc).width;
        assert_eq!(*desc, ImageDescriptionLayout::face(side));
        let len = if BAD_BUFFER.with(Cell::get) { 100 } else { (side * side * 4) as usize };
        let data = Box::leak(vec![0u8; len].into_boxed_slice());
        buf.write(deleter as *const () as usize);
        buf.add(1).write(data.as_mut_ptr() as usize);
        buf.add(2).write(len);
    }
    buf
}

unsafe extern "C" fn heap_new(_bytes: usize) -> *mut u8 {
    std::ptr::null_mut()
}

unsafe extern "C" fn heap_delete(_ptr: *mut u8, _bytes: usize) {}

extern "C" fn deleter(_data: *mut u8) {}

extern "C" fn upload(group: usize, location: *const ResourceLocationLayout, buf: *const usize) -> usize {
    // SAFETY: the bridge passes a live location and an image buffer holding at least 4 bytes.
    let (slot, pixel, bytes) = unsafe {
        let data = *buf.add(1) as *const u8;
        ((*location).path_hash, [*data, *data.add(1), *data.add(2), *data.add(3)], *buf.add(2))
    };
    if slot == FakeGame::PAGE_SLOT {
        FakeGame::record(GameCall::PageUpload { group, bytes, pixel });
    } else {
        FakeGame::record(GameCall::Upload { group, slot, pixel });
    }
    0
}

extern "C" fn get(
    group: usize,
    out: *mut usize,
    location: *const ResourceLocationLayout,
    _force_reload: u8,
    _layer: u64,
    _set_layer: u32,
) -> usize {
    // SAFETY: the bridge passes a live location and a writable, zeroed TexturePtr.
    let slot = unsafe { (*location).path_hash };
    FakeGame::record(GameCall::Get { group, slot });
    if !FAIL_GET.with(Cell::get) {
        // SAFETY: the out buffer is a 0x20 byte TexturePtr; the control blocks stay null, so releasing is a no-op.
        unsafe {
            out.write(0x1000 + slot as usize);
            out.add(2).write(0x2000 + slot as usize);
        }
    }
    out as usize
}

extern "C" fn unload(group: usize, location: *const ResourceLocationLayout, _flag: u8) -> u8 {
    // SAFETY: the bridge passes a live location.
    let slot = unsafe { (*location).path_hash };
    FakeGame::record(GameCall::Unload { group, slot });
    1
}

// keeps the TexturePtrLayout import used for the size check below
const _: () = assert!(size_of::<TexturePtrLayout>() == 0x20);
