import unittest

from sktmorph.subanta import SubantaGenerator


class TestSubantaExpandedParadigms(unittest.TestCase):
    def setUp(self):
        self.gen = SubantaGenerator()

    def test_long_u_stem_pum(self):
        table = self.gen.generate("vApU", "pum")
        self.assertEqual(table["prathamA"][0], "vApUH")

    def test_us_stem_nap(self):
        table = self.gen.generate("manus", "nap")
        self.assertIn("uH", table["prathamA"][0])

    def test_is_stem_nap(self):
        table = self.gen.generate("manis", "nap")
        self.assertIn("iH", table["prathamA"][0])

    def test_generate_with_prakriya(self):
        detail = self.gen.generate_with_prakriya("rAma", "pum")
        self.assertIn("prakriya", detail)
        self.assertEqual(len(detail["prakriya"]), 16)

    def test_long_i_stem_pum(self):
        table = self.gen.generate("karI", "pum")
        self.assertEqual(table["prathamA"], ["karI", "karyO", "karyaH"])


if __name__ == "__main__":
    unittest.main()
