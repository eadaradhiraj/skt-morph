# sktmorph/cli.py
import argparse
import sys
from .morphology import SktMorph

def main():
    parser = argparse.ArgumentParser(description="Sanskrit Morphology Analyzer & Generator (SLP1)")
    subparsers = parser.add_subparsers(dest="command", help="Available commands")
    
    # Analyze Command
    analyze_parser = subparsers.add_parser("analyze", help="Analyze a Sanskrit word in SLP1")
    analyze_parser.add_argument("word", type=str, help="Word to analyze in SLP1 (e.g. prabhavati)")
    
    # Generate Command
    generate_parser = subparsers.add_parser("generate", help="Generate a verb form")
    generate_parser.add_argument("--dhatu", type=str, required=True, help="Dhatu ID (e.g., 01.0001)")
    generate_parser.add_argument("--lakara", type=str, required=True, help="Lakara (e.g., plat for Lat)")
    generate_parser.add_argument("--purusha", type=int, required=True, help="1, 2, or 3")
    generate_parser.add_argument("--vacana", type=int, required=True, help="1, 2, or 3")
    generate_parser.add_argument("--prefixes", type=str, nargs="*", default=[], help="List of prefixes (e.g. vi A)")

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
            
    elif args.command == "generate":
        forms = morph.generate_tinanta(args.dhatu, args.lakara, args.purusha, args.vacana, prefixes=args.prefixes)
        print(f"Generated Forms: {forms}")
    else:
        parser.print_help()

if __name__ == "__main__":
    main()