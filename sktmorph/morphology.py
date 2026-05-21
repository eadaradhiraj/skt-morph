import sqlite3
import os
import json
import re
import glob
from dataclasses import dataclass
from typing import List, Tuple, Dict, Any, Optional
from .subanta import SubantaGenerator
from .sarvanama import SarvanamaGenerator

@dataclass
class MorphResult:
    word: str
    prefixes: List[str]
    dhatu: Optional[str]
    word_type: str  
    derivation: Optional[str] 
    prayoga: Optional[str] = None
    lakara: Optional[str] = None
    purusha: Optional[int] = None
    vacana: Optional[int] = None
    pratyaya: Optional[str] = None
    dhatu_details: Optional[Dict[str, Any]] = None
    pratipadika: Optional[str] = None
    linga: Optional[str] = None
    vibhakti: Optional[str] = None

PADA_RESTRICTIONS = {
    ("09.0001", "vi"): "A",
    ("09.0001", "pari"): "A",
    ("09.0001", "ava"): "A",
    ("01.0631", "vi"): "A",
    ("01.0631", "parA"): "A"
}

UPASARGA_SPLIT_RULES: List[Tuple[str, str, str]] =[
    ("aDo", "aDas", ""), ("aDas", "aDas", ""), ("aDaH", "aDas", ""),
    ("puro", "puras", ""), ("puras", "puras", ""), ("puraH", "puras", ""),
    ("tiro", "tiras", ""), ("tiras", "tiras", ""), ("tiraH", "tiras", ""),
    ("antar", "antar", ""), ("alam", "alam", ""), ("alaM", "alam", ""),
    ("prAdur", "prAdus", ""), ("prAduz", "prAdus", ""), ("prAduH", "prAdus", ""),
    ("Avir", "Avis", ""), ("Aviz", "Avis", ""), ("AviH", "Avis", ""),
    
    # "a/A" ending Upasargas
    ("prA", "pra", "a"), ("prA", "pra", "A"), ("pre", "pra", "i"), ("pre", "pra", "I"), ("pro", "pra", "u"), ("pro", "pra", "U"), ("prAr", "pra", "f"), ("prAr", "pra", "F"), ("pra", "pra", ""),
    ("upA", "upa", "a"), ("upA", "upa", "A"), ("upe", "upa", "i"), ("upe", "upa", "I"), ("upo", "upa", "u"), ("upo", "upa", "U"), ("upAr", "upa", "f"), ("upAr", "upa", "F"), ("upa", "upa", ""),
    ("avA", "ava", "a"), ("avA", "ava", "A"), ("ave", "ava", "i"), ("ave", "ava", "I"), ("avo", "ava", "u"), ("avo", "ava", "U"), ("avAr", "ava", "f"), ("avAr", "ava", "F"), ("ava", "ava", ""),
    ("apA", "apa", "a"), ("apA", "apa", "A"), ("ape", "apa", "i"), ("ape", "apa", "I"), ("apo", "apa", "u"), ("apo", "apa", "U"), ("apAr", "apa", "f"), ("apAr", "apa", "F"), ("apa", "apa", ""),
    ("A", "A", "a"), ("A", "A", "A"), ("e", "A", "i"), ("e", "A", "I"), ("o", "A", "u"), ("o", "A", "U"), ("Ar", "A", "f"), ("Ar", "A", "F"), ("A", "A", ""),
    ("parA", "parA", "a"), ("parA", "parA", "A"), ("pare", "parA", "i"), ("pare", "parA", "I"), ("paro", "parA", "u"), ("paro", "parA", "U"), ("parA", "parA", ""),
    
    # "i/I" ending Upasargas (Using Generalized Yan Sandhi Rules)
    ("vI", "vi", "i"), ("vI", "vi", "I"), ("vy", "vi", ""), ("vi", "vi", ""),
    ("nI", "ni", "i"), ("nI", "ni", "I"), ("ny", "ni", ""), ("ni", "ni", ""),
    ("aBI", "aBi", "i"), ("aBI", "aBi", "I"), ("aBy", "aBi", ""), ("aBi", "aBi", ""),
    ("pratI", "prati", "i"), ("pratI", "prati", "I"), ("praty", "prati", ""), ("prati", "prati", ""),
    ("parI", "pari", "i"), ("parI", "pari", "I"), ("pary", "pari", ""), ("pari", "pari", ""),
    ("atI", "ati", "i"), ("atI", "ati", "I"), ("aty", "ati", ""), ("ati", "ati", ""),
    ("aDI", "aDi", "i"), ("aDI", "aDi", "I"), ("aDy", "aDi", ""), ("aDi", "aDi", ""),
    ("apI", "api", "i"), ("apI", "api", "I"), ("apy", "api", ""), ("api", "api", ""),
    
    # "u/U" ending Upasargas (Using Generalized Yan Sandhi Rules)
    ("sU", "su", "u"), ("sU", "su", "U"), ("sv", "su", ""), ("su", "su", ""),
    ("anU", "anu", "u"), ("anU", "anu", "U"), ("anv", "anu", ""), ("anu", "anu", ""),
    
    # Consonant ending Upasargas
    ("saM", "sam", ""), ("saY", "sam", ""), ("saN", "sam", ""), ("saR", "sam", ""), ("san", "sam", ""), ("sam", "sam", ""),
    ("uddh", "ud", "h"), ("ut", "ud", ""), ("uc", "ud", "c"), ("uj", "ud", "j"), ("ul", "ud", "l"), ("ud", "ud", ""),
    ("nir", "nis", ""), ("niz", "nis", ""), ("niS", "nis", ""), ("nih", "nis", ""), ("nis", "nis", ""),
    ("dur", "dus", ""), ("duz", "dus", ""), ("duS", "dus", ""), ("duh", "dus", ""), ("dus", "dus", "")
]

def apply_forward_sandhi(prefix: str, word: str) -> str:
    if not prefix: return word
    word = word.lstrip('-')
    p_end = prefix[-1]
    w_start = word[0]
    w_rest = word[1:]
    vowels = {'a', 'A', 'i', 'I', 'u', 'U', 'f', 'F', 'e', 'E', 'o', 'O'}
    voiced_cons = {'g','G','j','J','q','Q','d','D','b','B','N','Y','R','n','m','y','r','l','v','h'}
    unvoiced_cons = {'k','K','c','C','w','W','t','T','p','P','S','z','s'}
    
    result = prefix + word
    if p_end in vowels and w_start == 'C': result = prefix + 'c' + word
    
    if p_end == 's':
        if prefix.endswith('as'):
            if w_start in voiced_cons: result = prefix[:-2] + 'o' + word
            elif w_start == 'a': result = prefix[:-2] + 'o' + w_rest
            elif w_start in vowels: result = prefix[:-2] + 'a' + word
            elif w_start in ['c', 'C']: result = prefix[:-1] + 'S' + word
            elif w_start in ['w', 'W']: result = prefix[:-1] + 'z' + word
            elif w_start in ['k', 'K', 'p', 'P']: result = prefix[:-1] + 'H' + word
        elif prefix.endswith('is') or prefix.endswith('us'):
            if w_start in voiced_cons or w_start in vowels: result = prefix[:-1] + 'r' + word
            elif w_start in ['c', 'C']: result = prefix[:-1] + 'S' + word
            elif w_start in ['w', 'W']: result = prefix[:-1] + 'z' + word
            elif w_start in ['k', 'K', 'p', 'P']: result = prefix[:-1] + 'z' + word
    elif p_end in ['a', 'A']:
        if w_start in ['a', 'A']: result = prefix[:-1] + 'A' + w_rest
        elif w_start in ['i', 'I']: result = prefix[:-1] + 'e' + w_rest
        elif w_start in ['u', 'U']: result = prefix[:-1] + 'o' + w_rest
        elif w_start in ['f', 'F']: result = prefix[:-1] + 'Ar' + w_rest
        elif w_start == 'e': result = prefix[:-1] + 'e' + w_rest
        elif w_start == 'E': result = prefix[:-1] + 'E' + w_rest
        elif w_start == 'o': result = prefix[:-1] + 'O' + w_rest
        elif w_start == 'O': result = prefix[:-1] + 'O' + w_rest
    elif p_end in ['i', 'I']:
        if w_start in vowels and w_start not in ['i', 'I']: result = prefix[:-1] + 'y' + word
        elif w_start in ['i', 'I']: result = prefix[:-1] + 'I' + w_rest
    elif p_end in ['u', 'U']:
        if w_start in vowels and w_start not in ['u', 'U']: result = prefix[:-1] + 'v' + word
        elif w_start in ['u', 'U']: result = prefix[:-1] + 'U' + w_rest
    elif prefix.endswith('m') and (w_start in voiced_cons or w_start in unvoiced_cons):
        result = prefix[:-1] + 'M' + word
    elif prefix == 'ud':
        if w_start in ['k', 'K', 'c', 'C', 'w', 'W', 't', 'T', 'p', 'P', 's', 'S', 'z']: result = 'ut' + word
        elif w_start == 'h': result = 'uddh' + w_rest
        
    trigger = False
    blockers = set('cCjJYSwWqQRtTdDlsS')
    chars = list(result)
    prefix_len = len(result) - len(word)
    natva_prefixes = {'pra', 'parA', 'pari', 'nis', 'dus', 'antar'}
    for i, char in enumerate(chars):
        if char in ['r', 'f', 'F', 'z']:
            if i < prefix_len and prefix not in natva_prefixes:
                continue
            trigger = True
        elif trigger and char == 'n':
            if i != len(chars) - 1: chars[i] = 'R'
        elif trigger and char in blockers:
            trigger = False
                
    return "".join(chars)

class SktMorph:
    def __init__(self, db_dir: str = None):
        if db_dir is None:
            db_dir = os.path.join(os.path.dirname(__file__), 'data')
        self.db_dir = db_dir
        
        main_db = os.path.join(self.db_dir, 'dhatus.sqlite')
        if not os.path.exists(main_db):
            raise FileNotFoundError(f"Database not found at {main_db}.")
            
        self.conn_dhatus = sqlite3.connect(main_db)
        self.conn_dhatus.row_factory = sqlite3.Row
        
        self.tinanta_conns = []
        self.krdanta_conns = []
        
        for f in glob.glob(os.path.join(self.db_dir, 'tinantas_*.sqlite')):
            c = sqlite3.connect(f)
            c.row_factory = sqlite3.Row
            c.execute(f"ATTACH DATABASE '{main_db}' AS ddb")
            self.tinanta_conns.append(c)
            
        for f in glob.glob(os.path.join(self.db_dir, 'krdantas_*.sqlite')):
            c = sqlite3.connect(f)
            c.row_factory = sqlite3.Row
            c.execute(f"ATTACH DATABASE '{main_db}' AS ddb")
            self.krdanta_conns.append(c)

    def get_candidate_splits(self, word: str) -> List[Tuple[List[str], str]]:
        candidates = [([], word)]
        queue = [([], word)]
        visited = set()
        while queue:
            current_prefixes, current_word = queue.pop(0)
            
            # Anti-freeze: Max 4 prefixes to prevent combinatorial explosions
            if len(current_prefixes) >= 4:
                continue
                
            for surface, actual, prepend in UPASARGA_SPLIT_RULES:
                if current_word.startswith(surface):
                    remainder = prepend + current_word[len(surface):]
                    
                    # Anti-freeze: Prevent infinite Savarna Dirgha loops (e.g. A + A = A yielding the exact same word)
                    if remainder == current_word:
                        continue
                        
                    if len(remainder) > 0:
                        new_prefixes = current_prefixes + [actual]
                        state = (tuple(new_prefixes), remainder)
                        if state not in visited:
                            visited.add(state)
                            candidates.append((list(new_prefixes), remainder))
                            queue.append((list(new_prefixes), remainder))
                            
                        if remainder.startswith("cC"):
                            remainder_C = remainder[1:]
                            state_C = (tuple(new_prefixes), remainder_C)
                            if state_C not in visited:
                                visited.add(state_C)
                                candidates.append((list(new_prefixes), remainder_C))
                                queue.append((list(new_prefixes), remainder_C))
                                
                        if 'R' in remainder:
                            remainder_n = remainder.replace('R', 'n')
                            state_n = (tuple(new_prefixes), remainder_n)
                            if state_n not in visited:
                                visited.add(state_n)
                                candidates.append((list(new_prefixes), remainder_n))
                                queue.append((list(new_prefixes), remainder_n))
                                
                        # REVERSE SATVA: Restore s from z for database lookup
                        if 'z' in remainder:
                            remainder_s = remainder.replace('zW', 'sT').replace('zw', 'st').replace('zR', 'sn').replace('z', 's')
                            state_s = (tuple(new_prefixes), remainder_s)
                            if state_s not in visited:
                                visited.add(state_s)
                                candidates.append((list(new_prefixes), remainder_s))
                                queue.append((list(new_prefixes), remainder_s))
                                
        return candidates

    def analyze(self, word_slp1: str) -> List[MorphResult]:
        candidates = self.get_candidate_splits(word_slp1)
        results = []

        for prefixes, base_word in candidates:
            if prefixes:
                reconstructed = base_word
                for p in reversed(prefixes):
                    reconstructed = apply_forward_sandhi(p, reconstructed)
                if reconstructed != word_slp1:
                    # Relax validation slightly to account for Satva (s -> z) which is root-dependent
                    satva_rec = reconstructed.replace('s', 'z').replace('sT', 'zW').replace('st', 'zw').replace('sn', 'zR')
                    if reconstructed != word_slp1 and satva_rec != word_slp1:
                        if reconstructed.replace('n', 'R') != word_slp1 and satva_rec.replace('n', 'R') != word_slp1:
                            continue

            for conn in self.tinanta_conns:
                try:
                    cursor = conn.cursor()
                    cursor.execute("""
                        SELECT t.*, d.details_json 
                        FROM tinantas t 
                        LEFT JOIN ddb.dhatus d ON t.dhatu_id = d.dhatu_id 
                        WHERE t.form_slp1 = ?
                    """, (base_word,))
                    for row in cursor.fetchall():
                        allowed_pada = None
                        for p in prefixes:
                            if (row["dhatu_id"], p) in PADA_RESTRICTIONS:
                                allowed_pada = PADA_RESTRICTIONS[(row["dhatu_id"], p)]
                        
                        if allowed_pada == "A" and row["lakara"].startswith("p"): continue
                        if allowed_pada == "P" and row["lakara"].startswith("a"): continue

                        details = json.loads(row['details_json']) if row['details_json'] else None
                        results.append(MorphResult(
                            word=word_slp1, prefixes=prefixes, dhatu=row['dhatu_id'],
                            word_type='tinanta', derivation=row['derivation'],
                            prayoga=row['prayoga'], lakara=row['lakara'],
                            purusha=row['purusha'], vacana=row['vacana'], dhatu_details=details
                        ))
                except sqlite3.OperationalError: pass
            
            for conn in self.krdanta_conns:
                try:
                    cursor = conn.cursor()
                    # Query bound forms (-lyap), bare stems, and full declined variants.
                    cursor.execute("""
                        SELECT k.*, d.details_json 
                        FROM krdantas k 
                        LEFT JOIN ddb.dhatus d ON k.dhatu_id = d.dhatu_id 
                        WHERE k.form_slp1 IN (?, ?, ?, ?, ?, ?)
                    """, (base_word, base_word + 'm', base_word + 'H', base_word + 'A', base_word + 'I', '-' + base_word))
                    for row in cursor.fetchall():
                        details = json.loads(row['details_json']) if row['details_json'] else None
                        results.append(MorphResult(
                            word=word_slp1, prefixes=prefixes, dhatu=row['dhatu_id'],
                            word_type='krdanta', derivation=row['derivation'],
                            pratyaya=row['pratyaya'], dhatu_details=details
                        ))
                except sqlite3.OperationalError: pass

        for match in SubantaGenerator().analyze(word_slp1):
            prati = match["pratipadika"]
            results.append(MorphResult(
                word=word_slp1, prefixes=[], dhatu=None, word_type="subanta", derivation=None,
                pratipadika=prati, linga=match["linga"],
                vibhakti=match["vibhakti"], vacana=match["vacana"]
            ))
            
            # Intelligent Declined Participle Check (Is this noun actually a Krdanta?)
            prati_splits = self.get_candidate_splits(prati)
            for p_prefixes, p_base in prati_splits:
                if p_prefixes:
                    reconstructed = p_base
                    for p in reversed(p_prefixes):
                        reconstructed = apply_forward_sandhi(p, reconstructed)
                    if reconstructed != prati:
                        satva_rec = reconstructed.replace("s", "z").replace("sT", "zW").replace("st", "zw").replace("sn", "zR")
                        if reconstructed != prati and satva_rec != prati:
                            if reconstructed.replace("n", "R") != prati and satva_rec.replace("n", "R") != prati:
                                continue
                for conn in self.krdanta_conns:
                    try:
                        cursor = conn.cursor()
                        cursor.execute("""
                            SELECT k.*, d.details_json 
                            FROM krdantas k 
                            LEFT JOIN ddb.dhatus d ON k.dhatu_id = d.dhatu_id 
                            WHERE k.form_slp1 IN (?, ?, ?, ?, ?, ?)
                        """, (p_base, p_base + "m", p_base + "H", p_base + "A", p_base + "I", "-" + p_base))
                        for row in cursor.fetchall():
                            details = json.loads(row["details_json"]) if row["details_json"] else None
                            results.append(MorphResult(
                                word=word_slp1, prefixes=p_prefixes, dhatu=row["dhatu_id"],
                                word_type="krdanta", derivation=row["derivation"],
                                pratyaya=row["pratyaya"], dhatu_details=details,
                                pratipadika=prati, linga=match["linga"],
                                vibhakti=match["vibhakti"], vacana=match["vacana"]
                            ))
                    except sqlite3.OperationalError: pass

        for match in SarvanamaGenerator().analyze(word_slp1):
            results.append(MorphResult(
                word=word_slp1, prefixes=[], dhatu=None, word_type="sarvanama", derivation=None,
                pratipadika=match["pratipadika"], linga=match["linga"],
                vibhakti=match["vibhakti"], vacana=match["vacana"]
            ))

        unique_results = []
        seen = set()
        for r in results:
            key = (r.word, tuple(r.prefixes), r.dhatu, r.word_type, r.derivation, r.prayoga, r.lakara, r.purusha, r.vacana, r.pratyaya, r.pratipadika, r.linga, r.vibhakti)
            if key not in seen:
                seen.add(key)
                unique_results.append(r)
        return unique_results

    def resolve_dhatu_ids(self, dhatu_query: str) -> List[str]:
        if re.match(r'^\d{2}\.\d{4}$', dhatu_query): return [dhatu_query]
        try:
            from indic_transliteration import sanscript
            from indic_transliteration.sanscript import transliterate
            dev_query = transliterate(dhatu_query, sanscript.SLP1, sanscript.DEVANAGARI)
        except ImportError:
            dev_query = dhatu_query

        cursor = self.conn_dhatus.cursor()
        cursor.execute("SELECT dhatu_id FROM dhatus WHERE details_json LIKE ? OR details_json LIKE ?", 
                       (f'%"{dev_query}"%', f'%"{dhatu_query}"%'))
        return [row['dhatu_id'] for row in cursor.fetchall()]

    def generate_tinanta(self, dhatu: str, lakara: str, purusha: int, vacana: int, 
                         derivation: str = 'shuddha', prayoga: str = 'kartari', 
                         prefixes: List[str] = None) -> List[str]:
        dhatu_ids = self.resolve_dhatu_ids(dhatu)
        raw_forms = []
        for d_id in dhatu_ids:
            for conn in self.tinanta_conns:
                try:
                    cursor = conn.cursor()
                    cursor.execute('''SELECT form_slp1 FROM tinantas 
                                      WHERE dhatu_id = ? AND lakara = ? AND purusha = ? 
                                      AND vacana = ? AND derivation = ? AND prayoga = ?''',
                                   (d_id, lakara, purusha, vacana, derivation, prayoga))
                    raw_forms.extend([row['form_slp1'] for row in cursor.fetchall()])
                except sqlite3.OperationalError: pass
            
        all_forms = []
        for raw in raw_forms:
            all_forms.extend([f.strip() for f in raw.replace(';', ',').split(',') if f.strip()])
            
        all_forms = sorted(list(set(all_forms)))
        if not all_forms: return []
        if not prefixes: return all_forms
            
        final_forms = []
        for form in all_forms:
            current_form = form
            for p in reversed(prefixes):
                current_form = apply_forward_sandhi(p, current_form)
            final_forms.append(current_form)
            
        return sorted(list(set(final_forms)))

    def generate_krdanta(self, dhatu: str, pratyaya: str, derivation: str = 'shuddha', 
                         prefixes: List[str] = None) -> List[str]:
        dhatu_ids = self.resolve_dhatu_ids(dhatu)
        raw_forms = []
        for d_id in dhatu_ids:
            for conn in self.krdanta_conns:
                try:
                    cursor = conn.cursor()
                    cursor.execute('''SELECT form_slp1 FROM krdantas 
                                      WHERE dhatu_id = ? AND pratyaya = ? AND derivation = ?''',
                                   (d_id, pratyaya, derivation))
                    raw_forms.extend([row['form_slp1'] for row in cursor.fetchall()])
                except sqlite3.OperationalError: pass
            
        all_forms = []
        for raw in raw_forms:
            all_forms.extend([f.strip() for f in raw.replace(';', ',').split(',') if f.strip()])
            
        all_forms = sorted(list(set(all_forms)))
        if not all_forms: return []
        if not prefixes: return all_forms
            
        final_forms = []
        for form in all_forms:
            current_form = form
            for p in reversed(prefixes):
                current_form = apply_forward_sandhi(p, current_form)
            final_forms.append(current_form)
            
        return sorted(list(set(final_forms)))

    def generate_subanta(self, pratipadika: str, linga: str) -> Dict[str, List[str]]:
        return SubantaGenerator().generate(pratipadika, linga)

    def generate_sarvanama(self, pratipadika: str, linga: str) -> Dict[str, List[str]]:
        return SarvanamaGenerator().generate(pratipadika, linga)
