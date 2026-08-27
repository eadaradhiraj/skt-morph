# SktMorph

Sanskrit morphology as a Rust library and WebAssembly module: **generate** forms and **analyse** them.

The model is **Pāṇini**, in the order and spirit of the **Siddhānta-Kaumudī** — dhātu → vikaraṇa → lakāra / kṛt / sup → sandhi. A form is correct when the sūtra-path is correct, not when it matches a website.

## Intention

- A small, embeddable engine for तिङन्त, कृदन्त, and सुबन्त (plus सर्वनाम).
- Reverse lookup that returns **every** valid parse, not a single guess.
- User-supplied **upasargas** on verbs and kṛts, with Pāṇinian sandhi — any combinatorially valid list, not only prefixes attested in a scrape.
- No prakriyā dump, no accents, no kośa or commentary.

ashtadhyayi.com data (`skt_morph.db`) is an optional **cross-check**. If live ≠ scrape, read the Kaumudī; do not copy the site into the engine.

## Scope (v1)

In: śuddha kartari तिङन्त (10 gaṇas; लट्, लङ्, लोट्, विधिलिङ्, लृट्, लिट्; parasmai and ātmanepada), listed kṛt pratyayas, ending-class सुबन्त (including foreign stems such as ट्रम्प → ट्रम्पेण), सर्वनाम.

Later: ṇic, san, yaṅ; fuller लिट् / लुङ् prakriyā; tighter ātmanepada (1.3.12–77).

`tinanta_overrides.rs` is a temporary patch list where the Kaumudī path is not written yet. New work goes into stems, endings, join, and sandhi — not into that table, and not into scrape rows.

## I/O

| Layer | Script / names |
| --- | --- |
| JS demo (`www/`) | Input: **Devanagari** or **Harvard-Kyoto**. Display: Devanagari. Grammatical labels: Sanskrit in Devanagari only (प्रथमा, एकवचन, लट्, …). |
| WASM / Rust | **SLP1 only** for stems, forms, dhātus, upasargas. Internal codes (`plat`, `prathamA`, `pum`; puruṣa/vacana 1–3 with 1 = प्रथमपुरुष). |

`www/translit.js` converts in and out. `www/labels.js` maps codes → देवनागरी.

## Generate

- **तिङन्त** — stem → ending → sandhi (`live_generate`). Optional upasargas (`prefix.rs`).
- **कृदन्त** — kṛt such as क्त, क्तवतु, शतृ, शानच्, तुमुन्, क्त्वा, ल्यप्, … Same prefixes.
- **सुबन्त / सर्वनाम** — ending-class (or pronoun) paradigms.

## Analyze

One surface form (after JS → SLP1) → every parse:

- तिङन्त: धातु, लकार, पुरुष, वचन, पद, उपसर्ग if present
- कृदन्त: धातु, कृत्, उपसर्ग if present
- सुबन्त / सर्वनाम: प्रातिपदिक, लिङ्ग, विभक्ति, वचन

## Build

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo build
wasm-pack build --target web   # → pkg/
cd www && npm i && npm run dev
```

Runtime WASM embeds the dhātupāṭha (`src/data/dhatus_compact.rs`, ~2259 roots). Do not ship the scrape DB in the browser.

Optional probe (not the spec):

```bash
cargo run --release --bin validate --features native-db -- --full --live
```

Uses `../skt-morph-data/data/skt_morph.db` (or `--db` / `SKT_MORPH_DB`). That scrape’s prefixes are only the site’s chandrika/upasarga pages.

Dated engine notes: [CHANGELOG.md](CHANGELOG.md).

