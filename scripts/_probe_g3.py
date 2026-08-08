import json
import sqlite3
import sys
import os

sys.path.insert(0, os.path.abspath("."))
from sktmorph.engine.lakara import normalize_lakara

conn = sqlite3.connect("sktmorph/data/dhatus.sqlite")
rows = conn.execute(
    "SELECT dhatu_id, details_json FROM dhatus WHERE json_extract(details_json, '$.gana') = '3' LIMIT 12"
).fetchall()
for did, dj in rows:
    d = json.loads(dj)
    print(did, d["dhatu"], d.get("pada"))

tc = sqlite3.connect("sktmorph/data/tinantas_shuddha_gana2_to_10.sqlite")
_, db = normalize_lakara("plat")
for did, _ in rows[:5]:
    r = tc.execute(
        "SELECT form_slp1 FROM tinantas WHERE dhatu_id=? AND lakara=? AND purusha=1 AND vacana=1 AND derivation='shuddha'",
        (did, db),
    ).fetchone()
    if r:
        print(did, "plat 1/1", r[0])
conn.close()
tc.close()
