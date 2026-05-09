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

    def test_missing_database(self):
        with self.assertRaises(FileNotFoundError):
            SktMorph("fake_path/fake_db.sqlite")

    def test_analyzer_base_verb(self):
        res = self.morph.analyze('Bavati')
        self.assertTrue(len(res) > 0)
        self.assertEqual(res[0].prefixes, [])
        self.assertEqual(res[0].dhatu, '01.0001')
        self.assertEqual(res[0].word_type, 'tinanta')

    def test_analyzer_single_prefix(self):
        res = self.morph.analyze('praBavati')
        valid = [r for r in res if r.prefixes == ['pra'] and r.dhatu == '01.0001']
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
        with patch.object(self.morph, 'conn') as mock_conn:
            mock_cursor = MagicMock()
            mock_conn.cursor.return_value = mock_cursor
            mock_cursor.fetchall.side_effect = [[{'form_slp1': 'fakeBavati', 'dhatu_id': '99.9999', 'derivation': 'shuddha', 'prayoga': 'kartari', 'lakara': 'plat', 'purusha': 1, 'vacana': 1, 'details_json': None}],[]]
            res = self.morph.analyze('fakeBavati')
            self.assertIsNone(res[0].dhatu_details)

    def test_missing_dhatu_details_krdanta(self):
        with patch.object(self.morph, 'conn') as mock_conn:
            mock_cursor = MagicMock()
            mock_conn.cursor.return_value = mock_cursor
            mock_cursor.fetchall.side_effect = [[],[{'form_slp1': 'fakeBavanam', 'dhatu_id': '99.9999', 'derivation': 'shuddha', 'pratyaya': 'lyuw', 'details_json': None}]]
            res = self.morph.analyze('fakeBavanam')
            self.assertIsNone(res[0].dhatu_details)

    def test_generator(self):
        forms = self.morph.generate_tinanta('01.0001', 'plat', 1, 1, prefixes=['pra'])
        self.assertIn('praBavati', forms)

    def test_generator_edge_cases(self):
        self.assertEqual(self.morph.generate_tinanta('99.9999', 'plat', 1, 1),[])

    def test_generate_sarvanama(self):
        res = self.morph.generate_sarvanama('tad', 'pum')
        self.assertIn('prathamA', res)

class TestCLI(unittest.TestCase):
    @patch('sys.argv', ['sktmorph', 'analyze', 'praBavati'])
    def test_cli_analyze(self):
        with patch('builtins.print'):
            cli.main()

    @patch('sys.argv', ['sktmorph', 'analyze', 'fakeWordXyz'])
    def test_cli_analyze_not_found(self):
        with patch('builtins.print'):
            cli.main()

    @patch('sys.argv',['sktmorph', 'generate_verb', '--dhatu', '01.0001', '--lakara', 'plat', '--purusha', '1', '--vacana', '1'])
    def test_cli_generate_verb(self):
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
            with self.assertRaises(SystemExit):
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
