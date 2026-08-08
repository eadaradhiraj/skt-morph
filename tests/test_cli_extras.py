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
            "01.0001", "plat", 1, 1, derivation="san", prayoga="karmani", prefixes=[], live=True
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

    def test_print_forms_json(self):
        with patch("builtins.print") as mock_print:
            cli._print_forms(["rAmaH"], as_json=True)
            payload = json.loads(mock_print.call_args[0][0])
            self.assertEqual(payload["forms"], ["rAmaH"])

    def test_lakara_help_lists_codes(self):
        help_text = cli._lakara_help()
        self.assertIn("plat", help_text)
        self.assertIn("alat", help_text)

    @patch("sys.argv", ["sktmorph", "generate_verb", "--dhatu", "01.0001", "--lakara", "plat", "--purusha", "1", "--vacana", "1", "--with-prakriya"])
    def test_cli_generate_verb_live_prakriya_text(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            output = "\n".join(str(c[0][0]) for c in mock_print.call_args_list)
            self.assertIn("Bavati", output)
            self.assertIn("3.1.68", output)

    @patch("sys.argv", ["sktmorph", "generate_verb", "--dhatu", "99.9999", "--lakara", "plat", "--purusha", "1", "--vacana", "1", "--with-prakriya"])
    def test_cli_generate_verb_prakriya_skips_missing_form(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            self.assertFalse(mock_print.called)

    @patch("sys.argv", ["sktmorph", "generate_verb", "--dhatu", "01.0001", "--lakara", "plat", "--purusha", "1", "--vacana", "1", "--with-prakriya", "--json"])
    def test_cli_generate_verb_live_prakriya_json(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            payload = json.loads(mock_print.call_args[0][0])
            self.assertEqual(payload["form"], "Bavati")
            self.assertTrue(len(payload["prakriya"]) >= 2)

    @patch("sys.argv", ["sktmorph", "generate_verb", "--dhatu", "01.0001", "--lakara", "plat", "--purusha", "1", "--vacana", "1", "--with-prakriya", "--prefixes", "pra"])
    def test_cli_generate_verb_prakriya_with_prefix(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            output = mock_print.call_args_list[0][0][0]
            self.assertIn("pra", output)

    @patch("sys.argv", ["sktmorph", "generate_verb", "--dhatu", "01.0001", "--lakara", "plat", "--purusha", "1", "--vacana", "1", "--lookup-only"])
    @patch("sktmorph.cli.SktMorph.generate_tinanta", return_value=["fromdb"])
    def test_cli_generate_verb_lookup_only(self, mock_gen, *_mocks):
        cli.main()
        mock_gen.assert_called_once()
        self.assertFalse(mock_gen.call_args.kwargs["live"])

    @patch("sys.argv", ["sktmorph", "generate_verb", "--dhatu", "01.0001", "--lakara", "plat", "--purusha", "1", "--vacana", "1", "--json"])
    def test_cli_generate_verb_json(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            payload = json.loads(mock_print.call_args[0][0])
            self.assertIn("forms", payload)
            self.assertIn("Bavati", payload["forms"])

    @patch("sys.argv", ["sktmorph", "generate_krdanta", "--dhatu", "01.0001", "--pratyaya", "lyuw", "--json"])
    @patch("sktmorph.cli.SktMorph.generate_krdanta", return_value=["form"])
    def test_cli_generate_krdanta_json(self, mock_gen, *_mocks):
        with patch("builtins.print") as mock_print:
            cli.main()
            payload = json.loads(mock_print.call_args[0][0])
            self.assertEqual(payload["forms"], ["form"])

    @patch("sys.argv", ["sktmorph", "generate_noun", "--base", "rAma", "--linga", "pum", "--json"])
    def test_cli_generate_noun_json(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            payload = json.loads(mock_print.call_args[0][0])
            self.assertIn("prathamA", payload)

    @patch("sys.argv", ["sktmorph", "generate_taddhita", "--pratipadika", "rAma", "--pratyaya", "tva", "--linga", "nap", "--json"])
    def test_cli_generate_taddhita_json(self):
        with patch("builtins.print") as mock_print:
            cli.main()
            payload = json.loads(mock_print.call_args[0][0])
            self.assertEqual(payload["stem"], "rAmatva")


if __name__ == "__main__":
    unittest.main()
