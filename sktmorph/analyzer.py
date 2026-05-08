import sqlite3
from pathlib import Path

PREFIX_SPLITS = [
    ('saM', 'sam', ''), ('sam', 'sam', ''), ('prA', 'pra', 'a'), 
    ('pre', 'pra', 'i'), ('pra', 'pra', ''), ('A', 'A', ''),
    ('vya', 'vi', 'a'), ('vi', 'vi', '')
]

SUBANTA_ENDINGS = {'i': 'Saptami Eka (Loc. Sg.)'}

class SanskritAnalyzer:
    def __init__(self):
        self.db_path = Path(__file__).parent / "data" / "sanskrit_forms.sqlite"
        self.conn = sqlite3.connect(self.db_path)
        self.conn.row_factory = sqlite3.Row

    def _base_lookup(self, exact_word):
        tinantas = [dict(row) for row in self.conn.execute(
            "SELECT DISTINCT dhatu_id, derivation, lakara, purusha, vacana FROM tinanta WHERE form_slp1 = ?", (exact_word,))]
        krdantas = [dict(row) for row in self.conn.execute(
            "SELECT DISTINCT dhatu_id, derivation, pratyaya FROM krdanta WHERE form_slp1 = ?", (exact_word,))]
        return tinantas, krdantas

    def _analyze_subanta(self, word):
        results = []
        for suffix, info in SUBANTA_ENDINGS.items():
            if word.endswith(suffix):
                stem = word[:-len(suffix)]
                _, krd_stems = self._base_lookup(stem)
                for k in krd_stems:
                    k['subanta_info'] = info
                    results.append({'type': 'krdanta', 'data': k})
        return results

    def _recursive_analyze(self, word, current_prefixes, depth=0):
        results = []
        tins, krds = self._base_lookup(word)
        for t in tins:
            t['prefixes'] = list(current_prefixes)
            results.append({'type': 'tinanta', 'data': t})
        for k in krds:
            k['prefixes'] = list(current_prefixes)
            results.append({'type': 'krdanta', 'data': k})
        subs = self._analyze_subanta(word)
        for s in subs:
            s['data']['prefixes'] = list(current_prefixes)
            results.append(s)
        if depth < 3:
            for sandhi, actual, replace in PREFIX_SPLITS:
                if word.startswith(sandhi):
                    rem = replace + word[len(sandhi):]
                    if len(rem) > 1:
                        new_pref = list(current_prefixes)
                        new_pref.append(actual)
                        results.extend(self._recursive_analyze(rem, new_pref, depth + 1))
        return results

    def analyze(self, word_slp1):
        unique = {}
        for r in self._recursive_analyze(word_slp1, []):
            unique[str(r)] = r
        return list(unique.values())

    def close(self):
        self.conn.close()
