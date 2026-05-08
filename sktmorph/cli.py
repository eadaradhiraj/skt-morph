import argparse
import sys
from sktmorph.analyzer import SanskritAnalyzer
from sktmorph.generator import SanskritGenerator

def format_voice(derivation):
    """Extracts Voice (Kartari/Karmani) from derivation tags."""
    if 'kartari' in derivation: return 'Kartari (Active)'
    if 'karmani' in derivation: return 'Karmani (Passive)'
    if 'bhave' in derivation: return 'Bhave (Impersonal)'
    return 'Unknown Voice'

def main():
    parser = argparse.ArgumentParser(description="Skt-Morph: Sanskrit Morphological Tool")
    
    group = parser.add_mutually_exclusive_group(required=True)
    
    # dest="word" maps the input of --analyze to args.word
    group.add_argument("--analyze", dest="word", help="Word to analyze (SLP1)")
    group.add_argument("--generate", action="store_true", help="Generate a form")
    
    # Generation arguments
    parser.add_argument("--root", help="Dhatu name (e.g. BU) or ID (e.g. 01.0001)")
    parser.add_argument("--lakara", help="e.g. lat, lit, lan")
    parser.add_argument("--purusha", help="prathama, madhyama, uttama")
    parser.add_argument("--vacana", help="eka, dvi, bahu")
    parser.add_argument("--prefixes", help="Comma separated prefixes e.g. sam,pra")

    args = parser.parse_args()

    # --- ANALYSIS BRANCH ---
    if args.word:
        analyzer = SanskritAnalyzer()
        results = analyzer.analyze(args.word)

        if not results:
            print(f"\nNo results found for '{args.word}'")
            analyzer.close()
            return

        print(f"\nAnalysis for: {args.word}")
        print("=" * 60)

        for item in results:
            data = item['data']
            prefix_str = " + ".join(data['prefixes']) if data['prefixes'] else "None"

            if item['type'] == 'tinanta':
                voice = format_voice(data['derivation'])
                print(f"[Conjugation]")
                print(f"  Prefixes : {prefix_str}")
                print(f"  Root ID  : {data['dhatu_id']}")
                print(f"  Voice    : {voice}")
                print(f"  Tense    : {data['lakara']}")
                print(f"  Person   : {data['purusha']}")
                print(f"  Number   : {data['vacana']}")
                print("-" * 60)

            elif item['type'] == 'krdanta':
                subanta = data.get('subanta_info', 'Pratipadika / Prathama')
                print(f"[Krdanta]")
                print(f"  Prefixes     : {prefix_str}")
                print(f"  Root ID      : {data['dhatu_id']}")
                print(f"  Krdanta Name : {data['pratyaya']}")
                print(f"  Subanta Note : {subanta}")
                print("-" * 60)
        
        analyzer.close()

    # --- GENERATION BRANCH ---
    elif args.generate:
        if not all([args.root, args.lakara, args.purusha, args.vacana]):
            print("Error: Generation requires --root, --lakara, --purusha, and --vacana")
            return

        gen = SanskritGenerator()
        prefixes = args.prefixes.split(',') if args.prefixes else []
        
        word = gen.generate_tinanta(
            root_or_id=args.root, 
            lakara=args.lakara, 
            purusha=args.purusha, 
            vacana=args.vacana, 
            prefixes=prefixes
        )
        
        if word:
            print(f"\nGenerated Form: {word}")
        else:
            print(f"\nForm not found for Root:{args.root}, Lakara:{args.lakara} in shuddha_kartari")
        
        gen.analyzer.close()

if __name__ == "__main__":
    main()