use std::alloc::{Layout, alloc, dealloc};
use std::cell::{Cell, RefCell};

use amml_msvc::{MsvcHeap, MsvcString};

thread_local! {
    static JSON: RefCell<String> = const { RefCell::new(String::new()) };
    static THIS: Cell<usize> = const { Cell::new(0) };
    static LIVE: Cell<i64> = const { Cell::new(0) };
}

/// A fake `Json::Value::toStyledString` that builds a known text in a game-heap string, and a counting heap.
pub struct FakeGeometry;

impl FakeGeometry {
    pub fn to_styled_string_address() -> usize {
        to_styled_string as *const () as usize
    }

    pub fn heap() -> MsvcHeap {
        MsvcHeap::new(heap_new, heap_delete)
    }

    /// The text the next calls build.
    pub fn set_json(text: &str) {
        JSON.with(|json| *json.borrow_mut() = text.to_string());
    }

    /// The `this` the last call got.
    pub fn last_this() -> usize {
        THIS.with(Cell::get)
    }

    /// Blocks allocated by the fake heap and not freed yet.
    pub fn live_allocations() -> i64 {
        LIVE.with(Cell::get)
    }
}

extern "C" fn to_styled_string(this: usize, out: *mut MsvcString) -> *mut MsvcString {
    THIS.with(|cell| cell.set(this));
    let text = JSON.with(|json| json.borrow().clone());
    // SAFETY: the fake heap functions below are a valid allocator pair.
    let string = unsafe { MsvcString::new_in(&FakeGeometry::heap(), &text) }.unwrap();
    // SAFETY: the caller passes room for a std::string, like the game does.
    unsafe { out.write(string) };
    out
}

unsafe extern "C" fn heap_new(bytes: usize) -> *mut u8 {
    LIVE.with(|live| live.set(live.get() + 1));
    // SAFETY: the layout has a non-zero size and a power of two alignment.
    unsafe { alloc(Layout::from_size_align(bytes.max(1), 16).unwrap()) }
}

unsafe extern "C" fn heap_delete(ptr: *mut u8, bytes: usize) {
    LIVE.with(|live| live.set(live.get() - 1));
    // SAFETY: `ptr` came from `heap_new` with the same size.
    unsafe { dealloc(ptr, Layout::from_size_align(bytes.max(1), 16).unwrap()) };
}
