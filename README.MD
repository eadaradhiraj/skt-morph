# SktMorph: Sanskrit Morphology Analyzer & Generator

A lightning-fast, offline Python tool to analyze and generate Sanskrit morphology (Verbs, Participles, and Nouns). It uses **SLP1** transliteration internally for high-speed algorithmic Sandhi processing and bundles pre-built SQLite databases of root and shabda forms.

## Features
- **Tinanta (Verbs) & Krdanta (Participles)**: Full analysis and generation with multiple Upasargas (prefixes). Supports fetching by numeric ID (`01.0001`) or smart string matching (`BU`).
- **Subanta (Nouns)**: Algorithmic declension for ajanta (`a, A, i, I, u, U, f`), halanta (`in, as, at, an, ad, c, z, us, is`), and agent stems (`DAtA` → `DAtf`).
- **Sarvanama (Pronouns)**: `tad`, `kim`, `asmad`, `yuzmad`, `sarva`, `idam`, `etad`, `yad`, `ubha`, `ena`, `eka`, `dvi`, `sva`, `am`, `traya`, `catur`, `purva`, `para`, `apara`, `anya`.
- **Taddhita (Nominal Derivatives)**: `tva`, `tal`, `matup`/`vat`, `mayat`, `ka`, `yat`, `a`, `Iya`, `tA`, `ini`, `ana`, `thak`, `itac` — with SQLite lookup plus rule-based fallback.
- **Prakriyā traces**: Optional step-by-step derivation hints (`--with-prakriya`) from bundled shabda data and pedagogical rules.
- **Parse ranking**: Analyses include a `confidence` score to surface the most likely parse first.
- **Devanagari I/O**: Optional `--devanagari` on CLI commands when `indic-transliteration` is installed.
- **100% Offline & Fast**: No API calls during analysis.
- **100% Test Coverage**: Enforced via CI and `python scripts/check_coverage.py`.

## Installation

```bash
pip install git+https://github.com/eadaradhiraj/skt-morph.git
```

Optional Devanagari support:

```bash
pip install indic-transliteration
```

## Command Line Usage (CLI)

All commands accept SLP1 by default. Add `--devanagari` to read Devanagari input and print Devanagari output (requires `indic-transliteration`).

### Analyze

```bash
python -m sktmorph analyze praBavati
python -m sktmorph analyze rAmaH --with-prakriya
python -m sktmorph analyze te --top 3 --json
python -m sktmorph analyze rAmaH --devanagari --with-prakriya
python -m sktmorph analyze rAmatvam --type taddhita    # filter: verb, noun, pronoun, participle, declension, taddhita
```

Analysis results are ranked by `confidence` (highest first). Use `--with-prakriya` to include shabda/prakriyā steps when available. Use `--top N` to limit results and `--json` for compact JSON output (single object or array).

### Generate

```bash
python -m sktmorph generate_verb --dhatu BU --lakara plat --purusha 1 --vacana 1 --prefixes vi
python -m sktmorph generate_verb --dhatu 01.0001 --lakara plat --purusha 1 --vacana 1 --prayoga karmani --derivation san
python -m sktmorph generate_krdanta --dhatu BU --pratyaya lyuw --prefixes anu --derivation shuddha
python -m sktmorph generate_taddhita --pratipadika rAma --pratyaya tva --linga nap --with-prakriya
python -m sktmorph generate_taddhita --pratipadika rAma --pratyaya thak --linga pum
python -m sktmorph generate_taddhita --pratipadika bala --pratyaya itac --linga pum
python -m sktmorph generate_noun --base vApU --linga pum --with-prakriya
python -m sktmorph generate_pronoun --base traya --linga pum --with-prakriya
```

## Python API Usage

```python
from sktmorph.morphology import SktMorph

morph = SktMorph()

# Analyze (prakriyā omitted by default)
results = morph.analyze("vyAkaroti")
results = morph.analyze("rAmaH", include_prakriya=True)

# Generate taddhita with optional prakriyā trace
taddhita = morph.generate_taddhita("rAma", "tva", "nap", include_prakriya=True)

# Generate noun declension with prakriyā
noun = morph.generate_subanta("rAma", "pum", include_prakriya=True)
print(noun["declension"]["prathamA"])
print(noun["prakriya"][:2])

# Generate pronoun with prakriyā
pronoun = morph.generate_sarvanama("traya", "pum", include_prakriya=True)
```

### Analyze result fields

Each `MorphResult` includes: `word_type`, `pratipadika`, `linga`, `vibhakti`, `vacana`, `confidence`, and optionally `prakriya` (when `include_prakriya=True`).

## Development

```bash
pip install -e ".[devanagari]"
pip install -r requirements-dev.txt
python scripts/check_coverage.py
python scripts/ensure_data.py
```

CI runs on every push/PR to `main` (Python 3.9, 3.11, 3.12) via `.github/workflows/ci.yml`.

## Building Databases

```bash
python scripts/build_db.py          # dhatus, tinantas, krdantas + shabda ingest
python scripts/build_taddhita_db.py # seed taddhita rows only
python scripts/ingest_shabda.py     # taddhita + shabdaprakriya from ashtadhyayi.com
python scripts/ensure_data.py       # verify bundled SQLite files exist
python scripts/check_coverage.py    # run tests with 100% coverage gate
```

Bundled `.sqlite` files under `sktmorph/data/` are the runtime source. After a fresh clone, run `python scripts/build_db.py` and `python scripts/ingest_shabda.py` if databases are missing. Large database files may warrant Git LFS for forks that regenerate data frequently.

## Acknowledgments
Morphological datasets for roots (Dhatus), conjugations (Tinantas), participles (Krdantas), and shabda/prakriyā forms were sourced from **[ashtadhyayi.com](https://ashtadhyayi.com/)**.
