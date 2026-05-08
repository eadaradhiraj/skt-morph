import sqlite3
import glob
import json
import os
from pathlib import Path
from indic_transliteration import sanscript

# --- Configuration & Setup ---

def dev2slp(text):
    """Convert Devanagari to SLP1."""
    if not text:
        return ""
    return sanscript.transliterate(text, sanscript.DEVANAGARI, sanscript.SLP1)

def setup_database(db_path):
    """Create the SQLite database and schemas."""
    if db_path.exists():
        db_path.unlink()
        
    conn = sqlite3.connect(db_path)
    c = conn.cursor()

    c.execute('''CREATE TABLE tinanta (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        dhatu_id TEXT,
        derivation TEXT,
        lakara TEXT,
        purusha TEXT,
        vacana TEXT,
        form_slp1 TEXT
    )''')
    
    c.execute('''CREATE TABLE krdanta (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        dhatu_id TEXT,
        derivation TEXT,
        pratyaya TEXT,
        form_slp1 TEXT
    )''')

    c.execute('CREATE INDEX idx_tinanta_form ON tinanta(form_slp1)')
    c.execute('CREATE INDEX idx_krdanta_form ON krdanta(form_slp1)')
    
    return conn, c

# --- Parsing Logic ---

def process_tinanta_file(filepath, c):
    """Parses kartari/karmani files (Conjugations)."""
    derivation = filepath.stem.replace('dhatuforms_', '')
    purushas =["Prathama", "Madhyama", "Uttama"]
    vacanas = ["Eka", "Dvi", "Bahu"]

    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)

    for dhatu_id, lakara_dict in data.items():
        for lakara, forms_str in lakara_dict.items():
            form_groups = forms_str.split(';')
            if len(form_groups) == 9:
                for idx, form_group in enumerate(form_groups):
                    p_idx = idx // 3
                    v_idx = idx % 3
                    for form in form_group.split(','):
                        clean_form = form.strip()
                        if clean_form:
                            c.execute("""
                                INSERT INTO tinanta (dhatu_id, derivation, lakara, purusha, vacana, form_slp1)
                                VALUES (?, ?, ?, ?, ?, ?)
                            """, (dhatu_id, derivation, lakara, purushas[p_idx], vacanas[v_idx], dev2slp(clean_form)))

def process_krdanta_file(filepath, c):
    """Parses krut files (Krdantas)."""
    derivation = filepath.stem.replace('dhatuforms_', '')
    
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)

    for dhatu_id, pratyaya_dict in data.items():
        for pratyaya, forms_str in pratyaya_dict.items():
            for form in forms_str.split(','):
                clean_form = form.strip()
                if clean_form:
                    c.execute("""
                         INSERT INTO krdanta (dhatu_id, derivation, pratyaya, form_slp1)
                         VALUES (?, ?, ?, ?)
                    """, (dhatu_id, derivation, dev2slp(pratyaya), dev2slp(clean_form)))

# --- Main Execution ---

def main():
    # Define absolute paths
    script_dir = Path(__file__).resolve().parent
    project_root = script_dir.parent
    raw_data_dir = project_root / "data_raw"
    db_path = project_root / "sktmorph" / "data" / "sanskrit_forms.sqlite"

    # Prepare directories
    db_path.parent.mkdir(parents=True, exist_ok=True)
    
    print(f"Building Database at: {db_path}")
    conn, c = setup_database(db_path)

    # Process all relevant files
    txt_files = glob.glob(str(raw_data_dir / "dhatuforms_*.txt"))
    
    for filepath_str in txt_files:
        path = Path(filepath_str)
        print(f"Processing: {path.name}")
        
        if 'kartari' in path.name or 'karmani' in path.name:
            process_tinanta_file(path, c)
        elif 'krut' in path.name:
            process_krdanta_file(path, c)
        else:
            print(f"  > Skipping unknown file type: {path.name}")

    conn.commit()
    conn.close()
    print("Database build complete! 🚀")

if __name__ == "__main__":
    main()