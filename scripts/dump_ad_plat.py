import sqlite3
c = sqlite3.connect("sktmorph/data/tinantas_shuddha_gana2_to_10.sqlite")
rows = c.execute(
    """SELECT purusha, vacana, form_slp1 FROM tinantas
       WHERE dhatu_id='02.0001' AND lakara='plat' AND derivation='shuddha'
       AND prayoga='kartari' ORDER BY purusha, vacana"""
).fetchall()
for r in rows:
    print(r)
