# SktMorph: Sanskrit Morphology Analyzer & Generator

A lightning-fast, offline Python tool to analyze and generate Sanskrit morphology (Verbs, Participles, and Nouns). It uses **SLP1** transliteration internally for high-speed algorithmic Sandhi processing and bundles pre-built SQLite databases of root and shabda forms.

## Features
- **Tinanta (Verbs) & Krdanta (Participles)**: Full analysis and generation with multiple Upasargas (prefixes). Supports fetching by numeric ID (`01.0001`) or smart string matching (`BU`).
- **Subanta (Nouns)**: Algorithmic declension for ajanta (`a, A, i, I, u, U, f`), halanta (`in, as, at, an, ad, c, z, us, is`), and agent stems (`DAtA` → `DAtf`).
- **Sarvanama (Pronouns)**: `tad`, `kim`, `asmad`, `yuzmad`, `sarva`, `idam`, `etad`, `yad`, `ubha`, `ena`, `eka`, `dvi`, `sva`, `am`.
- **Taddhita (Nominal Derivatives)**: `tva`, `tal`, `matup`/`vat`, `mayat`, `ka`, `yat`, `a`, `Iya`, `tA`, `ini`, `ana` — with SQLite lookup plus rule-based fallback.
- **Prakriyā traces**: Optional step-by-step derivation hints (`--with-prakriya`) from bundled shabda data and pedagogical rules.
- **Parse ranking**: Analyses include a `confidence` score to surface the most likely parse first.
- **Devanagari I/O**: Optional `--devanagari` on CLI commands when `indic-transliteration` is installed.
- **100% Offline & Fast**: No API calls during analysis.
- **100% Test Coverage**: Enforced via `python scripts/check_coverage.py`.

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
python -m sktmorph analyze rAmaH --devanagari --with-prakriya
python -m sktmorph analyze rAmatvam --type taddhita    # filter: verb, noun, pronoun, participle, declension, taddhita
```

Analysis results are ranked by `confidence` (highest first). Use `--with-prakriya` to include shabda/prakriyā steps when available.

### Generate

```bash
python -m sktmorph generate_verb --dhatu BU --lakara plat --purusha 1 --vacana 1 --prefixes vi
python -m sktmorph generate_krdanta --dhatu BU --pratyaya lyuw --prefixes anu
python -m sktmorph generate_taddhita --pratipadika rAma --pratyaya tva --linga nap --with-prakriya
python -m sktmorph generate_taddhita --pratipadika rAma --pratyaya Iya --linga pum
python -m sktmorph generate_noun --base vApU --linga pum --with-prakriya
python -m sktmorph generate_pronoun --base eka --linga pum
python -m sktmorph analyze rAmatvam --type taddhita
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
```

### Analyze result fields

Each `MorphResult` includes: `word_type`, `pratipadika`, `linga`, `vibhakti`, `vacana`, `confidence`, and optionally `prakriya` (when `include_prakriya=True`).

## Development

```bash
pip install pytest coverage indic-transliteration
python scripts/check_coverage.py
```

## Building Databases

```bash
python scripts/build_db.py          # dhatus, tinantas, krdantas + shabda ingest
python scripts/build_taddhita_db.py   # seed taddhita rows only
python scripts/ingest_shabda.py     # taddhita + shabdaprakriya from ashtadhyayi.com
python scripts/check_coverage.py    # run tests with 100% coverage gate
```

## Acknowledgments
Morphological datasets for roots (Dhatus), conjugations (Tinantas), participles (Krdantas), and shabda/prakriyā forms were sourced from **[ashtadhyayi.com](https://ashtadhyayi.com/)**.
