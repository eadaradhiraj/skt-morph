import json
import os
import sqlite3
import tempfile
import unittest

from sktmorph.shabda_db import ShabdaPrakriyaStore, VIBHAKTI_NUM_TO_NAME


class TestShabdaDb(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.NamedTemporaryFile(suffix=".sqlite", delete=False)
        self.tmp.close()
        conn = sqlite3.connect(self.tmp.name)
        conn.execute(
            """CREATE TABLE form_prakriya (
                form_slp1 TEXT, word_slp1 TEXT, vibhakti TEXT, vacana INTEGER, steps_json TEXT
            )"""
        )
        conn.execute(
            "INSERT INTO form_prakriya VALUES (?, ?, ?, ?, ?)",
            ("aH", "a", "prathamA", 1, json.dumps([{"step": "a + s"}])),
        )
        conn.commit()
        conn.close()
        self.store = ShabdaPrakriyaStore(self.tmp.name)

    def tearDown(self):
        self.store.close()
        os.unlink(self.tmp.name)

    def test_lookup_form(self):
        rows = self.store.lookup_form("aH")
        self.assertEqual(len(rows), 1)
        self.assertEqual(rows[0]["word_slp1"], "a")
        self.assertEqual(rows[0]["steps"][0]["step"], "a + s")

    def test_lookup_missing_form(self):
        self.assertEqual(self.store.lookup_form("missing"), [])

    def test_missing_db_returns_empty(self):
        store = ShabdaPrakriyaStore(os.path.join(tempfile.gettempdir(), "nonexistent_shabda.sqlite"))
        self.assertEqual(store.lookup_form("aH"), [])
        store.close()

    def test_default_db_path(self):
        store = ShabdaPrakriyaStore()
        self.assertTrue(store.db_path.endswith("shabdaprakriya.sqlite"))
        store.close()

    def test_vibhakti_map(self):
        self.assertEqual(VIBHAKTI_NUM_TO_NAME["1"], "prathamA")


if __name__ == "__main__":
    unittest.main()
