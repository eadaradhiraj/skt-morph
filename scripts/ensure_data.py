"""Verify bundled SQLite databases exist; print rebuild instructions if not."""
import glob
import os
import sys

DATA_DIR = os.path.join(os.path.dirname(__file__), "..", "sktmorph", "data")

REQUIRED = [
    "dhatus.sqlite",
    "taddhitas.sqlite",
    "shabdaprakriya.sqlite",
]

OPTIONAL_GLOBS = [
    "tinantas_*.sqlite",
    "krdantas_*.sqlite",
]


def missing_required() -> list:
    return [name for name in REQUIRED if not os.path.exists(os.path.join(DATA_DIR, name))]


def missing_optional_globs() -> dict:
    missing = {}
    for pattern in OPTIONAL_GLOBS:
        matches = glob.glob(os.path.join(DATA_DIR, pattern))
        if not matches:
            missing[pattern] = []
    return missing


def check_data(strict: bool = False) -> int:
    """Return 0 when required DBs exist; 1 otherwise. strict also requires verb DB shards."""
    missing = missing_required()
    if missing:
        print("Missing required databases:", ", ".join(missing))
        print("Rebuild with:")
        print("  python scripts/build_db.py")
        print("  python scripts/ingest_shabda.py")
        return 1

    optional_missing = missing_optional_globs()
    if optional_missing:
        print("Warning: optional verb/participle shards missing:")
        for pattern in optional_missing:
            print(f"  {pattern}")
        print("Verb/krdanta analysis requires:")
        print("  python scripts/build_db.py")
        if strict:
            return 1

    print(f"All required databases present in {os.path.abspath(DATA_DIR)}")
    return 0


def main() -> int:
    strict = "--strict" in sys.argv
    return check_data(strict=strict)


if __name__ == "__main__":
    raise SystemExit(main())
