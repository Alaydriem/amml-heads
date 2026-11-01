Task 2: recipes for the chat heads entries
Build: 1.26.60.05, BuildId 6ad79ee4-17d51891a1119914
Exe: %LOCALAPPDATA%\amml\instances\1.26.60.05-locator-heads\Minecraft.Windows.exe

STATUS: DONE_WITH_CONCERNS

1. Entries and resolved values (26.60)

| Entry | Kind | File | Value | Recipes | Checks |
| GuiMessage.ctor | function, hooked | recipes/gui_message.hcl (new) | 0xe8d420 | 2 (body + function_start_of; call in GuiData::_addMessage read as branch target) | function_start |
| BitmapFont.vtable | global | recipes/font.hcl (added) | 0xf0ea8a0 | 2 (lea in constructor 0x158aa80; lea in constructor 0x158b4f0) | section .rdata |
| BitmapFont._getCharWidth | function, hooked | recipes/font.hcl (added) | 0x15902c0 | 2 (body anchor + function_start_of; head anchor + function_start_of, same function) | function_start, in_vtable BitmapFont.vtable 25..33 |
| PlayerListAddEntry.mName | field | recipes/player_list_packet.hcl (added) | 0x20 | 2 (setter call in the PlayerListPacket handler; first string copy in 0x4045d50) | range 16..64 |
| PlayerListAddEntry.mXUID | field | recipes/player_list_packet.hcl (added) | 0x40 | 2 (XUID list collection in the handler; second string copy in 0x4045d50) | range 32..128; notes say it is an inference |
| ClientInstance.getFontHandle | vslot | existing, not changed | 0x57 (87) | existing | existing |
| FontHandle.dtor | function | existing (font_handle.hcl), not changed | 0xe7f9e0 | existing | existing |

Each new pattern was checked with `inspect scan` before use: 1 match each.
No existing entry was changed.

2. derive (twice, --compare data/6ad79ee4-17d51891a1119914.hcl)
- Run 1: derived 304/304 (100.00%) in 14.0s, deterministic. Failures none, degraded none, unverified none.
- Run 2: derived 304/304 (100.00%) in 13.9s, deterministic.
- The two data files are byte-identical (cmp). The reports differ only in the elapsed line.
- Every value in data/6ad79ee4-17d51891a1119914.hcl (296 entries) is unchanged in the new output.
- The new output has 8 values that the stored data file does not have: the 5 new entries above, plus
  ClientInstance.getMinecraftInput (0x108), ClientInstance.requestLeaveGame (0xe) and
  PacketTraceNetworkPeer.receivePacket (0x226b6a0). Those 3 recipes were already in ../amml/recipes before this task; they are not mine.

3. lint
- `lint: 304 entries, 0 open findings, 85 kept bytes, 0 not for this build`, exit 0. No finding or kept byte on the new entries.

4. signature (extra check, not required)
- GuiMessage.ctor: rcx:64, rdx:32, r8:64, r9:64, stack 0x28..0x50 (6 args). Matches the findings.
- BitmapFont._getCharWidth: args_int rcx:64, r8:32, returns "int". The tool misses edx (the character goes straight to the callee 0xe6c330 without a read) and cannot see the float return in xmm0. The findings prove edx = character and xmm0 = width. The hook must take rcx, edx, r8b and return f32.

5. HeadsEntries::CHAT (heads/amml/src/entries/heads_entries.rs), [EntryId; 7]:
  GUI_MESSAGE_CTOR                 "GuiMessage.ctor"
  BITMAP_FONT_VTABLE               "BitmapFont.vtable"
  BITMAP_FONT_GET_CHAR_WIDTH       "BitmapFont._getCharWidth"
  PLAYER_LIST_ADD_ENTRY_NAME       "PlayerListAddEntry.mName"
  PLAYER_LIST_ADD_ENTRY_XUID       "PlayerListAddEntry.mXUID"
  CLIENT_INSTANCE_GET_FONT_HANDLE  "ClientInstance.getFontHandle"
  FONT_HANDLE_DTOR                 "FontHandle.dtor"
MANIFEST is now [EntryId; 33] (26 + the 7 above, appended). heads/amml/amml.hcl lists the same 7 at the end (33 lines).
ALL (17) is not changed.

Tests: heads/amml/tests/entries/heads_entries.rs
- entry_ids_are_distinct now also asserts CHAT has no duplicates and shares no id with ALL.
- new: manifest_has_every_chat_entry_once (MANIFEST has no duplicates and holds every CHAT id).
- heads_mod.rs manifest_lists_every_entry_the_mod_resolves was not changed and passes.

6. Check and test results
locator-heads:
- mise run check: exit 0 (fmt, check, clippy -D warnings clean).
- mise run test: `Summary [   0.746s] 166 tests run: 166 passed, 0 skipped`
../amml:
- mise run test: `Summary [  31.325s] 1606 tests run: 1606 passed, 9 skipped`; doctests all ok. exit 0.
- mise run check: exit 101. cargo fmt passes; clippy fails on a pre-existing error not related to this task:
  `ui\kit\tests\widget\segmented.rs:191:34: use of deprecated associated function egui::Id::new` (2 errors, amml-ui integration test). No recipe file is compiled by clippy. I did not fix it (out of the files I may edit).

7. Private instance
Copied gui_message.hcl, font.hcl, player_list_packet.hcl to
%LOCALAPPDATA%\amml\instances\1.26.60.05-locator-heads\amml\recipes\ (cmp: identical).
font_handle.hcl in the instance is the same as the repo. client_instance.hcl in the instance has getFontHandle but is older than the repo (it lacks requestLeaveGame and getMinecraftInput); I did not copy it because this task did not change it.

8. Concerns
- The recipes were written and verified on 26.60 only. The 26.51 exe was not checked; the new entries are likely to fail there (no wrong value, a failed entry). Each note says so.
- PlayerListAddEntry.mXUID = 0x40 is an inference (header order + XUID list use). Confirm in game.
- BitmapFont._getCharWidth: both recipes are in the same function; the in_vtable window (25..33) is the independent fact. The signature tool reports returns "int" and no edx; the hook must follow the findings (rcx, edx, r8b -> xmm0 f32).
- ../amml `mise run check` fails on a pre-existing clippy deprecation error in ui/kit/tests/widget/segmented.rs.
- The stored data file data/6ad79ee4-17d51891a1119914.hcl does not hold the 5 new entries (nor 3 older ones). It was not regenerated; the task did not ask for it.
- The brief puts BitmapFont entries in font.hcl (per the plan) rather than a separate bitmap_font.hcl, so font.hcl now holds two classes.

Task Completed
