"""Analyze tinanta/krdanta DB patterns for live engine tables."""
import json
import sqlite3
from collections import defaultdict

DATA = "sktmorph/data"

def main():
    t = sqlite3.connect(f"{DATA}/tinantas_shuddha_gana1.sqlite")
    lakar = [r[0] for r in t.execute("SELECT DISTINCT lakara FROM tinantas ORDER BY lakara")]
    print("LAKARAS", lakar)

    d = sqlite3.connect(f"{DATA}/dhatus.sqlite")
    rows = d.execute("SELECT dhatu_id, details_json FROM dhatus").fetchall()
    by_gana = defaultdict(list)
    for did, js in rows:
        det = json.loads(js)
        by_gana[int(det.get("gana", 0))].append((did, det.get("dhatu"), det.get("pada")))

    for g in sorted(by_gana):
        sample = by_gana[g][0]
        did = sample[0]
        shard = "tinantas_shuddha_gana1.sqlite" if did.startswith("01.") else "tinantas_shuddha_gana2_to_10.sqlite"
        tc = sqlite3.connect(f"{DATA}/{shard}")
        form = tc.execute(
            "SELECT form_slp1 FROM tinantas WHERE dhatu_id=? AND lakara='plat' AND purusha=1 AND vacana=1 AND prayoga='kartari' AND derivation='shuddha'",
            (did,),
        ).fetchone()
        print(f"gana {g}: {sample[1]} ({sample[2]}) -> {form[0] if form else 'MISSING'}")
        tc.close()

    k = sqlite3.connect(f"{DATA}/krdantas_gana1.sqlite")
    praty = [r[0] for r in k.execute("SELECT DISTINCT pratyaya FROM krdantas ORDER BY pratyaya")]
    print("KRDANTA pratyayas", len(praty), praty[:40])

if __name__ == "__main__":
    main()
