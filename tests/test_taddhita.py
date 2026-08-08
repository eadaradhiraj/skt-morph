import json
import os
import shutil
import sqlite3
import tempfile
import unittest
from unittest.mock import patch

from sktmorph import cli
from sktmorph.morphology import SktMorph
from sktmorph.taddhita import (
    TaddhitaGenerator,
    derive_stem_rule,
    normalize_pratyaya,
    split_taddhita_stem,
)

VIBHAKTIS = [
    "prathamA", "dvitIyA", "tfIyA", "caturTI",
    "paYcamI", "zazWI", "saptamI", "samboDana",
]

# (pratipadika, pratyaya, expected_stem)
STEM_DERIVATION_CASES = [
    ("rAma", "tva", "rAmatva"),
    ("rAma", "tal", "rAmatala"),
    ("rAma", "matup", "rAmavat"),
    ("rAma", "vat", "rAmavat"),
    ("rAma", "mat", "rAmavat"),
    ("rAma", "mayat", "rAmamaya"),
    ("nara", "tva", "naratva"),
    ("nara", "matup", "naravat"),
    ("anna", "tva", "annatva"),
    ("anna", "mayat", "annamaya"),
    ("grAma", "tva", "grAmatva"),
    ("grAma", "tal", "grAmatala"),
    ("rAjan", "tva", "rAjatva"),
    ("rAjan", "matup", "rAjavat"),
    ("hari", "tva", "haritva"),
    ("hari", "matup", "harivat"),
    ("guRin", "tva", "guRitva"),
    ("senA", "tva", "senatva"),
    ("madhu", "tva", "madhutva"),
    ("kartf", "tva", "kartftva"),
    ("rAma", "ka", "rAmaka"),
    ("rAma", "yat", "rAmayata"),
    ("rAma", "Iya", "rAmIya"),
    ("rAma", "tA", "rAmatA"),
    ("rAma", "a", "rAma"),
    ("rAm", "a", "rAma"),
    ("senA", "Iya", "senIya"),
    ("Buddh", "Iya", "BuddhIya"),
    ("putra", "ini", "putrini"),
    ("rAma", "ana", "rAmAna"),
    ("rAma", "thak", "rAmIka"),
    ("bala", "itac", "balita"),
    ("deva", "itac", "devita"),
    ("rAjan", "thak", "rAjanIka"),
    ("budh", "itac", "budhita"),
    ("rAma", "Tya", "rAmAya"),
    ("deva", "Tyan", "devAya"),
    ("deva", "Ca", "devIya"),
    ("nara", "Ca", "narIya"),
    ("senA", "Tya", "senAya"),
]

# (pratipadika, pratyaya, linga, expected_stem, prathama_row)
GENERATION_CASES = [
    ("rAma", "tva", "nap", "rAmatva", ["rAmatvam", "rAmatve", "rAmatvAni"]),
    ("rAma", "tal", "nap", "rAmatala", ["rAmatalam", "rAmatale", "rAmatalAni"]),
    ("rAma", "mayat", "pum", "rAmamaya", ["rAmamayaH", "rAmamayO", "rAmamayAH"]),
    ("nara", "tva", "nap", "naratva", ["naratvam", "naratve", "naratvAni"]),
    ("nara", "matup", "pum", "naravat", ["naravAn", "naravaRtO", "naravaRtaH"]),
    ("anna", "mayat", "pum", "annamaya", ["annamayaH", "annamayO", "annamayAH"]),
    ("anna", "tva", "nap", "annatva", ["annatvam", "annatve", "annatvAni"]),
    ("grAma", "tal", "pum", "grAmatala", ["grAmatalaH", "grAmatalO", "grAmatalAH"]),
    ("grAma", "tva", "nap", "grAmatva", ["grAmatvam", "grAmatve", "grAmatvAni"]),
    ("rAjan", "tva", "nap", "rAjatva", ["rAjatvam", "rAjatve", "rAjatvAni"]),
    ("hari", "tva", "nap", "haritva", ["haritvam", "haritve", "haritvAni"]),
    ("hari", "matup", "pum", "harivat", ["harivAn", "harivaRtO", "harivaRtaH"]),
    ("senA", "tva", "nap", "senatva", ["senatvam", "senatve", "senatvAni"]),
    ("madhu", "tva", "nap", "madhutva", ["madhutvam", "madhutve", "madhutvAni"]),
    ("guRin", "tva", "nap", "guRitva", ["guRitvam", "guRitve", "guRitvAni"]),
]

# (declined_form, pratipadika, pratyaya, vibhakti, vacana)
ANALYZE_CASES = [
    ("rAmatvam", "rAma", "tva", "prathamA", 1),
    ("rAmatvam", "rAma", "tva", "dvitIyA", 1),
    ("rAmatalam", "rAma", "tal", "prathamA", 1),
    ("rAmamayaH", "rAma", "mayat", "prathamA", 1),
    ("naratvam", "nara", "tva", "prathamA", 1),
    ("naravAn", "nara", "matup", "prathamA", 1),
    ("annamayaH", "anna", "mayat", "prathamA", 1),
    ("grAmatalaH", "grAma", "tal", "prathamA", 1),
    ("rAjatvam", "rAjan", "tva", "prathamA", 1),
    ("haritvam", "hari", "tva", "prathamA", 1),
    ("rAmatvena", "rAma", "tva", "tfIyA", 1),
    ("rAmatvasya", "rAma", "tva", "zazWI", 1),
    ("rAmatve", "rAma", "tva", "saptamI", 1),
]

# (stem, expected_pair)
SPLIT_CASES = [
    ("rAmatva", ("rAma", "tva")),
    ("rAmatala", ("rAma", "tal")),
    ("rAmamaya", ("rAma", "mayat")),
    ("naravat", ("nara", "matup")),
    ("annamaya", ("anna", "mayat")),
    ("rAjatva", ("rAjan", "tva")),
    ("haritva", ("hari", "tva")),
    ("guRitva", ("guRin", "tva")),
    ("senatva", ("senA", "tva")),
    ("rAmaka", ("rAma", "ka")),
    ("rAmayata", ("rAma", "yat")),
    ("rAmIya", ("rAma", "Iya")),
    ("rAmatA", ("rAma", "tA")),
    ("putrini", ("putra", "ini")),
    ("rAmAna", ("rAma", "ana")),
]


class TestTaddhitaPratyayaNormalization(unittest.TestCase):
    def test_all_aliases_map_to_canonical(self):
        self.assertEqual(normalize_pratyaya("tva"), "tva")
        self.assertEqual(normalize_pratyaya("tal"), "tal")
        self.assertEqual(normalize_pratyaya("mayat"), "mayat")
        self.assertEqual(normalize_pratyaya("matup"), "matup")
        self.assertEqual(normalize_pratyaya("vat"), "matup")
        self.assertEqual(normalize_pratyaya("mat"), "matup")

    def test_unsupported_pratyaya_raises(self):
        with self.assertRaises(ValueError):
            normalize_pratyaya("unknown")
        with self.assertRaises(ValueError):
            normalize_pratyaya("")

    def test_ka_yat_supported(self):
        self.assertEqual(normalize_pratyaya("ka"), "ka")
        self.assertEqual(normalize_pratyaya("yat"), "yat")
        self.assertEqual(normalize_pratyaya("Iya"), "Iya")
        self.assertEqual(normalize_pratyaya("tA"), "tA")
        self.assertEqual(normalize_pratyaya("ini"), "ini")
        self.assertEqual(normalize_pratyaya("ana"), "ana")
        self.assertEqual(normalize_pratyaya("a"), "a")


class TestTaddhitaStemDerivation(unittest.TestCase):
    def test_stem_derivation_matrix(self):
        for pratipadika, pratyaya, expected in STEM_DERIVATION_CASES:
            with self.subTest(pratipadika=pratipadika, pratyaya=pratyaya):
                self.assertEqual(derive_stem_rule(pratipadika, pratyaya), expected)

    def test_empty_pratipadika_returns_none(self):
        self.assertIsNone(derive_stem_rule("", "tva"))


class TestTaddhitaStemSplitting(unittest.TestCase):
    def test_split_known_stems(self):
        for stem, expected_pair in SPLIT_CASES:
            with self.subTest(stem=stem):
                splits = split_taddhita_stem(stem)
                self.assertIn(expected_pair, splits)

    def test_split_does_not_split_short_strings(self):
        self.assertEqual(split_taddhita_stem("tva"), [])
        self.assertEqual(split_taddhita_stem("a"), [])

    def test_split_adds_an_candidate(self):
        splits = split_taddhita_stem("ruktva")
        self.assertIn(("rukan", "tva"), splits)

    def test_derive_stem_rule_unknown_canonical_returns_none(self):
        with patch("sktmorph.taddhita.normalize_pratyaya", return_value="bogus"):
            self.assertIsNone(derive_stem_rule("x", "bogus"))


class TestTaddhitaGenerator(unittest.TestCase):
    def setUp(self):
        self.gen = TaddhitaGenerator()
        self._temp_gens = []

    def _close_temp_gens(self):
        for gen in self._temp_gens:
            if gen._conn:
                gen._conn.close()
                gen._conn = None
        self._temp_gens.clear()

    def tearDown(self):
        self._close_temp_gens()

    def _make_temp_gen(self, db_path):
        gen = TaddhitaGenerator(db_path=db_path)
        self._temp_gens.append(gen)
        return gen

    def test_generation_matrix(self):
        for pratipadika, pratyaya, linga, stem, prathama in GENERATION_CASES:
            with self.subTest(pratipadika=pratipadika, pratyaya=pratyaya, linga=linga):
                result = self.gen.generate(pratipadika, pratyaya, linga)
                self.assertEqual(result["pratipadika"], pratipadika)
                self.assertEqual(result["pratyaya"], normalize_pratyaya(pratyaya))
                self.assertEqual(result["linga"], linga)
                self.assertEqual(result["stem"], stem)
                self.assertEqual(result["declension"]["prathamA"], prathama)

    def test_declension_has_all_vibhaktis(self):
        result = self.gen.generate("rAma", "tva", "nap")
        for vibhakti in VIBHAKTIS:
            with self.subTest(vibhakti=vibhakti):
                self.assertIn(vibhakti, result["declension"])
                self.assertEqual(len(result["declension"][vibhakti]), 3)

    def test_analyze_stem_round_trip(self):
        for pratipadika, pratyaya, linga, stem, _ in GENERATION_CASES:
            with self.subTest(stem=stem):
                matches = self.gen.analyze_stem(stem)
                self.assertTrue(
                    any(
                        self.gen.derive_stem(m["pratipadika"], m["pratyaya"]) == stem
                        for m in matches
                    ),
                    f"No verified match for stem {stem}",
                )

    def test_db_lookup_overrides_rules(self):
        tmp = tempfile.mkdtemp()
        db_path = os.path.join(tmp, "taddhitas.sqlite")
        try:
            conn = sqlite3.connect(db_path)
            conn.execute(
                "CREATE TABLE taddhitas (pratipadika TEXT, pratyaya TEXT, linga TEXT, stem_slp1 TEXT, source TEXT)"
            )
            conn.execute(
                "INSERT INTO taddhitas VALUES (?, ?, ?, ?, ?)",
                ("rAma", "tva", "nap", "customStem", "test"),
            )
            conn.commit()
            conn.close()

            gen = self._make_temp_gen(db_path)
            self.assertEqual(gen.derive_stem("rAma", "tva", "nap"), "customStem")
        finally:
            self._close_temp_gens()
            shutil.rmtree(tmp, ignore_errors=True)

    def test_db_analyze_returns_linga(self):
        tmp = tempfile.mkdtemp()
        db_path = os.path.join(tmp, "taddhitas.sqlite")
        try:
            conn = sqlite3.connect(db_path)
            conn.execute(
                "CREATE TABLE taddhitas (pratipadika TEXT, pratyaya TEXT, linga TEXT, stem_slp1 TEXT, source TEXT)"
            )
            conn.execute(
                "INSERT INTO taddhitas VALUES (?, ?, ?, ?, ?)",
                ("deva", "tva", "pum", "devatvamOnly", "test"),
            )
            conn.commit()
            conn.close()

            gen = self._make_temp_gen(db_path)
            matches = gen.analyze_stem("devatvamOnly")
            db_matches = [m for m in matches if m.get("linga") == "pum"]
            self.assertEqual(len(db_matches), 1)
            self.assertEqual(db_matches[0]["pratipadika"], "deva")

            conn = sqlite3.connect(db_path)
            conn.execute(
                "INSERT INTO taddhitas VALUES (?, ?, ?, ?, ?)",
                ("deva", "tva", "pum", "devatvamOnly", "dup"),
            )
            conn.commit()
            conn.close()
            dup_matches = gen.analyze_stem("devatvamOnly")
            self.assertEqual(len([m for m in dup_matches if m["pratipadika"] == "deva"]), 1)
        finally:
            self._close_temp_gens()
            shutil.rmtree(tmp, ignore_errors=True)

    def test_lookup_skips_duplicate_rule_match(self):
        matches = self.gen.analyze_stem("rAmatva")
        self.assertEqual(len([m for m in matches if m["pratipadika"] == "rAma"]), 1)

    def test_missing_db_falls_back_to_rules(self):
        with tempfile.TemporaryDirectory() as tmp:
            gen = TaddhitaGenerator(db_path=os.path.join(tmp, "missing.sqlite"))
            self.assertEqual(gen.derive_stem("rAma", "tva"), "rAmatva")

    def test_unsupported_pratyaya_raises(self):
        with self.assertRaises(ValueError):
            self.gen.generate("rAma", "unknown", "nap")

    def test_undeclinable_stem_raises(self):
        with self.assertRaises(NotImplementedError):
            self.gen.generate("vAc", "tva", "stri")

    def test_missing_stem_raises(self):
        with patch.object(self.gen, "derive_stem", return_value=None):
            with self.assertRaises(NotImplementedError):
                self.gen.generate("rAma", "tva", "nap")

    def test_generate_includes_prakriya(self):
        result = self.gen.generate("rAma", "tva", "nap", include_prakriya=True)
        self.assertIn("prakriya", result)
        self.assertGreater(len(result["prakriya"]), 2)
        self.assertEqual(result["prakriya"][0]["kind"], "taddhita")


class TestTaddhitaMorphologyIntegration(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.morph = SktMorph()

    def test_analyze_taddhita_cases(self):
        for word, prati, pratyaya, vibhakti, vacana in ANALYZE_CASES:
            with self.subTest(word=word, vibhakti=vibhakti):
                results = self.morph.analyze(word, allowed_types=["taddhita"])
                valid = [
                    r for r in results
                    if r.pratipadika == prati
                    and r.pratyaya == pratyaya
                    and r.vibhakti == vibhakti
                    and r.vacana == vacana
                ]
                self.assertTrue(len(valid) > 0, f"No taddhita parse for {word}")

    def test_analyze_taddhita_coexists_with_subanta(self):
        results = self.morph.analyze("rAmatvam")
        types = {r.word_type for r in results}
        self.assertIn("subanta", types)
        self.assertIn("taddhita", types)

    def test_analyze_filter_excludes_taddhita(self):
        results = self.morph.analyze("rAmatvam", allowed_types=["subanta"])
        self.assertTrue(all(r.word_type == "subanta" for r in results))
        self.assertTrue(len(results) > 0)

    def test_analyze_filter_taddhita_only(self):
        results = self.morph.analyze("rAmatvam", allowed_types=["taddhita"])
        self.assertTrue(all(r.word_type == "taddhita" for r in results))
        self.assertTrue(len(results) > 0)

    def test_generate_taddhita_api(self):
        result = self.morph.generate_taddhita("rAma", "tva", "nap", include_prakriya=True)
        self.assertEqual(result["stem"], "rAmatva")
        self.assertIn("declension", result)
        self.assertIn("prakriya", result)
        self.assertEqual(result["declension"]["prathamA"][0], "rAmatvam")

    def test_generate_all_pratyaya_types_via_api(self):
        cases = [
            ("rAma", "tva", "nap"),
            ("rAma", "tal", "nap"),
            ("nara", "matup", "pum"),
            ("anna", "mayat", "pum"),
        ]
        for prati, pratyaya, linga in cases:
            with self.subTest(pratyaya=pratyaya):
                result = self.morph.generate_taddhita(prati, pratyaya, linga)
                self.assertEqual(result["pratyaya"], normalize_pratyaya(pratyaya))
                self.assertEqual(len(result["declension"]), 8)

    def test_full_pipeline_generate_then_analyze(self):
        generated = self.morph.generate_taddhita("grAma", "tal", "pum")
        form = generated["declension"]["prathamA"][0]
        results = self.morph.analyze(form, allowed_types=["taddhita"])
        self.assertTrue(any(r.pratipadika == "grAma" and r.pratyaya == "tal" for r in results))


class TestTaddhitaCLI(unittest.TestCase):
    @patch("sys.argv", ["sktmorph", "generate_taddhita", "--pratipadika", "rAma", "--pratyaya", "tva", "--linga", "nap"])
    def test_cli_generate_taddhita(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            output = mock_print.call_args[0][0]
            data = json.loads(output)
            self.assertEqual(data["stem"], "rAmatva")
            self.assertEqual(data["declension"]["prathamA"], ["rAmatvam", "rAmatve", "rAmatvAni"])

    @patch("sys.argv", ["sktmorph", "generate_taddhita", "--pratipadika", "nara", "--pratyaya", "vat", "--linga", "pum"])
    def test_cli_generate_taddhita_vat_alias(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            data = json.loads(mock_print.call_args[0][0])
            self.assertEqual(data["pratyaya"], "matup")
            self.assertEqual(data["stem"], "naravat")

    @patch("sys.argv", ["sktmorph", "analyze", "rAmatvam", "--type", "taddhita"])
    def test_cli_analyze_taddhita_filter(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            self.assertTrue(mock_print.called)
            payloads = []
            for call in mock_print.call_args_list:
                try:
                    payloads.append(json.loads(call[0][0]))
                except (json.JSONDecodeError, IndexError):
                    pass
            self.assertTrue(len(payloads) > 0)
            self.assertTrue(all(p["word_type"] == "taddhita" for p in payloads))
            self.assertTrue(any(p["pratyaya"] == "tva" for p in payloads))
            self.assertTrue(any(p["pratipadika"] == "rAma" for p in payloads))

    @patch("sys.argv", ["sktmorph", "generate_taddhita", "--pratipadika", "vAc", "--pratyaya", "tva", "--linga", "stri"])
    def test_cli_generate_taddhita_error(self):
        with patch("builtins.print"):
            with self.assertRaises(SystemExit):
                cli.main()


if __name__ == "__main__":
    unittest.main()
