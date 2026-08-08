"""Pāṇinian-style derivation traces (pedagogical; not a full rule engine)."""
from typing import Any, Dict, List, Optional

SUTRA_TADDHITA = {
    "tva": ["5.3.23", "5.3.25"],
    "tal": ["5.3.23"],
    "matup": ["5.2.94", "5.2.96"],
    "mayat": ["5.4.21"],
    "ka": ["4.3.105", "5.3.70"],
    "yat": ["4.1.83", "4.1.84"],
    "a": ["4.3.54", "5.4.3"],
    "Iya": ["4.3.54", "7.1.17"],
    "tA": ["4.1.15", "7.3.114"],
    "ini": ["5.3.71"],
    "ana": ["5.3.72"],
    "thak": ["5.3.70", "4.4.72"],
    "itac": ["4.4.62", "7.2.115"],
    "Tya": ["4.1.88", "4.1.104"],
    "Tyan": ["4.1.88", "4.1.84"],
    "Ca": ["4.3.58", "5.3.53"],
}

SUTRA_DECLENSION = ["4.1.2", "7.1.12"]
SUTRA_SANDHI = ["6.1.101", "8.4.40"]

VIBHAKTI_NAMES = [
    "prathamA", "dvitIyA", "tfIyA", "caturTI",
    "paYcamI", "zazWI", "saptamI", "samboDana",
]


def trace_taddhita_derivation(pratipadika: str, pratyaya: str, stem: str) -> List[Dict[str, Any]]:
    steps = [
        {
            "step": f"{pratipadika} + {pratyaya}",
            "sutras": SUTRA_TADDHITA.get(pratyaya, ["4.3.1"]),
            "kind": "taddhita",
        },
        {
            "step": stem,
            "sutras": SUTRA_SANDHI,
            "kind": "stem",
        },
    ]
    return steps


def trace_declension(stem: str, suffix: str, form: str, vibhakti: str) -> List[Dict[str, Any]]:
    return [
        {
            "step": f"{stem} + {suffix}",
            "sutras": SUTRA_DECLENSION,
            "kind": "declension",
            "vibhakti": vibhakti,
        },
        {"step": form, "sutras": SUTRA_SANDHI, "kind": "form"},
    ]


def trace_declension_table(
    base: str,
    ending: str,
    endings_table: List[List[str]],
    table: Dict[str, List[str]],
) -> List[Dict[str, Any]]:
    """One prakriya step per vibhakti (singular slot) for a generated paradigm."""
    steps: List[Dict[str, Any]] = []
    stem = base + ending
    for idx, vibhakti in enumerate(VIBHAKTI_NAMES):
        suffix_group = endings_table[idx][0]
        suffix = suffix_group.split(",")[0]
        form = table[vibhakti][0].split("/")[0]
        steps.extend(trace_declension(stem, suffix, form, vibhakti))
    return steps


def merge_traces(*traces: Optional[List[Dict[str, Any]]]) -> List[Dict[str, Any]]:
    merged: List[Dict[str, Any]] = []
    for trace in traces:
        if trace:
            merged.extend(trace)
    return merged


def fallback_prakriya_for_parse(
    word: str,
    word_type: str,
    pratipadika: Optional[str],
    pratyaya: Optional[str],
    linga: Optional[str],
    vibhakti: Optional[str],
    stem: Optional[str] = None,
) -> Optional[List[Dict[str, Any]]]:
    """Pedagogical fallback when shabda DB has no stored trace."""
    if word_type == "taddhita" and pratipadika and pratyaya and stem:
        return trace_taddhita_derivation(pratipadika, pratyaya, stem)
    if word_type in ("subanta", "sarvanama", "taddhita") and pratipadika and vibhakti:
        label = pratipadika if word_type == "sarvanama" else (stem or pratipadika)
        return [
            {
                "step": f"{label} ({vibhakti})",
                "sutras": SUTRA_DECLENSION,
                "kind": "declension",
                "vibhakti": vibhakti,
            },
            {"step": word, "sutras": SUTRA_SANDHI, "kind": "form"},
        ]
    return None
