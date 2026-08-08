import os
import sys
import unittest
from unittest.mock import patch

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import scripts.ensure_data as ensure_data
import scripts.smoke_test as smoke_test


class TestEnsureData(unittest.TestCase):
    def test_check_data_success_when_databases_present(self):
        with patch.object(ensure_data, "REQUIRED", ["dhatus.sqlite"]):
            with patch.object(ensure_data, "OPTIONAL_GLOBS", ["tinantas_*.sqlite"]):
                with patch("os.path.exists", return_value=True):
                    with patch("glob.glob", return_value=["tinantas_shuddha_gana1.sqlite"]):
                        self.assertEqual(ensure_data.check_data(strict=False), 0)

    def test_check_data_failure_lists_missing(self):
        with patch.object(ensure_data, "REQUIRED", ["missing.sqlite"]):
            with patch("os.path.exists", return_value=False):
                with patch("builtins.print"):
                    self.assertEqual(ensure_data.check_data(strict=False), 1)

    def test_check_data_strict_requires_optional_shards(self):
        with patch.object(ensure_data, "REQUIRED", ["dhatus.sqlite"]):
            with patch.object(ensure_data, "OPTIONAL_GLOBS", ["tinantas_*.sqlite"]):
                with patch("os.path.exists", return_value=True):
                    with patch("glob.glob", return_value=[]):
                        with patch("builtins.print"):
                            self.assertEqual(ensure_data.check_data(strict=True), 1)

    def test_main_strict_flag(self):
        with patch.object(ensure_data, "check_data", return_value=0) as mock_check:
            with patch.object(sys, "argv", ["ensure_data.py", "--strict"]):
                self.assertEqual(ensure_data.main(), 0)
            mock_check.assert_called_once_with(strict=True)


class TestSmokeTest(unittest.TestCase):
    def test_main_returns_1_when_data_missing(self):
        with patch.object(smoke_test, "check_data", return_value=1):
            self.assertEqual(smoke_test.main(), 1)

    def test_main_runs_checks_when_data_present(self):
        with patch.object(smoke_test, "check_data", return_value=0):
            with patch.object(smoke_test, "SktMorph") as mock_morph_cls:
                morph = mock_morph_cls.return_value
                morph.analyze.return_value = [object()]
                morph.generate_tinanta.return_value = ["Bavati"]
                with patch("builtins.print"):
                    self.assertEqual(smoke_test.main(), 0)


if __name__ == "__main__":
    unittest.main()
