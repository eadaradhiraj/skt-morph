import re
from typing import Dict, List, Optional

def apply_natva(word_stem: str, suffix: str) -> str:
    if 'n' not in suffix: return suffix
    word = word_stem + suffix
    n_pos = suffix.find('n')
    full_n_pos = len(word_stem) + n_pos
    
    if full_n_pos == len(word) - 1: return suffix
    
    trigger = False
    blockers = set('cCjJYSwWqQRtTdDnlS') 
    
    for i in range(full_n_pos):
        char = word[i]
        if char in['r', 'f', 'F', 'z']: trigger = True
        elif trigger and char in blockers: trigger = False
            
    if trigger: return suffix.replace('n', 'R', 1)
    return suffix

PARADIGMS = {
    ('a', 'pum'): [['aH', 'O', 'AH'],['am', 'O', 'An'], ['ena', 'AByAm', 'EH'],['Aya', 'AByAm', 'eByaH'], ['At', 'AByAm', 'eByaH'],['asya', 'ayoH', 'AnAm'], ['e', 'ayoH', 'ezu'],['a', 'O', 'AH']],
    ('a', 'nap'): [['am', 'e', 'Ani'],['am', 'e', 'Ani'], ['ena', 'AByAm', 'EH'],['Aya', 'AByAm', 'eByaH'], ['At', 'AByAm', 'eByaH'],['asya', 'ayoH', 'AnAm'], ['e', 'ayoH', 'ezu'],['a', 'e', 'Ani']],
    ('A', 'stri'): [['A', 'e', 'AH'],['Am', 'e', 'AH'], ['ayA', 'AByAm', 'ABiH'],['AyE', 'AByAm', 'AByaH'],['AyAH', 'AByAm', 'AByaH'], ['AyAH', 'ayoH', 'AnAm'],['AyAm', 'ayoH', 'Azu'], ['e', 'e', 'AH']],
    ('i', 'pum'): [['iH', 'I', 'ayaH'], ['im', 'I', 'In'],['inA', 'iByAm', 'iBiH'], ['aye', 'iByAm', 'iByaH'],['eH', 'iByAm', 'iByaH'], ['eH', 'yoH', 'InAm'], ['O', 'yoH', 'izu'], ['e', 'I', 'ayaH']],
    ('i', 'stri'): [['iH', 'I', 'ayaH'], ['im', 'I', 'IH'], ['yA', 'iByAm', 'iBiH'],['yE,aye', 'iByAm', 'iByaH'],['yAH,eH', 'iByAm', 'iByaH'], ['yAH,eH', 'yoH', 'InAm'],['yAm,O', 'yoH', 'izu'], ['e', 'I', 'ayaH']],
    ('i', 'nap'): [['i', 'inI', 'Ini'], ['i', 'inI', 'Ini'],['inA', 'iByAm', 'iBiH'], ['ine', 'iByAm', 'iByaH'],['inaH', 'iByAm', 'iByaH'],['inaH', 'inoH', 'InAm'], ['ini', 'inoH', 'izu'], ['i,e', 'inI', 'Ini']],
    ('u', 'pum'): [['uH', 'U', 'avaH'],['um', 'U', 'Un'], ['unA', 'uByAm', 'uBiH'],['ave', 'uByAm', 'uByaH'], ['oH', 'uByAm', 'uByaH'], ['oH', 'voH', 'UnAm'], ['O', 'voH', 'uzu'],['o', 'U', 'avaH']],
    ('u', 'stri'): [['uH', 'U', 'avaH'], ['um', 'U', 'UH'],['vA', 'uByAm', 'uBiH'], ['vE,ave', 'uByAm', 'uByaH'],['vAH,oH', 'uByAm', 'uByaH'],['vAH,oH', 'voH', 'UnAm'], ['vAm,O', 'voH', 'uzu'],['o', 'U', 'avaH']],
    ('u', 'nap'): [['u', 'unI', 'Uni'],['u', 'unI', 'Uni'], ['unA', 'uByAm', 'uBiH'],['une', 'uByAm', 'uByaH'],['unaH', 'uByAm', 'uByaH'], ['unaH', 'unoH', 'UnAm'], ['uni', 'unoH', 'uzu'], ['u,o', 'unI', 'Uni']],
    ('f', 'pum'): [['A', 'arO', 'araH'], ['aram', 'arO', 'Fn'],['rA', 'fByAm', 'fBiH'], ['re', 'fByAm', 'fByaH'],['uH', 'fByAm', 'fByaH'],['uH', 'roH', 'FnAm'], ['ari', 'roH', 'fzu'], ['aH', 'arO', 'araH']],
    ('f', 'stri'): [['A', 'arO', 'araH'],['aram', 'arO', 'FH'], ['rA', 'fByAm', 'fBiH'],['re', 'fByAm', 'fByaH'], ['uH', 'fByAm', 'fByaH'], ['uH', 'roH', 'FnAm'], ['ari', 'roH', 'fzu'],['aH', 'arO', 'araH']],
    ('f', 'nap'): [['f', 'fnI', 'Fni'], ['f', 'fnI', 'Fni'],['fnA', 'fByAm', 'fBiH'], ['fne', 'fByAm', 'fByaH'],['fnaH', 'fByAm', 'fByaH'],['fnaH', 'fnoH', 'FnAm'], ['fni', 'fnoH', 'fzu'],['f,ar', 'fnI', 'Fni']]
}

class SubantaGenerator:
    def __init__(self):
        self.vibhakti_names =["prathamA", "dvitIyA", "tfIyA", "caturTI", "paYcamI", "zazWI", "saptamI", "samboDana"]

    def _generate_table(self, base: str, endings: List[List[str]]) -> Dict[str, List[str]]:
        table = {}
        for i, vibhakti in enumerate(self.vibhakti_names):
            row = []
            for suffix_group in endings[i]:
                forms =[base + apply_natva(base, s) for s in suffix_group.split(',')]
                row.append("/".join(forms))
            table[vibhakti] = row
        return table

    def generate(self, pratipadika: str, linga: str) -> Optional[Dict[str, List[str]]]:
        if not pratipadika: return None
        end_char = pratipadika[-1]
        base = pratipadika[:-1]
        
        endings = PARADIGMS.get((end_char, linga))
        if endings:
            return self._generate_table(base, endings)
        raise NotImplementedError(f"Generation for {end_char}-anta {linga} is not yet implemented.")

    def analyze(self, word: str) -> List[Dict[str, str]]:
        """Algorithmically reverses Subanta forms back to their base properties."""
        results =[]
        for (vowel, linga), endings in PARADIGMS.items():
            for vibh_idx, vibhakti in enumerate(self.vibhakti_names):
                for vacana_idx, suffix_group in enumerate(endings[vibh_idx]):
                    for original_suffix in suffix_group.split(','):
                        base_stripped_len = len(word) - len(original_suffix)
                        # A valid stem must have at least 1 character
                        if base_stripped_len > 0:
                            base_stripped = word[:base_stripped_len]
                            surface_suffix = apply_natva(base_stripped, original_suffix)
                            # If the hypothetical form matches our word, we have a valid analysis!
                            if word == base_stripped + surface_suffix:
                                results.append({
                                    'pratipadika': base_stripped + vowel,
                                    'linga': linga,
                                    'vibhakti': vibhakti,
                                    'vacana': vacana_idx + 1
                                })
        return results
