# SktMorph WASM — Sanskrit Conjugations, Participles, Declensions

Rust + WebAssembly. Live Pāṇinian engine, zero SQLite at runtime.

## Focus
- **Conjugations** (`tinanta`): 10 gaṇas, 6 lakāras (lat/laṅ/loṭ/vidhi-liṅ/lṛṭ/liṭ), parasmai/ātmanepadī, shuddha/kartari — via `src/engine/` (phonology → stems → endings → join)
- **Participles** (`krdanta`): 20 pratyayas (kta, ktavatu, Satṛ, Sānach, tumun, ktvā, lyap...)
- **Declensions**: `subanta` (27 paradigms) + `sarvanama` (70 pronouns)
- **Search**: `analyze(word)` reverse lookup + `search(prefix)` autocomplete via embedded compact dhatu index (validated vs `skt-morph-data` 2259 dhatus, 1.2G DB gold)

## Build
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo build
wasm-pack build --target web   # → pkg/
cargo run --bin validate       # vs skt-morph-data gold
cargo run --bin test_gen
```
Demo: `cd www && npm i && npm run dev`

## Data
Gold reference: `../skt-morph-data` (scraped from ashtadhyayi.com, 10 ganas, 10M forms). Rust embeds compact `src/data/dhatus_compact.rs` (152KB) generated from SQLite at build time — no runtime DB.

## Dead code removed
`taddhita.py`, `shabda_db`, `prakriya`, `ranking`, `translit` (now JS), `nich/san/yaṅ` derivatives, 320MB tinantas/krdantas SQLite, `scripts/` — kept live engine only.
