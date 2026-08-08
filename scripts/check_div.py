import sqlite3, json
r = sqlite3.connect("sktmorph/data/dhatus.sqlite").execute(
    "SELECT details_json FROM dhatus WHERE dhatu_id='04.0001'"
).fetchone()
print(json.loads(r[0])["dhatu"])
