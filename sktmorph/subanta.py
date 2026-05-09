import re
from typing import Dict, List, Optional

def apply_natva(word_stem: str, suffix: str) -> str:
    if not ('n' in suffix): return suffix
    trigger = False
    blockers = set('cCjJYSwWqQRtTdDnslS') 
    for char in word_stem:
        if char in['r', 'f', 'F', 'z']: trigger = True
        elif trigger and char in blockers: trigger = False
    if trigger: return suffix.replace('n', 'R', 1)
    return suffix

class SubantaGenerator:
    def __init__(self):
        self.vibhakti_names =["prathamA", "dvitIyA", "tfIyA", "caturTI", "paYcamI", "zazWI", "saptamI", "samboDana"]

    def _generate_table(self, base: str, endings: List[List[str]]) -> Dict[str, List[str]]:
        table = {}
        for i, vibhakti in enumerate(self.vibhakti_names):
            table[vibhakti] = [base + apply_natva(base, suffix) for suffix in endings[i]]
        return table

    def generate(self, pratipadika: str, linga: str) -> Optional[Dict[str, List[str]]]:
        if not pratipadika: return None
        end_char = pratipadika[-1]
        
        if end_char == 'a' and linga == 'pum':
            return self._generate_table(pratipadika[:-1], [['aH', 'O', 'AH'], ['am', 'O', 'An'],['ena', 'AByAm', 'EH'], ['Aya', 'AByAm', 'eByaH'],['At', 'AByAm', 'eByaH'], ['asya', 'ayoH', 'AnAm'],['e', 'ayoH', 'ezu'], ['a', 'O', 'AH']
            ])
        elif end_char == 'a' and linga == 'nap':
            return self._generate_table(pratipadika[:-1], [
                ['am', 'e', 'Ani'],['am', 'e', 'Ani'], ['ena', 'AByAm', 'EH'],['Aya', 'AByAm', 'eByaH'],
                ['At', 'AByAm', 'eByaH'],['asya', 'ayoH', 'AnAm'], ['e', 'ayoH', 'ezu'],['a', 'e', 'Ani']
            ])
        elif end_char == 'A' and linga == 'stri':
            return self._generate_table(pratipadika[:-1], [['A', 'e', 'AH'], ['Am', 'e', 'AH'], ['ayA', 'AByAm', 'ABiH'], ['AyE', 'AByAm', 'AByaH'],['AyAH', 'AByAm', 'AByaH'], ['AyAH', 'ayoH', 'AnAm'],['AyAm', 'ayoH', 'Azu'], ['e', 'e', 'AH']
            ])
        raise NotImplementedError(f"Generation for {end_char}-anta {linga} is not yet implemented.")
