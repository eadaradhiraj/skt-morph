"""Validate live tinanta engine against all bundled dhatus (shuddha kartari)."""
import json
import sqlite3
import sys

sys.path.insert(0, ".")
from scripts.validate_engine import LAKARAS, DATA, db_variants_for_cell, shard
from sktmorph.engine.lakara import kartari_compatible, normalize_lakara
from sktmorph.engine.tinanta import LiveTinantaEngine

EXTENDED_LAKARAS = LAKARAS + ["plit", "alat", "alot", "alang", "alit"]


def main() -> int:
    lakaras = EXTENDED_LAKARAS if "--extended" in sys.argv else LAKARAS
    conn = sqlite3.connect(f"{DATA}/dhatus.sqlite")
    engine = LiveTinantaEngine(conn)
    rows = conn.execute("SELECT dhatu_id, details_json FROM dhatus ORDER BY dhatu_id").fetchall()
    total_cells = 0
    mismatches = []

    for did, details_json in rows:
        details = json.loads(details_json)
        pada = details.get("pada") or "P"
        tc = sqlite3.connect(shard(did))
        for lak in lakaras:
            if not kartari_compatible(pada, lak):
                continue
            canon, db_lk = normalize_lakara(lak)
            trows = tc.execute(
                """SELECT purusha, vacana, form_slp1 FROM tinantas
                   WHERE dhatu_id=? AND lakara=? AND derivation='shuddha'
                   AND prayoga='kartari' ORDER BY purusha, vacana""",
                (did, db_lk),
            ).fetchall()
            for pu in (1, 2, 3):
                for va in (1, 2, 3):
                    db_variants = db_variants_for_cell(trows, pu, va)
                    if not db_variants:
                        continue
                    total_cells += 1
                    live_forms, _ = engine.generate_all(did, canon, pu, va)
                    if not any(lf in db_variants for lf in live_forms):
                        mismatches.append(
                            (did, details.get("dhatu"), lak, pu, va, live_forms, db_variants[:3])
                        )
        tc.close()

    conn.close()
    matched = total_cells - len(mismatches)
    pct = (100.0 * matched / total_cells) if total_cells else 0.0
    print(f"Cells: {total_cells}  Matched: {matched}  ({pct:.1f}%)  Mismatches: {len(mismatches)}")
    for m in mismatches[:25]:
        print(m)
    return 1 if mismatches else 0


if __name__ == "__main__":
    raise SystemExit(main())
