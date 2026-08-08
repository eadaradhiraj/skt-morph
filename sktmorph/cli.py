import argparse
import sys
import json
import dataclasses
from typing import Any, Dict
from .morphology import SktMorph, MorphResult, apply_forward_sandhi
from .constants import DERIVATIONS, LAKARA_LABELS, LAKARAS, PRAYOGAS
from .taddhita import PRATYAYA_ALIASES
from .sarvanama import PRONOUNS
from .translit import has_devanagari_support, maybe_from_slp1, maybe_to_slp1


def _result_dict(result: MorphResult, with_prakriya: bool = False) -> Dict[str, Any]:
    data = dataclasses.asdict(result)
    if not with_prakriya:
        data.pop("prakriya", None)
    return data


def _payload_dict(payload: Dict[str, Any], with_prakriya: bool = False) -> Dict[str, Any]:
    if with_prakriya:
        return payload
    return {key: value for key, value in payload.items() if key != "prakriya"}


def _print_json(payload: Any, devanagari: bool = False, compact: bool = False) -> None:
    if compact:
        text = json.dumps(payload, ensure_ascii=False, separators=(",", ":"))
    else:
        indent = 2 if isinstance(payload, list) else 4
        text = json.dumps(payload, ensure_ascii=False, indent=indent)
    if devanagari and has_devanagari_support():
        text = _devanagariize_json(text)
    print(text)


def _devanagariize_json(text: str) -> str:
    import re

    def repl(match):
        return maybe_from_slp1(match.group(0), devanagari=True)

    return re.sub(r"[A-Za-z][A-Za-z0-9,/.\-_]*", repl, text)


def _add_io_flags(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--devanagari",
        action="store_true",
        help="Accept Devanagari input and/or print Devanagari output",
    )


def _add_json_flag(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--json",
        action="store_true",
        help="Emit compact JSON output",
    )


def _print_forms(forms, devanagari: bool = False, as_json: bool = False) -> None:
    if devanagari:
        forms = [maybe_from_slp1(f, True) for f in forms]
    if as_json:
        _print_json({"forms": forms}, devanagari=False, compact=True)
    else:
        print(f"Generated Forms: {forms}")


def _lakara_help() -> str:
    return "; ".join(f"{code} ({label})" for code, label in LAKARA_LABELS.items())


def main():
    parser = argparse.ArgumentParser(description="Sanskrit Morphology Analyzer & Generator (SLP1)")
    subparsers = parser.add_subparsers(dest="command", help="Available commands")

    analyze_parser = subparsers.add_parser("analyze", help="Analyze a Sanskrit word in SLP1")
    analyze_parser.add_argument("word", type=str, help="Word to analyze (SLP1 or Devanagari with --devanagari)")
    analyze_parser.add_argument(
        "--type",
        type=str,
        choices=["verb", "declension", "participle", "noun", "pronoun", "taddhita"],
        help="Filter output by grammatical type",
    )
    analyze_parser.add_argument(
        "--with-prakriya",
        action="store_true",
        help="Include prakriya derivation traces in output",
    )
    analyze_parser.add_argument(
        "--top",
        type=int,
        default=None,
        metavar="N",
        help="Return at most N ranked analyses",
    )
    analyze_parser.add_argument(
        "--json",
        action="store_true",
        help="Emit compact JSON (single object or array)",
    )
    _add_io_flags(analyze_parser)

    gen_verb_parser = subparsers.add_parser("generate_verb", help="Generate a verb form")
    gen_verb_parser.add_argument("--dhatu", type=str, required=True)
    gen_verb_parser.add_argument(
        "--lakara",
        type=str,
        required=True,
        choices=LAKARAS,
        help=f"Lakara code. Values: {_lakara_help()}",
    )
    gen_verb_parser.add_argument("--purusha", type=int, required=True, choices=[1, 2, 3])
    gen_verb_parser.add_argument("--vacana", type=int, required=True, choices=[1, 2, 3])
    gen_verb_parser.add_argument("--prefixes", type=str, nargs="*", default=[])
    gen_verb_parser.add_argument(
        "--prayoga",
        type=str,
        default="kartari",
        choices=list(PRAYOGAS),
        help="Voice (kartari=active, karmani=middle/passive)",
    )
    gen_verb_parser.add_argument(
        "--derivation",
        type=str,
        default="shuddha",
        choices=list(DERIVATIONS),
        help="Derivation class for the verb form",
    )
    gen_verb_parser.add_argument(
        "--lookup-only",
        action="store_true",
        help="Skip live derivation and use SQLite tinanta lookup only",
    )
    gen_verb_parser.add_argument(
        "--with-prakriya",
        action="store_true",
        help="Include live derivation steps (requires live engine; JSON output with --json)",
    )
    _add_json_flag(gen_verb_parser)
    _add_io_flags(gen_verb_parser)

    gen_krdanta_parser = subparsers.add_parser("generate_krdanta", help="Generate a krdanta (participle)")
    gen_krdanta_parser.add_argument("--dhatu", type=str, required=True)
    gen_krdanta_parser.add_argument("--pratyaya", type=str, required=True)
    gen_krdanta_parser.add_argument("--prefixes", type=str, nargs="*", default=[])
    gen_krdanta_parser.add_argument(
        "--derivation",
        type=str,
        default="shuddha",
        choices=list(DERIVATIONS),
        help="Derivation class for the participle",
    )
    _add_json_flag(gen_krdanta_parser)
    _add_io_flags(gen_krdanta_parser)

    taddhita_choices = sorted(set(PRATYAYA_ALIASES.keys()))
    gen_taddhita_parser = subparsers.add_parser("generate_taddhita", help="Generate a taddhita (nominal derivative)")
    gen_taddhita_parser.add_argument("--pratipadika", type=str, required=True)
    gen_taddhita_parser.add_argument("--pratyaya", type=str, required=True, choices=taddhita_choices)
    gen_taddhita_parser.add_argument("--linga", type=str, required=True, choices=["pum", "stri", "nap"])
    gen_taddhita_parser.add_argument(
        "--with-prakriya",
        action="store_true",
        help="Include prakriya derivation traces in output",
    )
    _add_json_flag(gen_taddhita_parser)
    _add_io_flags(gen_taddhita_parser)

    gen_noun_parser = subparsers.add_parser("generate_noun", help="Generate noun declensions")
    gen_noun_parser.add_argument("--base", type=str, required=True)
    gen_noun_parser.add_argument("--linga", type=str, required=True, choices=["pum", "stri", "nap"])
    gen_noun_parser.add_argument(
        "--with-prakriya",
        action="store_true",
        help="Include prakriya derivation traces in output",
    )
    _add_json_flag(gen_noun_parser)
    _add_io_flags(gen_noun_parser)

    pronoun_choices = sorted({base for base, _ in PRONOUNS.keys()})
    gen_pronoun_parser = subparsers.add_parser("generate_pronoun", help="Generate pronoun declensions")
    gen_pronoun_parser.add_argument("--base", type=str, required=True, choices=pronoun_choices)
    gen_pronoun_parser.add_argument("--linga", type=str, required=True, choices=["pum", "stri", "nap", "any"])
    gen_pronoun_parser.add_argument(
        "--with-prakriya",
        action="store_true",
        help="Include prakriya derivation traces in output",
    )
    _add_json_flag(gen_pronoun_parser)
    _add_io_flags(gen_pronoun_parser)

    args = parser.parse_args()

    try:
        morph = SktMorph()
    except FileNotFoundError as e:
        print(f"Error: {e}")
        sys.exit(1)

    devanagari = getattr(args, "devanagari", False)

    if args.command == "analyze":
        allowed_types = None
        if args.type:
            type_map = {
                "verb": ["tinanta"],
                "declension": ["subanta", "sarvanama"],
                "noun": ["subanta"],
                "pronoun": ["sarvanama"],
                "participle": ["krdanta"],
                "taddhita": ["taddhita"],
            }
            allowed_types = type_map[args.type]

        word = maybe_to_slp1(args.word, devanagari=devanagari)
        results = morph.analyze(word, allowed_types=allowed_types, include_prakriya=args.with_prakriya)

        if args.top is not None:
            results = results[: args.top]

        if not results:
            print(f"No morphological data found for '{args.word}'.")
        elif args.json:
            payloads = [_result_dict(res, args.with_prakriya) for res in results]
            output = payloads[0] if len(payloads) == 1 else payloads
            _print_json(output, devanagari=devanagari, compact=True)
        else:
            for res in results:
                _print_json(_result_dict(res, args.with_prakriya), devanagari=devanagari)

    elif args.command == "generate_verb":
        if args.with_prakriya and not args.lookup_only:
            from .engine.tinanta import LiveTinantaEngine

            engine = LiveTinantaEngine(morph.conn_dhatus)
            queries = morph.resolve_dhatu_ids(args.dhatu) or [args.dhatu]
            payloads = []
            for query in queries:
                form, steps = engine.generate(
                    query,
                    args.lakara,
                    args.purusha,
                    args.vacana,
                    args.derivation,
                    args.prayoga,
                )
                if not form:
                    continue
                current = form
                for prefix in reversed(args.prefixes):
                    current = apply_forward_sandhi(prefix, current)
                payloads.append(
                    {
                        "form": current,
                        "dhatu": query,
                        "prakriya": [step.to_prakriya() for step in steps],
                    }
                )
            if args.json:
                output = payloads[0] if len(payloads) == 1 else payloads
                _print_json(output, devanagari=devanagari, compact=True)
            else:
                for item in payloads:
                    print(item["form"])
                    for step in item["prakriya"]:
                        print(f"  {step['step']}  [{', '.join(step['sutras'])}]")
        else:
            forms = morph.generate_tinanta(
                args.dhatu,
                args.lakara,
                args.purusha,
                args.vacana,
                derivation=args.derivation,
                prayoga=args.prayoga,
                prefixes=args.prefixes,
                live=not args.lookup_only,
            )
            _print_forms(forms, devanagari=devanagari, as_json=args.json)

    elif args.command == "generate_krdanta":
        forms = morph.generate_krdanta(
            args.dhatu,
            args.pratyaya,
            derivation=args.derivation,
            prefixes=args.prefixes,
        )
        _print_forms(forms, devanagari=devanagari, as_json=args.json)

    elif args.command == "generate_taddhita":
        try:
            base = maybe_to_slp1(args.pratipadika, devanagari=devanagari)
            result = morph.generate_taddhita(
                base, args.pratyaya, args.linga, include_prakriya=args.with_prakriya
            )
            _print_json(
                _payload_dict(result, args.with_prakriya),
                devanagari=devanagari,
                compact=args.json,
            )
        except (NotImplementedError, ValueError) as e:
            print(f"Error: {e}")
            sys.exit(1)

    elif args.command == "generate_noun":
        try:
            base = maybe_to_slp1(args.base, devanagari=devanagari)
            if args.with_prakriya:
                result = morph.generate_subanta(base, args.linga, include_prakriya=True)
                _print_json(result, devanagari=devanagari, compact=args.json)
            else:
                table = morph.generate_subanta(base, args.linga, include_prakriya=False)
                _print_json(table, devanagari=devanagari, compact=args.json)
        except NotImplementedError as e:
            print(f"Error: {e}")
            sys.exit(1)

    elif args.command == "generate_pronoun":
        try:
            result = morph.generate_sarvanama(
                args.base, args.linga, include_prakriya=args.with_prakriya
            )
            _print_json(
                _payload_dict(result, args.with_prakriya),
                devanagari=devanagari,
                compact=args.json,
            )
        except NotImplementedError as e:
            print(f"Error: {e}")
            sys.exit(1)

    else:
        parser.print_help()


if __name__ == "__main__":
    main()
