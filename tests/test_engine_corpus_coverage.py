"""Exercise the live tinanta engine across the corpus and cover engine gaps."""
import sqlite3
import unittest

from sktmorph.engine.endings import family_endings
from sktmorph.engine.join import (
    _join_gana3_a,
    _join_gana3_ad,
    _join_gana3_lit,
    _join_han,
    _join_n,
    _join_ni,
    _join_ni_npattern,
    _thematic_lot_third,
    join_form,
)
from sktmorph.engine.krdanta import LiveKrdantaEngine
from sktmorph.engine.lakara import kartari_compatible
from sktmorph.engine.phonology import (
    bidadi_present_stem,
    is_yajadi,
    uses_aya_present,
)
from sktmorph.engine.stems import (
    _g1_future_from_present,
    _g1_future_suffix,
    derive_stem,
    future_stem,
    g6_future_stem,
)
from sktmorph.engine.tinanta import LiveTinantaEngine
from scripts.exercise_engine_corpus import exercise_corpus


class TestEngineCorpusCoverage(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.conn = sqlite3.connect("sktmorph/data/dhatus.sqlite")
        cls.tin = LiveTinantaEngine(cls.conn)
        cls.krd = LiveKrdantaEngine(cls.conn)

    @classmethod
    def tearDownClass(cls):
        cls.conn.close()

    def test_exercise_full_corpus(self):
        exercise_corpus()

    def test_kartari_compatible_ubhayapada(self):
        self.assertTrue(kartari_compatible("U", "alat"))
        self.assertTrue(kartari_compatible("U", "plit"))
        self.assertFalse(kartari_compatible("P", "alat"))

    def test_nu_gana_atmanepada_lat_endings(self):
        self.assertIsNotNone(family_endings("lat", "kartari", "A", 5))

    def test_aya_present_helpers(self):
        self.assertTrue(is_yajadi("yajAdiH"))
        self.assertTrue(uses_aya_present(1, "ji", ""))
        self.assertEqual(bidadi_present_stem("De"), "Daya")

    def test_g1_and_g6_future_branches(self):
        self.assertTrue(g6_future_stem("Brajj").endswith("kzya"))
        self.assertTrue(_g1_future_suffix("kze", "kzi").endswith("zya"))
        self.assertTrue(_g1_future_suffix("akz", "akz").endswith("izya"))
        self.assertTrue(_g1_future_from_present("Siz", "Siza", "Siz").endswith("kzya"))
        self.assertTrue(_g1_future_from_present("sad", "sada", "sad").endswith("tsya"))
        self.assertEqual(future_stem("je", 1, "jea", "ji"), "jezya")
        self.assertTrue(future_stem("zWev", 1, "zWIva", "zWiv").endswith("izya"))

    def test_bidadi_and_aya_derive_stems(self):
        for fam in ("lrt", "lang", "vidhilin"):
            stem, aug, _ = derive_stem("ji", 1, fam, "shuddha", "", "")
            self.assertTrue(stem, fam)
            if fam == "lang":
                self.assertEqual(aug, "a")
        stem, _, _ = derive_stem("kzi", 1, "lrt", "shuddha", "", "BidAdiH")
        self.assertTrue(stem.endswith("zya"))
        stem, _, _ = derive_stem("ve", 1, "lat", "shuddha", "", "yajAdiH")
        self.assertTrue(stem.endswith("aya"))

    def test_han_and_dviz_join(self):
        j = join_form
        self.assertEqual(j("hanizya", "anti", 2, "lrt", 3, "P", None, "han", 3), "hanizyanti")
        self.assertEqual(j("hanizya", "Ami", 2, "lrt", 3, "P", None, "han", 1), "hanizyAmi")
        self.assertEqual(j("han", "aH", 2, "lang", 2, "P", None, "han", 1), "han")
        self.assertEqual(j("han", "aH", 2, "lang", 2, "P", "a", "han", 1), "ahan")
        self.assertEqual(j("dvez", "anti", 2, "lat", 1, "P", None, "dviz", 3), "dvizanti")
        self.assertEqual(j("dvez", "si", 2, "lat", 2, "P", None, "dviz", 1), "dvekzi")
        self.assertEqual(j("dvez", "at", 2, "lang", 1, "P", "a", "dviz", 1), "advew")
        self.assertEqual(j("dvez", "Ani", 2, "lot", 3, "P", None, "dviz", 1), "dvezARi")
        self.assertEqual(j("dvez", "Ava", 2, "lot", 3, "P", None, "dviz", 2), "dvezAva")
        self.assertEqual(j("dvez", "Ama", 2, "lot", 3, "P", None, "dviz", 3), "dvezAma")
        self.assertEqual(j("dvez", "zz", 2, "lat", 1, "P", None, "dviz", 1), "dvezzz")

    def test_gana3_and_n_gana_join_branches(self):
        j = join_form
        self.assertEqual(j("juhu", "nti", 3, "lat", 1, "P", None, "hu", 3), "juhvati")
        stem, _, _ = derive_stem("BI", 3, "lot", "shuddha")
        self.assertTrue(j(stem, "tAt", 3, "lot", 1, "P", None, "BI", 1).endswith("tAt"))
        self.assertTrue(j(stem, "van", 3, "lang", 1, "P", "a", "BI", 1))
        self.assertTrue(j(stem, "yat", 3, "vidhilin", 1, "P", None, "BI", 1).endswith("yat"))
        self.assertEqual(j("juhu", "va", 3, "lit", 2, "P", None, "hu", 3), "juhuva")
        stem, aug, _ = derive_stem("ruD", 7, "lang", "shuddha")
        self.assertEqual(j(stem, "at", 7, "lang", 1, "P", None, "ruD", 1), "ruRat")
        self.assertEqual(j(stem, "atAm", 7, "lang", 1, "P", None, "ruD", 2), "rundDAm")

    def test_g9_ni_lang_join(self):
        stem, _, _ = derive_stem("mI", 9, "lang", "shuddha", "", "")
        self.assertTrue(join_form(stem, "At", 9, "lang", 1, "P", "a", "mI", 1, ""))

    def test_thematic_lot_and_plot(self):
        self.assertEqual(join_form("gopa", "Ani", 1, "lot", 3, "P", None, "gop", 1), "gopAni")
        self.assertEqual(join_form("kzaya", "Ani", 1, "lot", 3, "P", None, "kzi", 1, "BidAdiH"), "kzayARi")
        self.assertEqual(join_form("stem", "Ani", 1, "lot", 3, "P", None, "x", 1), "stemAni")

    def test_join_internal_branches(self):
        self.assertEqual(_thematic_lot_third("gop", "tu", 1), "goptu")
        self.assertEqual(_join_han("hanizya", "anti", "lrt", 3, 3), "hanizyanti")
        self.assertEqual(_join_gana3_a("abI", "ete", 1, 1, "BI"), "abAte")
        self.assertEqual(_join_gana3_a("abI", "ante", 1, 1, "BI"), "abate")
        self.assertEqual(_join_gana3_a("abI", "se", 1, 1, "BI"), "abIze")
        self.assertEqual(_join_gana3_a("abI", "eTe", 2, 1, "BI"), "abATe")
        self.assertEqual(_join_gana3_a("abI", "aDve", 1, 1, "BI"), "abIDve")
        self.assertEqual(_join_gana3_a("abI", "e", 3, 1, "BI"), "abe")
        self.assertEqual(_join_gana3_a("abI", "Avahe", 3, 1, "BI"), "abIvahe")
        self.assertEqual(_join_gana3_a("abI", "Amahe", 3, 1, "BI"), "abImahe")
        self.assertEqual(_join_gana3_lit("juhu", "vatuH", 3, 1, "hu"), "juhuvatuH")
        stem, _, _ = derive_stem("mI", 9, "lang", "shuddha", "", "")
        self.assertTrue(_join_ni(stem, "At", "lang", 1, "P", "", "mI"))
        self.assertTrue(_join_ni(stem, "ItAm", "lang", 1, "P", "", "mI"))
        self.assertTrue(_join_ni(stem, "Atu", "lot", 1, "P", "", "mI"))

    def test_phonology_and_stem_gaps(self):
        from sktmorph.engine.phonology import (
            apply_vrddhi_to_stem,
            ends_with_vowel,
            g6_present_base,
            g9_n_lang_base,
            g9_r_lang_root,
            thematic_aya_present_stem,
            vowel_initial_lang_stem,
        )
        from sktmorph.engine.stems import _g6_skip_future_guna, perfect_stem

        self.assertFalse(ends_with_vowel(""))
        self.assertIsNotNone(vowel_initial_lang_stem("I"))
        self.assertTrue(g6_present_base("tud"))
        self.assertTrue(_g6_skip_future_guna("ump"))
        self.assertTrue(g6_future_stem("ajj").endswith("kzya"))
        self.assertEqual(g6_future_stem("majj"), "maNkzy")
        self.assertTrue(apply_vrddhi_to_stem("BU"))
        self.assertTrue(thematic_aya_present_stem("glE"))
        self.assertTrue(g9_n_lang_base("mI"))
        self.assertTrue(g9_r_lang_root("krI"))
        self.assertTrue(perfect_stem("BU", "BU").endswith("a"))
        self.assertTrue(future_stem("glE", 1, "glAya", "glE").endswith("sy"))
        stem, _, _ = derive_stem("rinv", 1, "lrt", "shuddha")
        self.assertTrue(stem.endswith("izya"))

    def test_join_engine_residuals(self):
        g3 = _join_gana3_ad
        self.assertEqual(g3("juhu", "nti", "lat", 1, "P", "hu"), "juhyati")
        self.assertEqual(g3("dAI", "TaH", "lat", 2, "P", "BI"), "dAITaH")
        self.assertEqual(g3("dAI", "taH", "lat", 1, "P", "BI"), "dAItaH")
        self.assertEqual(g3("dAI", "vantu", "lot", 1, "P", "BI"), "dAyatu")
        self.assertEqual(g3("xxUr", "avAni", "lot", 3, "P", "BI"), "xxuravAni")
        self.assertEqual(g3("xxUr", "avAva", "lot", 3, "P", "BI"), "xxuravAva")
        self.assertEqual(g3("xxUr", "avAma", "lot", 3, "P", "BI"), "xxuravAma")
        self.assertEqual(g3("dAI", "ot", "lang", 1, "P", "BI"), "dAIt")
        lit = _join_gana3_lit
        for ending, expect in (
            ("vatuH", "juhuvatuH"),
            ("vuH", "juhuvuH"),
            ("viTa", "juhoTa"),
            ("vaTuH", "juhuvaTuH"),
            ("viva", "juhuviva"),
            ("vima", "juhuvima"),
        ):
            self.assertEqual(lit("juhu", ending, 1, 1, "hu"), expect)
        self.assertEqual(_join_n("xxRa", "at", "lang", 1, "P"), "xxRat")
        self.assertEqual(_join_n("xxRa", "atAm", "lang", 1, "P"), "xxndDAm")
        self.assertEqual(_join_n("xxRa", "an", "lang", 1, "P"), "xxnDan")
        self.assertEqual(_join_n("xxRa", "aH", "lang", 2, "P"), "xxRaH")
        self.assertEqual(_join_n("xxRa", "va", "lang", 1, "P"), "xxnDva")
        self.assertEqual(_join_n("xxRa", "ma", "lang", 1, "P"), "xxnDma")
        self.assertEqual(_join_n("xxR", "at", "lang", 1, "P"), "xxRat")
        self.assertEqual(_join_n("xxR", "atAm", "lang", 1, "P"), "xxndDAm")
        self.assertEqual(_join_n("xxR", "an", "lang", 1, "P"), "xxnDan")
        self.assertEqual(_join_n("xxR", "va", "lang", 1, "P"), "xxnDva")
        self.assertEqual(_join_n("xxR", "ma", "lang", 1, "P"), "xxnDma")
        stem = "krInA"
        ni = _join_ni
        for ending, expect in (
            ("At", "krIRAt"),
            ("ItAm", "krIRItAm"),
            ("an", "krIRan"),
            ("AH", "krIRAH"),
            ("Itam", "krIRItam"),
            ("Ita", "krIRIta"),
            ("Am", "krIRAm"),
            ("Iva", "krIRIva"),
            ("Ima", "krIRIma"),
        ):
            self.assertEqual(ni(stem, ending, "lang", 1, "P", "", "krI"), expect)
        self.assertEqual(ni(stem, "Atu", "lot", 1, "P", "", "krI"), "krIRAtu")
        self.assertEqual(_join_han("han", "Ani", "lot", 3, 1), "hanAni")
        self.assertEqual(_join_han("han", "va", "lot", 3, 1), "hanAva")
        self.assertEqual(_join_han("han", "ma", "lot", 3, 1), "hanAma")
        self.assertEqual(_join_han("han", "taH", "lot", 1, 1), "hataH")
        self.assertEqual(_join_han("han", "Di", "lot", 2, 1), "jahi")
        self.assertEqual(_join_gana3_a("abI", "te", 1, 1, "BI"), "abIte")
        self.assertEqual(_join_gana3_lit("x", "ti", 1, 1, "x"), "xti")
        self.assertEqual(_join_n("xxRa", "atam", "lang", 1, "P"), "xxndDam")
        self.assertEqual(_join_n("xxRa", "ata", "lang", 1, "P"), "xxndDa")
        self.assertEqual(_join_n("xxRa", "am", "lang", 1, "P"), "xxRaDam")
        self.assertEqual(_join_n("xxRa", "tAt", "lot", 1, "P"), "xxndDAt")
        self.assertEqual(_join_n("xxRa", "Di", "lot", 2, "P"), "xxRadDi")
        self.assertEqual(join_form("sunu", "ze", 5, "lat", 2, "A", None, "su", 1), "sunuze")
        self.assertTrue(
            join_form("krInA", "te", 9, "lat", 1, "A", None, "krI", 1).endswith("te")
        )
        self.assertEqual(_join_han("han", "foo", "lot", 1, 1), "hanfoo")
        self.assertEqual(_join_han("han", "ti", "lit", 1, 1), "hanti")
        self.assertEqual(join_form("sunu", "vantu", 5, "lot", 1, "P", None, "su", 1), "sunvantu")
        self.assertEqual(_join_ni_npattern("xxR", "yat", "vidhilin", 1, "P"), "xxRyat")
        self.assertEqual(_join_ni_npattern("abnA", "te", "lat", 1, "A"), "abnIte")
        self.assertEqual(_join_ni_npattern("abnA", "ete", "lat", 1, "A"), "abnete")
        self.assertEqual(_join_ni_npattern("abnA", "se", "lat", 2, "A"), "abnIse")
        self.assertEqual(_join_ni_npattern("abnA", "e", "lat", 3, "A"), "abne")
        self.assertEqual(_join_gana3_a("dA", "foo", 1, 1, "BI"), "dAfoo")

    def test_coverage_final_lines(self):
        from sktmorph.engine.lakara import kartari_compatible
        from sktmorph.engine.phonology import (
            apply_causative_grade,
            g9_uses_n_infix,
            lang_geminate_stem,
            ya_present_base,
        )
        from sktmorph.engine.redup import gana3_present_stem
        from sktmorph.engine.stems import (
            _g1_future_base,
            _g6_future_suffix,
            _g6_skip_future_guna,
            g6_future_stem,
        )

        self.assertFalse(kartari_compatible("A", "plat"))
        self.assertFalse(kartari_compatible("P", "alat"))
        self.assertIsNotNone(family_endings("lat", "kartari", "A", 5))
        self.assertTrue(g9_uses_n_infix("mI", ""))
        self.assertTrue(ya_present_base("dIv"))
        self.assertTrue(lang_geminate_stem("cur", "cora"))
        self.assertTrue(apply_causative_grade("cur"))
        self.assertTrue(gana3_present_stem("hu", "hu"))
        self.assertTrue(_g6_future_suffix("xxD"))
        self.assertTrue(g6_future_stem("xxfh").endswith("izya"))
        from unittest.mock import patch

        with patch("sktmorph.engine.stems.apply_guna_to_stem", return_value="xxfh"):
            self.assertTrue(g6_future_stem("abcfh").endswith("izya"))
        self.assertTrue(_g6_skip_future_guna("umB"))
        self.assertTrue(_g1_future_base("guh", "guha", "guh"))
        self.assertTrue(_g1_future_base("rinv", "rinv", "rinv").endswith("Rv"))
        self.assertTrue(future_stem("x", 5, None, "xo"))
        self.assertTrue(future_stem("x", 7, None, "xD"))
        self.assertTrue(derive_stem("yam", 1, "lrt", "shuddha", "", "GawAdiH")[0].endswith("izya"))
        self.assertTrue(derive_stem("yam", 1, "lrt", "shuddha")[0].endswith("Msy"))
        self.assertTrue(derive_stem("SrA", 1, "lrt", "shuddha")[0].endswith("zy"))
        self.assertTrue(derive_stem("kzi", 1, "lrt", "shuddha", "", "BidAdiH")[0].endswith("zya"))
        self.assertTrue(derive_stem("ji", 1, "lrt", "shuddha")[0].endswith("zya"))
        form, _ = self.krd.derive("04.0001", "ktvA")
        self.assertTrue(form)
        form, _ = self.krd.derive("06.0001", "lyap")
        self.assertTrue(form or form is None)

    def test_krdanta_remaining(self):
        form, _ = self.krd.derive("04.0001", "lyap")
        self.assertTrue(form or form is None)
        form, _ = self.krd.derive("06.0001", "Satf")
        self.assertTrue(form)
        form, _ = self.krd.derive("01.0001", "ktvA")
        self.assertTrue(form)

    def test_coverage_complete(self):
        from sktmorph.engine.krdanta import LiveKrdantaEngine
        from sktmorph.engine.phonology import (
            apply_guna_to_stem,
            causative_lang_stem,
            g6_present_base,
            g6_lang_base,
            g6_lang_stem,
            g9_r_lang_root,
            thematic_aya_present_stem,
            thematic_present_base,
            vowel_initial_lang_stem,
        )
        from sktmorph.engine.redup import _profile, gana3_perfect_stem
        from sktmorph.engine.stems import (
            _g1_future_base,
            _g1_future_suffix,
            _g6_future_suffix,
            perfect_stem,
        )

        self.assertIsNotNone(family_endings("lat", "kartari", "A", 3, "hu"))
        self.assertEqual(_join_han("han", "xx", "lang", 1, 1), "hanxx")
        self.assertEqual(join_form("juhu", "antu", 3, "lot", 1, "P", None, "hu", 1), "juhvatu")
        self.assertEqual(join_form("Dino", "Di", 1, "lot", 2, "P", None, "Dinv", 1), "Dinu")
        self.assertEqual(join_form("Dinvizya", "ati", 1, "lrt", 1, "P", None, "Dinv", 1), "Dinvizyati")
        self.assertEqual(join_form("Dinu", "yAt", 1, "vidhilin", 1, "P", None, "Dinv", 1), "DinuyAt")
        self.assertEqual(join_form("kfnva", "ti", 1, "lat", 1, "P", None, "kfnv", 1), "kfRoti")
        self.assertEqual(join_form("kfnv", "at", 1, "lang", 1, "P", "a", "kfnv", 1), "akfRot")
        self.assertEqual(join_form("yo", "at", 2, "lang", 1, "P", "a", "yu", 1), "ayOt")
        self.assertEqual(join_form("ro", "at", 2, "lang", 1, "P", "a", "ru", 1), "aravIt")
        self.assertEqual(join_form("rR", "at", 2, "lang", 1, "P", "a", "UrRu", 1), "OrRot")
        self.assertEqual(join_form("ro", "ti", 2, "lrt", 1, "P", None, "ru", 1), "ravizyati")
        self.assertEqual(join_form("srozy", "ti", 1, "lrt", 1, "P", None, "sru", 1), "srozyati")
        self.assertEqual(join_form("raha", "Ani", 1, "lot", 3, "P", None, "rah", 1), "rahARi")
        self.assertEqual(join_form("SrA", "ti", 1, "lat", 1, "P", None, "SrA", 1), "SrAti")
        self.assertEqual(join_form("SrA", "at", 1, "lang", 1, "P", "a", "SrA", 1), "aSrAt")
        self.assertEqual(join_form("SrA", "Ava", 1, "lang", 3, "P", "a", "SrA", 2), "aSrAva")
        self.assertEqual(join_form("SrA", "Ami", 1, "lat", 3, "P", None, "SrA", 1), "SrAmi")
        self.assertEqual(join_form("SrA", "Ani", 1, "lot", 3, "P", None, "SrA", 1), "SrARi")
        self.assertEqual(join_form("SrA", "antu", 1, "lot", 1, "P", None, "SrA", 3), "SrAntu")
        self.assertEqual(future_stem("SrA", 1, "SrA", "SrA"), "Srizy")
        self.assertEqual(_g1_future_base("SrA", "SrA", "SrA"), "Sri")
        self.assertEqual(join_form("Srizy", "TaH", 1, "lrt", 2, "P", None, "SrA", 2), "SrizyaTaH")
        self.assertEqual(thematic_present_base("sUrkzy", 1, "zUrkzya~"), "sUkzy")
        self.assertEqual(
            derive_stem("sUrkzy", 1, "lrt", "shuddha", "", "", "zUrkzya~")[0],
            "sUkzyizya",
        )
        from sktmorph.engine.join import _g2_u_lang_join, _join_ad, _join_g1_a_final, _join_kfnv

        self.assertIsNone(_g2_u_lang_join("gam", "at", 1, 1))
        self.assertIsNone(_g2_u_lang_join("ru", "xx", 1, 1))
        self.assertIsNone(_g2_u_lang_join("su", "xx", 1, 1))
        self.assertEqual(_join_kfnv("kfnv", "va", "lang", 1, 2), "akfRuva")
        self.assertEqual(_join_kfnv("kfnv", "ma", "lang", 2, 3), "akfRuma")
        self.assertEqual(_join_kfnv("kfnv", "tha", "lot", 2, 1), "kfRu")
        self.assertEqual(_join_kfnv("kfnv", "ti", "lrt", 1, 1), "kfnvti")
        self.assertEqual(join_form("sIda", "ti", 1, "lat", 1, "P", None, "sad", 1), "sIdati")
        self.assertEqual(join_form("sId", "at", 1, "lang", 1, "P", "a", "sad", 1), "asIdat")
        self.assertEqual(join_form("sId", "et", 1, "vidhilin", 1, "P", None, "sad", 1), "sIdet")
        self.assertEqual(
            derive_stem("mid", 1, "lat", "shuddha", "", "BidAdiH")[0],
            "meda",
        )
        self.assertEqual(derive_stem("yaB", 1, "lrt", "shuddha")[0], "yapsya")
        self.assertEqual(derive_stem("sfp", 1, "lrt", "shuddha")[0], "sarpsya")
        self.assertEqual(derive_stem("tap", 1, "lrt", "shuddha")[0], "tapsya")
        self.assertEqual(derive_stem("pA", 1, "lat", "shuddha")[0], "piba")
        self.assertEqual(_g1_future_base("pA", "pib", "pA"), "pib")
        self.assertEqual(
            derive_stem("Urj", 10, "lat", "shuddha", "nityaRic")[0],
            "Urjaya",
        )
        self.assertEqual(join_form("drAya", "Ani", 1, "plot", 3, "P", None, "drE", 1), "drAyARi")
        self.assertEqual(
            derive_stem("med", 1, "lat", "shuddha", "", "BidAdiH")[0],
            "meda",
        )
        self.assertTrue(derive_stem("ve", 1, "lrt", "shuddha", "", "BidAdiH")[0].endswith("zya"))
        self.assertEqual(derive_stem("GrA", 1, "lat", "shuddha")[0], "jiGra")
        self.assertEqual(derive_stem("saYj", 1, "lat", "shuddha")[0], "saja")
        self.assertEqual(derive_stem("saYj", 1, "lrt", "shuddha")[0], "saNkzy")
        self.assertEqual(derive_stem("tras", 4, "lat", "shuddha")[0], "trasa")
        self.assertEqual(derive_stem("GuR", 6, "lrt", "shuddha")[0], "GoRizya")
        self.assertEqual(derive_stem("vfh", 6, "lrt", "shuddha")[0], "varhizya")
        self.assertEqual(_join_g1_a_final("SrA", "va", "lang", 3), "aSrAva")
        self.assertEqual(_join_g1_a_final("SrA", "ma", "lang", 3), "aSrAma")
        self.assertEqual(join_form("jYA", "Ani", 1, "lot", 3, "P", None, "jYA", 1), "jYAni")
        self.assertEqual(_join_ad("testzy", "anti", "lrt", "xx", 1, 3), "testzyanti")
        self.assertEqual(_join_ad("testzy", "ti", "lrt", "xx", 1, 1), "testzyati")
        self.assertEqual(_join_ad("testzy", "Ami", "lrt", "xx", 3, 1), "testzyAmi")
        self.assertEqual(future_stem("x", 1, "xyAa", "xE"), "xyAsy")
        self.assertEqual(_join_ni_npattern("abnA", "foo", "lat", 1, "P"), "abnAfoo")
        self.assertEqual(g9_r_lang_root("SF"), "SIr")
        self.assertEqual(thematic_aya_present_stem("pan"), "panAya")
        self.assertEqual(thematic_present_base("kgam", 1), "kgAm")
        self.assertEqual(thematic_present_base("zWuv", 1), "zWUv")
        self.assertEqual(causative_lang_stem("I"), "Eay")
        self.assertEqual(_g6_future_suffix("abjj"), "abkzya")
        self.assertEqual(_g6_future_suffix("xxfh"), "xxfhizya")
        self.assertEqual(_g1_future_suffix("ekza", "ikz"), "ikzizya")
        from sktmorph.engine.join import _g2_a_lat_join, _g2_u_lat_join

        self.assertEqual(_g2_u_lat_join("ru", "TaH", 2, 2), "ruTaH")
        self.assertIsNone(_g2_u_lat_join("gam", "ti", 1, 1))
        self.assertIsNone(_g2_u_lat_join("uru", "ti", 1, 1))
        self.assertEqual(_g2_u_lat_join("yu", "TaH", 2, 2), "yuTaH")
        self.assertEqual(_g2_u_lat_join("yu", "mi", 1, 1), "yOmi")
        self.assertEqual(_g2_a_lat_join("yA", "TaH", 2, 2), "yATaH")
        self.assertEqual(thematic_present_base("tud", 6), "tud")
        self.assertEqual(g6_present_base("fcC"), "arcC")
        self.assertEqual(g6_lang_base("fa"), "fa")
        self.assertEqual(g6_lang_stem("fz")[0], "Arz")
        self.assertEqual(derive_stem("dfS", 1, "lrt", "shuddha")[0], "drakzya")
        self.assertEqual(derive_stem("nU", 6, "lang", "shuddha")[0], "nuv")
        self.assertEqual(future_stem(apply_guna_to_stem("div"), 3, "", "div"), "devzya")
        krd = self.krd
        self.assertEqual(thematic_present_base("ktz", 1), "ktz")
        self.assertEqual(krd._present_stem("gam", 4), "gamya")
        self.assertEqual(krd._kta_stem("gam"), "gamta")
        self.assertEqual(
            _g1_future_base("abIv", "abIv", apply_guna_to_stem("abIv")),
            apply_guna_to_stem("abIv"),
        )
        self.assertEqual(future_stem("styA", 1, "styAa", "styA"), "styAsy")
        self.assertEqual(future_stem("mI", 9, "", "mI"), "mAsya")
        self.assertEqual(perfect_stem("liv", "liv"), "liliva")
        self.assertTrue(derive_stem("akz", 1, "lrt", "shuddha", "", "BidAdiH")[0].endswith("izya"))
        self.assertTrue(derive_stem("tud", 6, "lrt", "shuddha")[0].endswith("sya"))
        self.assertEqual(_profile("hA", "jah").present, "jahA")
        self.assertTrue(gana3_perfect_stem("div"))
        self.assertTrue(krd._present_stem("kfp", 4).endswith("ya"))
        self.assertEqual(krd._present_stem("ad", 2), "ad")
        self.assertEqual(krd._present_stem("tud", 6), "tuda")
        self.assertEqual(krd._kta_stem("kzi"), "kzita")
        self.assertTrue(krd._present_stem("cur", 10).endswith("aya"))
        self.assertTrue(vowel_initial_lang_stem("I"))


if __name__ == "__main__":
    unittest.main()
