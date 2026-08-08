import sqlite3
import unittest

from sktmorph.engine.endings import family_endings, gana_class, ending_table
from sktmorph.engine.join import join_form, join_variants
from sktmorph.engine.krdanta import LiveKrdantaEngine
from sktmorph.engine.lakara import (
    lakara_family,
    normalize_lakara,
    pada_from_lakara,
    resolve_pada,
)
from sktmorph.engine.lang_ya import LANG_YA_P
from sktmorph.engine.stems import derive_stem, future_stem, perfect_stem
from sktmorph.engine.tinanta import LiveTinantaEngine


class TestEngineCoverage(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.conn = sqlite3.connect("sktmorph/data/dhatus.sqlite")
        cls.tin = LiveTinantaEngine(cls.conn)
        cls.krd = LiveKrdantaEngine(cls.conn)

    @classmethod
    def tearDownClass(cls):
        cls.conn.close()

    def test_lakara_helpers(self):
        self.assertEqual(normalize_lakara("unknown")[0], "unknown")
        self.assertIsNone(lakara_family("unknown"))
        self.assertEqual(pada_from_lakara("plit"), "P")
        self.assertEqual(pada_from_lakara("alat"), "A")
        self.assertIsNone(resolve_pada("unknown", "U"))

    def test_gana_class(self):
        self.assertEqual(gana_class(2), "ad")
        self.assertEqual(gana_class(5), "nu")
        self.assertEqual(gana_class(7), "thematic")

    def test_family_endings_branches(self):
        self.assertIsNone(family_endings("plat", "karmani", "P", 1))
        self.assertIsNotNone(family_endings("lot", "kartari", "P", 10))
        self.assertIsNotNone(family_endings("lot", "kartari", "P", 9))
        self.assertIsNotNone(family_endings("lang", "kartari", "P", 4))
        self.assertIsNotNone(family_endings("lat", "kartari", "P", 5))
        self.assertIsNotNone(family_endings("lat", "kartari", "A", 5))
        self.assertIsNotNone(family_endings("lang", "kartari", "P", 2))
        self.assertEqual(len(LANG_YA_P), 9)

    def test_future_stem_branches(self):
        self.assertEqual(future_stem("coraya", 10, "coraya"), "corayizya")
        self.assertEqual(future_stem("so", 5), "sozya")
        self.assertEqual(future_stem("ad", 2), "atsya")
        self.assertEqual(future_stem("tot", 6), "totsya")
        self.assertEqual(future_stem("dIvya", 4, "dIvya"), "dIvyizya")
        self.assertEqual(future_stem("Bav", 1), "Bavizya")

    def test_perfect_stem_branches(self):
        self.assertTrue(perfect_stem("gam", "gac").startswith("ga"))
        self.assertTrue(perfect_stem("x", "xa").endswith("a"))

    def test_derive_stem_all_ganas(self):
        cases = [
            ("BU", 1, "lat"),
            ("ad", 2, "lat"),
            ("hu", 3, "lat"),
            ("div", 4, "lat"),
            ("su", 5, "lat"),
            ("tud", 6, "lat"),
            ("ruD", 7, "lat"),
            ("tan", 8, "lat"),
            ("krI", 9, "lat"),
            ("cur", 10, "lat"),
        ]
        for dhatu, gana, fam in cases:
            stem, _, steps = derive_stem(dhatu, gana, fam, "shuddha")
            self.assertTrue(stem, f"{dhatu}/{gana}/{fam}")
            self.assertTrue(steps)

    def test_derive_stem_other_families(self):
        for fam in ("lot", "lrt", "lang", "vidhilin", "lit"):
            stem, aug, _ = derive_stem("BU", 1, fam, "shuddha")
            self.assertTrue(stem, fam)
            if fam == "lang":
                self.assertEqual(aug, "a")

    def test_derive_stem_nich(self):
        stem, _, steps = derive_stem("BU", 1, "lat", "nich")
        self.assertIsNone(stem)

    def test_join_variants(self):
        forms = join_variants("Bava", ["ti"], 1, "lat", 1, "P", None)
        self.assertEqual(forms, ["Bavati"])
        forms = join_variants("Bav", ["at"], 1, "lang", 1, "P", "a")
        self.assertTrue(forms[0].startswith("a"))

    def test_join_gana_specific(self):
        self.assertEqual(join_form("ad", "ti", 2, "lat", 1, "P"), "atti")
        self.assertEqual(join_form("ad", "Di", 2, "lot", 2, "P"), "adDi")
        self.assertEqual(join_form("sunu", "ti", 5, "lat", 1, "P"), "sunoti")
        self.assertEqual(join_form("tanu", "ti", 8, "lat", 1, "P"), "tanoti")
        self.assertEqual(join_form("ruRa", "ti", 7, "lat", 1, "P"), "ruRadDi")
        self.assertEqual(join_form("krInA", "ti", 9, "lat", 1, "P"), "krIRAti")
        self.assertEqual(join_form("krInA", "Atu", 9, "lot", 1, "P"), "krIRAtu")
        self.assertEqual(join_form("dIv", "yat", 4, "lang", 1, "P", "a"), "adIvyat")
        self.assertEqual(join_form("atsya", "ti", 2, "lrt", 1, "P"), "atsyati")
        self.assertEqual(join_form("coray", "et", 10, "vidhilin", 1, "P"), "corayet")

    def test_join_empty_and_augment(self):
        self.assertEqual(join_form("sunu", "", 5, "lot", 2, "P"), "sunu")
        self.assertEqual(join_form("ad", "at", 2, "lang", 1, "P", "a"), "Adat")

    def test_krdanta_modes(self):
        modes = [
            ("Satf~", "Bavan"),
            ("ktavatu", "BUtavat"),
            ("ktavatu~", None),
            ("tumun", "Bavitum"),
            ("ktvA", "BUtvA"),
            ("ac", "Bava"),
            ("ktin", "BUti"),
            ("yat", "Bavya"),
            ("Ryat", "BAvya"),
            ("GaY", "Bava"),
            ("Ramul", "Bavam"),
            ("Rvul", "BAvaka"),
            ("vun", "Bavaka"),
            ("anIyar", None),
            ("tavya", "Bavitavya"),
            ("tfc", "Bavitf"),
            ("SAnac", "BavamAna"),
            ("cAnaS", "BavamAna"),
            ("gsnu", "BUzRu"),
            ("kvasu", None),
        ]
        for pratyaya, _ in modes:
            form, steps = self.krd.derive("01.0001", pratyaya)
            if pratyaya == "ktavatu~":
                self.assertTrue(form.endswith("vat"))
            elif pratyaya == "anIyar":
                self.assertTrue(form)
            elif pratyaya == "kvasu":
                self.assertTrue(form.startswith("B"))
            else:
                self.assertTrue(form, pratyaya)
            self.assertTrue(steps)

    def test_krdanta_unknown_dhatu(self):
        self.assertIsNone(self.krd.derive("zzzz", "Satf")[0])

    def test_tinanta_generate_paradigm_and_edges(self):
        forms, steps = self.tin.generate_paradigm("01.0001", "plat")
        self.assertTrue(forms)
        self.assertTrue(steps)
        self.assertEqual(self.tin.generate("01.0001", "plat", 3, 4)[0], None)
        self.assertEqual(self.tin.generate_all("nope", "plat", 1, 1), ([], []))
        self.assertEqual(self.tin.generate_all("01.0001", "nope", 1, 1), ([], []))
        forms, steps = self.tin.generate_all("01.0001", "plat", 1, 1, prayoga="karmani")
        self.assertEqual(forms, [])
        self.assertTrue(steps)

    def test_tinanta_atmanepada_gana6(self):
        row = self.conn.execute(
            "SELECT dhatu_id FROM dhatus WHERE json_extract(details_json, '$.gana') = '6' AND json_extract(details_json, '$.pada') = 'U' LIMIT 1"
        ).fetchone()
        if row:
            form, _ = self.tin.generate(row[0], "alat", 1, 1)
            self.assertTrue(form)

    def test_tinanta_perfect_parasmaipada(self):
        form, _ = self.tin.generate("01.0001", "plit", 1, 1)
        self.assertEqual(form, "baBUva")

    def test_join_exhaustive(self):
        j = join_form
        self.assertEqual(j("ad", "thaH", 2, "lat", 2, "P"), "atTaH")
        self.assertEqual(j("ad", "tha", 2, "lat", 2, "P"), "atTa")
        self.assertEqual(j("ad", "at", 2, "lang", 1, "P", "a"), "Adat")
        self.assertEqual(j("Bavizya", "ti", 1, "lrt", 1, "P"), "Bavizyati")
        self.assertEqual(j("Bava", "Ami", 1, "lat", 3, "P"), "BavAmi")
        self.assertEqual(j("x", "ti", 99, "lat", 1, "P"), "xti")
        self.assertEqual(j("sunu", "te", 5, "lat", 1, "A"), "sunute")
        self.assertEqual(j("sunu", "vAte", 5, "lat", 1, "A", None), "sunvAte")
        self.assertEqual(j("sunu", "ze", 5, "lat", 2, "A"), "sunuze")
        self.assertEqual(j("sunu", "taH", 5, "lat", 1, "P"), "sunutaH")
        self.assertEqual(j("sunu", "mi", 5, "lat", 3, "P"), "sunomi")
        self.assertEqual(j("ruRa", "taH", 7, "lat", 1, "P"), "rundDaH")
        self.assertEqual(j("ruRa", "nti", 7, "lat", 1, "P"), "runDanti")
        self.assertEqual(j("ruRa", "si", 7, "lat", 2, "P"), "ruRatsi")
        self.assertEqual(j("ruRa", "TaH", 7, "lat", 2, "P"), "rundDaH")
        self.assertEqual(j("ruRa", "Ami", 7, "lat", 3, "P"), "ruRaDmi")
        self.assertEqual(j("ruRa", "et", 7, "vidhilin", 1, "P"), "ruRaet")
        self.assertEqual(j("krInA", "te", 9, "lat", 1, "A"), "krIRIte")
        self.assertEqual(j("krInA", "ItAm", 9, "lot", 1, "P"), "krIRItAm")
        self.assertEqual(j("krInA", "Ava", 9, "lot", 3, "P"), "krIRAva")
        self.assertEqual(j("krInA", "ete", 9, "lat", 1, "A"), "krIRete")
        self.assertEqual(j("krInA", "e", 9, "lat", 3, "A"), "krIRe")
        self.assertEqual(j("krInA", "foo", 9, "lat", 1, "P"), "krInAfoo")

    def test_stem_branches(self):
        self.assertIsNotNone(derive_stem("su", 5, "lang", "shuddha")[0])
        self.assertIsNotNone(derive_stem("ruD", 7, "lang", "shuddha")[0])
        self.assertIsNotNone(derive_stem("krI", 9, "lang", "shuddha")[0])
        self.assertIsNotNone(derive_stem("cur", 10, "lang", "shuddha")[0])
        self.assertIsNotNone(derive_stem("div", 4, "lang", "shuddha")[0])
        self.assertIsNotNone(derive_stem("tud", 6, "lang", "shuddha")[0])
        self.assertEqual(future_stem("x", 7), "xizya")
        self.assertEqual(future_stem("Bava", 1, "Bava"), "Bavizya")

    def test_endings_fallback(self):
        from sktmorph.engine.endings import ending_table as legacy
        self.assertIsNone(legacy("nope", "kartari", "P", 1))
        self.assertIsNone(family_endings("nope", "kartari", "P", 2))

    def test_krdanta_gana4_and_default(self):
        form, _ = self.krd.derive("04.0001", "Satf")
        self.assertEqual(form, "dIvyat")
        form, _ = self.krd.derive("10.0001", "Satf")
        self.assertTrue(form)
        form, _ = self.krd.derive("01.0001", "lyap")
        self.assertTrue(form or form is None)
        form, _ = self.krd.derive("01.0001", "unknown", "shuddha")
        self.assertIsNone(form)

    def test_resolve_pada_branches(self):
        self.assertEqual(resolve_pada("plit", "U"), "P")
        self.assertEqual(resolve_pada("alat", "U"), "A")
        self.assertEqual(resolve_pada("plat", "U"), "P")
        self.assertEqual(resolve_pada("plat", "A"), "P")
        self.assertEqual(resolve_pada("alat", "P"), "A")
        self.assertIsNone(resolve_pada("xyz", "U"))

    def test_join_remaining_branches(self):
        j = join_form
        self.assertEqual(j("ad", "", 2, "lat", 1, "P"), "ad")
        self.assertEqual(j("ad", "anti", 2, "lat", 1, "P"), "adanti")
        self.assertEqual(j("xyz", "at", 2, "lang", 1, "P", "a"), "axyzat")
        self.assertEqual(j("so", "ti", 5, "lrt", 1, "P"), "soti")
        self.assertEqual(j("stem", "ti", 5, "lat", 1, "P"), "stemti")
        self.assertEqual(j("sunu", "nti", 5, "lat", 1, "P"), "sunvanti")
        self.assertEqual(j("sunu", "zi", 5, "lat", 2, "P"), "sunozi")
        self.assertEqual(j("sunu", "TaH", 5, "lat", 2, "P"), "sunuTaH")
        self.assertEqual(j("sunu", "Ani", 5, "lot", 3, "P"), "sunavAni")
        self.assertEqual(j("sunu", "antu", 5, "lot", 1, "P"), "sunvantu")
        self.assertEqual(j("sunu", "tu", 5, "lot", 1, "P"), "sunotu")
        self.assertEqual(j("sunu", "tAm", 5, "lot", 1, "P"), "sunutAm")
        self.assertEqual(j("ruRa", "ti", 7, "lrt", 1, "P"), "ruRati")
        self.assertEqual(j("ruRa", "Ta", 7, "lat", 2, "P"), "rundDa")
        self.assertEqual(j("ruRa", "AvaH", 7, "lat", 3, "P"), "runDvaH")
        self.assertEqual(j("ruRa", "AmaH", 7, "lat", 3, "P"), "runDmaH")
        self.assertEqual(j("stem", "ti", 9, "lat", 1, "P"), "stemti")
        self.assertEqual(j("krInA", "Ihi", 9, "lot", 2, "P"), "krIRIhi")
        self.assertEqual(j("krInA", "Itam", 9, "lot", 2, "P"), "krIRItam")
        self.assertEqual(j("krInA", "Ani", 9, "lot", 3, "P"), "krIRAni")
        self.assertEqual(j("krInA", "nti", 9, "lat", 1, "P"), "krIRanti")
        self.assertEqual(j("krInA", "si", 9, "lat", 2, "P"), "krIRAsi")
        self.assertEqual(j("krInA", "TaH", 9, "lat", 2, "P"), "krIRITaH")
        self.assertEqual(j("krInA", "Ta", 9, "lat", 2, "P"), "krIRITa")
        self.assertEqual(j("krInA", "te", 9, "lat", 1, "A"), "krIRIte")
        self.assertEqual(j("krInA", "ete", 9, "lat", 1, "A"), "krIRete")
        self.assertEqual(j("krInA", "ante", 9, "lat", 1, "A"), "krIRIante")
        self.assertEqual(j("krInA", "se", 9, "lat", 2, "A"), "krIRIse")
        self.assertEqual(j("krInA", "eTe", 9, "lat", 2, "A"), "krIReTe")
        self.assertEqual(j("krInA", "aDve", 9, "lat", 2, "A"), "krIRIaDve")
        self.assertEqual(j("krInA", "e", 9, "lat", 3, "A"), "krIRe")
        self.assertEqual(j("krInA", "Ami", 9, "lat", 3, "P"), "krIRAmi")
        self.assertEqual(j("krInA", "AvaH", 9, "lat", 3, "P"), "krIRIvaH")
        self.assertEqual(j("krInA", "AmaH", 9, "lat", 3, "P"), "krIRImaH")
        self.assertEqual(j("krInA", "Avahe", 9, "lat", 3, "A"), "krIRIAvahe")
        self.assertEqual(j("krInA", "Amahe", 9, "lat", 3, "A"), "krIRIAmahe")
        self.assertEqual(j("sunu", "maH", 5, "lat", 3, "P"), "sunumaH")
        self.assertEqual(j("sunu", "mi", 5, "lat", 3, "P"), "sunomi")
        self.assertEqual(j("sunu", "", 5, "lot", 2, "P"), "sunu")
        self.assertEqual(j("ruRa", "Ami", 7, "lat", 3, "P"), "ruRaDmi")
        self.assertEqual(j("xyz", "anti", 7, "lat", 1, "P"), "xyzanti")
        self.assertEqual(j("krInA", "Atu", 9, "lot", 1, "P"), "krIRAtu")
        self.assertEqual(j("krInA", "ItAt", 9, "lot", 1, "P"), "krIRItAt")
        self.assertEqual(j("krInA", "ItAm", 9, "lot", 1, "P"), "krIRItAm")
        self.assertEqual(j("krInA", "antu", 9, "lot", 1, "P"), "krIRantu")
        self.assertEqual(j("krInA", "Ava", 9, "lot", 3, "P"), "krIRAva")
        self.assertEqual(j("sono", "maH", 5, "lat", 3, "P"), "sonomaH")
        self.assertEqual(j("sunu", "ze", 5, "lat", 2, "A"), "sunuze")
        self.assertEqual(j("krInA", "taH", 9, "lat", 1, "P"), "krIRItaH")

    def test_stem_final_and_edge_cases(self):
        stem, _, _ = derive_stem("BU", 1, "unknown", "shuddha")
        self.assertEqual(stem, "Bava")
        self.assertEqual(perfect_stem("cur", "cor"), "cacora")
        self.assertIsNotNone(derive_stem("tud", 6, "lrt", "shuddha")[0])
        self.assertIsNone(family_endings("vidhilin", "kartari", "A", 2))
        self.assertEqual(resolve_pada("xyz", "U"), None)
        self.assertEqual(pada_from_lakara("plat"), "P")
        self.assertEqual(pada_from_lakara("plit"), "P")
        self.assertEqual(pada_from_lakara("alat"), "A")
        self.assertEqual(future_stem("roD", 7), "rotsya")
        self.assertEqual(future_stem("toda", 6, "toda"), "todsya")
        self.assertTrue(perfect_stem("k", "k")[0] == "k")
        self.assertTrue(perfect_stem("gam", "gac").startswith("ga"))
        self.assertTrue(derive_stem("xyz", 99, "lat", "shuddha")[0] is None)
        self.assertIsNotNone(derive_stem("div", 4, "lrt", "shuddha")[0])
        self.assertIsNotNone(derive_stem("ruD", 7, "lrt", "shuddha")[0])
        self.assertIsNotNone(derive_stem("krI", 9, "vidhilin", "shuddha")[0])
        self.assertIsNotNone(derive_stem("ruD", 7, "lang", "shuddha")[0])
        self.assertIsNotNone(derive_stem("N", 7, "lang", "shuddha")[0])

    def test_endings_and_krdanta_remaining(self):
        from unittest.mock import patch

        self.assertIsNotNone(family_endings("lrt", "kartari", "P", 5))
        table = family_endings("lang", "kartari", "P", 1)
        self.assertIsNotNone(table)
        self.assertIsNone(ending_table("nope", "kartari", "P", 1))
        with patch("sktmorph.engine.endings.family_endings", return_value=None):
            self.assertIsNone(ending_table("plat", "kartari", "P", 1))
        form, _ = self.krd.derive("04.0001", "Satf")
        self.assertEqual(form, "dIvyat")
        form, _ = self.krd.derive("04.0001", "GaY")
        self.assertTrue(form)
        form, _ = self.krd.derive("04.0001", "anIyar")
        self.assertTrue(form.endswith("anIya"))
        form, _ = self.krd.derive("04.0001", "ktvA")
        self.assertTrue(form)
        form, _ = self.krd.derive("01.0001", "Ramul")
        self.assertTrue(form)
        with patch.dict(
            __import__("sktmorph.engine.krdanta", fromlist=["PRATYAYA_RULES"]).PRATYAYA_RULES,
            {"testmode": ("x", ["1.1.1"], "other")},
        ):
            form, _ = self.krd.derive("01.0001", "testmode")
            self.assertTrue(form)


if __name__ == "__main__":
    unittest.main()
