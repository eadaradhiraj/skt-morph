"""Run the test suite and enforce 100% coverage on sktmorph."""
import subprocess
import sys


def main() -> int:
    commands = [
        [sys.executable, "-m", "coverage", "run", "-m", "pytest", "tests/", "-q"],
        [sys.executable, "-m", "coverage", "report", "--fail-under=100"],
    ]
    for cmd in commands:
        result = subprocess.run(cmd, check=False)
        if result.returncode != 0:
            return result.returncode
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
