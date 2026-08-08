import json
import os
import sqlite3
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from scripts import ingest_shabda as ingest


class TestIngestShabda(unittest.TestCase):
    def setUp(self):
        self.tmpdir = tempfile.mkdtemp()
        self.orig_output = ingest.OUTPUT_DIR
        self.orig_taddhita = ingest.TADDHITA_DB
        self.orig_prakriya = ingest.PRAKRIYA_DB
        ingest.OUTPUT_DIR = self.tmpdir
        ingest.TADDHITA_DB = os.path.join(self.tmpdir, "taddhitas.sqlite")
        ingest.PRAKRIYA_DB = os.path.join(self.tmpdir, "shabdaprakriya.sqlite")

    def tearDown(self):
        ingest.OUTPUT_DIR = self.orig_output
        ingest.TADDHITA_DB = self.orig_taddhita
        ingest.PRAKRIYA_DB = self.orig_prakriya
        for name in os.listdir(self.tmpdir):
            os.unlink(os.path.join(self.tmpdir, name))
        os.rmdir(self.tmpdir)

    def test_map_linga_and_to_slp1_fallback(self):
        self.assertEqual(ingest.map_linga("P"), "pum")
        with patch.object(ingest, "transliterate", None):
            self.assertEqual(ingest.to_slp1("abc"), "abc")
        self.assertEqual(ingest.to_slp1(""), "")

    def test_extract_taddhita_rows(self):
        items = [
            {"word": "राम", "linga": "P"},
            {"word": "रामत्व", "linga": "N"},
        ]
        rows = ingest.extract_taddhita_rows(items)
        self.assertTrue(any(r[4] == "data2" for r in rows))

    def test_extract_vyutpatti_rows(self):
        items = [
            {"word": "base", "linga": "P"},
            {"word": "bad", "linga": "P"},
            {
                "word": "good",
                "linga": "P",
                "vyutpatti": "matup [[5.2.94]] taddhita derivation",
            },
        ]

        def fake_slp1(text):
            mapping = {"base": "rAma", "bad": "notataddhita", "good": "rAmavat"}
            return mapping.get(text, text)

        with patch.object(ingest, "to_slp1", side_effect=fake_slp1):
            rows = ingest.extract_taddhita_rows(items)
        self.assertTrue(any(r[0] == "rAma" and r[1] == "matup" for r in rows))

    def test_extract_heuristic_rows(self):
        word_set = {"rAma", "rAmAya", "rAmatva"}
        rows = ingest.extract_taddhita_heuristic_rows(word_set)
        pratyayas = {r[1] for r in rows if r[0] == "rAma"}
        self.assertIn("Tya", pratyayas)
        self.assertIn("tva", pratyayas)

    def test_build_taddhita_db(self):
        items = [
            {"word": "राम", "linga": "P"},
            {"word": "रामत्व", "linga": "N"},
        ]
        count = ingest.build_taddhita_db(items, seed_rows=[("rAma", "tva", "nap")])
        self.assertGreater(count, 0)
        conn = sqlite3.connect(ingest.TADDHITA_DB)
        row = conn.execute("SELECT COUNT(*) FROM taddhitas").fetchone()[0]
        conn.close()
        self.assertEqual(row, count)

    def test_build_prakriya_db(self):
        items = [
            {
                "form": "अः",
                "word": "अ",
                "vibhakti": "1",
                "vachan": 1,
                "steps": [{"step": "a + s", "sutras": ["1.3.2"]}],
            }
        ]
        count = ingest.build_prakriya_db(items)
        self.assertEqual(count, 1)
        conn = sqlite3.connect(ingest.PRAKRIYA_DB)
        row = conn.execute("SELECT form_slp1 FROM form_prakriya").fetchone()[0]
        conn.close()
        self.assertEqual(row, "aH")

    def test_build_prakriya_db_skips_incomplete(self):
        count = ingest.build_prakriya_db([{"form": "", "word": "x"}])
        self.assertEqual(count, 0)

    def test_ingest_all_without_download(self):
        data2 = {"data": [{"word": "राम", "linga": "P"}, {"word": "रामत्व", "linga": "N"}]}
        prakriya = [{"form": "अः", "word": "अ", "vibhakti": "1", "vachan": 1, "steps": []}]
        stats = ingest.ingest_all(use_download=False, data2_items=data2["data"], prakriya_items=prakriya)
        self.assertIn("taddhitas", stats)
        self.assertIn("prakriya_forms", stats)

    def test_download_uses_cached_file(self):
        os.makedirs(ingest.DATA_RAW, exist_ok=True)
        cached = os.path.join(ingest.DATA_RAW, "data2.txt")
        with open(cached, "w", encoding="utf-8") as f:
            json.dump({"data": []}, f)
        path = ingest.download_shabda_file("data2.txt")
        self.assertEqual(path, cached)


if __name__ == "__main__":
    unittest.main()
