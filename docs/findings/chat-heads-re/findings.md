# Chat heads: reverse-engineering facts (26.60.05)

Date: 2026-10-31. Build: 1.26.60.05, BuildId `6ad79ee4-17d51891a1119914`.
Exe: `%LOCALAPPDATA%\amml\instances\1.26.60.05-locator-heads\Minecraft.Windows.exe`.
Tools: `mise run inspect` (disasm, callers, xrefs, vtable, packets, function) and `mise run decompile` (Ghidra) in `../amml`. The game was not started.

Tags: [proven] = read from code in this exe (RVA and instructions are given). [inference] = not confirmed. Each inference says what would confirm it. All RVAs are relative to the image base `0x140000000`.

## Summary

| Fact | Value | Tag |
| --- | --- | --- |
| `GuiMessage` constructor | `0xe8d420` | [proven] |
| Constructor builds `mFullString` and `mFilteredFullString` | yes | [proven] |
| Field offsets (all 8 header values) | 0x00, 0x08, 0x28, 0x50, 0x70, 0x90, 0xB0, 0xD8, as the header | [proven] |
| `sizeof(GuiMessage)` | 0x110 | [proven] |
| Callers of the constructor | 2: `GuiData::_addMessage` `0xe8e420` (all chat), `0xe8cea0` (type 9, other list) | [proven] |
| HUD chat text | `mFullString` (+0x90), or `mFilteredFullString` (+0xB0) when the filter is on | [proven] |
| Chat window (T) text | same fields, same rule | [proven] |
| Insertion point | the constructor covers HUD and chat window | [proven] |
| `PlayerListAddEntry.mName` | +0x20 | [proven] |
| `PlayerListAddEntry.mXUID` | +0x40 | [inference] |
| `BitmapFont` vtable, slot 29 | `0xf0ea8a0`, slot 29 = `0x15902c0` | [proven] |
| Chat font = `FontHandle` default font | not confirmed | [inference] |
| Smooth font ("Noto Sans") | msdf font `OldSmoothFont` in a font alias, not `BitmapFont` | [proven] format / [inference] class |
| Screen outside a world with an E1 icon | How to Play, topic Armor (`:armor:` = U+E101) | [inference] |

## Step 1: `GuiMessage` constructor

### Where it is

`GuiMessage::GuiMessage` is at RVA `0xe8d420` (function `0xe8d420..0xe8e167`). [proven]

How it was found:

1. `ClientNetworkHandler` vtable `0xf2cfe40` (slot 179 = `0x6bdd410`, the existing `handle.PlayerListPacket` entry; base = `0xf2d03d8 - 179*8`). `inspect packets --create-packet 23768c0` gives the `TextPacket` handler slot 244 = `0x6b9e240`. [proven]
2. The handler switches on the packet type and calls one `GuiData` display function per type (Chat `0xe93840`, Whisper `0xe94740`, Translate `0xe94110`, Raw `0xe927b0`). [proven]
3. Each display function calls `0xe92a20`, then `GuiData::_addMessage` `0xe8e420`. `0xe92a20` builds a `MessageConfig` (11 arguments, a similar signature) and builds no full string. Do not hook it. [proven]
4. `_addMessage` calls `0xe8d420` at `0xe8e9d5` with the `MessageConfig` strings. That function builds the full string. It is the constructor. [proven]

### Registers and stack arguments [proven]

Prologue (`0xe8d420`): 8 pushes, `sub rsp,0xe8`, `lea rbp,[rsp+0x80]`. So the entry stack slot `[rsp+0x28 + (k-5)*8]` is `[rbp+0xd0 + (k-5)*8]` in the body.

| Arg | Location | Type | Evidence |
| --- | --- | --- | --- |
| this | `rcx` | `GuiMessage*` | `0xe8d44e mov rsi,rcx` |
| type | `edx` | `MessageType` (int) | `0xe8d451 mov [rcx],edx` |
| username | `r8` | `std::string const*` | `0xe8d44a mov [rbp+0x38],r8`, later copied to +0x70 |
| message | `r9` | `std::string const*` | `0xe8d45e mov r14,[r9+0x10]`, copied to +0x08 |
| filteredMessage | `[rsp+0x28]` | `std::optional<std::string>*` (by value; see below) | `0xe8d539 mov rdx,[rbp+0xd0]`, copied to +0x28 |
| ttsMessage | `[rsp+0x30]` | `std::string const*` | `0xe8d546 mov r14,[rbp+0xd8]`, copied to +0x50 |
| authorXuid | `[rsp+0x38]` | `std::string const*` | copied to +0xD8 (`0xe8d819 mov [rsi+0xd8],r12`) |
| lifetime | `[rsp+0x40]` | `float` | `0xe8d83f movss xmm0,[rbp+0xe8]`, `0xe8d861 movss [rsi+0xfc],xmm0` |
| forceVisible | `[rsp+0x48]` | `bool` | `0xe8d84e movzx ecx,byte ptr [rbp+0xf0]`, `0xe8d855 mov [rsi+0xf8],cl` |
| ttsRequired | `[rsp+0x50]` | `bool` | `0xe8d847 movzx eax,byte ptr [rbp+0xf8]`, `0xe8d85b mov [rsi+0xf9],al` |
| return | `rax` | `this` | decompile returns `param_1` |

The order matches the header: `(MessageType, username, message, filteredMessage, ttsMessage, authorXuid, float, bool, bool)`.

**The filtered message is passed by value.** MSVC passes a pointer to a temporary, and the callee destroys it: at the end of the constructor, when `param_5` is engaged and its capacity is 0x10 or more, it frees `*param_5`. [proven] A detour must call the original with the same pointer, and must not read the 5th argument after the original returns. Read the strings from `this` after the call.

### It builds `mFullString` and `mFilteredFullString` itself [proven]

After it copies the arguments, the constructor sets `mFullString` (+0x90) to empty (`0xe8d71a movups [rsi+0x90],xmm0`) and disengages `mFilteredFullString` (`0xe8d737 mov byte ptr [rsi+0xd0],0`). Then:

- `username.size() == 0`: `mFullString = message`. If `filteredMessage` is engaged, `mFilteredFullString = filteredMessage`; else it stays disengaged.
- `username.size() != 0`: `mFullString = "<" + username + "> " + message` (`0xe8da8f mov byte ptr [r12],0x3c` writes `<`; then `0x203e` = `"> "` is appended; then the message). If `filteredMessage` is engaged and not empty: `mFilteredFullString = "<" + username + "> " + filteredMessage`.

Important: the `<name> ` prefix depends only on `username` being not empty. It does not depend on the type.

### Field offsets [proven]

| Field | Offset | Writing instruction |
| --- | --- | --- |
| `mType` | 0x00 | `0xe8d451 mov [rcx],edx` |
| `mMessage` | 0x08 | `0xe8d456 movups [rcx+8],xmm0`, then copy of `r9` |
| `mFilteredMessage` (`optional<string>`, engaged byte at +0x48) | 0x28 | `0xe8d531 lea rcx,[rsi+0x28]` + optional copy `0x110e50` |
| `mTTSMessage` | 0x50 | `0xe8d554 movups [rsi+0x50],xmm0`, `0xe8d606 mov [rsi+0x50],r12` |
| `mUsername` | 0x70 | `0xe8d62d movups [rsi+0x70],xmm0`, `0xe8d6ed mov [rsi+0x70],r12` |
| `mFullString` | 0x90 | `0xe8d71a movups [rsi+0x90],xmm0`, built later |
| `mFilteredFullString` (`optional<string>`, engaged byte at +0xD0) | 0xB0 | `0xe8d737 mov byte ptr [rsi+0xd0],0`, `0xe8d9d7 cmp byte ptr [rsi+0xd0],1` |
| `mAuthorXuid` | 0xD8 | `0xe8d73e movups [rsi+0xd8],xmm0`, `0xe8d819 mov [rsi+0xd8],r12` |
| forceVisible | 0xF8 | `0xe8d855` |
| ttsRequired | 0xF9 | `0xe8d85b` |
| lifetime (float) | 0xFC | `0xe8d861` |
| (cleared) | 0x100 (u16), 0x104 (u32), 0x108 (u8) | `0xe8d869`, `0xe8d872`, `0xe8d87c` |

All 8 header values are confirmed.

`sizeof(GuiMessage)` = 0x110: the list add `0xe8c3d0` steps the vector `GuiData+0x150` by `0x110`, and `0xe8cea0` steps `GuiData+0x168` by `0x110`. [proven]

## Step 2: every path that makes a chat line

### Callers of the constructor [proven]

`inspect callers 0xe8d420` and `inspect xrefs --rva 0xe8d420` give the same 2 sites, and no data pointer:

1. `0xe8e9d5` in `GuiData::_addMessage(MessageConfig&&, int)` `0xe8e420`. Every chat display function goes through it (10 callers: `0xe927b0`, `0xe93840`, `0xe939a0`, `0xe94110`, `0xe94410`, `0xe94740`, `0xe95fb0`, `0xe963a0`, `0xe96600`, `0xe98360`).
2. `0xe8d1fd` in `0xe8cea0`. It makes type 9 messages with an empty username and puts them in a different list (`GuiData+0x168`). It is not chat.

`_addMessage` puts the new `GuiMessage` in one of two places. Both keep the strings the constructor built:

- `GuiData+0x150` (`std::vector<GuiMessage>`, the chat list) through `0xe8c3d0`. The message is moved there field by field. A move does not rebuild `mFullString`. [proven]
- A map at `GuiData+0xC38` keyed by `mAuthorXuid` (held messages, push at `0xeba900`; the tree insert compares the key with the author XUID string of the config). [proven] What releases them is not known. [inference] They are moves of the same object, so an edit made after the constructor goes with them.

No other writer of `mFullString` was found. Copies and moves (`0xebb650`, `0xeba900`) carry it over. A full search for all writes to +0x90 was not done.

### What the HUD shows [proven]

The HUD chat updater `0x4b5eb90` walks the list from `0x2681e20` (the `GuiData+0x150` vector) with a step of `0x110`. For each message with lifetime (+0xFC) > 0 and (the hide flag off, or forceVisible +0xF8 == 1), it reads `+0x90` or, through `0x15ffc0` (`optional::value_or`), `+0xB0`.

### What the chat window (T) shows [proven]

The chat screen controller adds one factory child per message with `0x4ff4a00` (it loads `chat_screen_messages` and `#text_tts`). Its callers:

- `0x4ff51b0` (fill on open): walks all messages, step `0x110`; text = `+0xB0` value or `+0x90`; TTS = `+0x50`.
- `0x4ff46d0` (new message event 6): text from `0x2682380` (last message: `0x2682405 add rax,-0x80` = end - 0x80 = last element + 0x90, `mFullString`) or from `0x26824d0` -> `0xe982f0` (last element + 0xB0, `mFilteredFullString`, when `controller+0xef8` is set).

The chat window does not build its text from `mUsername` and `mMessage`. It shows `mFullString` (or `mFilteredFullString`).

### Insertion point

The constructor `0xe8d420` covers both the HUD and the chat window. The detour edits `this+0x90` and, when `this+0xD0 == 1`, `this+0xB0`, after the original returns. [proven]

## Step 3: text forms

Lang file: `data/resource_packs/vanilla/texts/en_US.lang` (the `en_US.lang` inside `texts.brarchive` is empty, 0 bytes). [proven]

```
chat.type.text=<%s> %s
chat.type.emote=* %s %s
commands.message.display.incoming=%1$s whispers to you: %2$s
multiplayer.player.joined=%s joined the game
multiplayer.player.joined.realms=%s joined the Realm
multiplayer.player.left=%s left the game
multiplayer.player.left.realms=%s left the Realm
```

`TextPacket` type to `GuiMessage` `MessageType` (the `mov edx,imm` before each `MessageConfig` call): Raw -> 2 (`0xe92879`), Chat -> 1 (`0xe938fd`), Translate -> 3 (`0xe94230`), SystemMessage -> 4 (`0xe94537`), Whisper -> 5 (`0xe95259`), Announcement -> 8 (`0xe966bd`). The `_addMessage` log prefixes agree: 1 `[CHAT]: `, 3 `[LOCALIZED] `, 4 `[SYSTEM]`, 5 `[WHISPER]: `. [proven]

The packet column comes from the integrated server code in the client exe (single player, LAN host). A dedicated BDS server is another binary built from the same code. [inference] that BDS sends the same packets; confirm with one LAN or BDS session and a log of `mType`, `mUsername`, `mFullString`.

| Line | Packet the server sends | `MessageType` | `mUsername` | `mFullString` | Tag |
| --- | --- | --- | --- | --- | --- |
| Chat | `TextPacket` Chat, author `Name`, message `text` | 1 | `Name` | `<Name> text` | [proven] |
| Whisper | `TextPacket` Whisper, author `Name`, message `text` | 5 | `Name` | `<Name> §7§oName whispers to you: text` | [proven] from code, see note |
| `/me` | `TextPacket` **Chat**, author **empty**, message `* Name action` | 1 | empty | `* Name action` | [proven] |
| Join | `TextPacket` Translate, key `§e%multiplayer.player.joined`, param `Name` | 3 | empty | `§eName joined the game` | [proven] |
| Leave | `TextPacket` Translate, key `§e%multiplayer.player.left`, param `Name` | 3 | empty | `§eName left the game` | [proven] |
| Join / leave (Realms) | same, keys `...joined.realms` / `...left.realms` | 3 | empty | `§eName joined the Realm` / `§eName left the Realm` | [proven] key, [inference] when used |

Evidence:

- **Chat.** Handler case 1 calls `0xe93840(guiData, author, message, filtered, xuid, platformId)`, which builds a type 1 config with `username = author`. The constructor adds `<author> `. [proven]
- **Whisper.** `0xe94740`: `I18n::get(out=[rbp+0x230], key=[rbp+0x250]="commands.message.display.incoming" (bytes at 0xf5b517a), params=[author, message])` (`0xe95187 call r10`), then `0x11d1f0(out=[rbp+0x210], "§7§o", translated)` (`0xe95205`; `§7` at `0xf090408`, `§o` at `0xf090708`), then `MessageConfig(type 5, username=author (r8=rdi), message=[rbp+0x210], ...)` (`0xe95261`). The constructor then adds `<author> ` because the username is not empty. [proven] Note: this gives the name twice. Confirm the shown text in game.
- **`/me`.** `MeCommand` vtable `0xf44f610` (factory `0xbec6020`, setup `0xbebb890` loads `commands.me.description`); `execute` = slot 2 `0xbebbb00`. It formats `"* {} "` (format string at `0xf83e6d6`) with the origin name, appends the message, and calls `0xbebd480`, which calls the factory `0x3d73980` with author = empty string (`0xf07b2a0`). That factory makes the packet with type 1 (`0x3d73f90(out, 1, author, message, ...)`). [proven] The XUID passed with it comes from the command origin. [inference] Confirm by a log of `mAuthorXuid` for a `/me` line. The client function `0x1fb76e0` that uses `chat.type.emote` is a different path, not the `TextPacket` path.
- **Join.** `0x22b3040` builds `"§e"` (bytes at `0xf0904e8`) + `"%multiplayer.player.joined"` (`0xf5f5e2f`), or the `.realms` key (`0xf54ef49`) when a 6 character string matches, with one param = the player name `Player+0xbe8`. Then `0x3d74a40` makes a Translate packet (`0x3d74c80(out, 2, ...)`). The client handler case 2 runs `I18n::get` (`0x6b9e44a`) and calls `0xe94110`, which makes a type 3 config with an empty username. [proven]
- **Leave.** `0x2297490` builds `"§e"` (`0x229761d` / `0x2297627` read `0xf0904ea` / `0xf0904e8`) + `"%multiplayer.player.left"` (`0x229762a`) with param `Player+0xbe8`. [proven]

## Step 4: player list Add entry

Layout of one `PlayerListPacketPayload::mEntries` element (size 0xD8, variant index byte at +0xD0, from the existing entries). The client `PlayerListEntry` is built from it by `0x4045d50` (called at `0x6bde027`), which copies each field. [proven]

| Offset in Add entry | Field | Copied to `PlayerListEntry` | Tag |
| --- | --- | --- | --- |
| 0x08 | `mUUID` (16 bytes) | +0x08 | [proven] |
| 0x18 | `mId` (ActorUniqueID) | +0x00 | [proven] |
| 0x20 | `mName` (`std::string`) | +0x18 | [proven] |
| 0x40 | `mXUID` (`std::string`) | +0x38 | [inference] |
| 0x60 | `std::string` (`mPlatformOnlineId` by header order) | +0x58 | [inference] |
| 0x80 | `std::string` (new on 26.60, `mPlayFabId` per the existing entry note) | +0x78 | [inference] |
| 0xA0 | `mBuildPlatform` (int) | +0x98 | [inference] |
| 0xA8 | `mSkin` (shared_ptr) | +0xA0 | [proven] (existing entry `PlayerListAddEntry.mSkin`) |

`mName` = +0x20 [proven]: in `LegacyClientNetworkHandler::handle(PlayerListPacket)` `0x6bdd410`, for the local player's entry:

```
0x6bde014  lea rdx,[r14+0x20]
0x6bde018  mov rcx,rax
0x6bde01b  call 0x2bf940
```

`0x2bf940` copies that string into `Player+0xbe8`. `Player+0xbe8` is the player name: the join and leave code (`0x22b3040`, `0x2297677`) passes `Player+0xbe8` as the `%s` of `multiplayer.player.joined/left`.

`mXUID` = +0x40 [inference]: it is the next `std::string` after `mName`, which is the header order (`mId, mUUID, mName, mXUID, ...`). The handler collects `entry+0x40` of each Add entry into a list of strings (`0x6bde6a6 lea rdx,[r14+0x40]`, `0x6bde6b1 call 0x11ae20`) and passes the list to `0x624b50` after the loop. That fits a list of XUIDs, but it is not proof. To confirm: log `entry+0x40` in game; an Xbox XUID is a decimal number of about 16 digits.

## Step 5: font

- `BitmapFont` vtable `0xf0ea8a0`: slot 29 = `0x15902c0`, slot 30 = `0x15905b0` (`inspect vtable --rva f0ea8a0 --from 28 --to 31`). Three code sites load the vtable: `0x158aae4` and `0x158b529` (two constructors, called from the font repository builders `0xec8800` and `0xec61d0`) and `0x15aecec`. The exe has no RTTI name for `BitmapFont` (`--class .?AVBitmapFont@@` is not found), so the class name comes from the spike. [proven: address and slot]
- `_getCharWidth` `0x15902c0`: `0x15902ce mov ebx,r8d` (force unicode in `r8b`), `0x15902d1 mov rsi,rcx` (font), `0x1590306 test bl,bl`, `edx` = character (passed on to `0xe6c330`). It calls its own vtable slots 33 (`[rax+0x108]`) and 34 (`[rax+0x110]`). The result in `xmm0` is proven by the spike log, which read the float results. [proven]
- Chat font = `FontHandle` default font (+0x30) from `ClientInstance.getFontHandle` (vslot 0x57): **not confirmed.** [inference] The chat labels in `hud_screen.json` (`chat_label`) and `chat_screen.json` (`messages_text`) choose their font by name, with `"font_type": "$chat_font_type"` bound to `#chat_font_type`. The spike swapped slot 29 in the shared vtable, which changes every `BitmapFont`, so it does not prove which instance chat uses. To confirm: with Mojangles selected, compare the font pointer that `_getCharWidth` receives while a chat line is measured with `FontHandle+0x30`.
- Smooth font. The chat typeface has two options (`chat_settings_menu_screen.json`): `typeface.mojangles` = Mojangles, and `typeface.notoSans` = Noto Sans, with `"$radio_label_font_type":"smooth"`. `font/font_metadata.json` (in `font.brarchive`) defines `OldSmoothFont` with `"font_format":"msdf"` from `font/smooth/smooth_` (the `smooth_XX.fontdata` pages have no `E0` or `E1` page), and font aliases that add `UnicodeFont` for some ranges. [proven] The private use range in the aliases:
  - `SmoothFontLatin`: U+E000-E0FF and U+E200-F8FF go to `UnicodeFont`. U+E100-E1FF is not covered.
  - `SmoothWithEmoticons`: U+E100-E1FF goes to `UnicodeFont` with `font_scale` 0.65.
  - `SmoothSmall`: U+E000-F8FF goes to `UnicodeFont`.

  The smooth font is an msdf font inside a font alias, so the chat font object is not a `BitmapFont`. [inference] Which alias the `smooth` font type uses was not found (the names `UnicodeFont`, `SmoothFontLatin`, `SmoothSmall` are registered in `0xe82f40`). To confirm: with Noto Sans selected, read the vtable of the font that draws a chat line.

## Step 6: an E1 icon before a world

- The vanilla UI JSON (183 files in `ui.brarchive`) has no U+E100-E10F character and no `\uE10x` escape. `en_US.lang` has no literal U+E100-E10F character either. No other text file under `data/` has one. [proven]
- The E1 icons come from emoticon tokens. `font/emoticons.json` maps names to code points: `shank` E100, `armor` E101, `minecoin` E102, `token` E102, `code_builder_button` E103, `immersive_reader_button` E104, `hollow_star` E106, `solid_star` E107, `wood_pickaxe` E108, `wood_sword` E109, `crafting_table` E10A, `furnace` E10B, `heart` E10C, `camera` E10D, `horse_ride` E10E, `boat_ride` E10F. `en_US.lang` uses these tokens in 26 lines. [proven]
- Screen outside a world: **How to Play**, opened from the main menu (Settings, then How to Play), topic **Armor**. `howtoplay.armor.text.1` has `:armor:` (U+E101). Other topics: Food (`:shank:` E100), Crafting a Tool (`:wood_pickaxe:` E108, `:crafting_table:` E10A), Death and Respawn (`:heart:` E10C). [inference] that the token is turned into the glyph when the label is drawn, and that this loads `glyph_E1`; confirm when Task 9 opens the screen and sees the icon.

## Concerns for the plan

1. **`/me` is a Chat line (type 1) with an empty username.** Its full string is `* Name action`. Spec Decision 6 puts the glyph for Chat at the start of the full string by XUID; for `/me` that puts it before `*`, not before the name, and the username fallback has no name. Core needs a rule for "type 1 with empty username": find the name after `* `.
2. **Whisper full string** is `<Name> §7§oName whispers to you: text` by the code (the name twice). The glyph at the start (before `<`) still sits before a name. Check the shown text in game.
3. **Font check (Decision 7).** The chat label font is chosen by `$chat_font_type`, not provably the `getFontHandle` default font. If Noto Sans is selected and `FontHandle+0x30` stays a `BitmapFont`, the check passes while chat draws with the smooth alias. Then a U+E1xx glyph is either not covered (`SmoothFontLatin`) or drawn by `UnicodeFont` at 0.65 scale (`SmoothWithEmoticons`). A safer check reads the chat typeface setting.
4. **The filtered message argument is destroyed by the callee.** The detour must not touch argument 5 after the original call.
5. **`mXUID` at +0x40 is an inference** (see Step 4).
6. Held messages in `GuiData+0xC38` are released at a time that is not known. They keep the edited strings, but a held line can show later than the frame the face was written.

Task Completed
