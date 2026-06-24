import json
import sqlite3
import os
import glob
from indic_transliteration import sanscript
from indic_transliteration.sanscript import transliterate

INPUT_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "data_raw"))
OUTPUT_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "sktmorph", "data"))
DB_DHATUS = os.path.join(OUTPUT_DIR, "dhatus.sqlite")
DERIVATIONS =['shuddha', 'nich', 'san', 'yang', 'yangluk']
SHARDS = ['gana1', 'gana2_to_10']

def to_slp1(text):
    if not text: return ""
    return transliterate(text, sanscript.DEVANAGARI, sanscript.SLP1)

def to_slp1_recursive(data):
    if isinstance(data, str):
        return to_slp1(data)
    elif isinstance(data, dict):
        return {to_slp1(k): to_slp1_recursive(v) for k, v in data.items()}
    elif isinstance(data, list):
        return [to_slp1_recursive(item) for item in data]
    return data

def get_shard(dhatu_id):
    return 'gana1' if dhatu_id.startswith('01.') else 'gana2_to_10'

def init_dbs():
    if not os.path.exists(OUTPUT_DIR): os.makedirs(OUTPUT_DIR)
    for db in glob.glob(os.path.join(OUTPUT_DIR, "*.sqlite")): os.remove(db)
    
    conn_d = sqlite3.connect(DB_DHATUS)
    conn_d.execute('CREATE TABLE dhatus (dhatu_id TEXT PRIMARY KEY, details_json TEXT)')
    
    conn_k_dict = {}
    for shard in SHARDS:
        conn = sqlite3.connect(os.path.join(OUTPUT_DIR, f"krdantas_{shard}.sqlite"))
        conn.execute('CREATE TABLE krdantas (dhatu_id TEXT, derivation TEXT, pratyaya TEXT, form_slp1 TEXT)')
        conn.execute('CREATE INDEX idx_krdanta_form ON krdantas(form_slp1)')
        conn.execute('CREATE INDEX idx_krdanta_dhatu ON krdantas(dhatu_id)')
        conn_k_dict[shard] = conn
    
    conn_t_dict = {}
    for der in DERIVATIONS:
        conn_t_dict[der] = {}
        for shard in SHARDS:
            conn = sqlite3.connect(os.path.join(OUTPUT_DIR, f"tinantas_{der}_{shard}.sqlite"))
            conn.execute('CREATE TABLE tinantas (dhatu_id TEXT, derivation TEXT, prayoga TEXT, lakara TEXT, purusha INTEGER, vacana INTEGER, form_slp1 TEXT)')
            conn.execute('CREATE INDEX idx_tinanta_form ON tinantas(form_slp1)')
            conn.execute('CREATE INDEX idx_tinanta_dhatu ON tinantas(dhatu_id)')
            conn_t_dict[der][shard] = conn
            
    return conn_d, conn_k_dict, conn_t_dict

def process_data_txt(conn):
    filepath = os.path.join(INPUT_DIR, "data.txt")
    if not os.path.exists(filepath): return
    with open(filepath, 'r', encoding='utf-8') as f: data = json.load(f)
    for item in data.get("data",[]):
        raw_id = item.get("i", "")
        dhatu_id = f"{raw_id[1:3]}.{raw_id[3:]}" if len(raw_id) == 7 else raw_id
        slp1_item = to_slp1_recursive(item)
        conn.execute("INSERT OR REPLACE INTO dhatus VALUES (?, ?)", (dhatu_id, json.dumps(slp1_item, ensure_ascii=False)))
    conn.commit()

def convert_p_to_a_a_anta(form_p, lakara, cell_idx):
    mapping = {
        'plat': [('ti', 'te'), ('ataH', 'ete'), ('anti', 'ante'), ('si', 'se'), ('aTaH', 'eTe'), ('aTa', 'aDve'), ('Ami', 'e'), ('AvaH', 'Avahe'), ('AmaH', 'Amahe')],
        'plrt': [('ti', 'te'), ('ataH', 'ete'), ('anti', 'ante'), ('si', 'se'), ('aTaH', 'eTe'), ('aTa', 'aDve'), ('Ami', 'e'), ('AvaH', 'Avahe'), ('AmaH', 'Amahe')],
        'plot': [('atu', 'atAm'), ('atAm', 'etAm'), ('antu', 'antAm'), ('a', 'asva'), ('atam', 'eTAm'), ('ata', 'aDvam'), ('Ani', 'E'), ('Ava', 'AvahE'), ('Ama', 'AmahE')],
        'plan': [('at', 'ata'), ('atAm', 'etAm'), ('an', 'anta'), ('aH', 'aTAH'), ('atam', 'eTAm'), ('ata', 'aDvam'), ('am', 'e'), ('Ava', 'Avahi'), ('Ama', 'Amahi')],
        'pvidhilin': [('et', 'eta'), ('etAm', 'eyAtAm'), ('eyuH', 'eran'), ('eH', 'eTAH'), ('etam', 'eyATAm'), ('eta', 'eDvam'), ('eyam', 'eya'), ('eva', 'evahi'), ('ema', 'emahi')]
    }
    if lakara not in mapping: return None
    p_suf, a_suf = mapping[lakara][cell_idx]
    
    if lakara == 'plot' and cell_idx == 3:
        if form_p.endswith('tAt'): return form_p[:-3] + 'sva'
        if form_p.endswith('hi'): return form_p[:-2] + 'sva'
            
    if form_p.endswith(p_suf): return form_p[:-len(p_suf)] + a_suf
    return None

def process_tinanta_file(conn_dict, filepath, derivation, prayoga):
    if derivation not in conn_dict: return
    INJECT_A_ROOTS = {"01.1137", "01.0631", "01.1077"} # gam, ji, sTA
    with open(filepath, 'r', encoding='utf-8') as f: data = json.load(f)
    for dhatu_id, lakaras in data.items():
        shard = get_shard(dhatu_id)
        cursor = conn_dict[derivation][shard].cursor()
        for lakara, forms_str in lakaras.items():
            cells = forms_str.split(';')
            if len(cells) != 9: continue
            for i, cell in enumerate(cells):
                for form in cell.split(','):
                    f_slp1 = to_slp1(form.strip())
                    if f_slp1:
                        cursor.execute("INSERT INTO tinantas VALUES (?, ?, ?, ?, ?, ?, ?)",
                                       (dhatu_id, derivation, prayoga, lakara, (i//3)+1, (i%3)+1, f_slp1))
                        if prayoga == "kartari" and dhatu_id in INJECT_A_ROOTS:
                            a_form = convert_p_to_a_a_anta(f_slp1, lakara, i)
                            if a_form:
                                a_lakara = "a" + lakara[1:]
                                cursor.execute("INSERT INTO tinantas VALUES (?, ?, ?, ?, ?, ?, ?)",
                                               (dhatu_id, derivation, prayoga, a_lakara, (i//3)+1, (i%3)+1, a_form))

def process_krdanta_file(conn_dict, filepath, derivation):
    INJECT_A_ROOTS = {"01.1137", "01.0631", "01.1077"}
    with open(filepath, 'r', encoding='utf-8') as f: data = json.load(f)
    for dhatu_id, pratyayas in data.items():
        shard = get_shard(dhatu_id)
        cursor = conn_dict[shard].cursor()
        for pratyaya, forms_str in pratyayas.items():
            for form in [f.strip() for f in forms_str.split(',') if f.strip()]:
                f_slp1 = to_slp1(form)
                if f_slp1:
                    cursor.execute("INSERT INTO krdantas VALUES (?, ?, ?, ?)",
                                   (dhatu_id, derivation, to_slp1(pratyaya), f_slp1))
                    
                    # Inject SAnac (Atmanepada participle) using Satf as the base
                    if to_slp1(pratyaya) == "Satf" and dhatu_id in INJECT_A_ROOTS:
                        if f_slp1.endswith('at'):
                            a_form = f_slp1[:-1] + 'mAna' # e.g. tizWat -> tizWamAna
                            cursor.execute("INSERT INTO krdantas VALUES (?, ?, ?, ?)",
                                           (dhatu_id, derivation, "SAnac", a_form))

def main():
    print("Building balanced Shard Databases (Fully converting all JSON to SLP1)...")
    conn_d, conn_k_dict, conn_t_dict = init_dbs()
    process_data_txt(conn_d)
    
    for filepath in glob.glob(os.path.join(INPUT_DIR, "dhatuforms_*.txt")):
        parts = os.path.basename(filepath).replace(".txt", "").split("_")
        if "krut" in parts or "krut.txt" in filepath:
            process_krdanta_file(conn_k_dict, filepath, parts[2] if len(parts) > 2 and parts[1] == "vidyut" else "shuddha")
        elif "kartari" in parts or "karmani" in parts:
            process_tinanta_file(conn_t_dict, filepath, parts[2], parts[3])
            
    for conns in conn_k_dict.values(): conns.commit(); conns.close()
    for der in conn_t_dict.values():
        for conn in der.values(): conn.commit(); conn.close()
    print("Successfully built and converted all databases to SLP1!")

if __name__ == "__main__":
    main()
