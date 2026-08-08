import sqlite3
c = sqlite3.connect("sktmorph/data/tinantas_shuddha_gana2_to_10.sqlite")
for did in ["04.0001", "05.0001", "07.0001", "02.0001"]:
    row = c.execute(
        """SELECT form_slp1 FROM tinantas WHERE dhatu_id=? AND lakara='plrut'
           AND purusha=1 AND vacana=1 AND derivation='shuddha' AND prayoga='kartari'""",
        (did,),
    ).fetchone()
    print(did, row[0] if row else None)
