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

In: śuddha kartari तिङन्त (10 gaṇas; लट्, लङ्, लोट्, विधिलिङ्, लृट्, लिट्, लुङ्, आशीर्लिङ्; parasmai and ātmanepada), listed kṛt pratyayas, ending-class सुबन्त (including foreign stems such as ट्रम्प → ट्रम्पेण), सर्वनाम. Started: णिच् / सन् / यङ्, कर्मणि यक्. Optional अर्थ on 1.3 meaning-conditioned pada.

Later: fuller यङ् (now yaN + yaNluk luk 2.4.74, e.g. `boBU`/`vAvac`), remaining क्त ādeśa (जग्ध/पक्व/सोढ/भग्न/द्यूत/नद्ध/जीन; class sandhi 8.2.30/36/37/40/42/45, 6.4.37/42, 6.1.15/16/45), fuller सुबन्त (now `j/d/t/p/h/B/r/s/S/as/is/us` halanta). Derived tinanta stays out of the analyze index.

All तिङन्त is now live (`stems`/`adadi`/`join`/sutras) — no override table and no scrape rows in the generate path.

## I/O

| Layer | Script / names |
| --- | --- |
| JS demo (`www/`) | Input: **Devanagari** or **Harvard-Kyoto**. Display: Devanagari. Grammatical labels: Sanskrit in Devanagari only (प्रथमा, एकवचन, लट्, …). |
| WASM / Rust | **SLP1 only** for stems, forms, dhātus, upasargas. Internal codes (`plat`, `prathamA`, `pum`; puruṣa/vacana 1–3 with 1 = प्रथमपुरुष). |

`www/translit.js` converts in and out. `www/labels.js` maps codes → देवनागरी.

## Generate

- **तिङन्त** — stem → ending → sandhi (`live_generate`). Optional upasargas (`prefix.rs`). 1.3 अर्थ is engine-only (no demo dropdown). Optional derivation: णिच् / सन् / यङ् / यङ्लुक् (luk 2.4.74, parasmai `boBU`/`vAvac` vs yaN `boBUya` ātmane) / कर्मणि (`derived.rs`; not indexed in analyze) — sūtra-cited in `phonology`/`join`/`derived`.
- **कृदन्त** — kṛt such as क्त (class sandhi before इट्: मुक्त/लब्ध/भिन्न/गत/गीत/उत/उष्ट/रक्त; named ādeśa जग्ध/पक्व/सोढ/भग्न/द्यूत/नद्ध/जीन), क्तवतु, शतृ, शानच्, तुमुन्, क्त्वा, ल्यप्, तव्य, तृच्, … Same prefixes. Sūtra docs on `krdanta::nistha_base`.
- **सुबन्त / सर्वनाम** — ending-class (now `j/d/t/p/h/B/r/s/S/as/is/us/an/in` halanta + `a/ A/ i/ I/ u/ U/ f` etc., plus `o/O` for गो/नौ) or pronoun paradigms — halanta via 8.2.30/39/66 in `declension::subanta`.

## Analyze

One surface form (after JS → SLP1) → every parse:

- तिङन्त: धातु, लकार, पुरुष, वचन, पद, उपसर्ग if present
- कृदन्त: धातु, कृत्, उपसर्ग if present
- सुबन्त / सर्वनाम: प्रातिपदिक, लिङ्ग, विभक्ति, वचन

Demo: **विश्लेषण** / **तिङन्तान्वेषण** / **सुबन्तविश्लेषण**. Generate is one **धातु** page (तिङन्त + कृदन्त, as on ashtadhyayi.com). Upasargas: 1.4.59 chips (what the scrape stores) plus a free field; unlisted prefixes still take Pāṇinian sandhi, they just have no `skt_morph.db` gold.

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

Recent: 64 iters (2026-09-01) — de-noise 70k, KRTS 19→60+, clamp, yaNluk UI, a11y/Enter, headers for all modules, dhatu HashMap cache 15s→7s, analyze/tinanta empty guards, derived/phonology/join/analyze hints + natva test, verb try/catch, lib clamp (186 tests, 1.2M wasm). Dated engine notes: [CHANGELOG.md](CHANGELOG.md).

