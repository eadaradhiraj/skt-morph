from typing import Dict, List, Optional

# Core Sarvanamas hardcoded according to Pāṇinian paradigm tables
PRONOUNS = {
    ('tad', 'pum'): [['saH', 'tO', 'te'],['tam', 'tO', 'tAn'], ['tena', 'tAByAm', 'tEH'],['tasmE', 'tAByAm', 'teByaH'],['tasmAt', 'tAByAm', 'teByaH'], ['tasya', 'tayoH', 'tezAm'],['tasmin', 'tayoH', 'tezu']],
    ('tad', 'stri'): [['sA', 'te', 'tAH'],['tAm', 'te', 'tAH'],['tayA', 'tAByAm', 'tABiH'], ['tasyE', 'tAByAm', 'tAByaH'],['tasyAH', 'tAByAm', 'tAByaH'],['tasyAH', 'tayoH', 'tAsAm'], ['tasyAm', 'tayoH', 'tAzu']],
    ('tad', 'nap'): [['tat', 'te', 'tAni'], ['tat', 'te', 'tAni'], ['tena', 'tAByAm', 'tEH'],['tasmE', 'tAByAm', 'teByaH'], ['tasmAt', 'tAByAm', 'teByaH'],['tasya', 'tayoH', 'tezAm'],['tasmin', 'tayoH', 'tezu']],
    
    ('kim', 'pum'): [['kaH', 'kO', 'ke'],['kam', 'kO', 'kAn'], ['kena', 'kAByAm', 'kEH'],['kasmE', 'kAByAm', 'keByaH'],['kasmAt', 'kAByAm', 'keByaH'], ['kasya', 'kayoH', 'kezAm'],['kasmin', 'kayoH', 'kezu']],
    ('kim', 'stri'): [['kA', 'ke', 'kAH'],['kAm', 'ke', 'kAH'],['kayA', 'kAByAm', 'kABiH'], ['kasyE', 'kAByAm', 'kAByaH'],['kasyAH', 'kAByAm', 'kAByaH'], ['kasyAH', 'kayoH', 'kAsAm'], ['kasyAm', 'kayoH', 'kAzu']],
    ('kim', 'nap'): [['kim', 'ke', 'kAni'], ['kim', 'ke', 'kAni'],['kena', 'kAByAm', 'kEH'], ['kasmE', 'kAByAm', 'keByaH'],['kasmAt', 'kAByAm', 'keByaH'],['kasya', 'kayoH', 'kezAm'], ['kasmin', 'kayoH', 'kezu']],
    
    # Asmad and Yuzmad apply universally regardless of gender
    ('asmad', 'any'): [['aham', 'AvAm', 'vayam'], ['mAm,mA', 'AvAm,nO', 'asmAn,naH'],['mayA', 'AvAByAm', 'asmABiH'],['mahyam,me', 'AvAByAm,nO', 'asmaByam,naH'],['mat', 'AvAByAm', 'asmat'], ['mama,me', 'AvayoH,nO', 'asmAkam,naH'],['mayi', 'AvayoH', 'asmAsu']],
    ('yuzmad', 'any'): [['tvam', 'yuvAm', 'yUyam'],['tvAm,tvA', 'yuvAm,vAm', 'yuzmAn,vaH'],['tvayA', 'yuvAByAm', 'yuzmABiH'],['tuByam,te', 'yuvAByAm,vAm', 'yuzmaByam,vaH'],['tvat', 'yuvAByAm', 'yuzmat'],['tava,te', 'yuvayoH,vAm', 'yuzmAkam,vaH'], ['tvayi', 'yuvayoH', 'yuzmAsu']],
    
    ('sarva', 'pum'): [['sarvaH', 'sarvO', 'sarve'], ['sarvam', 'sarvO', 'sarvAn'],['sarveRa', 'sarvAByAm', 'sarvEH'],['sarvasmE', 'sarvAByAm', 'sarveByaH'], ['sarvasmAt', 'sarvAByAm', 'sarveByaH'],['sarvasya', 'sarvayoH', 'sarvezAm'],['sarvasmin', 'sarvayoH', 'sarvezu'], ['sarva', 'sarvO', 'sarve']]
}

class SarvanamaGenerator:
    def __init__(self):
        self.vibhakti_names =["prathamA", "dvitIyA", "tfIyA", "caturTI", "paYcamI", "zazWI", "saptamI", "samboDana"]

    def generate(self, base: str, linga: str) -> Optional[Dict[str, List[str]]]:
        if base in ['asmad', 'yuzmad']:
            linga = 'any'
            
        endings = PRONOUNS.get((base, linga))
        if not endings:
            raise NotImplementedError(f"Pronoun '{base}' in gender '{linga}' is not implemented.")
            
        table = {}
        for i, forms in enumerate(endings):
            # Formats alternate forms beautifully with slashes (e.g. mAm/mA)
            table[self.vibhakti_names[i]] =[f.replace(',', '/') for f in forms]
        return table

    def analyze(self, word: str) -> List[Dict[str, str]]:
        results =[]
        for (base, linga), table in PRONOUNS.items():
            for vibh_idx, row in enumerate(table):
                for vacana_idx, forms_str in enumerate(row):
                    for form in forms_str.split(','):
                        if form == word:
                            results.append({
                                'pratipadika': base,
                                'linga': linga,
                                'vibhakti': self.vibhakti_names[vibh_idx],
                                'vacana': vacana_idx + 1
                            })
        return results
