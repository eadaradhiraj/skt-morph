import unittest
from unittest.mock import patch, MagicMock
import sys
import runpy
import warnings
from sktmorph.morphology import SktMorph, apply_forward_sandhi
from sktmorph import cli

class TestSktMorph(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.morph = SktMorph()

    def test_forward_sandhi_vowels(self):
        self.assertEqual(apply_forward_sandhi('pra', 'aBavat'), 'prABavat')
        self.assertEqual(apply_forward_sandhi('pra', 'iRvat'), 'preRvat')
        self.assertEqual(apply_forward_sandhi('vi', 'Akaroti'), 'vyAkaroti')
        self.assertEqual(apply_forward_sandhi('anu', 'aBavat'), 'anvaBavat')
        self.assertEqual(apply_forward_sandhi('ud', 'harati'), 'uddharati')

    def test_forward_sandhi_edge_cases(self):
        self.assertEqual(apply_forward_sandhi('', 'Bavati'), 'Bavati')
        self.assertEqual(apply_forward_sandhi('anu', 'eti'), 'anveti')
        self.assertEqual(apply_forward_sandhi('sam', 'karoti'), 'saMkaroti')
        self.assertEqual(apply_forward_sandhi('ud', 'gacCati'), 'udgacCati') 
        self.assertEqual(apply_forward_sandhi('prati', 'ikzate'), 'pratIkzate')
        self.assertEqual(apply_forward_sandhi('su', 'uktam'), 'sUktam')
        self.assertEqual(apply_forward_sandhi('ud', 'padyate'), 'utpadyate')
        
        self.assertEqual(apply_forward_sandhi('puras', 'gamanam'), 'purogamanam')
        self.assertEqual(apply_forward_sandhi('nis', 'kAmati'), 'nizkAmati')
        self.assertEqual(apply_forward_sandhi('nis', 'vahamAna'), 'nirvahamARa')
        
        self.assertEqual(apply_forward_sandhi('puras', 'atra'), 'purotra')
        self.assertEqual(apply_forward_sandhi('puras', 'uvAca'), 'purauvAca')
        self.assertEqual(apply_forward_sandhi('puras', 'carati'), 'puraScarati')
        self.assertEqual(apply_forward_sandhi('puras', 'wIkatI'), 'purazwIkatI')
        self.assertEqual(apply_forward_sandhi('puras', 'karoti'), 'puraHkaroti')


    def test_tuk_agama_sandhi(self):
        # Test Forward Sandhi
        self.assertEqual(apply_forward_sandhi("pra", "CAdya"), "pracCAdya")
        self.assertEqual(apply_forward_sandhi("A", "CAdya"), "AcCAdya")
        
        # Test Reverse Analyzer
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        mock_cursor.fetchall.return_value = [{"form_slp1": "-CAdya", "dhatu_id": "10.0370", "derivation": "shuddha", "pratyaya": "lyap", "details_json": None}]
        with patch.object(self.morph, "krdanta_conns", [mock_conn]):
            res = self.morph.analyze("pracCAdya")
            valid = [r for r in res if r.word_type == "krdanta" and "pra" in r.prefixes]
            self.assertTrue(len(valid) > 0)

    def test_missing_database(self):
        with self.assertRaises(FileNotFoundError):
            SktMorph(db_dir="fake_path")

    def test_analyzer_base_verb(self):
        res = self.morph.analyze('Bavati')
        self.assertTrue(len(res) > 0)
        valid = [r for r in res if r.prefixes == [] and r.dhatu == '01.0001' and r.word_type == 'tinanta']
        self.assertTrue(len(valid) > 0)

    def test_analyzer_single_prefix(self):
        res = self.morph.analyze('praBavati')
        valid = [r for r in res if r.prefixes == ['pra'] and r.dhatu == '01.0001']
        self.assertTrue(len(valid) > 0)

    def test_analyzer_krdanta(self):
        res = self.morph.analyze('Bavanam')
        valid = [r for r in res if r.word_type == 'krdanta' and r.pratyaya == 'lyuw']
        self.assertTrue(len(valid) > 0)
        
    def test_lyap_prefix_analyzer(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        mock_cursor.fetchall.return_value = [{"form_slp1": "-yujya", "dhatu_id": "07.0007", "derivation": "shuddha", "pratyaya": "lyap", "details_json": None}]
        with patch.object(self.morph, "krdanta_conns", [mock_conn]):
            res = self.morph.analyze("upayujya")
            valid = [r for r in res if r.word_type == "krdanta" and "upa" in r.prefixes]
            self.assertTrue(len(valid) > 0)

    def test_strict_forward_validation(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        
        def fake_fetchall():
            args = mock_cursor.execute.call_args
            if args and args[0][1][0] == "vahamAna":
                return [{"form_slp1": "vahamAna", "dhatu_id": "01.1159", "derivation": "shuddha", "pratyaya": "SAnac", "details_json": None}]
            return []
            
        mock_cursor.fetchall.side_effect = fake_fetchall
        with patch.object(self.morph, "krdanta_conns", [mock_conn]):
            res1 = self.morph.analyze("nirvahamAna")
            valid1 = [r for r in res1 if r.word_type == "krdanta"]
            self.assertEqual(len(valid1), 0)
            
            res2 = self.morph.analyze("nirvahamARa")
            valid2 = [r for r in res2 if r.word_type == "krdanta" and "nis" in r.prefixes]
            self.assertTrue(len(valid2) > 0)

    def test_pada_restrictions(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        
        def fake_fetchall():
            args = mock_cursor.execute.call_args
            if args and args[0][1][0] == "krIRAti":
                return [{"form_slp1": "krIRAti", "dhatu_id": "09.0001", "derivation": "shuddha", "prayoga": "kartari", "lakara": "plat", "purusha": 1, "vacana": 1, "details_json": None}]
            if args and args[0][1][0] == "krIRIte":
                return [{"form_slp1": "krIRIte", "dhatu_id": "09.0001", "derivation": "shuddha", "prayoga": "kartari", "lakara": "alat", "purusha": 1, "vacana": 1, "details_json": None}]
            return []
            
        mock_cursor.fetchall.side_effect = fake_fetchall
        with patch.object(self.morph, "tinanta_conns", [mock_conn]):
            res_bad = self.morph.analyze("vikrIRAti")
            self.assertEqual(len([r for r in res_bad if r.word_type == "tinanta"]), 0)
            
            res_good = self.morph.analyze("vikrIRIte")
            self.assertTrue(len([r for r in res_good if r.word_type == "tinanta"]) > 0)

    def test_analyzer_subanta(self):
        res = self.morph.analyze('BavadBiH')
        valid = [r for r in res if r.word_type == 'subanta' and r.pratipadika == 'Bavat']
        self.assertTrue(len(valid) > 0)

    def test_analyzer_sarvanama(self):
        res = self.morph.analyze("aham")
        valid = [r for r in res if r.word_type == "sarvanama" and r.pratipadika == "asmad"]
        self.assertTrue(len(valid) > 0)

    def test_adhas_prefix(self):
        res_full = self.morph.analyze("aDogamanam")
        self.assertTrue(any(r.word_type == "krdanta" and "aDas" in r.prefixes for r in res_full))
        
        res_bare = self.morph.analyze("aDogamana")
        self.assertTrue(any(r.word_type == "krdanta" and "aDas" in r.prefixes for r in res_bare))
        
        self.assertEqual(apply_forward_sandhi("aDas", "gamanam"), "aDogamanam")

    def test_missing_dhatu_details_tinanta(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        def fake_fetchall():
            args = mock_cursor.execute.call_args
            if args and args[0][1][0] == "fakeBavati":
                return [{"form_slp1": "fakeBavati", "dhatu_id": "99.9999", "derivation": "shuddha", "prayoga": "kartari", "lakara": "plat", "purusha": 1, "vacana": 1, "pratyaya": None, "details_json": None}]
            return []
        mock_cursor.fetchall.side_effect = fake_fetchall
        with patch.object(self.morph, "tinanta_conns", [mock_conn]):
            res = self.morph.analyze("fakeBavati")
            self.assertIsNone(res[0].dhatu_details)

    def test_missing_dhatu_details_krdanta(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        def fake_fetchall():
            args = mock_cursor.execute.call_args
            if args and args[0][1][0] == "fakeBavanam":
                return [{"form_slp1": "fakeBavanam", "dhatu_id": "99.9999", "derivation": "shuddha", "prayoga": None, "lakara": None, "purusha": None, "vacana": None, "pratyaya": "lyuw", "details_json": None}]
            return []
        mock_cursor.fetchall.side_effect = fake_fetchall
        with patch.object(self.morph, "tinanta_conns", [mock_conn]), patch.object(self.morph, "krdanta_conns", [mock_conn]):
            res = self.morph.analyze("fakeBavanam")
            self.assertIsNone(res[0].dhatu_details)

    def test_resolve_dhatu_ids_success(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        mock_cursor.fetchall.return_value = [{'dhatu_id': '01.0001'}]
        with patch.object(self.morph, 'conn_dhatus', mock_conn):
            ids = self.morph.resolve_dhatu_ids('BU')
            self.assertEqual(ids, ['01.0001'])

    @patch.dict('sys.modules', {'indic_transliteration': None})
    def test_resolve_dhatu_ids_import_error(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        mock_cursor.fetchall.return_value = [{'dhatu_id': '01.0001'}]
        with patch.object(self.morph, 'conn_dhatus', mock_conn):
            ids = self.morph.resolve_dhatu_ids('BU')
            self.assertEqual(ids, ['01.0001'])

    def test_operational_errors(self):
        import sqlite3
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        mock_cursor.execute.side_effect = sqlite3.OperationalError("Mock Error")
        
        with patch.object(self.morph, 'tinanta_conns', [mock_conn]), patch.object(self.morph, 'krdanta_conns', [mock_conn]):
            self.assertEqual(self.morph.generate_tinanta("01.0001", "plat", 1, 1), [])
            self.assertEqual(self.morph.generate_krdanta("01.0001", "lyuw"), [])
            res = self.morph.analyze("fakeWord")
            self.assertFalse(any(r.word_type in ["tinanta", "krdanta"] for r in res))
            # Trigger the OperationalError inside the Subanta-to-Krdanta bridge
            self.morph.analyze("AhvayamAnena")

    def test_generator_tinanta(self):
        forms = self.morph.generate_tinanta('01.0001', 'plat', 1, 1, prefixes=['pra'])
        self.assertIn('praBavati', forms)

    def test_generator_tinanta_edge_cases(self):
        self.assertEqual(self.morph.generate_tinanta('99.9999', 'plat', 1, 1), [])
        
    def test_generator_krdanta(self):
        forms = self.morph.generate_krdanta('01.0001', 'lyuw', prefixes=['pra'])
        self.assertIn('praBavaRam', forms)

    def test_generator_krdanta_edge_cases(self):
        self.assertEqual(self.morph.generate_krdanta('99.9999', 'lyuw'), [])
        forms = self.morph.generate_krdanta('01.0001', 'lyuw')
        self.assertIn('Bavanam', forms)

    def test_generator_split_before_prefix(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        mock_cursor.fetchall.return_value = [{"form_slp1": "Bavanam,BAvana;BUtvA"}]
        with patch.object(self.morph, 'krdanta_conns', [mock_conn]):
            forms = self.morph.generate_krdanta("01.0001", "lyuw", prefixes=["anu"])
            self.assertEqual(forms, sorted(["anuBAvana", "anuBavanam", "anuBUtvA"]))

    def test_lyap_prefix_generator(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        mock_cursor.fetchall.return_value = [{"form_slp1": "-yujya"}]
        with patch.object(self.morph, "krdanta_conns", [mock_conn]):
            forms = self.morph.generate_krdanta("07.0007", "lyap", prefixes=["upa"])
            self.assertEqual(forms, ["upayujya"])

    def test_generate_sarvanama(self):
        res = self.morph.generate_sarvanama('tad', 'pum')
        self.assertIn('prathamA', res)



    def test_reverse_satva_analysis(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        mock_cursor.fetchall.return_value = [{"form_slp1": "secaye", "dhatu_id": "06.0140", "derivation": "shuddha", "prayoga": "kartari", "lakara": "plat", "purusha": 1, "vacana": 1, "pratyaya": None, "details_json": None}]
        with patch.object(self.morph, "tinanta_conns", [mock_conn]):
            res = self.morph.analyze("aBizecaye")
            valid = [r for r in res if r.word_type == "tinanta" and "aBi" in r.prefixes]
            self.assertTrue(len(valid) > 0)


    def test_complex_multi_prefix(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        
        def fake_fetchall():
            args = mock_cursor.execute.call_args
            if args and args[0][1][0] == "IkzeTAH":
                return [{"form_slp1": "IkzeTAH", "dhatu_id": "01.0016", "derivation": "shuddha", "prayoga": "kartari", "lakara": "vidhilin", "purusha": 2, "vacana": 1, "details_json": None}]
            return []
            
        mock_cursor.fetchall.side_effect = fake_fetchall
        with patch.object(self.morph, "tinanta_conns", [mock_conn]):
            res = self.morph.analyze("pratyudIkzeTAH")
            valid = [r for r in res if r.word_type == "tinanta" and r.prefixes == ["prati", "ud"]]
            self.assertTrue(len(valid) > 0)


    def test_infinite_loop_prevention(self):
        # AhvayamAnena caused an infinite loop due to ("A", "A", "A") returning the same string
        res = self.morph.analyze("AhvayamAnena")
        valid = [r for r in res if r.word_type == "subanta"]
        self.assertTrue(len(valid) > 0)


    def test_max_prefixes_limit(self):
        # Trigger the max prefix limit (>=4) anti-freeze block
        # vi + ati + vi + ati + karoti (Depth 4+)
        res = self.morph.analyze("vyativyativyatikaroti")
        self.assertTrue(len(res) == 0 or len(res) > 0) # Just ensures it executes without hanging


    def test_declined_krdanta_analysis(self):
        # AhvayamAnena -> A + hve + SAnac (Declined in 3rd Case)
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        
        def fake_fetchall():
            args = mock_cursor.execute.call_args
            if args and args[0][1][0] == "hvayamAna":
                return [{"form_slp1": "hvayamAna", "dhatu_id": "01.1143", "derivation": "shuddha", "pratyaya": "SAnac", "details_json": None}]
            return []
            
        mock_cursor.fetchall.side_effect = fake_fetchall
        with patch.object(self.morph, "krdanta_conns", [mock_conn]):
            res = self.morph.analyze("AhvayamAnena")
            # Must identify as Krdanta, find the Dhatu, AND know the declension case!
            valid = [r for r in res if r.word_type == "krdanta" and r.dhatu == "01.1143" and r.vibhakti == "tfIyA"]
            self.assertTrue(len(valid) > 0)

class TestCLI(unittest.TestCase):
    @patch('sys.argv', ['sktmorph', 'analyze', 'praBavati'])
    def test_cli_analyze(self):
        with patch('builtins.print'):
            cli.main()

    @patch('sys.argv', ['sktmorph', 'analyze', 'fakeWordXyz'])
    def test_cli_analyze_not_found(self):
        with patch('builtins.print'):
            cli.main()

    @patch('sys.argv', ['sktmorph', 'generate_verb', '--dhatu', '01.0001', '--lakara', 'plat', '--purusha', '1', '--vacana', '1'])
    def test_cli_generate_verb(self):
        with patch('builtins.print'):
            cli.main()
            
    @patch('sys.argv', ['sktmorph', 'generate_krdanta', '--dhatu', '01.0001', '--pratyaya', 'lyuw', '--prefixes', 'pra'])
    def test_cli_generate_krdanta(self):
        with patch('builtins.print'):
            cli.main()

    @patch('sys.argv', ['sktmorph', 'generate_noun', '--base', 'manas', '--linga', 'nap'])
    def test_cli_generate_noun(self):
        with patch('builtins.print'):
            cli.main()

    @patch('sys.argv', ['sktmorph', 'generate_noun', '--base', 'vAc', '--linga', 'stri'])
    @patch('sktmorph.cli.SktMorph.generate_subanta')
    def test_cli_generate_noun_error(self, mock_gen):
        mock_gen.side_effect = NotImplementedError("Noun Error")
        with patch('builtins.print'):
            with self.assertRaises(SystemExit):
                cli.main()

    @patch('sys.argv', ['sktmorph', 'generate_pronoun', '--base', 'tad', '--linga', 'pum'])
    def test_cli_generate_pronoun(self):
        with patch('builtins.print'):
            cli.main()

    @patch('sys.argv', ['sktmorph', 'generate_pronoun', '--base', 'tad', '--linga', 'pum'])
    @patch('sktmorph.cli.SktMorph.generate_sarvanama')
    def test_cli_generate_pronoun_error(self, mock_gen):
        mock_gen.side_effect = NotImplementedError("Pronoun Error")
        with patch('builtins.print'):
            with self.assertRaises(SystemExit):
                cli.main()

    @patch('sys.argv', ['sktmorph', 'analyze', 'praBavati'])
    @patch('sktmorph.cli.SktMorph')
    def test_cli_db_error(self, mock_sktmorph):
        mock_sktmorph.side_effect = FileNotFoundError("DB Missing")
        with patch('builtins.print'):
            with self.assertRaises(SystemExit):
                cli.main()

    @patch('sys.argv', ['sktmorph'])
    def test_cli_no_args(self):
        with patch('argparse.ArgumentParser.print_help'):
            cli.main()

    @patch('sys.argv', ['sktmorph', 'analyze', 'praBavati'])
    def test_module_executions(self):
        with warnings.catch_warnings():
            warnings.simplefilter("ignore", RuntimeWarning)
            with patch('builtins.print'):
                runpy.run_module('sktmorph.cli', run_name='__main__')
                runpy.run_module('sktmorph.__main__', run_name='__main__')

if __name__ == '__main__':
    unittest.main()
