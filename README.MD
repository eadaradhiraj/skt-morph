# SktMorph WASM — तिङन्त, कृदन्त, सुबन्त

Rust + WebAssembly morphology. Generates forms and analyses them. No prakriyā trace, no accents, no kośa commentary.

## Contract

| Layer | Script / names |
| --- | --- |
| JS demo (`www/`) | Input: **Devanagari** or **Harvard-Kyoto**. Display: Devanagari. Grammatical labels: Sanskrit in Devanagari only (प्रथमा, एकवचन, लट्, …). |
| WASM / Rust | **SLP1 only** for all stems, forms, dhātus, upasargas. Codes internally (`plat`, `prathamA`, `pum`, puruṣa/vacana as 1–3). |

`www/translit.js` converts in and out. `www/labels.js` maps codes → देवनागरी names. Multiple analyses are all returned.

## Generate

- **तिङन्त** (`tinanta`): 10 gaṇas, 6 lakāras (लट् / लङ् / लोट् / विधिलिङ् / लृट् / लिट्), parasmai/ātmanepada, śuddha kartari. Optional **upasargas** (user list → sandhi in `prefix.rs`).
- **कृदन्त** (`krdanta`): kṛt pratyayas (क्त, क्तवतु, शतृ, शानच्, तुमुन्, क्त्वा, ल्यप्, …). Same prefix support.
- **सुबन्त** (`subanta`): ending-class paradigms (including foreign stems such as ट्रम्प → ट्रम्पेण). **सर्वनाम** tables for pronouns.

## Analyze

Given one surface form (after JS → SLP1), return **every** parse that exists:

- तिङन्त: धातु, लकार, पुरुष, वचन, पद, उपसर्ग if present
- कृदन्त: धातु, कृत्, उपसर्ग if present
- सुबन्त / सर्वनाम: प्रातिपदिक, लिङ्ग, विभक्ति, वचन

Example: ट्रम्पेण → प्रातिपदिक ट्रम्प, तृतीया, एकवचन.

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

Gold reference: `../skt-morph-data` (ashtadhyayi.com scrape). Runtime WASM embeds `src/data/dhatus_compact.rs` (~2259 dhātus). Gold tables (`tinanta_gold`, `krdanta_gold`, `subanta_gold`) are **not** in the default WASM build (`native-db` / `wasm-gold` only).

## Iterations

### 2026-08-27 — I/O and labels

- Locked: JS Devanagari/HK in; WASM SLP1; labels Sanskrit Devanagari; all parses kept.
- Demo: `www/translit.js`, `www/labels.js`, `www/demo.js` — convert before/after WASM; tables and analysis cards in देवनागरी.
- Prefix fields still comma-separated; converted to SLP1 before `generate_*_with_prefix`.

### 2026-08-28 — Engine: lookup, generate tests, analyze index

- `src/engine/dhatu.rs`: resolve id / name / it-marker (`gam` → `gamx` 01.1137). Shared by tinanta, kṛdanta, gold keys.
- Analyze no longer precomputes upasarga × every form (~2.4M). Index is **unprefixed** forms; `prefix::split_upasarga_candidates` peels उपसर्ग at query time.
- Tests: भू/गम् लट्, प्र+भू, क्त, रामेण / ट्रम्पेण (SLP1 `wrampeRa`).

### 2026-08-28 — Engine: live_generate vs overrides

- `tinanta.rs` is the pipeline (gold → override table → `live_generate`). Per-root tables live in `tinanta_overrides.rs`.
- *akzi विधिलिङ्* is a stem (`…ANkz`) + standard endings, not a special join.
- गम् लृट् is **गमिष्यति** (`gamizya`), not the present stem गच्छ्. Dropped the `gamx` override block; live path covers it.
