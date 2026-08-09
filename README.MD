# SktMorph: Sanskrit Morphology Analyzer & Generator

A lightning-fast, offline Python tool to analyze and generate Sanskrit morphology (Verbs, Participles, and Nouns). It uses **SLP1** transliteration internally for high-speed algorithmic Sandhi processing.

## Architecture

- **Live Pāṇinian engine** (`sktmorph/engine/`): derives tinanta forms by applying rules (guṇa, śap, endings, sandhi) with sūtra-tagged steps — **default for verb generation**.
- **SQLite reference data**: dhātus, tinantas, krdantas, taddhitas, shabdaprakriya — used for analysis, fallback when a live rule is not yet implemented, and krdanta lookup.
- **Rule-based shabda**: subanta, sarvanama, taddhita paradigms in Python.

Bundled databases are committed directly (each shard **under 50 MB**). Check with:

```bash
python scripts/check_db_sizes.py
```

## Features
- **Live tinanta derivation**: all primary lakāras (`plat`, `plot`, `plrt`, `plan`, `pvidhilin`, `plit`) for gaṇas 1–10 (shuddha, kartari), plus ātmanepada (`alat`, …) for reduplicating and thematic roots
- **Live kṛdanta derivation**: ~35 pratyayas (participles, gerunds, absolutives, etc.) with DB variant merge
- **Tinanta & Krdanta analysis/generation**: SQLite-backed with live-first verb/participle generation
- **Subanta, Sarvanama, Taddhita**: algorithmic + SQLite
- **Prakriyā traces**: shabda DB + live engine steps + pedagogical fallbacks
- **Parse ranking**, **Devanagari I/O**

## Installation

```bash
git clone git@github.com:eadaradhiraj/skt-morph.git
cd skt-morph
pip install -e ".[devanagari,dev]"
python scripts/ensure_data.py
python scripts/smoke_test.py
```

Optional Devanagari: `pip install indic-transliteration` or `pip install -e ".[devanagari]"`

## Command Line Usage

### Analyze

```bash
python -m sktmorph analyze rAmaH
python -m sktmorph analyze rAmaH --with-prakriya
python -m sktmorph analyze te --top 3 --json
```

### Generate verb (live engine by default)

```bash
python -m sktmorph generate_verb --dhatu BU --lakara plat --purusha 1 --vacana 1
python -m sktmorph generate_verb --dhatu BU --lakara plat --purusha 1 --vacana 1 --with-prakriya --json
python -m sktmorph generate_verb --dhatu BU --lakara plat --purusha 1 --vacana 1 --lookup-only
python -m sktmorph generate_verb --dhatu BU --lakara plat --purusha 1 --vacana 1 --prefixes vi --json
```

Use `--lookup-only` to skip live derivation and read pre-built tinanta SQLite tables instead.

### Other generators

```bash
python -m sktmorph generate_krdanta --dhatu BU --pratyaya lyuw --json
python -m sktmorph generate_taddhita --pratipadika rAma --pratyaya Tya --linga pum --json
python -m sktmorph generate_noun --base rAma --linga pum --with-prakriya
python -m sktmorph generate_pronoun --base paJcan --linga pum
```

## Python API

```python
from sktmorph.morphology import SktMorph

morph = SktMorph()

# Live derivation (default)
forms = morph.generate_tinanta("BU", "plat", 1, 1)

# SQLite lookup fallback only
forms = morph.generate_tinanta("BU", "plat", 1, 1, live=False)

# Live engine directly with prakriyā steps
from sktmorph.engine import LiveTinantaEngine
engine = LiveTinantaEngine(morph.conn_dhatus)
form, steps = engine.generate("01.0001", "plat", 1, 1)
```

## Development

```bash
pip install -e ".[devanagari,dev]"
python scripts/check_db_sizes.py
python scripts/validate_engine.py
python scripts/validate_corpus.py
python scripts/validate_krdanta.py
python scripts/check_coverage.py
python scripts/smoke_test.py
```

### Building databases

```bash
python scripts/build_db.py
python scripts/ingest_shabda.py
python scripts/ensure_data.py --strict
```

If any `.sqlite` file exceeds 50 MB, split it before committing:

```bash
python scripts/check_db_sizes.py
```

## Live engine roadmap

| Area | Status |
|------|--------|
| Primary lakāras (`plat`–`pvidhilin`) gaṇas 1–10 | Live; **~83.1%** full-corpus match (`validate_corpus.py`, Aug 2026) |
| Gaṇa 9 (R-infix vs n-infix) | Live via per-root n-infix set (`mI`, `sku`, …); default R-infix |
| Gaṇa 10 causative vs thematic | Live via `nityaRic` tag routing |
| BidAdi (`antarganas`) | Present `-aya-`; laṅ/vidhilin stems |
| `-Aya-` presents (`gup`, `glE`, …) | Live (`thematic_aya_present_stem`) |
| Perfect `plit` | Live (gaṇa 1; gaṇa 3 hu; thematic pattern) |
| Gaṇa 3 reduplication | Live (`engine/redup.py` — hu, BI, hrI, pF, dA, …) |
| Ātmanepada (`alat`, `alot`, `alang`, `alit`) | Live (gaṇa 3 I-stems; thematic otherwise) |
| Live kṛdanta (~35 pratyayas) | Live + DB variant merge |
| Full-corpus validation | `python scripts/validate_corpus.py` — triage with `analyze_corpus_wrong.py` |
| Karmani, niç, san, yaṅ derivatives | Planned |
| Subanta / taddhita live (beyond current rules) | Partial |

## Acknowledgments

Morphological datasets from **[ashtadhyayi.com](https://ashtadhyayi.com/)**. Live engine follows Pāṇinian rule ordering pedagogically; it is not a reimplementation of vidyut.
