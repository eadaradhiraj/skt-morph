import argparse
from sktmorph.analyzer import SanskritAnalyzer
from sktmorph.generator import SanskritGenerator

def main():
    parser = argparse.ArgumentParser(description="Skt-Morph Tool")
    group = parser.add_mutually_exclusive_group(required=True)
    
    # Analysis Flag
    group.add_argument("--analyze", help="Word to analyze")
    
    # Generation Flags
    group.add_argument("--generate", action="store_true", help="Generate a form")
    
    parser.add_argument("--root", help="Dhatu ID (e.g. 01.0001)")
    parser.add_argument("--lakara", help="e.g. plat, pqlot")
    parser.add_argument("--purusha", help="Prathama, Madhyama, Uttama")
    parser.add_argument("--vacana", help="Eka, Dvi, Bahu")
    parser.add_argument("--prefixes", help="Comma separated prefixes e.g. sam,pra")

    args = parser.parse_args()
    
    if args.analyze:

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
            # Format the prefix string
            prefix_str = " + ".join(data['prefixes']) if data['prefixes'] else "None"

            if item['type'] == 'tinanta':
                voice = format_voice(data['derivation'])
                # Clean up lakara name (e.g. plat -> lat)
                lakara = data['lakara'].replace('p', '').replace('a', '') if data['lakara'].startswith(('p','a')) else data['lakara']
                
                print(f"[Conjugation]")
                print(f"  Prefixes : {prefix_str}")
                print(f"  Root ID  : {data['dhatu_id']}")
                print(f"  Voice    : {voice} ({data['derivation']})")
                print(f"  Tense    : {lakara}")
                print(f"  Person   : {data['purusha']}")
                print(f"  Number   : {data['vacana']}")
                print("-" * 60)

            elif item['type'] == 'krdanta':
                subanta = data.get('subanta_note', 'Pratipadika / Prathama')
                
                print(f"[Krdanta]")
                print(f"  Prefixes     : {prefix_str}")
                print(f"  Root ID      : {data['dhatu_id']}")
                print(f"  Krdanta Name : {data['pratyaya']}")
                print(f"  Subanta Note : {subanta}")
                print("-" * 60)

                analyzer.close()

    elif args.generate:
        if not all([args.root, args.lakara, args.purusha, args.vacana]):
            print("Error: Generation requires --root, --lakara, --purusha, and --vacana")
            return

        gen = SanskritGenerator()
        prefixes = args.prefixes.split(',') if args.prefixes else []
        
        # Result
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
            print(f"\nForm not found for Root:{args.root}, Lakara:{args.lakara}...")


if __name__ == "__main__":
    main()