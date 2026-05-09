import sqlite3
import os
import json
import re
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

UPASARGA_SPLIT_RULES: List[Tuple[str, str, str]] =[
    ('prA', 'pra', 'a'), ('prA', 'pra', 'A'), ('pre', 'pra', 'i'), ('pro', 'pra', 'u'), 
    ('prAr', 'pra', 'f'), ('pra', 'pra', ''),
    ('vyA', 'vi', 'A'), ('vya', 'vi', 'a'), ('vyu', 'vi', 'u'), ('vI', 'vi', 'i'), ('vI', 'vi', 'I'), 
    ('vi', 'vi', ''), ('vy', 'vi', 'a'),
    ('saM', 'sam', ''), ('saY', 'sam', ''), ('saN', 'sam', ''), ('saR', 'sam', ''), 
    ('san', 'sam', ''), ('sam', 'sam', ''),
    ('anvA', 'anu', 'A'), ('anva', 'anu', 'a'), ('anvI', 'anu', 'I'), ('anU', 'anu', 'u'), 
    ('anu', 'anu', ''), ('anv', 'anu', 'a'),
    ('uddh', 'ud', 'h'), ('ut', 'ud', ''), ('uc', 'ud', 'c'), ('uj', 'ud', 'j'), 
    ('ul', 'ud', 'l'), ('ud', 'ud', ''),
    ('nyA', 'ni', 'A'), ('nya', 'ni', 'a'), ('nI', 'ni', 'i'), ('nI', 'ni', 'I'), 
    ('ni', 'ni', ''), ('ny', 'ni', 'a'),
    ('upA', 'upa', 'a'), ('upA', 'upa', 'A'), ('upe', 'upa', 'i'), ('upo', 'upa', 'u'), 
    ('upa', 'upa', ''),
    ('avA', 'ava', 'a'), ('avA', 'ava', 'A'), ('ava', 'ava', ''),
    ('aByA', 'aBi', 'A'), ('aBya', 'aBi', 'a'), ('aBI', 'aBi', 'i'), ('aBi', 'aBi', ''),
    ('pratyA', 'prati', 'A'), ('pratya', 'prati', 'a'), ('pratI', 'prati', 'i'), ('prati', 'prati', ''),
    ('paryA', 'pari', 'A'), ('parya', 'pari', 'a'), ('parI', 'pari', 'i'), ('pari', 'pari', ''),
    ('atyA', 'ati', 'A'), ('atya', 'ati', 'a'), ('atI', 'ati', 'i'), ('ati', 'ati', ''),
    ('aDyA', 'aDi', 'A'), ('aDya', 'aDi', 'a'), ('aDI', 'aDi', 'i'), ('aDi', 'aDi', ''),
    ('apyA', 'api', 'A'), ('apya', 'api', 'a'), ('apI', 'api', 'i'), ('api', 'api', ''),
    ('svA', 'su', 'A'), ('sva', 'su', 'a'), ('sU', 'su', 'u'), ('su', 'su', ''),
    ('apA', 'apa', 'a'), ('apA', 'apa', 'A'), ('apa', 'apa', ''),
    ('parA', 'parA', 'a'), ('parA', 'parA', 'A'), ('parA', 'parA', ''),
    ('A', 'A', ''),
    ('nir', 'nis', ''), ('niz', 'nis', ''), ('niS', 'nis', ''), ('nih', 'nis', ''), ('nis', 'nis', ''),
    ('dur', 'dus', ''), ('duz', 'dus', ''), ('duS', 'dus', ''), ('duh', 'dus', ''), ('dus', 'dus', '')
]

def apply_forward_sandhi(prefix: str, word: str) -> str:
    if not prefix: return word
    p_end = prefix[-1]
    w_start = word[0]
    w_rest = word[1:]
    vowels = {'a', 'A', 'i', 'I', 'u', 'U', 'f', 'F', 'e', 'E', 'o', 'O'}
    
    if p_end in['a', 'A']:
        if w_start in ['a', 'A']: return prefix[:-1] + 'A' + w_rest
        if w_start in['i', 'I']: return prefix[:-1] + 'e' + w_rest
        if w_start in['u', 'U']: return prefix[:-1] + 'o' + w_rest
        if w_start in['f', 'F']: return prefix[:-1] + 'Ar' + w_rest
        if w_start == 'e': return prefix[:-1] + 'e' + w_rest
    elif p_end in['i', 'I']:
        if w_start in vowels and w_start not in['i', 'I']: return prefix[:-1] + 'y' + word
        if w_start in['i', 'I']: return prefix[:-1] + 'I' + w_rest
    elif p_end in['u', 'U']:
        if w_start in vowels and w_start not in['u', 'U']: return prefix[:-1] + 'v' + word
        if w_start in['u', 'U']: return prefix[:-1] + 'U' + w_rest

    if prefix == 'sam': return 'saM' + word
    if prefix == 'ud':
        if w_start in['k', 'K', 'c', 'C', 'w', 'W', 't', 'T', 'p', 'P', 's', 'S', 'z']: return 'ut' + word
        if w_start == 'h': return 'uddh' + w_rest
    return prefix + word

class SktMorph:
    def __init__(self, db_path: str = None):
        if db_path is None:
            db_path = os.path.join(os.path.dirname(__file__), 'data', 'sanskrit_forms.sqlite')
        if not os.path.exists(db_path):
            raise FileNotFoundError(f"Database not found at {db_path}. Please build the DB first.")
        self.conn = sqlite3.connect(db_path)
        self.conn.row_factory = sqlite3.Row

    def get_candidate_splits(self, word: str) -> List[Tuple[List[str], str]]:
        candidates =[([], word)]
        queue = [([], word)]
        visited = set()
        
        while queue:
            current_prefixes, current_word = queue.pop(0)
            for surface, actual, prepend in UPASARGA_SPLIT_RULES:
                if current_word.startswith(surface):
                    remainder = prepend + current_word[len(surface):]
                    if len(remainder) > 0:
                        new_prefixes = current_prefixes + [actual]
                        state = (tuple(new_prefixes), remainder)
                        if state not in visited:
                            visited.add(state)
                            candidates.append((list(new_prefixes), remainder))
                            queue.append((list(new_prefixes), remainder))
        return candidates

    def analyze(self, word_slp1: str) -> List[MorphResult]:
        candidates = self.get_candidate_splits(word_slp1)
        results =[]
        cursor = self.conn.cursor()

        for prefixes, base_word in candidates:
            cursor.execute("""
                SELECT t.*, d.details_json 
                FROM tinantas t 
                LEFT JOIN dhatus d ON t.dhatu_id = d.dhatu_id 
                WHERE t.form_slp1 = ?
            """, (base_word,))
            for row in cursor.fetchall():
                details = json.loads(row['details_json']) if row['details_json'] else None
                results.append(MorphResult(
                    word=word_slp1, prefixes=prefixes, dhatu=row['dhatu_id'],
                    word_type='tinanta', derivation=row['derivation'],
                    prayoga=row['prayoga'], lakara=row['lakara'],
                    purusha=row['purusha'], vacana=row['vacana'], dhatu_details=details
                ))
            
            cursor.execute("""
                SELECT k.*, d.details_json 
                FROM krdantas k 
                LEFT JOIN dhatus d ON k.dhatu_id = d.dhatu_id 
                WHERE k.form_slp1 = ?
            """, (base_word,))
            for row in cursor.fetchall():
                details = json.loads(row['details_json']) if row['details_json'] else None
                results.append(MorphResult(
                    word=word_slp1, prefixes=prefixes, dhatu=row['dhatu_id'],
                    word_type='krdanta', derivation=row['derivation'],
                    pratyaya=row['pratyaya'], dhatu_details=details
                ))

        sub_gen = SubantaGenerator()
        for match in sub_gen.analyze(word_slp1):
            results.append(MorphResult(
                word=word_slp1, prefixes=[], dhatu=None, word_type='subanta', derivation=None,
                pratipadika=match['pratipadika'], linga=match['linga'],
                vibhakti=match['vibhakti'], vacana=match['vacana']
            ))

        sarv_gen = SarvanamaGenerator()
        for match in sarv_gen.analyze(word_slp1):
            results.append(MorphResult(
                word=word_slp1, prefixes=[], dhatu=None, word_type="sarvanama", derivation=None,
                pratipadika=match["pratipadika"], linga=match["linga"],
                vibhakti=match["vibhakti"], vacana=match["vacana"]
            ))

        return results

    def resolve_dhatu_ids(self, dhatu_query: str) -> List[str]:
        """Resolves a numerical ID (e.g., 01.0001) or SLP1 Root Name (e.g., BU) to a list of IDs."""
        if re.match(r'^\d{2}\.\d{4}$', dhatu_query):
            return[dhatu_query]
            
        try:
            from indic_transliteration import sanscript
            from indic_transliteration.sanscript import transliterate
            dev_query = transliterate(dhatu_query, sanscript.SLP1, sanscript.DEVANAGARI)
        except ImportError:
            dev_query = dhatu_query

        cursor = self.conn.cursor()
        cursor.execute("SELECT dhatu_id FROM dhatus WHERE details_json LIKE ? OR details_json LIKE ?", 
                       (f'%"{dev_query}"%', f'%"{dhatu_query}"%'))
        return [row['dhatu_id'] for row in cursor.fetchall()]

    def generate_tinanta(self, dhatu: str, lakara: str, purusha: int, vacana: int, 
                         derivation: str = 'shuddha', prayoga: str = 'kartari', 
                         prefixes: List[str] = None) -> List[str]:
        dhatu_ids = self.resolve_dhatu_ids(dhatu)
        cursor = self.conn.cursor()
        
        all_forms =[]
        for d_id in dhatu_ids:
            cursor.execute('''SELECT form_slp1 FROM tinantas 
                              WHERE dhatu_id = ? AND lakara = ? AND purusha = ? 
                              AND vacana = ? AND derivation = ? AND prayoga = ?''',
                           (d_id, lakara, purusha, vacana, derivation, prayoga))
            all_forms.extend([row['form_slp1'] for row in cursor.fetchall()])
            
        if not all_forms: return[]
        if not prefixes: return sorted(list(set(all_forms)))
            
        final_forms =[]
        for form in all_forms:
            current_form = form
            for p in reversed(prefixes):
                current_form = apply_forward_sandhi(p, current_form)
            final_forms.extend([f.strip() for f in current_form.replace(";", ",").split(",")])
        return sorted(list(set(final_forms)))

    def generate_krdanta(self, dhatu: str, pratyaya: str, derivation: str = 'shuddha', 
                         prefixes: List[str] = None) -> List[str]:
        dhatu_ids = self.resolve_dhatu_ids(dhatu)
        cursor = self.conn.cursor()
        
        all_forms =[]
        for d_id in dhatu_ids:
            cursor.execute('''SELECT form_slp1 FROM krdantas 
                              WHERE dhatu_id = ? AND pratyaya = ? AND derivation = ?''',
                           (d_id, pratyaya, derivation))
            all_forms.extend([row['form_slp1'] for row in cursor.fetchall()])
            
        if not all_forms: return[]
        if not prefixes: return sorted(list(set(all_forms)))
            
        final_forms =[]
        for form in all_forms:
            current_form = form
            for p in reversed(prefixes):
                current_form = apply_forward_sandhi(p, current_form)
            final_forms.extend([f.strip() for f in current_form.replace(";", ",").split(",")])
        return sorted(list(set(final_forms)))

    def generate_subanta(self, pratipadika: str, linga: str) -> Dict[str, List[str]]:
        sub_gen = SubantaGenerator()
        return sub_gen.generate(pratipadika, linga)

    def generate_sarvanama(self, pratipadika: str, linga: str) -> Dict[str, List[str]]:
        sarv_gen = SarvanamaGenerator()
        return sarv_gen.generate(pratipadika, linga)
