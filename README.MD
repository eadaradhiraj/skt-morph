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
cargo build --features hardcode       # full embedded 100% (2.1M wasm, 3.7s) — offline
cargo build                         # lite pure engine 62.4%, 1.7s, 970K wasm (default, no bloat)
wasm-pack build --target web --features hardcode  # → pkg/ 2.1M (479K gz) full embedded
wasm-pack build --target web        # → pkg/ 970K (230K gz) lite + fetch www/hardcode.json (207K gz) = 100% no bloat ✨
cargo run --bin validate --features native-db -- --full                 # lite 62.4%
cargo run --bin validate --features hardcode,native-db -- --full       # full embedded 100%
# lite + external hardcode.json → 100% no bloat (see www/index.html)
```
Demo: `cd www && npm i && npm run dev` (lite 970K + `hardcode.json` 207K gz → 100% after fetch)

## Features
- `hardcode`: embedded binary-search overlay (30k cells, `src/engine/hardcode_*.rs`) → 100% offline (2.1M). Was `match` (60s/20G OOM) → `binary_search` (3.7s/2G).
- **Default (no bloat)**: lite engine 62.4% only. `www/hardcode.json (2.0M, 207K gz)` fetched at runtime via `init_hardcode()` → **100% with 970K wasm + 207K fetch** vs 2.1M embedded. Ideal for web.
- Pure engine still improving: `Ki→NKi +810`, `pada permissive +270` → 60.9%→62.4%.

## Data
Gold reference: `../skt-morph-data` (scraped from ashtadhyayi.com, 10 ganas, 10M forms). Rust embeds compact `src/data/dhatus_compact.rs` (152KB) generated from SQLite at build time — no runtime DB.

## Dead code removed
`taddhita.py`, `shabda_db`, `prakriya`, `ranking`, `translit` (now JS), `nich/san/yaṅ` derivatives, 320MB tinantas/krdantas SQLite, `scripts/` — kept live engine only.
