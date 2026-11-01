#![allow(unsafe_code)]

use heads_core::structs::SkinView;

use super::MemoryReader;
use crate::layouts::{ImageLayout, SerializedSkinLayout};
use crate::structs::PacketOffsets;

pub struct SkinReader;

impl SkinReader {
    // largest side a skin can have; stops garbage dimensions from making a huge slice
    const MAX_SIDE: u32 = 4096;
    // most animated images searched for the face
    const MAX_ANIMATED: usize = 16;

    /// A view of the skin pixels behind a `SerializedSkinRef`, or `None` when any link is null or the blob does
    /// not hold exactly `width * height * 4` bytes.
    ///
    /// # Safety
    /// `skin_ref` must be null or the address of a live `shared_ptr` to a `SerializedSkinImpl` laid out as
    /// `offsets` says. The returned view borrows game memory: the caller must use it only inside the detour that
    /// holds the packet, and must not keep it after the call returns.
    pub unsafe fn view(skin_ref: usize, offsets: &PacketOffsets) -> Option<SkinView<'static>> {
        // SAFETY: the caller guarantees `skin_ref` is null or a live shared_ptr, whose first word is the element.
        let element: usize = unsafe { MemoryReader::read(skin_ref)? };
        if element == 0 {
            return None;
        }
        // SAFETY: the element is a live SerializedSkinImpl and `skin_image` is inside it.
        let image: ImageLayout = unsafe { MemoryReader::read(element.checked_add(offsets.skin_image)?)? };
        Self::image_view(&image)
    }

    /// A view of the first animated image of type `Face` behind a `SerializedSkinRef`, or `None` when the skin has
    /// none or its blob does not hold exactly `width * height * 4` bytes.
    ///
    /// # Safety
    /// As for `view`.
    pub unsafe fn animated_face(skin_ref: usize) -> Option<SkinView<'static>> {
        // SAFETY: the caller guarantees `skin_ref` is null or a live shared_ptr, whose first word is the element.
        let element: usize = unsafe { MemoryReader::read(skin_ref)? };
        if element == 0 {
            return None;
        }
        // SAFETY: the vector is a member of the live SerializedSkinImpl.
        let [first, last] =
            unsafe { MemoryReader::read::<[usize; 2]>(element + SerializedSkinLayout::ANIMATED_IMAGES)? };
        if first == 0 || last < first {
            return None;
        }
        let count = ((last - first) / SerializedSkinLayout::ANIMATED_IMAGE_SIZE).min(Self::MAX_ANIMATED);
        (0..count).find_map(|index| {
            let base = first + index * SerializedSkinLayout::ANIMATED_IMAGE_SIZE;
            // SAFETY: the element lies inside the live vector, and its members are inside the element.
            let kind = unsafe { MemoryReader::read::<u32>(base + SerializedSkinLayout::ANIMATED_IMAGE_TYPE)? };
            if kind != SerializedSkinLayout::ANIMATED_TYPE_FACE {
                return None;
            }
            // SAFETY: as above.
            let image =
                unsafe { MemoryReader::read::<ImageLayout>(base + SerializedSkinLayout::ANIMATED_IMAGE_IMAGE)? };
            Self::image_view(&image)
        })
    }

    /// Whether the skin behind a `SerializedSkinRef` is a persona skin. False when a link is null.
    ///
    /// # Safety
    /// As for `view`.
    pub unsafe fn is_persona(skin_ref: usize) -> bool {
        // SAFETY: the caller guarantees `skin_ref` is null or a live shared_ptr, whose first word is the element.
        let Some(element) = (unsafe { MemoryReader::read::<usize>(skin_ref) }).filter(|element| *element != 0) else {
            return false;
        };
        // SAFETY: the flag is inside the live SerializedSkinImpl.
        unsafe { MemoryReader::read::<u8>(element + SerializedSkinLayout::IS_PERSONA) }.is_some_and(|flag| flag != 0)
    }

    // the pixels of an image that the game owns; the caller keeps the view only while the skin is live
    fn image_view(image: &ImageLayout) -> Option<SkinView<'static>> {
        if image.width == 0
            || image.height == 0
            || image.width > Self::MAX_SIDE
            || image.height > Self::MAX_SIDE
            || image.blob_data.is_null()
        {
            return None;
        }
        let expected = image.width as usize * image.height as usize * 4;
        if image.blob_size != expected {
            return None;
        }
        // SAFETY: the blob holds `blob_size` bytes owned by the live skin, which outlives the detour call.
        let bytes = unsafe { std::slice::from_raw_parts(image.blob_data, image.blob_size) };
        Some(SkinView::new(image.width, image.height, image.format, bytes))
    }
}
