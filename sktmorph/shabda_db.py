"""Load shabda/prakriya SQLite databases built from ashtadhyayi.com data."""
import json
import os
import sqlite3
from typing import Any, Dict, List, Optional

VIBHAKTI_NUM_TO_NAME = {
    "1": "prathamA",
    "2": "dvitIyA",
    "3": "tfIyA",
    "4": "caturTI",
    "5": "paYcamI",
    "6": "zazWI",
    "7": "saptamI",
    "8": "samboDana",
}


class ShabdaPrakriyaStore:
    def __init__(self, db_path: str = None):
        if db_path is None:
            db_path = os.path.join(os.path.dirname(__file__), "data", "shabdaprakriya.sqlite")
        self.db_path = db_path
        self._conn = None
        if os.path.exists(db_path):
            self._conn = sqlite3.connect(db_path)
            self._conn.row_factory = sqlite3.Row

    def lookup_form(self, form_slp1: str) -> List[Dict[str, Any]]:
        if not self._conn:
            return []
        cursor = self._conn.cursor()
        cursor.execute(
            "SELECT word_slp1, vibhakti, vacana, steps_json FROM form_prakriya WHERE form_slp1 = ?",
            (form_slp1,),
        )
        rows = []
        for r in cursor.fetchall():
            rows.append(
                {
                    "word_slp1": r["word_slp1"],
                    "vibhakti": r["vibhakti"],
                    "vacana": int(r["vacana"]),
                    "steps": json.loads(r["steps_json"]),
                }
            )
        return rows

    def close(self):
        if self._conn:
            self._conn.close()
            self._conn = None
