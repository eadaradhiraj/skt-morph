import unittest

from sktmorph.sarvanama import SarvanamaGenerator, PRONOUNS

VIBHAKTIS = [
    "prathamA", "dvitIyA", "tfIyA", "caturTI",
    "paYcamI", "zazWI", "saptamI",
]

# (base, linga, prathama, dvitiya, tritiya)
PRONOUN_CASES = [
    ("tad", "pum", ["saH", "tO", "te"], ["tam", "tO", "tAn"], ["tena", "tAByAm", "tEH"]),
    ("tad", "stri", ["sA", "te", "tAH"], ["tAm", "te", "tAH"], ["tayA", "tAByAm", "tABiH"]),
    ("tad", "nap", ["tat", "te", "tAni"], ["tat", "te", "tAni"], ["tena", "tAByAm", "tEH"]),
    ("kim", "pum", ["kaH", "kO", "ke"], ["kam", "kO", "kAn"], ["kena", "kAByAm", "kEH"]),
    ("kim", "stri", ["kA", "ke", "kAH"], ["kAm", "ke", "kAH"], ["kayA", "kAByAm", "kABiH"]),
    ("kim", "nap", ["kim", "ke", "kAni"], ["kim", "ke", "kAni"], ["kena", "kAByAm", "kEH"]),
    ("asmad", "any", ["aham", "AvAm", "vayam"], ["mAm/mA", "AvAm/nO", "asmAn/naH"], ["mayA", "AvAByAm", "asmABiH"]),
    ("yuzmad", "any", ["tvam", "yuvAm", "yUyam"], ["tvAm/tvA", "yuvAm/vAm", "yuzmAn/vaH"], ["tvayA", "yuvAByAm", "yuzmABiH"]),
    ("sarva", "pum", ["sarvaH", "sarvO", "sarve"], ["sarvam", "sarvO", "sarvAn"], ["sarveRa", "sarvAByAm", "sarvEH"]),
    ("sarva", "stri", ["sarvA", "sarve", "sarvAH"], ["sarvAm", "sarve", "sarvAH"], ["sarvayA", "sarvAByAm", "sarvABiH"]),
    ("sarva", "nap", ["sarvam", "sarve", "sarvARi"], ["sarvam", "sarve", "sarvARi"], ["sarveRa", "sarvAByAm", "sarvEH"]),
    ("idam", "pum", ["ayam", "imO", "ime"], ["enam", "imO", "imAn"], ["enena", "AByAm", "EBiH"]),
    ("idam", "stri", ["iyam", "ime", "imAH"], ["imAm", "ime", "imAH"], ["ayA", "AByAm", "ABiH"]),
    ("idam", "nap", ["idam", "ime", "imAni"], ["idam", "ime", "imAni"], ["enena", "AByAm", "EBiH"]),
    ("etad", "pum", ["ezaH", "etO", "ete"], ["etam", "etO", "etAn"], ["etena", "etAByAm", "etEBiH"]),
    ("etad", "stri", ["etA", "ete", "etAH"], ["etAm", "ete", "etAH"], ["etayA", "etAByAm", "etABiH"]),
    ("etad", "nap", ["etat", "ete", "etAni"], ["etat", "ete", "etAni"], ["etena", "etAByAm", "etEBiH"]),
    ("yad", "pum", ["yaH", "yO", "ye"], ["yam", "yO", "yAn"], ["yena", "yAByAm", "yEH"]),
    ("yad", "stri", ["yA", "ye", "yAH"], ["yAm", "ye", "yAH"], ["yayA", "yAByAm", "yABiH"]),
    ("yad", "nap", ["yat", "ye", "yAni"], ["yat", "ye", "yAni"], ["yena", "yAByAm", "yEH"]),
    ("ubha", "any", ["ubhau", "ubhau", "ubhe"], ["ubhau", "ubhau", "ubhe"], ["ubhABhyAm", "ubhABhyAm", "ubhABhiH"]),
    ("ena", "pum", ["ezaH", "etO", "ete"], ["etam", "etO", "etAn"], ["etena", "etAByAm", "etEBiH"]),
    ("ena", "stri", ["etA", "ete", "etAH"], ["etAm", "ete", "etAH"], ["etayA", "etAByAm", "etABiH"]),
    ("ena", "nap", ["etat", "ete", "etAni"], ["etat", "ete", "etAni"], ["etena", "etAByAm", "etEBiH"]),
    ("eka", "pum", ["ekaH", "ekO", "eke"], ["ekam", "ekO", "ekAn"], ["ekena", "ekAByAm", "ekEBiH"]),
    ("sva", "pum", ["svaH", "svO", "sve"], ["svam", "svO", "svAn"], ["svena", "svAByAm", "svEBiH"]),
    ("traya", "pum", ["trayaH", "trayO", "trayaH"], ["trayam", "trayO", "trIn"], ["tribhiH", "tribhyAm", "tribhiH"]),
    ("catur", "pum", ["catvAraH", "catvArO", "catvAraH"], ["caturam", "catvArO", "catvARi"], ["caturBiH", "caturByAm", "caturBiH"]),
    ("anya", "pum", ["anyaH", "anyO", "anye"], ["anyam", "anyO", "anyAn"], ["anyena", "anyAByAm", "anyEH"]),
    ("purva", "pum", ["purvaH", "purvO", "purve"], ["purvam", "purvO", "purvAn"], ["purveRa", "purvAByAm", "purvEH"]),
    ("paJcan", "pum", ["paJcaH", "paJcO", "paJcaH"], ["paJcam", "paJcO", "paJcan"], ["paJcaBiH", "paJcaByAm", "paJcaBiH"]),
    ("zaq", "pum", ["zaq", "zaqO", "zaq"], ["zaqam", "zaqO", "zaws"], ["zaqBiH", "zaqByAm", "zaqBiH"]),
    ("saptan", "pum", ["saptaH", "saptO", "saptaH"], ["saptam", "saptO", "saptAn"], ["saptaBiH", "saptaByAm", "saptaBiH"]),
]

ANALYZE_CASES = [
    ("saH", "tad", "pum", "prathamA", 1),
    ("sA", "tad", "stri", "prathamA", 1),
    ("tat", "tad", "nap", "prathamA", 1),
    ("kaH", "kim", "pum", "prathamA", 1),
    ("kim", "kim", "nap", "prathamA", 1),
    ("aham", "asmad", "any", "prathamA", 1),
    ("tvam", "yuzmad", "any", "prathamA", 1),
    ("sarvaH", "sarva", "pum", "prathamA", 1),
    ("ayam", "idam", "pum", "prathamA", 1),
    ("iyam", "idam", "stri", "prathamA", 1),
    ("etat", "etad", "nap", "prathamA", 1),
    ("yaH", "yad", "pum", "prathamA", 1),
    ("te", "tad", "pum", "prathamA", 3),
    ("te", "yuzmad", "any", "caturTI", 1),
    ("me", "asmad", "any", "zazWI", 1),
    ("tasmE", "tad", "pum", "caturTI", 1),
]


class TestSarvanamaGenerationComprehensive(unittest.TestCase):
    def setUp(self):
        self.gen = SarvanamaGenerator()

    def test_all_pronoun_paradigms_defined(self):
        bases = {
            "tad", "kim", "asmad", "yuzmad", "sarva", "idam", "etad", "yad",
            "ubha", "ena", "eka", "dvi", "sva", "am",
            "traya", "catur", "purva", "para", "apara", "anya",
            "paJcan", "zaq", "saptan", "azwan", "navan", "daSan",
        }
        defined = {base for base, _ in PRONOUNS.keys()}
        self.assertTrue(bases.issubset(defined))

    def test_generation_matrix(self):
        for base, linga, prathama, dvitiya, tritiya in PRONOUN_CASES:
            with self.subTest(base=base, linga=linga):
                table = self.gen.generate(base, linga)
                self.assertEqual(table["prathamA"], prathama)
                self.assertEqual(table["dvitIyA"], dvitiya)
                self.assertEqual(table["tfIyA"], tritiya)

    def test_asmad_yuzmad_ubha_ignore_passed_linga(self):
        asmad = self.gen.generate("asmad", "pum")
        self.assertEqual(asmad["prathamA"], ["aham", "AvAm", "vayam"])
        yuzmad = self.gen.generate("yuzmad", "stri")
        self.assertEqual(yuzmad["prathamA"], ["tvam", "yuvAm", "yUyam"])
        ubha = self.gen.generate("ubha", "pum")
        self.assertEqual(ubha["prathamA"], ["ubhau", "ubhau", "ubhe"])

    def test_seven_vibhaktis_for_core_pronouns(self):
        table = self.gen.generate("tad", "pum")
        for v in VIBHAKTIS:
            self.assertIn(v, table)
            self.assertEqual(len(table[v]), 3)

    def test_sarva_has_sambodhana(self):
        table = self.gen.generate("sarva", "pum")
        self.assertIn("samboDana", table)


class TestSarvanamaAnalyzeComprehensive(unittest.TestCase):
    def setUp(self):
        self.gen = SarvanamaGenerator()

    def test_analyze_cases(self):
        for word, base, linga, vibhakti, vacana in ANALYZE_CASES:
            with self.subTest(word=word, base=base):
                matches = self.gen.analyze(word)
                valid = [
                    m for m in matches
                    if m["pratipadika"] == base
                    and m["linga"] == linga
                    and m["vibhakti"] == vibhakti
                    and m["vacana"] == vacana
                ]
                self.assertTrue(len(valid) > 0, f"Failed to analyze {word} as {base}")

    def test_overloaded_form_te(self):
        matches = self.gen.analyze("te")
        bases = {(m["pratipadika"], m["linga"]) for m in matches}
        self.assertIn(("tad", "pum"), bases)
        self.assertIn(("tad", "stri"), bases)
        self.assertIn(("yuzmad", "any"), bases)


class TestSarvanamaEdgeCases(unittest.TestCase):
    def setUp(self):
        self.gen = SarvanamaGenerator()

    def test_unknown_pronoun_raises(self):
        with self.assertRaises(NotImplementedError):
            self.gen.generate("unknown", "pum")

    def test_invalid_gender_for_tad_raises(self):
        with self.assertRaises(NotImplementedError):
            self.gen.generate("tad", "any")


if __name__ == "__main__":
    unittest.main()
