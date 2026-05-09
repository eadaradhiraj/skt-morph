# SktMorph: Sanskrit Morphology Analyzer & Generator

A lightning-fast, offline Python tool to analyze and generate Sanskrit morphology (Verbs, Participles, and Nouns). It uses **SLP1** transliteration for high-speed algorithmic Sandhi processing and bundles a pre-built SQLite database of root forms.

## Features
- **Tinanta (Verbs) & Krdanta (Participles)**: Full analysis and generation with multiple Upasargas (prefixes). Supports fetching by numeric ID (`01.0001`) or smart string matching (`BU`).
- **Subanta (Nouns)**: Algorithmic declension generation supporting `a, A, i, u, f (ṛ)` ending nouns and halanta stems (`in, as, at`) in all genders, with accurate *Natva* Sandhi rules.
- **Sarvanama (Pronouns)**: Specialized generator and analyzer for `tad`, `kim`, `asmad`, `yuzmad`, and `sarva`.
- **Root Details**: Fetches Pāṇinian Dhatu details (Gana, Meaning, Pada) during analysis.
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
Analyzes a combined word (with or without prefixes) and strips the Sandhi to find the root or noun stem.
```bash
python -m sktmorph analyze praBavati
python -m sktmorph analyze rAmeRa
```

### 2. Generate Verbs (Tinanta)
Generates verbs by attaching optional prefixes to base roots.
```bash
python -m sktmorph generate_verb --dhatu BU --lakara plat --purusha 1 --vacana 1 --prefixes vi A
```

### 3. Generate Participles (Krdanta)
Generates participles by dynamically attaching prefixes to root forms.
```bash
python -m sktmorph generate_krdanta --dhatu BU --pratyaya lyuw --prefixes anu
```

### 4. Generate Nouns (Subanta) & Pronouns (Sarvanama)
Generates the 24 declension forms for a given base and gender.
```bash
python -m sktmorph generate_noun --base hari --linga pum
python -m sktmorph generate_pronoun --base asmad --linga any
```

## Python API Usage

You can easily import `SktMorph` into your own NLP projects.

```python
from sktmorph.morphology import SktMorph

morph = SktMorph()

# 1. Analyze a Verb
results = morph.analyze('vyAkaroti')
for res in results:
    if res.word_type == 'tinanta':
        print(res.dhatu)            # '08.0010'
        print(res.prefixes)         # ['vi', 'A']
        print(res.dhatu_details)    # JSON dict of root meaning, gana, etc.

# 2. Generate Noun Declensions
noun_table = morph.generate_subanta('Pala', 'nap')
print(noun_table['tfIyA'])          # ['Palena', 'PalAByAm', 'PalEH']
```

## Acknowledgments
The underlying morphological datasets for roots (Dhatus), conjugations (Tinantas), and participles (Krdantas) used to build the bundled database in this project were sourced from **[ashtadhyayi.com](https://ashtadhyayi.com/)**.