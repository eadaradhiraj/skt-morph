import unittest
from unittest.mock import patch

from sktmorph import translit


class TestTranslit(unittest.TestCase):
    def test_noop_without_devanagari_flag(self):
        self.assertEqual(translit.maybe_to_slp1("rAmaH", False), "rAmaH")

    def test_has_devanagari_support(self):
        self.assertIsInstance(translit.has_devanagari_support(), bool)

    @patch.object(translit, "_HAS_INDIC", False)
    def test_to_slp1_without_indic(self):
        self.assertEqual(translit.to_slp1("राम"), "राम")

    @patch.object(translit, "_HAS_INDIC", True)
    @patch.object(translit, "_transliterate", return_value="rAma")
    @patch.object(translit, "_looks_like_devanagari", return_value=True)
    def test_to_slp1_with_indic(self, *_mocks):
        self.assertEqual(translit.to_slp1("राम"), "rAma")

    @patch.object(translit, "_HAS_INDIC", True)
    @patch.object(translit, "_transliterate", return_value="राम")
    def test_from_slp1_with_indic(self, _mock):
        self.assertEqual(translit.from_slp1("rAma"), "राम")

    def test_maybe_from_slp1_without_devanagari(self):
        self.assertEqual(translit.maybe_from_slp1("rAmaH", False), "rAmaH")

    def test_import_error_path(self):
        import importlib
        import sys

        module_name = "sktmorph.translit"
        backup = sys.modules.pop(module_name, None)
        try:
            with patch.dict(sys.modules, {"indic_transliteration": None}):
                with patch.dict("sys.modules", {"indic_transliteration.sanscript": None}):
                    mod = importlib.import_module(module_name)
                    importlib.reload(mod)
                    self.assertFalse(mod.has_devanagari_support())
        finally:
            if backup is not None:
                sys.modules[module_name] = backup
            importlib.reload(importlib.import_module(module_name))


if __name__ == "__main__":
    unittest.main()
