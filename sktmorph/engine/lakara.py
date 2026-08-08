"""Lakāra normalization and classification."""

from typing import Optional, Tuple

# User-facing codes (constants.py) -> bundled SQLite codes
CANONICAL_TO_DB = {
    "plat": "plat",
    "plrt": "plrut",
    "plot": "plot",
    "plan": "plang",
    "pvidhilin": "pvidhiling",
    "alat": "alat",
    "alrt": "alrut",
    "alot": "alot",
    "alan": "alang",
    "aling": "aashirling",
    "alit": "alit",
}

DB_TO_CANONICAL = {v: k for k, v in CANONICAL_TO_DB.items()}
DB_TO_CANONICAL["pashirling"] = "pvidhilin"
DB_TO_CANONICAL["plit"] = "plat"  # parasmaipada perfect uses distinct endings

# Lakāra family drives stem formation and ending tables
LAKARA_FAMILIES = {
    "plat": "lat",
    "alat": "lat",
    "plot": "lot",
    "alot": "lot",
    "plrut": "lrt",
    "alrut": "lrt",
    "plang": "lang",
    "alang": "lang",
    "pvidhiling": "vidhilin",
    "pashirling": "vidhilin",
    "aashirling": "vidhilin",
    "avidhiling": "vidhilin",
    "plit": "lit",
    "alit": "lit",
}


def normalize_lakara(lakara: str) -> Tuple[str, str]:
    """Return (canonical_code, db_code)."""
    code = lakara.strip()
    if code in CANONICAL_TO_DB:
        return code, CANONICAL_TO_DB[code]
    if code in DB_TO_CANONICAL:
        return DB_TO_CANONICAL[code], code
    return code, code


def lakara_family(db_lakara: str) -> Optional[str]:
    return LAKARA_FAMILIES.get(db_lakara)


def pada_from_lakara(db_lakara: str) -> str:
    if db_lakara == "plit":
        return "P"
    if db_lakara.startswith("a"):
        return "A"
    return "P"


def resolve_pada(db_lakara: str, root_pada: str) -> Optional[str]:
    """Effective pada for this lakāra given root pada restriction."""
    if db_lakara == "plit":
        return "P"
    if db_lakara.startswith("a"):
        return "A"
    if db_lakara.startswith("p"):
        return "P"
    return None
