import os, sys
sys.path.insert(0, os.path.abspath("."))
from scripts.validate_engine import main, SAMPLES, LAKARAS, shard, DATA
from sktmorph.engine.lakara import normalize_lakara
from sktmorph.engine.tinanta import LiveTinantaEngine
import sqlite3

conn = sqlite3.connect(f"{DATA}/dhatus.sqlite")
engine = LiveTinantaEngine(conn)
did = "01.0001"
tc = sqlite3.connect(shard(did))
for lak in LAKARAS:
    canon, db_lk = normalize_lakara(lak)
    rows = tc.execute(
        """SELECT purusha, vacana, form_slp1 FROM tinantas
           WHERE dhatu_id=? AND lakara=? AND derivation='shuddha'
           AND prayoga='kartari' ORDER BY purusha, vacana""",
        (did, db_lk),
    ).fetchall()
    for pu, va, db_form in rows:
        live_forms, _ = engine.generate_all(did, canon, pu, va)
        db_variants = [f.strip() for f in db_form.replace(";", ",").split(",")]
        if not any(lf in db_variants for lf in live_forms):
            print(lak, pu, va, live_forms, db_variants)
tc.close()
