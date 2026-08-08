import os, sys
sys.path.insert(0, os.path.abspath("."))
from scripts.validate_engine import LAKARAS, shard, DATA, db_variants_for_cell
from sktmorph.engine.lakara import normalize_lakara
from sktmorph.engine.tinanta import LiveTinantaEngine
import sqlite3

def check_gana(g, did):
    conn = sqlite3.connect(f"{DATA}/dhatus.sqlite")
    engine = LiveTinantaEngine(conn)
    tc = sqlite3.connect(shard(did))
    n = 0
    for lak in LAKARAS:
        canon, db_lk = normalize_lakara(lak)
        rows = tc.execute(
            """SELECT purusha, vacana, form_slp1 FROM tinantas
               WHERE dhatu_id=? AND lakara=? AND derivation='shuddha'
               AND prayoga='kartari' ORDER BY purusha, vacana""",
            (did, db_lk),
        ).fetchall()
        for pu in (1, 2, 3):
            for va in (1, 2, 3):
                db_variants = db_variants_for_cell(rows, pu, va)
                if not db_variants:
                    continue
                live_forms, _ = engine.generate_all(did, canon, pu, va)
                if not any(lf in db_variants for lf in live_forms):
                    n += 1
                    if n <= 5:
                        print(f"g{g} {lak} {pu}/{va}", live_forms, db_variants)
    tc.close()
    conn.close()
    print(f"gana {g}: {n} mismatches")

for g, did in [(4,"04.0001"),(6,"06.0001"),(10,"10.0001"),(2,"02.0001"),(5,"05.0001"),(7,"07.0001"),(8,"08.0001"),(9,"09.0001")]:
    check_gana(g, did)
