"""Cross-validate live tinanta engine against bundled SQLite."""
import sqlite3
import sys

DATA = "sktmorph/data"
SAMPLES = {
    1: "01.0001", 2: "02.0001", 3: "03.0001", 4: "04.0001",
    5: "05.0001", 6: "06.0001", 7: "07.0001", 8: "08.0001",
    9: "09.0001", 10: "10.0001",
}
LAKARAS = ["plat", "plot", "plrt", "plan", "pvidhilin"]


def shard(did):
    return f"{DATA}/tinantas_shuddha_gana1.sqlite" if did.startswith("01.") else f"{DATA}/tinantas_shuddha_gana2_to_10.sqlite"


def db_variants_for_cell(rows, pu, va):
    variants = []
    for row_pu, row_va, db_form in rows:
        if row_pu == pu and row_va == va:
            variants.extend(f.strip() for f in db_form.replace(";", ",").split(",") if f.strip())
    return variants


def main():
    from sktmorph.engine.lakara import normalize_lakara
    from sktmorph.engine.tinanta import LiveTinantaEngine

    conn = sqlite3.connect(f"{DATA}/dhatus.sqlite")
    engine = LiveTinantaEngine(conn)
    mismatches = []

    for g, did in SAMPLES.items():
        tc = sqlite3.connect(shard(did))
        for lak in LAKARAS:
            canon, db_lk = normalize_lakara(lak)
            rows = tc.execute(
                """SELECT purusha, vacana, form_slp1 FROM tinantas
                   WHERE dhatu_id=? AND lakara=? AND derivation='shuddha'
                   AND prayoga='kartari' ORDER BY purusha, vacana""",
                (did, db_lk),
            ).fetchall()
            seen = set()
            for pu in (1, 2, 3):
                for va in (1, 2, 3):
                    key = (g, lak, pu, va)
                    if key in seen:
                        continue
                    seen.add(key)
                    db_variants = db_variants_for_cell(rows, pu, va)
                    if not db_variants:
                        continue
                    live_forms, _ = engine.generate_all(did, canon, pu, va)
                    if not any(lf in db_variants for lf in live_forms):
                        mismatches.append((g, lak, pu, va, live_forms, db_variants))
        tc.close()

    conn.close()
    print(f"Mismatches: {len(mismatches)}")
    for m in mismatches[:30]:
        print(m)
    return 1 if mismatches else 0


if __name__ == "__main__":
    raise SystemExit(main())
