"""Dump plat paradigms per gana for engine development."""
import json
import sqlite3
from collections import defaultdict

DATA = "sktmorph/data"
SAMPLES = {
    1: "01.0001", 2: "02.0001", 3: "03.0001", 4: "04.0001",
    5: "05.0001", 6: "06.0001", 7: "07.0001", 8: "08.0001",
    9: "09.0001", 10: "10.0001",
}

def shard(did):
    return f"{DATA}/tinantas_shuddha_gana1.sqlite" if did.startswith("01.") else f"{DATA}/tinantas_shuddha_gana2_to_10.sqlite"

for g, did in SAMPLES.items():
    c = sqlite3.connect(shard(did))
    rows = c.execute(
        """SELECT lakara, purusha, vacana, form_slp1 FROM tinantas
           WHERE dhatu_id=? AND prayoga='kartari' AND derivation='shuddha'
           AND lakara IN ('plat','plot','plan','plrt','pvidhiling','alat','alot')
           ORDER BY lakara, purusha, vacana""",
        (did,),
    ).fetchall()
    by_l = defaultdict(list)
    for lk, pu, va, f in rows:
        by_l[lk].append(f)
    print(f"\n=== GANA {g} ({did}) ===")
    for lk in sorted(by_l):
        print(lk, by_l[lk][:3], "...", len(by_l[lk]))
    c.close()
