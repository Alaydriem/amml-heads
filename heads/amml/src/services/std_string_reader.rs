#![allow(unsafe_code)]

use super::MemoryReader;

/// Reads a MSVC `std::string` in game memory: size at +0x10, capacity at +0x18, the bytes inline up to 15
/// characters and behind the pointer in the first word above that.
pub struct StdStringReader;

impl StdStringReader {
    const INLINE_CAPACITY: usize = 15;

    /// The length of the string, or `None` when its size and capacity do not look sane.
    ///
    /// # Safety
    /// `address` must be null or the address of a live `std::string`.
    pub unsafe fn size(address: usize) -> Option<usize> {
        if address == 0 {
            return None;
        }
        // SAFETY: the caller guarantees the string is live; the reader rejects null.
        let (size, capacity) = unsafe {
            (
                MemoryReader::read::<usize>(address.checked_add(0x10)?)?,
                MemoryReader::read::<usize>(address.checked_add(0x18)?)?,
            )
        };
        (size <= capacity).then_some(size)
    }

    /// The text of the string, or `None` when it does not look sane or is longer than `max` bytes.
    ///
    /// # Safety
    /// `address` must be null or the address of a live `std::string`.
    pub unsafe fn text(address: usize, max: usize) -> Option<String> {
        // SAFETY: the caller guarantees the string is live.
        let size = unsafe { Self::size(address) }?;
        if size > max {
            return None;
        }
        let data = if size <= Self::INLINE_CAPACITY && Self::is_inline(address)? {
            address
        } else {
            // SAFETY: a heap string keeps its data pointer in the first word.
            let pointer = unsafe { MemoryReader::read::<usize>(address) }?;
            if pointer == 0 {
                return None;
            }
            pointer
        };
        // SAFETY: `size` bytes are readable at `data`, inline or on the heap.
        let bytes = unsafe { std::slice::from_raw_parts(data as *const u8, size) };
        Some(String::from_utf8_lossy(bytes).into_owned())
    }

    // whether the capacity says the bytes are inline
    fn is_inline(address: usize) -> Option<bool> {
        // SAFETY: only reached after `size` read the same string.
        let capacity = unsafe { MemoryReader::read::<usize>(address + 0x18) }?;
        Some(capacity <= Self::INLINE_CAPACITY)
    }
}
