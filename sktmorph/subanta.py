import re
from typing import Any, Dict, List, Optional
from .prakriya import trace_declension_table

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
        if char in ['r', 'f', 'F', 'z']: trigger = True
        elif trigger and char in blockers: trigger = False
            
    if trigger: return suffix.replace('n', 'R', 1)
    return suffix

PARADIGMS = {
    # Ajantas
    ('a', 'pum'): [['aH', 'O', 'AH'],['am', 'O', 'An'],['ena', 'AByAm', 'EH'],['Aya', 'AByAm', 'eByaH'], ['At', 'AByAm', 'eByaH'],['asya', 'ayoH', 'AnAm'], ['e', 'ayoH', 'ezu'],['a', 'O', 'AH']],
    ('a', 'nap'): [['am', 'e', 'Ani'],['am', 'e', 'Ani'], ['ena', 'AByAm', 'EH'],['Aya', 'AByAm', 'eByaH'], ['At', 'AByAm', 'eByaH'],['asya', 'ayoH', 'AnAm'], ['e', 'ayoH', 'ezu'],['a', 'e', 'Ani']],
    ('A', 'stri'): [['A', 'e', 'AH'],['Am', 'e', 'AH'], ['ayA', 'AByAm', 'ABiH'],['AyE', 'AByAm', 'AByaH'],['AyAH', 'AByAm', 'AByaH'],['AyAH', 'ayoH', 'AnAm'],['AyAm', 'ayoH', 'Azu'],['e', 'e', 'AH']],
    ('i', 'pum'): [['iH', 'I', 'ayaH'], ['im', 'I', 'In'],['inA', 'iByAm', 'iBiH'], ['aye', 'iByAm', 'iByaH'],['eH', 'iByAm', 'iByaH'], ['eH', 'yoH', 'InAm'],['O', 'yoH', 'izu'], ['e', 'I', 'ayaH']],
    ('i', 'stri'): [['iH', 'I', 'ayaH'], ['im', 'I', 'IH'],['yA', 'iByAm', 'iBiH'],['yE,aye', 'iByAm', 'iByaH'],['yAH,eH', 'iByAm', 'iByaH'],['yAH,eH', 'yoH', 'InAm'],['yAm,O', 'yoH', 'izu'], ['e', 'I', 'ayaH']],
    ('i', 'nap'): [['i', 'inI', 'Ini'], ['i', 'inI', 'Ini'],['inA', 'iByAm', 'iBiH'], ['ine', 'iByAm', 'iByaH'],['inaH', 'iByAm', 'iByaH'],['inaH', 'inoH', 'InAm'],['ini', 'inoH', 'izu'], ['i,e', 'inI', 'Ini']],
    ('I', 'stri'): [['I', 'yO', 'yaH'], ['Im', 'yO', 'IH'], ['yA', 'IByAm', 'IBiH'], ['yE', 'IByAm', 'IByaH'], ['yAH', 'IByAm', 'IByaH'], ['yAH', 'yoH', 'InAm'], ['yAm', 'yoH', 'Izu'], ['i', 'yO', 'yaH']],
    ('I', 'pum'): [['I', 'yO', 'yaH'], ['Im', 'yO', 'In'], ['yA', 'IByAm', 'IBiH'], ['ye', 'IByAm', 'IByaH'], ['eH', 'IByAm', 'IByaH'], ['eH', 'yoH', 'InAm'], ['O', 'yoH', 'Izu'], ['i', 'yO', 'yaH']],
    ('u', 'pum'): [['uH', 'U', 'avaH'],['um', 'U', 'Un'], ['unA', 'uByAm', 'uBiH'],['ave', 'uByAm', 'uByaH'], ['oH', 'uByAm', 'uByaH'],['oH', 'voH', 'UnAm'], ['O', 'voH', 'uzu'],['o', 'U', 'avaH']],
    ('u', 'stri'): [['uH', 'U', 'avaH'],['um', 'U', 'UH'],['vA', 'uByAm', 'uBiH'],['vE,ave', 'uByAm', 'uByaH'],['vAH,oH', 'uByAm', 'uByaH'],['vAH,oH', 'voH', 'UnAm'], ['vAm,O', 'voH', 'uzu'],['o', 'U', 'avaH']],
    ('u', 'nap'): [['u', 'unI', 'Uni'],['u', 'unI', 'Uni'], ['unA', 'uByAm', 'uBiH'],['une', 'uByAm', 'uByaH'],['unaH', 'uByAm', 'uByaH'],['unaH', 'unoH', 'UnAm'], ['uni', 'unoH', 'uzu'],['u,o', 'unI', 'Uni']],
    ('U', 'pum'): [['UH', 'U', 'avaH'],['Um', 'U', 'Un'], ['UnA', 'uByAm', 'uBiH'],['Ave', 'uByAm', 'uByaH'], ['oH', 'uByAm', 'uByaH'],['oH', 'voH', 'UnAm'], ['O', 'voH', 'Uzu'],['o', 'U', 'avaH']],
    ('U', 'stri'): [['UH', 'U', 'avaH'],['Um', 'U', 'UH'],['vA', 'uByAm', 'uBiH'],['vE,ave', 'uByAm', 'uByaH'],['vAH,oH', 'uByAm', 'uByaH'],['vAH,oH', 'voH', 'UnAm'], ['vAm,O', 'voH', 'Uzu'],['o', 'U', 'avaH']],
    ('U', 'nap'): [['U', 'unI', 'Uni'],['U', 'unI', 'Uni'], ['UnA', 'uByAm', 'uBiH'],['Une', 'uByAm', 'uByaH'],['UnaH', 'uByAm', 'uByaH'],['UnaH', 'UnoH', 'UnAm'], ['Uni', 'UnoH', 'Uzu'],['U,o', 'unI', 'Uni']],
    ('f', 'pum'): [['A', 'arO', 'araH'], ['aram', 'arO', 'Fn'],['rA', 'fByAm', 'fBiH'], ['re', 'fByAm', 'fByaH'],['uH', 'fByAm', 'fByaH'],['uH', 'roH', 'FnAm'],['ari', 'roH', 'fzu'], ['aH', 'arO', 'araH']],
    ('f', 'stri'): [['A', 'arO', 'araH'],['aram', 'arO', 'FH'], ['rA', 'fByAm', 'fBiH'],['re', 'fByAm', 'fByaH'], ['uH', 'fByAm', 'fByaH'],['uH', 'roH', 'FnAm'], ['ari', 'roH', 'fzu'],['aH', 'arO', 'araH']],
    ('f', 'nap'): [['f', 'fnI', 'Fni'], ['f', 'fnI', 'Fni'],['fnA', 'fByAm', 'fBiH'], ['fne', 'fByAm', 'fByaH'],['fnaH', 'fByAm', 'fByaH'],['fnaH', 'fnoH', 'FnAm'], ['fni', 'fnoH', 'fzu'],['f,ar', 'fnI', 'Fni']],
    
    # Halantas
    ('in', 'pum'): [['I', 'inO', 'inaH'], ['inam', 'inO', 'inaH'],['inA', 'iByAm', 'iBiH'],['ine', 'iByAm', 'iByaH'], ['inaH', 'iByAm', 'iByaH'],['inaH', 'inoH', 'inAm'], ['ini', 'inoH', 'izu'],['in', 'inO', 'inaH']],
    ('as', 'nap'): [['aH', 'asI', 'AMsi'],['aH', 'asI', 'AMsi'], ['asA', 'oByAm', 'oBiH'],['ase', 'oByAm', 'oByaH'],['asaH', 'oByAm', 'oByaH'], ['asaH', 'asoH', 'asAm'],['asi', 'asoH', 'aHsu'], ['aH', 'asI', 'AMsi']],
    ('at', 'pum'): [['An', 'antO', 'antaH'], ['antam', 'antO', 'ataH'],['atA', 'adByAm', 'adBiH'],['ate', 'adByAm', 'adByaH'], ['ataH', 'adByAm', 'adByaH'],['ataH', 'atoH', 'atAm'], ['ati', 'atoH', 'atsu'],['an', 'antO', 'antaH']],
    # n-anta (rAjan-type)
    ('an', 'pum'): [['A', 'AnO', 'AnaH'], ['Anam', 'AnO', 'YaH'], ['YA', 'aByAm', 'aBiH'], ['Ye', 'aByAm', 'aByaH'], ['YaH', 'aByAm', 'aByaH'], ['YaH', 'YoH', 'YAm'], ['Yi,Yani', 'YoH', 'asu'], ['an', 'AnO', 'AnaH']],
    # c-anta (vAc-type, stri)
    ('c', 'stri'): [['g', 'caH', 'caH'], ['cam', 'ce', 'caH'], ['cA', 'gByAm', 'gBiH'], ['ce', 'gByAm', 'gByaH'], ['caH', 'gByAm', 'gByaH'], ['caH', 'coH', 'gAm'], ['ci', 'coH', 'su'], ['g', 'caH', 'caH']],
    # ad-anta (pad-type, nap)
    ('ad', 'nap'): [['ad', 'adI', 'AmSi'], ['adam', 'adI', 'AmSi'], ['adA', 'aByAm', 'aBiH'], ['ade', 'aByAm', 'aByaH'], ['adaH', 'aByAm', 'aByaH'], ['adaH', 'adoh', 'Am'], ['adi', 'adoh', 'atsu'], ['ad', 'adI', 'AmSi']],
    # z-anta (Siz-type, pum)
    ('z', 'pum'): [['H', 'qO', 'qaH'], ['am', 'qO', 'qaH'], ['qA', 'ByAm', 'BiH'], ['e', 'ByAm', 'ByaH'], ['aH', 'ByAm', 'ByaH'], ['aH', 'oH', 'Am'], ['i', 'oH', 'su'], ['H', 'qO', 'qaH']],
    # at-anta (jagat-type, nap; also covers other t-stems)
    ('at', 'nap'): [['at', 'atI', 'AMsi'], ['atam', 'atI', 'AMsi'], ['atA', 'ByAm', 'BiH'], ['ate', 'ByAm', 'ByaH'], ['ataH', 'ByAm', 'ByaH'], ['ataH', 'atoH', 'Am'], ['ati', 'atoH', 'atsu'], ['at', 'atI', 'AMsi']],
    ('us', 'nap'): [['uH', 'uSI', 'UMSi'],['uH', 'uSI', 'UMSi'], ['usA', 'oByAm', 'oBiH'],['use', 'oByAm', 'oByaH'],['usaH', 'oByAm', 'oByaH'], ['usaH', 'usoH', 'usAm'],['usi', 'usoH', 'uHsu'], ['uH', 'uSI', 'UMSi']],
    ('is', 'nap'): [['iH', 'iSI', 'IMSi'],['iH', 'iSI', 'IMSi'], ['isA', 'oByAm', 'oBiH'],['ise', 'oByAm', 'oByaH'],['isaH', 'oByAm', 'oByaH'], ['isaH', 'isoH', 'isAm'],['isi', 'isoH', 'iHsu'], ['iH', 'iSI', 'IMSi']],
}

class SubantaGenerator:
    def __init__(self):
        self.vibhakti_names = ["prathamA", "dvitIyA", "tfIyA", "caturTI", "paYcamI", "zazWI", "saptamI", "samboDana"]
        self.supported_endings = sorted(list(set([k[0] for k in PARADIGMS.keys()])), key=len, reverse=True)

    def _normalize_pratipadika(self, pratipadika: str, linga: str) -> str:
        if linga == "pum" and pratipadika.endswith("A") and len(pratipadika) > 1:
            f_stem = pratipadika[:-1] + "f"
            for ending in self.supported_endings:
                if f_stem.endswith(ending) and PARADIGMS.get((ending, linga)):
                    return f_stem
        return pratipadika

    def _generate_table(self, base: str, endings: List[List[str]]) -> Dict[str, List[str]]:
        table = {}
        for i, vibhakti in enumerate(self.vibhakti_names):
            row = []
            for suffix_group in endings[i]:
                forms = [base + apply_natva(base, s) for s in suffix_group.split(',')]
                row.append("/".join(forms))
            table[vibhakti] = row
        return table

    def generate(self, pratipadika: str, linga: str) -> Optional[Dict[str, List[str]]]:
        detail = self.generate_detail(pratipadika, linga)
        return detail["declension"] if detail else None

    def generate_detail(self, pratipadika: str, linga: str) -> Optional[Dict[str, Any]]:
        if not pratipadika:
            return None
        pratipadika = self._normalize_pratipadika(pratipadika, linga)
        for ending in self.supported_endings:
            if pratipadika.endswith(ending):
                base = pratipadika[:-len(ending)]
                endings = PARADIGMS.get((ending, linga))
                if endings:
                    table = self._generate_table(base, endings)
                    return {
                        "stem": pratipadika,
                        "linga": linga,
                        "base": base,
                        "ending": ending,
                        "declension": table,
                        "endings_table": endings,
                    }
        raise NotImplementedError(f"Generation for '{pratipadika}' ({linga}) is not yet implemented.")

    def generate_with_prakriya(self, pratipadika: str, linga: str) -> Dict[str, Any]:
        detail = self.generate_detail(pratipadika, linga)
        detail["prakriya"] = trace_declension_table(
            detail["base"], detail["ending"], detail["endings_table"], detail["declension"]
        )
        return detail

    def analyze(self, word: str) -> List[Dict[str, str]]:
        results = []
        for (ending, linga), endings_table in PARADIGMS.items():
            for vibh_idx, vibhakti in enumerate(self.vibhakti_names):
                for vacana_idx, suffix_group in enumerate(endings_table[vibh_idx]):
                    for original_suffix in suffix_group.split(','):
                        base_stripped_len = len(word) - len(original_suffix)
                        if base_stripped_len > 0:
                            base_stripped = word[:base_stripped_len]
                            surface_suffix = apply_natva(base_stripped, original_suffix)
                            if word == base_stripped + surface_suffix:
                                results.append({
                                    'pratipadika': base_stripped + ending,
                                    'linga': linga,
                                    'vibhakti': vibhakti,
                                    'vacana': vacana_idx + 1
                                })
        return results
