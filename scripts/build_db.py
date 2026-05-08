import sqlite3
import glob
import json
import os
from pathlib import Path
from indic_transliteration import sanscript

def dev2slp(text):
    if not text: return ""
    return sanscript.transliterate(text, sanscript.DEVANAGARI, sanscript.SLP1)

# Inside scripts/build_db.py

def setup_database(db_path):
    if db_path.exists():
        db_path.unlink()
    conn = sqlite3.connect(db_path)
    c = conn.cursor()

    # Create dhatu_meta with gana
    c.execute('CREATE TABLE dhatu_meta (dhatu_id TEXT PRIMARY KEY, root_slp1 TEXT, gana TEXT)')

    c.execute('''CREATE TABLE tinanta (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        dhatu_id TEXT, derivation TEXT, lakara TEXT,
        purusha TEXT, vacana TEXT, form_slp1 TEXT
    )''')
    
    c.execute('''CREATE TABLE krdanta (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        dhatu_id TEXT, derivation TEXT, pratyaya TEXT, form_slp1 TEXT
    )''')

    c.execute('CREATE INDEX idx_tinanta_form ON tinanta(form_slp1)')
    c.execute('CREATE INDEX idx_krdanta_form ON krdanta(form_slp1)')
    c.execute('CREATE INDEX idx_meta_root ON dhatu_meta(root_slp1)')
    
    return conn, c

def process_data_json(filepath, c):
    """Maps Devanagari root to SLP1 and links to the baseindex ID."""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = json.load(f)
        for entry in content.get('data', []):
            d_id = entry.get('baseindex')  # e.g., "01.0001"
            root_dev = entry.get('dhatu')  # e.g., "भू"
            gana = entry.get('gana')      # e.g., "1"
            
            if d_id and root_dev:
                root_slp = dev2slp(root_dev) # "भू" -> "BU"
                c.execute("INSERT OR IGNORE INTO dhatu_meta (dhatu_id, root_slp1, gana) VALUES (?, ?, ?)",
                          (d_id, root_slp, gana))

def process_tinanta_file(filepath, c):
    derivation = filepath.stem.replace('dhatuforms_', '')
    purushas = ["Prathama", "Madhyama", "Uttama"]
    vacanas = ["Eka", "Dvi", "Bahu"]
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)
    for d_id, lak_dict in data.items():
        for lak, f_str in lak_dict.items():
            form_groups = f_str.split(';')
            if len(form_groups) == 9:
                for idx, f_grp in enumerate(form_groups):
                    for form in f_grp.split(','):
                        clean = form.strip()
                        if clean:
                            c.execute("INSERT INTO tinanta (dhatu_id, derivation, lakara, purusha, vacana, form_slp1) VALUES (?,?,?,?,?,?)",
                                      (d_id, derivation, lak, purushas[idx//3], vacanas[idx%3], dev2slp(clean)))

def process_krdanta_file(filepath, c):
    derivation = filepath.stem.replace('dhatuforms_', '')
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)
    for d_id, pratyaya_dict in data.items():
        for pratyaya, f_str in pratyaya_dict.items():
            for form in f_str.split(','):
                clean = form.strip()
                if clean:
                    c.execute("INSERT INTO krdanta (dhatu_id, derivation, pratyaya, form_slp1) VALUES (?,?,?,?)",
                             (d_id, derivation, dev2slp(pratyaya), dev2slp(clean)))

def main():
    script_dir = Path(__file__).resolve().parent
    project_root = script_dir.parent
    raw_data_dir = project_root / "data_raw"
    db_path = project_root / "sktmorph" / "data" / "sanskrit_forms.sqlite"

    db_path.parent.mkdir(parents=True, exist_ok=True)
    conn, c = setup_database(db_path)

    # 1. Process Metadata (data.txt)
    data_file = raw_data_dir / "data.txt"
    if data_file.exists():
        print(f"Processing Metadata: {data_file.name}")
        process_data_json(data_file, c)

    # 2. Process Forms
    txt_files = glob.glob(str(raw_data_dir / "dhatuforms_*.txt"))
    for f_str in txt_files:
        path = Path(f_str)
        print(f"Processing Forms: {path.name}")
        if item['type'] == 'tinanta':
            voice = format_voice(data['derivation'])
            root_display = data.get('root_slp1', 'Unknown')
            print(f"[Conjugation]")
            print(f"  Prefixes : {prefix_str}")
            print(f"  Root     : {root_display} ({data['dhatu_id']})")
            print(f"  Voice    : {voice}")
        if 'kartari' in path.name or 'karmani' in path.name:
            process_tinanta_file(path, c)
        elif 'krut' in path.name:
            process_krdanta_file(path, c)

    conn.commit()
    conn.close()
    print("Database build complete! 🚀")

if __name__ == "__main__":
    main()