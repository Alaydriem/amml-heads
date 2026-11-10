#![allow(unsafe_code)]

/// Reads plain values from game memory. Every read checks for a null address first.
pub struct MemoryReader;

impl MemoryReader {
    /// The value at `address`, or `None` when the address is null.
    ///
    /// # Safety
    /// A non-null `address` must be readable for `size_of::<T>()` bytes, and any bit pattern must be a valid `T`.
    pub unsafe fn read<T: Copy>(address: usize) -> Option<T> {
        if address == 0 {
            return None;
        }
        // SAFETY: the caller guarantees the address is readable; the read is unaligned-safe.
        Some(unsafe { (address as *const T).read_unaligned() })
    }
}
