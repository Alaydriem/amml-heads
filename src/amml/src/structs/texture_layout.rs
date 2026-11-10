use crate::layouts::ResourceLocationLayout;

/// The texture group and `cg::ImageBuffer` layouts the texture bridge needs, read from the field entries when the
/// mod enables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureLayout {
    group_base: usize,
    buffer_words: usize,
    deleter_word: usize,
    data_word: usize,
    size_word: usize,
}

impl TextureLayout {
    /// The most words of `ImageBuffer` the bridge builds on its stack.
    pub const BUFFER_CAPACITY_WORDS: usize = 16;
    const WORD: usize = size_of::<usize>();

    /// `None` when the layout does not fit the bridge: an `ImageBuffer` larger than `BUFFER_CAPACITY_WORDS` words,
    /// a blob member that is not a word inside it, or a `ResourceLocation` whose size is not that of
    /// `ResourceLocationLayout`.
    pub fn new(
        group_base: usize,
        buffer_size: usize,
        buffer_deleter: usize,
        buffer_data: usize,
        buffer_byte_size: usize,
        resource_location_size: usize,
    ) -> Option<Self> {
        if resource_location_size != size_of::<ResourceLocationLayout>() {
            return None;
        }
        let buffer_words = Self::words(buffer_size)?;
        if buffer_words > Self::BUFFER_CAPACITY_WORDS {
            return None;
        }
        Some(Self {
            group_base,
            buffer_words,
            deleter_word: Self::member_word(buffer_deleter, buffer_words)?,
            data_word: Self::member_word(buffer_data, buffer_words)?,
            size_word: Self::member_word(buffer_byte_size, buffer_words)?,
        })
    }

    /// The offset from a `TextureGroup` to its `TextureGroupBase`, which `getTexture` runs on.
    pub fn group_base(&self) -> usize {
        self.group_base
    }

    /// `sizeof(ImageBuffer)` in words.
    pub fn buffer_words(&self) -> usize {
        self.buffer_words
    }

    /// The word of `ImageBuffer` that holds the blob deleter.
    pub fn deleter_word(&self) -> usize {
        self.deleter_word
    }

    /// The word of `ImageBuffer` that holds the blob data pointer.
    pub fn data_word(&self) -> usize {
        self.data_word
    }

    /// The word of `ImageBuffer` that holds the blob byte size.
    pub fn size_word(&self) -> usize {
        self.size_word
    }

    fn words(bytes: usize) -> Option<usize> {
        (bytes != 0 && bytes.is_multiple_of(Self::WORD)).then_some(bytes / Self::WORD)
    }

    fn member_word(offset: usize, buffer_words: usize) -> Option<usize> {
        (offset.is_multiple_of(Self::WORD) && offset / Self::WORD < buffer_words).then_some(offset / Self::WORD)
    }
}
