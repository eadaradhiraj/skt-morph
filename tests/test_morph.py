import unittest
import io
import runpy
from contextlib import redirect_stdout
from unittest.mock import patch
from sktmorph.analyzer import SanskritAnalyzer
from sktmorph.generator import SanskritGenerator, apply_sandhi
from sktmorph.cli import main as cli_main, format_voice

class TestSktMorph(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.analyzer = SanskritAnalyzer()
        cls.generator = SanskritGenerator()
        conn = cls.analyzer.conn
        # INJECT DATA
        conn.execute("INSERT OR IGNORE INTO dhatu_meta VALUES ('01.0001', 'BU', '1')")
        conn.execute("INSERT OR IGNORE INTO krdanta (dhatu_id, derivation, pratyaya, form_slp1) VALUES ('01.0001', 'test', 'Satf', 'Bavat')")
        conn.execute("INSERT OR IGNORE INTO tinanta (dhatu_id, derivation, lakara, purusha, vacana, form_slp1) VALUES ('01.0001', 'vidyut_shuddha_kartari', 'plat', 'Prathama', 'Eka', 'Bavati')")
        conn.commit()

    @classmethod
    def tearDownClass(cls):
        cls.analyzer.close()
        cls.generator.analyzer.close()

    def test_cli_generate_failure_print(self):
        """Target: sktmorph/cli.py line 90"""
        # Call generate with a nonexistent lakara to force the 'else' block
        args = ['sktmorph', '--generate', '--root', 'BU', '--lakara', 'NOT_REAL', '--purusha', 'p', '--vacana', 'e']
        with patch('sys.argv', args):
            with redirect_stdout(io.StringIO()) as f:
                cli_main()
            self.assertIn("Form not found", f.getvalue())

    def test_cli_generate_with_prefixes(self):
        """Target: sktmorph/generator.py lines 33-34"""
        # Must include --prefixes to trigger the loop in the generator logic
        args = ['sktmorph', '--generate', '--root', 'BU', '--lakara', 'lat', '--purusha', 'prathama', '--vacana', 'eka', '--prefixes', 'pra']
        with patch('sys.argv', args):
            with redirect_stdout(io.StringIO()) as f:
                cli_main()
            self.assertIn("praBavati", f.getvalue())

    def test_analyzer_full_recursion(self):
        # Ensures 100% on analyzer.py
        self.analyzer.analyze("saMpraBavati")
        self.analyzer.analyze("Bavat")
        self.analyzer.analyze("Ax")

    def test_run_main_entry_points(self):
        # Covers the if __name__ == "__main__" blocks
        with patch('sys.argv', ['sktmorph', '--analyze', 'Bavati']):
            with redirect_stdout(io.StringIO()):
                runpy.run_module('sktmorph', run_name='__main__')
                runpy.run_module('sktmorph.cli', run_name='__main__')

    def test_format_voice_complete(self):
        self.assertEqual(format_voice('karmani'), 'Karmani (Passive)')
        self.assertEqual(format_voice('bhave'), 'Bhave (Impersonal)')
        self.assertEqual(format_voice('none'), 'Unknown Voice')

    def test_sandhi_logic(self):
        self.assertEqual(apply_sandhi("", "X"), "X")
        self.assertEqual(apply_sandhi("sam", "B"), "saMB")
        self.assertEqual(apply_sandhi("sam", "a"), "sama")
        self.assertEqual(apply_sandhi("pra", "a"), "prA")
        self.assertEqual(apply_sandhi("pra", "i"), "pre")
        self.assertEqual(apply_sandhi("vi", "a"), "vya")

    def test_cli_analyze_errors(self):
        with patch('sys.argv', ['sktmorph', '--analyze', 'notfound']):
            with redirect_stdout(io.StringIO()):
                cli_main()
        with patch('sys.argv', ['sktmorph', '--generate', '--root', 'BU']):
            with redirect_stdout(io.StringIO()):
                cli_main()

if __name__ == '__main__':
    unittest.main()
