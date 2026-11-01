# Chat heads design: open questions before planning

Date: 2026-10-31. Builder check of `docs/superpowers/specs/2026-10-31-chat-heads-design.md` (saved by the lead; the subagent could not write files).

Ranked questions:

1. Do departed players stay in the XUID and name lookup after a `PlayerListPacket` Remove? If not, leave lines and late chat lines mostly get no head.
2. Does "a cell already holds a face" mean in the CPU page at packet time, or after the GPU upload? If after, join lines almost never get a head, because the join line arrives in the same batch as the Add. Today the face channel is drained only in the locator bar render detour.
3. With `heads.chat` off, Decision 9 passes widths through but keeps old glyphs. Spike item 4 shows that this overlaps the face: a visible fault.
4. If chat history survives `onLevelExit` (`GuiData::mGuiMessages` belongs to `GuiData`), clearing the cells gives old lines a wrong face in the next world.
5. If the early page load (Decision 8) does not fix the startup failure, ship with the limitation, or add investigation work?
6. Must heads also show in the open chat screen? If the 9-argument `GuiMessage` constructor is not the only path, may the builder move the insertion point?
7. When the author XUID is empty (offline-mode BDS, LAN), fall back to a name match, or give no head?

Task Completed
