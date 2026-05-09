# sktmorph/morphology.py
import sqlite3
import os
from dataclasses import dataclass
from typing import List, Tuple, Dict, Any, Optional

@dataclass
class MorphResult:
    """Dataclass to hold the analyzed morphological data."""
    word: str
    prefixes: List[str]
    dhatu: str
    word_type: str  # 'tinanta' (verb) or 'krdanta' (derivative)
    derivation: str # 'shuddha', 'nich', 'san', etc.
    
    # Specific to Tinanta (Conjugations)
    prayoga: Optional[str] = None
    lakara: Optional[str] = None
    purusha: Optional[int] = None
    vacana: Optional[int] = None
    
    # Specific to Krdanta
    pratyaya: Optional[str] = None

# A robust heuristic list of Upasarga surface forms to reverse-engineer prefixes.
# Format: (Surface Form, Actual Upasarga, Vowel/Consonant to prepend to the rest of the word)
UPASARGA_SPLIT_RULES: List[Tuple[str, str, str]] =[
    # PRA
    ('prA', 'pra', 'a'), ('prA', 'pra', 'A'), ('pre', 'pra', 'i'), ('pro', 'pra', 'u'), 
    ('prAr', 'pra', 'f'), ('pra', 'pra', ''),
    # VI
    ('vyA', 'vi', 'A'), ('vya', 'vi', 'a'), ('vyu', 'vi', 'u'), ('vI', 'vi', 'i'), ('vI', 'vi', 'I'), 
    ('vi', 'vi', ''), ('vy', 'vi', 'a'),
    # SAM
    ('saM', 'sam', ''), ('saY', 'sam', ''), ('saN', 'sam', ''), ('saR', 'sam', ''), 
    ('san', 'sam', ''), ('sam', 'sam', ''),
    # ANU
    ('anvA', 'anu', 'A'), ('anva', 'anu', 'a'), ('anvI', 'anu', 'I'), ('anU', 'anu', 'u'), 
    ('anu', 'anu', ''), ('anv', 'anu', 'a'),
    # UD
    ('uddh', 'ud', 'h'), ('ut', 'ud', ''), ('uc', 'ud', 'c'), ('uj', 'ud', 'j'), 
    ('ul', 'ud', 'l'), ('ud', 'ud', ''),
    # NI
    ('nyA', 'ni', 'A'), ('nya', 'ni', 'a'), ('nI', 'ni', 'i'), ('nI', 'ni', 'I'), 
    ('ni', 'ni', ''), ('ny', 'ni', 'a'),
    # UPA
    ('upA', 'upa', 'a'), ('upA', 'upa', 'A'), ('upe', 'upa', 'i'), ('upo', 'upa', 'u'), 
    ('upa', 'upa', ''),
    # AVA
    ('avA', 'ava', 'a'), ('avA', 'ava', 'A'), ('ava', 'ava', ''),
    # ABI
    ('aByA', 'aBi', 'A'), ('aBya', 'aBi', 'a'), ('aBI', 'aBi', 'i'), ('aBi', 'aBi', ''),
    # PRATI
    ('pratyA', 'prati', 'A'), ('pratya', 'prati', 'a'), ('pratI', 'prati', 'i'), ('prati', 'prati', ''),
    # PARI
    ('paryA', 'pari', 'A'), ('parya', 'pari', 'a'), ('parI', 'pari', 'i'), ('pari', 'pari', ''),
    # ATI
    ('atyA', 'ati', 'A'), ('atya', 'ati', 'a'), ('atI', 'ati', 'i'), ('ati', 'ati', ''),
    # ADI
    ('aDyA', 'aDi', 'A'), ('aDya', 'aDi', 'a'), ('aDI', 'aDi', 'i'), ('aDi', 'aDi', ''),
    # API
    ('apyA', 'api', 'A'), ('apya', 'api', 'a'), ('apI', 'api', 'i'), ('api', 'api', ''),
    # SU
    ('svA', 'su', 'A'), ('sva', 'su', 'a'), ('sU', 'su', 'u'), ('su', 'su', ''),
    # APA / PARA / A
    ('apA', 'apa', 'a'), ('apA', 'apa', 'A'), ('apa', 'apa', ''),
    ('parA', 'parA', 'a'), ('parA', 'parA', 'A'), ('parA', 'parA', ''),
    ('A', 'A', ''),
    # NIS / DUS
    ('nir', 'nis', ''), ('niz', 'nis', ''), ('niS', 'nis', ''), ('nih', 'nis', ''), ('nis', 'nis', ''),
    ('dur', 'dus', ''), ('duz', 'dus', ''), ('duS', 'dus', ''), ('duh', 'dus', ''), ('dus', 'dus', '')
]

def apply_forward_sandhi(prefix: str, word: str) -> str:
    """Applies basic Forward Sandhi to join an Upasarga with a verb form (SLP1)."""
    if not prefix:
        return word
    
    p_end = prefix[-1]
    w_start = word[0]
    w_rest = word[1:]
    vowels = {'a', 'A', 'i', 'I', 'u', 'U', 'f', 'F', 'e', 'E', 'o', 'O'}
    
    # 1. Vowel-ending prefixes (a, A, i, I, u, U)
    if p_end in ['a', 'A']:
        if w_start in ['a', 'A']: return prefix[:-1] + 'A' + w_rest
        if w_start in ['i', 'I']: return prefix[:-1] + 'e' + w_rest
        if w_start in ['u', 'U']: return prefix[:-1] + 'o' + w_rest
        if w_start in ['f', 'F']: return prefix[:-1] + 'Ar' + w_rest # Vrddhi
        if w_start == 'e': return prefix[:-1] + 'e' + w_rest # Pararupa (e.g. pra + ejate)
        
    elif p_end in ['i', 'I']:
        if w_start in vowels and w_start not in ['i', 'I']:
            return prefix[:-1] + 'y' + word
        if w_start in ['i', 'I']: return prefix[:-1] + 'I' + w_rest
            
    elif p_end in ['u', 'U']:
        if w_start in vowels and w_start not in ['u', 'U']:
            return prefix[:-1] + 'v' + word
        if w_start in ['u', 'U']: return prefix[:-1] + 'U' + w_rest

    # 2. Consonant-ending prefixes
    if prefix == 'sam':
        return 'saM' + word # Simplified Anusvara
    if prefix == 'ud':
        if w_start in['k', 'K', 'c', 'C', 'w', 'W', 't', 'T', 'p', 'P', 's', 'S', 'z']:
            return 'ut' + word
        if w_start == 'h':
            return 'uddh' + w_rest
            
    return prefix + word

from .subanta import SubantaGenerator

class SktMorph:
    """Main Class handling Generation and Analysis."""
    
    def __init__(self, db_path: str = None):
        if db_path is None:
            db_path = os.path.join(os.path.dirname(__file__), 'data', 'sanskrit_forms.sqlite')
        
        if not os.path.exists(db_path):
            raise FileNotFoundError(f"Database not found at {db_path}. Please build the DB first.")
            
        self.conn = sqlite3.connect(db_path)
        self.conn.row_factory = sqlite3.Row

    def get_candidate_splits(self, word: str) -> List[Tuple[List[str], str]]:
        """Recursively strip Upasargas to generate all possible combinations."""
        candidates = [([], word)]
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
        """Analyzes an SLP1 word and returns all valid grammatical interpretations."""
        candidates = self.get_candidate_splits(word_slp1)
        results =[]
        cursor = self.conn.cursor()

        for prefixes, base_word in candidates:
            # Check Tinantas
            cursor.execute("SELECT * FROM tinantas WHERE form_slp1 = ?", (base_word,))
            for row in cursor.fetchall():
                results.append(MorphResult(
                    word=word_slp1,
                    prefixes=prefixes,
                    dhatu=row['dhatu_id'],
                    word_type='tinanta',
                    derivation=row['derivation'],
                    prayoga=row['prayoga'],
                    lakara=row['lakara'],
                    purusha=row['purusha'],
                    vacana=row['vacana']
                ))
            
            # Check Krdantas
            cursor.execute("SELECT * FROM krdantas WHERE form_slp1 = ?", (base_word,))
            for row in cursor.fetchall():
                results.append(MorphResult(
                    word=word_slp1,
                    prefixes=prefixes,
                    dhatu=row['dhatu_id'],
                    word_type='krdanta',
                    derivation=row['derivation'],
                    pratyaya=row['pratyaya']
                ))

        return results

    def generate_tinanta(self, dhatu: str, lakara: str, purusha: int, vacana: int, 
                         derivation: str = 'shuddha', prayoga: str = 'kartari', 
                         prefixes: List[str] = None) -> List[str]:
        """Generates SLP1 Tinanta forms. Applies multiple prefixes sequentially."""
        cursor = self.conn.cursor()
        cursor.execute('''SELECT form_slp1 FROM tinantas 
                          WHERE dhatu_id = ? AND lakara = ? AND purusha = ? 
                          AND vacana = ? AND derivation = ? AND prayoga = ?''',
                       (dhatu, lakara, purusha, vacana, derivation, prayoga))
        
        forms =[row['form_slp1'] for row in cursor.fetchall()]
        if not forms:
            return[]

        if not prefixes:
            return forms
            
        final_forms =[]
        for form in forms:
            current_form = form
            # Apply prefixes in reverse order (closest to root first)
            for p in reversed(prefixes):
                current_form = apply_forward_sandhi(p, current_form)
            final_forms.append(current_form)
            
        return final_forms
    def generate_subanta(self, pratipadika: str, linga: str) -> Dict[str, List[str]]:
        """Generates noun declensions using algorithmic rules."""
        sub_gen = SubantaGenerator()
        return sub_gen.generate(pratipadika, linga)
