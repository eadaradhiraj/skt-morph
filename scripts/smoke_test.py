"""Quick post-install smoke test for sktmorph."""
import json
import os
import sys

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from scripts.ensure_data import check_data
from sktmorph.morphology import SktMorph
from sktmorph.sarvanama import SarvanamaGenerator
from sktmorph.subanta import SubantaGenerator
from sktmorph.taddhita import derive_stem_rule


def main() -> int:
    if check_data(strict=False) != 0:
        return 1

    try:
        morph = SktMorph()
    except FileNotFoundError as exc:
        print(f"Smoke test failed: {exc}")
        return 1

    checks = []

    def record(name, ok, detail=""):
        checks.append((name, ok, detail))
        status = "ok" if ok else "FAIL"
        suffix = f" ({detail})" if detail else ""
        print(f"[{status}] {name}{suffix}")

    results = morph.analyze("rAmaH")
    record("analyze rAmaH", len(results) > 0, f"{len(results)} parses")

    sub = SubantaGenerator().generate("rAma", "pum")
    record("generate noun rAma", sub is not None and "prathamA" in sub)

    pron = SarvanamaGenerator().generate("paJcan", "pum")
    record("generate pronoun paJcan", pron["prathamA"][0] == "paJcaH")

    stem = derive_stem_rule("rAma", "Tya")
    record("derive taddhita Tya", stem == "rAmAya", stem or "none")

    try:
        tinanta = morph.generate_tinanta("01.0001", "plat", 1, 1)
        record("generate tinanta", len(tinanta) > 0, tinanta[0] if tinanta else "empty")
    except Exception as exc:
        record("generate tinanta", False, str(exc))

    failed = [name for name, ok, _ in checks if not ok]
    if failed:
        print("Smoke test failed:", ", ".join(failed))
        return 1

    print(json.dumps({"status": "ok", "checks": len(checks)}, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
