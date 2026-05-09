# sktmorph/cli.py
import argparse
import sys
import json
from .morphology import SktMorph

def main():
    parser = argparse.ArgumentParser(description="Sanskrit Morphology Analyzer & Generator (SLP1)")
    subparsers = parser.add_subparsers(dest="command", help="Available commands")
    
    # Analyze Command
    analyze_parser = subparsers.add_parser("analyze", help="Analyze a Sanskrit word in SLP1")
    analyze_parser.add_argument("word", type=str, help="Word to analyze in SLP1 (e.g. prabhavati)")
    
    # Generate Verb Command
    gen_verb_parser = subparsers.add_parser("generate_verb", help="Generate a verb form")
    gen_verb_parser.add_argument("--dhatu", type=str, required=True)
    gen_verb_parser.add_argument("--lakara", type=str, required=True)
    gen_verb_parser.add_argument("--purusha", type=int, required=True)
    gen_verb_parser.add_argument("--vacana", type=int, required=True)
    gen_verb_parser.add_argument("--prefixes", type=str, nargs="*", default=[])

    # Generate Noun Command
    gen_noun_parser = subparsers.add_parser("generate_noun", help="Generate noun declensions (Subantas)")
    gen_noun_parser.add_argument("--base", type=str, required=True, help="Base noun (pratipadika) in SLP1 e.g., rAma")
    gen_noun_parser.add_argument("--linga", type=str, required=True, choices=['pum', 'stri', 'nap'], help="Gender")

    args = parser.parse_args()
    
    try:
        morph = SktMorph()
    except FileNotFoundError as e:
        print(f"Error: {e}")
        sys.exit(1)

    if args.command == "analyze":
        results = morph.analyze(args.word)
        if not results:
            print(f"No morphological data found for '{args.word}'.")
        for res in results:
            print(res)
            
    elif args.command == "generate_verb":
        forms = morph.generate_tinanta(args.dhatu, args.lakara, args.purusha, args.vacana, prefixes=args.prefixes)
        print(f"Generated Forms: {forms}")
        
    elif args.command == "generate_noun":
        try:
            table = morph.generate_subanta(args.base, args.linga)
            print(json.dumps(table, indent=4, ensure_ascii=False))
        except NotImplementedError as e:
            print(f"Error: {e}")
            sys.exit(1)
            
    else:
        parser.print_help()

if __name__ == "__main__":
    main()