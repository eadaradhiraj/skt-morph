"""Build or refresh the bundled taddhitas.sqlite database."""
import os
import sqlite3
import sys

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from sktmorph.taddhita import SEED_ENTRIES, derive_stem_rule, normalize_pratyaya

OUTPUT_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "sktmorph", "data"))
DB_PATH = os.path.join(OUTPUT_DIR, "taddhitas.sqlite")


def build_taddhita_db(db_path: str = DB_PATH) -> int:
    os.makedirs(os.path.dirname(db_path), exist_ok=True)
    if os.path.exists(db_path):
        os.remove(db_path)

    conn = sqlite3.connect(db_path)
    conn.execute(
        """CREATE TABLE taddhitas (
            pratipadika TEXT,
            pratyaya TEXT,
            linga TEXT,
            stem_slp1 TEXT,
            source TEXT
        )"""
    )
    conn.execute("CREATE INDEX idx_taddhita_stem ON taddhitas(stem_slp1)")
    conn.execute("CREATE INDEX idx_taddhita_lookup ON taddhitas(pratipadika, pratyaya, linga)")

    count = 0
    for pratipadika, pratyaya, linga in SEED_ENTRIES:
        canonical = normalize_pratyaya(pratyaya)
        stem = derive_stem_rule(pratipadika, canonical)
        if not stem:
            continue
        conn.execute(
            "INSERT INTO taddhitas VALUES (?, ?, ?, ?, ?)",
            (pratipadika, canonical, linga, stem, "seed"),
        )
        count += 1

    conn.commit()
    conn.close()
    return count


if __name__ == "__main__":
    n = build_taddhita_db()
    print(f"Built {DB_PATH} with {n} seed taddhita entries.")
