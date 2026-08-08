"""Verify all bundled SQLite shards stay under GitHub's 50 MB file limit."""
import glob
import os
import sys

DATA_DIR = os.path.join(os.path.dirname(__file__), "..", "sktmorph", "data")
LIMIT_BYTES = 50 * 1024 * 1024


def main() -> int:
    oversized = []
    for path in sorted(glob.glob(os.path.join(DATA_DIR, "*.sqlite"))):
        size = os.path.getsize(path)
        mb = size / (1024 * 1024)
        status = "ok" if size < LIMIT_BYTES else "OVER LIMIT"
        print(f"{status:10} {mb:6.1f} MB  {os.path.basename(path)}")
        if size >= LIMIT_BYTES:
            oversized.append(path)

    if oversized:
        print("\nSplit oversized databases before committing:")
        print("  python scripts/split_sqlite_shards.py")
        return 1
    print(f"\nAll SQLite files under 50 MB in {os.path.abspath(DATA_DIR)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
