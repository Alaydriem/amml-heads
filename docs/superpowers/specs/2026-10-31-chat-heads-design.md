# Chat Heads: design (Bedrock 26.60)

Date: 2026-10-31. Status: shipped 2026-10-31 (plan `docs/superpowers/plans/2026-10-31-chat-heads.md`; ledger and rulings `docs/findings/plan-ledger-chat/progress.md`; game-code facts `docs/findings/chat-heads-re/findings.md`).

## What shipped, and changes from the decisions below

- Heads show in the HUD chat and the chat window for chat lines, whispers, `/me`, and join and leave lines (checked in game by the operator, single player).
- **Face size (changes Decisions 1 and 4).** A face is drawn at 10x10 (nearest sampled from the 8x8 skin face) at cell x 0 to 9, y 2 to 11, the place where the vanilla hunger and armor glyphs sit. The width is 11.0, not 16.0. The full 16x16 cell was twice the text height and overlapped the lines above and below. The operator chose 10x10 in game.
- **`/me` (changes Decision 6).** In 26.60, `/me` arrives as a Chat line (type 1) with an empty username and the full string `* Name action`. It uses the localized rule.
- **Anchored names (changes Decision 6).** For localized lines and `/me`, the name must start the line (after `§` codes and an optional `* `). A player name found elsewhere in the line gives no head.
- **Face code points in text.** U+E110 to U+E1FF typed into a chat line are removed, so a player cannot show another player's face in chat.
- **Stale names.** Every player list Add entry is recorded (a bounded list), so a new player with the name or XUID of an old player gives no head instead of the old face.
- **No head before the page is up.** No glyph is inserted until the first page upload in the process has succeeded. The page is uploaded again after a world exit.

## Known problems

- The early-load check (Decision 8) was not run: How to Play no longer exists in 26.60, and no screen before a world that shows an E1 glyph is known. Faces drew in every session that started from a fresh launch.
- Not checked in game: a second player (join, leave and chat lines from another player, a skin change), the Noto Sans chat font (Decision 7 checks only the `getFontHandle` default font), and the `heads.chat` setting off.
- `PlayerListAddEntry.mXUID` (+0x40) is an inference. A wrong offset gives no head on chat lines; it cannot give a wrong face, because XUID matches must be unique.
- A face code point typed into text outside chat (a sign, a nametag, an item name) still draws a face.
Evidence: `docs/findings/chat-heads-spike/findings.md` (spike proven in game), `docs/superpowers/specs/2026-10-30-locator-heads-design.md` (the face pipeline this feature reuses).

## Intent

In the chat, show the player's skin face just before the player's name, as in `[face] <Alaydriem> hi`. It uses the faces that locator-heads already extracts for the locator bar. It is part of the `alaydriem-locator-heads` mod, not a new mod. The operator plays on vanilla servers.

## Goals

- Bedrock 26.60.05 Windows client (BuildId `6ad79ee4-17d51891a1119914`), standalone AMML host. This is the only build that is tested.
- These lines get a head, before the player name:
  - chat messages (`<Name> message`);
  - whispers;
  - `/me` actions (`* Name action`);
  - join and leave messages (`Name joined the game`, `Name left the game`).
- The head shows in the chat lines over the game and in the full chat window.
- The head is drawn as a font glyph by the game's own text renderer, so line wrap, scrolling, fade, chat history and the chat screen stay vanilla.
- A face change (a skin packet) updates the head in every line that shows it, old lines too.
- A new setting turns chat heads on and off, separately from the locator bar heads.

## Non-goals

- Server formatted chat (rank prefixes, custom colours, plugin or Geyser raw text lines). A line that is not one of the four vanilla kinds above gets no head.
- Names inside the message body (a player who mentions another player).
- The smooth font option, if it uses a font class other than `BitmapFont`. Lines then get no head (see Decision 7).
- Resource packs that replace `glyph_E1`. The mod's page replaces theirs in rows 1 to 15.
- Any change to what the client sends. The glyph goes only into lines that this client shows.
- 26.51 and other builds.

## Anti-goals (the feature fails even if a head appears)

- **A wrong face.** A line shows the head of a player who did not write it or is not named in it. This includes an old line that shows a new player's face because a glyph cell was given to someone else. When the mod is not sure, the line gets no head.
- **A performance cost the operator can feel.** No allocation per frame, no work per frame when nothing changed, no blocking on the game thread, no stutter when many players join at once.
- **A visible fault when a face is missing.** No box, no gap, no stray character. A line without a face looks exactly as it does in vanilla.
- The game crashes or freezes on a malformed message, a player who leaves, or a world change.

## Decisions

1. **Glyph page.**
   - The mod owns the page `font/glyph_E1` (256x256 RGBA, 16x16 cells).
   - It keeps row 0 as vanilla has it, and uses rows 1 to 15 (U+E110 to U+E1FF, 240 cells) for faces.
   - The page is uploaded with `TextureGroup::uploadTexture` under the `ResourceLocation` `font/glyph_E1` (file system 0), as in the spike. Proven: a re-upload shows at once in every line.
   - Each face is the extracted 32x32 face, scaled down 2x to 16x16 (the skin face is 8x8, so nothing is lost).
   - The vanilla row 0 pixels come from the vanilla `glyph_E1` (the builder decides how: embedded in the DLL, or read from the game at runtime).
2. **One upload per frame at most.** All face changes since the last upload are written into one CPU copy of the page, and the page is uploaded once, in the next frame. A frame with no change does nothing. The upload runs from the per-frame `ClientInstance::update` detour, because the locator bar render detour only runs when another player is on the bar.
3. **Glyph cells never move between players while the game runs.** (Builder check question 4: chat history belongs to `GuiData` and can outlive a world. My call.)
   - A cell is given to a player (by UUID) the first time the player gets a face. It stays that player's cell until the game closes. A world exit does not clear it, so an old line in the chat history never shows another player's face.
   - A skin change rewrites the player's own cell. The same player in a later world uses the same cell.
   - A player who leaves keeps the cell, so old lines still show the right face.
   - When the 240 cells are used up, a new player gets no cell and their lines get no head. The bound is 240 for the life of the process, with no eviction.
   - The cell map is separate from the face store (capacity 128, LRU), because the face store evicts and the cells must not.
   - The cell is given, and the face is written into the CPU copy of the page, when the face is extracted on the packet side. (Builder check question 2: a join line arrives in the same batch as the player list Add. My call.) The page upload follows in the next frame (Decision 2), so a line can show an empty cell for at most one frame. This is accepted.
4. **Width fix.** A detour on `BitmapFont::_getCharWidth(int, bool) -> float` (vtable slot 29, RVA `0x15902c0`) returns 16.0 for a character in U+E110 to U+E1FF that has a cell with a face. It does this whatever the setting is, so a line that already has a glyph never shows an overlap. Every other character goes to the original. It is a normal AMML function detour with a recipe entry, not a vtable write. The range check comes first, so other characters pay one compare.
5. **Insertion point.**
   - A detour on the `GuiMessage` constructor (`GuiMessage(MessageType, username, message, filteredMessage, ttsMessage, authorXuid, lifetime, forceVisible, ttsRequired)`). After the original runs, the detour edits `mFullString` and `mFilteredFullString` (when it has a value) of the new message.
   - The decision of what to insert, and where, is a core service. The detour only reads the strings, asks core, and writes the result back.
   - The detour only inserts a glyph whose cell already holds a face. A player with no face yet gets no head (the line is not edited later).
   - The head must show in both places chat appears: the chat lines over the game (HUD) and the full chat window. (Operator decision.) If the in-game check shows that one of them does not use `mFullString` from this constructor, the builder may move the insertion to another point (for example `GuiData::_addMessage` or the `display*Message` functions). The rules of Decisions 3, 6 and 7 do not change.
6. **Who a line belongs to.**
   - The lookup (XUID to UUID, name to UUID) is filled from `PlayerListPacket` add entries. A Remove does not delete the entry, so a leave line, or a chat line just after the Remove, still finds the player. (Builder check question 1. My call.) An entry lives as long as the player's cell: the bound is the cell bound, 240, for the life of the process. A player with no cell is not kept.
   - `ChatMessage` and `WhisperMessage`: the author XUID (`mAuthorXuid`) gives the player. The glyph goes at the start of the full string, before the `<` of a chat line and before the name of a whisper. When the XUID is empty (an offline-mode or LAN server), the line falls back to the username, with the same "exactly one match" rule as below. (Builder check question 7. My call.)
   - `LocalizedMessage` (join, leave, `/me`): the name is found in the full string. The glyph goes just before the name, after any formatting codes (`§e`) in front of it. It is inserted only when exactly one player in the lookup (online or departed) has a name that matches, and the name appears once in the string at a word boundary. Otherwise there is no head.
   - Every other message type: no head.
7. **Font class check.** The glyph is inserted only while the chat font is a `BitmapFont`. The check compares the font vtable with the `BitmapFont` vtable. When the smooth font option gives another class, no glyph is inserted.
8. **Early page load.** In the spike, when the font loaded `glyph_E1` before the first upload, the faces did not draw. To avoid this, the mod uploads its page (vanilla row 0, empty rows 1 to 15) on the first `ClientInstance::update` frame, before any world loads. An in-game check of this is part of the work (open a screen that shows an E1 icon before joining a world, then join and check the heads). If the check fails, the feature still finishes and ships: in that case chat lines in that session show no heads (they look vanilla, and the locator bar is not affected). The failure is written down as a known problem for a later round. (Operator decision.)
9. **Setting.** A new boolean descriptor field, `heads.chat`, default `true`, with the same name and default in `HeadsSettings`. When it is off, no glyph is inserted into new lines. Lines that already have a glyph keep it, and the width fix stays on for them (Decision 4), so they still draw correctly. (Builder check question 3. My call.)
10. **Logging.** One info line when the page is first uploaded, one warn line when the cells run out (once per session), and the failure reason when a hook or entry is missing. Nothing per message.

## Constraints

- All rules in `CLAUDE.md` apply: logic in `locator-heads-core` (forbid unsafe), thin detours, one struct per file, tests that mirror `src/`, bounded collections with a test that proves each bound, a Criterion bench for each hot path.
- Hot paths for this feature: the width detour (called for each character of each text the game measures), and the message edit (once per chat line). Both get a bench. The width detour must not allocate.
- New bounds: glyph cells 240 for the life of the process, with no eviction (a player past the bound gets no cell); the XUID and name lookup holds only players with a cell, so its bound is also 240 (state both in the types).
- A failed entry or hook for this feature turns off chat heads only. The locator bar heads keep working.
- Memory writes into game strings use the game heap (`MsvcHeap`), as the existing string code does.

## Left to the builder

- How the vanilla row 0 pixels are obtained (Decision 1).
- The AMML recipes for the new entries: the `GuiMessage` constructor, `BitmapFont::_getCharWidth`, the `BitmapFont` vtable, and the chat font (through `ClientInstance.getFontHandle`, as in the spike).
- How to get the XUID and name of each player list entry (the `PlayerListEntry` layout).
- The exact text forms of join, leave and `/me` lines in 26.60, read from the game and tested.
- How the edit of `mFullString` is written back (a new `std::string` on the game heap).
