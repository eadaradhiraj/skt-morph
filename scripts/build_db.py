import json
import sqlite3
import os
import glob
from indic_transliteration import sanscript
from indic_transliteration.sanscript import transliterate

INPUT_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "data_raw"))
OUTPUT_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "sktmorph", "data"))

DB_DHATUS = os.path.join(OUTPUT_DIR, "dhatus.sqlite")
DB_KRDANTAS = os.path.join(OUTPUT_DIR, "krdantas.sqlite")
DERIVATIONS = ['shuddha', 'nich', 'san', 'yang', 'yangluk']

def to_slp1(text):
    if not text: return ""
    return transliterate(text, sanscript.DEVANAGARI, sanscript.SLP1)

def init_dbs():
    if not os.path.exists(OUTPUT_DIR):
        os.makedirs(OUTPUT_DIR)
        
    # Delete all old sqlite files to start fresh
    for db in glob.glob(os.path.join(OUTPUT_DIR, "*.sqlite")):
        os.remove(db)
    
    conn_d = sqlite3.connect(DB_DHATUS)
    conn_k = sqlite3.connect(DB_KRDANTAS)
    
    conn_d.execute('CREATE TABLE dhatus (dhatu_id TEXT PRIMARY KEY, details_json TEXT)')
    
    conn_k.execute('CREATE TABLE krdantas (dhatu_id TEXT, derivation TEXT, pratyaya TEXT, form_slp1 TEXT)')
    conn_k.execute('CREATE INDEX idx_krdanta_form ON krdantas(form_slp1)')
    conn_k.execute('CREATE INDEX idx_krdanta_dhatu ON krdantas(dhatu_id)')
    
    conn_t_dict = {}
    for der in DERIVATIONS:
        path = os.path.join(OUTPUT_DIR, f"tinantas_{der}.sqlite")
        conn = sqlite3.connect(path)
        conn.execute('CREATE TABLE tinantas (dhatu_id TEXT, derivation TEXT, prayoga TEXT, lakara TEXT, purusha INTEGER, vacana INTEGER, form_slp1 TEXT)')
        conn.execute('CREATE INDEX idx_tinanta_form ON tinantas(form_slp1)')
        conn.execute('CREATE INDEX idx_tinanta_dhatu ON tinantas(dhatu_id)')
        conn_t_dict[der] = conn
        
    conn_d.commit()
    conn_k.commit()
    for conn in conn_t_dict.values(): conn.commit()
    
    return conn_d, conn_k, conn_t_dict

def process_data_txt(conn):
    filepath = os.path.join(INPUT_DIR, "data.txt")
    if not os.path.exists(filepath): return
    cursor = conn.cursor()
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)
    for item in data.get("data",[]):
        raw_id = item.get("i", "")
        dhatu_id = f"{raw_id[1:3]}.{raw_id[3:]}" if len(raw_id) == 7 else raw_id
        cursor.execute("INSERT OR REPLACE INTO dhatus VALUES (?, ?)", (dhatu_id, json.dumps(item, ensure_ascii=False)))
    conn.commit()

def process_tinanta_file(conn_dict, filepath, derivation, prayoga):
    if derivation not in conn_dict: return
    cursor = conn_dict[derivation].cursor()
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)
    for dhatu_id, lakaras in data.items():
        for lakara, forms_str in lakaras.items():
            cells = forms_str.split(';')
            if len(cells) != 9: continue
            for i, cell in enumerate(cells):
                for form in cell.split(','):
                    form_slp1 = to_slp1(form.strip())
                    if form_slp1:
                        cursor.execute("INSERT INTO tinantas VALUES (?, ?, ?, ?, ?, ?, ?)",
                                       (dhatu_id, derivation, prayoga, lakara, (i // 3) + 1, (i % 3) + 1, form_slp1))
    conn_dict[derivation].commit()

def process_krdanta_file(conn, filepath, derivation):
    cursor = conn.cursor()
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)
    for dhatu_id, pratyayas in data.items():
        for pratyaya, forms_str in pratyayas.items():
            for form in[f.strip() for f in forms_str.split(',') if f.strip()]:
                form_slp1 = to_slp1(form)
                if form_slp1:
                    cursor.execute("INSERT INTO krdantas VALUES (?, ?, ?, ?)",
                                   (dhatu_id, derivation, to_slp1(pratyaya), form_slp1))
    conn.commit()

def main():
    print("Building databases (Splitting Tinantas into 5 derivations)...")
    conn_d, conn_k, conn_t_dict = init_dbs()
    process_data_txt(conn_d)
    
    search_pattern = os.path.join(INPUT_DIR, "dhatuforms_*.txt")
    for filepath in glob.glob(search_pattern):
        filename = os.path.basename(filepath)
        parts = filename.replace(".txt", "").split("_")
        if "krut" in parts or filename == "dhatuforms_krut.txt":
            derivation = parts[2] if len(parts) > 2 and parts[1] == "vidyut" else "shuddha"
            process_krdanta_file(conn_k, filepath, derivation)
        elif "kartari" in parts or "karmani" in parts:
            process_tinanta_file(conn_t_dict, filepath, parts[2], parts[3])
            
    print("Databases built successfully!")
    conn_d.close()
    conn_k.close()
    for conn in conn_t_dict.values(): conn.close()

if __name__ == "__main__":
    main()
