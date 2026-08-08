import sqlite3
import unittest

from sktmorph.morphology import SktMorph


class TestLiveKrdantaIntegration(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.morph = SktMorph()

    def test_live_krdanta_satf(self):
        forms = self.morph.generate_krdanta("BU", "Satf", live=True)
        self.assertIn("Bavat", forms)

    def test_live_krdanta_fallback(self):
        forms = self.morph.generate_krdanta("BU", "Satf", live=False)
        self.assertTrue(forms)

    def test_live_krdanta_unknown_pratyaya_falls_back(self):
        forms = self.morph.generate_krdanta("BU", "not-a-pratyaya-xyz", live=True)
        self.assertEqual(forms, [])

    def test_tinanta_lakara_alias(self):
        forms = self.morph.generate_tinanta("BU", "plan", 1, 1, live=True)
        self.assertIn("aBavat", forms)


if __name__ == "__main__":
    unittest.main()
