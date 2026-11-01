use std::alloc::{Layout, alloc, dealloc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, PoisonError};

use amml_msvc::MsvcHeap;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static FREED: AtomicUsize = AtomicUsize::new(0);
static FREED_SIZES: Mutex<Vec<usize>> = Mutex::new(Vec::new());

/// A game heap stand-in backed by the Rust allocator, so strings built and freed through `MsvcHeap` are real. It
/// counts the blocks it gives and frees, and keeps the size passed to each free.
pub struct FakeHeap;

impl FakeHeap {
    pub fn heap() -> MsvcHeap {
        MsvcHeap::new(heap_new, heap_delete)
    }

    pub fn allocated() -> usize {
        ALLOCATED.load(Ordering::SeqCst)
    }

    pub fn freed() -> usize {
        FREED.load(Ordering::SeqCst)
    }

    /// The size passed to each free, in order.
    pub fn freed_sizes() -> Vec<usize> {
        FREED_SIZES.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }
}

unsafe extern "C" fn heap_new(bytes: usize) -> *mut u8 {
    ALLOCATED.fetch_add(1, Ordering::SeqCst);
    // SAFETY: `MsvcHeap` never asks for zero bytes.
    unsafe { alloc(Layout::from_size_align(bytes, 16).unwrap()) }
}

unsafe extern "C" fn heap_delete(ptr: *mut u8, bytes: usize) {
    FREED.fetch_add(1, Ordering::SeqCst);
    FREED_SIZES.lock().unwrap_or_else(PoisonError::into_inner).push(bytes);
    // SAFETY: `MsvcHeap` frees with the size it allocated, so the layout matches.
    unsafe { dealloc(ptr, Layout::from_size_align(bytes, 16).unwrap()) }
}
