import argparse
import sys
import json
import dataclasses
from .morphology import SktMorph

def main():
    parser = argparse.ArgumentParser(description="Sanskrit Morphology Analyzer & Generator (SLP1)")
    subparsers = parser.add_subparsers(dest="command", help="Available commands")
    
    analyze_parser = subparsers.add_parser("analyze", help="Analyze a Sanskrit word in SLP1")
    analyze_parser.add_argument("word", type=str, help="Word to analyze in SLP1 (e.g. prabhavati)")
    analyze_parser.add_argument("--type", type=str, choices=["verb", "declension", "participle", "noun", "pronoun"], help="Filter output by grammatical type")
    
    gen_verb_parser = subparsers.add_parser("generate_verb", help="Generate a verb form")
    gen_verb_parser.add_argument("--dhatu", type=str, required=True)
    gen_verb_parser.add_argument("--lakara", type=str, required=True)
    gen_verb_parser.add_argument("--purusha", type=int, required=True)
    gen_verb_parser.add_argument("--vacana", type=int, required=True)
    gen_verb_parser.add_argument("--prefixes", type=str, nargs="*", default=[])

    gen_krdanta_parser = subparsers.add_parser("generate_krdanta", help="Generate a krdanta (participle)")
    gen_krdanta_parser.add_argument("--dhatu", type=str, required=True)
    gen_krdanta_parser.add_argument("--pratyaya", type=str, required=True)
    gen_krdanta_parser.add_argument("--prefixes", type=str, nargs="*", default=[])

    gen_noun_parser = subparsers.add_parser("generate_noun", help="Generate noun declensions")
    gen_noun_parser.add_argument("--base", type=str, required=True)
    gen_noun_parser.add_argument("--linga", type=str, required=True, choices=["pum", "stri", "nap"])

    gen_pronoun_parser = subparsers.add_parser("generate_pronoun", help="Generate pronoun declensions")
    gen_pronoun_parser.add_argument("--base", type=str, required=True, choices=["tad", "kim", "asmad", "yuzmad", "sarva"])
    gen_pronoun_parser.add_argument("--linga", type=str, required=True, choices=["pum", "stri", "nap", "any"])

    args = parser.parse_args()
    
    try:
        morph = SktMorph()
    except FileNotFoundError as e:
        print(f"Error: {e}")
        sys.exit(1)

    if args.command == "analyze":
        allowed_types = None
        if args.type:
            type_map = {
                "verb": ["tinanta"],
                "declension": ["subanta", "sarvanama"],
                "noun": ["subanta"],
                "pronoun": ["sarvanama"],
                "participle": ["krdanta"]
            }
            allowed_types = type_map[args.type]
            
        results = morph.analyze(args.word, allowed_types=allowed_types)
        
        if not results:
            print(f"No morphological data found for '{args.word}'.")
        for res in results:
            print(json.dumps(dataclasses.asdict(res), ensure_ascii=False, indent=2))
            
    elif args.command == "generate_verb":
        forms = morph.generate_tinanta(args.dhatu, args.lakara, args.purusha, args.vacana, prefixes=args.prefixes)
        print(f"Generated Forms: {forms}")

    elif args.command == "generate_krdanta":
        forms = morph.generate_krdanta(args.dhatu, args.pratyaya, prefixes=args.prefixes)
        print(f"Generated Forms: {forms}")
        
    elif args.command == "generate_noun":
        try:
            table = morph.generate_subanta(args.base, args.linga)
            print(json.dumps(table, indent=4, ensure_ascii=False))
        except NotImplementedError as e:
            print(f"Error: {e}")
            sys.exit(1)

    elif args.command == "generate_pronoun":
        try:
            table = morph.generate_sarvanama(args.base, args.linga)
            print(json.dumps(table, indent=4, ensure_ascii=False))
        except NotImplementedError as e:
            print(f"Error: {e}")
            sys.exit(1)
            
    else:
        parser.print_help()

if __name__ == "__main__":
    main()
