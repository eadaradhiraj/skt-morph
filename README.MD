# SktMorph: Sanskrit Morphology Analyzer & Generator

A lightning-fast, offline Python tool to analyze and generate Sanskrit morphology (Verbs, Participles, and Nouns). It uses **SLP1** transliteration for high-speed algorithmic Sandhi processing and bundles a pre-built SQLite database of root forms.

## Features
- **Tinanta (Verbs) & Krdanta (Participles)**: Full analysis and generation with multiple Upasargas (prefixes).
- **Subanta (Nouns)**: Algorithmic declension generation supporting `a, A, i, u, f (ṛ)` ending nouns in all genders, with accurate *Natva* Sandhi rules.
- **Root Details**: Fetches Pāṇinian Dhatu details (Gana, Meaning, Padi) during analysis.
- **100% Offline & Fast**: No API calls; relies on a bundled SQLite database.
- **100% Test Coverage**: Rock-solid algorithmic foundation.

## Installation

You can install this package directly from GitHub:
```bash
pip install git+https://github.com/eadaradhiraj/skt-morph.git
```

## Command Line Usage (CLI)

The package provides a powerful CLI tool accessible via `python -m sktmorph`.

### 1. Analyze a Word
Analyzes a combined word (with or without prefixes) and strips the Sandhi to find the root.
```bash
python -m sktmorph analyze praBavati
```

### 2. Generate Verbs (Tinanta)
Generates verbs by attaching optional prefixes to base roots.
```bash
# dhatu: 01.0001 (bhū), lakara: plat (Lat), purusha: 1 (Prathama), vacana: 1 (Eka)
python -m sktmorph generate_verb --dhatu 01.0001 --lakara plat --purusha 1 --vacana 1 --prefixes vi A
```

### 3. Generate Nouns (Subanta)
Generates the 24 declension forms for a given base and gender.
```bash
python -m sktmorph generate_noun --base hari --linga pum
python -m sktmorph generate_noun --base mati --linga stri
```

## Python API Usage

You can easily import `SktMorph` into your own NLP projects.

```python
from sktmorph.morphology import SktMorph

morph = SktMorph()

# 1. Analyze
results = morph.analyze('vyAkaroti')
for res in results:
    print(res.dhatu)            # '08.0010'
    print(res.prefixes)         #['vi', 'A']
    print(res.dhatu_details)    # JSON dict of root meaning, gana, etc.

# 2. Generate Subantas
noun_table = morph.generate_subanta('Pala', 'nap')
print(noun_table['tfIyA'])      # ['Palena', 'PalAByAm', 'PalEH']
```