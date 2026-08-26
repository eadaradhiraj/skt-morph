# SktMorph WASM — Sanskrit Conjugations, Participles, Declensions

Rust + WebAssembly. Live Pāṇinian engine.

## Focus
Only four features — no prakriya/sutra/kosha/commentary:

- **Conjugations** (`tinanta`): 10 gaṇas, 6 lakāras (lat/laṅ/loṭ/vidhi-liṅ/lṛṭ/liṭ), parasmai/ātmanepadī, shuddha/kartari — `src/engine/` (phonology → stems → endings → join) — with **prefix** support (`pra/sama/anv` sandhi via `prefix.rs`)
- **Participles** (`krdanta`): 20 pratyayas (kta, ktavatu, Satṛ, Sānach, tumun, ktvā, lyap...) — with prefixes
- **Declensions**: `subanta` (27 paradigms) + `sarvanama` (70 pronouns)
- **Search**: `analyze(word)` reverse lookup + `search(prefix)` autocomplete via embedded compact dhatu index (2259 dhatus)

## Build
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo build
wasm-pack build --target web  # → pkg/
cargo run --bin validate --features native-db -- --full  # vs gold (skt-morph-data)
```
Demo: `cd www && npm i && npm run dev`

## Data
Gold reference: `../skt-morph-data` (scraped from ashtadhyayi.com, 10 ganas, 10M forms). Rust embeds compact `src/data/dhatus_compact.rs` (152KB) generated from SQLite at build time — no runtime DB, no hardcode overlay.
