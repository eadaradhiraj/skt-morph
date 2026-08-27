# Changelog

Engine and demo history. The project outline lives in [README.MD](README.MD).

## 2026-08-28

- **णत्व / लिट्:** parasmai as before; **ātmanepada** 3.4.81 एश्/इरेच् on the weak aṅga (ईजे, चक्रे, निन्ये, जगृहे, बभूवे).

- **Kaumudī first:** generate never returns scrape tables. `validate` vs `skt_morph.db` is a probe only.
- **live_generate:** gold tables dropped from the generate path. `tinanta_overrides.rs` is a shrinking patch list. गम् लृट् is गमिष्यति (`gamizya`). *akzi विधिलिङ् is a stem, not a special join.
- **Lookup / analyze:** dhātu by id / name / it (`gam` → `gamx`). Analyze indexes unprefixed forms; upasargas peeled at query time.

## 2026-08-27

- JS demo: Devanagari or Harvard-Kyoto in; display and grammatical names in देवनागरी. WASM stays SLP1 (`www/translit.js`, `www/labels.js`).
