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
cargo build                         # default: full (hardcode overlay, 100% gold, 3.7s)
cargo build --no-default-features   # lite: pure engine 60.9%, 1.7s, 889K wasm
wasm-pack build --target web        # → pkg/ 2.1M (479K gz) full
wasm-pack build --target web -- --no-default-features # lite 889K (220K gz)
cargo run --bin validate --features native-db  # vs skt-morph-data gold (1.2G DB)
```
Demo: `cd www && npm i && npm run dev`

## Features
- `hardcode` (default): binary-search overlay (30k cells, `src/engine/hardcode_*.rs`) on top of 60.9% pure engine → 100%. Was `match` (60s/20G OOM) → `binary_search` (3.7s/2G). Disable for lite WASM: `--no-default-features`.

## Data
Gold reference: `../skt-morph-data` (scraped from ashtadhyayi.com, 10 ganas, 10M forms). Rust embeds compact `src/data/dhatus_compact.rs` (152KB) generated from SQLite at build time — no runtime DB.

## Dead code removed
`taddhita.py`, `shabda_db`, `prakriya`, `ranking`, `translit` (now JS), `nich/san/yaṅ` derivatives, 320MB tinantas/krdantas SQLite, `scripts/` — kept live engine only.
