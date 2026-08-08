import sqlite3
import os
from typing import Dict, List, Optional, Tuple
from .subanta import SubantaGenerator
from .prakriya import merge_traces, trace_declension_table, trace_taddhita_derivation

PRATYAYA_ALIASES = {
    "vat": "matup",
    "mat": "matup",
    "tal": "tal",
    "tva": "tva",
    "mayat": "mayat",
    "matup": "matup",
    "ka": "ka",
    "yat": "yat",
    "a": "a",
    "Iya": "Iya",
    "iya": "Iya",
    "tA": "tA",
    "ta": "tA",
    "ini": "ini",
    "ana": "ana",
}

SEED_ENTRIES = [
    ("rAma", "tva", "nap"),
    ("rAma", "tal", "nap"),
    ("rAma", "matup", "pum"),
    ("rAma", "mayat", "pum"),
    ("rAma", "Iya", "pum"),
    ("rAma", "tA", "stri"),
    ("nara", "matup", "pum"),
    ("nara", "tva", "nap"),
    ("anna", "mayat", "pum"),
    ("anna", "tva", "nap"),
    ("grAma", "tal", "pum"),
    ("grAma", "tva", "nap"),
    ("rAjan", "tva", "nap"),
    ("hari", "tva", "nap"),
    ("hari", "matup", "pum"),
    ("deva", "a", "pum"),
    ("putra", "ini", "pum"),
]

STEM_SUFFIXES: List[Tuple[str, str]] = [
    ("tala", "tal"),
    ("tva", "tva"),
    ("maya", "mayat"),
    ("yata", "yat"),
    ("Iya", "Iya"),
    ("Ana", "ana"),
    ("ini", "ini"),
    ("tA", "tA"),
    ("vat", "matup"),
    ("mat", "matup"),
    ("ka", "ka"),
]

LINGA_MAP = {"pum": "pum", "stri": "stri", "nap": "nap", "P": "pum", "S": "stri", "N": "nap"}


def normalize_pratyaya(pratyaya: str) -> str:
    key = pratyaya.strip()
    if key not in PRATYAYA_ALIASES:
        raise ValueError(
            f"Unsupported taddhita pratyaya '{pratyaya}'. "
            f"Supported: {', '.join(sorted(set(PRATYAYA_ALIASES.keys())))}"
        )
    return PRATYAYA_ALIASES[key]


def _append_suffix(stem: str, suffix: str) -> str:
    if stem.endswith("a") and suffix in ("ini", "ana", "Iya"):
        return stem[:-1] + suffix
    if stem.endswith("a") and suffix.startswith("t"):
        return stem + suffix
    if stem.endswith("A") and suffix.startswith("t"):
        return stem[:-1] + "a" + suffix
    if stem.endswith("an"):
        return stem[:-2] + "a" + suffix
    if stem.endswith("in"):
        return stem[:-2] + "i" + suffix
    return stem + suffix


def _derive_iya(pratipadika: str) -> str:
    if pratipadika.endswith("a") and len(pratipadika) > 1:
        return pratipadika[:-1] + "Iya"
    if pratipadika.endswith("A") and len(pratipadika) > 1:
        return pratipadika[:-1] + "Iya"
    return pratipadika + "Iya"


def derive_stem_rule(pratipadika: str, pratyaya: str) -> Optional[str]:
    if not pratipadika:
        return None
    pratyaya = normalize_pratyaya(pratyaya)

    if pratyaya == "tva":
        return _append_suffix(pratipadika, "tva")
    if pratyaya == "tal":
        return _append_suffix(pratipadika, "tala")
    if pratyaya == "matup":
        return _append_suffix(pratipadika, "vat")
    if pratyaya == "mayat":
        return _append_suffix(pratipadika, "maya")
    if pratyaya == "ka":
        return _append_suffix(pratipadika, "ka")
    if pratyaya == "yat":
        return _append_suffix(pratipadika, "yata")
    if pratyaya == "a":
        if pratipadika.endswith("a"):
            return pratipadika
        return pratipadika + "a"
    if pratyaya == "Iya":
        return _derive_iya(pratipadika)
    if pratyaya == "tA":
        return _append_suffix(pratipadika, "tA")
    if pratyaya == "ini":
        return _append_suffix(pratipadika, "ini")
    if pratyaya == "ana":
        if pratipadika.endswith("a") and len(pratipadika) > 1:
            return pratipadika[:-1] + "Ana"
        return _append_suffix(pratipadika, "ana")
    return None


def split_taddhita_stem(stem: str) -> List[Tuple[str, str]]:
    results = []
    for surface_suffix, pratyaya in STEM_SUFFIXES:
        if not stem.endswith(surface_suffix) or len(stem) <= len(surface_suffix):
            continue
        base = stem[: -len(surface_suffix)]
        candidates = [base]
        if pratyaya in ("tva", "tal", "mayat", "yat", "tA", "Iya", "ini", "ana"):
            if not base.endswith(("a", "A", "i", "I", "u", "U", "f", "F")):
                candidates.append(base + "an")
            if base.endswith("a") and len(base) > 1:
                candidates.append(base + "n")
                candidates.append(base[:-1] + "A")
            if base.endswith("i") and len(base) > 1:
                candidates.append(base + "n")
            if pratyaya == "Iya" and not base.endswith("a"):
                candidates.append(base + "a")
            if pratyaya in ("ini", "ana") and not base.endswith("a"):
                candidates.append(base + "a")
        for pratipadika in candidates:
            results.append((pratipadika, pratyaya))
    return results


class TaddhitaGenerator:
    def __init__(self, db_path: str = None):
        if db_path is None:
            db_path = os.path.join(os.path.dirname(__file__), "data", "taddhitas.sqlite")
        self.db_path = db_path
        self.subanta = SubantaGenerator()
        self._conn = None
        if os.path.exists(db_path):
            self._conn = sqlite3.connect(db_path)
            self._conn.row_factory = sqlite3.Row

    def derive_stem(self, pratipadika: str, pratyaya: str, linga: str = None) -> Optional[str]:
        pratyaya = normalize_pratyaya(pratyaya)
        linga = LINGA_MAP.get(linga, linga) if linga else None

        if self._conn:
            cursor = self._conn.cursor()
            if linga:
                cursor.execute(
                    "SELECT stem_slp1 FROM taddhitas WHERE pratipadika = ? AND pratyaya = ? AND linga = ?",
                    (pratipadika, pratyaya, linga),
                )
            else:
                cursor.execute(
                    "SELECT stem_slp1 FROM taddhitas WHERE pratipadika = ? AND pratyaya = ? LIMIT 1",
                    (pratipadika, pratyaya),
                )
            row = cursor.fetchone()
            if row:
                return row["stem_slp1"]

        return derive_stem_rule(pratipadika, pratyaya)

    def lookup_by_stem(self, stem: str) -> List[Dict[str, str]]:
        rows = []
        seen = set()
        if self._conn:
            cursor = self._conn.cursor()
            cursor.execute(
                "SELECT pratipadika, pratyaya, linga, stem_slp1 FROM taddhitas WHERE stem_slp1 = ?",
                (stem,),
            )
            for r in cursor.fetchall():
                key = (r["pratipadika"], r["pratyaya"])
                if key not in seen:
                    seen.add(key)
                    rows.append(
                        {
                            "pratipadika": r["pratipadika"],
                            "pratyaya": r["pratyaya"],
                            "linga": r["linga"],
                            "stem": r["stem_slp1"],
                        }
                    )

        for pratipadika, pratyaya in split_taddhita_stem(stem):
            key = (pratipadika, pratyaya)
            if key in seen:
                continue
            if self.derive_stem(pratipadika, pratyaya) == stem:
                seen.add(key)
                rows.append(
                    {
                        "pratipadika": pratipadika,
                        "pratyaya": pratyaya,
                        "linga": None,
                        "stem": stem,
                    }
                )
        return rows

    def generate(self, pratipadika: str, pratyaya: str, linga: str, include_prakriya: bool = True) -> Dict:
        linga = LINGA_MAP.get(linga, linga)
        canonical = normalize_pratyaya(pratyaya)
        stem = self.derive_stem(pratipadika, pratyaya, linga)
        if not stem:
            raise NotImplementedError(
                f"Could not derive taddhita stem for '{pratipadika}' + '{pratyaya}'."
            )
        try:
            detail = self.subanta.generate_detail(stem, linga)
            declension = detail["declension"]
        except NotImplementedError as exc:
            raise NotImplementedError(
                f"Derived stem '{stem}' from '{pratipadika}' + '{pratyaya}' "
                f"but cannot decline it as {linga}: {exc}"
            ) from exc

        result = {
            "pratipadika": pratipadika,
            "pratyaya": canonical,
            "linga": linga,
            "stem": stem,
            "declension": declension,
        }
        if include_prakriya:
            taddhita_trace = trace_taddhita_derivation(pratipadika, canonical, stem)
            declension_trace = trace_declension_table(
                detail["base"], detail["ending"], detail["endings_table"], declension
            )
            result["prakriya"] = merge_traces(taddhita_trace, declension_trace)
        return result

    def analyze_stem(self, stem: str) -> List[Dict[str, str]]:
        return self.lookup_by_stem(stem)
