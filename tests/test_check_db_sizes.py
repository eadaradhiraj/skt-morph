import os
import sys
import unittest
from unittest.mock import patch

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import scripts.check_db_sizes as check_db_sizes


class TestCheckDbSizes(unittest.TestCase):
    def test_main_ok_when_under_limit(self):
        with patch("glob.glob", return_value=["/fake/a.sqlite"]):
            with patch("os.path.getsize", return_value=1024):
                with patch("builtins.print"):
                    self.assertEqual(check_db_sizes.main(), 0)

    def test_main_fails_when_over_limit(self):
        with patch("glob.glob", return_value=["/fake/big.sqlite"]):
            with patch("os.path.getsize", return_value=check_db_sizes.LIMIT_BYTES):
                with patch("builtins.print"):
                    self.assertEqual(check_db_sizes.main(), 1)


if __name__ == "__main__":
    unittest.main()
