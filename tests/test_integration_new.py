import json
import os
import sqlite3
import tempfile
import unittest
from unittest.mock import patch

from sktmorph.morphology import SktMorph
from sktmorph.subanta import PARADIGMS, SubantaGenerator


class TestSubantaAgentStem(unittest.TestCase):
    def test_datA_normalizes_to_datf(self):
        gen = SubantaGenerator()
        table = gen.generate("DAtA", "pum")
        self.assertEqual(table["prathamA"][0], "DAtA")

    def test_non_agent_a_stem_unchanged(self):
        gen = SubantaGenerator()
        table = gen.generate("rAma", "pum")
        self.assertEqual(table["prathamA"][0], "rAmaH")

    def test_normalize_keeps_stem_when_no_matching_f_paradigm(self):
        gen = SubantaGenerator()
        with patch.object(gen, "supported_endings", ["in"]):
            with patch.dict(PARADIGMS, {("in", "pum"): PARADIGMS[("in", "pum")]}, clear=True):
                self.assertEqual(gen._normalize_pratipadika("DAtA", "pum"), "DAtA")


class TestMorphologyPrakriyaAndRanking(unittest.TestCase):
    def test_analyze_attaches_shabda_prakriya(self):
        tmp = tempfile.NamedTemporaryFile(suffix=".sqlite", delete=False)
        tmp.close()
        db_path = tmp.name
        conn = sqlite3.connect(db_path)
        conn.execute(
            """CREATE TABLE form_prakriya (
                form_slp1 TEXT, word_slp1 TEXT, vibhakti TEXT, vacana INTEGER, steps_json TEXT
            )"""
        )
        conn.execute(
            "INSERT INTO form_prakriya VALUES (?, ?, ?, ?, ?)",
            ("rAmaH", "rAma", "prathamA", 1, json.dumps([{"step": "rAma + H"}])),
        )
        conn.commit()
        conn.close()

        morph = SktMorph()
        morph._shabda = morph._shabda.__class__(db_path)
        try:
            results = morph.analyze("rAmaH", include_prakriya=True)
            with_prakriya = [r for r in results if r.prakriya]
            self.assertTrue(with_prakriya)
            self.assertEqual(with_prakriya[0].prakriya[0]["step"], "rAma + H")
        finally:
            morph._shabda.close()
            os.unlink(db_path)

    def test_analyze_without_prakriya_by_default(self):
        morph = SktMorph()
        results = morph.analyze("rAmaH", include_prakriya=False)
        self.assertTrue(all(r.prakriya is None for r in results))

    def test_generate_subanta_with_prakriya(self):
        morph = SktMorph()
        result = morph.generate_subanta("rAma", "pum", include_prakriya=True)
        self.assertIn("prakriya", result)
        self.assertIn("declension", result)
        self.assertEqual(result["declension"]["prathamA"][0], "rAmaH")


if __name__ == "__main__":
    unittest.main()
