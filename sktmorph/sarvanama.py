from typing import Dict, List, Optional

# Core Sarvanamas hardcoded according to Pāṇinian paradigm tables
PRONOUNS = {
    ('tad', 'pum'): [['saH', 'tO', 'te'],['tam', 'tO', 'tAn'], ['tena', 'tAByAm', 'tEH'],['tasmE', 'tAByAm', 'teByaH'],['tasmAt', 'tAByAm', 'teByaH'], ['tasya', 'tayoH', 'tezAm'],['tasmin', 'tayoH', 'tezu']],
    ('tad', 'stri'): [['sA', 'te', 'tAH'],['tAm', 'te', 'tAH'],['tayA', 'tAByAm', 'tABiH'], ['tasyE', 'tAByAm', 'tAByaH'],['tasyAH', 'tAByAm', 'tAByaH'],['tasyAH', 'tayoH', 'tAsAm'], ['tasyAm', 'tayoH', 'tAzu']],
    ('tad', 'nap'): [['tat', 'te', 'tAni'], ['tat', 'te', 'tAni'], ['tena', 'tAByAm', 'tEH'],['tasmE', 'tAByAm', 'teByaH'], ['tasmAt', 'tAByAm', 'teByaH'],['tasya', 'tayoH', 'tezAm'],['tasmin', 'tayoH', 'tezu']],
    
    ('kim', 'pum'): [['kaH', 'kO', 'ke'],['kam', 'kO', 'kAn'], ['kena', 'kAByAm', 'kEH'],['kasmE', 'kAByAm', 'keByaH'],['kasmAt', 'kAByAm', 'keByaH'], ['kasya', 'kayoH', 'kezAm'],['kasmin', 'kayoH', 'kezu']],
    ('kim', 'stri'): [['kA', 'ke', 'kAH'],['kAm', 'ke', 'kAH'],['kayA', 'kAByAm', 'kABiH'], ['kasyE', 'kAByAm', 'kAByaH'],['kasyAH', 'kAByAm', 'kAByaH'], ['kasyAH', 'kayoH', 'kAsAm'], ['kasyAm', 'kayoH', 'kAzu']],
    ('kim', 'nap'): [['kim', 'ke', 'kAni'], ['kim', 'ke', 'kAni'],['kena', 'kAByAm', 'kEH'], ['kasmE', 'kAByAm', 'keByaH'],['kasmAt', 'kAByAm', 'keByaH'],['kasya', 'kayoH', 'kezAm'], ['kasmin', 'kayoH', 'kezu']],
    
    ('asmad', 'any'): [['aham', 'AvAm', 'vayam'], ['mAm,mA', 'AvAm,nO', 'asmAn,naH'],['mayA', 'AvAByAm', 'asmABiH'],['mahyam,me', 'AvAByAm,nO', 'asmaByam,naH'],['mat', 'AvAByAm', 'asmat'], ['mama,me', 'AvayoH,nO', 'asmAkam,naH'],['mayi', 'AvayoH', 'asmAsu']],
    ('yuzmad', 'any'): [['tvam', 'yuvAm', 'yUyam'],['tvAm,tvA', 'yuvAm,vAm', 'yuzmAn,vaH'],['tvayA', 'yuvAByAm', 'yuzmABiH'],['tuByam,te', 'yuvAByAm,vAm', 'yuzmaByam,vaH'],['tvat', 'yuvAByAm', 'yuzmat'],['tava,te', 'yuvayoH,vAm', 'yuzmAkam,vaH'], ['tvayi', 'yuvayoH', 'yuzmAsu']],
    
    ('sarva', 'pum'): [['sarvaH', 'sarvO', 'sarve'], ['sarvam', 'sarvO', 'sarvAn'],['sarveRa', 'sarvAByAm', 'sarvEH'],['sarvasmE', 'sarvAByAm', 'sarveByaH'], ['sarvasmAt', 'sarvAByAm', 'sarveByaH'],['sarvasya', 'sarvayoH', 'sarvezAm'],['sarvasmin', 'sarvayoH', 'sarvezu'], ['sarva', 'sarvO', 'sarve']],
    ('sarva', 'stri'): [['sarvA', 'sarve', 'sarvAH'], ['sarvAm', 'sarve', 'sarvAH'], ['sarvayA', 'sarvAByAm', 'sarvABiH'], ['sarvasyE', 'sarvAByAm', 'sarvAByaH'], ['sarvasyAH', 'sarvAByAm', 'sarvAByaH'], ['sarvasyAH', 'sarvayoH', 'sarvAsAm'], ['sarvasyAm', 'sarvayoH', 'sarvAzu'], ['sarve', 'sarve', 'sarvAH']],
    ('sarva', 'nap'): [['sarvam', 'sarve', 'sarvARi'], ['sarvam', 'sarve', 'sarvARi'], ['sarveRa', 'sarvAByAm', 'sarvEH'], ['sarvasmE', 'sarvAByAm', 'sarveByaH'], ['sarvasmAt', 'sarvAByAm', 'sarveByaH'], ['sarvasya', 'sarvayoH', 'sarvezAm'], ['sarvasmin', 'sarvayoH', 'sarvezu'], ['sarva', 'sarve', 'sarvARi']],

    ('idam', 'pum'): [['ayam', 'imO', 'ime'], ['enam', 'imO', 'imAn'], ['enena', 'AByAm', 'EBiH'], ['asmai', 'AByAm', 'EByaH'], ['asmAt', 'AByAm', 'EByaH'], ['asya', 'imoH', 'ezAm'], ['asmin', 'imoH', 'ezu']],
    ('idam', 'stri'): [['iyam', 'ime', 'imAH'], ['imAm', 'ime', 'imAH'], ['ayA', 'AByAm', 'ABiH'], ['asyE', 'AByAm', 'AByaH'], ['asyAH', 'AByAm', 'AByaH'], ['asyAH', 'imoH', 'imAm'], ['asyAm', 'imoH', 'imAsu']],
    ('idam', 'nap'): [['idam', 'ime', 'imAni'], ['idam', 'ime', 'imAni'], ['enena', 'AByAm', 'EBiH'], ['asmai', 'AByAm', 'EByaH'], ['asmAt', 'AByAm', 'EByaH'], ['asya', 'imoH', 'ezAm'], ['asmin', 'imoH', 'ezu']],

    ('etad', 'pum'): [['ezaH', 'etO', 'ete'], ['etam', 'etO', 'etAn'], ['etena', 'etAByAm', 'etEBiH'], ['etasmE', 'etAByAm', 'eteByaH'], ['etasmAt', 'etAByAm', 'eteByaH'], ['etasya', 'etayoH', 'etezAm'], ['etasmin', 'etayoH', 'etezu']],
    ('etad', 'stri'): [['etA', 'ete', 'etAH'], ['etAm', 'ete', 'etAH'], ['etayA', 'etAByAm', 'etABiH'], ['etasyE', 'etAByAm', 'etAByaH'], ['etasyAH', 'etAByAm', 'etAByaH'], ['etasyAH', 'etayoH', 'etAsAm'], ['etasyAm', 'etayoH', 'etAsu']],
    ('etad', 'nap'): [['etat', 'ete', 'etAni'], ['etat', 'ete', 'etAni'], ['etena', 'etAByAm', 'etEBiH'], ['etasmE', 'etAByAm', 'eteByaH'], ['etasmAt', 'etAByAm', 'eteByaH'], ['etasya', 'etayoH', 'etezAm'], ['etasmin', 'etayoH', 'etezu']],

    ('yad', 'pum'): [['yaH', 'yO', 'ye'], ['yam', 'yO', 'yAn'], ['yena', 'yAByAm', 'yEH'], ['yasmE', 'yAByAm', 'yeByaH'], ['yasmAt', 'yAByAm', 'yeByaH'], ['yasya', 'yayoH', 'yezAm'], ['yasmin', 'yayoH', 'yezu']],
    ('yad', 'stri'): [['yA', 'ye', 'yAH'], ['yAm', 'ye', 'yAH'], ['yayA', 'yAByAm', 'yABiH'], ['yasyE', 'yAByAm', 'yAByaH'], ['yasyAH', 'yAByAm', 'yAByaH'], ['yasyAH', 'yayoH', 'yAsAm'], ['yasyAm', 'yayoH', 'yAsu']],
    ('yad', 'nap'): [['yat', 'ye', 'yAni'], ['yat', 'ye', 'yAni'], ['yena', 'yAByAm', 'yEH'], ['yasmE', 'yAByAm', 'yeByaH'], ['yasmAt', 'yAByAm', 'yeByaH'], ['yasya', 'yayoH', 'yezAm'], ['yasmin', 'yayoH', 'yezu']],

    ('ubha', 'any'): [['ubhau', 'ubhau', 'ubhe'], ['ubhau', 'ubhau', 'ubhe'], ['ubhABhyAm', 'ubhABhyAm', 'ubhABhiH'], ['ubhABhyAm', 'ubhABhyAm', 'ubhABhyaH'], ['ubhABhyAm', 'ubhABhyAm', 'ubhABhyaH'], ['ubhayoH', 'ubhayoH', 'ubhAm'], ['ubhayoH', 'ubhayoH', 'ubhAsu']],
    ('ena', 'pum'): [['ezaH', 'etO', 'ete'], ['etam', 'etO', 'etAn'], ['etena', 'etAByAm', 'etEBiH'], ['etasmE', 'etAByAm', 'eteByaH'], ['etasmAt', 'etAByAm', 'eteByaH'], ['etasya', 'etayoH', 'etezAm'], ['etasmin', 'etayoH', 'etezu']],
    ('ena', 'stri'): [['etA', 'ete', 'etAH'], ['etAm', 'ete', 'etAH'], ['etayA', 'etAByAm', 'etABiH'], ['etasyE', 'etAByAm', 'etAByaH'], ['etasyAH', 'etAByAm', 'etAByaH'], ['etasyAH', 'etayoH', 'etAsAm'], ['etasyAm', 'etayoH', 'etAsu']],
    ('ena', 'nap'): [['etat', 'ete', 'etAni'], ['etat', 'ete', 'etAni'], ['etena', 'etAByAm', 'etEBiH'], ['etasmE', 'etAByAm', 'eteByaH'], ['etasmAt', 'etAByAm', 'eteByaH'], ['etasya', 'etayoH', 'etezAm'], ['etasmin', 'etayoH', 'etezu']],

    ('eka', 'pum'): [['ekaH', 'ekO', 'eke'], ['ekam', 'ekO', 'ekAn'], ['ekena', 'ekAByAm', 'ekEBiH'], ['ekasmE', 'ekAByAm', 'ekeByaH'], ['ekasmAt', 'ekAByAm', 'ekeByaH'], ['ekasya', 'ekayoH', 'ekezAm'], ['ekasmin', 'ekayoH', 'ekezu']],
    ('eka', 'stri'): [['ekA', 'eke', 'ekAH'], ['ekAm', 'eke', 'ekAH'], ['ekayA', 'ekAByAm', 'ekABiH'], ['ekasyE', 'ekAByAm', 'ekAByaH'], ['ekasyAH', 'ekAByAm', 'ekAByaH'], ['ekasyAH', 'ekayoH', 'ekAsAm'], ['ekasyAm', 'ekayoH', 'ekAsu']],
    ('eka', 'nap'): [['ekam', 'eke', 'ekAni'], ['ekam', 'eke', 'ekAni'], ['ekena', 'ekAByAm', 'ekEBiH'], ['ekasmE', 'ekAByAm', 'ekeByaH'], ['ekasmAt', 'ekAByAm', 'ekeByaH'], ['ekasya', 'ekayoH', 'ekezAm'], ['ekasmin', 'ekayoH', 'ekezu']],

    ('dvi', 'pum'): [['dviH', 'dvO', 'dve'], ['dvim', 'dvO', 'dvIn'], ['dviByA', 'dviByAm', 'dviBiH'], ['dviByE', 'dviByAm', 'dviByaH'], ['dviByaH', 'dviByAm', 'dviByaH'], ['dviByaH', 'dviByoH', 'dviByAm'], ['dviByi', 'dviByoH', 'dviByasu']],
    ('dvi', 'stri'): [['dviH', 'dve', 'dviH'], ['dviHm', 'dve', 'dviH'], ['dviByA', 'dviByAm', 'dviBiH'], ['dviByE', 'dviByAm', 'dviByaH'], ['dviByaH', 'dviByAm', 'dviByaH'], ['dviByaH', 'dviByoH', 'dviByAm'], ['dviByi', 'dviByoH', 'dviByasu']],
    ('dvi', 'nap'): [['dvi', 'dvi', 'dvi'], ['dvi', 'dvi', 'dvi'], ['dviByA', 'dviByAm', 'dviBiH'], ['dviByE', 'dviByAm', 'dviByaH'], ['dviByaH', 'dviByAm', 'dviByaH'], ['dviByaH', 'dviByoH', 'dviByAm'], ['dviByi', 'dviByoH', 'dviByasu']],

    ('sva', 'pum'): [['svaH', 'svO', 'sve'], ['svam', 'svO', 'svAn'], ['svena', 'svAByAm', 'svEBiH'], ['svasmE', 'svAByAm', 'sveByaH'], ['svasmAt', 'svAByAm', 'sveByaH'], ['svasya', 'svayoH', 'svezAm'], ['svasmin', 'svayoH', 'svezu']],
    ('sva', 'stri'): [['svA', 'sve', 'svAH'], ['svAm', 'sve', 'svAH'], ['svayA', 'svAByAm', 'svABiH'], ['svasyE', 'svAByAm', 'svAByaH'], ['svasyAH', 'svAByAm', 'svAByaH'], ['svasyAH', 'svayoH', 'svAsAm'], ['svasyAm', 'svayoH', 'svAsu']],
    ('sva', 'nap'): [['svam', 'sve', 'svAni'], ['svam', 'sve', 'svAni'], ['svena', 'svAByAm', 'svEBiH'], ['svasmE', 'svAByAm', 'sveByaH'], ['svasmAt', 'svAByAm', 'sveByaH'], ['svasya', 'svayoH', 'svezAm'], ['svasmin', 'svayoH', 'svezu']],

    ('am', 'any'): [['am', 'am', 'am'], ['am', 'am', 'am'], ['am', 'am', 'am'], ['am', 'am', 'am'], ['am', 'am', 'am'], ['am', 'am', 'am'], ['am', 'am', 'am']],
}

class SarvanamaGenerator:
    def __init__(self):
        self.vibhakti_names =["prathamA", "dvitIyA", "tfIyA", "caturTI", "paYcamI", "zazWI", "saptamI", "samboDana"]

    def generate(self, base: str, linga: str) -> Optional[Dict[str, List[str]]]:
        if base in ['asmad', 'yuzmad', 'ubha', 'am']:
            linga = 'any'
            
        endings = PRONOUNS.get((base, linga))
        if not endings:
            raise NotImplementedError(f"Pronoun '{base}' in gender '{linga}' is not implemented.")
            
        table = {}
        for i, forms in enumerate(endings):
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
