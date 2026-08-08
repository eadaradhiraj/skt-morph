import sys
sys.path.insert(0, ".")
from sktmorph.engine.tinanta import LiveTinantaEngine
from sktmorph.engine.stems import derive_stem
from sktmorph.engine.join import join_form
import sqlite3

conn = sqlite3.connect("sktmorph/data/dhatus.sqlite")
engine = LiveTinantaEngine(conn)
info = engine.load_dhatu("09.0001")
print(info)
stem, aug, steps = derive_stem(info["dhatu"], info["gana"], "lot", "shuddha")
print("stem", stem, "gana", info["gana"])
print("join Atu", join_form(stem, "Atu", 9, "lot", 1, "P", aug))
forms, _ = engine.generate_all("09.0001", "plot", 1, 1)
print("live", forms)
conn.close()
