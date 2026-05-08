from sktmorph.analyzer import SanskritAnalyzer

def apply_sandhi(prefix, base):
    """Joins prefixes to the base form using basic Sandhi."""
    # pra + aBavat -> prABavat
    if prefix.endswith('a') and base.startswith('a'):
        return prefix[:-1] + 'A' + base[1:]
    # pra + ikzate -> prekzate
    if prefix.endswith('a') and base.startswith('i'):
        return prefix[:-1] + 'e' + base[1:]
    # vi + ayate -> vyayate
    if prefix.endswith('i') and base.startswith('a'):
        return prefix[:-1] + 'y' + base
    # sam + Bavati -> saMBavati
    if prefix == 'sam' and base[0] not in 'aeiouAIUfFxX':
        return 'saM' + base
    return prefix + base

class SanskritGenerator:
    def __init__(self):
        self.analyzer = SanskritAnalyzer()

    # Inside sktmorph/generator.py

    def get_dhatu_id(self, root_slp1):
        """Looks up 'BU' and returns '01.0001'"""
        # Note: root_slp1 must be exact SLP1 (e.g. 'BU' not 'bu')
        query = "SELECT dhatu_id FROM dhatu_meta WHERE root_slp1 = ?"
        res = self.analyzer.conn.execute(query, (root_slp1,)).fetchone()
        return res['dhatu_id'] if res else None

    def generate_tinanta(self, root_or_id, lakara, purusha, vacana, derivation='vidyut_shuddha_kartari', prefixes=None):
        # 1. Handle Root Name vs ID
        if "." in root_or_id:
            dhatu_id = root_or_id
        else:
            dhatu_id = self.get_dhatu_id(root_or_id)
        
        if not dhatu_id:
            return None

        # 2. Map Tense names to your specific JSON keys
        # Your files use 'plat' for lat, 'plit' for lit, etc.
        tense_map = {
            'lat': 'plat',
            'lit': 'plit',
            'lut': 'plut',
            'lrt': 'pqlat', # or pqlrt depending on your files
            'lot': 'pqlot',
            'lan': 'plang',
        }
        db_lakara = tense_map.get(lakara.lower(), lakara)

        # 3. Format strings for DB matching
        purusha = purusha.title() # eka -> Eka
        vacana = vacana.title()   # prathama -> Prathama

        query = """
            SELECT form_slp1 FROM tinanta 
            WHERE dhatu_id = ? AND lakara = ? AND purusha = ? AND vacana = ? AND derivation = ?
        """
        row = self.analyzer.conn.execute(query, (dhatu_id, db_lakara, purusha, vacana, derivation)).fetchone()
        
        if not row:
            return None
        
        # 4. Apply prefixes with Sandhi
        result = row['form_slp1']
        if prefixes:
            for p in reversed(prefixes):
                result = apply_sandhi(p, result)
        
        return result