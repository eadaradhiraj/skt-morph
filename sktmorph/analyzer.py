import sqlite3
from pathlib import Path

# Mapping of Sandhi forms to actual prefixes and the replacement character for the remainder.
# Sorted roughly by length so longer combinations are matched before shorter ones.
PREFIX_SPLITS =[
    # SAM
    ('saM', 'sam', ''), ('sam', 'sam', ''), ('saN', 'sam', ''), ('saY', 'sam', ''), ('saR', 'sam', ''), ('san', 'sam', ''),
    # PRA / PARA
    ('pratyA', 'prati', 'A'), ('pratya', 'prati', 'a'), ('prati', 'prati', ''),
    ('paryA', 'pari', 'A'), ('parya', 'pari', 'a'), ('pari', 'pari', ''),
    ('prA', 'pra', 'a'), ('prA', 'pra', 'A'), ('pre', 'pra', 'i'), ('pra', 'pra', ''),
    ('parA', 'parA', ''),
    # UPA / APA / API
    ('upA', 'upa', 'a'), ('upA', 'upa', 'A'), ('upe', 'upa', 'i'), ('upa', 'upa', ''),
    ('apA', 'apa', 'a'), ('apA', 'apa', 'A'), ('ape', 'apa', 'i'), ('apa', 'apa', ''),
    ('apyA', 'api', 'A'), ('apya', 'api', 'a'), ('api', 'api', ''),
    # VI / NI / ADHI / ABHI / ATI
    ('vyA', 'vi', 'A'), ('vya', 'vi', 'a'), ('vi', 'vi', ''),
    ('nyA', 'ni', 'A'), ('nya', 'ni', 'a'), ('ni', 'ni', ''),
    ('aDyA', 'aDi', 'A'), ('aDya', 'aDi', 'a'), ('aDi', 'aDi', ''),
    ('aByA', 'aBi', 'A'), ('aBya', 'aBi', 'a'), ('aBi', 'aBi', ''),
    ('atyA', 'ati', 'A'), ('atya', 'ati', 'a'), ('ati', 'ati', ''),
    # UD / UT
    ('udA', 'ud', 'A'), ('uda', 'ud', 'a'), ('ud', 'ud', ''), ('ut', 'ud', ''),
    # AVA
    ('avA', 'ava', 'a'), ('avA', 'ava', 'A'), ('ave', 'ava', 'i'), ('ava', 'ava', ''),
    # NIS / DUR
    ('nir', 'nir', ''), ('nis', 'nir', ''), ('niz', 'nir', ''), ('niH', 'nir', ''),
    ('dur', 'dur', ''), ('dus', 'dur', ''), ('duz', 'dur', ''), ('duH', 'dur', ''),
    # A / SU
    ('svA', 'su', 'A'), ('sva', 'su', 'a'), ('su', 'su', ''),
    ('A', 'A', ''),
]

class SanskritAnalyzer:
    def __init__(self):
        self.db_path = Path(__file__).parent / "data" / "sanskrit_forms.sqlite"
        if not self.db_path.exists():
            raise FileNotFoundError("Database missing! Run build_db.py")
        
        self.conn = sqlite3.connect(self.db_path)
        self.conn.row_factory = sqlite3.Row

    def _base_lookup(self, exact_word):
        """Looks up the exact word in the database without touching prefixes."""
        tinantas =[dict(row) for row in self.conn.execute(
            "SELECT DISTINCT dhatu_id, derivation, lakara, purusha, vacana FROM tinanta WHERE form_slp1 = ?", 
            (exact_word,)
        )]
        krdantas =[dict(row) for row in self.conn.execute(
            "SELECT DISTINCT dhatu_id, derivation, pratyaya FROM krdanta WHERE form_slp1 = ?", 
            (exact_word,)
        )]
        return tinantas, krdantas

    def _recursive_analyze(self, word, current_prefixes, depth=0):
        """Recursively strip prefixes and search the database."""
        results =[]
        
        # 1. Base case: Check if current word is a valid root form
        tinantas, krdantas = self._base_lookup(word)
        
        # Also check for simple Krdanta Subanta (like Saptami 'Bavati' -> base 'Bavat')
        if not tinantas and not krdantas and word.endswith('i'):
            _, sub_krdantas = self._base_lookup(word[:-1]) # Strip 'i', check 'Bavat'
            for k in sub_krdantas:
                k['subanta_note'] = 'Saptami Eka-vacana'
                krdantas.append(k)

        for t in tinantas:
            t['prefixes'] = list(current_prefixes)
            results.append({'type': 'tinanta', 'data': t})
            
        for k in krdantas:
            k['prefixes'] = list(current_prefixes)
            results.append({'type': 'krdanta', 'data': k})

        # 2. Recursive case: Try to peel off another prefix
        # Max 3 prefixes (e.g., sam + pra + A) to prevent infinite loops
        if depth < 3:
            for sandhi_form, actual_prefix, replacement in PREFIX_SPLITS:
                if word.startswith(sandhi_form):
                    remainder = replacement + word[len(sandhi_form):]
                    if len(remainder) > 1: # Sanity check
                        new_prefixes = list(current_prefixes)
                        new_prefixes.append(actual_prefix)
                        results.extend(self._recursive_analyze(remainder, new_prefixes, depth + 1))
                        
        return results

    def analyze(self, word_slp1):
        """Main entry point for analysis."""
        # Use a dictionary to easily remove duplicates
        unique_results = {}
        raw_results = self._recursive_analyze(word_slp1,[])
        
        for r in raw_results:
            # Create a unique hash for each result so we don't print duplicates
            key = str(r)
            unique_results[key] = r
            
        return list(unique_results.values())

    def close(self):
        self.conn.close()