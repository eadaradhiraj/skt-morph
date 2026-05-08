import argparse
import sys
from sktmorph.analyzer import SanskritAnalyzer
from sktmorph.generator import SanskritGenerator

def format_voice(derivation):
    """Converts technical derivation strings to readable Voice names."""
    if 'kartari' in derivation: return 'Kartari (Active)'
    if 'karmani' in derivation: return 'Karmani (Passive)'
    if 'bhave' in derivation: return 'Bhave (Impersonal)'
    return derivation

def format_lakara(lakara):
    """Cleans up technical codes (e.g., 'plat' -> 'lat', 'plang' -> 'lang')."""
    # Remove 'p' (parasmaipada) or 'a' (atmanepada) prefixes from technical keys
    clean = lakara
    if len(lakara) > 3 and (lakara.startswith('p') or lakara.startswith('a')):
        clean = lakara[1:]
    
    # Map common abbreviations
    mapping = {
        'lat': 'laṭ (Present)',
        'lit': 'liṭ (Perfect)',
        'lut': 'luṭ (First Future)',
        'lrt': 'lṛṭ (Simple Future)',
        'lot': 'loṭ (Imperative)',
        'lang': 'laṅ (Imperfect)',
        'ling': 'vidhi-liṅ (Potential)',
        'ashirlin': 'āśīr-liṅ (Benedictive)',
        'lun': 'luṅ (Aorist)',
        'lrin': 'lṛṅ (Conditional)'
    }
    return mapping.get(clean, clean)

def main():
    parser = argparse.ArgumentParser(description="Skt-Morph: Sanskrit Morphological Tool")
    
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--analyze", dest="word", help="Word to analyze (SLP1)")
    group.add_argument("--generate", action="store_true", help="Generate a form")
    
    # Generation arguments
    parser.add_argument("--root", help="Dhatu name (e.g. BU) or ID (e.g. 01.0001)")
    parser.add_argument("--lakara", help="e.g. lat, lit, lan, lot")
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
        print("=" * 70)

        for item in results:
            data = item['data']
            # Format prefixes
            prefix_str = " + ".join(data['prefixes']) if data['prefixes'] else "None"
            # Get root name from metadata join
            root_name = data.get('root_slp1', 'Unknown')

            if item['type'] == 'tinanta':
                voice = format_voice(data['derivation'])
                tense = format_lakara(data['lakara'])
                
                print(f"[Conjugation]")
                print(f"  Prefixes : {prefix_str}")
                print(f"  Root     : {root_name} ({data['dhatu_id']})")
                print(f"  Voice    : {voice}")
                print(f"  Tense    : {tense}")
                print(f"  Grammar  : {data['purusha']} {data['vacana']}")
                print("-" * 70)

            elif item['type'] == 'krdanta':
                subanta = data.get('subanta_info', 'Pratipadika / Prathama')
                print(f"[Krdanta]")
                print(f"  Prefixes     : {prefix_str}")
                print(f"  Root         : {root_name} ({data['dhatu_id']})")
                print(f"  Krdanta Name : {data['pratyaya']}")
                print(f"  Subanta Note : {subanta}")
                print("-" * 70)
        
        analyzer.close()

    # --- GENERATION BRANCH ---
    elif args.generate:
        if not all([args.root, args.lakara, args.purusha, args.vacana]):
            print("\nError: Generation requires --root, --lakara, --purusha, and --vacana")
            sys.exit(1)

        gen = SanskritGenerator()
        prefix_list = args.prefixes.split(',') if args.prefixes else []
        
        word = gen.generate_tinanta(
            root_or_id=args.root, 
            lakara=args.lakara, 
            purusha=args.purusha, 
            vacana=args.vacana, 
            prefixes=prefix_list
        )
        
        if word:
            print(f"\nResult: {word}")
        else:
            print(f"\nForm not found for Root: {args.root}, Lakara: {args.lakara}")
        
        gen.analyzer.close()

if __name__ == "__main__":
    main()