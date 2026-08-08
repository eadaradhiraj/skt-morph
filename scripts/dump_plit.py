import sqlite3
c = sqlite3.connect("sktmorph/data/tinantas_shuddha_gana1.sqlite")
rows = c.execute(
    """SELECT purusha, vacana, form_slp1 FROM tinantas
       WHERE dhatu_id='01.0001' AND lakara='plit' AND derivation='shuddha'
       AND prayoga='kartari' ORDER BY purusha, vacana"""
).fetchall()
for r in rows:
    print(r)
