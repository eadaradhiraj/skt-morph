import json
import sqlite3
import sys

sys.path.insert(0, ".")
from scripts.validate_engine import LAKARAS, db_variants_for_cell, shard
from sktmorph.engine.lakara import kartari_compatible
from sktmorph.engine.tinanta import LiveTinantaEngine


def exercise_corpus() -> None:
    conn = sqlite3.connect("sktmorph/data/dhatus.sqlite")
    engine = LiveTinantaEngine(conn)
    for did, details_json in conn.execute("SELECT dhatu_id, details_json FROM dhatus").fetchall():
        details = json.loads(details_json)
        pada = details.get("pada") or "P"
        for lak in LAKARAS:
            if not kartari_compatible(pada, lak):
                continue
            for pu in (1, 2, 3):
                for va in (1, 2, 3):
                    engine.generate_all(did, lak, pu, va)


if __name__ == "__main__":
    exercise_corpus()
