import sqlite3
from pathlib import Path

PREFIX_SPLITS = [
    ('saM', 'sam', ''), ('sam', 'sam', ''), ('saN', 'sam', ''), ('saY', 'sam', ''), ('saR', 'sam', ''), ('san', 'sam', ''),
    ('pratyA', 'prati', 'A'), ('pratya', 'prati', 'a'), ('prati', 'prati', ''),
    ('paryA', 'pari', 'A'), ('parya', 'pari', 'a'), ('pari', 'pari', ''),
    ('upA', 'upa', 'a'), ('upA', 'upa', 'A'), ('upe', 'upa', 'i'), ('upa', 'upa', ''),
    ('apA', 'apa', 'a'), ('apA', 'apa', 'A'), ('ape', 'apa', 'i'), ('apa', 'apa', ''),
    ('vyA', 'vi', 'A'), ('vya', 'vi', 'a'), ('vi', 'vi', ''),
    ('nyA', 'ni', 'A'), ('nya', 'ni', 'a'), ('ni', 'ni', ''),
    ('aDyA', 'aDi', 'A'), ('aDya', 'aDi', 'a'), ('aDi', 'aDi', ''),
    ('aByA', 'aBi', 'A'), ('aBya', 'aBi', 'a'), ('aBi', 'aBi', ''),
    ('atyA', 'ati', 'A'), ('atya', 'ati', 'a'), ('ati', 'ati', ''),
    ('udA', 'ud', 'A'), ('uda', 'ud', 'a'), ('ud', 'ud', ''), ('ut', 'ud', ''),
    ('avA', 'ava', 'a'), ('avA', 'ava', 'A'), ('ave', 'ava', 'i'), ('ava', 'ava', ''),
    ('nir', 'nir', ''), ('nis', 'nir', ''), ('niz', 'nir', ''), ('niH', 'nir', ''),
    ('dur', 'dur', ''), ('dus', 'dur', ''), ('duz', 'dur', ''), ('duH', 'dur', ''),
    ('svA', 'su', 'A'), ('sva', 'su', 'a'), ('su', 'su', ''),
    ('prA', 'pra', 'a'), ('prA', 'pra', 'A'), ('pre', 'pra', 'i'), ('pra', 'pra', ''),
    ('parA', 'parA', ''), ('A', 'A', ''), ('apa', 'apa', ''), ('anu', 'anu', '')
]

SUBANTA_ENDINGS = {'i': 'Saptami Eka (Loc. Sg.)', 'am': 'Dvitiya Eka', 'as': 'Prathama Bahu'}

class SanskritAnalyzer:
    def __init__(self):
        self.db_path = Path(__file__).parent / "data" / "sanskrit_forms.sqlite"
        self.conn = sqlite3.connect(self.db_path)
        self.conn.row_factory = sqlite3.Row

    def _base_lookup(self, word):
        t_q = "SELECT t.dhatu_id, t.derivation, t.lakara, t.purusha, t.vacana, m.root_slp1 FROM tinanta t LEFT JOIN dhatu_meta m ON t.dhatu_id = m.dhatu_id WHERE t.form_slp1 = ?"
        k_q = "SELECT k.dhatu_id, k.derivation, k.pratyaya, m.root_slp1 FROM krdanta k LEFT JOIN dhatu_meta m ON k.dhatu_id = m.dhatu_id WHERE k.form_slp1 = ?"
        tins = [dict(row) for row in self.conn.execute(t_q, (word,))]
        krds = [dict(row) for row in self.conn.execute(k_q, (word,))]
        return tins, krds

    def _analyze_subanta(self, word):
        results = []
        for suffix, info in SUBANTA_ENDINGS.items():
            if word.endswith(suffix):
                stem = word[:-len(suffix)]
                _, krds = self._base_lookup(stem)
                for k in krds:
                    item = k.copy()
                    item['subanta_info'] = info
                    results.append({'type': 'krdanta', 'data': item})
        return results

    def _recursive_analyze(self, word, prefixes, depth=0):
        results = []
        tins, krds = self._base_lookup(word)
        for t in tins:
            t['prefixes'] = tuple(prefixes)
            results.append({'type': 'tinanta', 'data': t})
        for k in krds:
            k['prefixes'] = tuple(prefixes)
            results.append({'type': 'krdanta', 'data': k})
        for s in self._analyze_subanta(word):
            s['data']['prefixes'] = tuple(prefixes)
            results.append(s)
        if depth < 3:
            for s, a, r in PREFIX_SPLITS:
                if word.startswith(s):
                    rem = r + word[len(s):]
                    if len(rem) > 1:
                        results.extend(self._recursive_analyze(rem, prefixes + [a], depth + 1))
        return results

    def analyze(self, word):
        unique = {}
        for r in self._recursive_analyze(word, []):
            d = r['data']
            # Stricter key: Ignore derivation source to prevent duplicates
            if r['type'] == 'tinanta':
                key = (d['dhatu_id'], d['lakara'], d['purusha'], d['vacana'], d['prefixes'])
            else:
                key = (d['dhatu_id'], d['pratyaya'], d.get('subanta_info'), d['prefixes'])
            unique[key] = r
        
        final = list(unique.values())
        for f in final: f['data']['prefixes'] = list(f['data']['prefixes'])
        return final

    def close(self):
        self.conn.close()
