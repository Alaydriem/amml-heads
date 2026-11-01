### Task 6: Benches

**Files:**
- Create: `heads/bench/Cargo.toml`, `heads/bench/src/lib.rs`, `heads/bench/benches/heads.rs`

- [ ] **Step 1: Write the Criterion benches:**
  - `extract_64`
  - `extract_256`
  - `decode_tag`
  - `texture_for_hit_128` (a full store)
- [ ] **Step 2: Run** `mise run bench`. Record the medians in the ledger. Expected: `decode_tag` and `texture_for_hit_128` take less than 100 ns each. If not, report to the operator before Task 8.
- [ ] **Step 3: Checkpoint.**

