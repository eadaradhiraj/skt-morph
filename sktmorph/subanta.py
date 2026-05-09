import re
from typing import Dict, List, Optional

def apply_natva(word_stem: str, suffix: str) -> str:
    """Applies the Natva rule, handling triggers in both stem and suffix."""
    if 'n' not in suffix: return suffix
    word = word_stem + suffix
    n_pos = suffix.find('n')
    full_n_pos = len(word_stem) + n_pos
    
    # Padānta rule: no natva at the end of a word
    if full_n_pos == len(word) - 1: return suffix
    
    trigger = False
    blockers = set('cCjJYSwWqQRtTdDnlS') # All consonants that block natva
    
    for i in range(full_n_pos):
        char = word[i]
        if char in['r', 'f', 'F', 'z']:
            trigger = True
        elif trigger and char in blockers:
            trigger = False
            
    if trigger:
        return suffix.replace('n', 'R', 1)
    return suffix

class SubantaGenerator:
    def __init__(self):
        self.vibhakti_names =["prathamA", "dvitIyA", "tfIyA", "caturTI", "paYcamI", "zazWI", "saptamI", "samboDana"]

    def _generate_table(self, base: str, endings: List[List[str]]) -> Dict[str, List[str]]:
        table = {}
        for i, vibhakti in enumerate(self.vibhakti_names):
            row = []
            for suffix_group in endings[i]:
                # Handles optional dual forms (e.g., "yE,aye" -> matyE/mataye)
                forms =[base + apply_natva(base, s) for s in suffix_group.split(',')]
                row.append("/".join(forms))
            table[vibhakti] = row
        return table

    def generate(self, pratipadika: str, linga: str) -> Optional[Dict[str, List[str]]]:
        if not pratipadika: return None
        end_char = pratipadika[-1]
        base = pratipadika[:-1]
        
        # --- a-anta ---
        if end_char == 'a' and linga == 'pum':
            return self._generate_table(base,[
                ['aH', 'O', 'AH'], ['am', 'O', 'An'],['ena', 'AByAm', 'EH'],['Aya', 'AByAm', 'eByaH'], ['At', 'AByAm', 'eByaH'],['asya', 'ayoH', 'AnAm'], ['e', 'ayoH', 'ezu'], ['a', 'O', 'AH']
            ])
        elif end_char == 'a' and linga == 'nap':
            return self._generate_table(base, [
                ['am', 'e', 'Ani'],['am', 'e', 'Ani'], ['ena', 'AByAm', 'EH'],['Aya', 'AByAm', 'eByaH'], ['At', 'AByAm', 'eByaH'], 
                ['asya', 'ayoH', 'AnAm'], ['e', 'ayoH', 'ezu'],['a', 'e', 'Ani']
            ])
            
        # --- A-anta ---
        elif end_char == 'A' and linga == 'stri':
            return self._generate_table(base, [
                ['A', 'e', 'AH'], ['Am', 'e', 'AH'],['ayA', 'AByAm', 'ABiH'],['AyE', 'AByAm', 'AByaH'], ['AyAH', 'AByAm', 'AByaH'], 
                ['AyAH', 'ayoH', 'AnAm'], ['AyAm', 'ayoH', 'Azu'], ['e', 'e', 'AH']
            ])
            
        # --- i-anta ---
        elif end_char == 'i' and linga == 'pum':
            return self._generate_table(base, [
                ['iH', 'I', 'ayaH'],['im', 'I', 'In'], ['inA', 'iByAm', 'iBiH'],['aye', 'iByAm', 'iByaH'], ['eH', 'iByAm', 'iByaH'], 
                ['eH', 'yoH', 'InAm'], ['O', 'yoH', 'izu'], ['e', 'I', 'ayaH']
            ])
        elif end_char == 'i' and linga == 'stri':
            return self._generate_table(base, [['iH', 'I', 'ayaH'], ['im', 'I', 'IH'],['yA', 'iByAm', 'iBiH'],['yE,aye', 'iByAm', 'iByaH'], ['yAH,eH', 'iByAm', 'iByaH'],['yAH,eH', 'yoH', 'InAm'],['yAm,O', 'yoH', 'izu'], ['e', 'I', 'ayaH']
            ])
        elif end_char == 'i' and linga == 'nap':
            return self._generate_table(base, [['i', 'inI', 'Ini'], ['i', 'inI', 'Ini'],['inA', 'iByAm', 'iBiH'],['ine', 'iByAm', 'iByaH'], ['inaH', 'iByAm', 'iByaH'],['inaH', 'inoH', 'InAm'], ['ini', 'inoH', 'izu'],['i,e', 'inI', 'Ini']
            ])
            
        # --- u-anta ---
        elif end_char == 'u' and linga == 'pum':
            return self._generate_table(base, [
                ['uH', 'U', 'avaH'], ['um', 'U', 'Un'],['unA', 'uByAm', 'uBiH'],['ave', 'uByAm', 'uByaH'], ['oH', 'uByAm', 'uByaH'], 
                ['oH', 'voH', 'UnAm'], ['O', 'voH', 'uzu'],['o', 'U', 'avaH']
            ])
        elif end_char == 'u' and linga == 'stri':
            return self._generate_table(base, [['uH', 'U', 'avaH'], ['um', 'U', 'UH'],['vA', 'uByAm', 'uBiH'], 
                ['vE,ave', 'uByAm', 'uByaH'], ['vAH,oH', 'uByAm', 'uByaH'],['vAH,oH', 'voH', 'UnAm'],['vAm,O', 'voH', 'uzu'], ['o', 'U', 'avaH']
            ])
        elif end_char == 'u' and linga == 'nap':
            return self._generate_table(base, [['u', 'unI', 'Uni'], ['u', 'unI', 'Uni'],['unA', 'uByAm', 'uBiH'], 
                ['une', 'uByAm', 'uByaH'], ['unaH', 'uByAm', 'uByaH'],['unaH', 'unoH', 'UnAm'], ['uni', 'unoH', 'uzu'],['u,o', 'unI', 'Uni']
            ])
            
        # --- f-anta (ṛ-anta) ---
        elif end_char == 'f' and linga == 'pum':
            return self._generate_table(base, [
                ['A', 'arO', 'araH'],['aram', 'arO', 'Fn'], ['rA', 'fByAm', 'fBiH'],['re', 'fByAm', 'fByaH'],['uH', 'fByAm', 'fByaH'], 
                ['uH', 'roH', 'FnAm'],['ari', 'roH', 'fzu'], ['aH', 'arO', 'araH']
            ])
        elif end_char == 'f' and linga == 'stri':
            return self._generate_table(base,[
                ['A', 'arO', 'araH'], ['aram', 'arO', 'FH'],['rA', 'fByAm', 'fBiH'],['re', 'fByAm', 'fByaH'], ['uH', 'fByAm', 'fByaH'],['uH', 'roH', 'FnAm'], ['ari', 'roH', 'fzu'],['aH', 'arO', 'araH']
            ])
        elif end_char == 'f' and linga == 'nap':
            return self._generate_table(base, [['f', 'fnI', 'Fni'], ['f', 'fnI', 'Fni'],['fnA', 'fByAm', 'fBiH'],['fne', 'fByAm', 'fByaH'], ['fnaH', 'fByAm', 'fByaH'],['fnaH', 'fnoH', 'FnAm'], ['fni', 'fnoH', 'fzu'], ['f,ar', 'fnI', 'Fni']
            ])
            
        raise NotImplementedError(f"Generation for {end_char}-anta {linga} is not yet implemented.")
