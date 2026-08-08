import sqlite3
import unittest

from sktmorph.engine.redup import (
    gana3_future_stem,
    gana3_join_mode,
    gana3_lang_stem,
    gana3_present_stem,
    gana3_vidhilin_stem,
)
from sktmorph.engine.tinanta import LiveTinantaEngine


class TestGana3Redup(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.engine = LiveTinantaEngine(sqlite3.connect("sktmorph/data/dhatus.sqlite"))

    def test_hu_profiles(self):
        self.assertEqual(gana3_present_stem("hu"), "juhu")
        self.assertEqual(gana3_join_mode("hu"), "nu")
        self.assertEqual(gana3_lang_stem("hu"), "juh")
        self.assertEqual(gana3_vidhilin_stem("hu"), "juhuy")
        self.assertEqual(gana3_future_stem("hu"), "hozya")

    def test_hu_live_primary_lakaras(self):
        for lak in ("plat", "plot", "plrt", "plan", "pvidhilin"):
            form, _ = self.engine.generate("03.0001", lak, 1, 1)
            self.assertTrue(form)

    def test_hu_plit(self):
        form, _ = self.engine.generate("03.0001", "plit", 1, 1)
        self.assertEqual(form, "juhAva")

    def test_ma_alat(self):
        form, _ = self.engine.generate("03.0007", "alat", 1, 1)
        self.assertEqual(form, "mimIte")


if __name__ == "__main__":
    unittest.main()
