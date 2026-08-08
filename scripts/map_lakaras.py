"""Map canonical lakara codes to DB codes and sample stems."""
import sqlite3

DATA = "sktmorph/data"
CANONICAL = {
    "plat": "plat",
    "plrt": "plrut",
    "plot": "plot",
    "plan": "plang",
    "pvidhilin": "pvidhiling",
    "alat": "alat",
    "alrt": "alrut",
    "alot": "alot",
    "alan": "alang",
    "aling": "aashirling",  # also pashirling for P
    "alit": "alit",
}

def shard(did):
    return f"{DATA}/tinantas_shuddha_gana1.sqlite" if did.startswith("01.") else f"{DATA}/tinantas_shuddha_gana2_to_10.sqlite"

# gana 1 BU plat plot plang pvidhiling plrut
did = "01.0001"
c = sqlite3.connect(shard(did))
for canon, db in CANONICAL.items():
    rows = c.execute(
        """SELECT purusha, vacana, form_slp1 FROM tinantas
           WHERE dhatu_id=? AND lakara=? AND derivation='shuddha' AND prayoga='kartari'
           ORDER BY purusha, vacana""",
        (did, db),
    ).fetchall()
    if rows:
        print(canon, "->", db, [r[2] for r in rows[:3]], "...")
    else:
        print(canon, "->", db, "NONE")

# Also pashirling for P benedictive
rows = c.execute(
    """SELECT form_slp1 FROM tinantas WHERE dhatu_id=? AND lakara='pashirling'
       AND purusha=1 AND vacana=1 AND derivation='shuddha' AND prayoga='kartari'""",
    (did,),
).fetchall()
print("pashirling (P benedictive):", [r[0] for r in rows])

c.close()
