import unittest
import io
import runpy
from contextlib import redirect_stdout
from unittest.mock import patch
from sktmorph.analyzer import SanskritAnalyzer
from sktmorph.generator import SanskritGenerator, apply_sandhi
from sktmorph.cli import main as cli_main, format_voice

class TestSktMorphFull(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.analyzer = SanskritAnalyzer()
        cls.generator = SanskritGenerator()
        conn = cls.analyzer.conn
        conn.execute("INSERT OR IGNORE INTO dhatu_meta VALUES ('01.0001', 'BU', '1')")
        conn.execute("INSERT OR IGNORE INTO krdanta (dhatu_id, derivation, pratyaya, form_slp1) VALUES ('01.0001', 'test', 'Satf', 'Bavat')")
        conn.execute("INSERT OR IGNORE INTO tinanta (dhatu_id, derivation, lakara, purusha, vacana, form_slp1) VALUES ('01.0001', 'vidyut_shuddha_kartari', 'plat', 'Prathama', 'Eka', 'Bavati')")
        conn.commit()

    @classmethod
    def tearDownClass(cls):
        cls.analyzer.close()
        cls.generator.analyzer.close()

    def test_entry_points(self):
        with patch('sys.argv', ['sktmorph', '--analyze', 'Bavati']):
            with redirect_stdout(io.StringIO()):
                runpy.run_module('sktmorph', run_name='__main__')
                runpy.run_module('sktmorph.cli', run_name='__main__')

    def test_cli_analyze_fail(self):
        with patch('sys.argv', ['sktmorph', '--analyze', 'notaword']):
            with redirect_stdout(io.StringIO()) as f:
                cli_main()
            self.assertIn("No results found", f.getvalue())

    def test_cli_generate_full(self):
        # Test success with abbreviations (p, e)
        args = ['sktmorph', '--generate', '--root', 'BU', '--lakara', 'lat', '--purusha', 'p', '--vacana', 'e', '--prefixes', 'pra']
        with patch('sys.argv', args):
            with redirect_stdout(io.StringIO()) as f:
                cli_main()
            self.assertIn("Result: praBavati", f.getvalue())
        
        # Test missing args
        with patch('sys.argv', ['sktmorph', '--generate', '--root', 'BU']):
            with redirect_stdout(io.StringIO()):
                with self.assertRaises(SystemExit):
                    cli_main()

        # Test form not found
        args_fail = ['sktmorph', '--generate', '--root', 'BU', '--lakara', 'bad', '--purusha', 'p', '--vacana', 'e']
        with patch('sys.argv', args_fail):
            with redirect_stdout(io.StringIO()) as f:
                cli_main()
            self.assertIn("Form not found", f.getvalue())

    def test_voice_formatting(self):
        self.assertEqual(format_voice('karmani'), 'Karmani (Passive)')
        self.assertEqual(format_voice('bhave'), 'Bhave (Impersonal)')
        self.assertEqual(format_voice('other'), 'other')

    def test_sandhi_exhaustive(self):
        self.assertEqual(apply_sandhi("", "X"), "X")
        self.assertEqual(apply_sandhi("pra", "aBavat"), "prABavat")
        self.assertEqual(apply_sandhi("pra", "ikzate"), "prekzate")
        self.assertEqual(apply_sandhi("sam", "Bavati"), "saMBavati")
        self.assertEqual(apply_sandhi("sam", "arcati"), "samarcati")
        self.assertEqual(apply_sandhi("vi", "ayate"), "vyayate")

    def test_analyzer_krdanta_recursion(self):
        self.analyzer.analyze("saMBavat")
        self.analyzer.analyze("Ax")

if __name__ == '__main__':
    unittest.main()
