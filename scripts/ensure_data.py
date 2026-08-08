"""Verify bundled SQLite databases exist; print rebuild instructions if not."""
import os
import sys

DATA_DIR = os.path.join(os.path.dirname(__file__), "..", "sktmorph", "data")
REQUIRED = [
    "dhatus.sqlite",
    "taddhitas.sqlite",
    "shabdaprakriya.sqlite",
]
OPTIONAL_GLOBS = ["tinantas_*.sqlite", "krdantas_*.sqlite"]


def main() -> int:
    missing = [name for name in REQUIRED if not os.path.exists(os.path.join(DATA_DIR, name))]
    if missing:
        print("Missing required databases:", ", ".join(missing))
        print("Rebuild with:")
        print("  python scripts/build_db.py")
        print("  python scripts/ingest_shabda.py")
        return 1
    print(f"All required databases present in {os.path.abspath(DATA_DIR)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
