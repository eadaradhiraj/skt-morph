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


    def test_gati_prefixes(self):
        # Test Analysis Stripping
        res = self.morph.analyze("purogamanam")
        valid =[r for r in res if r.word_type == "krdanta" and "puras" in r.prefixes]
        self.assertTrue(len(valid) > 0)
        
        # Test Generation Sandhi
        self.assertEqual(apply_forward_sandhi("puras", "gamanam"), "purogamanam")
        self.assertEqual(apply_forward_sandhi("puras", "carati"), "puraScarati")
        self.assertEqual(apply_forward_sandhi("puras", "wIkatI"), "purazwIkatI")

    def test_missing_database(self):
        with self.assertRaises(FileNotFoundError):
            SktMorph(db_dir="fake_path")

    def test_analyzer_base_verb(self):
        res = self.morph.analyze('Bavati')
        self.assertTrue(len(res) > 0)
        valid =[r for r in res if r.prefixes == [] and r.dhatu == '01.0001' and r.word_type == 'tinanta']
        self.assertTrue(len(valid) > 0)

    def test_analyzer_single_prefix(self):
        res = self.morph.analyze('praBavati')
        valid =[r for r in res if r.prefixes == ['pra'] and r.dhatu == '01.0001']
        self.assertTrue(len(valid) > 0)

    def test_analyzer_krdanta(self):
        res = self.morph.analyze('Bavanam')
        valid =[r for r in res if r.word_type == 'krdanta' and r.pratyaya == 'lyuw']
        self.assertTrue(len(valid) > 0)
        
    def test_analyzer_subanta(self):
        res = self.morph.analyze('BavadBiH')
        valid =[r for r in res if r.word_type == 'subanta' and r.pratipadika == 'Bavat']
        self.assertTrue(len(valid) > 0)

    def test_analyzer_sarvanama(self):
        res = self.morph.analyze("aham")
        valid =[r for r in res if r.word_type == "sarvanama" and r.pratipadika == "asmad"]
        self.assertTrue(len(valid) > 0)

    def test_missing_dhatu_details_tinanta(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        mock_cursor.fetchall.side_effect = [[{'form_slp1': 'fakeBavati', 'dhatu_id': '99.9999', 'derivation': 'shuddha', 'prayoga': 'kartari', 'lakara': 'plat', 'purusha': 1, 'vacana': 1, 'pratyaya': None, 'details_json': None}],[]]
        with patch.object(self.morph, 'tinanta_conns', [mock_conn]):
            res = self.morph.analyze('fakeBavati')
            self.assertIsNone(res[0].dhatu_details)

    def test_missing_dhatu_details_krdanta(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        # Tinanta returns empty, Krdanta returns the mocked row
        mock_cursor.fetchall.side_effect = [[],[{'form_slp1': 'fakeBavanam', 'dhatu_id': '99.9999', 'derivation': 'shuddha', 'prayoga': None, 'lakara': None, 'purusha': None, 'vacana': None, 'pratyaya': 'lyuw', 'details_json': None}]]
        with patch.object(self.morph, 'tinanta_conns', [mock_conn]), patch.object(self.morph, 'krdanta_conns', [mock_conn]):
            res = self.morph.analyze('fakeBavanam')
            self.assertIsNone(res[0].dhatu_details)

    def test_resolve_dhatu_ids_success(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        mock_cursor.fetchall.return_value =[{'dhatu_id': '01.0001'}]
        with patch.object(self.morph, 'conn_dhatus', mock_conn):
            ids = self.morph.resolve_dhatu_ids('BU')
            self.assertEqual(ids, ['01.0001'])

    @patch.dict('sys.modules', {'indic_transliteration': None})
    def test_resolve_dhatu_ids_import_error(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        mock_cursor.fetchall.return_value =[{'dhatu_id': '01.0001'}]
        with patch.object(self.morph, 'conn_dhatus', mock_conn):
            ids = self.morph.resolve_dhatu_ids('BU')
            self.assertEqual(ids,['01.0001'])

    def test_operational_errors(self):
        import sqlite3
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        mock_cursor.execute.side_effect = sqlite3.OperationalError("Mock Error")
        
        with patch.object(self.morph, 'tinanta_conns',[mock_conn]), patch.object(self.morph, 'krdanta_conns',[mock_conn]):
            self.assertEqual(self.morph.generate_tinanta("01.0001", "plat", 1, 1),[])
            self.assertEqual(self.morph.generate_krdanta("01.0001", "lyuw"),[])
            res = self.morph.analyze("fakeWord")
            self.assertFalse(any(r.word_type in["tinanta", "krdanta"] for r in res))

    def test_generator_tinanta(self):
        forms = self.morph.generate_tinanta('01.0001', 'plat', 1, 1, prefixes=['pra'])
        self.assertIn('praBavati', forms)

    def test_generator_tinanta_edge_cases(self):
        self.assertEqual(self.morph.generate_tinanta('99.9999', 'plat', 1, 1),[])
        
    def test_generator_krdanta(self):
        forms = self.morph.generate_krdanta('01.0001', 'lyuw', prefixes=['pra'])
        self.assertIn('praBavanam', forms)

    def test_generator_krdanta_edge_cases(self):
        self.assertEqual(self.morph.generate_krdanta('99.9999', 'lyuw'),[])
        forms = self.morph.generate_krdanta('01.0001', 'lyuw')
        self.assertIn('Bavanam', forms)

    def test_generator_split_before_prefix(self):
        mock_conn = MagicMock()
        mock_cursor = MagicMock()
        mock_conn.cursor.return_value = mock_cursor
        mock_cursor.fetchall.return_value =[{"form_slp1": "Bavanam,BAvana;BUtvA"}]
        with patch.object(self.morph, 'krdanta_conns', [mock_conn]):
            forms = self.morph.generate_krdanta("01.0001", "lyuw", prefixes=["anu"])
            self.assertEqual(forms, sorted(["anuBAvana", "anuBavanam", "anuBUtvA"]))

    def test_generate_sarvanama(self):
        res = self.morph.generate_sarvanama('tad', 'pum')
        self.assertIn('prathamA', res)



    def test_adhas_prefix(self):
        # 1. Test standard dictionary form
        res_full = self.morph.analyze("aDogamanam")
        self.assertTrue(any(r.word_type == "krdanta" and "aDas" in r.prefixes for r in res_full))
        
        # 2. Test bare stem smart lookup
        res_bare = self.morph.analyze("aDogamana")
        self.assertTrue(any(r.word_type == "krdanta" and "aDas" in r.prefixes for r in res_bare))
        
        # 3. Test forward sandhi generation
        self.assertEqual(apply_forward_sandhi("aDas", "gamanam"), "aDogamanam")

class TestCLI(unittest.TestCase):
    @patch('sys.argv',['sktmorph', 'analyze', 'praBavati'])
    def test_cli_analyze(self):
        with patch('builtins.print'):
            cli.main()

    @patch('sys.argv',['sktmorph', 'analyze', 'fakeWordXyz'])
    def test_cli_analyze_not_found(self):
        with patch('builtins.print'):
            cli.main()

    @patch('sys.argv',['sktmorph', 'generate_verb', '--dhatu', '01.0001', '--lakara', 'plat', '--purusha', '1', '--vacana', '1'])
    def test_cli_generate_verb(self):
        with patch('builtins.print'):
            cli.main()
            
    @patch('sys.argv',['sktmorph', 'generate_krdanta', '--dhatu', '01.0001', '--pratyaya', 'lyuw', '--prefixes', 'pra'])
    def test_cli_generate_krdanta(self):
        with patch('builtins.print'):
            cli.main()

    @patch('sys.argv',['sktmorph', 'generate_noun', '--base', 'manas', '--linga', 'nap'])
    def test_cli_generate_noun(self):
        with patch('builtins.print'):
            cli.main()

    @patch('sys.argv',['sktmorph', 'generate_noun', '--base', 'vAc', '--linga', 'stri'])
    @patch('sktmorph.cli.SktMorph.generate_subanta')
    def test_cli_generate_noun_error(self, mock_gen):
        mock_gen.side_effect = NotImplementedError("Noun Error")
        with patch('builtins.print'):
            with self.assertRaises(SystemExit):
                cli.main()

    @patch('sys.argv',['sktmorph', 'generate_pronoun', '--base', 'tad', '--linga', 'pum'])
    def test_cli_generate_pronoun(self):
        with patch('builtins.print'):
            cli.main()

    @patch('sys.argv',['sktmorph', 'generate_pronoun', '--base', 'tad', '--linga', 'pum'])
    @patch('sktmorph.cli.SktMorph.generate_sarvanama')
    def test_cli_generate_pronoun_error(self, mock_gen):
        mock_gen.side_effect = NotImplementedError("Pronoun Error")
        with patch('builtins.print'):
            with self.assertRaises(SystemExit):
                cli.main()

    @patch('sys.argv',['sktmorph', 'analyze', 'praBavati'])
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

    @patch('sys.argv',['sktmorph', 'analyze', 'praBavati'])
    def test_module_executions(self):
        with warnings.catch_warnings():
            warnings.simplefilter("ignore", RuntimeWarning)
            with patch('builtins.print'):
                runpy.run_module('sktmorph.cli', run_name='__main__')
                runpy.run_module('sktmorph.__main__', run_name='__main__')

if __name__ == '__main__':
    unittest.main()
