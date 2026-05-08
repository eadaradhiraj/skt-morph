from .analyzer import SanskritAnalyzer

def apply_sandhi(prefix, base):
    if not prefix: return base
    if prefix.endswith('a') and base.startswith('a'):
        return prefix[:-1] + 'A' + base[1:]
    if prefix.endswith('a') and base.startswith('i'):
        return prefix[:-1] + 'e' + base[1:]
    if prefix.endswith('i') and base.startswith('a'):
        return prefix[:-1] + 'y' + base
    if prefix == 'sam' and base[0] not in 'aeiouAIUfFxX':
        return 'saM' + base
    return prefix + base

class SanskritGenerator:
    def __init__(self):
        self.analyzer = SanskritAnalyzer()

    def get_dhatu_id(self, root_slp1):
        res = self.analyzer.conn.execute(
            "SELECT dhatu_id FROM dhatu_meta WHERE root_slp1 = ?", (root_slp1,)).fetchone()
        return res['dhatu_id'] if res else None

    def generate_tinanta(self, root_or_id, lakara, purusha, vacana, derivation='vidyut_shuddha_kartari', prefixes=None):
        d_id = root_or_id if "." in root_or_id else self.get_dhatu_id(root_or_id)
        if not d_id: return None
        db_lakara = {'lat': 'plat', 'lan': 'plang'}.get(lakara, lakara)
        query = "SELECT form_slp1 FROM tinanta WHERE dhatu_id=? AND lakara=? AND purusha=? AND vacana=? AND derivation=?"
        row = self.analyzer.conn.execute(query, (d_id, db_lakara, purusha.title(), vacana.title(), derivation)).fetchone()
        if not row: return None
        res = row['form_slp1']
        if prefixes:
            for p in reversed(prefixes):
                res = apply_sandhi(p, res)
        return res
