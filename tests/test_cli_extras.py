import json
import unittest
from unittest.mock import patch

from sktmorph import cli
from sktmorph import translit


class TestCLIDevanagariAndPrakriya(unittest.TestCase):
    @patch("sys.argv", ["sktmorph", "analyze", "rAmaH", "--with-prakriya"])
    def test_cli_analyze_with_prakriya_flag(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            outputs = [json.loads(call[0][0]) for call in mock_print.call_args_list]
            self.assertTrue(any("prakriya" in item for item in outputs))

    @patch("sys.argv", ["sktmorph", "generate_noun", "--base", "rAma", "--linga", "pum", "--with-prakriya"])
    def test_cli_generate_noun_with_prakriya(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            payload = json.loads(mock_print.call_args[0][0])
            self.assertIn("prakriya", payload)
            self.assertIn("declension", payload)

    @patch("sys.argv", ["sktmorph", "generate_verb", "--dhatu", "01.0001", "--lakara", "plat", "--purusha", "1", "--vacana", "1", "--devanagari"])
    @patch("sktmorph.cli.has_devanagari_support", return_value=True)
    @patch("sktmorph.cli.maybe_from_slp1", side_effect=lambda text, devanagari=False: f"DEV:{text}")
    def test_cli_generate_verb_devanagari_output(self, *_mocks):
        with patch("builtins.print") as mock_print:
            cli.main()
            output = mock_print.call_args[0][0]
            self.assertTrue(output.startswith("Generated Forms:"))
            self.assertIn("DEV:", output)

    @patch("sys.argv", ["sktmorph", "generate_krdanta", "--dhatu", "01.0001", "--pratyaya", "lyuw", "--devanagari"])
    @patch("sktmorph.cli.has_devanagari_support", return_value=True)
    @patch("sktmorph.cli.maybe_from_slp1", side_effect=lambda text, devanagari=False: f"DEV:{text}")
    def test_cli_generate_krdanta_devanagari_output(self, *_mocks):
        with patch("builtins.print") as mock_print:
            cli.main()
            self.assertIn("DEV:", mock_print.call_args[0][0])

    @patch("sktmorph.cli.has_devanagari_support", return_value=True)
    @patch("sktmorph.cli.maybe_from_slp1", return_value="राम")
    def test_devanagariize_json(self, *_mocks):
        text = cli._devanagariize_json('{"word":"rAmaH"}')
        self.assertIn("राम", text)

    @patch("sktmorph.cli.has_devanagari_support", return_value=False)
    def test_print_json_skips_devanagari_without_support(self, _mock):
        with patch("builtins.print") as mock_print:
            cli._print_json({"word": "rAmaH"}, devanagari=True)
            self.assertIn("rAmaH", mock_print.call_args[0][0])

    @patch("sktmorph.cli.has_devanagari_support", return_value=True)
    @patch("sktmorph.cli._devanagariize_json", return_value="DEVTEXT")
    def test_print_json_uses_devanagariize(self, _mock_d, _mock_h):
        with patch("builtins.print") as mock_print:
            cli._print_json({"word": "rAmaH"}, devanagari=True)
            mock_print.assert_called_once_with("DEVTEXT")

    @patch("sys.argv", ["sktmorph", "analyze", "te", "--top", "1", "--json"])
    def test_cli_analyze_top_json(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            raw = mock_print.call_args[0][0]
            payload = json.loads(raw)
            self.assertIsInstance(payload, dict)
            self.assertNotIn(": ", raw)

    @patch("sys.argv", ["sktmorph", "analyze", "te", "--top", "2", "--json"])
    def test_cli_analyze_top_json_array(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            payload = json.loads(mock_print.call_args[0][0])
            self.assertIsInstance(payload, list)
            self.assertEqual(len(payload), 2)

    @patch("sys.argv", ["sktmorph", "generate_verb", "--dhatu", "01.0001", "--lakara", "plat", "--purusha", "1", "--vacana", "1", "--prayoga", "karmani", "--derivation", "san"])
    @patch("sktmorph.cli.SktMorph.generate_tinanta", return_value=["form"])
    def test_cli_generate_verb_prayoga_derivation(self, mock_gen, *_mocks):
        cli.main()
        mock_gen.assert_called_once_with(
            "01.0001", "plat", 1, 1, derivation="san", prayoga="karmani", prefixes=[]
        )

    @patch("sys.argv", ["sktmorph", "generate_krdanta", "--dhatu", "01.0001", "--pratyaya", "lyuw", "--derivation", "nich"])
    @patch("sktmorph.cli.SktMorph.generate_krdanta", return_value=["form"])
    def test_cli_generate_krdanta_derivation(self, mock_gen, *_mocks):
        cli.main()
        mock_gen.assert_called_once_with("01.0001", "lyuw", derivation="nich", prefixes=[])

    @patch("sys.argv", ["sktmorph", "generate_pronoun", "--base", "tad", "--linga", "pum", "--with-prakriya"])
    def test_cli_generate_pronoun_with_prakriya(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            payload = json.loads(mock_print.call_args[0][0])
            self.assertIn("prakriya", payload)
            self.assertIn("declension", payload)

    def test_print_json_compact(self):
        with patch("builtins.print") as mock_print:
            cli._print_json({"a": 1}, compact=True)
            self.assertEqual(mock_print.call_args[0][0], '{"a":1}')


if __name__ == "__main__":
    unittest.main()
