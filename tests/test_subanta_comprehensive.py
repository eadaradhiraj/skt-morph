import unittest

from sktmorph.subanta import SubantaGenerator, apply_natva

VIBHAKTIS = [
    "prathamA", "dvitIyA", "tfIyA", "caturTI",
    "paYcamI", "zazWI", "saptamI", "samboDana",
]

# (pratipadika, linga, prathama, dvitiya, tritiya)
AJANTA_CASES = [
    ("hari", "pum", ["hariH", "harI", "harayaH"], ["harim", "harI", "harIn"], ["hariRA", "hariByAm", "hariBiH"]),
    ("PalA", "stri", ["PalA", "Pale", "PalAH"], ["PalAm", "Pale", "PalAH"], ["PalayA", "PalAByAm", "PalABiH"]),
    ("Pala", "nap", ["Palam", "Pale", "PalAni"], ["Palam", "Pale", "PalAni"], ["Palena", "PalAByAm", "PalEH"]),
    ("nadI", "stri", ["nadI", "nadyO", "nadyaH"], ["nadIm", "nadyO", "nadIH"], ["nadyA", "nadIByAm", "nadIBiH"]),
    ("madhu", "pum", ["madhuH", "madhU", "madhavaH"], ["madhum", "madhU", "madhUn"], ["madhunA", "madhuByAm", "madhuBiH"]),
    ("kartf", "pum", ["kartA", "kartarO", "kartaraH"], ["kartaram", "kartarO", "kartFn"], ["kartrA", "kartfByAm", "kartfBiH"]),
]

HALANTA_CASES = [
    ("guRin", "pum", ["guRI", "guRinO", "guRinaH"], ["guRinA", "guRiByAm", "guRiBiH"]),
    ("manas", "nap", ["manaH", "manasI", "manAMsi"], ["manasA", "manoByAm", "manoBiH"]),
    ("Bavat", "pum", ["BavAn", "BavantO", "BavantaH"], ["BavatA", "BavadByAm", "BavadBiH"]),
    ("rAjan", "pum", ["rAjA", "rAjAnO", "rAjAnaH"], ["rAjYA", "rAjaByAm", "rAjaBiH"]),
    ("vAc", "stri", ["vAg", "vAcaH", "vAcaH"], ["vAcA", "vAgByAm", "vAgBiH"]),
    ("pad", "nap", ["pad", "padI", "pAmSi"], ["padA", "paByAm", "paBiH"]),
    ("Siz", "pum", ["SiH", "SiqO", "SiqaH"], ["SiqA", "SiByAm", "SiBiH"]),
    ("jagat", "nap", ["jagat", "jagatI", "jagAMsi"], ["jagatA", "jagByAm", "jagBiH"]),
]

ANALYZE_CASES = [
    ("rAmeRa", "rAma", "pum", "tfIyA", 1),
    ("BavadBiH", "Bavat", "pum", "tfIyA", 3),
    ("manoByAm", "manas", "nap", "caturTI", 2),
    ("kariRAm", "karin", "pum", "zazWI", 3),
    ("rAjYA", "rAjan", "pum", "tfIyA", 1),
    ("vAg", "vAc", "stri", "prathamA", 1),
    ("pAmSi", "pad", "nap", "prathamA", 3),
    ("SiqO", "Siz", "pum", "prathamA", 2),
    ("jagatI", "jagat", "nap", "prathamA", 2),
    ("nadyAm", "nadI", "stri", "saptamI", 1),
]


class TestNatvaComprehensive(unittest.TestCase):
    def test_natva_after_r(self):
        self.assertEqual(apply_natva("rAm", "ena"), "eRa")

    def test_natva_no_trigger(self):
        self.assertEqual(apply_natva("dev", "ena"), "ena")

    def test_natva_final_n_untouched(self):
        self.assertEqual(apply_natva("rAm", "An"), "An")

    def test_natva_rt_stem(self):
        self.assertEqual(apply_natva("dAt", "fnA"), "fRA")

    def test_natva_blocker_resets(self):
        self.assertEqual(apply_natva("pit", "FnAm"), "FRAm")


class TestAjantaGeneration(unittest.TestCase):
    def setUp(self):
        self.gen = SubantaGenerator()

    def test_ajanta_paradigms(self):
        for prati, linga, prathama, dvitiya, tritiya in AJANTA_CASES:
            with self.subTest(pratipadika=prati, linga=linga):
                table = self.gen.generate(prati, linga)
                self.assertEqual(table["prathamA"], prathama)
                self.assertEqual(table["dvitIyA"], dvitiya)
                self.assertEqual(table["tfIyA"], tritiya)

    def test_all_vibhaktis_present(self):
        table = self.gen.generate("hari", "pum")
        for v in VIBHAKTIS:
            self.assertIn(v, table)
            self.assertEqual(len(table[v]), 3)


class TestHalantaGenerationComprehensive(unittest.TestCase):
    def setUp(self):
        self.gen = SubantaGenerator()

    def test_halanta_paradigms(self):
        for prati, linga, prathama, tritiya_row in HALANTA_CASES:
            with self.subTest(pratipadika=prati, linga=linga):
                table = self.gen.generate(prati, linga)
                self.assertEqual(table["prathamA"], prathama)
                self.assertEqual(table["tfIyA"], tritiya_row)


class TestSubantaAnalyzeComprehensive(unittest.TestCase):
    def setUp(self):
        self.gen = SubantaGenerator()

    def test_analyze_cases(self):
        for word, prati, linga, vibhakti, vacana in ANALYZE_CASES:
            with self.subTest(word=word):
                matches = self.gen.analyze(word)
                valid = [
                    m for m in matches
                    if m["pratipadika"] == prati
                    and m["linga"] == linga
                    and m["vibhakti"] == vibhakti
                    and m["vacana"] == vacana
                ]
                self.assertTrue(len(valid) > 0, f"Failed to analyze {word}")

    def test_generate_analyze_round_trip(self):
        table = self.gen.generate("rAma", "pum")
        for vibhakti in ("prathamA", "tfIyA", "saptamI"):
            for vacana_idx, form in enumerate(table[vibhakti]):
                primary = form.split("/")[0]
                with self.subTest(form=primary, vibhakti=vibhakti):
                    matches = self.gen.analyze(primary)
                    self.assertTrue(
                        any(m["pratipadika"] == "rAma" and m["vibhakti"] == vibhakti for m in matches)
                    )


class TestSubantaEdgeCases(unittest.TestCase):
    def setUp(self):
        self.gen = SubantaGenerator()

    def test_wrong_gender_raises(self):
        with self.assertRaises(NotImplementedError):
            self.gen.generate("vAc", "pum")

    def test_empty_input(self):
        self.assertIsNone(self.gen.generate("", "pum"))

    def test_unknown_stem_raises(self):
        with self.assertRaises(NotImplementedError):
            self.gen.generate("xyzUnknown", "pum")


if __name__ == "__main__":
    unittest.main()
