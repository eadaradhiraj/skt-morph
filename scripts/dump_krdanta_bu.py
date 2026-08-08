"""Extract krdanta patterns for gana 1 BU."""
import sqlite3
DATA = "sktmorph/data"
c = sqlite3.connect(f"{DATA}/krdantas_gana1.sqlite")
rows = c.execute(
    "SELECT pratyaya, form_slp1 FROM krdantas WHERE dhatu_id='01.0001' AND derivation='shuddha' ORDER BY pratyaya"
).fetchall()
for p, f in rows:
    print(p, "->", f)
