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

    ('traya', 'pum'): [['trayaH', 'trayO', 'trayaH'], ['trayam', 'trayO', 'trIn'], ['tribhiH', 'tribhyAm', 'tribhiH'], ['tribhyaH', 'tribhyAm', 'tribhyaH'], ['tribhyaH', 'tribhyAm', 'tribhyaH'], ['trayasya', 'trayoH', 'trayARAm'], ['trizu', 'trizu', 'trizu']],
    ('traya', 'stri'): [['tisrA', 'tisro', 'tisraH'], ['tisrAm', 'tisro', 'tisraH'], ['tisrbhiH', 'tisrbhyAm', 'tisrbhiH'], ['tisrbhyaH', 'tisrbhyAm', 'tisrbhyaH'], ['tisrbhyaH', 'tisrbhyAm', 'tisrbhyaH'], ['tisrasya', 'tisroH', 'tisrARAm'], ['tisrizu', 'tisrizu', 'tisrizu']],
    ('traya', 'nap'): [['trayam', 'trIRi', 'trIRi'], ['trayam', 'trIRi', 'trIRi'], ['tribhiH', 'tribhyAm', 'tribhiH'], ['tribhyaH', 'tribhyAm', 'tribhyaH'], ['tribhyaH', 'tribhyAm', 'tribhyaH'], ['trayasya', 'trayoH', 'trayARAm'], ['trizu', 'trizu', 'trizu']],

    ('catur', 'pum'): [['catvAraH', 'catvArO', 'catvAraH'], ['caturam', 'catvArO', 'catvARi'], ['caturBiH', 'caturByAm', 'caturBiH'], ['caturByaH', 'caturByAm', 'caturByaH'], ['caturByaH', 'caturByAm', 'caturByaH'], ['caturasya', 'caturNoH', 'caturARAm'], ['caturzu', 'caturzu', 'caturzu']],
    ('catur', 'stri'): [['catasrA', 'catasro', 'catasraH'], ['catasrAm', 'catasro', 'catasraH'], ['catasrbhiH', 'catasrbhyAm', 'catasrbhiH'], ['catasrbhyaH', 'catasrbhyAm', 'catasrbhyaH'], ['catasrbhyaH', 'catasrbhyAm', 'catasrbhyaH'], ['catasrasya', 'catasroH', 'catasrARAm'], ['catasrizu', 'catasrizu', 'catasrizu']],
    ('catur', 'nap'): [['catur', 'catvArI', 'catvARi'], ['catur', 'catvArI', 'catvARi'], ['caturBiH', 'caturByAm', 'caturBiH'], ['caturByaH', 'caturByAm', 'caturByaH'], ['caturByaH', 'caturByAm', 'caturByaH'], ['caturasya', 'caturNoH', 'caturARAm'], ['caturzu', 'caturzu', 'caturzu']],

    ('purva', 'pum'): [['purvaH', 'purvO', 'purve'], ['purvam', 'purvO', 'purvAn'], ['purveRa', 'purvAByAm', 'purvEH'], ['purvasmE', 'purvAByAm', 'purveByaH'], ['purvasmAt', 'purvAByAm', 'purveByaH'], ['purvasya', 'purvayoH', 'purvezAm'], ['purvasmin', 'purvayoH', 'purvezu'], ['purva', 'purvO', 'purve']],
    ('purva', 'stri'): [['purvA', 'purve', 'purvAH'], ['purvAm', 'purve', 'purvAH'], ['purvayA', 'purvAByAm', 'purvABiH'], ['purvasyE', 'purvAByAm', 'purvAByaH'], ['purvasyAH', 'purvAByAm', 'purvAByaH'], ['purvasyAH', 'purvayoH', 'purvAsAm'], ['purvasyAm', 'purvayoH', 'purvAzu'], ['purve', 'purve', 'purvAH']],
    ('purva', 'nap'): [['purvam', 'purve', 'purvARi'], ['purvam', 'purve', 'purvARi'], ['purveRa', 'purvAByAm', 'purvEH'], ['purvasmE', 'purvAByAm', 'purveByaH'], ['purvasmAt', 'purvAByAm', 'purveByaH'], ['purvasya', 'purvayoH', 'purvezAm'], ['purvasmin', 'purvayoH', 'purvezu'], ['purva', 'purve', 'purvARi']],

    ('para', 'pum'): [['paraH', 'parO', 'pare'], ['param', 'parO', 'parAn'], ['pareRa', 'parAByAm', 'parEH'], ['parasmE', 'parAByAm', 'pareByaH'], ['parasmAt', 'parAByAm', 'pareByaH'], ['parasya', 'parayoH', 'parezAm'], ['parasmin', 'parayoH', 'parezu'], ['para', 'parO', 'pare']],
    ('para', 'stri'): [['parA', 'pare', 'parAH'], ['parAm', 'pare', 'parAH'], ['parayA', 'parAByAm', 'parABiH'], ['parasyE', 'parAByAm', 'parAByaH'], ['parasyAH', 'parAByAm', 'parAByaH'], ['parasyAH', 'parayoH', 'parAsAm'], ['parasyAm', 'parayoH', 'parAzu'], ['pare', 'pare', 'parAH']],
    ('para', 'nap'): [['param', 'pare', 'parARi'], ['param', 'pare', 'parARi'], ['pareRa', 'parAByAm', 'parEH'], ['parasmE', 'parAByAm', 'pareByaH'], ['parasmAt', 'parAByAm', 'pareByaH'], ['parasya', 'parayoH', 'parezAm'], ['parasmin', 'parayoH', 'parezu'], ['para', 'pare', 'parARi']],

    ('apara', 'pum'): [['aparaH', 'aparO', 'apare'], ['aparam', 'aparO', 'aparAn'], ['apareRa', 'aparAByAm', 'aparEH'], ['aparasmE', 'aparAByAm', 'apareByaH'], ['aparasmAt', 'aparAByAm', 'apareByaH'], ['aparasya', 'aparayoH', 'aparezAm'], ['aparasmin', 'aparayoH', 'aparezu'], ['apara', 'aparO', 'apare']],
    ('apara', 'stri'): [['aparA', 'apare', 'aparAH'], ['aparAm', 'apare', 'aparAH'], ['aparayA', 'aparAByAm', 'aparABiH'], ['aparasyE', 'aparAByAm', 'aparAByaH'], ['aparasyAH', 'aparAByAm', 'aparAByaH'], ['aparasyAH', 'aparayoH', 'aparAsAm'], ['aparasyAm', 'aparayoH', 'aparAzu'], ['apare', 'apare', 'aparAH']],
    ('apara', 'nap'): [['aparam', 'apare', 'aparARi'], ['aparam', 'apare', 'aparARi'], ['apareRa', 'aparAByAm', 'aparEH'], ['aparasmE', 'aparAByAm', 'apareByaH'], ['aparasmAt', 'aparAByAm', 'apareByaH'], ['aparasya', 'aparayoH', 'aparezAm'], ['aparasmin', 'aparayoH', 'aparezu'], ['apara', 'apare', 'aparARi']],

    ('anya', 'pum'): [['anyaH', 'anyO', 'anye'], ['anyam', 'anyO', 'anyAn'], ['anyena', 'anyAByAm', 'anyEH'], ['anyasmE', 'anyAByAm', 'anyeByaH'], ['anyasmAt', 'anyAByAm', 'anyeByaH'], ['anyasya', 'anyayoH', 'anyezAm'], ['anyasmin', 'anyayoH', 'anyezu'], ['anya', 'anyO', 'anye']],
    ('anya', 'stri'): [['anyA', 'anye', 'anyAH'], ['anyAm', 'anye', 'anyAH'], ['anyayA', 'anyAByAm', 'anyABiH'], ['anyasyE', 'anyAByAm', 'anyAByaH'], ['anyasyAH', 'anyAByAm', 'anyAByaH'], ['anyasyAH', 'anyayoH', 'anyAsAm'], ['anyasyAm', 'anyayoH', 'anyAzu'], ['anye', 'anye', 'anyAH']],
    ('anya', 'nap'): [['anyam', 'anye', 'anyARi'], ['anyam', 'anye', 'anyARi'], ['anyena', 'anyAByAm', 'anyEH'], ['anyasmE', 'anyAByAm', 'anyeByaH'], ['anyasmAt', 'anyAByAm', 'anyeByaH'], ['anyasya', 'anyayoH', 'anyezAm'], ['anyasmin', 'anyayoH', 'anyezu'], ['anya', 'anye', 'anyARi']],

    ('paJcan', 'pum'): [['paJcaH', 'paJcO', 'paJcaH'], ['paJcam', 'paJcO', 'paJcan'], ['paJcaBiH', 'paJcaByAm', 'paJcaBiH'], ['paJcaByaH', 'paJcaByAm', 'paJcaByaH'], ['paJcaByaH', 'paJcaByAm', 'paJcaByaH'], ['paJcasya', 'paJcoH', 'paJcAnAm'], ['paJcasu', 'paJcasu', 'paJcasu']],
    ('paJcan', 'stri'): [['paYcI', 'paYco', 'paYcaH'], ['paYcIm', 'paYco', 'paYcaH'], ['paYcIBiH', 'paYcIByAm', 'paYcIBiH'], ['paYcIByaH', 'paYcIByAm', 'paYcIByaH'], ['paYcIByaH', 'paYcIByAm', 'paYcIByaH'], ['paYcIByaH', 'paYcoH', 'paYcInAm'], ['paYcIsu', 'paYcoH', 'paYcIsu']],
    ('paJcan', 'nap'): [['paJcan', 'paJcAni', 'paJcAni'], ['paJcan', 'paJcAni', 'paJcAni'], ['paJcaBiH', 'paJcaByAm', 'paJcaBiH'], ['paJcaByaH', 'paJcaByAm', 'paJcaByaH'], ['paJcaByaH', 'paJcaByAm', 'paJcaByaH'], ['paJcasya', 'paJcoH', 'paJcAnAm'], ['paJcasu', 'paJcasu', 'paJcasu']],

    ('zaq', 'pum'): [['zaq', 'zaqO', 'zaq'], ['zaqam', 'zaqO', 'zaws'], ['zaqBiH', 'zaqByAm', 'zaqBiH'], ['zaqByaH', 'zaqByAm', 'zaqByaH'], ['zaqByaH', 'zaqByAm', 'zaqByaH'], ['zaqasya', 'zaqNoH', 'zawsAm'], ['zaqzu', 'zaqzu', 'zaqzu']],
    ('zaq', 'stri'): [['zaqI', 'zaqo', 'zaqaH'], ['zaqIm', 'zaqo', 'zaqaH'], ['zaqIBiH', 'zaqIByAm', 'zaqIBiH'], ['zaqIByaH', 'zaqIByAm', 'zaqIByaH'], ['zaqIByaH', 'zaqIByAm', 'zaqIByaH'], ['zaqIByaH', 'zaqoH', 'zaqInAm'], ['zaqIsu', 'zaqoH', 'zaqIsu']],
    ('zaq', 'nap'): [['zaq', 'zaqRI', 'zaqRi'], ['zaq', 'zaqRI', 'zaqRi'], ['zaqBiH', 'zaqByAm', 'zaqBiH'], ['zaqByaH', 'zaqByAm', 'zaqByaH'], ['zaqByaH', 'zaqByAm', 'zaqByaH'], ['zaqasya', 'zaqNoH', 'zawsAm'], ['zaqzu', 'zaqzu', 'zaqzu']],

    ('saptan', 'pum'): [['saptaH', 'saptO', 'saptaH'], ['saptam', 'saptO', 'saptAn'], ['saptaBiH', 'saptaByAm', 'saptaBiH'], ['saptaByaH', 'saptaByAm', 'saptaByaH'], ['saptaByaH', 'saptaByAm', 'saptaByaH'], ['saptasya', 'saptoH', 'saptAnAm'], ['saptasu', 'saptasu', 'saptasu']],
    ('saptan', 'stri'): [['saptI', 'sapto', 'saptaH'], ['saptIm', 'sapto', 'saptaH'], ['saptIBiH', 'saptIByAm', 'saptIBiH'], ['saptIByaH', 'saptIByAm', 'saptIByaH'], ['saptIByaH', 'saptIByAm', 'saptIByaH'], ['saptIByaH', 'saptoH', 'saptInAm'], ['saptIsu', 'saptoH', 'saptIsu']],
    ('saptan', 'nap'): [['sapta', 'saptAni', 'saptAni'], ['sapta', 'saptAni', 'saptAni'], ['saptaBiH', 'saptaByAm', 'saptaBiH'], ['saptaByaH', 'saptaByAm', 'saptaByaH'], ['saptaByaH', 'saptaByAm', 'saptaByaH'], ['saptasya', 'saptoH', 'saptAnAm'], ['saptasu', 'saptasu', 'saptasu']],

    ('azwan', 'pum'): [['azwaH', 'azwO', 'azwaH'], ['azwam', 'azwO', 'azWAn'], ['azwaBiH', 'azwaByAm', 'azwaBiH'], ['azwaByaH', 'azwaByAm', 'azwaByaH'], ['azwaByaH', 'azwaByAm', 'azwaByaH'], ['azwasya', 'azwoH', 'azwAnAm'], ['azwasu', 'azwasu', 'azwasu']],
    ('azwan', 'stri'): [['azwI', 'azwo', 'azwaH'], ['azwIm', 'azwo', 'azwaH'], ['azwIBiH', 'azwIByAm', 'azwIBiH'], ['azwIByaH', 'azwIByAm', 'azwIByaH'], ['azwIByaH', 'azwIByAm', 'azwIByaH'], ['azwIByaH', 'azwoH', 'azwInAm'], ['azwIsu', 'azwoH', 'azwIsu']],
    ('azwan', 'nap'): [['azwan', 'azwAni', 'azwAni'], ['azwan', 'azwAni', 'azwAni'], ['azwaBiH', 'azwaByAm', 'azwaBiH'], ['azwaByaH', 'azwaByAm', 'azwaByaH'], ['azwaByaH', 'azwaByAm', 'azwaByaH'], ['azwasya', 'azwoH', 'azwAnAm'], ['azwasu', 'azwasu', 'azwasu']],

    ('navan', 'pum'): [['navaH', 'navO', 'navaH'], ['navam', 'navO', 'navAn'], ['navaBiH', 'navaByAm', 'navaBiH'], ['navaByaH', 'navaByAm', 'navaByaH'], ['navaByaH', 'navaByAm', 'navaByaH'], ['navasya', 'navoH', 'navAnAm'], ['navasu', 'navasu', 'navasu']],
    ('navan', 'stri'): [['navI', 'navo', 'navaH'], ['navIm', 'navo', 'navaH'], ['navIBiH', 'navIByAm', 'navIBiH'], ['navIByaH', 'navIByAm', 'navIByaH'], ['navIByaH', 'navIByAm', 'navIByaH'], ['navIByaH', 'navoH', 'navInAm'], ['navIsu', 'navoH', 'navIsu']],
    ('navan', 'nap'): [['nava', 'navAni', 'navAni'], ['nava', 'navAni', 'navAni'], ['navaBiH', 'navaByAm', 'navaBiH'], ['navaByaH', 'navaByAm', 'navaByaH'], ['navaByaH', 'navaByAm', 'navaByaH'], ['navasya', 'navoH', 'navAnAm'], ['navasu', 'navasu', 'navasu']],

    ('daSan', 'pum'): [['daSaH', 'daSO', 'daSaH'], ['daSam', 'daSO', 'daSAn'], ['daSaBiH', 'daSaByAm', 'daSaBiH'], ['daSaByaH', 'daSaByAm', 'daSaByaH'], ['daSaByaH', 'daSaByAm', 'daSaByaH'], ['daSasya', 'daSoH', 'daSAnAm'], ['daSasu', 'daSasu', 'daSasu']],
    ('daSan', 'stri'): [['daSI', 'daSo', 'daSaH'], ['daSIm', 'daSo', 'daSaH'], ['daSIBiH', 'daSIByAm', 'daSIBiH'], ['daSIByaH', 'daSIByAm', 'daSIByaH'], ['daSIByaH', 'daSIByAm', 'daSIByaH'], ['daSIByaH', 'daSoH', 'daSInAm'], ['daSIsu', 'daSoH', 'daSIsu']],
    ('daSan', 'nap'): [['daSa', 'daSAni', 'daSAni'], ['daSa', 'daSAni', 'daSAni'], ['daSaBiH', 'daSaByAm', 'daSaBiH'], ['daSaByaH', 'daSaByAm', 'daSaByaH'], ['daSaByaH', 'daSaByAm', 'daSaByaH'], ['daSasya', 'daSoH', 'daSAnAm'], ['daSasu', 'daSasu', 'daSasu']],
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
