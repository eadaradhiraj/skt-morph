import unittest

from sktmorph import translit


@unittest.skipUnless(translit.has_devanagari_support(), "indic-transliteration not installed")
class TestTranslitWithIndic(unittest.TestCase):
    def test_round_trip_devanagari(self):
        slp1 = translit.to_slp1("राम")
        self.assertEqual(slp1, "rAma")
        self.assertEqual(translit.from_slp1("rAma"), "राम")

    def test_to_slp1_non_devanagari_passthrough(self):
        self.assertEqual(translit.to_slp1("rAma"), "rAma")

    def test_empty_strings(self):
        self.assertEqual(translit.to_slp1(""), "")
        self.assertEqual(translit.from_slp1(""), "")

    def test_maybe_helpers(self):
        self.assertEqual(translit.maybe_from_slp1("rAma", True), "राम")
        self.assertEqual(translit.maybe_to_slp1("राम", True), "rAma")


if __name__ == "__main__":
    unittest.main()
