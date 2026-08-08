"""Extract ending suffixes by subtracting known stems from DB forms."""
import json
import sqlite3
from collections import defaultdict

DATA = "sktmorph/data"
SAMPLES = {
    1: ("01.0001", "Bava"),
    2: ("02.0001", "ad"),  # guna of ad = ad? ad stays
    4: ("04.0001", "dIvya"),
    5: ("05.0001", "suno"),
    6: ("06.0001", "tuda"),
    7: ("07.0001", "ruRa"),  # tentative
    8: ("08.0001", "tano"),
    9: ("09.0001", "krInA"),  # tentative
    10: ("10.0001", "coraya"),
}

LAKARAS = ["plat", "plot", "pvidhiling", "plang", "plrt", "alat", "alot", "alang"]

def shard(did):
    return f"{DATA}/tinantas_shuddha_gana1.sqlite" if did.startswith("01.") else f"{DATA}/tinantas_shuddha_gana2_to_10.sqlite"

for g, (did, stem) in SAMPLES.items():
    c = sqlite3.connect(shard(did))
    print(f"\n=== GANA {g} stem={stem} ===")
    for lk in LAKARAS:
        rows = c.execute(
            """SELECT purusha, vacana, form_slp1 FROM tinantas
               WHERE dhatu_id=? AND lakara=? AND derivation='shuddha' AND prayoga='kartari'
               ORDER BY purusha, vacana""",
            (did, lk),
        ).fetchall()
        if not rows:
            continue
        suffixes = []
        for pu, va, form in rows:
            if form.startswith(stem):
                suffixes.append(form[len(stem):])
            else:
                suffixes.append(f"?({form})")
        print(lk, suffixes[:9])
    c.close()
