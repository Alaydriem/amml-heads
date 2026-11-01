# Task 1 ledger: chat heads RE facts

Commands: `mise run inspect` (disasm, callers, xrefs, vtable, packets, function) and `mise run decompile` in `../amml` on the 26.60.05 exe. No build, no test, no game run (docs only).

| Fact | Value | Tag |
| --- | --- | --- |
| GuiMessage constructor RVA | `0xe8d420` | [proven] |
| Constructor args | rcx this, edx type, r8 username*, r9 message*, [rsp+28] filtered optional* (callee frees it), [rsp+30] tts*, [rsp+38] authorXuid*, [rsp+40] float lifetime, [rsp+48] bool forceVisible, [rsp+50] bool ttsRequired; returns this | [proven] |
| Builds mFullString / mFilteredFullString | yes; `<user> msg` when username not empty, else msg | [proven] |
| Offsets mType/mMessage/mFilteredMessage/mTTSMessage/mUsername/mFullString/mFilteredFullString/mAuthorXuid | 0x00/0x08/0x28/0x50/0x70/0x90/0xB0/0xD8 | [proven] |
| Engaged bytes | mFilteredMessage +0x48, mFilteredFullString +0xD0 | [proven] |
| forceVisible / ttsRequired / lifetime | +0xF8 / +0xF9 / +0xFC | [proven] |
| sizeof(GuiMessage) | 0x110 | [proven] |
| Constructor callers | `0xe8e420` (_addMessage, all chat), `0xe8cea0` (type 9, other list) | [proven] |
| MessageConfig ctor (do not hook) | `0xe92a20` | [proven] |
| HUD text source | +0x90 or +0xB0 (`0x4b5eb90`) | [proven] |
| Chat window text source | +0x90 or +0xB0 (`0x4ff51b0`, `0x4ff46d0`) | [proven] |
| Insertion point covers HUD and chat window | the constructor | [proven] |
| TextPacket type to MessageType | Raw 2, Chat 1, Translate 3, System 4, Whisper 5, Announcement 8 | [proven] |
| Chat full string | `<Name> text` (type 1) | [proven] |
| Whisper full string | `<Name> §7§oName whispers to you: text` (type 5) | [proven] from code |
| /me full string | `* Name action` (type 1, empty username) | [proven] |
| Join full string | `§eName joined the game` (type 3) | [proven] |
| Leave full string | `§eName left the game` (type 3) | [proven] |
| BDS sends the same packets as the integrated server | yes | [inference] |
| /me mAuthorXuid comes from the command origin | yes | [inference] |
| PlayerListAddEntry.mName | +0x20 | [proven] |
| PlayerListAddEntry.mXUID | +0x40 | [inference] |
| BitmapFont vtable / slot 29 | `0xf0ea8a0` / `0x15902c0` (rcx font, edx char, r8b force unicode, xmm0 result) | [proven] |
| Chat font = FontHandle+0x30 | not confirmed; chat labels use `$chat_font_type` | [inference] |
| Smooth font | Noto Sans = msdf `OldSmoothFont` in a font alias; not BitmapFont | [proven] format / [inference] class |
| E1 icon in vanilla UI JSON | none; icons come from lang tokens (`:armor:` and others) | [proven] |
| Screen outside a world with an E1 icon | How to Play, topic Armor (U+E101) | [inference] |

Task Completed
