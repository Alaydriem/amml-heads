# Chat heads spike: findings

Date: 2026-10-31. Build: 26.60.05 (`6ad79ee4-17d51891a1119914`). Instance: private `1.26.60.05-locator-heads`.

Question: can the mod show a player head in a chat line, using the faces it already has?

Answer: yes, through a font glyph. The game draws a glyph from a page that the mod uploads at runtime. The page can be changed while the game runs, and every line that uses it changes. One width fix is necessary.

Tags: [proven] = seen in game or read from disassembly. [header] = LeviLamina 26.60 headers. [inference] = not checked.

## What was proven in game

1. A runtime upload of `font/glyph_E1` with `TextureGroup::uploadTexture` (the same call the locator bar uses) replaces the page that the chat font draws. The glyphs draw in their own colours: the text colour does not tint them. [proven]
2. A second upload, after the font has drawn the page, shows at once in all lines, old and new. Red, blue and green uploads 30 s apart were all seen, in all 7 lines at the same time. [proven]
3. The face (8x8, with the hat) fits a 16x16 cell with no loss. [proven]
4. Without a fix, the glyph advance is 4 (the width of an empty vanilla cell), but the full 16x16 cell is drawn. The next glyph then covers the right three quarters of the face. With `_getCharWidth` returning 16 for the face glyphs, spacing is correct. [proven]

## Font facts

- The game names a fixed list of 224 glyph pages in code. Only `E0` and `E1` are in the private use area. `E2` to `F8` are not loaded. [proven, exe strings]
- Vanilla `glyph_E1` is 256x256 (16x16 cells). Only row 0 is used (about 16 icons). Rows 1 to 15 are empty: 240 cells, which is more than the face store capacity (128). So U+E110 to U+E18F can map one to one to the face store slots. [proven, from `font.brarchive`]
- `BitmapFont` keeps its own CPU copy of each page (`mBitmapFontImages`) and measures the widths from that copy once per page (`mSheetScannedForWidthsAndOffsets`, `mUnicodeWidths`). A texture upload does not change the measured widths. [header, proven by item 4]
- `BitmapFont::buildChar` (vtable slot 23) always draws the full cell for a unicode glyph. It uses the `_getCharWidth` result only as the advance. [proven, disassembly at RVA `0x1590f60`]
- Width units are pixels of the glyph page: `A` = 6, U+E100 = 10, U+E0A0 (512 px page) = 28, empty E1 cell = 4. [proven, log]
- Width 17 (16 + 1 spacing) made the face glyphs not draw in that run. Width 16 draws. See open risk 1: the 17 run also had the startup width probe, so 17 may not be the cause. [proven / unclear]

## Addresses (26.60.05, RVA)

| What | Value |
| --- | --- |
| `BitmapFont` vtable | `0xf0ea8a0` |
| slot 23 `buildChar` | `0x1590f60` |
| slot 29 `_getCharWidth(int, bool) -> float` | `0x15902c0` |
| slot 30 `_scanUnicodeCharacterSize` | `0x15905b0` |
| `ClientInstance.getFontHandle` | vslot 87 (existing AMML entry) |
| `FontHandle` default font `shared_ptr<Font>` | +0x30 |
| `mUnicodeWidths` map in `BitmapFont` | +0x388 (buckets +0x398, mask +0x3B0) |

## What the spike code did (temporary)

- `TextureBridge::spike_upload_page`: builds the `ResourceLocation` `font/glyph_E1` (file system 0) and uploads a 256x256 RGBA page.
- `RenderState`: when the first face is uploaded, copies vanilla `glyph_E1`, puts the face in U+E110 and a colour in U+E111, and uploads. It re-uploads every 1800 frames.
- `client_update` detour: calls the render state once per frame, so faces upload without the locator bar.
- `SpikeFont`: gets the default font through `getFontHandle`, and swaps vtable slot 29 for a function that returns 16 for U+E110 to U+E1FF.

## Open risks

1. **Early page load.** In runs 3 to 5 the spike called `_getCharWidth` for U+E100 and U+E110 at startup, before the first upload. In those runs the face glyphs did not draw: the vanilla empty cells were drawn. When the first use of E1 was after the upload, it worked, and later re-uploads also worked. A vanilla UI that shows an E1 icon before the first upload may cause the same failure. Test: open a screen that uses an E1 icon before joining a world. A candidate fix is to upload the page once at mod start, before any UI. [inference]
2. **The vtable swap** changes every `BitmapFont`, and writes to read-only memory. A product version needs an AMML entry for `BitmapFont._getCharWidth` and a normal detour. [inference]
3. **The smooth font.** With the smooth font option on, chat may use another font class (`MSDFFont` or `TrueTypeFont`) with its own width code. Not tested. [inference]
4. **Sender to face.** `TextPacket` carries the sender name and XUID for player chat. Server formatted chat (raw text, many servers and Geyser) has no sender field, so a match by name is best effort. Not tested.
5. **Where to insert the glyph.** In the packet is simple, but the glyph then also goes to chat logs and copied text. Before display needs another hook. Not tested.
6. **Vanilla row 0 of E1.** The mod must keep the vanilla icons in row 0 when it uploads its page. Resource packs that replace `glyph_E1` would lose their own cells in rows 1 to 15. [inference]

Task Completed
