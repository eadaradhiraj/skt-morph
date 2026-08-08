import sqlite3
c = sqlite3.connect("sktmorph/data/tinantas_shuddha_gana1.sqlite")
for lk in [
    "plrt", "plrut", "plrung", "plut", "plung", "plit",
    "alrt", "alrut", "alit", "aashirling", "avidhiling", "pashirling",
    "plang", "alang", "pvidhiling",
]:
    n = c.execute("SELECT COUNT(*) FROM tinantas WHERE lakara=?", (lk,)).fetchone()[0]
    if not n:
        continue
    row = c.execute(
        """SELECT form_slp1 FROM tinantas WHERE dhatu_id='01.0001'
           AND lakara=? AND purusha=1 AND vacana=1
           AND derivation='shuddha' AND prayoga='kartari'""",
        (lk,),
    ).fetchone()
    print(lk, n, "BU:", row[0] if row else "-")
