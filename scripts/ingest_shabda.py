"""Ingest ashtadhyayi.com shabda data into bundled SQLite databases."""
import json
import os
import re
import sqlite3
import ssl
import sys
import urllib.request
from typing import Dict, Iterable, List, Optional, Set, Tuple

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from sktmorph.taddhita import LINGA_MAP, derive_stem_rule, normalize_pratyaya, split_taddhita_stem

try:
    from indic_transliteration import sanscript
    from indic_transliteration.sanscript import transliterate
except ImportError:
    sanscript = None
    transliterate = None

DATA_RAW = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "data_raw"))
OUTPUT_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "sktmorph", "data"))
TADDHITA_DB = os.path.join(OUTPUT_DIR, "taddhitas.sqlite")
PRAKRIYA_DB = os.path.join(OUTPUT_DIR, "shabdaprakriya.sqlite")

SHABDA_URLS = {
    "data2.txt": "https://raw.githubusercontent.com/ashtadhyayi-com/data/master/shabda/data2.txt",
    "shabdaprakriya.txt": "https://raw.githubusercontent.com/ashtadhyayi-com/data/master/shabda/shabdaprakriya.txt",
}

VIBHAKTI_NUM_TO_NAME = {
    "1": "prathamA",
    "2": "dvitIyA",
    "3": "tfIyA",
    "4": "caturTI",
    "5": "paYcamI",
    "6": "zazWI",
    "7": "saptamI",
    "8": "samboDana",
}


def to_slp1(text: str) -> str:
    if not text:
        return ""
    if transliterate is None:
        return text
    return transliterate(text, sanscript.DEVANAGARI, sanscript.SLP1)


def map_linga(code: str) -> str:
    return LINGA_MAP.get(code, code)


def download_shabda_file(name: str) -> str:
    os.makedirs(DATA_RAW, exist_ok=True)
    path = os.path.join(DATA_RAW, name)
    if os.path.exists(path) and os.path.getsize(path) > 1000:
        return path
    url = SHABDA_URLS[name]
    ctx = ssl.create_default_context()
    ctx.check_hostname = False
    ctx.verify_mode = ssl.CERT_NONE
    req = urllib.request.Request(url, headers={"User-Agent": "skt-morph-build"})
    with urllib.request.urlopen(req, context=ctx, timeout=180) as resp:
        raw = resp.read()
    with open(path, "wb") as f:
        f.write(raw)
    return path


def load_json(path: str):
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def extract_taddhita_rows(items: Iterable[dict]) -> List[Tuple[str, str, str, str, str]]:
    word_entries: List[Tuple[str, str]] = []
    word_set: Set[str] = set()
    for item in items:
        word = to_slp1(item.get("word", ""))
        if not word:
            continue
        linga = map_linga(item.get("linga", "pum"))
        word_entries.append((word, linga))
        word_set.add(word)

    rows = []
    seen = set()
    for stem, linga in word_entries:
        for pratipadika, pratyaya in split_taddhita_stem(stem):
            if pratipadika not in word_set:
                continue
            if derive_stem_rule(pratipadika, pratyaya) != stem:
                continue
            key = (pratipadika, pratyaya, linga, stem)
            if key in seen:
                continue
            seen.add(key)
            rows.append((pratipadika, pratyaya, linga, stem, "data2"))

    vyutpatti_re = re.compile(r"\[\[(?:\d\.)?\d+\.\d+\.\d+\]\]")
    vyutpatti_keywords = ("taddhita", "tva", "matup", "mayat", "Iya", "tA", "ini", "ana", "yat", "tal", "thak", "itac")
    for item in items:
        vy = item.get("vyutpatti") or ""
        if not vy or not any(k in vy for k in vyutpatti_keywords):
            continue
        if not vyutpatti_re.search(vy):
            continue
        stem = to_slp1(item.get("word", ""))
        if not stem:
            continue
        linga = map_linga(item.get("linga", "pum"))
        for pratipadika, pratyaya in split_taddhita_stem(stem):
            if pratipadika not in word_set:
                continue
            derived = derive_stem_rule(pratipadika, pratyaya)
            if derived and derived != stem and pratyaya not in ("a", "Iya"):
                continue
            key = (pratipadika, pratyaya, linga, stem)
            if key not in seen:
                seen.add(key)
                rows.append((pratipadika, pratyaya, linga, stem, "vyutpatti"))
    return rows


def build_taddhita_db(items: Iterable[dict], seed_rows: Optional[List[Tuple[str, str, str]]] = None) -> int:
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    if os.path.exists(TADDHITA_DB):
        os.remove(TADDHITA_DB)
    conn = sqlite3.connect(TADDHITA_DB)
    conn.execute(
        """CREATE TABLE taddhitas (
            pratipadika TEXT, pratyaya TEXT, linga TEXT, stem_slp1 TEXT, source TEXT
        )"""
    )
    conn.execute("CREATE INDEX idx_taddhita_stem ON taddhitas(stem_slp1)")
    conn.execute("CREATE INDEX idx_taddhita_lookup ON taddhitas(pratipadika, pratyaya, linga)")

    count = 0
    if seed_rows:
        for pratipadika, pratyaya, linga in seed_rows:
            stem = derive_stem_rule(pratipadika, normalize_pratyaya(pratyaya))
            if stem:
                conn.execute(
                    "INSERT INTO taddhitas VALUES (?, ?, ?, ?, ?)",
                    (pratipadika, normalize_pratyaya(pratyaya), linga, stem, "seed"),
                )
                count += 1

    for row in extract_taddhita_rows(items):
        conn.execute("INSERT INTO taddhitas VALUES (?, ?, ?, ?, ?)", row)
        count += 1

    conn.commit()
    conn.close()
    return count


def build_prakriya_db(items: Iterable[dict]) -> int:
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    if os.path.exists(PRAKRIYA_DB):
        os.remove(PRAKRIYA_DB)
    conn = sqlite3.connect(PRAKRIYA_DB)
    conn.execute(
        """CREATE TABLE form_prakriya (
            form_slp1 TEXT,
            word_slp1 TEXT,
            vibhakti TEXT,
            vacana INTEGER,
            steps_json TEXT
        )"""
    )
    conn.execute("CREATE INDEX idx_form_prakriya ON form_prakriya(form_slp1)")

    count = 0
    for item in items:
        form = to_slp1(item.get("form", ""))
        word = to_slp1(item.get("word", ""))
        if not form or not word:
            continue
        vibh = VIBHAKTI_NUM_TO_NAME.get(str(item.get("vibhakti", "")), "prathamA")
        vacana = int(item.get("vachan") or item.get("vacana") or 1)
        steps = item.get("steps") or []
        conn.execute(
            "INSERT INTO form_prakriya VALUES (?, ?, ?, ?, ?)",
            (form, word, vibh, vacana, json.dumps(steps, ensure_ascii=False)),
        )
        count += 1

    conn.commit()
    conn.close()
    return count


def ingest_all(use_download: bool = True, data2_items: Optional[List[dict]] = None,
               prakriya_items: Optional[List[dict]] = None) -> Dict[str, int]:
    if data2_items is None:
        path = download_shabda_file("data2.txt") if use_download else os.path.join(DATA_RAW, "data2.txt")
        data2_items = load_json(path)["data"]
    if prakriya_items is None:
        path = download_shabda_file("shabdaprakriya.txt") if use_download else os.path.join(DATA_RAW, "shabdaprakriya.txt")
        payload = load_json(path)
        prakriya_items = payload.get("data") or payload[list(payload.keys())[0]]

    from sktmorph.taddhita import SEED_ENTRIES

    t_count = build_taddhita_db(data2_items, seed_rows=SEED_ENTRIES)
    p_count = build_prakriya_db(prakriya_items)
    return {"taddhitas": t_count, "prakriya_forms": p_count}


if __name__ == "__main__":
    stats = ingest_all(use_download=True)
    print(f"Built databases in {OUTPUT_DIR}: {stats}")
