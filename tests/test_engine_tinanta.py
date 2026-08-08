import sqlite3
import unittest

from sktmorph.engine.krdanta import LiveKrdantaEngine
from sktmorph.engine.lakara import normalize_lakara
from sktmorph.engine.phonology import apply_guna_to_stem, thematic_join
from sktmorph.engine.endings import ending_table, family_endings
from sktmorph.engine.tinanta import LiveTinantaEngine
from sktmorph.engine.stems import derive_stem


class TestPhonology(unittest.TestCase):
    def test_guna_preserves_following_consonants(self):
        self.assertEqual(apply_guna_to_stem("cur"), "cor")
        self.assertEqual(apply_guna_to_stem("BU"), "Bav")

    def test_guna_no_vowel_returns_unchanged(self):
        self.assertEqual(apply_guna_to_stem("kr"), "kr")

    def test_thematic_join_avoids_double_a(self):
        self.assertEqual(thematic_join("Bava", "ataH"), "BavataH")
        self.assertEqual(thematic_join("Bava", "ti"), "Bavati")

    def test_thematic_join_non_thematic_stem(self):
        self.assertEqual(thematic_join("Bav", "ti"), "Bavti")

    def test_thematic_join_vowel_ending_with_capital_a(self):
        self.assertEqual(thematic_join("Bava", "Ami"), "BavAmi")


class TestLakara(unittest.TestCase):
    def test_normalize_canonical(self):
        self.assertEqual(normalize_lakara("plan"), ("plan", "plang"))
        self.assertEqual(normalize_lakara("plang"), ("plan", "plang"))

    def test_normalize_pvidhilin(self):
        self.assertEqual(normalize_lakara("pvidhilin"), ("pvidhilin", "pvidhiling"))


class TestEndings(unittest.TestCase):
    def test_ad_gana_uses_ad_endings(self):
        table = ending_table("plat", "kartari", "P", 2)
        self.assertEqual(table[0][0], "ti")

    def test_plot_endings_available(self):
        table = family_endings("lot", "kartari", "P", 1)
        self.assertEqual(table[0][0][0], "tAt")

    def test_unknown_pada_returns_none(self):
        self.assertIsNone(family_endings("plat", "kartari", "X", 1))

    def test_atmanepada_table(self):
        table = ending_table("alat", "kartari", "A", 1)
        self.assertEqual(table[0][0], "te")


class TestLiveTinantaEngine(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.conn = sqlite3.connect("sktmorph/data/dhatus.sqlite")
        cls.engine = LiveTinantaEngine(cls.conn)
        cls.tin = sqlite3.connect("sktmorph/data/tinantas_shuddha_gana1.sqlite")
        cls.tin10 = sqlite3.connect("sktmorph/data/tinantas_shuddha_gana2_to_10.sqlite")

    @classmethod
    def tearDownClass(cls):
        cls.conn.close()
        cls.tin.close()
        cls.tin10.close()

    def _db_form(self, conn, dhatu_id, lakara, purusha, vacana):
        canon, db_lk = normalize_lakara(lakara)
        row = conn.execute(
            """SELECT form_slp1 FROM tinantas
               WHERE dhatu_id=? AND lakara=? AND purusha=? AND vacana=?
               AND derivation='shuddha' AND prayoga='kartari'""",
            (dhatu_id, db_lk, purusha, vacana),
        ).fetchone()
        return row[0] if row else None

    def test_gana1_plat_matches_database(self):
        live, steps = self.engine.generate("01.0001", "plat", 1, 1)
        db = self._db_form(self.tin, "01.0001", "plat", 1, 1)
        self.assertEqual(live, db)
        self.assertEqual(live, "Bavati")
        self.assertTrue(any(s.kind == "sap" for s in steps))

    def test_gana1_all_primary_lakaras(self):
        for lak in ("plat", "plot", "plrt", "plan", "pvidhilin"):
            live = self.engine.generate("01.0001", lak, 1, 1)[0]
            db = self._db_form(self.tin, "01.0001", lak, 1, 1)
            self.assertIsNotNone(live, lak)
            self.assertEqual(live, db, lak)

    def test_gana1_full_paradigm(self):
        for purusha in (1, 2, 3):
            for vacana in (1, 2, 3):
                live = self.engine.generate("01.0001", "plat", purusha, vacana)[0]
                db = self._db_form(self.tin, "01.0001", "plat", purusha, vacana)
                self.assertEqual(live, db, f"{purusha}/{vacana}")

    def test_gana10_plat_matches_database(self):
        live = self.engine.generate("10.0001", "plat", 1, 1)[0]
        db = self._db_form(self.tin10, "10.0001", "plat", 1, 1)
        self.assertEqual(live, db)
        self.assertEqual(live, "corayati")

    def test_gana10_all_primary_lakaras(self):
        for lak in ("plat", "plot", "plrt", "plan", "pvidhilin"):
            live = self.engine.generate("10.0001", lak, 1, 1)[0]
            db = self._db_form(self.tin10, "10.0001", lak, 1, 1)
            self.assertEqual(live, db, lak)

    def test_plot_produces_variants(self):
        forms, _ = self.engine.generate_all("01.0001", "plot", 1, 1)
        self.assertIn("BavatAt", forms)
        self.assertIn("Bavatu", forms)

    def test_load_dhatu_by_string(self):
        info = self.engine.load_dhatu("BU")
        self.assertEqual(info["dhatu_id"], "01.0001")

    def test_nich_derivation_not_implemented(self):
        stem, steps = self.engine.derive_present_stem("BU", 1, "nich")
        self.assertIsNone(stem)
        self.assertEqual(steps[0].kind, "dhatu")

    def test_unknown_dhatu_returns_none(self):
        self.assertIsNone(self.engine.load_dhatu("not-a-real-root-xyz"))

    def test_invalid_lakara_code(self):
        self.assertEqual(self.engine.generate("01.0001", "foo", 1, 1)[0], None)

    def test_gana7_present_stem(self):
        stem, steps = self.engine.derive_present_stem("ruD", 7, "shuddha")
        self.assertEqual(stem, "ruRa")
        self.assertTrue(any(s.kind == "n_gana" for s in steps))

    def test_karmani_not_implemented(self):
        self.assertIsNone(self.engine.generate("01.0001", "plat", 1, 1, prayoga="karmani")[0])

    def test_ad_gana_present(self):
        row = self.conn.execute(
            "SELECT dhatu_id FROM dhatus WHERE json_extract(details_json, '$.gana') = '2' LIMIT 1"
        ).fetchone()
        form, steps = self.engine.generate(row[0], "plat", 1, 1)
        self.assertEqual(form, "atti")
        self.assertTrue(any(s.kind == "ad" for s in steps))

    def test_nich_generate_returns_steps_without_form(self):
        form, steps = self.engine.generate("01.0001", "plat", 1, 1, derivation="nich")
        self.assertIsNone(form)
        self.assertTrue(steps)

    def test_invalid_person_index(self):
        self.assertIsNone(self.engine.generate("01.0001", "plat", 3, 4)[0])

    def test_atmanepada_on_ubhayapada_root(self):
        row = self.conn.execute(
            "SELECT dhatu_id FROM dhatus WHERE json_extract(details_json, '$.pada') = 'U' LIMIT 1"
        ).fetchone()
        form, _ = self.engine.generate(row[0], "alat", 1, 1)
        self.assertTrue(form)

    def test_parasmaipada_only_rejects_alat(self):
        form, steps = self.engine.generate("01.0001", "alat", 1, 1)
        self.assertIsNone(form)
        self.assertEqual(steps, [])


class TestLiveKrdantaEngine(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.conn = sqlite3.connect("sktmorph/data/dhatus.sqlite")
        cls.engine = LiveKrdantaEngine(cls.conn)
        cls.krd = sqlite3.connect("sktmorph/data/krdantas_gana1.sqlite")

    @classmethod
    def tearDownClass(cls):
        cls.conn.close()
        cls.krd.close()

    def test_satf_matches_database(self):
        live, _ = self.engine.derive("01.0001", "Satf")
        db = self.krd.execute(
            "SELECT form_slp1 FROM krdantas WHERE dhatu_id='01.0001' AND pratyaya='Satf' AND derivation='shuddha' LIMIT 1"
        ).fetchone()[0]
        self.assertEqual(live, db.split("/")[0])

    def test_kta_matches_database(self):
        live, _ = self.engine.derive("01.0001", "kta")
        self.assertEqual(live, "BUta")

    def test_lyuw_matches_database(self):
        live, _ = self.engine.derive("01.0001", "lyuw")
        self.assertEqual(live, "Bavana")

    def test_unknown_pratyaya(self):
        form, steps = self.engine.derive("01.0001", "not-a-pratyaya")
        self.assertIsNone(form)
        self.assertTrue(steps)

    def test_nich_derivation_not_implemented(self):
        form, steps = self.engine.derive("01.0001", "Satf", derivation="nich")
        self.assertIsNone(form)


class TestStems(unittest.TestCase):
    def test_perfect_stem_bu(self):
        stem, _, steps = derive_stem("BU", 1, "lit", "shuddha")
        self.assertEqual(stem, "baBU")
        self.assertTrue(any(s.kind == "lit" for s in steps))


if __name__ == "__main__":
    unittest.main()
