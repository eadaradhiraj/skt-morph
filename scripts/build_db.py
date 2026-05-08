import json
import sqlite3
import os
import glob
from indic_transliteration import sanscript
from indic_transliteration.sanscript import transliterate

# Define Absolute Paths
INPUT_DIR = "/home/edhiraj/Documents/projs/skt-morph/data_raw"
OUTPUT_DIR = "/home/edhiraj/Documents/projs/skt-morph/sktmorph/data"
DB_NAME = os.path.join(OUTPUT_DIR, "sanskrit_forms.sqlite")

def to_slp1(text):
    if not text: return ""
    return transliterate(text, sanscript.DEVANAGARI, sanscript.SLP1)

def init_db():
    # Ensure output directory exists
    if not os.path.exists(OUTPUT_DIR):
        os.makedirs(OUTPUT_DIR)
        
    if os.path.exists(DB_NAME):
        os.remove(DB_NAME)
    
    conn = sqlite3.connect(DB_NAME)
    cursor = conn.cursor()
    
    # 1. Dhatus (from data.txt)
    cursor.execute('''CREATE TABLE dhatus (
                        dhatu_id TEXT PRIMARY KEY,
                        details_json TEXT
                      )''')
                      
    # 2. Conjugations
    cursor.execute('''CREATE TABLE tinantas (
                        dhatu_id TEXT,
                        derivation TEXT,
                        prayoga TEXT,
                        lakara TEXT,
                        purusha INTEGER,
                        vacana INTEGER,
                        form_slp1 TEXT
                      )''')
                      
    # 3. Participles/Nouns
    cursor.execute('''CREATE TABLE krdantas (
                        dhatu_id TEXT,
                        derivation TEXT,
                        pratyaya TEXT,
                        form_slp1 TEXT
                      )''')

    # 4. Literary Prayogas (from dhatuprayogas.txt)
    cursor.execute('''CREATE TABLE prayogas (
                        dhatu_id TEXT,
                        form_slp1 TEXT,
                        book TEXT,
                        details_json TEXT
                      )''')

    # Create Indexes for lightning-fast analysis
    cursor.execute('CREATE INDEX idx_tinanta_form ON tinantas(form_slp1)')
    cursor.execute('CREATE INDEX idx_krdanta_form ON krdantas(form_slp1)')
    cursor.execute('CREATE INDEX idx_tinanta_dhatu ON tinantas(dhatu_id)')
    cursor.execute('CREATE INDEX idx_krdanta_dhatu ON krdantas(dhatu_id)')
    cursor.execute('CREATE INDEX idx_prayoga_form ON prayogas(form_slp1)')
    
    conn.commit()
    return conn

def process_data_txt(conn):
    filepath = os.path.join(INPUT_DIR, "data.txt")
    if not os.path.exists(filepath):
        print(f"Skipping {filepath} (Not found)")
        return

    print(f"Processing Dhatus: {filepath}")
    cursor = conn.cursor()
    
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)
        
    for item in data.get("data",[]):
        raw_id = item.get("i", "")
        # Convert "1010001" to "01.0001" to match other files
        if len(raw_id) == 7:
            dhatu_id = f"{raw_id[1:3]}.{raw_id[3:]}"
        else:
            dhatu_id = raw_id
            
        details_json = json.dumps(item, ensure_ascii=False)
        cursor.execute(
            "INSERT OR REPLACE INTO dhatus VALUES (?, ?)",
            (dhatu_id, details_json)
        )
    conn.commit()

def process_prayogas_txt(conn):
    filepath = os.path.join(INPUT_DIR, "dhatuprayogas.txt")
    if not os.path.exists(filepath):
        return

    print(f"Processing Prayogas: {filepath}")
    cursor = conn.cursor()
    
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)
        
    for dhatu_id, forms in data.items():
        for key, usages in forms.items():
            # Example key: "plang_1_1_अभवत्" -> Extract "अभवत्"
            parts = key.split('_')
            form_dev = parts[-1] if len(parts) > 0 else key
            form_slp1 = to_slp1(form_dev)
            
            for usage in usages:
                book = usage.get("book", "")
                details_json = json.dumps(usage, ensure_ascii=False)
                cursor.execute(
                    "INSERT INTO prayogas VALUES (?, ?, ?, ?)",
                    (dhatu_id, form_slp1, book, details_json)
                )
    conn.commit()

def process_tinanta_file(conn, filepath, derivation, prayoga):
    print(f"Processing Tinantas: {os.path.basename(filepath)}")
    cursor = conn.cursor()
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)
        
    for dhatu_id, lakaras in data.items():
        for lakara, forms_str in lakaras.items():
            cells = forms_str.split(';')
            if len(cells) != 9:
                continue
            
            for i, cell in enumerate(cells):
                purusha = (i // 3) + 1  # 1: Prathama, 2: Madhyama, 3: Uttama
                vacana = (i % 3) + 1    # 1: Eka, 2: Dvi, 3: Bahu
                
                for form in cell.split(','):
                    form_slp1 = to_slp1(form.strip())
                    if form_slp1:
                        cursor.execute(
                            "INSERT INTO tinantas VALUES (?, ?, ?, ?, ?, ?, ?)",
                            (dhatu_id, derivation, prayoga, lakara, purusha, vacana, form_slp1)
                        )
    conn.commit()

def process_krdanta_file(conn, filepath, derivation):
    print(f"Processing Krdantas: {os.path.basename(filepath)}")
    cursor = conn.cursor()
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)
        
    for dhatu_id, pratyayas in data.items():
        for pratyaya, forms_str in pratyayas.items():
            forms =[f.strip() for f in forms_str.split(',') if f.strip()]
            
            for form in forms:
                form_slp1 = to_slp1(form)
                pratyaya_slp1 = to_slp1(pratyaya)
                if form_slp1:
                    cursor.execute(
                        "INSERT INTO krdantas VALUES (?, ?, ?, ?)",
                        (dhatu_id, derivation, pratyaya_slp1, form_slp1)
                    )
    conn.commit()

def main():
    conn = init_db()
    
    # 1. Process Master Dhatu Data
    process_data_txt(conn)
    
    # 2. Process Usage data
    process_prayogas_txt(conn)
    
    # 3. Process all Dhatu Forms
    search_pattern = os.path.join(INPUT_DIR, "dhatuforms_*.txt")
    for filepath in glob.glob(search_pattern):
        filename = os.path.basename(filepath)
        parts = filename.replace(".txt", "").split("_")
        
        if "krut" in parts or filename == "dhatuforms_krut.txt":
            derivation = "shuddha"
            if len(parts) > 2 and parts[1] == "vidyut":
                derivation = parts[2]
            process_krdanta_file(conn, filepath, derivation)
            
        elif "kartari" in parts or "karmani" in parts:
            derivation = parts[2]
            prayoga = parts[3]
            process_tinanta_file(conn, filepath, derivation, prayoga)
            
    print(f"Database successfully built at:\n{DB_NAME}")
    conn.close()

if __name__ == "__main__":
    main()