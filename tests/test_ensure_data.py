import os
import sys
import unittest
from unittest.mock import patch

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import scripts.ensure_data as ensure_data


class TestEnsureData(unittest.TestCase):
    def test_main_success_when_databases_present(self):
        with patch.object(ensure_data, "REQUIRED", ["dhatus.sqlite"]):
            with patch("os.path.exists", return_value=True):
                self.assertEqual(ensure_data.main(), 0)

    def test_main_failure_lists_missing(self):
        with patch.object(ensure_data, "REQUIRED", ["missing.sqlite"]):
            with patch("os.path.exists", return_value=False):
                with patch("builtins.print") as mock_print:
                    self.assertEqual(ensure_data.main(), 1)
                    output = " ".join(str(call) for call in mock_print.call_args_list)
                    self.assertIn("missing.sqlite", output)
                    self.assertIn("build_db.py", output)


if __name__ == "__main__":
    unittest.main()
