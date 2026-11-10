use std::mem::offset_of;

use alaydriem_heads::layouts::GuiMessageLayout;
use amml_msvc::MsvcString;

use super::{FakeHeap, FakeMemory};

/// A built `GuiMessage` (0x110 bytes) whose strings live on the `FakeHeap`.
pub struct FakeGuiMessage {
    memory: FakeMemory,
}

impl FakeGuiMessage {
    pub fn new(kind: i32, username: &str, full: &str, filtered: Option<&str>, xuid: &str) -> Self {
        let mut memory = FakeMemory::new(size_of::<GuiMessageLayout>());
        memory.put(offset_of!(GuiMessageLayout, kind), kind);
        let mut message = Self { memory };
        message.put_string(offset_of!(GuiMessageLayout, message), "");
        message.put_string(offset_of!(GuiMessageLayout, tts_message), "");
        message.put_string(offset_of!(GuiMessageLayout, username), username);
        message.put_string(offset_of!(GuiMessageLayout, full_string), full);
        message.put_string(offset_of!(GuiMessageLayout, author_xuid), xuid);
        if let Some(filtered) = filtered {
            message.put_string(offset_of!(GuiMessageLayout, filtered_full_string), filtered);
            message.memory.put(offset_of!(GuiMessageLayout, filtered_full_string) + 0x20, 1u8);
        }
        message
    }

    pub fn addr(&self) -> usize {
        self.memory.addr()
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.memory.bytes()
    }

    pub fn full(&self) -> String {
        self.layout().full_string.to_str().unwrap().to_string()
    }

    pub fn filtered(&self) -> Option<String> {
        self.layout().filtered_full_string.get().map(|text| text.to_str().unwrap().to_string())
    }

    fn layout(&self) -> &GuiMessageLayout {
        // SAFETY: the buffer is a whole GuiMessage and lives as long as `self`.
        unsafe { &*(self.memory.addr() as *const GuiMessageLayout) }
    }

    fn put_string(&mut self, offset: usize, text: &str) {
        // SAFETY: the fake heap is a working allocator.
        let string = unsafe { MsvcString::new_in(&FakeHeap::heap(), text) }.unwrap();
        // SAFETY: the string slot is inside the buffer, 8 byte aligned.
        unsafe { ((self.memory.addr() + offset) as *mut MsvcString).write(string) };
    }
}
