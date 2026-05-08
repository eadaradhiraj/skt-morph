import argparse
from sktmorph.analyzer import SanskritAnalyzer

def format_voice(derivation):
    """Extracts Voice (Kartari/Karmani) from your specific filename derivation tags."""
    if 'kartari' in derivation: return 'Kartari (Active)'
    if 'karmani' in derivation: return 'Karmani (Passive)'
    if 'bhave' in derivation: return 'Bhave (Impersonal)'
    return 'Unknown Voice'

def main():
    parser = argparse.ArgumentParser(description="Skt-Morph: Sanskrit Morphological Analyzer")
    # Added the --analyze flag
    parser.add_argument("--analyze", dest="word", required=True, help="Word to analyze (SLP1)")
    args = parser.parse_args()

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

if __name__ == "__main__":
    main()