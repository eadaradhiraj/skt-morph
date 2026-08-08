"""Validate live krdanta engine against bundled SQLite shards."""
import glob
import json
import sqlite3
import sys

sys.path.insert(0, ".")
from sktmorph.engine.krdanta import LiveKrdantaEngine

DATA = "sktmorph/data"


def main() -> int:
    conn = sqlite3.connect(f"{DATA}/dhatus.sqlite")
    engine = LiveKrdantaEngine(conn)
    mismatches = []
    checked = 0
    supported = 0

    for path in sorted(glob.glob(f"{DATA}/krdantas_*.sqlite")):
        tc = sqlite3.connect(path)
        rows = tc.execute(
            """SELECT dhatu_id, pratyaya, form_slp1 FROM krdantas
               WHERE derivation='shuddha' GROUP BY dhatu_id, pratyaya"""
        ).fetchall()
        for did, pratyaya, db_form in rows:
            checked += 1
            live_forms, _ = engine.generate_all(did, pratyaya)
            if not live_forms:
                continue
            supported += 1
            db_variants = [f.strip() for f in db_form.replace(";", ",").split(",") if f.strip()]
            if not any(lf in db_variants for lf in live_forms):
                details = json.loads(
                    conn.execute(
                        "SELECT details_json FROM dhatus WHERE dhatu_id=?", (did,)
                    ).fetchone()[0]
                )
                mismatches.append((did, details.get("dhatu"), pratyaya, live_forms, db_variants[:2]))
        tc.close()

    conn.close()
    print(f"Checked: {checked}  Live-supported: {supported}  Mismatches: {len(mismatches)}")
    for m in mismatches[:30]:
        print(m)
    return 1 if mismatches else 0


if __name__ == "__main__":
    raise SystemExit(main())
