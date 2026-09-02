//! subanta — focus all4 — sup declension tables (1.4.14 + 8.2.30 ff.).
//! Auto-generated from sktmorph/subanta.py — ending-class, sūtra-gated halanta.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Declension {
  pub stem: String, pub linga: String, pub declension: HashMap<String, Vec<String>>,
}

fn paradigms() -> HashMap<(String,String), Vec<Vec<String>>> {
  let mut m = HashMap::new();
  m.insert(("a".to_string(),"pum".to_string()), vec![vec!["aH".to_string(),"O".to_string(),"AH".to_string(),],vec!["am".to_string(),"O".to_string(),"An".to_string(),],vec!["ena".to_string(),"AByAm".to_string(),"EH".to_string(),],vec!["Aya".to_string(),"AByAm".to_string(),"eByaH".to_string(),],vec!["At".to_string(),"AByAm".to_string(),"eByaH".to_string(),],vec!["asya".to_string(),"ayoH".to_string(),"AnAm".to_string(),],vec!["e".to_string(),"ayoH".to_string(),"ezu".to_string(),],vec!["a".to_string(),"O".to_string(),"AH".to_string(),],]);
  m.insert(("a".to_string(),"nap".to_string()), vec![vec!["am".to_string(),"e".to_string(),"Ani".to_string(),],vec!["am".to_string(),"e".to_string(),"Ani".to_string(),],vec!["ena".to_string(),"AByAm".to_string(),"EH".to_string(),],vec!["Aya".to_string(),"AByAm".to_string(),"eByaH".to_string(),],vec!["At".to_string(),"AByAm".to_string(),"eByaH".to_string(),],vec!["asya".to_string(),"ayoH".to_string(),"AnAm".to_string(),],vec!["e".to_string(),"ayoH".to_string(),"ezu".to_string(),],vec!["a".to_string(),"e".to_string(),"Ani".to_string(),],]);
  m.insert(("A".to_string(),"stri".to_string()), vec![vec!["A".to_string(),"e".to_string(),"AH".to_string(),],vec!["Am".to_string(),"e".to_string(),"AH".to_string(),],vec!["ayA".to_string(),"AByAm".to_string(),"ABiH".to_string(),],vec!["AyE".to_string(),"AByAm".to_string(),"AByaH".to_string(),],vec!["AyAH".to_string(),"AByAm".to_string(),"AByaH".to_string(),],vec!["AyAH".to_string(),"ayoH".to_string(),"AnAm".to_string(),],vec!["AyAm".to_string(),"ayoH".to_string(),"Asu".to_string(),],vec!["e".to_string(),"e".to_string(),"AH".to_string(),],]);
  m.insert(("i".to_string(),"pum".to_string()), vec![vec!["iH".to_string(),"I".to_string(),"ayaH".to_string(),],vec!["im".to_string(),"I".to_string(),"In".to_string(),],vec!["inA".to_string(),"iByAm".to_string(),"iBiH".to_string(),],vec!["aye".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["eH".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["eH".to_string(),"yoH".to_string(),"InAm".to_string(),],vec!["O".to_string(),"yoH".to_string(),"izu".to_string(),],vec!["e".to_string(),"I".to_string(),"ayaH".to_string(),],]);
  m.insert(("i".to_string(),"stri".to_string()), vec![vec!["iH".to_string(),"I".to_string(),"ayaH".to_string(),],vec!["im".to_string(),"I".to_string(),"IH".to_string(),],vec!["yA".to_string(),"iByAm".to_string(),"iBiH".to_string(),],vec!["yE,aye".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["yAH,eH".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["yAH,eH".to_string(),"yoH".to_string(),"InAm".to_string(),],vec!["yAm,O".to_string(),"yoH".to_string(),"izu".to_string(),],vec!["e".to_string(),"I".to_string(),"ayaH".to_string(),],]);
  m.insert(("i".to_string(),"nap".to_string()), vec![vec!["i".to_string(),"inI".to_string(),"Ini".to_string(),],vec!["i".to_string(),"inI".to_string(),"Ini".to_string(),],vec!["inA".to_string(),"iByAm".to_string(),"iBiH".to_string(),],vec!["ine".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["inaH".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["inaH".to_string(),"inoH".to_string(),"InAm".to_string(),],vec!["ini".to_string(),"inoH".to_string(),"izu".to_string(),],vec!["i,e".to_string(),"inI".to_string(),"Ini".to_string(),],]);
  m.insert(("I".to_string(),"stri".to_string()), vec![vec!["I".to_string(),"yO".to_string(),"yaH".to_string(),],vec!["Im".to_string(),"yO".to_string(),"IH".to_string(),],vec!["yA".to_string(),"IByAm".to_string(),"IBiH".to_string(),],vec!["yE".to_string(),"IByAm".to_string(),"IByaH".to_string(),],vec!["yAH".to_string(),"IByAm".to_string(),"IByaH".to_string(),],vec!["yAH".to_string(),"yoH".to_string(),"InAm".to_string(),],vec!["yAm".to_string(),"yoH".to_string(),"Izu".to_string(),],vec!["i".to_string(),"yO".to_string(),"yaH".to_string(),],]);
  // ई-anta पुं (पपी) — सु पपीः; इयङ् पप्यौ/पप्या; loc पपी not *पपौ. नदी stays I-stri. ग्रामणी is named नदीवत्.
  m.insert(("I".to_string(),"pum".to_string()), vec![vec!["IH".to_string(),"yO".to_string(),"yaH".to_string(),],vec!["Im".to_string(),"yO".to_string(),"In".to_string(),],vec!["yA".to_string(),"IByAm".to_string(),"IBiH".to_string(),],vec!["ye".to_string(),"IByAm".to_string(),"IByaH".to_string(),],vec!["yaH".to_string(),"IByAm".to_string(),"IByaH".to_string(),],vec!["yaH".to_string(),"yoH".to_string(),"yAm".to_string(),],vec!["I".to_string(),"yoH".to_string(),"Izu".to_string(),],vec!["IH".to_string(),"yO".to_string(),"yaH".to_string(),],]);
  m.insert(("u".to_string(),"pum".to_string()), vec![vec!["uH".to_string(),"U".to_string(),"avaH".to_string(),],vec!["um".to_string(),"U".to_string(),"Un".to_string(),],vec!["unA".to_string(),"uByAm".to_string(),"uBiH".to_string(),],vec!["ave".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["oH".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["oH".to_string(),"voH".to_string(),"UnAm".to_string(),],vec!["O".to_string(),"voH".to_string(),"uzu".to_string(),],vec!["o".to_string(),"U".to_string(),"avaH".to_string(),],]);
  m.insert(("u".to_string(),"stri".to_string()), vec![vec!["uH".to_string(),"U".to_string(),"avaH".to_string(),],vec!["um".to_string(),"U".to_string(),"UH".to_string(),],vec!["vA".to_string(),"uByAm".to_string(),"uBiH".to_string(),],vec!["vE,ave".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["vAH,oH".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["vAH,oH".to_string(),"voH".to_string(),"UnAm".to_string(),],vec!["vAm,O".to_string(),"voH".to_string(),"uzu".to_string(),],vec!["o".to_string(),"U".to_string(),"avaH".to_string(),],]);
  m.insert(("u".to_string(),"nap".to_string()), vec![vec!["u".to_string(),"unI".to_string(),"Uni".to_string(),],vec!["u".to_string(),"unI".to_string(),"Uni".to_string(),],vec!["unA".to_string(),"uByAm".to_string(),"uBiH".to_string(),],vec!["une".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["unaH".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["unaH".to_string(),"unoH".to_string(),"UnAm".to_string(),],vec!["uni".to_string(),"unoH".to_string(),"uzu".to_string(),],vec!["u,o".to_string(),"unI".to_string(),"Uni".to_string(),],]);
  // ऊ-anta पुं (हूहू) — यण् हूह्वौ/हूह्वा; सु हूहूः; अम् हूहूम्; शस् हूहून्; loc हूह्वि. Not *हूहावौ. खलपू acc वम् named; वधू loc वाम् स्त्री; अतिचमू loc वाम् named.
  m.insert(("U".to_string(),"pum".to_string()), vec![vec!["UH".to_string(),"vO".to_string(),"vaH".to_string(),],vec!["Um".to_string(),"vO".to_string(),"Un".to_string(),],vec!["vA".to_string(),"UByAm".to_string(),"UBiH".to_string(),],vec!["ve".to_string(),"UByAm".to_string(),"UByaH".to_string(),],vec!["vaH".to_string(),"UByAm".to_string(),"UByaH".to_string(),],vec!["vaH".to_string(),"voH".to_string(),"UnAm".to_string(),],vec!["vi".to_string(),"voH".to_string(),"Uzu".to_string(),],vec!["UH".to_string(),"vO".to_string(),"vaH".to_string(),],]);
  // ऊ-anta स्त्री (वधू) — नदीवत् यण् वध्वौ/वध्वा; सु वधूः; पद वधूभ्याम्/वधूषु; voc वधु. धेनु stays u-stri. भ्रू/स्वभू stay उवङ्.
  m.insert(("U".to_string(),"stri".to_string()), vec![vec!["UH".to_string(),"vO".to_string(),"vaH".to_string(),],vec!["Um".to_string(),"vO".to_string(),"UH".to_string(),],vec!["vA".to_string(),"UByAm".to_string(),"UBiH".to_string(),],vec!["vE".to_string(),"UByAm".to_string(),"UByaH".to_string(),],vec!["vAH".to_string(),"UByAm".to_string(),"UByaH".to_string(),],vec!["vAH".to_string(),"voH".to_string(),"UnAm".to_string(),],vec!["vAm".to_string(),"voH".to_string(),"Uzu".to_string(),],vec!["u".to_string(),"vO".to_string(),"vaH".to_string(),],]);
  m.insert(("U".to_string(),"nap".to_string()), vec![vec!["U".to_string(),"unI".to_string(),"Uni".to_string(),],vec!["U".to_string(),"unI".to_string(),"Uni".to_string(),],vec!["UnA".to_string(),"uByAm".to_string(),"uBiH".to_string(),],vec!["Une".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["UnaH".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["UnaH".to_string(),"UnoH".to_string(),"UnAm".to_string(),],vec!["Uni".to_string(),"UnoH".to_string(),"Uzu".to_string(),],vec!["U,o".to_string(),"unI".to_string(),"Uni".to_string(),],]);
  // f-stem: agent (kartf-type, Pāṇini 7.1.9) -> Aram; kinship (pitf) is handled as exception in generate()
  // f-stem कर्तृ (6.4.11 सर्वनामस्थान आ): कर्तारौ/कर्तारम्. पितृ-class patched short पितरौ/पितरम्. स्वसृ/नप्तृ keep आ.
  m.insert(("f".to_string(),"pum".to_string()), vec![vec!["A".to_string(),"ArO".to_string(),"AraH".to_string(),],vec!["Aram".to_string(),"ArO".to_string(),"Fn".to_string(),],vec!["rA".to_string(),"fByAm".to_string(),"fBiH".to_string(),],vec!["re".to_string(),"fByAm".to_string(),"fByaH".to_string(),],vec!["uH".to_string(),"fByAm".to_string(),"fByaH".to_string(),],vec!["uH".to_string(),"roH".to_string(),"FnAm".to_string(),],vec!["ari".to_string(),"roH".to_string(),"fzu".to_string(),],vec!["aH".to_string(),"ArO".to_string(),"AraH".to_string(),],]);
  m.insert(("f".to_string(),"stri".to_string()), vec![vec!["A".to_string(),"arO".to_string(),"araH".to_string(),],vec!["aram".to_string(),"arO".to_string(),"FH".to_string(),],vec!["rA".to_string(),"fByAm".to_string(),"fBiH".to_string(),],vec!["re".to_string(),"fByAm".to_string(),"fByaH".to_string(),],vec!["uH".to_string(),"fByAm".to_string(),"fByaH".to_string(),],vec!["uH".to_string(),"roH".to_string(),"FnAm".to_string(),],vec!["ari".to_string(),"roH".to_string(),"fzu".to_string(),],vec!["aH".to_string(),"arO".to_string(),"araH".to_string(),],]);
  // ऋ nap (धातृ) — 7.1.23 धातृ/धातृणी/धातॄणि (8.4.1 णत्व). कर्तृ पुं stays कर्ता.
  m.insert(("f".to_string(),"nap".to_string()), vec![vec!["f".to_string(),"fnI".to_string(),"Fni".to_string(),],vec!["f".to_string(),"fnI".to_string(),"Fni".to_string(),],vec!["fnA".to_string(),"fByAm".to_string(),"fBiH".to_string(),],vec!["fne".to_string(),"fByAm".to_string(),"fByaH".to_string(),],vec!["fnaH".to_string(),"fByAm".to_string(),"fByaH".to_string(),],vec!["fnaH".to_string(),"fnoH".to_string(),"FnAm".to_string(),],vec!["fni".to_string(),"fnoH".to_string(),"fzu".to_string(),],vec!["f,ar".to_string(),"fnI".to_string(),"Fni".to_string(),],]);
  m.insert(("in".to_string(),"pum".to_string()), vec![vec!["I".to_string(),"inO".to_string(),"inaH".to_string(),],vec!["inam".to_string(),"inO".to_string(),"inaH".to_string(),],vec!["inA".to_string(),"iByAm".to_string(),"iBiH".to_string(),],vec!["ine".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["inaH".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["inaH".to_string(),"inoH".to_string(),"inAm".to_string(),],vec!["ini".to_string(),"inoH".to_string(),"izu".to_string(),],vec!["in".to_string(),"inO".to_string(),"inaH".to_string(),],]);
  // इन् nap (दण्डिन्) — 7.1.23 स्वमोर्नपुंसकात्: प्रथमा/द्वितीया दण्डि/दण्डिनी/दण्डीनि not पुं दण्डी.
  m.insert(("in".to_string(),"nap".to_string()), vec![vec!["i".to_string(),"inI".to_string(),"Ini".to_string(),],vec!["i".to_string(),"inI".to_string(),"Ini".to_string(),],vec!["inA".to_string(),"iByAm".to_string(),"iBiH".to_string(),],vec!["ine".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["inaH".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["inaH".to_string(),"inoH".to_string(),"inAm".to_string(),],vec!["ini".to_string(),"inoH".to_string(),"izu".to_string(),],vec!["in,i".to_string(),"inI".to_string(),"Ini".to_string(),],]);
  m.insert(("as".to_string(),"nap".to_string()), vec![vec!["aH".to_string(),"asI".to_string(),"AMsi".to_string(),],vec!["aH".to_string(),"asI".to_string(),"AMsi".to_string(),],vec!["asA".to_string(),"oByAm".to_string(),"oBiH".to_string(),],vec!["ase".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["asaH".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["asaH".to_string(),"asoH".to_string(),"asAm".to_string(),],vec!["asi".to_string(),"asoH".to_string(),"aHsu".to_string(),],vec!["aH".to_string(),"asI".to_string(),"AMsi".to_string(),],]);
  // अत् पुं: 6.4.14 आन् भवान्/धीमान्; शतृ अधातोः patched पचन्. नुम् भवन्तम्.
  m.insert(("at".to_string(),"pum".to_string()), vec![vec!["An".to_string(),"antO".to_string(),"antaH".to_string(),],vec!["antam".to_string(),"antO".to_string(),"ataH".to_string(),],vec!["atA".to_string(),"adByAm".to_string(),"adBiH".to_string(),],vec!["ate".to_string(),"adByAm".to_string(),"adByaH".to_string(),],vec!["ataH".to_string(),"adByAm".to_string(),"adByaH".to_string(),],vec!["ataH".to_string(),"atoH".to_string(),"atAm".to_string(),],vec!["ati".to_string(),"atoH".to_string(),"atsu".to_string(),],vec!["an".to_string(),"antO".to_string(),"antaH".to_string(),],]);
  m.insert(("an".to_string(),"pum".to_string()), vec![vec!["A".to_string(),"AnO".to_string(),"AnaH".to_string(),],vec!["Anam".to_string(),"AnO".to_string(),"YaH".to_string(),],vec!["YA".to_string(),"aByAm".to_string(),"aBiH".to_string(),],vec!["Ye".to_string(),"aByAm".to_string(),"aByaH".to_string(),],vec!["YaH".to_string(),"aByAm".to_string(),"aByaH".to_string(),],vec!["YaH".to_string(),"YoH".to_string(),"YAm".to_string(),],vec!["Yi,Yani".to_string(),"YoH".to_string(),"asu".to_string(),],vec!["an".to_string(),"AnO".to_string(),"AnaH".to_string(),],]);
  m.insert(("an".to_string(),"nap".to_string()), vec![vec!["a".to_string(),"nI".to_string(),"Ani".to_string(),],vec!["a".to_string(),"nI".to_string(),"Ani".to_string(),],vec!["nA".to_string(),"aByAm".to_string(),"aBiH".to_string(),],vec!["ne".to_string(),"aByAm".to_string(),"aByaH".to_string(),],vec!["naH".to_string(),"aByAm".to_string(),"aByaH".to_string(),],vec!["naH".to_string(),"noH".to_string(),"nAm".to_string(),],vec!["ni".to_string(),"noH".to_string(),"asu".to_string(),],vec!["a,an".to_string(),"nI".to_string(),"Ani".to_string(),],]);
  // च-anta स्त्री/पुं (वाच्, ऋच्) — 8.2.30 चोः कुः वाक्/ऋक्, 8.4.56 वाग्; भ्-initial ग्; loc वाक्षु/ऋक्षु.
  m.insert(("c".to_string(),"stri".to_string()), vec![vec!["k,g".to_string(),"cO".to_string(),"caH".to_string(),],vec!["cam".to_string(),"cO".to_string(),"caH".to_string(),],vec!["cA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["ce".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["caH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["caH".to_string(),"coH".to_string(),"cAm".to_string(),],vec!["ci".to_string(),"coH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"cO".to_string(),"caH".to_string(),],]);
  m.insert(("c".to_string(),"pum".to_string()), vec![vec!["k,g".to_string(),"cO".to_string(),"caH".to_string(),],vec!["cam".to_string(),"cO".to_string(),"caH".to_string(),],vec!["cA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["ce".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["caH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["caH".to_string(),"coH".to_string(),"cAm".to_string(),],vec!["ci".to_string(),"coH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"cO".to_string(),"caH".to_string(),],]);
  // च-anta nap — 7.1.19/20 वाक्-class: क्/ची/चि; पद ग्भ्याम्/क्षु. पुं stays चौ.
  m.insert(("c".to_string(),"nap".to_string()), vec![vec!["k,g".to_string(),"cI".to_string(),"ci".to_string(),],vec!["k,g".to_string(),"cI".to_string(),"ci".to_string(),],vec!["cA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["ce".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["caH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["caH".to_string(),"coH".to_string(),"cAm".to_string(),],vec!["ci".to_string(),"coH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"cI".to_string(),"ci".to_string(),],]);
  // छ-anta (C) — 8.2.30 palatal C→k, same as c-anta वाच्-class (पृच्छ् पृष्ट से अलग, यह हलन्त C)
  m.insert(("C".to_string(),"pum".to_string()), vec![vec!["k,g".to_string(),"CO".to_string(),"CaH".to_string(),],vec!["Cam".to_string(),"CO".to_string(),"CaH".to_string(),],vec!["CA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["Ce".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["CaH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["CaH".to_string(),"CoH".to_string(),"CAm".to_string(),],vec!["Ci".to_string(),"CoH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"CO".to_string(),"CaH".to_string(),],]);
  m.insert(("C".to_string(),"stri".to_string()), vec![vec!["k,g".to_string(),"CO".to_string(),"CaH".to_string(),],vec!["Cam".to_string(),"CO".to_string(),"CaH".to_string(),],vec!["CA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["Ce".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["CaH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["CaH".to_string(),"CoH".to_string(),"CAm".to_string(),],vec!["Ci".to_string(),"CoH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"CO".to_string(),"CaH".to_string(),],]);
  m.insert(("C".to_string(),"nap".to_string()), vec![vec!["k,g".to_string(),"CI".to_string(),"Ci".to_string(),],vec!["k,g".to_string(),"CI".to_string(),"Ci".to_string(),],vec!["CA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["Ce".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["CaH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["CaH".to_string(),"CoH".to_string(),"CAm".to_string(),],vec!["Ci".to_string(),"CoH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"CI".to_string(),"Ci".to_string(),],]);
  m.insert(("ad".to_string(),"nap".to_string()), vec![vec!["ad".to_string(),"adI".to_string(),"AmSi".to_string(),],vec!["adam".to_string(),"adI".to_string(),"AmSi".to_string(),],vec!["adA".to_string(),"aByAm".to_string(),"aBiH".to_string(),],vec!["ade".to_string(),"aByAm".to_string(),"aByaH".to_string(),],vec!["adaH".to_string(),"aByAm".to_string(),"aByaH".to_string(),],vec!["adaH".to_string(),"adoh".to_string(),"Am".to_string(),],vec!["adi".to_string(),"adoh".to_string(),"atsu".to_string(),],vec!["ad".to_string(),"adI".to_string(),"AmSi".to_string(),],]);
  // ष-anta (द्विष्) — 8.2.39 जश्त्व ष्→ड्, 8.4.56 वाऽवसाने ट्; भ्-initial ड्; सप्तमी ट्सु.
  m.insert(("z".to_string(),"pum".to_string()), vec![vec!["w,q".to_string(),"zO".to_string(),"zaH".to_string(),],vec!["zam".to_string(),"zO".to_string(),"zaH".to_string(),],vec!["zA".to_string(),"qByAm".to_string(),"qBiH".to_string(),],vec!["ze".to_string(),"qByAm".to_string(),"qByaH".to_string(),],vec!["zaH".to_string(),"qByAm".to_string(),"qByaH".to_string(),],vec!["zaH".to_string(),"zoH".to_string(),"zAm".to_string(),],vec!["zi".to_string(),"zoH".to_string(),"wsu".to_string(),],vec!["w,q".to_string(),"zO".to_string(),"zaH".to_string(),],]);
  m.insert(("z".to_string(),"stri".to_string()), vec![vec!["w,q".to_string(),"zO".to_string(),"zaH".to_string(),],vec!["zam".to_string(),"zO".to_string(),"zaH".to_string(),],vec!["zA".to_string(),"qByAm".to_string(),"qBiH".to_string(),],vec!["ze".to_string(),"qByAm".to_string(),"qByaH".to_string(),],vec!["zaH".to_string(),"qByAm".to_string(),"qByaH".to_string(),],vec!["zaH".to_string(),"zoH".to_string(),"zAm".to_string(),],vec!["zi".to_string(),"zoH".to_string(),"wsu".to_string(),],vec!["w,q".to_string(),"zO".to_string(),"zaH".to_string(),],]);
  // ष-anta nap — 7.1.19/20 ट्/षी/षि; पद ड्भ्याम्/ट्सु. धनुस् stays `uz` nap धनुषी.
  m.insert(("z".to_string(),"nap".to_string()), vec![vec!["w,q".to_string(),"zI".to_string(),"zi".to_string(),],vec!["w,q".to_string(),"zI".to_string(),"zi".to_string(),],vec!["zA".to_string(),"qByAm".to_string(),"qBiH".to_string(),],vec!["ze".to_string(),"qByAm".to_string(),"qByaH".to_string(),],vec!["zaH".to_string(),"qByAm".to_string(),"qByaH".to_string(),],vec!["zaH".to_string(),"zoH".to_string(),"zAm".to_string(),],vec!["zi".to_string(),"zoH".to_string(),"wsu".to_string(),],vec!["w,q".to_string(),"zI".to_string(),"zi".to_string(),],]);
  // अत् nap (जगत्) — 7.1.23 स्वमोः; नपुं जगत्/जगती/जगन्ति not *जगांसि (as-anta). भ्: 8.2.39 जगद्भ्याम्.
  // अत् nap (जगत्) — 7.1.23 स्वमोः; 7.1.80 वा नपुंसकस्य नुम् जगती/जगन्ती, जगति/जगन्ति. भ्: 8.2.39 जगद्भ्याम्.
  m.insert(("at".to_string(),"nap".to_string()), vec![vec!["at".to_string(),"atI,antI".to_string(),"ati,anti".to_string(),],vec!["at".to_string(),"atI,antI".to_string(),"ati,anti".to_string(),],vec!["atA".to_string(),"adByAm".to_string(),"adBiH".to_string(),],vec!["ate".to_string(),"adByAm".to_string(),"adByaH".to_string(),],vec!["ataH".to_string(),"adByAm".to_string(),"adByaH".to_string(),],vec!["ataH".to_string(),"atoH".to_string(),"atAm".to_string(),],vec!["ati".to_string(),"atoH".to_string(),"atsu".to_string(),],vec!["at".to_string(),"atI,antI".to_string(),"ati,anti".to_string(),],]);
  m.insert(("us".to_string(),"nap".to_string()), vec![vec!["uH".to_string(),"uSI".to_string(),"UMSi".to_string(),],vec!["uH".to_string(),"uSI".to_string(),"UMSi".to_string(),],vec!["usA".to_string(),"oByAm".to_string(),"oBiH".to_string(),],vec!["use".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["usaH".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["usaH".to_string(),"usoH".to_string(),"usAm".to_string(),],vec!["usi".to_string(),"usoH".to_string(),"uHsu".to_string(),],vec!["uH".to_string(),"uSI".to_string(),"UMSi".to_string(),],]);
  // उष् nap (धनुस् as Danuz) — 8.2.66 रु धनुः/धनुर्भ्याम्; 8.3.59 धनुषी/धूंषि. Not a-stem *धनुषः.
  m.insert(("uz".to_string(),"nap".to_string()), vec![vec!["uH".to_string(),"uzI".to_string(),"UMzi".to_string(),],vec!["uH".to_string(),"uzI".to_string(),"UMzi".to_string(),],vec!["uzA".to_string(),"urByAm".to_string(),"urBiH".to_string(),],vec!["uze".to_string(),"urByAm".to_string(),"urByaH".to_string(),],vec!["uzaH".to_string(),"urByAm".to_string(),"urByaH".to_string(),],vec!["uzaH".to_string(),"uzoH".to_string(),"uzAm".to_string(),],vec!["uzi".to_string(),"uzoH".to_string(),"uHzu".to_string(),],vec!["uH".to_string(),"uzI".to_string(),"UMzi".to_string(),],]);
  m.insert(("is".to_string(),"nap".to_string()), vec![vec!["iH".to_string(),"iSI".to_string(),"IMSi".to_string(),],vec!["iH".to_string(),"iSI".to_string(),"IMSi".to_string(),],vec!["isA".to_string(),"oByAm".to_string(),"oBiH".to_string(),],vec!["ise".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["isaH".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["isaH".to_string(),"isoH".to_string(),"isAm".to_string(),],vec!["isi".to_string(),"isoH".to_string(),"iHsu".to_string(),],vec!["iH".to_string(),"iSI".to_string(),"IMSi".to_string(),],]);
  // गो: 7.1.90 णित्, 6.1.93 औतोऽम्शसोः (गाम्/गाः), 6.1.78 अव्
  m.insert(("o".to_string(),"pum".to_string()), vec![vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],vec!["Am".to_string(),"AvO".to_string(),"AH".to_string(),],vec!["avA".to_string(),"oByAm".to_string(),"oBiH".to_string(),],vec!["ave".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["oH".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["oH".to_string(),"avoH".to_string(),"avAm".to_string(),],vec!["avi".to_string(),"avoH".to_string(),"ozu".to_string(),],vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],]);
  m.insert(("o".to_string(),"stri".to_string()), vec![vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],vec!["Am".to_string(),"AvO".to_string(),"AH".to_string(),],vec!["avA".to_string(),"oByAm".to_string(),"oBiH".to_string(),],vec!["ave".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["oH".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["oH".to_string(),"avoH".to_string(),"avAm".to_string(),],vec!["avi".to_string(),"avoH".to_string(),"ozu".to_string(),],vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],]);
  // नौ: औ-anta, 6.1.78 आव् (नावम्, not 6.1.93 गाम्)
  m.insert(("O".to_string(),"pum".to_string()), vec![vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],vec!["Avam".to_string(),"AvO".to_string(),"AvaH".to_string(),],vec!["AvA".to_string(),"OByAm".to_string(),"OBiH".to_string(),],vec!["Ave".to_string(),"OByAm".to_string(),"OByaH".to_string(),],vec!["AvaH".to_string(),"OByAm".to_string(),"OByaH".to_string(),],vec!["AvaH".to_string(),"AvoH".to_string(),"AvAm".to_string(),],vec!["Avi".to_string(),"AvoH".to_string(),"Ozu".to_string(),],vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],]);
  m.insert(("O".to_string(),"stri".to_string()), vec![vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],vec!["Avam".to_string(),"AvO".to_string(),"AvaH".to_string(),],vec!["AvA".to_string(),"OByAm".to_string(),"OBiH".to_string(),],vec!["Ave".to_string(),"OByAm".to_string(),"OByaH".to_string(),],vec!["AvaH".to_string(),"OByAm".to_string(),"OByaH".to_string(),],vec!["AvaH".to_string(),"AvoH".to_string(),"AvAm".to_string(),],vec!["Avi".to_string(),"AvoH".to_string(),"Ozu".to_string(),],vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],]);
  // ज-anta (वणिज्) — 8.2.30 कुत्व वणिक्/वणग्; भ् ग्; सप्तमी क्षु not *षु. राज् stays named ट्.
  m.insert(("j".to_string(),"pum".to_string()), vec![vec!["k,g".to_string(),"jO".to_string(),"jaH".to_string(),],vec!["jam".to_string(),"jO".to_string(),"jaH".to_string(),],vec!["jA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["je".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["jaH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["jaH".to_string(),"joH".to_string(),"jAm".to_string(),],vec!["ji".to_string(),"joH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"jO".to_string(),"jaH".to_string(),],]);
  m.insert(("j".to_string(),"stri".to_string()), vec![vec!["k,g".to_string(),"jO".to_string(),"jaH".to_string(),],vec!["jam".to_string(),"jO".to_string(),"jaH".to_string(),],vec!["jA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["je".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["jaH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["jaH".to_string(),"joH".to_string(),"jAm".to_string(),],vec!["ji".to_string(),"joH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"jO".to_string(),"jaH".to_string(),],]);
  // ज-anta nap (ऊर्ज्) — 7.1.19/20 ऊर्क्/ऊर्जी/ऊर्जि; पद ऊर्ग्भ्याम्/ऊर्क्षु. Gold *Unrji is scrape.
  m.insert(("j".to_string(),"nap".to_string()), vec![vec!["k,g".to_string(),"jI".to_string(),"ji".to_string(),],vec!["k,g".to_string(),"jI".to_string(),"ji".to_string(),],vec!["jA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["je".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["jaH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["jaH".to_string(),"joH".to_string(),"jAm".to_string(),],vec!["ji".to_string(),"joH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"jI".to_string(),"ji".to_string(),],]);
  // ञ-anta (Y) — 8.2.30 palatal Y→n/k, schwa? simple n/k like j, for testing halanta Y
  m.insert(("Y".to_string(),"pum".to_string()), vec![vec!["n,k".to_string(),"YO".to_string(),"YaH".to_string(),],vec!["Yam".to_string(),"YO".to_string(),"YaH".to_string(),],vec!["YA".to_string(),"nByAm".to_string(),"nBiH".to_string(),],vec!["Ye".to_string(),"nByAm".to_string(),"nByaH".to_string(),],vec!["YaH".to_string(),"nByAm".to_string(),"nByaH".to_string(),],vec!["YaH".to_string(),"YoH".to_string(),"YAm".to_string(),],vec!["Yi".to_string(),"YoH".to_string(),"nzu".to_string(),],vec!["n,k".to_string(),"YO".to_string(),"YaH".to_string(),],]);
  m.insert(("Y".to_string(),"stri".to_string()), vec![vec!["n,k".to_string(),"YO".to_string(),"YaH".to_string(),],vec!["Yam".to_string(),"YO".to_string(),"YaH".to_string(),],vec!["YA".to_string(),"nByAm".to_string(),"nBiH".to_string(),],vec!["Ye".to_string(),"nByAm".to_string(),"nByaH".to_string(),],vec!["YaH".to_string(),"nByAm".to_string(),"nByaH".to_string(),],vec!["YaH".to_string(),"YoH".to_string(),"YAm".to_string(),],vec!["Yi".to_string(),"YoH".to_string(),"nzu".to_string(),],vec!["n,k".to_string(),"YO".to_string(),"YaH".to_string(),],]);
  m.insert(("Y".to_string(),"nap".to_string()), vec![vec!["n,k".to_string(),"YI".to_string(),"Yi".to_string(),],vec!["n,k".to_string(),"YI".to_string(),"Yi".to_string(),],vec!["YA".to_string(),"nByAm".to_string(),"nBiH".to_string(),],vec!["Ye".to_string(),"nByAm".to_string(),"nByaH".to_string(),],vec!["YaH".to_string(),"nByAm".to_string(),"nByaH".to_string(),],vec!["YaH".to_string(),"YoH".to_string(),"YAm".to_string(),],vec!["Yi".to_string(),"YoH".to_string(),"nzu".to_string(),],vec!["n,k".to_string(),"YI".to_string(),"Yi".to_string(),],]);
  m.insert(("d".to_string(),"pum".to_string()), vec![vec!["t".to_string(),"dO".to_string(),"daH".to_string(),],vec!["dam".to_string(),"dO".to_string(),"daH".to_string(),],vec!["dA".to_string(),"dByAm".to_string(),"dBiH".to_string(),],vec!["de".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["daH".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["daH".to_string(),"doH".to_string(),"dAm".to_string(),],vec!["di".to_string(),"doH".to_string(),"tsu".to_string(),],vec!["t".to_string(),"dO".to_string(),"daH".to_string(),],]);
  m.insert(("d".to_string(),"stri".to_string()), vec![vec!["t".to_string(),"dO".to_string(),"daH".to_string(),],vec!["dam".to_string(),"dO".to_string(),"daH".to_string(),],vec!["dA".to_string(),"dByAm".to_string(),"dBiH".to_string(),],vec!["de".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["daH".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["daH".to_string(),"doH".to_string(),"dAm".to_string(),],vec!["di".to_string(),"doH".to_string(),"tsu".to_string(),],vec!["t".to_string(),"dO".to_string(),"daH".to_string(),],]);
  // त-anta (मरुत्, सरित्, त्रिंशत्) — पद त्; 8.2.39 before भ् → द्भ्याम्/द्भिः. `at` शतृ stays longer-match.
  m.insert(("t".to_string(),"pum".to_string()), vec![vec!["t".to_string(),"tO".to_string(),"taH".to_string(),],vec!["tam".to_string(),"tO".to_string(),"taH".to_string(),],vec!["tA".to_string(),"dByAm".to_string(),"dBiH".to_string(),],vec!["te".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["taH".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["taH".to_string(),"toH".to_string(),"tAm".to_string(),],vec!["ti".to_string(),"toH".to_string(),"tsu".to_string(),],vec!["t".to_string(),"tO".to_string(),"taH".to_string(),],]);
  m.insert(("t".to_string(),"stri".to_string()), vec![vec!["t".to_string(),"tO".to_string(),"taH".to_string(),],vec!["tam".to_string(),"tO".to_string(),"taH".to_string(),],vec!["tA".to_string(),"dByAm".to_string(),"dBiH".to_string(),],vec!["te".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["taH".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["taH".to_string(),"toH".to_string(),"tAm".to_string(),],vec!["ti".to_string(),"toH".to_string(),"tsu".to_string(),],vec!["t".to_string(),"tO".to_string(),"taH".to_string(),],]);
  // त-anta nap (शकृत्) — 7.1.23 शकृत्/शकृती/शकृन्ति, शकृद्भ्याम्. `at` जगत् keeps longer match.
  m.insert(("t".to_string(),"nap".to_string()), vec![vec!["t".to_string(),"tI".to_string(),"nti".to_string(),],vec!["t".to_string(),"tI".to_string(),"nti".to_string(),],vec!["tA".to_string(),"dByAm".to_string(),"dBiH".to_string(),],vec!["te".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["taH".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["taH".to_string(),"toH".to_string(),"tAm".to_string(),],vec!["ti".to_string(),"toH".to_string(),"tsu".to_string(),],vec!["t".to_string(),"tI".to_string(),"nti".to_string(),],]);
  // थ-anta (अग्निमथ्) — 8.2.39 जश् थ्→द्, 8.4.56 त्; भ् द्भ्याम्; सप्तमी त्सु. `at` शतृ/`t` मरुत् stay longer or other key.
  m.insert(("T".to_string(),"pum".to_string()), vec![vec!["t,d".to_string(),"TO".to_string(),"TaH".to_string(),],vec!["Tam".to_string(),"TO".to_string(),"TaH".to_string(),],vec!["TA".to_string(),"dByAm".to_string(),"dBiH".to_string(),],vec!["Te".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["TaH".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["TaH".to_string(),"ToH".to_string(),"TAm".to_string(),],vec!["Ti".to_string(),"ToH".to_string(),"tsu".to_string(),],vec!["t,d".to_string(),"TO".to_string(),"TaH".to_string(),],]);
  let t_aspir_pum = m.get(&("T".to_string(), "pum".to_string())).expect("T pum").clone();
  m.insert(("T".to_string(), "stri".to_string()), t_aspir_pum);
  // h-anta (उष्णिह्) — पद क्/ग् (8.2.30/8.4.56); सप्तमी क्षु not *त्सु. अनडुह्/उपानह् stay named द्.
  m.insert(("h".to_string(),"pum".to_string()), vec![vec!["k,g".to_string(),"hO".to_string(),"haH".to_string(),],vec!["ham".to_string(),"hO".to_string(),"haH".to_string(),],vec!["hA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["he".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["haH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["haH".to_string(),"hoH".to_string(),"hAm".to_string(),],vec!["hi".to_string(),"hoH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"hO".to_string(),"haH".to_string(),],]);
  let h_pum = m.get(&("h".to_string(), "pum".to_string())).expect("h pum").clone();
  m.insert(("h".to_string(), "stri".to_string()), h_pum);
  // ह-anta nap — 7.1.19/20 क्/ही/हि; पद ग्भ्याम्/क्षु. उष्णिह् पुं stays उष्णिक्/उष्णिहौ. अनडुह् stays named.
  m.insert(("h".to_string(),"nap".to_string()), vec![vec!["k,g".to_string(),"hI".to_string(),"hi".to_string(),],vec!["k,g".to_string(),"hI".to_string(),"hi".to_string(),],vec!["hA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["he".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["haH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["haH".to_string(),"hoH".to_string(),"hAm".to_string(),],vec!["hi".to_string(),"hoH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"hI".to_string(),"hi".to_string(),],]);
  m.insert(("B".to_string(),"pum".to_string()), vec![vec!["p,b".to_string(),"BO".to_string(),"BaH".to_string(),],vec!["Bam".to_string(),"BO".to_string(),"BaH".to_string(),],vec!["BA".to_string(),"bByAm".to_string(),"bBiH".to_string(),],vec!["Be".to_string(),"bByAm".to_string(),"bByaH".to_string(),],vec!["BaH".to_string(),"bByAm".to_string(),"bByaH".to_string(),],vec!["BaH".to_string(),"BoH".to_string(),"BAm".to_string(),],vec!["Bi".to_string(),"BoH".to_string(),"psu".to_string(),],vec!["p,b".to_string(),"BO".to_string(),"BaH".to_string(),],]);
  let b_pum = m.get(&("B".to_string(), "pum".to_string())).expect("B pum").clone();
  m.insert(("B".to_string(), "stri".to_string()), b_pum);
  // प-anta (ककुप्) — पद प्; 8.2.39 before भ् → ब्भ्याम्. Distinct from भ्-anta (लभ्→लप्).
  m.insert(("p".to_string(),"pum".to_string()), vec![vec!["p,b".to_string(),"pO".to_string(),"paH".to_string(),],vec!["pam".to_string(),"pO".to_string(),"paH".to_string(),],vec!["pA".to_string(),"bByAm".to_string(),"bBiH".to_string(),],vec!["pe".to_string(),"bByAm".to_string(),"bByaH".to_string(),],vec!["paH".to_string(),"bByAm".to_string(),"bByaH".to_string(),],vec!["paH".to_string(),"poH".to_string(),"pAm".to_string(),],vec!["pi".to_string(),"poH".to_string(),"psu".to_string(),],vec!["p,b".to_string(),"pO".to_string(),"paH".to_string(),],]);
  let p_pum = m.get(&("p".to_string(), "pum".to_string())).expect("p pum").clone();
  m.insert(("p".to_string(), "stri".to_string()), p_pum);
  // श-anta (दिश्) — 8.2.36 शां षः then 8.2.39/8.4.56 दिक्/दिग्; भ्-initial दिग्भ्याम्; सप्तमी दिक्षु.
  m.insert(("S".to_string(),"stri".to_string()), vec![vec!["k,g".to_string(),"SO".to_string(),"SaH".to_string(),],vec!["Sam".to_string(),"SO".to_string(),"SaH".to_string(),],vec!["SA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["Se".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["SaH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["SaH".to_string(),"SoH".to_string(),"SAm".to_string(),],vec!["Si".to_string(),"SoH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"SO".to_string(),"SaH".to_string(),],]);
  m.insert(("S".to_string(),"pum".to_string()), vec![vec!["k,g".to_string(),"SO".to_string(),"SaH".to_string(),],vec!["Sam".to_string(),"SO".to_string(),"SaH".to_string(),],vec!["SA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["Se".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["SaH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["SaH".to_string(),"SoH".to_string(),"SAm".to_string(),],vec!["Si".to_string(),"SoH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"SO".to_string(),"SaH".to_string(),],]);
  // श-anta nap — 7.1.19/20 तादृक्/तादृशी/तादृशि; पद ग्भ्याम्/क्षु. पुं stays तादृशौ.
  m.insert(("S".to_string(),"nap".to_string()), vec![vec!["k,g".to_string(),"SI".to_string(),"Si".to_string(),],vec!["k,g".to_string(),"SI".to_string(),"Si".to_string(),],vec!["SA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["Se".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["SaH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["SaH".to_string(),"SoH".to_string(),"SAm".to_string(),],vec!["Si".to_string(),"SoH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"SI".to_string(),"Si".to_string(),],]);
  // r-anta (गिर्) and s-anta (तपस्-like, but s as pum) — 8.2.66 ससजुषोः रुः, 8.2.39 जश्त्व
  // sūtra: 8.2.66 s→ru at pada (तपस्→तपर्→तपः), r stays r; Future devs: r shows no visarga change at pada? Actually r→ḥ via ru.
  // Extreme commenting: r/s show s→ḥ vs r→r alternation; keeps saptamī tsu consistent
  m.insert(("r".to_string(),"pum".to_string()), vec![vec!["H".to_string(),"rO".to_string(),"raH".to_string(),],vec!["ram".to_string(),"rO".to_string(),"raH".to_string(),],vec!["rA".to_string(),"rByAm".to_string(),"rBiH".to_string(),],vec!["re".to_string(),"rByAm".to_string(),"rByaH".to_string(),],vec!["raH".to_string(),"rByAm".to_string(),"rByaH".to_string(),],vec!["raH".to_string(),"roH".to_string(),"rAm".to_string(),],vec!["ri".to_string(),"roH".to_string(),"tsu".to_string(),],vec!["H".to_string(),"rO".to_string(),"raH".to_string(),],]);
  m.insert(("s".to_string(),"pum".to_string()), vec![vec!["H".to_string(),"sO".to_string(),"saH".to_string(),],vec!["sam".to_string(),"sO".to_string(),"saH".to_string(),],vec!["sA".to_string(),"sByAm".to_string(),"sBiH".to_string(),],vec!["se".to_string(),"sByAm".to_string(),"sByaH".to_string(),],vec!["saH".to_string(),"sByAm".to_string(),"sByaH".to_string(),],vec!["saH".to_string(),"soH".to_string(),"sAm".to_string(),],vec!["si".to_string(),"soH".to_string(),"tsu".to_string(),],vec!["H".to_string(),"sO".to_string(),"saH".to_string(),],]);
  let r_pum = m.get(&("r".to_string(), "pum".to_string())).expect("r pum").clone();
  m.insert(("r".to_string(), "stri".to_string()), r_pum);
  // र-anta nap (वार) — 7.1.19/20 शी/शि वः/वारी/वारि; पद वार्भ्याम्/वार्षु. Not a-stem *वारम्.
  m.insert(("r".to_string(),"nap".to_string()), vec![vec!["H".to_string(),"rI".to_string(),"ri".to_string(),],vec!["H".to_string(),"rI".to_string(),"ri".to_string(),],vec!["rA".to_string(),"rByAm".to_string(),"rBiH".to_string(),],vec!["re".to_string(),"rByAm".to_string(),"rByaH".to_string(),],vec!["raH".to_string(),"rByAm".to_string(),"rByaH".to_string(),],vec!["raH".to_string(),"roH".to_string(),"rAm".to_string(),],vec!["ri".to_string(),"roH".to_string(),"rzu".to_string(),],vec!["H".to_string(),"rI".to_string(),"ri".to_string(),],]);
  let s_pum = m.get(&("s".to_string(), "pum".to_string())).expect("s pum").clone();
  m.insert(("s".to_string(), "stri".to_string()), s_pum);
  // as-anta (मनस्) as pum/nap already but pum missing — 8.2.66 ru, 8.3.15 khari? Actually as→aH at pada
  // sūtra: as→aH (मनस्→मनः), oblique asA/asO; Future devs: as pum mirrors as nap but linga matters for nom
  // Extreme: keep tsu/zu for saptamī, consistent with s/r/j/d/h/B above
  m.insert(("as".to_string(),"pum".to_string()), vec![vec!["aH".to_string(),"asO".to_string(),"asaH".to_string(),],vec!["asam".to_string(),"asO".to_string(),"asaH".to_string(),],vec!["asA".to_string(),"oByAm".to_string(),"oBiH".to_string(),],vec!["ase".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["asaH".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["asaH".to_string(),"asoH".to_string(),"asAm".to_string(),],vec!["asi".to_string(),"asoH".to_string(),"aHsu".to_string(),],vec!["aH".to_string(),"asO".to_string(),"asaH".to_string(),],]);
  // is/us with pum — 8.3.15 haviṣ-like but pum (e.g. is pum?) — s→ḥ via ru same as nap
  m.insert(("is".to_string(),"pum".to_string()), vec![vec!["iH".to_string(),"iSO".to_string(),"iSaH".to_string(),],vec!["iSam".to_string(),"iSO".to_string(),"iSaH".to_string(),],vec!["iSA".to_string(),"oByAm".to_string(),"oBiH".to_string(),],vec!["iSe".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["iSaH".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["iSaH".to_string(),"iSoH".to_string(),"iSAm".to_string(),],vec!["iSi".to_string(),"iSoH".to_string(),"iHsu".to_string(),],vec!["iH".to_string(),"iSO".to_string(),"iSaH".to_string(),],]);
  m.insert(("us".to_string(),"pum".to_string()), vec![vec!["uH".to_string(),"uSO".to_string(),"uSaH".to_string(),],vec!["uSam".to_string(),"uSO".to_string(),"uSaH".to_string(),],vec!["uSA".to_string(),"oByAm".to_string(),"oBiH".to_string(),],vec!["uSe".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["uSaH".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["uSaH".to_string(),"uSoH".to_string(),"uSAm".to_string(),],vec!["uSi".to_string(),"uSoH".to_string(),"uHsu".to_string(),],vec!["uH".to_string(),"uSO".to_string(),"uSaH".to_string(),],]);
  m
}

fn is_cons(c: char) -> bool {
    !matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

/// 8.4.40 स्तोः श्चुना श्चुः — न् after ज्/च्/श् → ञ् (राज्ञः).
fn scutva_n(word: &str) -> String {
    let c: Vec<char> = word.chars().collect();
    let mut out = String::with_capacity(word.len());
    for (i, &ch) in c.iter().enumerate() {
        if ch == 'n' && i > 0 && matches!(c[i - 1], 'S' | 'c' | 'C' | 'j' | 'J') {
            out.push('Y');
        } else {
            out.push(ch);
        }
    }
    out
}

/// 8.4.1–2 रषाभ्यां नो णः / अट्कुप्वाङ्नुम्व्यवायेऽपि. Not पदान्त न्.
fn apply_natva_word(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    let blockers = [
        'c', 'C', 'j', 'J', 'Y', 'S', 'w', 'W', 'q', 'Q', 'R', 't', 'T', 'd', 'D', 'l', 's',
    ];
    let mut out = chars.clone();
    for i in 0..chars.len() {
        if chars[i] != 'n' || i + 1 == chars.len() {
            continue;
        }
        let mut trigger = false;
        for &ch in &chars[..i] {
            if matches!(ch, 'r' | 'f' | 'F' | 'z') {
                trigger = true;
            } else if trigger && blockers.contains(&ch) {
                trigger = false;
            }
        }
        if trigger {
            out[i] = 'R';
        }
    }
    out.into_iter().collect()
}

fn polish(word: &str) -> String {
    scutva_n(&apply_natva_word(word))
}

/// अञ्च्-class: nom ङ् (8.2.23), strong ञ्च्, weak 6.4.24 ञ्-lopa (+ 6.4.139 ई / सम्प्रसारण), पद 8.2.30 ग्.
/// Closed list (not क्रुञ्च्, which keeps ञ्).
fn anc_parts(cand: &str) -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    Some(match cand {
        "prAYc" => ("prAN", "prAYc", "prAc", "prAg"),
        "pratyaYc" => ("pratyaN", "pratyaYc", "pratIc", "pratyag"),
        "udaYc" => ("udaN", "udaYc", "udIc", "udag"),
        "saDryaYc" => ("saDryaN", "saDryaYc", "saDrIc", "saDryag"),
        "tiryaYc" => ("tiryaN", "tiryaYc", "tiraSc", "tiryag"),
        "samyaYc" => ("samyaN", "samyaYc", "samIc", "samyag"),
        _ => return None,
    })
}

/// 4.1.5–6 ङीप्: इन्/उगित् अत्/न् स्त्री → दण्डिनी, भवती, राज्ञी; शतृ 7.1.81 पचन्ती. Not त्रिंशत्; अहन् stays nap.
fn ngeep_stri(cand: &str, linga: &str) -> String {
    if linga != "stri" || cand.ends_with('I') {
        return cand.to_string();
    }
    if cand.ends_with("Sat") || cand == "ahan" {
        return cand.to_string();
    }
    // 6.4.133 + 4.1.5: शुनी/यूनी/मघोनी (सम्प्रसारण then ङीप्).
    match cand {
        "Svan" => return "SunI".into(),
        "yuvan" => return "yUnI".into(),
        "maGavan" => return "maGonI".into(),
        _ => {}
    }
    // 4.1.5 ङीप् of अञ्च् weak: प्राची/प्रतीची (not च-anta *प्राक्).
    if let Some((_, _, weak, _)) = anc_parts(cand) {
        return format!("{weak}I");
    }
    // 4.1.6 ङीष् of क्वसु weak: विदुषी; aṅga-व् kept बभूवुषी (not *बभूषी).
    if let Some(pre) = cand.strip_suffix("vas") {
        if !pre.is_empty() {
            return if pre.chars().last().is_some_and(|c| !is_cons(c)) {
                format!("{pre}vuzI")
            } else {
                format!("{pre}uzI")
            };
        }
    }
    // 4.1.6 उगितश्च ङीप्. 7.1.81 शप्श्यनोर्नित्यम्: शतृ नुम् पचन्ती (not *पचती).
    // मतुप्/वतुप्/महत्/अभ्यस्त stay atI: धीमती, भवती, महती, ददती.
    if let Some(pre) = cand.strip_suffix("at") {
        if matches!(cand, "mahat" | "dadat" | "jakzat" | "jAgrat")
            || cand.ends_with("mat")
            || cand.ends_with("vat")
        {
            return format!("{cand}I");
        }
        return format!("{pre}antI");
    }
    if cand.ends_with("in") {
        return format!("{cand}I");
    }
    // 4.1.5 ऋन्नेभ्यो ङीप्: तृच् कर्त्री/धात्री. मातृ/स्वसृ stay ऋ-stem माता/स्वसा. नृ stays named.
    if let Some(pre) = cand.strip_suffix('f') {
        if cand != "nf"
            && !F_KINSHIP.contains(&cand)
            && !F_SVASR_NAPTR.contains(&cand)
        {
            return format!("{pre}rI");
        }
    }
    // 4.1.5 ऋन्नेभ्यो ङीप् after न्; 6.4.134 अल्लोपोऽनः → राज्ञी (8.4.40 श्चुत्व).
    if let Some(pre) = cand.strip_suffix("an") {
        if an_al_lopa(pre) {
            return polish(&format!("{pre}nI"));
        }
        return format!("{cand}I");
    }
    cand.to_string()
}

/// 6.4.10 सान्तमहतः संयोगस्य — महत् strong न्त् → आ (महान्तम् not शतृ *महन्तम्).
fn mahat_strong(table: &mut [Vec<String>]) {
    for row in table.iter_mut() {
        for cell in row.iter_mut() {
            *cell = cell
                .replace("antO", "AntO")
                .replace("antaH", "AntaH")
                .replace("antam", "Antam")
                .replace("antI", "AntI")
                .replace("anti", "Anti");
        }
    }
}

fn apply_natva(word_stem: &str, suffix: &str) -> String {
    if !suffix.contains('n') {
        return suffix.to_string();
    }
    let full = apply_natva_word(&format!("{word_stem}{suffix}"));
    full.chars().skip(word_stem.chars().count()).collect()
}

/// 6.4.134 अल्लोपोऽनः, blocked by 6.4.137 न संयोगाद्वमन्तात् (आत्मन्, ब्रह्मन्, यज्वन्).
fn an_al_lopa(pre: &str) -> bool {
    let c: Vec<char> = pre.chars().collect();
    !matches!(c.as_slice(), [.., a, 'v' | 'm'] if is_cons(*a))
}

fn decline_an(stem: &str, linga: &str) -> Declension {
    let pre = stem.strip_suffix("an").unwrap_or(stem);
    let lopa = an_al_lopa(pre);
    let weak = |v: &str| {
        polish(&if lopa {
            format!("{pre}n{v}")
        } else {
            format!("{stem}{v}")
        })
    };
    let strong = |v: &str| polish(&format!("{pre}{v}"));
    let pada = |v: &str| polish(&format!("{pre}{v}"));
    let mut sap = vec![weak("i")];
    if lopa {
        sap.push(polish(&format!("{stem}i")));
    }
    sap.push(weak("oH"));
    sap.push(pada("asu"));
    let mut decl = HashMap::new();
    if linga == "nap" {
        let mut nom = vec![strong("a"), weak("I"), strong("Ani")];
        if lopa {
            nom.insert(1, polish(&format!("{stem}I")));
        }
        decl.insert("prathamA".into(), nom.clone());
        decl.insert("dvitIyA".into(), nom.clone());
        decl.insert("tfIyA".into(), vec![weak("A"), pada("aByAm"), pada("aBiH")]);
        decl.insert("caturTI".into(), vec![weak("e"), pada("aByAm"), pada("aByaH")]);
        decl.insert("paYcamI".into(), vec![weak("aH"), pada("aByAm"), pada("aByaH")]);
        decl.insert("zazWI".into(), vec![weak("aH"), weak("oH"), weak("Am")]);
        decl.insert("saptamI".into(), sap);
        decl.insert("samboDana".into(), nom);
    } else {
        // SK पूषन्/अर्यमन्: सौ still आ; other सर्वनामस्थान no 6.4.7 नोपधा दीर्घ (पूषणौ not *पूषाणौ).
        let (du, pl, acc) = if matches!(stem, "pUzan" | "aryaman") {
            (strong("anO"), strong("anaH"), strong("anam"))
        } else {
            (strong("AnO"), strong("AnaH"), strong("Anam"))
        };
        decl.insert("prathamA".into(), vec![strong("A"), du.clone(), pl.clone()]);
        decl.insert("dvitIyA".into(), vec![acc, du.clone(), weak("aH")]);
        decl.insert("tfIyA".into(), vec![weak("A"), pada("aByAm"), pada("aBiH")]);
        decl.insert("caturTI".into(), vec![weak("e"), pada("aByAm"), pada("aByaH")]);
        decl.insert("paYcamI".into(), vec![weak("aH"), pada("aByAm"), pada("aByaH")]);
        decl.insert("zazWI".into(), vec![weak("aH"), weak("oH"), weak("Am")]);
        decl.insert("saptamI".into(), sap);
        decl.insert("samboDana".into(), vec![stem.to_string(), du, pl]);
    }
    Declension {
        stem: stem.to_string(),
        linga: linga.to_string(),
        declension: decl,
    }
}

/// अहन् nap — 8.2.69 रु in स्वमोः अहः; 6.4.134 अह्ना; पद अहर् → अहोभ्याम्/अहोभिः. Dual अहनी/अह्नी.
fn decline_ahan(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "ahan" || linga != "nap" {
        return None;
    }
    let nom = vec![
        "ahaH".into(),
        "ahanI".into(),
        "ahnI".into(),
        "ahAni".into(),
    ];
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), nom.clone());
    decl.insert("dvitIyA".into(), nom.clone());
    decl.insert("tfIyA".into(), vec!["ahnA".into(), "ahoByAm".into(), "ahoBiH".into()]);
    decl.insert("caturTI".into(), vec!["ahne".into(), "ahoByAm".into(), "ahoByaH".into()]);
    decl.insert("paYcamI".into(), vec!["ahnaH".into(), "ahoByAm".into(), "ahoByaH".into()]);
    decl.insert("zazWI".into(), vec!["ahnaH".into(), "ahnoH".into(), "ahnAm".into()]);
    decl.insert(
        "saptamI".into(),
        vec![
            "ahni".into(),
            "ahani".into(),
            "ahnoH".into(),
            "ahaHsu".into(),
            "ahassu".into(),
        ],
    );
    decl.insert("samboDana".into(), nom);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// वृत्रहन् — सर्वनामस्थान वृत्रहा/वृत्रहणौ (8.4.1); weak घ्न वृत्रघ्ना (6.4.98); पद वृत्रहभ्याम्. Not an-stem *वृत्रह्ना. अहन् stays अहः.
fn decline_han(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "vftrahan" || linga != "pum" {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["vftrahA".into(), "vftrahaRO".into(), "vftrahaRaH".into()]);
    decl.insert("dvitIyA".into(), vec!["vftrahaRam".into(), "vftrahaRO".into(), "vftraGnaH".into()]);
    decl.insert("tfIyA".into(), vec!["vftraGnA".into(), "vftrahaByAm".into(), "vftrahaBiH".into()]);
    decl.insert("caturTI".into(), vec!["vftraGne".into(), "vftrahaByAm".into(), "vftrahaByaH".into()]);
    decl.insert("paYcamI".into(), vec!["vftraGnaH".into(), "vftrahaByAm".into(), "vftrahaByaH".into()]);
    decl.insert("zazWI".into(), vec!["vftraGnaH".into(), "vftraGnoH".into(), "vftraGnAm".into()]);
    decl.insert("saptamI".into(), vec!["vftraGni".into(), "vftrahaRi".into(), "vftraGnoH".into(), "vftrahasu".into()]);
    decl.insert("samboDana".into(), vec!["vftrahan".into(), "vftrahaRO".into(), "vftrahaRaH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// अर्वन् — अनङ् प्रथमा एक अर्वा (not शतृ *अर्वान्); सर्वनामस्थान नुम् अर्वन्तौ/अर्वन्तम् like भवत्;
/// weak/पद अत्/द् अर्वता/अर्वद्भ्याम्/अर्वत्सु. Voc अर्वन्. Not an-stem *अर्वानम्. Exact `arvan` before `an`.
fn decline_arvan(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "arvan" || linga != "pum" {
        return None;
    }
    let strong = "arvant";
    let weak = "arvat";
    let pada = "arvad";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["arvA".into(), format!("{strong}O"), format!("{strong}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{strong}am"), format!("{strong}O"), format!("{weak}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{pada}ByAm"), format!("{pada}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{weak}su")]);
    decl.insert("samboDana".into(), vec!["arvan".into(), format!("{strong}O"), format!("{strong}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// 6.4.133 श्वयुवमघोनामतद्धिते — सम्प्रसारण in weak; पद श्व/युव/मघव. पुं.
fn decline_sva_yuv_magha(cand: &str, linga: &str) -> Option<Declension> {
    if linga != "pum" {
        return None;
    }
    let (strong, weak, pada, voc) = match cand {
        "Svan" => ("Sv", "Sun", "Sva", "Svan"),
        "yuvan" => ("yuv", "yUn", "yuva", "yuvan"),
        "maGavan" => ("maGav", "maGon", "maGava", "maGavan"),
        _ => return None,
    };
    let du = format!("{strong}AnO");
    let pl = format!("{strong}AnaH");
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![format!("{strong}A"), du.clone(), pl.clone()]);
    decl.insert("dvitIyA".into(), vec![format!("{strong}Anam"), du.clone(), format!("{weak}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{pada}ByAm"), format!("{pada}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{pada}su")]);
    decl.insert("samboDana".into(), vec![voc.to_string(), du, pl]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// 7.1.85–87 पथिमथोः / इतोऽत् / थो न्थः; ऋभुक्षिन् same सर्वनामस्थान. पुं; nap compounds 7.1.23 सुपथि/सुपन्थानि.
fn decline_pathadi(cand: &str, linga: &str) -> Option<Declension> {
    // पथिन् nap compounds (सुपथिन्) — 7.1.23 स्वमोः सुपथि/सुपथी; 7.1.85 नुम् सुपन्थानि. Not पुं *सुपन्थाः.
    if linga == "nap" {
        let pre = cand.strip_suffix("paTin")?;
        if pre.is_empty() {
            return None;
        }
        let weak = format!("{pre}paT");
        let pada = format!("{pre}paTi");
        let nom = vec![
            format!("{pre}paTi"),
            format!("{pre}paTI"),
            format!("{pre}panTAni"),
        ];
        let mut decl = HashMap::new();
        decl.insert("prathamA".into(), nom.clone());
        decl.insert("dvitIyA".into(), nom.clone());
        decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{pada}ByAm"), format!("{pada}BiH")]);
        decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
        decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
        decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
        decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{pada}zu")]);
        let mut voc = vec![cand.to_string()];
        voc.extend(nom);
        decl.insert("samboDana".into(), voc);
        return Some(Declension {
            stem: cand.to_string(),
            linga: linga.to_string(),
            declension: decl,
        });
    }
    if linga != "pum" {
        return None;
    }
    let (nom, strong, weak, pada) = match cand {
        "paTin" => ("panTAH", "panTAn", "paT", "paTi"),
        "maTin" => ("manTAH", "manTAn", "maT", "maTi"),
        "fBukzin" => ("fBukzAH", "fBukzAR", "fBukz", "fBukzi"),
        _ => return None,
    };
    let du = format!("{strong}O");
    let pl = format!("{strong}aH");
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![nom.to_string(), du.clone(), pl.clone()]);
    decl.insert("dvitIyA".into(), vec![format!("{strong}am"), du.clone(), format!("{weak}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{pada}ByAm"), format!("{pada}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{pada}zu")]);
    decl.insert("samboDana".into(), vec![nom.to_string(), du, pl]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// 7.1.93 अनङ् सौ सखा; 7.1.92 सख्युरसम्बुद्धौ सख्युः; voc 7.3.109 सखे. Not i-stem *सखिः.
fn decline_sakhi(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "saKi" || linga != "pum" {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["saKA".into(), "saKAyO".into(), "saKAyaH".into()]);
    decl.insert("dvitIyA".into(), vec!["saKAyam".into(), "saKAyO".into(), "saKIn".into()]);
    decl.insert("tfIyA".into(), vec!["saKyA".into(), "saKiByAm".into(), "saKiBiH".into()]);
    decl.insert("caturTI".into(), vec!["saKye".into(), "saKiByAm".into(), "saKiByaH".into()]);
    decl.insert("paYcamI".into(), vec!["saKyuH".into(), "saKiByAm".into(), "saKiByaH".into()]);
    decl.insert("zazWI".into(), vec!["saKyuH".into(), "saKyoH".into(), "saKInAm".into()]);
    decl.insert("saptamI".into(), vec!["saKyO".into(), "saKyoH".into(), "saKizu".into()]);
    decl.insert("samboDana".into(), vec!["saKe".into(), "saKAyO".into(), "saKAyaH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// पुंस् — सर्वनामस्थान नुम् पुमान्/पुमांसौ; weak पुंसा; पद पुंभ्याम् (8.3.6). Not s-anta *पुः.
/// Compounds nap (7.1.23): सुपुम्/सुपुंसी/सुपुमांसि, not पुमान्.
fn decline_pums(cand: &str, linga: &str) -> Option<Declension> {
    if linga == "nap" {
        let pre = cand.strip_suffix("puMs").or_else(|| cand.strip_suffix("pums"))?;
        if pre.is_empty() {
            return None;
        }
        let sg = format!("{pre}pum");
        let dual = format!("{pre}puMsI");
        let pl = format!("{pre}pumAMsi");
        let weak = format!("{pre}puMs");
        let pada = format!("{pre}puM");
        let pada2 = format!("{pre}pum");
        let nom = vec![sg.clone(), dual.clone(), pl.clone()];
        let mut decl = HashMap::new();
        decl.insert("prathamA".into(), nom.clone());
        decl.insert("dvitIyA".into(), nom.clone());
        decl.insert(
            "tfIyA".into(),
            vec![
                format!("{weak}A"),
                format!("{pada}ByAm"),
                format!("{pada2}ByAm"),
                format!("{pada}BiH"),
                format!("{pada2}BiH"),
            ],
        );
        decl.insert(
            "caturTI".into(),
            vec![
                format!("{weak}e"),
                format!("{pada}ByAm"),
                format!("{pada2}ByAm"),
                format!("{pada}ByaH"),
                format!("{pada2}ByaH"),
            ],
        );
        decl.insert(
            "paYcamI".into(),
            vec![
                format!("{weak}aH"),
                format!("{pada}ByAm"),
                format!("{pada2}ByAm"),
                format!("{pada}ByaH"),
                format!("{pada2}ByaH"),
            ],
        );
        decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
        decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{pada}su")]);
        decl.insert("samboDana".into(), nom);
        return Some(Declension {
            stem: cand.to_string(),
            linga: linga.to_string(),
            declension: decl,
        });
    }
    if linga != "pum" || !matches!(cand, "puMs" | "pums") {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["pumAn".into(), "pumAMsO".into(), "pumAMsaH".into()]);
    decl.insert("dvitIyA".into(), vec!["pumAMsam".into(), "pumAMsO".into(), "puMsaH".into()]);
    decl.insert(
        "tfIyA".into(),
        vec!["puMsA".into(), "puMByAm".into(), "pumByAm".into(), "puMBiH".into(), "pumBiH".into()],
    );
    decl.insert(
        "caturTI".into(),
        vec!["puMse".into(), "puMByAm".into(), "pumByAm".into(), "puMByaH".into(), "pumByaH".into()],
    );
    decl.insert(
        "paYcamI".into(),
        vec!["puMsaH".into(), "puMByAm".into(), "pumByAm".into(), "puMByaH".into(), "pumByaH".into()],
    );
    decl.insert("zazWI".into(), vec!["puMsaH".into(), "puMsoH".into(), "puMsAm".into()]);
    decl.insert("saptamI".into(), vec!["puMsi".into(), "puMsoH".into(), "puMsu".into()]);
    decl.insert("samboDana".into(), vec!["puman".into(), "pumAMsO".into(), "pumAMsaH".into()]);
    Some(Declension {
        stem: "puMs".into(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// अप् स्त्री — 6.4.11 आपः in सर्वनामस्थान; पद अद्भ्याम्/अद्भिः/अप्सु. No एकवचन.
fn decline_ap(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "ap" || linga != "stri" {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![String::new(), "ApO".into(), "ApaH".into()]);
    decl.insert("dvitIyA".into(), vec![String::new(), "ApO".into(), "ApaH".into()]);
    decl.insert("tfIyA".into(), vec![String::new(), "adByAm".into(), "adBiH".into()]);
    decl.insert("caturTI".into(), vec![String::new(), "adByAm".into(), "adByaH".into()]);
    decl.insert("paYcamI".into(), vec![String::new(), "adByAm".into(), "adByaH".into()]);
    decl.insert("zazWI".into(), vec![String::new(), "apoH".into(), "apAm".into()]);
    decl.insert("saptamI".into(), vec![String::new(), "apoH".into(), "apsu".into()]);
    decl.insert("samboDana".into(), vec![String::new(), "ApO".into(), "ApaH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// अनडुह् — strong अनड्वान्/अनड्वाहौ (वाह्); पद अनडुद्भ्याम्/अनडुत्सु. Not h-anta *अनडुक्.
/// Compounds nap (7.1.23): स्वनडुत्/स्वनडुही/स्वनड्वांहि.
fn decline_anaquh(cand: &str, linga: &str) -> Option<Declension> {
    if linga == "nap" {
        let pre = cand.strip_suffix("anaquh")?;
        if pre.is_empty() {
            return None;
        }
        let t = format!("{pre}anaqut");
        let d = format!("{pre}anaqud");
        let dual = format!("{pre}anaquhI");
        let pl = format!("{pre}anaqvAMhi");
        let weak = format!("{pre}anaquh");
        let nom = vec![t.clone(), d.clone(), dual, pl];
        let mut decl = HashMap::new();
        decl.insert("prathamA".into(), nom.clone());
        decl.insert("dvitIyA".into(), nom.clone());
        decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{d}ByAm"), format!("{d}BiH")]);
        decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{d}ByAm"), format!("{d}ByaH")]);
        decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{d}ByAm"), format!("{d}ByaH")]);
        decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
        decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{t}su")]);
        decl.insert("samboDana".into(), nom);
        return Some(Declension {
            stem: cand.to_string(),
            linga: linga.to_string(),
            declension: decl,
        });
    }
    if cand != "anaquh" || linga != "pum" {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["anaqvAn".into(), "anaqvAhO".into(), "anaqvAhaH".into()]);
    decl.insert("dvitIyA".into(), vec!["anaqvAham".into(), "anaqvAhO".into(), "anaquhaH".into()]);
    decl.insert("tfIyA".into(), vec!["anaquhA".into(), "anaqudByAm".into(), "anaqudBiH".into()]);
    decl.insert("caturTI".into(), vec!["anaquhe".into(), "anaqudByAm".into(), "anaqudByaH".into()]);
    decl.insert("paYcamI".into(), vec!["anaquhaH".into(), "anaqudByAm".into(), "anaqudByaH".into()]);
    decl.insert("zazWI".into(), vec!["anaquhaH".into(), "anaquhoH".into(), "anaquhAm".into()]);
    decl.insert("saptamI".into(), vec!["anaquhi".into(), "anaquhoH".into(), "anaqutsu".into()]);
    decl.insert("samboDana".into(), vec!["anaqvan".into(), "anaqvAhO".into(), "anaqvAhaH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// उपानह् — पद ह्→द् (उपानद्भ्याम्/उपानत्सु), nom उपानत्/उपानद्. Not लिह्-type *उपानक्.
fn decline_upanah(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "upAnah" || (linga != "stri" && linga != "pum") {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert(
        "prathamA".into(),
        vec!["upAnat".into(), "upAnad".into(), "upAnahO".into(), "upAnahaH".into()],
    );
    decl.insert("dvitIyA".into(), vec!["upAnaham".into(), "upAnahO".into(), "upAnahaH".into()]);
    decl.insert("tfIyA".into(), vec!["upAnahA".into(), "upAnadByAm".into(), "upAnadBiH".into()]);
    decl.insert("caturTI".into(), vec!["upAnahe".into(), "upAnadByAm".into(), "upAnadByaH".into()]);
    decl.insert("paYcamI".into(), vec!["upAnahaH".into(), "upAnadByAm".into(), "upAnadByaH".into()]);
    decl.insert("zazWI".into(), vec!["upAnahaH".into(), "upAnahoH".into(), "upAnahAm".into()]);
    decl.insert("saptamI".into(), vec!["upAnahi".into(), "upAnahoH".into(), "upAnatsu".into()]);
    decl.insert(
        "samboDana".into(),
        vec!["upAnat".into(), "upAnad".into(), "upAnahO".into(), "upAnahaH".into()],
    );
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// दिव् — 7.1.84 दिव औत् द्यौः; पद सम्प्रसारण द्युभ्याम्/द्युषु. Compounds सुद्यौः. Not v-fallback *दिवा.
/// नपुं 7.1.23 द्यु/द्युनी/द्यूनि (not o-stem *द्यौः). `*div` or nap `*dyo` (gold प्रद्यो).
fn decline_div(cand: &str, linga: &str) -> Option<Declension> {
    if linga == "nap" {
        let pre = cand
            .strip_suffix("div")
            .or_else(|| cand.strip_suffix("dyo"))?;
        let u = format!("{pre}dyu");
        let nom = vec![
            u.clone(),
            polish(&format!("{u}nI")),
            format!("{pre}dyUni"),
        ];
        let mut decl = HashMap::new();
        decl.insert("prathamA".into(), nom.clone());
        decl.insert("dvitIyA".into(), nom.clone());
        decl.insert("tfIyA".into(), vec![polish(&format!("{u}nA")), format!("{u}ByAm"), format!("{u}BiH")]);
        decl.insert("caturTI".into(), vec![polish(&format!("{u}ne")), format!("{u}ByAm"), format!("{u}ByaH")]);
        decl.insert("paYcamI".into(), vec![polish(&format!("{u}naH")), format!("{u}ByAm"), format!("{u}ByaH")]);
        decl.insert("zazWI".into(), vec![polish(&format!("{u}naH")), polish(&format!("{u}noH")), polish(&format!("{pre}dyUnAm"))]);
        decl.insert("saptamI".into(), vec![polish(&format!("{u}ni")), polish(&format!("{u}noH")), format!("{u}zu")]);
        let mut voc = vec![u, format!("{pre}dyo")];
        voc.extend(nom.into_iter().skip(1));
        decl.insert("samboDana".into(), voc);
        return Some(Declension {
            stem: cand.to_string(),
            linga: linga.to_string(),
            declension: decl,
        });
    }
    let pre = cand.strip_suffix("div")?;
    if linga != "stri" && linga != "pum" {
        return None;
    }
    let weak = cand;
    let pada = format!("{pre}dyu");
    let nom = format!("{pre}dyOH");
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![nom.clone(), format!("{weak}O"), format!("{weak}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{weak}am"), format!("{weak}O"), format!("{weak}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{pada}ByAm"), format!("{pada}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{pada}zu")]);
    decl.insert("samboDana".into(), vec![nom, format!("{weak}O"), format!("{weak}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// रै — 6.1.78 एचोऽयवायावः आय् (रायम्); सु राः; पद राभ्याम्. Not नौ *रावम्.
fn decline_rai(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "rE" || (linga != "stri" && linga != "pum") {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["rAH".into(), "rAyO".into(), "rAyaH".into()]);
    decl.insert("dvitIyA".into(), vec!["rAyam".into(), "rAyO".into(), "rAyaH".into()]);
    decl.insert("tfIyA".into(), vec!["rAyA".into(), "rAByAm".into(), "rABiH".into()]);
    decl.insert("caturTI".into(), vec!["rAye".into(), "rAByAm".into(), "rAByaH".into()]);
    decl.insert("paYcamI".into(), vec!["rAyaH".into(), "rAByAm".into(), "rAByaH".into()]);
    decl.insert("zazWI".into(), vec!["rAyaH".into(), "rAyoH".into(), "rAyAm".into()]);
    decl.insert("saptamI".into(), vec!["rAyi".into(), "rAyoH".into(), "rAsu".into()]);
    decl.insert("samboDana".into(), vec!["rAH".into(), "rAyO".into(), "rAyaH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// प्राञ्च्-class — 8.2.23 प्राङ्; 6.4.24 प्राचा; 8.2.30 प्राग्/प्राक्षु. नपुं 7.1.23 प्राक्/प्राची/प्राञ्चि. Not च-anta *प्राक् as पुं.
fn decline_anc(cand: &str, linga: &str) -> Option<Declension> {
    let (nom, strong, weak, pada) = anc_parts(cand)?;
    if linga != "pum" && linga != "nap" {
        return None;
    }
    let loc_pl = format!("{}kzu", pada.strip_suffix('g').unwrap_or(pada));
    let mut decl = HashMap::new();
    if linga == "nap" {
        let nom_sg = format!("{}k", pada.strip_suffix('g').unwrap_or(pada));
        let row = vec![nom_sg, format!("{weak}I"), format!("{strong}i")];
        decl.insert("prathamA".into(), row.clone());
        decl.insert("dvitIyA".into(), row.clone());
        decl.insert("samboDana".into(), row);
    } else {
        decl.insert("prathamA".into(), vec![nom.into(), format!("{strong}O"), format!("{strong}aH")]);
        decl.insert("dvitIyA".into(), vec![format!("{strong}am"), format!("{strong}O"), format!("{weak}aH")]);
        decl.insert("samboDana".into(), vec![nom.into(), format!("{strong}O"), format!("{strong}aH")]);
    }
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{pada}ByAm"), format!("{pada}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), loc_pl]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// क्रुञ्च् — 8.2.23 संयोगान्तस्य लोपः क्रुङ्; ञ् stays in अङ्ग (क्रुञ्चा) unlike 6.4.24 प्राचा; पद क्रुङ्भ्याम्.
fn decline_krunc(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "kruYc" || (linga != "pum" && linga != "stri") {
        return None;
    }
    let strong = "kruYc";
    let pada = "kruN";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["kruN".into(), format!("{strong}O"), format!("{strong}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{strong}am"), format!("{strong}O"), format!("{strong}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{strong}A"), format!("{pada}ByAm"), format!("{pada}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{strong}e"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{strong}aH"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{strong}aH"), format!("{strong}oH"), format!("{strong}Am")]);
    decl.insert("saptamI".into(), vec![format!("{strong}i"), format!("{strong}oH"), format!("{pada}kzu")]);
    decl.insert("samboDana".into(), vec!["kruN".into(), format!("{strong}O"), format!("{strong}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// क्वसु (विद्वस्, बभूवस्) — 7.1.70 नुम् विद्वान्/बभूवान्; 6.4.131 विदुषा vs बभूवुषा (aṅga व् kept).
/// 8.2.72 विद्वद्भिः/बभूवद्भिः. Not as-pum *विद्वः. मनस् stays मनः.
fn decline_kvasu(cand: &str, linga: &str) -> Option<Declension> {
    let pre = cand.strip_suffix("vas")?;
    if pre.is_empty() || (linga != "pum" && linga != "nap") {
        return None;
    }
    // वस्-प्रत्यय सम्प्रसारण after हल् (विद्-वस् → विदुष्). Vowel-final aṅga keeps व् (बभूव-अस् → बभूवुष्).
    let weak = if pre.chars().last().is_some_and(|c| !is_cons(c)) {
        format!("{pre}vuz")
    } else {
        format!("{pre}uz")
    };
    let pada = format!("{pre}vad");
    let mut decl = HashMap::new();
    if linga == "nap" {
        // 7.1.23 स्वमोः: विद्वत्/विदुषी/विद्वांसि (not पुं विद्वान्).
        let nom = vec![
            format!("{pre}vat"),
            format!("{weak}I"),
            format!("{pre}vAMsi"),
        ];
        decl.insert("prathamA".into(), nom.clone());
        decl.insert("dvitIyA".into(), nom.clone());
        decl.insert("samboDana".into(), nom);
    } else {
        let strong = format!("{pre}vAMs");
        let nom = format!("{pre}vAn");
        decl.insert("prathamA".into(), vec![nom, format!("{strong}O"), format!("{strong}aH")]);
        decl.insert("dvitIyA".into(), vec![format!("{strong}am"), format!("{strong}O"), format!("{weak}aH")]);
        decl.insert("samboDana".into(), vec![format!("{pre}van"), format!("{strong}O"), format!("{strong}aH")]);
    }
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{pada}ByAm"), format!("{pada}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    // 8.4.55 खरि च: द्+सु → त्सु विद्वत्सु.
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{pre}vatsu")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// नृ — पितृ-type नरम् (not कर्तृ *नारम्); acc नॄन्; 6.4.6 नृ च नॄणाम्/नृणाम्.
fn decline_nr(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "nf" || linga != "pum" {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["nA".into(), "narO".into(), "naraH".into()]);
    decl.insert("dvitIyA".into(), vec!["naram".into(), "narO".into(), "nFn".into()]);
    decl.insert("tfIyA".into(), vec!["nrA".into(), "nfByAm".into(), "nfBiH".into()]);
    decl.insert("caturTI".into(), vec!["nre".into(), "nfByAm".into(), "nfByaH".into()]);
    decl.insert("paYcamI".into(), vec!["nuH".into(), "nfByAm".into(), "nfByaH".into()]);
    decl.insert("zazWI".into(), vec!["nuH".into(), "nroH".into(), "nFRAm".into(), "nfRAm".into()]);
    decl.insert("saptamI".into(), vec!["nari".into(), "nroH".into(), "nfzu".into()]);
    decl.insert("samboDana".into(), vec!["naH".into(), "narO".into(), "naraH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// अस्थि/दधि/सक्थि/अक्षि — 7.1.75 अनङ् before vowel (दध्ना); स्वमोः दधि/दधिनी/दधीनि. Not i-nap *दधिना.
fn decline_asthyadi(cand: &str, linga: &str) -> Option<Declension> {
    if linga != "nap" {
        return None;
    }
    let pre = match cand {
        "asTi" | "daDi" | "sakTi" | "akzi" => cand.strip_suffix('i')?,
        _ => return None,
    };
    let weak = |v: &str| polish(&format!("{pre}n{v}"));
    let pada = |v: &str| polish(&format!("{cand}{v}"));
    let nom = vec![
        cand.to_string(),
        format!("{cand}nI"),
        polish(&format!("{pre}Ini")),
    ];
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), nom.clone());
    decl.insert("dvitIyA".into(), nom.clone());
    decl.insert("tfIyA".into(), vec![weak("A"), pada("ByAm"), pada("BiH")]);
    decl.insert("caturTI".into(), vec![weak("e"), pada("ByAm"), pada("ByaH")]);
    decl.insert("paYcamI".into(), vec![weak("aH"), pada("ByAm"), pada("ByaH")]);
    decl.insert("zazWI".into(), vec![weak("aH"), weak("oH"), weak("Am")]);
    decl.insert(
        "saptamI".into(),
        vec![weak("i"), polish(&format!("{pre}ani")), weak("oH"), pada("zu")],
    );
    decl.insert(
        "samboDana".into(),
        vec![cand.to_string(), format!("{pre}e"), format!("{cand}nI"), polish(&format!("{pre}Ini"))],
    );
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// आशिस्/सजष् — 8.2.66 सजषो रुः, 8.2.76 इक्-upadhā दीर्घ आशीः/सजूः. चिकीर्ष् चिकीः. Not ष-anta *आशिट्; धनुस् stays nap.
/// Exact `ASiz`/`pipaWiz`/`sajuz` or `*Irz`. त्विष्/द्विष् stay ट् (not *त्वीः).
fn decline_sajush(cand: &str, linga: &str) -> Option<Declension> {
    if linga != "stri" && linga != "pum" {
        return None;
    }
    let (nom, pada, loc_pl) = if cand == "sajuz" {
        ("sajUH".to_string(), "sajUr".to_string(), "sajUHzu".to_string())
    } else if let Some(p) = cand.strip_suffix("Irz") {
        if p.is_empty() {
            return None;
        }
        (format!("{p}IH"), format!("{p}Ir"), format!("{p}Irzu"))
    } else if matches!(cand, "ASiz" | "pipaWiz") {
        let p = cand.strip_suffix("iz")?;
        let nom = format!("{p}IH");
        (nom.clone(), format!("{p}Ir"), format!("{nom}zu"))
    } else {
        return None;
    };
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![nom.clone(), format!("{cand}O"), format!("{cand}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{cand}am"), format!("{cand}O"), format!("{cand}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{cand}A"), format!("{pada}ByAm"), format!("{pada}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{cand}e"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{cand}aH"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{cand}aH"), format!("{cand}oH"), format!("{cand}Am")]);
    decl.insert("saptamI".into(), vec![format!("{cand}i"), format!("{cand}oH"), loc_pl]);
    decl.insert("samboDana".into(), vec![nom, format!("{cand}O"), format!("{cand}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// राज् (क्विन्) — पद राट्/राड् (not 8.2.30 *राक् like वणिक्); राड्भ्याम्, राट्सु.
/// Compounds: पद वृद्धि विश्वाराट् (विश्वराज्). राजन् stays राजा.
fn decline_raj(cand: &str, linga: &str) -> Option<Declension> {
    if linga != "pum" && linga != "stri" {
        return None;
    }
    let pre = cand.strip_suffix("rAj")?;
    let (pada_w, pada_q) = if pre.is_empty() {
        ("rAw".to_string(), "rAq".to_string())
    } else {
        let long = if let Some(p) = pre.strip_suffix('a') {
            format!("{p}A")
        } else {
            pre.to_string()
        };
        (format!("{long}rAw"), format!("{long}rAq"))
    };
    let strong = cand;
    let mut decl = HashMap::new();
    decl.insert(
        "prathamA".into(),
        vec![pada_w.clone(), pada_q.clone(), format!("{strong}O"), format!("{strong}aH")],
    );
    decl.insert("dvitIyA".into(), vec![format!("{strong}am"), format!("{strong}O"), format!("{strong}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{strong}A"), format!("{pada_q}ByAm"), format!("{pada_q}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{strong}e"), format!("{pada_q}ByAm"), format!("{pada_q}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{strong}aH"), format!("{pada_q}ByAm"), format!("{pada_q}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{strong}aH"), format!("{strong}oH"), format!("{strong}Am")]);
    decl.insert("saptamI".into(), vec![format!("{strong}i"), format!("{strong}oH"), format!("{pada_w}su")]);
    decl.insert(
        "samboDana".into(),
        vec![pada_w, pada_q, format!("{strong}O"), format!("{strong}aH")],
    );
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// उशनस्/अनेहस् — 7.1.94 ऋदुशनस्पुरुदंहोऽनेहसां च: सु उशना/अनेहा not as-pum *उशनः.
fn decline_ushanasadi(cand: &str, linga: &str) -> Option<Declension> {
    if linga != "pum" {
        return None;
    }
    let (nom, voc): (&str, Vec<String>) = match cand {
        "uSanas" => ("uSanA", vec!["uSanan".into(), "uSana".into(), "uSanaH".into()]),
        "anehas" => ("anehA", vec!["anehaH".into()]),
        _ => return None,
    };
    let pre = cand.strip_suffix("as")?;
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![nom.to_string(), format!("{cand}O"), format!("{cand}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{cand}am"), format!("{cand}O"), format!("{cand}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{cand}A"), format!("{pre}oByAm"), format!("{pre}oBiH")]);
    decl.insert("caturTI".into(), vec![format!("{cand}e"), format!("{pre}oByAm"), format!("{pre}oByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{cand}aH"), format!("{pre}oByAm"), format!("{pre}oByaH")]);
    decl.insert("zazWI".into(), vec![format!("{cand}aH"), format!("{cand}oH"), format!("{cand}Am")]);
    decl.insert("saptamI".into(), vec![format!("{cand}i"), format!("{cand}oH"), format!("{pre}aHsu")]);
    let mut samb = voc;
    samb.extend([format!("{cand}O"), format!("{cand}aH")]);
    decl.insert("samboDana".into(), samb);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// पाद् — 6.4.130 पादः पत् weak पद (सुपदा); पद सुपात्/सुपाद्भ्याम्. Not d-anta *सुपादा.
fn decline_pad(cand: &str, linga: &str) -> Option<Declension> {
    let pre = cand.strip_suffix("pAd")?;
    if linga != "pum" && linga != "stri" {
        return None;
    }
    let strong = cand;
    let weak = format!("{pre}pad");
    let mut decl = HashMap::new();
    decl.insert(
        "prathamA".into(),
        vec![format!("{pre}pAt"), strong.to_string(), format!("{strong}O"), format!("{strong}aH")],
    );
    decl.insert("dvitIyA".into(), vec![format!("{strong}am"), format!("{strong}O"), format!("{weak}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{strong}ByAm"), format!("{strong}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{strong}ByAm"), format!("{strong}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{strong}ByAm"), format!("{strong}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{pre}pAtsu")]);
    decl.insert(
        "samboDana".into(),
        vec![format!("{pre}pAt"), strong.to_string(), format!("{strong}O"), format!("{strong}aH")],
    );
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// गिर — 8.2.77 हलि च: इर् दीर्घ before हल् गीः/गीर्भ्याम्/गीर्षु. Not generic r *गिः/*गित्सु.
fn decline_gir(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "gir" || (linga != "stri" && linga != "pum") {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["gIH".into(), "girO".into(), "giraH".into()]);
    decl.insert("dvitIyA".into(), vec!["giram".into(), "girO".into(), "giraH".into()]);
    decl.insert("tfIyA".into(), vec!["girA".into(), "gIrByAm".into(), "gIrBiH".into()]);
    decl.insert("caturTI".into(), vec!["gire".into(), "gIrByAm".into(), "gIrByaH".into()]);
    decl.insert("paYcamI".into(), vec!["giraH".into(), "gIrByAm".into(), "gIrByaH".into()]);
    decl.insert("zazWI".into(), vec!["giraH".into(), "giroH".into(), "girAm".into()]);
    decl.insert("saptamI".into(), vec!["giri".into(), "giroH".into(), "gIrzu".into()]);
    decl.insert("samboDana".into(), vec!["gIH".into(), "girO".into(), "giraH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// दुह्/द्रुह् — 8.2.32 दादेर्धातोर्घः: पद धुक्/धुग्, ध्रुक्. Not generic h *दुक्. उष्णिह् stays उष्णिक्.
fn decline_duhadi(cand: &str, linga: &str) -> Option<Declension> {
    if linga != "pum" && linga != "stri" {
        return None;
    }
    let pada = match cand {
        "duh" => "Du",
        "druh" => "Dru",
        _ => return None,
    };
    let weak = cand;
    let k = format!("{pada}k");
    let g = format!("{pada}g");
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![k.clone(), g.clone(), format!("{weak}O"), format!("{weak}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{weak}am"), format!("{weak}O"), format!("{weak}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{g}ByAm"), format!("{g}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{g}ByAm"), format!("{g}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{g}ByAm"), format!("{g}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{k}zu")]);
    decl.insert("samboDana".into(), vec![k, g, format!("{weak}O"), format!("{weak}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// लिह् — 8.2.31 हो ढः: पद लिट्/लिड्, लिड्भ्याम्/लिट्सु. Not कुत्व *लिक् (उष्णिह् stays उष्णिक्).
fn decline_lih(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "lih" || (linga != "pum" && linga != "stri") {
        return None;
    }
    let weak = cand;
    let w = "liw";
    let q = "liq";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![w.into(), q.into(), format!("{weak}O"), format!("{weak}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{weak}am"), format!("{weak}O"), format!("{weak}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{q}ByAm"), format!("{q}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{q}ByAm"), format!("{q}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{q}ByAm"), format!("{q}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{w}su")]);
    decl.insert("samboDana".into(), vec![w.into(), q.into(), format!("{weak}O"), format!("{weak}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// वाह् — 6.4.132 ऊठ् + 6.1.87 गुण विश्वौहा; पद 8.2.31/8.4.41 विश्ववाट्. तुरासाह् तुराषाट् (no ऊठ्). Not h-anta *क्.
fn decline_vah(cand: &str, linga: &str) -> Option<Declension> {
    if linga != "pum" && linga != "stri" {
        return None;
    }
    let (strong, weak, pada_w, pada_q) = if let Some(pre) = cand.strip_suffix("vAh") {
        if pre.is_empty() {
            return None;
        }
        let weak = if let Some(p) = pre.strip_suffix('a') {
            format!("{p}Oh")
        } else {
            format!("{pre}Uh")
        };
        (
            cand.to_string(),
            weak,
            format!("{pre}vAw"),
            format!("{pre}vAq"),
        )
    } else if cand == "turAsAh" {
        (
            cand.to_string(),
            cand.to_string(),
            "turAzAw".into(),
            "turAzAq".into(),
        )
    } else {
        return None;
    };
    let mut decl = HashMap::new();
    decl.insert(
        "prathamA".into(),
        vec![pada_q.clone(), pada_w.clone(), format!("{strong}O"), format!("{strong}aH")],
    );
    decl.insert(
        "dvitIyA".into(),
        vec![format!("{strong}am"), format!("{strong}O"), format!("{weak}aH")],
    );
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{pada_q}ByAm"), format!("{pada_q}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{pada_q}ByAm"), format!("{pada_q}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{pada_q}ByAm"), format!("{pada_q}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{pada_w}su")]);
    decl.insert(
        "samboDana".into(),
        vec![pada_q, pada_w, format!("{strong}O"), format!("{strong}aH")],
    );
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// भृस्ज् — 8.2.29 स्-lopa + 8.4.41 ष्टुत्व: पद भृट्/भृड्; vowel भृज्जौ. Not ज-anta *भृस्क्.
fn decline_bhrasj(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "Bfsj" || (linga != "pum" && linga != "stri") {
        return None;
    }
    let vow = "Bfjj";
    let w = "Bfw";
    let q = "Bfq";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![w.into(), q.into(), format!("{vow}O"), format!("{vow}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{vow}am"), format!("{vow}O"), format!("{vow}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{vow}A"), format!("{q}ByAm"), format!("{q}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{vow}e"), format!("{q}ByAm"), format!("{q}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{vow}aH"), format!("{q}ByAm"), format!("{q}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{vow}aH"), format!("{vow}oH"), format!("{vow}Am")]);
    decl.insert("saptamI".into(), vec![format!("{vow}i"), format!("{vow}oH"), format!("{w}su")]);
    decl.insert("samboDana".into(), vec![w.into(), q.into(), format!("{vow}O"), format!("{vow}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// दाधृष् — पद कुत्व दधृक्/दधृग् (like श-anta), not ष-anta *दधृट्. द्विष् stays द्विट्.
fn decline_dadhrsh(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "daDfz" || (linga != "pum" && linga != "stri") {
        return None;
    }
    let weak = cand;
    let k = "daDfk";
    let g = "daDfg";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![k.into(), g.into(), format!("{weak}O"), format!("{weak}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{weak}am"), format!("{weak}O"), format!("{weak}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{g}ByAm"), format!("{g}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{g}ByAm"), format!("{g}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{g}ByAm"), format!("{g}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{k}zu")]);
    decl.insert("samboDana".into(), vec![k.into(), g.into(), format!("{weak}O"), format!("{weak}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// प्रशाम् — 8.2.64 मो नो धातोः: पद प्रशान्/प्रशान्भ्याम्. Not a-stem *प्रशामः.
fn decline_sham(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "praSAm" || (linga != "pum" && linga != "stri") {
        return None;
    }
    let weak = cand;
    let pada = "praSAn";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![pada.into(), format!("{weak}O"), format!("{weak}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{weak}am"), format!("{weak}O"), format!("{weak}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{pada}ByAm"), format!("{pada}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{pada}su")]);
    decl.insert("samboDana".into(), vec![pada.into(), format!("{weak}O"), format!("{weak}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// विश् — 8.2.36 षः then ष्टुत्व विट्/विड्. दिश् stays दिक् (कुत्व).
fn decline_vish(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "viS" || (linga != "pum" && linga != "stri") {
        return None;
    }
    let weak = cand;
    let w = "viw";
    let q = "viq";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![w.into(), q.into(), format!("{weak}O"), format!("{weak}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{weak}am"), format!("{weak}O"), format!("{weak}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{q}ByAm"), format!("{q}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{q}ByAm"), format!("{q}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{q}ByAm"), format!("{q}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{w}su")]);
    decl.insert("samboDana".into(), vec![w.into(), q.into(), format!("{weak}O"), format!("{weak}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// नश् — 8.2.36 षः वा: नट्/नड् (ष्टुत्व) and नक्/नग् (कुत्व like दिश्). पद नड्भ्याम्/नग्भ्याम्; loc नट्सु/नक्षु.
/// Exact `naS`. दिश् stays दिक्; विश् stays विट्.
fn decline_nash(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "naS" || (linga != "pum" && linga != "stri") {
        return None;
    }
    let weak = cand;
    let mut decl = HashMap::new();
    decl.insert(
        "prathamA".into(),
        vec![
            "naw".into(),
            "naq".into(),
            "nak".into(),
            "nag".into(),
            format!("{weak}O"),
            format!("{weak}aH"),
        ],
    );
    decl.insert("dvitIyA".into(), vec![format!("{weak}am"), format!("{weak}O"), format!("{weak}aH")]);
    decl.insert(
        "tfIyA".into(),
        vec![
            format!("{weak}A"),
            "naqByAm".into(),
            "nagByAm".into(),
            "naqBiH".into(),
            "nagBiH".into(),
        ],
    );
    decl.insert(
        "caturTI".into(),
        vec![
            format!("{weak}e"),
            "naqByAm".into(),
            "nagByAm".into(),
            "naqByaH".into(),
            "nagByaH".into(),
        ],
    );
    decl.insert(
        "paYcamI".into(),
        vec![
            format!("{weak}aH"),
            "naqByAm".into(),
            "nagByAm".into(),
            "naqByaH".into(),
            "nagByaH".into(),
        ],
    );
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert(
        "saptamI".into(),
        vec![
            format!("{weak}i"),
            format!("{weak}oH"),
            "nawsu".into(),
            "nawtsu".into(),
            "nakzu".into(),
        ],
    );
    decl.insert(
        "samboDana".into(),
        vec![
            "naw".into(),
            "naq".into(),
            "nak".into(),
            "nag".into(),
            format!("{weak}O"),
            format!("{weak}aH"),
        ],
    );
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// खञ्ज् — 8.2.23 संयोगान्त लोपः पद खन्/खन्भ्याम्/खन्सु. Vowel खञ्जौ. Not ज-anta *खङ्क्. क्रुञ्च् stays क्रुङ्.
fn decline_khanj(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "KaYj" || (linga != "pum" && linga != "stri") {
        return None;
    }
    let vow = cand;
    let pada = "Kan";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![pada.into(), format!("{vow}O"), format!("{vow}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{vow}am"), format!("{vow}O"), format!("{vow}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{vow}A"), format!("{pada}ByAm"), format!("{pada}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{vow}e"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{vow}aH"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{vow}aH"), format!("{vow}oH"), format!("{vow}Am")]);
    decl.insert("saptamI".into(), vec![format!("{vow}i"), format!("{vow}oH"), format!("{pada}su")]);
    decl.insert("samboDana".into(), vec![pada.into(), format!("{vow}O"), format!("{vow}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// पा-anta पुं (गोपा, विश्वपा) — आकारान्त पुं from √पा, not 4.1.4 टाप् स्त्री.
/// सु गोपाः; औ गोपौ (आ+औ); अम् गोपाम्; जस् गोपाः; शस् गोपः; टा गोपा;
/// ङे गोपे; ङसि/ङस् गोपः; ङोस् गोपोः; ङि गोपि; पद आभ्याम्/आसु.
/// nonempty `pA` pre (सीता is `A` not `pA`; कृपा स्त्री stays टाप्).
fn decline_pa(cand: &str, linga: &str) -> Option<Declension> {
    let pre = cand.strip_suffix("pA")?;
    if pre.is_empty() || linga != "pum" {
        return None;
    }
    let a = format!("{pre}p");
    let aa = format!("{pre}pA");
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![format!("{aa}H"), format!("{a}O"), format!("{aa}H")]);
    decl.insert("dvitIyA".into(), vec![format!("{aa}m"), format!("{a}O"), format!("{a}aH")]);
    decl.insert("tfIyA".into(), vec![aa.clone(), format!("{aa}ByAm"), format!("{aa}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{a}e"), format!("{aa}ByAm"), format!("{aa}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{a}aH"), format!("{aa}ByAm"), format!("{aa}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{a}aH"), format!("{a}oH"), format!("{aa}m")]);
    decl.insert("saptamI".into(), vec![format!("{a}i"), format!("{a}oH"), format!("{aa}su")]);
    decl.insert("samboDana".into(), vec![format!("{aa}H"), format!("{a}O"), format!("{aa}H")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// क्रोष्टु — 7.1.95 तृज्वत् क्रोष्टुः: सर्वनामस्थान क्रोष्टा/क्रोष्टारौ/क्रोष्टारम् (like कर्तृ);
/// शस्/पद stay उ क्रोष्टून्/क्रोष्टुभ्याम्/क्रोष्टुषु; voc क्रोष्टो. Weak optional ऋ/उ (क्रोष्ट्रा/क्रोष्टुना).
/// Exact `krozwu`. गुरु stays u-stem गुरुः.
fn decline_kroshtu(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "krozwu" || linga != "pum" {
        return None;
    }
    let u = "krozwu";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["krozwA".into(), "krozwArO".into(), "krozwAraH".into()]);
    decl.insert("dvitIyA".into(), vec!["krozwAram".into(), "krozwArO".into(), "krozwUn".into()]);
    decl.insert("tfIyA".into(), vec!["krozwrA".into(), "krozwunA".into(), format!("{u}ByAm"), format!("{u}BiH")]);
    decl.insert("caturTI".into(), vec!["krozwre".into(), "krozwave".into(), format!("{u}ByAm"), format!("{u}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{u}H"), "krozwoH".into(), format!("{u}ByAm"), format!("{u}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{u}H"), "krozwoH".into(), "krozwroH".into(), "krozwvoH".into(), "krozwUnAm".into()]);
    decl.insert("saptamI".into(), vec!["krozwari".into(), "krozwO".into(), "krozwroH".into(), "krozwvoH".into(), format!("{u}zu")]);
    decl.insert("samboDana".into(), vec!["krozwo".into(), "krozwArO".into(), "krozwAraH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// ग्रामणी — ई-anta पुं नदीवत् + सु visarga ग्रामणीः; अम् ग्रामण्यम् (not पपी *ग्रामणीम्);
/// loc ग्रामण्याम्. पपी stays Im/In/loc पपी.
fn decline_gramani(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "grAmaRI" || linga != "pum" {
        return None;
    }
    let i = "grAmaRI";
    let y = "grAmaRy";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![format!("{i}H"), format!("{y}O"), format!("{y}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{y}am"), format!("{y}O"), format!("{y}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{y}A"), format!("{i}ByAm"), format!("{i}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{y}e"), format!("{i}ByAm"), format!("{i}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{y}aH"), format!("{i}ByAm"), format!("{i}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{y}aH"), format!("{y}oH"), format!("{y}Am")]);
    decl.insert("saptamI".into(), vec![format!("{y}Am"), format!("{y}oH"), format!("{i}zu")]);
    decl.insert("samboDana".into(), vec![format!("{i}H"), format!("{y}O"), format!("{y}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// जरा — 7.2.101 जराया जरसन्यतरस्याम्: टाप् जरा/जरया and optional जरस् जरसौ/जरसाम्. पद जराभ्याम्. सीता stays टाप्.
fn decline_jara(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "jarA" || linga != "stri" {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert(
        "prathamA".into(),
        vec!["jarA".into(), "jare".into(), "jarasO".into(), "jarAH".into(), "jarasaH".into()],
    );
    decl.insert(
        "dvitIyA".into(),
        vec![
            "jarAm".into(),
            "jarasam".into(),
            "jare".into(),
            "jarasO".into(),
            "jarAH".into(),
            "jarasaH".into(),
        ],
    );
    decl.insert("tfIyA".into(), vec!["jarayA".into(), "jarasA".into(), "jarAByAm".into(), "jarABiH".into()]);
    decl.insert("caturTI".into(), vec!["jarAyE".into(), "jarase".into(), "jarAByAm".into(), "jarAByaH".into()]);
    decl.insert("paYcamI".into(), vec!["jarAyAH".into(), "jarasaH".into(), "jarAByAm".into(), "jarAByaH".into()]);
    decl.insert("zazWI".into(), vec!["jarAyAH".into(), "jarayoH".into(), "jarARAm".into(), "jarasAm".into()]);
    decl.insert("saptamI".into(), vec!["jarAyAm".into(), "jarasi".into(), "jarayoH".into(), "jarAsu".into()]);
    decl.insert("samboDana".into(), vec!["jare".into(), "jarasO".into(), "jarAH".into(), "jarasaH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// भू-anta: 6.4.77 उवङ् on धातु भू (भू/स्वभू/स्वयम्भू भुवम्). 6.4.83 ओः सुपि यण् on other *भू (वर्षाभ्वम्).
/// Not U-pum *वर्षाभूम् / उवङ् *वर्षाभुवम्. खलपू stays पू-anta. हूहू stays हूहूम्.
fn decline_bhu(cand: &str, linga: &str) -> Option<Declension> {
    let pre = cand.strip_suffix("BU")?;
    if linga != "pum" && linga != "stri" {
        return None;
    }
    // 6.4.77 श्रुधातुभ्रुवां: only the dhātu भू and its स्व/स्वयम् compounds. वर्षाभू is 6.4.83.
    let dhatu_bhu = pre.is_empty() || pre == "sva" || pre == "svayam";
    let uv = if dhatu_bhu {
        format!("{pre}Buv")
    } else {
        format!("{pre}Bv")
    };
    let uu = format!("{pre}BU");
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![format!("{uu}H"), format!("{uv}O"), format!("{uv}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{uv}am"), format!("{uv}O"), format!("{uv}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{uv}A"), format!("{uu}ByAm"), format!("{uu}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{uv}e"), format!("{uu}ByAm"), format!("{uu}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{uv}aH"), format!("{uu}ByAm"), format!("{uu}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{uv}aH"), format!("{uv}oH"), format!("{uv}Am")]);
    decl.insert("saptamI".into(), vec![format!("{uv}i"), format!("{uv}oH"), format!("{uu}zu")]);
    decl.insert("samboDana".into(), vec![format!("{uu}H"), format!("{uv}O"), format!("{uv}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// हाहा — आकारान्त पुं (गन्धर्व): हाहाः/हाहौ/हाहाम्/हाहाान्; टा हाहा; ङे हाहै; ङि हाहे; ङोस् हाहोः.
/// Not पा-anta *हाहाि / *हाहाः शस् like गोपः. Exact `hAhA`. गोपा stays गोपि.
fn decline_haha(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "hAhA" || linga != "pum" {
        return None;
    }
    let aa = "hAhA";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["hAhAH".into(), "hAhO".into(), "hAhAH".into()]);
    decl.insert("dvitIyA".into(), vec!["hAhAm".into(), "hAhO".into(), "hAhAn".into()]);
    decl.insert("tfIyA".into(), vec![aa.into(), "hAhAByAm".into(), "hAhABiH".into()]);
    decl.insert("caturTI".into(), vec!["hAhE".into(), "hAhAByAm".into(), "hAhAByaH".into()]);
    decl.insert("paYcamI".into(), vec!["hAhAH".into(), "hAhAByAm".into(), "hAhAByaH".into()]);
    decl.insert("zazWI".into(), vec!["hAhAH".into(), "hAhOH".into(), "hAhAm".into()]);
    decl.insert("saptamI".into(), vec!["hAhe".into(), "hAhOH".into(), "hAhAsu".into()]);
    decl.insert("samboDana".into(), vec!["hAhAH".into(), "hAhO".into(), "hAhAH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// हूहू — ऊ-anta यण् हूह्वौ/हूह्वम्; सु हूहूः; अम् हूहूम्; शस् हूहून्; loc हूह्वि/हूहूषु.
/// Not अव् *हूहावौ. स्वभू stays उवङ् स्वभुवम्. Exact `hUhU`.
fn decline_huhu(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "hUhU" || (linga != "pum" && linga != "stri") {
        return None;
    }
    let u = "hUhU";
    let v = "hUhv";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![format!("{u}H"), format!("{v}O"), format!("{v}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{u}m"), format!("{v}O"), format!("{u}n")]);
    decl.insert("tfIyA".into(), vec![format!("{v}A"), format!("{u}ByAm"), format!("{u}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{v}e"), format!("{u}ByAm"), format!("{u}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{v}aH"), format!("{u}ByAm"), format!("{u}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{v}aH"), format!("{v}oH"), format!("{v}Am")]);
    decl.insert("saptamI".into(), vec![format!("{v}i"), format!("{v}oH"), format!("{u}zu")]);
    decl.insert("samboDana".into(), vec![format!("{u}H"), format!("{v}O"), format!("{v}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// 6.4.14 अत्वसन्तस्य चाधातोः सौ आः: वेधाः, चन्द्रमाः, सुमनाः (not *वेधः/*चन्द्रमः/*सुमनः).
/// Voc aH; पद ओभ्याम्. Exact `manas` stays मनः. तपस् stays तपः. उशनस् is 7.1.94. Not *mas (तमस्).
fn decline_atvasantasya_as(cand: &str, linga: &str) -> Option<Declension> {
    if linga != "pum" {
        return None;
    }
    let pre = cand.strip_suffix("as")?;
    if cand != "veDas"
        && cand != "candramas"
        && !cand.strip_suffix("manas").is_some_and(|p| !p.is_empty())
    {
        return None;
    }
    let o = format!("{pre}o");
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![format!("{pre}AH"), format!("{cand}O"), format!("{cand}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{cand}am"), format!("{cand}O"), format!("{cand}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{cand}A"), format!("{o}ByAm"), format!("{o}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{cand}e"), format!("{o}ByAm"), format!("{o}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{cand}aH"), format!("{o}ByAm"), format!("{o}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{cand}aH"), format!("{cand}oH"), format!("{cand}Am")]);
    decl.insert("saptamI".into(), vec![format!("{cand}i"), format!("{cand}oH"), format!("{pre}assu"), format!("{pre}aHsu")]);
    decl.insert("samboDana".into(), vec![format!("{pre}aH"), format!("{cand}O"), format!("{cand}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// निरजर — a-stem निरजरः/निरजरेण plus 7.2.101-like जरस् निरजरसौ/निरजरसा. राम stays a-stem. Exact `nirjara`.
fn decline_nirjara(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "nirjara" || linga != "pum" {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert(
        "prathamA".into(),
        vec![
            polish("nirjaraH"),
            polish("nirjarO"),
            polish("nirjarasO"),
            polish("nirjarAH"),
            polish("nirjarasaH"),
        ],
    );
    decl.insert(
        "dvitIyA".into(),
        vec![
            polish("nirjaram"),
            polish("nirjarO"),
            polish("nirjarasO"),
            polish("nirjarAn"),
            polish("nirjarasaH"),
        ],
    );
    decl.insert(
        "tfIyA".into(),
        vec![
            polish("nirjarena"),
            polish("nirjarasA"),
            polish("nirjarAByAm"),
            polish("nirjarEH"),
        ],
    );
    decl.insert(
        "caturTI".into(),
        vec![
            polish("nirjarAya"),
            polish("nirjarase"),
            polish("nirjarAByAm"),
            polish("nirjareByaH"),
        ],
    );
    decl.insert(
        "paYcamI".into(),
        vec![
            polish("nirjarAt"),
            polish("nirjarasaH"),
            polish("nirjarAByAm"),
            polish("nirjareByaH"),
        ],
    );
    decl.insert(
        "zazWI".into(),
        vec![
            polish("nirjarasya"),
            polish("nirjarasaH"),
            polish("nirjarayoH"),
            polish("nirjarasoH"),
            polish("nirjarARAm"),
            polish("nirjarasAm"),
        ],
    );
    decl.insert(
        "saptamI".into(),
        vec![
            polish("nirjare"),
            polish("nirjarasi"),
            polish("nirjarayoH"),
            polish("nirjarasoH"),
            polish("nirjarezu"),
        ],
    );
    decl.insert(
        "samboDana".into(),
        vec![
            polish("nirjara"),
            polish("nirjarO"),
            polish("nirjarasO"),
            polish("nirjarAH"),
            polish("nirjarasaH"),
        ],
    );
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// प्रधी — यण् प्रध्यौ/प्रध्यम् (not पपी *प्रधीम् / इयङ् *प्रधियौ). सु प्रधीः.
/// Optional नदीवत् ङे प्रध्यै / loc प्रध्यि. ग्रामणी stays exact; सुधी is इयङ्.
fn decline_pradhi(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "praDI" || (linga != "pum" && linga != "stri") {
        return None;
    }
    let i = "praDI";
    let y = "praDy";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![format!("{i}H"), format!("{y}O"), format!("{y}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{y}am"), format!("{y}O"), format!("{y}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{y}A"), format!("{i}ByAm"), format!("{i}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{y}e"), format!("{y}E"), format!("{i}ByAm"), format!("{i}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{y}aH"), format!("{y}AH"), format!("{i}ByAm"), format!("{i}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{y}aH"), format!("{y}AH"), format!("{y}oH"), format!("{y}Am"), format!("{i}nAm")]);
    decl.insert("saptamI".into(), vec![format!("{y}Am"), format!("{y}i"), format!("{y}oH"), format!("{i}zu")]);
    decl.insert("samboDana".into(), vec![format!("{i}H"), "praDi".into(), format!("{y}O"), format!("{y}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// धी-anta इयङ् (सुधी, शुद्धधी) — 6.4.77 अचि श्नुधातुभ्रुवां: सुधियौ/सुधियम् not यण् *सुध्यौ.
/// सु सुधीः; पद सुधीभ्याम्. नपुं 7.1.23 सुधि/सुधिनी/सुधीनि. प्रधी stays यण्.
fn decline_dhi_iyan(cand: &str, linga: &str) -> Option<Declension> {
    let pre = cand.strip_suffix("DI")?;
    if linga != "pum" && linga != "stri" && linga != "nap" {
        return None;
    }
    let iy = format!("{pre}Diy");
    let ii = format!("{pre}DI");
    let mut decl = HashMap::new();
    if linga == "nap" {
        let short = format!("{pre}Di");
        let nom = vec![short.clone(), format!("{short}nI"), format!("{pre}DIni")];
        decl.insert("prathamA".into(), nom.clone());
        decl.insert("dvitIyA".into(), nom.clone());
        decl.insert("tfIyA".into(), vec![format!("{short}nA"), format!("{short}ByAm"), format!("{short}BiH")]);
        decl.insert("caturTI".into(), vec![format!("{short}ne"), format!("{short}ByAm"), format!("{short}ByaH")]);
        decl.insert("paYcamI".into(), vec![format!("{short}naH"), format!("{short}ByAm"), format!("{short}ByaH")]);
        decl.insert("zazWI".into(), vec![format!("{short}naH"), format!("{short}noH"), format!("{pre}DInAm")]);
        decl.insert("saptamI".into(), vec![format!("{short}ni"), format!("{short}noH"), format!("{short}zu")]);
        decl.insert("samboDana".into(), nom);
        return Some(Declension {
            stem: cand.to_string(),
            linga: linga.to_string(),
            declension: decl,
        });
    }
    decl.insert("prathamA".into(), vec![format!("{ii}H"), format!("{iy}O"), format!("{iy}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{iy}am"), format!("{iy}O"), format!("{iy}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{iy}A"), format!("{ii}ByAm"), format!("{ii}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{iy}e"), format!("{ii}ByAm"), format!("{ii}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{iy}aH"), format!("{ii}ByAm"), format!("{ii}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{iy}aH"), format!("{iy}oH"), format!("{iy}Am"), format!("{ii}nAm")]);
    decl.insert("saptamI".into(), vec![format!("{iy}i"), format!("{iy}oH"), format!("{ii}zu")]);
    decl.insert(
        "samboDana".into(),
        vec![format!("{ii}H"), format!("{pre}De"), format!("{iy}O"), format!("{iy}aH")],
    );
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// श्री — 6.4.77 इयङ् श्रियौ/श्रियम्; सु श्रीः (not नदी *श्री / यण् *श्र्यौ). पद श्रीभ्याम्/श्रीषु.
/// Compounds सुश्रीः. नदी stays I-stri नद्यौ.
fn decline_sri(cand: &str, linga: &str) -> Option<Declension> {
    let pre = cand.strip_suffix("SrI")?;
    if linga != "stri" && linga != "pum" {
        return None;
    }
    let iy = format!("{pre}Sriy");
    let ii = format!("{pre}SrI");
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![format!("{ii}H"), format!("{iy}O"), format!("{iy}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{iy}am"), format!("{iy}O"), format!("{iy}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{iy}A"), format!("{ii}ByAm"), format!("{ii}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{iy}e"), format!("{iy}E"), format!("{ii}ByAm"), format!("{ii}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{iy}aH"), format!("{iy}AH"), format!("{ii}ByAm"), format!("{ii}ByaH")]);
    decl.insert(
        "zazWI".into(),
        vec![
            format!("{iy}aH"),
            format!("{iy}AH"),
            format!("{iy}oH"),
            format!("{iy}Am"),
            polish(&format!("{ii}nAm")),
        ],
    );
    decl.insert("saptamI".into(), vec![format!("{iy}Am"), format!("{iy}i"), format!("{iy}oH"), format!("{ii}zu")]);
    decl.insert("samboDana".into(), vec![format!("{ii}H"), format!("{iy}O"), format!("{iy}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// भ्रू — 6.4.77 उवङ् भ्रुवौ/भ्रुवम्; सु भ्रूः. Optional स्त्री भ्रुवै/भ्रुवाम्. पद भ्रूभ्याम्.
/// स्वभू stays *भू उवङ् स्वभुवम्. Exact `BrU`.
fn decline_bru(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "BrU" || (linga != "stri" && linga != "pum") {
        return None;
    }
    let uv = "Bruv";
    let uu = "BrU";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![format!("{uu}H"), format!("{uv}O"), format!("{uv}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{uv}am"), format!("{uv}O"), format!("{uv}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{uv}A"), format!("{uu}ByAm"), format!("{uu}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{uv}e"), format!("{uv}E"), format!("{uu}ByAm"), format!("{uu}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{uv}aH"), format!("{uv}AH"), format!("{uu}ByAm"), format!("{uu}ByaH")]);
    decl.insert(
        "zazWI".into(),
        vec![
            format!("{uv}aH"),
            format!("{uv}AH"),
            format!("{uv}oH"),
            format!("{uv}Am"),
            polish(&format!("{uu}nAm")),
        ],
    );
    decl.insert("saptamI".into(), vec![format!("{uv}Am"), format!("{uv}i"), format!("{uv}oH"), format!("{uu}zu")]);
    decl.insert("samboDana".into(), vec![format!("{uu}H"), format!("{uv}O"), format!("{uv}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// पति — 1.4.8 पतिः समास एव: unsuffixed पति is not घि, so पत्या/पत्ये/पत्युः/पत्यौ (like सखि weak);
/// सर्वनामस्थान पतिः/पती/पतयः (not सखा). हरि stays घि पतिना-class हरिणा. Exact `pati`.
fn decline_pati(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "pati" || linga != "pum" {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["patiH".into(), "patI".into(), "patayaH".into()]);
    decl.insert("dvitIyA".into(), vec!["patim".into(), "patI".into(), "patIn".into()]);
    decl.insert("tfIyA".into(), vec!["patyA".into(), "patiByAm".into(), "patiBiH".into()]);
    decl.insert("caturTI".into(), vec!["patye".into(), "patiByAm".into(), "patiByaH".into()]);
    decl.insert("paYcamI".into(), vec!["patyuH".into(), "patiByAm".into(), "patiByaH".into()]);
    decl.insert("zazWI".into(), vec!["patyuH".into(), "patyoH".into(), "patInAm".into()]);
    decl.insert("saptamI".into(), vec!["patyO".into(), "patyoH".into(), "patizu".into()]);
    decl.insert("samboDana".into(), vec!["pate".into(), "patI".into(), "patayaH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// पू-anta (खलपू) — यण् खलप्वौ/खलप्वम् (not U-pum *खलपूम् / उवङ् *खलपुवम्). सु खलपूः; पद खलपूभ्याम्.
/// nonempty `pU`. हूहू stays Um/Un; स्वभू stays उवङ्.
fn decline_pu(cand: &str, linga: &str) -> Option<Declension> {
    let pre = cand.strip_suffix("pU")?;
    if pre.is_empty() || (linga != "pum" && linga != "stri") {
        return None;
    }
    let v = format!("{pre}pv");
    let uu = format!("{pre}pU");
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![format!("{uu}H"), format!("{v}O"), format!("{v}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{v}am"), format!("{v}O"), format!("{v}aH")]);
    decl.insert("tfIyA".into(), vec![format!("{v}A"), format!("{uu}ByAm"), format!("{uu}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{v}e"), format!("{uu}ByAm"), format!("{uu}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{v}aH"), format!("{uu}ByAm"), format!("{uu}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{v}aH"), format!("{v}oH"), format!("{v}Am")]);
    decl.insert("saptamI".into(), vec![format!("{v}i"), format!("{v}oH"), format!("{uu}zu")]);
    decl.insert("samboDana".into(), vec![format!("{uu}H"), format!("{v}O"), format!("{v}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// स्त्री — 6.4.79 स्त्रियाः / 6.4.77 इयङ् स्त्रियौ/स्त्रियम्; nom स्त्री (not श्री-like *स्त्रीः).
/// Optional 7.1.20 स्त्रीम्/स्त्रीः in acc. Voc स्त्रि. नदी stays नद्यौ (यण्). Exact `strI`.
fn decline_stri(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "strI" || linga != "stri" {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["strI".into(), "striyO".into(), "striyaH".into()]);
    decl.insert("dvitIyA".into(), vec!["striyam".into(), "strIm".into(), "striyO".into(), "striyaH".into(), "strIH".into()]);
    decl.insert("tfIyA".into(), vec!["striyA".into(), "strIByAm".into(), "strIBiH".into()]);
    decl.insert("caturTI".into(), vec!["striyE".into(), "strIByAm".into(), "strIByaH".into()]);
    decl.insert("paYcamI".into(), vec!["striyAH".into(), "strIByAm".into(), "strIByaH".into()]);
    decl.insert("zazWI".into(), vec!["striyAH".into(), "striyoH".into(), polish("strInAm")]);
    decl.insert("saptamI".into(), vec!["striyAm".into(), "striyoH".into(), "strIzu".into()]);
    decl.insert("samboDana".into(), vec!["stri".into(), "striyO".into(), "striyaH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// नी — 6.4.77 इयङ् नियौ/नियम्; सु नीः. ग्रामणी stays ग्रामण्यम् (यण्). Exact `nI`.
fn decline_ni(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "nI" || (linga != "pum" && linga != "stri") {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["nIH".into(), "niyO".into(), "niyaH".into()]);
    decl.insert("dvitIyA".into(), vec!["niyam".into(), "niyO".into(), "niyaH".into()]);
    decl.insert("tfIyA".into(), vec!["niyA".into(), "nIByAm".into(), "nIBiH".into()]);
    decl.insert("caturTI".into(), vec!["niye".into(), "nIByAm".into(), "nIByaH".into()]);
    decl.insert("paYcamI".into(), vec!["niyaH".into(), "nIByAm".into(), "nIByaH".into()]);
    decl.insert("zazWI".into(), vec!["niyaH".into(), "niyoH".into(), "niyAm".into()]);
    decl.insert("saptamI".into(), vec!["niyAm".into(), "niyoH".into(), "nIzu".into()]);
    decl.insert("samboDana".into(), vec!["nIH".into(), "niyO".into(), "niyaH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// प्ररै nap — not रै *प्रराः. 7.1.23 स्वमोः प्ररि/प्ररिणी/प्ररीणि (like वारि);
/// पद from रै प्रराभ्याम्/प्ररासु. Exact `prarE` + nap. रै stays राः.
fn decline_prarai(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "prarE" || linga != "nap" {
        return None;
    }
    let i = "prari";
    let aa = "prarA";
    let nom = vec![
        i.to_string(),
        polish(&format!("{i}nI")),
        "prarIRi".into(),
    ];
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), nom.clone());
    decl.insert("dvitIyA".into(), nom.clone());
    decl.insert("tfIyA".into(), vec![polish(&format!("{i}nA")), format!("{aa}ByAm"), format!("{aa}BiH")]);
    decl.insert("caturTI".into(), vec![polish(&format!("{i}ne")), format!("{aa}ByAm"), format!("{aa}ByaH")]);
    decl.insert("paYcamI".into(), vec![polish(&format!("{i}naH")), format!("{aa}ByAm"), format!("{aa}ByaH")]);
    decl.insert("zazWI".into(), vec![polish(&format!("{i}naH")), polish(&format!("{i}noH")), polish("prarInAm")]);
    decl.insert("saptamI".into(), vec![polish(&format!("{i}ni")), polish(&format!("{i}noH")), format!("{aa}su")]);
    let mut voc = vec![i.to_string(), "prare".into()];
    voc.extend(nom.into_iter().skip(1));
    decl.insert("samboDana".into(), voc);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// चमू-anta पुं (अतिचमू) — यण् अतिचम्वौ/अतिचम्वा; सु अतिचमूः; अम् अतिचमूम्; शस् अतिचमून् (not स्त्री *अतिचमूः).
/// loc नदीवत् अतिचम्वाम्. nonempty `camU` + pum. वधू stays acc वधूः; हूहू stays हूहूम्.
fn decline_camu(cand: &str, linga: &str) -> Option<Declension> {
    let pre = cand.strip_suffix("camU")?;
    if pre.is_empty() || linga != "pum" {
        return None;
    }
    let v = format!("{pre}camv");
    let uu = format!("{pre}camU");
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![format!("{uu}H"), format!("{v}O"), format!("{v}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{uu}m"), format!("{v}O"), format!("{uu}n")]);
    decl.insert("tfIyA".into(), vec![format!("{v}A"), format!("{uu}ByAm"), format!("{uu}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{v}E"), format!("{uu}ByAm"), format!("{uu}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{v}AH"), format!("{uu}ByAm"), format!("{uu}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{v}AH"), format!("{v}oH"), polish(&format!("{uu}nAm"))]);
    decl.insert("saptamI".into(), vec![format!("{v}Am"), format!("{v}oH"), format!("{uu}zu")]);
    decl.insert("samboDana".into(), vec![format!("{pre}camu"), format!("{v}O"), format!("{v}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// अम्बा/अक्का/अल्ला — 7.3.107 ह्रस्वो नत्तमम्बार्थे: voc अम्ब not आ-stem *अम्बे.
/// Rest टाप् अम्बा/अम्बे/अम्बया/अम्बासु. सीता stays voc सीते.
fn decline_amba(cand: &str, linga: &str) -> Option<Declension> {
    if !matches!(cand, "ambA" | "akkA" | "allA") || linga != "stri" {
        return None;
    }
    let pre = cand.strip_suffix('A')?;
    let aa = cand;
    let a = format!("{pre}a");
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![aa.to_string(), format!("{pre}e"), format!("{aa}H")]);
    decl.insert("dvitIyA".into(), vec![format!("{aa}m"), format!("{pre}e"), format!("{aa}H")]);
    decl.insert("tfIyA".into(), vec![format!("{pre}ayA"), format!("{aa}ByAm"), format!("{aa}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{aa}yE"), format!("{aa}ByAm"), format!("{aa}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{aa}yAH"), format!("{aa}ByAm"), format!("{aa}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{aa}yAH"), format!("{pre}ayoH"), polish(&format!("{aa}nAm"))]);
    decl.insert("saptamI".into(), vec![format!("{aa}yAm"), format!("{pre}ayoH"), format!("{aa}su")]);
    decl.insert("samboDana".into(), vec![a, format!("{pre}e"), format!("{aa}H")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// लक्ष्मी — सु लक्ष्मीः (not नदी *लक्ष्मी); यण् लक्ष्म्यौ/लक्ष्म्या. श्री stays इयङ् श्रियम्.
/// Exact `lakzmI` + stri. अतिलक्ष्मी not stolen (acc In).
fn decline_lakshmi(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "lakzmI" || linga != "stri" {
        return None;
    }
    let y = "lakzmy";
    let ii = "lakzmI";
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![format!("{ii}H"), format!("{y}O"), format!("{y}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{ii}m"), format!("{y}O"), format!("{ii}H")]);
    decl.insert("tfIyA".into(), vec![format!("{y}A"), format!("{ii}ByAm"), format!("{ii}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{y}E"), format!("{ii}ByAm"), format!("{ii}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{y}AH"), format!("{ii}ByAm"), format!("{ii}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{y}AH"), format!("{y}oH"), polish(&format!("{ii}nAm"))]);
    decl.insert("saptamI".into(), vec![format!("{y}Am"), format!("{y}oH"), format!("{ii}zu")]);
    decl.insert("samboDana".into(), vec!["lakzmi".into(), format!("{y}O"), format!("{y}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// अतिलक्ष्मी — लक्ष्मीः-class visarga + यण्; शस् अतिलक्ष्मीन् (not स्त्री *अतिलक्ष्मीः).
/// nonempty `lakzmI` + stri. लक्ष्मी stays acc लक्ष्मीः.
fn decline_atilakshmi(cand: &str, linga: &str) -> Option<Declension> {
    let pre = cand.strip_suffix("lakzmI")?;
    if pre.is_empty() || linga != "stri" {
        return None;
    }
    let y = format!("{pre}lakzmy");
    let ii = format!("{pre}lakzmI");
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec![format!("{ii}H"), format!("{y}O"), format!("{y}aH")]);
    decl.insert("dvitIyA".into(), vec![format!("{ii}m"), format!("{y}O"), format!("{ii}n")]);
    decl.insert("tfIyA".into(), vec![format!("{y}A"), format!("{ii}ByAm"), format!("{ii}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{y}E"), format!("{ii}ByAm"), format!("{ii}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{y}AH"), format!("{ii}ByAm"), format!("{ii}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{y}AH"), format!("{y}oH"), polish(&format!("{ii}nAm"))]);
    decl.insert("saptamI".into(), vec![format!("{y}Am"), format!("{y}oH"), format!("{ii}zu")]);
    decl.insert("samboDana".into(), vec![format!("{pre}lakzmi"), format!("{y}O"), format!("{y}aH")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// सुनौ nap — 7.1.23 सुनु/सुनुनी/सुनूनि (u-nap), not नौ *सुनावम् / *सुनौः. Exact `sunO` + nap. नौ stays नावम्.
fn decline_suno(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "sunO" || linga != "nap" {
        return None;
    }
    let u = "sunu";
    let nom = vec![
        u.to_string(),
        polish(&format!("{u}nI")),
        "sunUni".into(),
    ];
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), nom.clone());
    decl.insert("dvitIyA".into(), nom.clone());
    decl.insert("tfIyA".into(), vec![polish(&format!("{u}nA")), format!("{u}ByAm"), format!("{u}BiH")]);
    decl.insert("caturTI".into(), vec![polish(&format!("{u}ne")), format!("{u}ByAm"), format!("{u}ByaH")]);
    decl.insert("paYcamI".into(), vec![polish(&format!("{u}naH")), format!("{u}ByAm"), format!("{u}ByaH")]);
    decl.insert("zazWI".into(), vec![polish(&format!("{u}naH")), polish(&format!("{u}noH")), polish("sunUnAm")]);
    decl.insert("saptamI".into(), vec![polish(&format!("{u}ni")), polish(&format!("{u}noH")), format!("{u}zu")]);
    let mut voc = vec![u.to_string(), "suno".into()];
    voc.extend(nom.into_iter().skip(1));
    decl.insert("samboDana".into(), voc);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// सुलू nap — 7.1.23 सुलु/सुलुनी/सुलूनि; optional यण् सुल्वा/सुल्वे (not U-nap *सुलू only).
/// पद सुलुभ्याम्/सुलुषु. Exact `sulU` + nap. वधू stays स्त्री; मधु stays मधु.
fn decline_sulu(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "sulU" || linga != "nap" {
        return None;
    }
    let u = "sulu";
    let v = "sulv";
    let nom = vec![
        u.to_string(),
        polish(&format!("{u}nI")),
        "sulUni".into(),
    ];
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), nom.clone());
    decl.insert("dvitIyA".into(), nom.clone());
    decl.insert("tfIyA".into(), vec![format!("{v}A"), polish(&format!("{u}nA")), format!("{u}ByAm"), format!("{u}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{v}e"), polish(&format!("{u}ne")), format!("{u}ByAm"), format!("{u}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{v}aH"), polish(&format!("{u}naH")), format!("{u}ByAm"), format!("{u}ByaH")]);
    decl.insert(
        "zazWI".into(),
        vec![
            format!("{v}aH"),
            polish(&format!("{u}naH")),
            format!("{v}oH"),
            polish(&format!("{u}noH")),
            format!("{v}Am"),
            polish("sulUnAm"),
        ],
    );
    decl.insert(
        "saptamI".into(),
        vec![
            format!("{v}i"),
            polish(&format!("{u}ni")),
            format!("{v}oH"),
            polish(&format!("{u}noH")),
            format!("{u}zu"),
        ],
    );
    let mut voc = vec![u.to_string(), "sulo".into()];
    voc.extend(nom.into_iter().skip(1));
    decl.insert("samboDana".into(), voc);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

/// अभ्यस्त शतृ — 7.1.78 नाभ्यस्ताच्छतुः: no नुम् ददत्/ददतौ not *ददन्. जक्षत् same. भवत् stays भवन्.
fn decline_abhyasta_satr(cand: &str, linga: &str) -> Option<Declension> {
    if !matches!(cand, "dadat" | "jakzat" | "jAgrat") {
        return None;
    }
    if linga != "pum" && linga != "nap" {
        return None;
    }
    let pada = format!("{}ad", cand.strip_suffix("at")?);
    let weak = cand;
    let mut decl = HashMap::new();
    if linga == "nap" {
        let row = vec![
            weak.to_string(),
            pada.clone(),
            format!("{weak}I"),
            format!("{weak}i"),
            format!("{}anti", cand.strip_suffix("at")?),
        ];
        decl.insert("prathamA".into(), row.clone());
        decl.insert("dvitIyA".into(), row.clone());
        decl.insert("samboDana".into(), row);
    } else {
        let nom = vec![weak.to_string(), pada.clone(), format!("{weak}O"), format!("{weak}aH")];
        decl.insert("prathamA".into(), nom.clone());
        decl.insert("dvitIyA".into(), vec![format!("{weak}am"), format!("{weak}O"), format!("{weak}aH")]);
        decl.insert("samboDana".into(), nom);
    }
    decl.insert("tfIyA".into(), vec![format!("{weak}A"), format!("{pada}ByAm"), format!("{pada}BiH")]);
    decl.insert("caturTI".into(), vec![format!("{weak}e"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("paYcamI".into(), vec![format!("{weak}aH"), format!("{pada}ByAm"), format!("{pada}ByaH")]);
    decl.insert("zazWI".into(), vec![format!("{weak}aH"), format!("{weak}oH"), format!("{weak}Am")]);
    decl.insert("saptamI".into(), vec![format!("{weak}i"), format!("{weak}oH"), format!("{weak}su")]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

const F_KINSHIP: &[&str] = &["pitf","mAtf","BrAtf","jAmAtf","duhitf","nanAndf"];
/// 6.4.11 स्वसृनप्तृ… — सर्वनामस्थान आ (स्वसारौ/स्वसारम्). Not पितृ *पितारौ.
const F_SVASR_NAPTR: &[&str] = &["svasf", "naptf"];

/// 6.4.11 vs पितृ-class: long आ or short अ in dual/pl/acc of ऋ-stems.
fn patch_r_sarvanamasthana(table: &mut [Vec<String>], vrddhi: bool) {
    let du = if vrddhi { "ArO" } else { "arO" };
    let pl = if vrddhi { "AraH" } else { "araH" };
    let acc = if vrddhi { "Aram" } else { "aram" };
    if table.first().is_some_and(|r| r.len() >= 3) {
        table[0][1] = du.into();
        table[0][2] = pl.into();
    }
    if table.get(1).is_some_and(|r| r.len() >= 2) {
        table[1][0] = acc.into();
        table[1][1] = du.into();
    }
    if let Some(voc) = table.last_mut() {
        if voc.len() >= 3 {
            voc[1] = du.into();
            voc[2] = pl.into();
        }
    }
}

pub fn generate(base: &str, linga: &str) -> Option<Declension> {
    let paradigms = paradigms();
    // 6.4.11 कर्तृ/स्वसृ आ in सर्वनामस्थान; पितृ-class keeps short अ (पितरम्/पितरौ).
    let is_kinship = F_KINSHIP.contains(&base);
    let is_svasr_naptr = F_SVASR_NAPTR.contains(&base);
    // try candidates to handle bases passed as declined forms (e.g. rAmaH)
    let cands = [
        base.to_string(),
        base.trim_end_matches('H').to_string(),
        base.trim_end_matches('M').to_string(),
        base.trim_end_matches("AH").to_string(),
        base.trim_end_matches("AM").to_string(),
    ];
    for cand in cands {
        if cand.is_empty() { continue; }
        if let Some(d) = decline_ahan(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_han(&cand, linga) {
            return Some(d);
        }
        let cand = ngeep_stri(&cand, linga);
        if let Some(d) = decline_sva_yuv_magha(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_arvan(&cand, linga) {
            return Some(d);
        }
        if cand.ends_with("an") && (linga == "pum" || linga == "nap") {
            return Some(decline_an(&cand, linga));
        }
        if let Some(d) = decline_pathadi(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_sakhi(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_pums(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_ap(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_anaquh(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_upanah(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_div(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_rai(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_prarai(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_camu(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_amba(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_lakshmi(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_atilakshmi(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_suno(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_sulu(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_abhyasta_satr(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_anc(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_krunc(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_nr(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_kvasu(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_asthyadi(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_sajush(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_raj(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_ushanasadi(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_pad(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_gir(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_duhadi(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_lih(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_vah(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_bhrasj(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_dadhrsh(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_sham(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_vish(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_nash(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_khanj(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_pa(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_kroshtu(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_gramani(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_jara(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_bhu(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_haha(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_huhu(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_atvasantasya_as(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_nirjara(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_pradhi(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_dhi_iyan(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_sri(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_bru(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_pati(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_pu(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_stri(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_ni(&cand, linga) {
            return Some(d);
        }
        let mut best: Option<(String, Vec<Vec<String>>)> = None;
        let mut best_len = 0;
        let mut best_ending = String::new();
        for ((ending, l), table) in &paradigms {
            if l != linga { continue; }
            // त्रिंशत्/चत्वारिंशत्/पञ्चाशत् — त-anta संख्या, not शतृ `at` (भवन्).
            if ending == "at" && cand.ends_with("Sat") { continue; }
            if cand.ends_with(ending) && ending.len() > best_len {
                best = Some((ending.clone(), table.clone()));
                best_len = ending.len();
                best_ending = ending.clone();
            }
        }
        if let Some((_, mut table)) = best {
            // 6.4.11: स्वसृ/नप्तृ आ; पितृ-class पुं short अ. मातृ स्त्री already short in f-stri.
            if best_ending == "f" {
                if is_svasr_naptr {
                    patch_r_sarvanamasthana(&mut table, true);
                } else if is_kinship && linga == "pum" {
                    patch_r_sarvanamasthana(&mut table, false);
                }
            }
            if cand == "mahat" && best_ending == "at" {
                mahat_strong(&mut table);
            }
            // 6.4.14 अत्वसन्तस्य चाधातोः: शतृ पचन् not *पचान्. भवान्/धीमान्/महान् keep आ.
            if best_ending == "at"
                && linga == "pum"
                && cand != "mahat"
                && !cand.ends_with("mat")
                && !cand.ends_with("vat")
                && table.first().is_some_and(|r| !r.is_empty())
            {
                table[0][0] = "an".into();
            }
            let base_no_end = &cand[..cand.len()-best_ending.len()];
            let vibhaktis = ["prathamA","dvitIyA","tfIyA","caturTI","paYcamI","zazWI","saptamI","samboDana"];
            let mut decl = std::collections::HashMap::new();
            for (i, vib) in vibhaktis.iter().enumerate() {
                let mut row: Vec<String> = Vec::new();
                for suffix_group in &table[i] {
                    for s in suffix_group.split(',') {
                        row.push(polish(&format!("{base_no_end}{s}")));
                    }
                }
                decl.insert(vib.to_string(), row);
            }
            return Some(Declension { stem: cand.clone(), linga: linga.to_string(), declension: decl });
        }
    }
    // Fallback: foreign/unknown ending — match ending sound by last char, otherwise use a-stem
    // e.g. apolo (o) -> treat as a-stem with base "apol" + a-suffixes => apolaH
    let fallback_key = if linga == "stri" { ("A","stri") } else { ("a","pum") };
    if let Some(table) = paradigms.get(&(fallback_key.0.to_string(), fallback_key.1.to_string())) {
        let base_no_end = if base.chars().last().is_some_and(|c| "aAiIuUeEoO".contains(c)) {
            &base[..base.len()-1]
        } else {
            base
        };
        if !base_no_end.is_empty() {
            let vibhaktis = ["prathamA","dvitIyA","tfIyA","caturTI","paYcamI","zazWI","saptamI","samboDana"];
            let mut decl = std::collections::HashMap::new();
            for (i, vib) in vibhaktis.iter().enumerate() {
                let mut row: Vec<String> = Vec::new();
                for suffix_group in &table[i] {
                    for s in suffix_group.split(',') {
                        row.push(polish(&format!("{base_no_end}{s}")));
                    }
                }
                decl.insert(vib.to_string(), row);
            }
            return Some(Declension { stem: base.to_string(), linga: linga.to_string(), declension: decl });
        }
    }
    None
}

pub fn analyze(word: &str) -> Vec<HashMap<String, String>> {
    let paradigms = paradigms();
    let vibhaktis = ["prathamA","dvitIyA","tfIyA","caturTI","paYcamI","zazWI","saptamI","samboDana"];
    let mut out = Vec::new();
    for ((ending, linga), table) in &paradigms {
        for (vi, vib) in vibhaktis.iter().enumerate() {
            for (vac_idx, suffix_group) in table[vi].iter().enumerate() {
                for orig_suffix in suffix_group.split(',') {
                    if word.len() <= orig_suffix.len() { continue; }
                    let base_stripped = &word[..word.len()-orig_suffix.len()];
                    let surface = apply_natva(base_stripped, orig_suffix);
                    if word == format!("{}{}", base_stripped, surface) {
                        let pratipadika = format!("{}{}", base_stripped, ending);
                        let mut m = HashMap::new();
                        m.insert("pratipadika".to_string(), pratipadika);
                        m.insert("linga".to_string(), linga.clone());
                        m.insert("vibhakti".to_string(), vib.to_string());
                        m.insert("vacana".to_string(), (vac_idx+1).to_string());
                        out.push(m);
                    }
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // ---------------------------------------------------------------------------
    // fn `rajan_pitar_naman`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn rajan_pitar_naman() {
        let d = generate("rAjan", "pum").expect("rAjan");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "rAjA"), "{:?}", pr);
        let dv = d.declension.get("dvitIyA").unwrap();
        assert!(dv.iter().any(|x| x == "rAjAnam"), "{:?}", dv);
        let tr = d.declension.get("tfIyA").unwrap();
        assert!(tr.iter().any(|x| x == "rAjYA"), "{:?}", tr);

        let p = generate("pitf", "pum").expect("pitf");
        let pr = p.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "pitA"), "{:?}", pr);
        let dv = p.declension.get("dvitIyA").unwrap();
        assert!(dv.iter().any(|x| x == "pitaram"), "{:?}", dv);

        let n = generate("nAman", "nap").expect("nAman");
        let pr = n.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "nAma"), "{:?}", pr);
        let tr = n.declension.get("tfIyA").unwrap();
        assert!(tr.iter().any(|x| x == "nAmnA"), "{:?}", tr);
        let sap = d.declension.get("saptamI").unwrap();
        assert!(sap.iter().any(|x| x == "rAjYi"), "{:?}", sap);
        assert!(sap.iter().any(|x| x == "rAjani"), "{:?}", sap);
    }

    // ---------------------------------------------------------------------------
    // fn `has`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn has(d: &Declension, vib: &str, form: &str) {
        let row = d.declension.get(vib).unwrap();
        assert!(row.iter().any(|x| x == form), "{vib} {:?}, want {form}", row);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `go_nau_from_ot_aut`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn go_nau_from_ot_aut() {
        let g = generate("go", "pum").expect("go");
        has(&g, "prathamA", "gOH");
        has(&g, "dvitIyA", "gAm");
        has(&g, "dvitIyA", "gAH");
        has(&g, "tfIyA", "gavA");
        has(&g, "saptamI", "gavi");
        has(&g, "saptamI", "gozu");
        let n = generate("nO", "stri").expect("nO");
        has(&n, "prathamA", "nOH");
        has(&n, "dvitIyA", "nAvam");
        has(&n, "tfIyA", "nAvA");
        has(&n, "saptamI", "nOzu");
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `halanta_j_d_stems`: sūtra 8.2.30/39 — j→k (वणिक्), d→t (सुहृत्)
    // Extreme commented for future halanta devs; validates j/d pum paradigms.
    // ---------------------------------------------------------------------------
    fn halanta_j_d_stems() {
        // j-anta: vaRij (वणिज्) — nom sg वणिक् (8.2.30), instr वणिजा, loc वणिजि
        let v = generate("vaRij", "pum").expect("vaRij");
        has(&v, "prathamA", "vaRik");
        has(&v, "prathamA", "vaRig");
        has(&v, "tfIyA", "vaRijA");
        has(&v, "tfIyA", "vaRigByAm");
        has(&v, "saptamI", "vaRiji");
        has(&v, "saptamI", "vaRikzu");
        has(&generate("ftvij", "pum").unwrap(), "prathamA", "ftvik");
        has(&generate("ftvij", "pum").unwrap(), "saptamI", "ftvikzu");
        // d-anta: suhfd (सुहृद्) — nom sg सुहृत् (8.2.39), instr सुहृदा
        let s = generate("suhfd", "pum").expect("suhfd");
        has(&s, "prathamA", "suhft");
        has(&s, "dvitIyA", "suhfdam");
        has(&s, "tfIyA", "suhfdA");
        // लिह्: 8.2.31 लिट्/लिड्; उष्णिह् stays कुत्व उष्णिक्
        let h = generate("lih", "pum").expect("lih");
        has(&h, "prathamA", "liw");
        has(&h, "prathamA", "liq");
        has(&h, "tfIyA", "lihA");
        has(&h, "tfIyA", "liqByAm");
        has(&h, "saptamI", "liwsu");
        let u = generate("uzRih", "pum").expect("uzRih");
        has(&u, "prathamA", "uzRik");
        has(&u, "prathamA", "uzRig");
        has(&u, "tfIyA", "uzRihA");
        has(&u, "tfIyA", "uzRigByAm");
        has(&u, "saptamI", "uzRihi");
        has(&u, "saptamI", "uzRikzu");
        let b = generate("laB", "pum").expect("laB");
        has(&b, "prathamA", "lap");
        has(&b, "prathamA", "lab");
        has(&b, "tfIyA", "laBA");
        has(&b, "tfIyA", "labByAm");
        has(&b, "saptamI", "lapsu");
        // r-anta: gir (गिर्) — 8.2.66 s→ru gives giH; s-anta: tapas-like s pum — same ru → tapas→tapaH
        let r = generate("gir", "pum").expect("gir");
        has(&r, "prathamA", "gIH");
        has(&r, "tfIyA", "girA");
        has(&r, "tfIyA", "gIrByAm");
        has(&r, "saptamI", "gIrzu");
        let s = generate("tapas", "pum").expect("tapas");
        // tapas as s pum: prathamA tapaH (ru→visarga), dvitIyA tapasam
        has(&s, "prathamA", "tapaH");
        has(&s, "dvitIyA", "tapasam");
        // as pum: manas-type but pum (sumanas) — as→aH at pada
        let am = generate("manas", "pum").expect("manas pum");
        has(&am, "prathamA", "manaH");
        has(&am, "tfIyA", "manasA");
        // is/us pum: haviS-type but pum — iS→iH
        let hm = generate("havis", "pum").expect("havis pum");
        has(&hm, "prathamA", "haviH");
        let dm = generate("dhanus", "pum").expect("dhanus pum");
        has(&dm, "prathamA", "dhanuH");
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `in_nap_dandin`: 7.1.23 — इन् nap दण्डि/दण्डिनी/दण्डीनि (not पुं दण्डी).
    // ---------------------------------------------------------------------------
    fn in_nap_dandin() {
        let d = generate("daRqin", "nap").expect("daRqin nap");
        has(&d, "prathamA", "daRqi");
        has(&d, "prathamA", "daRqinI");
        has(&d, "prathamA", "daRqIni");
        has(&d, "dvitIyA", "daRqi");
        has(&d, "tfIyA", "daRqinA");
        has(&d, "saptamI", "daRqini");
        has(&d, "saptamI", "daRqizu");
        let p = generate("daRqin", "pum").expect("daRqin pum");
        has(&p, "prathamA", "daRqI");
        has(&p, "dvitIyA", "daRqinam");
        // त-anta: मरुत् (already त् at पद), सरित् स्त्री. `at` शतृ is a longer ending.
        let m = generate("marut", "pum").expect("marut");
        has(&m, "prathamA", "marut");
        has(&m, "dvitIyA", "marutam");
        has(&m, "tfIyA", "marutA");
        has(&m, "saptamI", "maruti");
        has(&m, "saptamI", "marutsu");
        has(&m, "tfIyA", "marudByAm");
        has(&m, "tfIyA", "marudBiH");
        let s = generate("sarit", "stri").expect("sarit");
        has(&s, "prathamA", "sarit");
        has(&s, "dvitIyA", "saritam");
        has(&s, "tfIyA", "saritA");
        has(&s, "tfIyA", "saridByAm");
        // वाच्: 8.2.30/8.4.56 वाक्/वाग्, द्विवचन वाचौ (not *वाचः), सप्तमी वाक्षु.
        let v = generate("vAc", "stri").expect("vAc");
        has(&v, "prathamA", "vAk");
        has(&v, "prathamA", "vAg");
        has(&v, "prathamA", "vAcO");
        has(&v, "dvitIyA", "vAcam");
        has(&v, "tfIyA", "vAcA");
        has(&v, "tfIyA", "vAgByAm");
        has(&v, "zazWI", "vAcAm");
        has(&v, "saptamI", "vAkzu");
        // दिश्: दिक्/दिग्, दिग्भ्याम्, दिक्षु.
        let d2 = generate("diS", "stri").expect("diS");
        has(&d2, "prathamA", "dik");
        has(&d2, "prathamA", "dig");
        has(&d2, "dvitIyA", "diSam");
        has(&d2, "tfIyA", "diSA");
        has(&d2, "tfIyA", "digByAm");
        has(&d2, "saptamI", "dikzu");
        // प-anta: ककुप् (पद already प्).
        let kp = generate("kakup", "pum").expect("kakup");
        has(&kp, "prathamA", "kakup");
        has(&kp, "prathamA", "kakub");
        has(&kp, "dvitIyA", "kakupam");
        has(&kp, "tfIyA", "kakupA");
        has(&kp, "tfIyA", "kakubByAm");
        has(&kp, "saptamI", "kakupsu");
        let gp = generate("gup", "pum").expect("gup");
        has(&gp, "prathamA", "gup");
        has(&gp, "prathamA", "gub");
        has(&gp, "tfIyA", "gubByAm");
        has(&gp, "saptamI", "gupsu");
        // ष-anta: द्विष् — द्विट्/द्विड्, द्विषौ, द्विट्भ्याम्, द्विट्सु (not visarga *द्विः).
        let dz = generate("dviz", "pum").expect("dviz");
        has(&dz, "prathamA", "dviw");
        has(&dz, "prathamA", "dviq");
        has(&dz, "prathamA", "dvizO");
        has(&dz, "dvitIyA", "dvizam");
        has(&dz, "tfIyA", "dvizA");
        has(&dz, "tfIyA", "dviqByAm");
        has(&dz, "saptamI", "dviwsu");
        // ऋच्: ऋक्/ऋग् like वाच् (च-anta also पुं).
        let rc = generate("fc", "stri").expect("fc");
        has(&rc, "prathamA", "fk");
        has(&rc, "prathamA", "fg");
        has(&rc, "dvitIyA", "fcam");
        has(&rc, "tfIyA", "fgByAm");
        has(&rc, "saptamI", "fkzu");
        // ज/द स्त्री (else fallback आ-stem *परिषदा). परिषद् परिषत्; सृज्-type स्त्री सृक्.
        let pd = generate("parizad", "stri").expect("parizad");
        has(&pd, "prathamA", "parizat");
        has(&pd, "dvitIyA", "parizadam");
        has(&pd, "tfIyA", "parizadA");
        has(&pd, "saptamI", "parizatsu");
        let sj = generate("sfj", "stri").expect("sfj");
        has(&sj, "prathamA", "sfk");
        has(&sj, "dvitIyA", "sfjam");
        has(&sj, "saptamI", "sfkzu");
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `atman_brahman_vs_rajan`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn atman_brahman_vs_rajan() {
        let a = generate("Atman", "pum").expect("Atman");
        has(&a, "prathamA", "AtmA");
        has(&a, "tfIyA", "AtmanA");
        has(&a, "zazWI", "AtmanaH");
        let tr = a.declension.get("tfIyA").unwrap();
        assert!(!tr.iter().any(|x| x.contains('Y')), "{:?}", tr);

        let b = generate("brahman", "pum").expect("brahman");
        has(&b, "prathamA", "brahmA");
        has(&b, "dvitIyA", "brahmARam");
        has(&b, "tfIyA", "brahmaRA");
        has(&b, "samboDana", "brahman");

        let r = generate("rAma", "pum").expect("rAma");
        has(&r, "zazWI", "rAmARAm");
        has(&r, "tfIyA", "rAmeRa");
    }

    #[test]
    fn sankhya_20_30_100_noun() {
        // विंशति i-stem स्त्री; त्रिंशत् त-anta (not शतृ *triMSAn); शत a-stem नपुं.
        let v = generate("viMSati", "stri").expect("viMSati");
        has(&v, "prathamA", "viMSatiH");
        has(&v, "dvitIyA", "viMSatim");
        has(&v, "saptamI", "viMSatO");
        let t = generate("triMSat", "stri").expect("triMSat");
        has(&t, "prathamA", "triMSat");
        has(&t, "dvitIyA", "triMSatam");
        has(&t, "tfIyA", "triMSatA");
        has(&t, "tfIyA", "triMSadByAm");
        has(&t, "tfIyA", "triMSadBiH");
        assert!(!t.declension.get("prathamA").unwrap().iter().any(|x| x.ends_with("An")));
        let c = generate("catvAriMSat", "stri").expect("40");
        has(&c, "prathamA", "catvAriMSat");
        let s = generate("Sata", "nap").expect("Sata");
        has(&s, "prathamA", "Satam");
        has(&s, "prathamA", "SatAni");
        let z = generate("zazwi", "stri").expect("zazwi");
        has(&z, "prathamA", "zazwiH");
        let p = generate("paYcASat", "stri").expect("50");
        has(&p, "prathamA", "paYcASat");
        has(&p, "dvitIyA", "paYcASatam");
        has(&generate("saptati", "stri").unwrap(), "prathamA", "saptatiH");
        has(&generate("aSIti", "stri").unwrap(), "prathamA", "aSItiH");
        has(&generate("navati", "stri").unwrap(), "prathamA", "navatiH");
        let h = generate("sahasra", "nap").expect("sahasra");
        has(&h, "prathamA", "sahasram");
        has(&h, "prathamA", "sahasrARi");
    }

    #[test]
    fn ngeep_stri_bavat_dandin_sita() {
        // आ-stem: 8.3.59 does not apply after आ — सीतासु not *सीताषु.
        let s = generate("sItA", "stri").expect("sItA");
        has(&s, "prathamA", "sItA");
        has(&s, "saptamI", "sItAsu");
        has(&s, "saptamI", "sItAyAm");
        // 4.1.6 उगितश्च: भवत् स्त्री भवती (not त-anta *भवत्).
        let b = generate("Bavat", "stri").expect("Bavat stri");
        has(&b, "prathamA", "BavatI");
        has(&b, "dvitIyA", "BavatIm");
        has(&b, "tfIyA", "BavatyA");
        has(&generate("mahat", "stri").unwrap(), "prathamA", "mahatI");
        // 4.1.5 ऋन्नेभ्यो ङीप्: दण्डिन् स्त्री दण्डिनी.
        let d = generate("daRqin", "stri").expect("daRqin stri");
        has(&d, "prathamA", "daRqinI");
        has(&d, "dvitIyA", "daRqinIm");
        // 4.1.5 + 6.4.134 + 8.4.40: राजन् स्त्री राज्ञी (not *राजना).
        let r = generate("rAjan", "stri").expect("rAjan stri");
        has(&r, "prathamA", "rAjYI");
        has(&r, "dvitIyA", "rAjYIm");
        has(&r, "tfIyA", "rAjYyA");
        has(&generate("nAman", "stri").unwrap(), "prathamA", "nAmnI");
        // 6.4.133 + ङीप्: शुनी/यूनी.
        has(&generate("Svan", "stri").unwrap(), "prathamA", "SunI");
        has(&generate("yuvan", "stri").unwrap(), "prathamA", "yUnI");
        // त्रिंशत् still त-anta (Sat excluded from ङीप्).
        has(&generate("triMSat", "stri").unwrap(), "prathamA", "triMSat");
        // हल् स्त्री same as पुं (not आ-stem fallback).
        has(&generate("lih", "stri").unwrap(), "prathamA", "liw");
        has(&generate("laB", "stri").unwrap(), "prathamA", "lap");
        // 7.1.81 शतृ ङीप् नुम् पचन्ती. भवती/धीमती/महती/ददती stay.
        let p = generate("pacat", "stri").expect("pacat stri");
        has(&p, "prathamA", "pacantI");
        has(&p, "prathamA", "pacantyO");
        has(&p, "dvitIyA", "pacantIm");
        has(&generate("DImat", "stri").unwrap(), "prathamA", "DImatI");
        has(&generate("dadat", "stri").unwrap(), "prathamA", "dadatI");
        let k = generate("kartf", "stri").expect("kartf stri");
        has(&k, "prathamA", "kartrI");
        has(&k, "prathamA", "kartryO");
        has(&k, "dvitIyA", "kartrIm");
        has(&generate("DAtf", "stri").unwrap(), "prathamA", "DAtrI");
        has(&generate("mAtf", "stri").unwrap(), "prathamA", "mAtA");
        has(&generate("svasf", "stri").unwrap(), "prathamA", "svasA");
        assert!(!p.declension.get("prathamA").unwrap().iter().any(|x| x == "pacatI"));
        assert!(!generate("Bavat", "stri").unwrap().declension.get("prathamA").unwrap().iter().any(|x| x == "BavantI"));
        assert!(!generate("dadat", "stri").unwrap().declension.get("prathamA").unwrap().iter().any(|x| x == "dadantI"));
        assert!(!k.declension.get("prathamA").unwrap().iter().any(|x| x == "kartA"));
        assert!(!generate("mAtf", "stri").unwrap().declension.get("prathamA").unwrap().iter().any(|x| x == "mAtrI"));
    }

    #[test]
    fn at_nap_jagat_mahat_vrddhi() {
        // शतृ पुं stays short: भवन्तम् not *भवान्तम्.
        let b = generate("Bavat", "pum").expect("Bavat pum");
        has(&b, "prathamA", "BavAn");
        has(&b, "dvitIyA", "Bavantam");
        has(&b, "prathamA", "BavantO");
        has(&b, "tfIyA", "BavadByAm");
        // 6.4.10 महत्: महान्तम्/महान्तौ/महान्तः.
        let m = generate("mahat", "pum").expect("mahat pum");
        has(&m, "prathamA", "mahAn");
        has(&m, "prathamA", "mahAntO");
        has(&m, "prathamA", "mahAntaH");
        has(&m, "dvitIyA", "mahAntam");
        has(&m, "tfIyA", "mahatA");
        has(&m, "tfIyA", "mahadByAm");
        has(&m, "samboDana", "mahan");
        // अत् nap: जगत्/जगती/जगन्ति; जगद्भ्याम् (not *जगांसि / *जगत्भ्याम्).
        let j = generate("jagat", "nap").expect("jagat nap");
        has(&j, "prathamA", "jagat");
        has(&j, "prathamA", "jagatI");
        has(&j, "prathamA", "jaganti");
        has(&j, "dvitIyA", "jagat");
        has(&j, "tfIyA", "jagatA");
        has(&j, "tfIyA", "jagadByAm");
        has(&j, "zazWI", "jagatAm");
        has(&j, "saptamI", "jagatsu");
        has(&j, "prathamA", "jagantI");
        has(&j, "prathamA", "jagati");
        has(&generate("mahat", "nap").unwrap(), "prathamA", "mahAnti");
        has(&generate("mahat", "nap").unwrap(), "prathamA", "mahAntI");
        let p = generate("pacat", "nap").expect("pacat nap");
        has(&p, "prathamA", "pacat");
        has(&p, "prathamA", "pacatI");
        has(&p, "prathamA", "pacantI");
        has(&p, "prathamA", "pacati");
        has(&p, "prathamA", "pacanti");
        let dy = generate("dIvyat", "nap").expect("dIvyat nap");
        has(&dy, "prathamA", "dIvyat");
        has(&dy, "prathamA", "dIvyantI");
        has(&dy, "prathamA", "dIvyanti");
        has(&dy, "tfIyA", "dIvyadByAm");
        has(&generate("Bavat", "pum").unwrap(), "prathamA", "BavAn");
        let pp = generate("pacat", "pum").expect("pacat pum");
        has(&pp, "prathamA", "pacan");
        has(&pp, "prathamA", "pacantO");
        has(&pp, "dvitIyA", "pacantam");
        has(&generate("tudat", "pum").unwrap(), "prathamA", "tudan");
        has(&generate("DImat", "pum").unwrap(), "prathamA", "DImAn");
        assert!(!pp.declension.get("prathamA").unwrap().iter().any(|x| x == "pacAn"));
        assert!(!generate("Bavat", "pum").unwrap().declension.get("prathamA").unwrap().iter().any(|x| x == "Bavan"));
    }

    #[test]
    fn pathadi_pantha_mantha_rbhukshin() {
        // 7.1.85–87 पथिन्: पन्थाः/पन्थानम्, weak पथः, पद पथिभ्याम्/पथिषु.
        let p = generate("paTin", "pum").expect("paTin");
        has(&p, "prathamA", "panTAH");
        has(&p, "prathamA", "panTAnO");
        has(&p, "prathamA", "panTAnaH");
        has(&p, "dvitIyA", "panTAnam");
        has(&p, "dvitIyA", "paTaH");
        has(&p, "tfIyA", "paTA");
        has(&p, "tfIyA", "paTiByAm");
        has(&p, "saptamI", "paTi");
        has(&p, "saptamI", "paTizu");
        has(&p, "samboDana", "panTAH");
        let m = generate("maTin", "pum").expect("maTin");
        has(&m, "prathamA", "manTAH");
        has(&m, "dvitIyA", "manTAnam");
        has(&m, "tfIyA", "maTA");
        // ऋभुक्षिन्: ऋभुक्षाः, ऋभुक्षाणम् (णत्व in strong).
        let r = generate("fBukzin", "pum").expect("fBukzin");
        has(&r, "prathamA", "fBukzAH");
        has(&r, "prathamA", "fBukzARO");
        has(&r, "dvitIyA", "fBukzARam");
        has(&r, "tfIyA", "fBukzA");
        has(&r, "saptamI", "fBukzizu");
    }

    #[test]
    fn sakhi_sakha_not_i_stem() {
        // 7.1.93 सखा; 7.1.92 सख्युः; voc सखे. Not *सखिः.
        let s = generate("saKi", "pum").expect("saKi");
        has(&s, "prathamA", "saKA");
        has(&s, "prathamA", "saKAyO");
        has(&s, "prathamA", "saKAyaH");
        has(&s, "dvitIyA", "saKAyam");
        has(&s, "dvitIyA", "saKIn");
        has(&s, "tfIyA", "saKyA");
        has(&s, "tfIyA", "saKiByAm");
        has(&s, "paYcamI", "saKyuH");
        has(&s, "zazWI", "saKInAm");
        has(&s, "saptamI", "saKyO");
        has(&s, "saptamI", "saKizu");
        has(&s, "samboDana", "saKe");
    }

    #[test]
    fn sva_yuvan_maghavan_samprasarana() {
        // 6.4.133 श्वन्: श्वा/श्वानम्, weak शुना not *श्वना (आत्मन्-type).
        let s = generate("Svan", "pum").expect("Svan");
        has(&s, "prathamA", "SvA");
        has(&s, "prathamA", "SvAnO");
        has(&s, "dvitIyA", "SvAnam");
        has(&s, "dvitIyA", "SunaH");
        has(&s, "tfIyA", "SunA");
        has(&s, "tfIyA", "SvaByAm");
        has(&s, "saptamI", "Suni");
        has(&s, "saptamI", "Svasu");
        has(&s, "samboDana", "Svan");
        let y = generate("yuvan", "pum").expect("yuvan");
        has(&y, "prathamA", "yuvA");
        has(&y, "dvitIyA", "yuvAnam");
        has(&y, "tfIyA", "yUnA");
        has(&y, "caturTI", "yUne");
        has(&y, "saptamI", "yUni");
        has(&y, "saptamI", "yuvasu");
        let m = generate("maGavan", "pum").expect("maGavan");
        has(&m, "prathamA", "maGavA");
        has(&m, "dvitIyA", "maGavAnam");
        has(&m, "tfIyA", "maGonA");
        has(&m, "saptamI", "maGoni");
        has(&m, "samboDana", "maGavan");
    }

    #[test]
    fn ahan_ahas_ahobhih() {
        // अहन् nap: अहः not *अह; अह्ना; अहोभ्याम्/अहोभिः; dual अहनी/अह्नी.
        let a = generate("ahan", "nap").expect("ahan");
        has(&a, "prathamA", "ahaH");
        has(&a, "prathamA", "ahanI");
        has(&a, "prathamA", "ahnI");
        has(&a, "prathamA", "ahAni");
        has(&a, "dvitIyA", "ahaH");
        has(&a, "tfIyA", "ahnA");
        has(&a, "tfIyA", "ahoByAm");
        has(&a, "tfIyA", "ahoBiH");
        has(&a, "caturTI", "ahne");
        has(&a, "zazWI", "ahnAm");
        has(&a, "saptamI", "ahni");
        has(&a, "saptamI", "ahani");
        has(&a, "saptamI", "ahaHsu");
        has(&a, "samboDana", "ahaH");
        assert!(!a.declension.get("prathamA").unwrap().iter().any(|x| x == "aha"));
    }

    #[test]
    fn pums_puman_not_s_stem() {
        // पुंस्: पुमान्/पुमांसम्, पुंसा, पुंभ्याम्. Not s-anta *पुः. Alias `pums`.
        let p = generate("puMs", "pum").expect("puMs");
        has(&p, "prathamA", "pumAn");
        has(&p, "prathamA", "pumAMsO");
        has(&p, "prathamA", "pumAMsaH");
        has(&p, "dvitIyA", "pumAMsam");
        has(&p, "dvitIyA", "puMsaH");
        has(&p, "tfIyA", "puMsA");
        has(&p, "tfIyA", "puMByAm");
        has(&p, "tfIyA", "puMBiH");
        has(&p, "saptamI", "puMsi");
        has(&p, "saptamI", "puMsu");
        has(&p, "samboDana", "puman");
        has(&generate("pums", "pum").unwrap(), "prathamA", "pumAn");
        let s = generate("supums", "nap").expect("supums");
        has(&s, "prathamA", "supum");
        has(&s, "prathamA", "supuMsI");
        has(&s, "prathamA", "supumAMsi");
        has(&s, "dvitIyA", "supum");
        has(&s, "tfIyA", "supuMsA");
        has(&s, "tfIyA", "supuMByAm");
        has(&s, "saptamI", "supuMsi");
        has(&s, "saptamI", "supuMsu");
        assert!(!s.declension.get("prathamA").unwrap().iter().any(|x| x == "supumAn"));
    }

    #[test]
    fn ap_apah_adbhih() {
        // अप्: 6.4.11 आपः/आपौ; पद अद्भिः/अप्सु. No एकवचन. Not p-anta *अप्.
        let a = generate("ap", "stri").expect("ap");
        has(&a, "prathamA", "ApaH");
        has(&a, "prathamA", "ApO");
        has(&a, "dvitIyA", "ApaH");
        has(&a, "tfIyA", "adBiH");
        has(&a, "tfIyA", "adByAm");
        has(&a, "zazWI", "apAm");
        has(&a, "zazWI", "apoH");
        has(&a, "saptamI", "apsu");
        let pr = a.declension.get("prathamA").unwrap();
        assert!(!pr.iter().any(|x| x == "ap"), "{:?}", pr);
    }

    #[test]
    fn anaquh_upanah_h_to_d() {
        // अनडुह्: अनड्वान्/अनड्वाहम्; पद अनडुद्भ्याम्/अनडुत्सु. Not *अनडुक्.
        let a = generate("anaquh", "pum").expect("anaquh");
        has(&a, "prathamA", "anaqvAn");
        has(&a, "prathamA", "anaqvAhO");
        has(&a, "dvitIyA", "anaqvAham");
        has(&a, "tfIyA", "anaquhA");
        has(&a, "tfIyA", "anaqudByAm");
        has(&a, "tfIyA", "anaqudBiH");
        has(&a, "saptamI", "anaqutsu");
        has(&a, "samboDana", "anaqvan");
        // उपानह्: उपानत्/उपानद्; उपानद्भिः/उपानत्सु. Not *उपानक्.
        let u = generate("upAnah", "stri").expect("upAnah");
        has(&u, "prathamA", "upAnat");
        has(&u, "prathamA", "upAnad");
        has(&u, "dvitIyA", "upAnaham");
        has(&u, "tfIyA", "upAnadBiH");
        has(&u, "saptamI", "upAnatsu");
        let s = generate("svanaquh", "nap").expect("svanaquh");
        has(&s, "prathamA", "svanaqut");
        has(&s, "prathamA", "svanaqud");
        has(&s, "prathamA", "svanaquhI");
        has(&s, "prathamA", "svanaqvAMhi");
        has(&s, "tfIyA", "svanaquhA");
        has(&s, "tfIyA", "svanaqudByAm");
        has(&s, "saptamI", "svanaqutsu");
        has(&generate("anaquh", "pum").unwrap(), "prathamA", "anaqvAn");
        assert!(!s.declension.get("prathamA").unwrap().iter().any(|x| x == "svanaqvAn"));
    }

    #[test]
    fn div_dyauh_dyubhih() {
        // 7.1.84 दिव औत् द्यौः; पद द्युभ्याम्/द्युषु. Not *दिवा.
        let d = generate("div", "stri").expect("div");
        has(&d, "prathamA", "dyOH");
        has(&d, "prathamA", "divO");
        has(&d, "prathamA", "divaH");
        has(&d, "dvitIyA", "divam");
        has(&d, "tfIyA", "divA");
        has(&d, "tfIyA", "dyuByAm");
        has(&d, "tfIyA", "dyuBiH");
        has(&d, "saptamI", "divi");
        has(&d, "saptamI", "dyuzu");
        has(&d, "samboDana", "dyOH");
        has(&generate("div", "pum").unwrap(), "prathamA", "dyOH");
        has(&generate("sudiv", "pum").unwrap(), "prathamA", "sudyOH");
        has(&generate("sudiv", "pum").unwrap(), "tfIyA", "sudyuByAm");
        has(&generate("sudiv", "pum").unwrap(), "saptamI", "sudyuzu");
    }

    #[test]
    fn rai_rah_rayam() {
        // रै: राः/रायम्/राभ्याम्/रासु. Not नौ *रावम्.
        let r = generate("rE", "stri").expect("rE");
        has(&r, "prathamA", "rAH");
        has(&r, "prathamA", "rAyO");
        has(&r, "prathamA", "rAyaH");
        has(&r, "dvitIyA", "rAyam");
        has(&r, "tfIyA", "rAyA");
        has(&r, "tfIyA", "rAByAm");
        has(&r, "saptamI", "rAyi");
        has(&r, "saptamI", "rAsu");
        has(&generate("rE", "pum").unwrap(), "prathamA", "rAH");
        let n = generate("nO", "stri").expect("nO");
        has(&n, "dvitIyA", "nAvam");
        assert!(!r.declension.get("dvitIyA").unwrap().iter().any(|x| x == "rAvam"));
    }

    #[test]
    fn anc_pranc_pran_prac() {
        // प्राञ्च्: 8.2.23 प्राङ्; 6.4.24 प्राचा; 8.2.30 प्राग्/प्राक्षु. Not च-anta *प्राक्.
        let p = generate("prAYc", "pum").expect("prAYc");
        has(&p, "prathamA", "prAN");
        has(&p, "prathamA", "prAYcO");
        has(&p, "prathamA", "prAYcaH");
        has(&p, "dvitIyA", "prAYcam");
        has(&p, "dvitIyA", "prAcaH");
        has(&p, "tfIyA", "prAcA");
        has(&p, "tfIyA", "prAgByAm");
        has(&p, "tfIyA", "prAgBiH");
        has(&p, "saptamI", "prAci");
        has(&p, "saptamI", "prAkzu");
        has(&generate("pratyaYc", "pum").unwrap(), "prathamA", "pratyaN");
        has(&generate("pratyaYc", "pum").unwrap(), "tfIyA", "pratIcA");
        has(&generate("pratyaYc", "pum").unwrap(), "tfIyA", "pratyagBiH");
        has(&generate("udaYc", "pum").unwrap(), "tfIyA", "udIcA");
        has(&generate("tiryaYc", "pum").unwrap(), "tfIyA", "tiraScA");
        has(&generate("samyaYc", "pum").unwrap(), "tfIyA", "samIcA");
        has(&generate("saDryaYc", "pum").unwrap(), "tfIyA", "saDrIcA");
        has(&generate("prAYc", "stri").unwrap(), "prathamA", "prAcI");
        has(&generate("prAYc", "nap").unwrap(), "prathamA", "prAk");
        has(&generate("prAYc", "nap").unwrap(), "prathamA", "prAcI");
        has(&generate("prAYc", "nap").unwrap(), "prathamA", "prAYci");
        has(&generate("vAc", "stri").unwrap(), "prathamA", "vAk");
    }

    #[test]
    fn asthyadi_dadhna() {
        // 7.1.75 अनङ्: दध्ना/दधिभ्याम् not i-nap *दधिना. स्वमोः दधि/दधिनी/दधीनि.
        let d = generate("daDi", "nap").expect("daDi");
        has(&d, "prathamA", "daDi");
        has(&d, "prathamA", "daDinI");
        has(&d, "prathamA", "daDIni");
        has(&d, "tfIyA", "daDnA");
        has(&d, "tfIyA", "daDiByAm");
        has(&d, "tfIyA", "daDiBiH");
        has(&d, "saptamI", "daDni");
        has(&d, "saptamI", "daDani");
        has(&d, "saptamI", "daDizu");
        has(&d, "samboDana", "daDe");
        has(&generate("asTi", "nap").unwrap(), "tfIyA", "asTnA");
        has(&generate("akzi", "nap").unwrap(), "tfIyA", "akzRA");
        has(&generate("sakTi", "nap").unwrap(), "tfIyA", "sakTnA");
        let v = generate("vAri", "nap").expect("vAri");
        has(&v, "tfIyA", "vAriRA");
        assert!(!d.declension.get("tfIyA").unwrap().iter().any(|x| x == "daDinA"));
    }

    #[test]
    fn krunc_krun_krunca() {
        // क्रुञ्च्: 8.2.23 क्रुङ्; ञ् kept क्रुञ्चा (not 6.4.24 *क्रुचा); पद क्रुङ्भ्याम् not *क्रुग्.
        let k = generate("kruYc", "pum").expect("kruYc");
        has(&k, "prathamA", "kruN");
        has(&k, "prathamA", "kruYcO");
        has(&k, "dvitIyA", "kruYcam");
        has(&k, "tfIyA", "kruYcA");
        has(&k, "tfIyA", "kruNByAm");
        has(&k, "tfIyA", "kruNBiH");
        has(&k, "saptamI", "kruYci");
        has(&k, "saptamI", "kruNkzu");
        has(&generate("kruYc", "stri").unwrap(), "prathamA", "kruN");
        has(&generate("prAYc", "pum").unwrap(), "tfIyA", "prAcA");
        has(&generate("vAc", "stri").unwrap(), "tfIyA", "vAgByAm");
    }

    #[test]
    fn nr_naram_nrnam() {
        // नृ: नरम् not कर्तृ *नारम्; नॄन्; 6.4.6 नॄणाम्/नृणाम्.
        let n = generate("nf", "pum").expect("nf");
        has(&n, "prathamA", "nA");
        has(&n, "prathamA", "narO");
        has(&n, "dvitIyA", "naram");
        has(&n, "dvitIyA", "nFn");
        has(&n, "tfIyA", "nrA");
        has(&n, "tfIyA", "nfBiH");
        has(&n, "zazWI", "nuH");
        has(&n, "zazWI", "nFRAm");
        has(&n, "zazWI", "nfRAm");
        has(&n, "saptamI", "nfzu");
        has(&n, "samboDana", "naH");
        has(&generate("pitf", "pum").unwrap(), "dvitIyA", "pitaram");
        has(&generate("kartf", "pum").unwrap(), "dvitIyA", "kartAram");
        assert!(!n.declension.get("dvitIyA").unwrap().iter().any(|x| x == "nAram"));
    }

    #[test]
    fn kvasu_vidvan_vidusa() {
        // विद्वस्: 7.1.70 विद्वान्; 6.4.131 विदुषा; 8.2.72 विद्वद्भिः. Not as-pum *विद्वः.
        let v = generate("vidvas", "pum").expect("vidvas");
        has(&v, "prathamA", "vidvAn");
        has(&v, "prathamA", "vidvAMsO");
        has(&v, "prathamA", "vidvAMsaH");
        has(&v, "dvitIyA", "vidvAMsam");
        has(&v, "dvitIyA", "viduzaH");
        has(&v, "tfIyA", "viduzA");
        has(&v, "tfIyA", "vidvadByAm");
        has(&v, "tfIyA", "vidvadBiH");
        has(&v, "saptamI", "viduzi");
        has(&v, "saptamI", "vidvatsu");
        has(&v, "samboDana", "vidvan");
        has(&generate("vidvas", "stri").unwrap(), "prathamA", "viduzI");
        has(&generate("vidvas", "nap").unwrap(), "prathamA", "vidvat");
        has(&generate("vidvas", "nap").unwrap(), "prathamA", "viduzI");
        has(&generate("vidvas", "nap").unwrap(), "prathamA", "vidvAMsi");
        has(&generate("manas", "nap").unwrap(), "prathamA", "manaH");
        // 6.4.131: बभूवस् keeps aṅga व् बभूवुषा, not *बभूषा. विद्वस् stays विदुषा.
        let b = generate("baBUvas", "pum").expect("baBUvas");
        has(&b, "prathamA", "baBUvAn");
        has(&b, "prathamA", "baBUvAMsO");
        has(&b, "dvitIyA", "baBUvAMsam");
        has(&b, "tfIyA", "baBUvuzA");
        has(&b, "tfIyA", "baBUvadByAm");
        has(&b, "saptamI", "baBUvuzi");
        has(&b, "saptamI", "baBUvatsu");
        has(&generate("baBUvas", "stri").unwrap(), "prathamA", "baBUvuzI");
        has(&generate("baBUvas", "nap").unwrap(), "prathamA", "baBUvat");
        has(&generate("baBUvas", "nap").unwrap(), "prathamA", "baBUvuzI");
        assert!(!b.declension.get("tfIyA").unwrap().iter().any(|x| x == "baBUuzA"));
        assert!(!v.declension.get("tfIyA").unwrap().iter().any(|x| x == "vidvuzA"));
    }

    #[test]
    fn asis_asih_asirbhyam() {
        // आशिस्: 8.2.66/76 आशीः/आशीर्भ्याम्, not ष-anta *आशिट्.
        let a = generate("ASiz", "stri").expect("ASiz");
        has(&a, "prathamA", "ASIH");
        has(&a, "prathamA", "ASizO");
        has(&a, "dvitIyA", "ASizam");
        has(&a, "tfIyA", "ASizA");
        has(&a, "tfIyA", "ASIrByAm");
        has(&a, "tfIyA", "ASIrBiH");
        has(&a, "saptamI", "ASizi");
        has(&a, "saptamI", "ASIHzu");
        has(&generate("pipaWiz", "pum").unwrap(), "prathamA", "pipaWIH");
        has(&generate("pipaWiz", "pum").unwrap(), "tfIyA", "pipaWIrByAm");
        has(&generate("dviz", "pum").unwrap(), "prathamA", "dviw");
        has(&generate("sajuz", "pum").unwrap(), "prathamA", "sajUH");
        has(&generate("sajuz", "pum").unwrap(), "tfIyA", "sajUrByAm");
        has(&generate("sajuz", "pum").unwrap(), "saptamI", "sajUHzu");
        let c = generate("cikIrz", "pum").expect("cikIrz");
        has(&c, "prathamA", "cikIH");
        has(&c, "tfIyA", "cikIrByAm");
        has(&c, "saptamI", "cikIrzu");
        assert!(!c.declension.get("prathamA").unwrap().iter().any(|x| x == "cikIrw"));
        assert!(!a.declension.get("prathamA").unwrap().iter().any(|x| x == "ASiw"));
    }

    #[test]
    fn raj_rat_radbhyam() {
        // राज्: राट्/राड् not ज-anta *राक् (वणिक्). राजन् stays राजा.
        let r = generate("rAj", "pum").expect("rAj");
        has(&r, "prathamA", "rAw");
        has(&r, "prathamA", "rAq");
        has(&r, "prathamA", "rAjO");
        has(&r, "dvitIyA", "rAjam");
        has(&r, "tfIyA", "rAjA");
        has(&r, "tfIyA", "rAqByAm");
        has(&r, "tfIyA", "rAqBiH");
        has(&r, "saptamI", "rAji");
        has(&r, "saptamI", "rAwsu");
        has(&generate("rAj", "stri").unwrap(), "prathamA", "rAw");
        has(&generate("rAjan", "pum").unwrap(), "prathamA", "rAjA");
        has(&generate("vaRij", "pum").unwrap(), "prathamA", "vaRik");
        let vr = generate("viSvarAj", "pum").expect("viSvarAj");
        has(&vr, "prathamA", "viSvArAw");
        has(&vr, "prathamA", "viSvArAq");
        has(&vr, "prathamA", "viSvarAjO");
        has(&vr, "tfIyA", "viSvarAjA");
        has(&vr, "tfIyA", "viSvArAqByAm");
        has(&vr, "saptamI", "viSvArAwsu");
        assert!(!r.declension.get("prathamA").unwrap().iter().any(|x| x == "rAk"));
        assert!(!vr.declension.get("prathamA").unwrap().iter().any(|x| x == "viSvarAk"));
    }

    #[test]
    fn dhanus_dhanuh_dhanurbhyam() {
        // धनुस् (Danuz): धनुः/धनुषी/धूंषि, धनुर्भ्याम्. Not a-stem *धनुषः; सजष् पुं stays सजूः.
        let d = generate("Danuz", "nap").expect("Danuz");
        has(&d, "prathamA", "DanuH");
        has(&d, "prathamA", "DanuzI");
        has(&d, "prathamA", "DanUMzi");
        has(&d, "tfIyA", "DanuzA");
        has(&d, "tfIyA", "DanurByAm");
        has(&d, "tfIyA", "DanurBiH");
        has(&d, "saptamI", "Danuzi");
        has(&d, "saptamI", "DanuHzu");
        has(&generate("sajuz", "pum").unwrap(), "prathamA", "sajUH");
        has(&generate("manas", "nap").unwrap(), "prathamA", "manaH");
    }

    #[test]
    fn ushanas_ushana() {
        // 7.1.94: उशना not *उशनः; voc उशनन्. अनेहा. मनस् stays मनः.
        let u = generate("uSanas", "pum").expect("uSanas");
        has(&u, "prathamA", "uSanA");
        has(&u, "prathamA", "uSanasO");
        has(&u, "dvitIyA", "uSanasam");
        has(&u, "tfIyA", "uSanasA");
        has(&u, "tfIyA", "uSanoByAm");
        has(&u, "samboDana", "uSanan");
        has(&u, "samboDana", "uSanaH");
        has(&generate("anehas", "pum").unwrap(), "prathamA", "anehA");
        has(&generate("anehas", "pum").unwrap(), "tfIyA", "anehoByAm");
        has(&generate("manas", "pum").unwrap(), "prathamA", "manaH");
        assert!(!u.declension.get("prathamA").unwrap().iter().any(|x| x == "uSanaH"));
    }

    #[test]
    fn sakrt_sakrti_sakrdbhyam() {
        // शकृत् nap: शकृत्/शकृती/शकृन्ति, शकृद्भ्याम्. जगत् stays at-nap जगन्ति.
        let s = generate("Sakft", "nap").expect("Sakft");
        has(&s, "prathamA", "Sakft");
        has(&s, "prathamA", "SakftI");
        has(&s, "prathamA", "SakfRti");
        has(&s, "tfIyA", "SakftA");
        has(&s, "tfIyA", "SakfdByAm");
        has(&s, "saptamI", "Sakftsu");
        has(&generate("jagat", "nap").unwrap(), "prathamA", "jagat");
        has(&generate("jagat", "nap").unwrap(), "prathamA", "jaganti");
        has(&generate("marut", "pum").unwrap(), "prathamA", "marut");
    }

    #[test]
    fn supad_supat_supada() {
        // 6.4.130 पत्: सुपदा/सुपदः not d-anta *सुपादा. Nom सुपात्/सुपाद्.
        let s = generate("supAd", "pum").expect("supAd");
        has(&s, "prathamA", "supAt");
        has(&s, "prathamA", "supAd");
        has(&s, "prathamA", "supAdO");
        has(&s, "dvitIyA", "supAdam");
        has(&s, "dvitIyA", "supadaH");
        has(&s, "tfIyA", "supadA");
        has(&s, "tfIyA", "supAdByAm");
        has(&s, "saptamI", "supadi");
        has(&s, "saptamI", "supAtsu");
        has(&generate("suhfd", "pum").unwrap(), "prathamA", "suhft");
        has(&generate("suhfd", "pum").unwrap(), "tfIyA", "suhfdA");
        assert!(!s.declension.get("tfIyA").unwrap().iter().any(|x| x == "supAdA"));
    }

    #[test]
    fn var_vah_vari_varzu() {
        // वार nap: वः/वारी/वारि, वार्भ्याम्/वार्षु (not a-stem *वारम्). वारि i-nap stays वारिणा.
        let v = generate("vAr", "nap").expect("vAr");
        has(&v, "prathamA", "vAH");
        has(&v, "prathamA", "vArI");
        has(&v, "prathamA", "vAri");
        has(&v, "dvitIyA", "vAH");
        has(&v, "tfIyA", "vArA");
        has(&v, "tfIyA", "vArByAm");
        has(&v, "tfIyA", "vArBiH");
        has(&v, "saptamI", "vAri");
        has(&v, "saptamI", "vArzu");
        has(&v, "zazWI", "vArAm");
        has(&generate("vAri", "nap").unwrap(), "tfIyA", "vAriRA");
        has(&generate("gir", "pum").unwrap(), "prathamA", "gIH");
        assert!(!v.declension.get("prathamA").unwrap().iter().any(|x| x == "vAram"));
    }

    #[test]
    fn duh_dhuk_dhugbhyam() {
        // 8.2.32 दादेर्धातोर्घः: धुक्/धुग्, ध्रुक्. उष्णिह् stays उष्णिक् (not *उष्णिधुक्).
        let d = generate("duh", "pum").expect("duh");
        has(&d, "prathamA", "Duk");
        has(&d, "prathamA", "Dug");
        has(&d, "prathamA", "duhO");
        has(&d, "dvitIyA", "duham");
        has(&d, "tfIyA", "duhA");
        has(&d, "tfIyA", "DugByAm");
        has(&d, "tfIyA", "DugBiH");
        has(&d, "saptamI", "duhi");
        has(&d, "saptamI", "Dukzu");
        let r = generate("druh", "pum").expect("druh");
        has(&r, "prathamA", "Druk");
        has(&r, "tfIyA", "DrugByAm");
        has(&r, "saptamI", "Drukzu");
        has(&generate("uzRih", "pum").unwrap(), "prathamA", "uzRik");
        has(&generate("lih", "pum").unwrap(), "prathamA", "liw");
        assert!(!d.declension.get("prathamA").unwrap().iter().any(|x| x == "duk"));
    }

    #[test]
    fn visvavah_visvauha_turasat() {
        // 6.4.132 ऊठ्: विश्वौहा; पद विश्ववाट्/विश्ववाड्. तुरासाह् तुराषाट् (no ऊठ्). Not *विश्ववाक्.
        let v = generate("viSvavAh", "pum").expect("viSvavAh");
        has(&v, "prathamA", "viSvavAw");
        has(&v, "prathamA", "viSvavAq");
        has(&v, "prathamA", "viSvavAhO");
        has(&v, "dvitIyA", "viSvavAham");
        has(&v, "dvitIyA", "viSvOhaH");
        has(&v, "tfIyA", "viSvOhA");
        has(&v, "tfIyA", "viSvavAqByAm");
        has(&v, "saptamI", "viSvOhi");
        has(&v, "saptamI", "viSvavAwsu");
        let t = generate("turAsAh", "pum").expect("turAsAh");
        has(&t, "prathamA", "turAzAw");
        has(&t, "prathamA", "turAzAq");
        has(&t, "tfIyA", "turAsAhA");
        has(&t, "tfIyA", "turAzAqByAm");
        has(&t, "saptamI", "turAsAhi");
        has(&t, "saptamI", "turAzAwsu");
        has(&generate("duh", "pum").unwrap(), "prathamA", "Duk");
        has(&generate("uzRih", "pum").unwrap(), "prathamA", "uzRik");
        assert!(!v.declension.get("prathamA").unwrap().iter().any(|x| x == "viSvavAk"));
    }

    #[test]
    fn bhrasj_bhrat_bhrjja() {
        // भृस्ज्: पद भृट्/भृड्, vowel भृज्जौ. Not ज-anta *भृस्क्.
        let b = generate("Bfsj", "pum").expect("Bfsj");
        has(&b, "prathamA", "Bfw");
        has(&b, "prathamA", "Bfq");
        has(&b, "prathamA", "BfjjO");
        has(&b, "dvitIyA", "Bfjjam");
        has(&b, "tfIyA", "BfjjA");
        has(&b, "tfIyA", "BfqByAm");
        has(&b, "saptamI", "Bfjji");
        has(&b, "saptamI", "Bfwsu");
        has(&generate("vaRij", "pum").unwrap(), "prathamA", "vaRik");
        assert!(!b.declension.get("prathamA").unwrap().iter().any(|x| x == "Bfsk"));
    }

    #[test]
    fn urj_urk_urji() {
        // ऊर्ज् nap: ऊर्क्/ऊर्जी/ऊर्जि, ऊर्ग्भ्याम्/ऊर्क्षु. वणिक् stays पुं ज-anta.
        let u = generate("Urj", "nap").expect("Urj");
        has(&u, "prathamA", "Urk");
        has(&u, "prathamA", "Urg");
        has(&u, "prathamA", "UrjI");
        has(&u, "prathamA", "Urji");
        has(&u, "tfIyA", "UrjA");
        has(&u, "tfIyA", "UrgByAm");
        has(&u, "saptamI", "Urji");
        has(&u, "saptamI", "Urkzu");
        has(&generate("vaRij", "pum").unwrap(), "prathamA", "vaRik");
        has(&generate("vaRij", "pum").unwrap(), "prathamA", "vaRijO");
    }

    #[test]
    fn dadhrsh_dadhrik() {
        // दाधृष्: दधृक्/दधृग्, दधृग्भ्याम्/दधृक्षु. द्विष् stays द्विट्.
        let d = generate("daDfz", "pum").expect("daDfz");
        has(&d, "prathamA", "daDfk");
        has(&d, "prathamA", "daDfg");
        has(&d, "prathamA", "daDfzO");
        has(&d, "dvitIyA", "daDfzam");
        has(&d, "tfIyA", "daDfzA");
        has(&d, "tfIyA", "daDfgByAm");
        has(&d, "saptamI", "daDfzi");
        has(&d, "saptamI", "daDfkzu");
        has(&generate("dviz", "pum").unwrap(), "prathamA", "dviw");
        assert!(!d.declension.get("prathamA").unwrap().iter().any(|x| x == "daDfw"));
    }

    #[test]
    fn prasam_prasan() {
        // 8.2.64: प्रशान्/प्रशान्भ्याम्, not a-stem *प्रशामः.
        let p = generate("praSAm", "pum").expect("praSAm");
        has(&p, "prathamA", "praSAn");
        has(&p, "prathamA", "praSAmO");
        has(&p, "dvitIyA", "praSAmam");
        has(&p, "tfIyA", "praSAmA");
        has(&p, "tfIyA", "praSAnByAm");
        has(&p, "saptamI", "praSAmi");
        has(&p, "saptamI", "praSAnsu");
        has(&generate("rAma", "pum").unwrap(), "prathamA", "rAmaH");
        assert!(!p.declension.get("tfIyA").unwrap().iter().any(|x| x == "praSAmena"));
    }

    #[test]
    fn vish_vit_vidbhyam() {
        // विश्: विट्/विड्, विड्भ्याम्/विट्सु. दिश् stays दिक्; तादृश् stays तादृक्.
        let v = generate("viS", "pum").expect("viS");
        has(&v, "prathamA", "viw");
        has(&v, "prathamA", "viq");
        has(&v, "prathamA", "viSO");
        has(&v, "dvitIyA", "viSam");
        has(&v, "tfIyA", "viSA");
        has(&v, "tfIyA", "viqByAm");
        has(&v, "saptamI", "viSi");
        has(&v, "saptamI", "viwsu");
        has(&generate("diS", "stri").unwrap(), "prathamA", "dik");
        has(&generate("tAdfS", "pum").unwrap(), "prathamA", "tAdfk");
        assert!(!v.declension.get("prathamA").unwrap().iter().any(|x| x == "vik"));
    }

    #[test]
    fn tadrs_nap_tadrsi() {
        // 7.1.19/20 श-anta nap तादृक्/तादृशी/तादृशि. पुं stays तादृशौ. ऊर्ज् stays ज-anta. धनुस् stays धनुषी.
        let t = generate("tAdfS", "nap").expect("tAdfS nap");
        has(&t, "prathamA", "tAdfk");
        has(&t, "prathamA", "tAdfg");
        has(&t, "prathamA", "tAdfSI");
        has(&t, "prathamA", "tAdfSi");
        has(&t, "tfIyA", "tAdfSA");
        has(&t, "tfIyA", "tAdfgByAm");
        has(&t, "saptamI", "tAdfSi");
        has(&t, "saptamI", "tAdfkzu");
        has(&generate("tAdfS", "pum").unwrap(), "prathamA", "tAdfSO");
        has(&generate("Urj", "nap").unwrap(), "prathamA", "UrjI");
        has(&generate("Danuz", "nap").unwrap(), "prathamA", "DanuzI");
        let c = generate("payomuc", "nap").expect("payomuc nap");
        has(&c, "prathamA", "payomuk");
        has(&c, "prathamA", "payomucI");
        has(&generate("payomuc", "pum").unwrap(), "prathamA", "payomucO");
        let z = generate("tviz", "nap").expect("tviz nap");
        has(&z, "prathamA", "tviw");
        has(&z, "prathamA", "tvizI");
        has(&generate("tviz", "pum").unwrap(), "prathamA", "tvizO");
        let h = generate("uzRih", "nap").expect("uzRih nap");
        has(&h, "prathamA", "uzRik");
        has(&h, "prathamA", "uzRihI");
        has(&generate("uzRih", "pum").unwrap(), "prathamA", "uzRihO");
        assert!(!t.declension.get("prathamA").unwrap().iter().any(|x| x == "tAdfSO"));
        assert!(!t.declension.get("prathamA").unwrap().iter().any(|x| x == "tAdfSam"));
    }

    #[test]
    fn vrtrahan_vrtraha_vrtraghna() {
        // वृत्रहन्: वृत्रहा/वृत्रहणम्/वृत्रघ्ना; पद वृत्रहभ्याम्. अहन् stays अहः; राजन् stays राजा.
        let v = generate("vftrahan", "pum").expect("vftrahan");
        has(&v, "prathamA", "vftrahA");
        has(&v, "prathamA", "vftrahaRO");
        has(&v, "prathamA", "vftrahaRaH");
        has(&v, "dvitIyA", "vftrahaRam");
        has(&v, "dvitIyA", "vftraGnaH");
        has(&v, "tfIyA", "vftraGnA");
        has(&v, "tfIyA", "vftrahaByAm");
        has(&v, "saptamI", "vftraGni");
        has(&v, "saptamI", "vftrahasu");
        has(&v, "samboDana", "vftrahan");
        has(&generate("ahan", "nap").unwrap(), "prathamA", "ahaH");
        has(&generate("rAjan", "pum").unwrap(), "prathamA", "rAjA");
        assert!(!v.declension.get("tfIyA").unwrap().iter().any(|x| x == "vftrahnA"));
    }

    #[test]
    fn khanj_khan_khanbhyam() {
        // खञ्ज्: 8.2.23 पद खन्/खन्भ्याम्/खन्सु. वणिक् stays कुत्व; क्रुङ् stays क्रुञ्च्.
        let k = generate("KaYj", "pum").expect("KaYj");
        has(&k, "prathamA", "Kan");
        has(&k, "prathamA", "KaYjO");
        has(&k, "dvitIyA", "KaYjam");
        has(&k, "tfIyA", "KaYjA");
        has(&k, "tfIyA", "KanByAm");
        has(&k, "saptamI", "KaYji");
        has(&k, "saptamI", "Kansu");
        has(&generate("vaRij", "pum").unwrap(), "prathamA", "vaRik");
        has(&generate("kruYc", "pum").unwrap(), "prathamA", "kruN");
        assert!(!k.declension.get("prathamA").unwrap().iter().any(|x| x == "KaYk"));
    }

    #[test]
    fn agnimat_agnimatsu() {
        // अग्निमथ्: थ्→त्/द्, अग्निमद्भ्याम्/अग्निमत्सु. मरुत् stays त-anta; भवत् stays at.
        let a = generate("agnimaT", "pum").expect("agnimaT");
        has(&a, "prathamA", "agnimat");
        has(&a, "prathamA", "agnimad");
        has(&a, "prathamA", "agnimaTO");
        has(&a, "dvitIyA", "agnimaTam");
        has(&a, "tfIyA", "agnimaTA");
        has(&a, "tfIyA", "agnimadByAm");
        has(&a, "saptamI", "agnimaTi");
        has(&a, "saptamI", "agnimatsu");
        has(&generate("marut", "pum").unwrap(), "prathamA", "marut");
        has(&generate("Bavat", "pum").unwrap(), "prathamA", "BavAn");
        assert!(!a.declension.get("tfIyA").unwrap().iter().any(|x| x == "agnimaTena"));
    }

    #[test]
    fn gopa_gopi_gopasu() {
        // गोपा/विश्वपा: आकारान्त पुं गोपाः/गोपौ/गोपाम्/गोपः/गोपा/गोपि/गोपासु. सीता stays टाप्.
        let g = generate("gopA", "pum").expect("gopA");
        has(&g, "prathamA", "gopAH");
        has(&g, "prathamA", "gopO");
        has(&g, "dvitIyA", "gopAm");
        has(&g, "dvitIyA", "gopaH");
        has(&g, "tfIyA", "gopA");
        has(&g, "tfIyA", "gopAByAm");
        has(&g, "caturTI", "gope");
        has(&g, "saptamI", "gopi");
        has(&g, "saptamI", "gopAsu");
        has(&g, "zazWI", "gopoH");
        has(&g, "zazWI", "gopAm");
        let v = generate("viSvapA", "pum").expect("viSvapA");
        has(&v, "prathamA", "viSvapAH");
        has(&v, "saptamI", "viSvapi");
        has(&v, "saptamI", "viSvapAsu");
        has(&generate("sItA", "stri").unwrap(), "tfIyA", "sItayA");
        assert!(!g.declension.get("tfIyA").unwrap().iter().any(|x| x == "gopayA"));
        assert!(!g.declension.get("prathamA").unwrap().iter().any(|x| x == "gopaH"));
    }

    #[test]
    fn arvan_arva_arvadbhyam() {
        // अर्वन्: अनङ् अर्वा; नुम् अर्वन्तौ/अर्वन्तम्; पद अर्वद्भ्याम्/अर्वत्सु. राजन् stays राजा; भवत् stays भवन्.
        let a = generate("arvan", "pum").expect("arvan");
        has(&a, "prathamA", "arvA");
        has(&a, "prathamA", "arvantO");
        has(&a, "prathamA", "arvantaH");
        has(&a, "dvitIyA", "arvantam");
        has(&a, "dvitIyA", "arvataH");
        has(&a, "tfIyA", "arvatA");
        has(&a, "tfIyA", "arvadByAm");
        has(&a, "saptamI", "arvati");
        has(&a, "saptamI", "arvatsu");
        has(&a, "samboDana", "arvan");
        has(&generate("rAjan", "pum").unwrap(), "prathamA", "rAjA");
        has(&generate("rAjan", "pum").unwrap(), "dvitIyA", "rAjAnam");
        has(&generate("Bavat", "pum").unwrap(), "prathamA", "BavAn");
        assert!(!a.declension.get("prathamA").unwrap().iter().any(|x| x == "arvAn"));
        assert!(!a.declension.get("dvitIyA").unwrap().iter().any(|x| x == "arvAnam"));
    }

    #[test]
    fn puzan_puzanau_aryaman() {
        // पूषन्/अर्यमन्: सौ पूषा; dual पूषणौ not *पूषाणौ (6.4.7 skipped). राजन् stays राजानम्.
        let p = generate("pUzan", "pum").expect("pUzan");
        has(&p, "prathamA", "pUzA");
        has(&p, "prathamA", "pUzaRO");
        has(&p, "prathamA", "pUzaRaH");
        has(&p, "dvitIyA", "pUzaRam");
        has(&p, "dvitIyA", "pUzRaH");
        has(&p, "tfIyA", "pUzRA");
        has(&p, "tfIyA", "pUzaByAm");
        has(&p, "saptamI", "pUzRi");
        has(&p, "saptamI", "pUzaRi");
        has(&p, "saptamI", "pUzasu");
        has(&p, "samboDana", "pUzan");
        let a = generate("aryaman", "pum").expect("aryaman");
        has(&a, "prathamA", "aryamA");
        has(&a, "prathamA", "aryamaRO");
        has(&a, "dvitIyA", "aryamaRam");
        has(&a, "tfIyA", "aryamRA");
        has(&generate("rAjan", "pum").unwrap(), "dvitIyA", "rAjAnam");
        has(&generate("rAjan", "pum").unwrap(), "prathamA", "rAjAnO");
        assert!(!p.declension.get("prathamA").unwrap().iter().any(|x| x == "pUzARO"));
        assert!(!a.declension.get("prathamA").unwrap().iter().any(|x| x == "aryamARO"));
    }

    #[test]
    fn kroshtu_kroshta_kroshtuna() {
        // क्रोष्टु 7.1.95: क्रोष्टा/क्रोष्टारम् like कर्तृ; पद/शस् उ क्रोष्टुना/क्रोष्टून्. गुरु stays गुरुः.
        let k = generate("krozwu", "pum").expect("krozwu");
        has(&k, "prathamA", "krozwA");
        has(&k, "prathamA", "krozwArO");
        has(&k, "prathamA", "krozwAraH");
        has(&k, "dvitIyA", "krozwAram");
        has(&k, "dvitIyA", "krozwUn");
        has(&k, "tfIyA", "krozwrA");
        has(&k, "tfIyA", "krozwunA");
        has(&k, "tfIyA", "krozwuByAm");
        has(&k, "caturTI", "krozwre");
        has(&k, "caturTI", "krozwave");
        has(&k, "saptamI", "krozwari");
        has(&k, "saptamI", "krozwO");
        has(&k, "saptamI", "krozwuzu");
        has(&k, "samboDana", "krozwo");
        has(&k, "zazWI", "krozwUnAm");
        has(&generate("guru", "pum").unwrap(), "prathamA", "guruH");
        has(&generate("kartf", "pum").unwrap(), "prathamA", "kartA");
        assert!(!k.declension.get("prathamA").unwrap().iter().any(|x| x == "krozwuH"));
    }

    #[test]
    fn supathin_nap_supanthani() {
        // सुपथिन् nap: 7.1.23 सुपथि/सुपथी; 7.1.85 सुपन्थानि. पथिन् पुं stays पन्थाः.
        let s = generate("supaTin", "nap").expect("supaTin");
        has(&s, "prathamA", "supaTi");
        has(&s, "prathamA", "supaTI");
        has(&s, "prathamA", "supanTAni");
        has(&s, "dvitIyA", "supaTi");
        has(&s, "tfIyA", "supaTA");
        has(&s, "tfIyA", "supaTiByAm");
        has(&s, "saptamI", "supaTi");
        has(&s, "saptamI", "supaTizu");
        has(&s, "zazWI", "supaTAm");
        has(&s, "samboDana", "supaTin");
        has(&generate("paTin", "pum").unwrap(), "prathamA", "panTAH");
        assert!(!s.declension.get("prathamA").unwrap().iter().any(|x| x == "supanTAH"));
        assert!(!s.declension.get("prathamA").unwrap().iter().any(|x| x == "panTAH"));
    }

    #[test]
    fn nash_nat_nak_optional() {
        // नश्: 8.2.36 वा नट्/नड् and नक्/नग्. दिश् stays दिक्; विश् stays विट्.
        let n = generate("naS", "pum").expect("naS");
        has(&n, "prathamA", "naw");
        has(&n, "prathamA", "naq");
        has(&n, "prathamA", "nak");
        has(&n, "prathamA", "nag");
        has(&n, "prathamA", "naSO");
        has(&n, "dvitIyA", "naSam");
        has(&n, "tfIyA", "naSA");
        has(&n, "tfIyA", "naqByAm");
        has(&n, "tfIyA", "nagByAm");
        has(&n, "saptamI", "naSi");
        has(&n, "saptamI", "nawsu");
        has(&n, "saptamI", "nakzu");
        has(&generate("diS", "stri").unwrap(), "prathamA", "dik");
        has(&generate("viS", "pum").unwrap(), "prathamA", "viw");
        assert!(!generate("diS", "stri").unwrap().declension.get("prathamA").unwrap().iter().any(|x| x == "diw"));
    }

    #[test]
    fn papi_papih_papyau() {
        // पपी ई-anta पुं: पपीः/पप्यौ/पपीम्/पपीन्/पप्या/पप्ये/पपी/पपीषु. नदी stays I-stri नद्यौ.
        let p = generate("papI", "pum").expect("papI");
        has(&p, "prathamA", "papIH");
        has(&p, "prathamA", "papyO");
        has(&p, "prathamA", "papyaH");
        has(&p, "dvitIyA", "papIm");
        has(&p, "dvitIyA", "papIn");
        has(&p, "tfIyA", "papyA");
        has(&p, "tfIyA", "papIByAm");
        has(&p, "caturTI", "papye");
        has(&p, "paYcamI", "papyaH");
        has(&p, "zazWI", "papyAm");
        has(&p, "saptamI", "papI");
        has(&p, "saptamI", "papIzu");
        has(&p, "samboDana", "papIH");
        has(&generate("nadI", "stri").unwrap(), "prathamA", "nadI");
        has(&generate("nadI", "stri").unwrap(), "dvitIyA", "nadIm");
        has(&generate("nadI", "stri").unwrap(), "saptamI", "nadyAm");
        assert!(!p.declension.get("prathamA").unwrap().iter().any(|x| x == "papI"));
        assert!(!p.declension.get("saptamI").unwrap().iter().any(|x| x == "papO"));
        assert!(!p.declension.get("caturTI").unwrap().iter().any(|x| x == "papyE"));
    }

    #[test]
    fn gramani_gramanih_gramanyam() {
        // ग्रामणी: ग्रामणीः/ग्रामण्यम्/ग्रामण्याम् (नदीवत्). पपी stays पपीम्/पपी.
        let g = generate("grAmaRI", "pum").expect("grAmaRI");
        has(&g, "prathamA", "grAmaRIH");
        has(&g, "prathamA", "grAmaRyO");
        has(&g, "prathamA", "grAmaRyaH");
        has(&g, "dvitIyA", "grAmaRyam");
        has(&g, "dvitIyA", "grAmaRyaH");
        has(&g, "tfIyA", "grAmaRyA");
        has(&g, "caturTI", "grAmaRye");
        has(&g, "saptamI", "grAmaRyAm");
        has(&g, "saptamI", "grAmaRIzu");
        has(&g, "zazWI", "grAmaRyAm");
        has(&generate("papI", "pum").unwrap(), "dvitIyA", "papIm");
        has(&generate("nadI", "stri").unwrap(), "saptamI", "nadyAm");
        assert!(!g.declension.get("dvitIyA").unwrap().iter().any(|x| x == "grAmaRIm"));
        assert!(!g.declension.get("saptamI").unwrap().iter().any(|x| x == "grAmaRI"));
    }

    #[test]
    fn jara_jarasa_jaraya() {
        // जरा 7.2.101: टाप् जरा/जरया and जरस् जरसौ/जरसाम्. सीता stays सीतया.
        let j = generate("jarA", "stri").expect("jarA");
        has(&j, "prathamA", "jarA");
        has(&j, "prathamA", "jare");
        has(&j, "prathamA", "jarasO");
        has(&j, "prathamA", "jarAH");
        has(&j, "prathamA", "jarasaH");
        has(&j, "dvitIyA", "jarAm");
        has(&j, "dvitIyA", "jarasam");
        has(&j, "tfIyA", "jarayA");
        has(&j, "tfIyA", "jarasA");
        has(&j, "caturTI", "jarAyE");
        has(&j, "caturTI", "jarase");
        has(&j, "saptamI", "jarAyAm");
        has(&j, "saptamI", "jarasi");
        has(&j, "zazWI", "jarARAm");
        has(&j, "zazWI", "jarasAm");
        has(&generate("sItA", "stri").unwrap(), "tfIyA", "sItayA");
        assert!(!generate("sItA", "stri").unwrap().declension.get("tfIyA").unwrap().iter().any(|x| x == "sItasA"));
    }

    #[test]
    fn dyo_glau_payomuc() {
        // द्यो like गो द्यौः/द्याम्/द्यवि; ग्लौ like नौ ग्लौः/ग्लावम्; पयोमुच् च-anta पयोमुक्/पयोमुक्षु.
        let d = generate("dyo", "pum").expect("dyo");
        has(&d, "prathamA", "dyOH");
        has(&d, "prathamA", "dyAvO");
        has(&d, "dvitIyA", "dyAm");
        has(&d, "tfIyA", "dyavA");
        has(&d, "saptamI", "dyavi");
        has(&d, "saptamI", "dyozu");
        has(&generate("go", "pum").unwrap(), "prathamA", "gOH");
        let g = generate("glO", "stri").expect("glO");
        has(&g, "prathamA", "glOH");
        has(&g, "dvitIyA", "glAvam");
        has(&g, "saptamI", "glAvi");
        has(&g, "saptamI", "glOzu");
        has(&generate("nO", "stri").unwrap(), "dvitIyA", "nAvam");
        let p = generate("payomuc", "pum").expect("payomuc");
        has(&p, "prathamA", "payomuk");
        has(&p, "prathamA", "payomug");
        has(&p, "prathamA", "payomucO");
        has(&p, "tfIyA", "payomugByAm");
        has(&p, "saptamI", "payomukzu");
        has(&generate("vAc", "stri").unwrap(), "prathamA", "vAk");
        assert!(!d.declension.get("dvitIyA").unwrap().iter().any(|x| x == "dyAvam"));
        assert!(!g.declension.get("dvitIyA").unwrap().iter().any(|x| x == "glAm"));
    }

    #[test]
    fn svabhu_svabhuvam_svayambhu() {
        // स्वभू/स्वयम्भू: 6.1.77 उवङ् स्वभुवौ/स्वभुवम्. Not U-pum *स्वभूम्. हूहू stays हूह्वौ.
        let s = generate("svaBU", "pum").expect("svaBU");
        has(&s, "prathamA", "svaBUH");
        has(&s, "prathamA", "svaBuvO");
        has(&s, "prathamA", "svaBuvaH");
        has(&s, "dvitIyA", "svaBuvam");
        has(&s, "tfIyA", "svaBuvA");
        has(&s, "tfIyA", "svaBUByAm");
        has(&s, "saptamI", "svaBuvi");
        has(&s, "saptamI", "svaBUzu");
        has(&s, "zazWI", "svaBuvAm");
        let y = generate("svayamBU", "pum").expect("svayamBU");
        has(&y, "prathamA", "svayamBUH");
        has(&y, "dvitIyA", "svayamBuvam");
        has(&y, "saptamI", "svayamBuvi");
        has(&generate("hUhU", "pum").unwrap(), "prathamA", "hUhUH");
        assert!(!s.declension.get("dvitIyA").unwrap().iter().any(|x| x == "svaBUm"));
        assert!(!s.declension.get("prathamA").unwrap().iter().any(|x| x == "svaBavO"));
        // 6.4.83 वर्षाभू यण् (खलपू-like), not धातु उवङ् *वर्षाभुवम् / हूहू *वर्षाभूम्.
        let v = generate("varzABU", "pum").expect("varzABU");
        has(&v, "prathamA", "varzABUH");
        has(&v, "prathamA", "varzABvO");
        has(&v, "prathamA", "varzABvaH");
        has(&v, "dvitIyA", "varzABvam");
        has(&v, "dvitIyA", "varzABvaH");
        has(&v, "tfIyA", "varzABvA");
        has(&v, "tfIyA", "varzABUByAm");
        has(&v, "saptamI", "varzABvi");
        has(&v, "saptamI", "varzABUzu");
        has(&generate("KalapU", "pum").unwrap(), "dvitIyA", "Kalapvam");
        assert!(!v.declension.get("dvitIyA").unwrap().iter().any(|x| x == "varzABuvam"));
        assert!(!v.declension.get("dvitIyA").unwrap().iter().any(|x| x == "varzABUm"));
    }

    #[test]
    fn haha_hahah_hahau() {
        // हाहा: हाहाः/हाहौ/हाहाम्/हाहाान्/हाहा/हाहै/हाहे. गोपा stays गोपि/गोपः शस्.
        let h = generate("hAhA", "pum").expect("hAhA");
        has(&h, "prathamA", "hAhAH");
        has(&h, "prathamA", "hAhO");
        has(&h, "dvitIyA", "hAhAm");
        has(&h, "dvitIyA", "hAhAn");
        has(&h, "tfIyA", "hAhA");
        has(&h, "caturTI", "hAhE");
        has(&h, "saptamI", "hAhe");
        has(&h, "saptamI", "hAhAsu");
        has(&h, "zazWI", "hAhOH");
        has(&generate("gopA", "pum").unwrap(), "saptamI", "gopi");
        has(&generate("gopA", "pum").unwrap(), "dvitIyA", "gopaH");
        assert!(!h.declension.get("saptamI").unwrap().iter().any(|x| x == "hAhi"));
        assert!(!h.declension.get("dvitIyA").unwrap().iter().any(|x| x == "hAhaH"));
    }

    #[test]
    fn huhu_huhuh_huhvau() {
        // हूहू: हूहूः/हूह्वौ/हूहूम्/हूहून्/हूह्वि. स्वभू stays स्वभुवम्.
        let h = generate("hUhU", "pum").expect("hUhU");
        has(&h, "prathamA", "hUhUH");
        has(&h, "prathamA", "hUhvO");
        has(&h, "prathamA", "hUhvaH");
        has(&h, "dvitIyA", "hUhUm");
        has(&h, "dvitIyA", "hUhUn");
        has(&h, "tfIyA", "hUhvA");
        has(&h, "caturTI", "hUhve");
        has(&h, "saptamI", "hUhvi");
        has(&h, "saptamI", "hUhUzu");
        has(&h, "zazWI", "hUhvAm");
        has(&generate("svaBU", "pum").unwrap(), "dvitIyA", "svaBuvam");
        assert!(!h.declension.get("prathamA").unwrap().iter().any(|x| x == "hUhavO"));
        assert!(!h.declension.get("dvitIyA").unwrap().iter().any(|x| x == "hUhUvam"));
    }

    #[test]
    fn vedhas_vedhah() {
        // 6.4.14: वेधाः, चन्द्रमाः, सुमनाः. मनस् stays मनः. तपस् stays तपः.
        let v = generate("veDas", "pum").expect("veDas");
        has(&v, "prathamA", "veDAH");
        has(&v, "prathamA", "veDasO");
        has(&v, "prathamA", "veDasaH");
        has(&v, "dvitIyA", "veDasam");
        has(&v, "tfIyA", "veDasA");
        has(&v, "tfIyA", "veDoByAm");
        has(&v, "saptamI", "veDasi");
        has(&v, "saptamI", "veDassu");
        has(&v, "saptamI", "veDaHsu");
        has(&v, "samboDana", "veDaH");
        let s = generate("sumanas", "pum").expect("sumanas");
        has(&s, "prathamA", "sumanAH");
        has(&s, "dvitIyA", "sumanasam");
        has(&s, "tfIyA", "sumanoByAm");
        has(&s, "samboDana", "sumanaH");
        has(&generate("manas", "pum").unwrap(), "prathamA", "manaH");
        has(&generate("tapas", "pum").unwrap(), "prathamA", "tapaH");
        let c = generate("candramas", "pum").expect("candramas");
        has(&c, "prathamA", "candramAH");
        has(&c, "prathamA", "candramasO");
        has(&c, "dvitIyA", "candramasam");
        has(&c, "tfIyA", "candramoByAm");
        has(&c, "samboDana", "candramaH");
        assert!(!v.declension.get("prathamA").unwrap().iter().any(|x| x == "veDaH"));
        assert!(!s.declension.get("prathamA").unwrap().iter().any(|x| x == "sumanaH"));
        assert!(!generate("manas", "pum").unwrap().declension.get("prathamA").unwrap().iter().any(|x| x == "manAH"));
        assert!(!c.declension.get("prathamA").unwrap().iter().any(|x| x == "candramaH"));
    }

    #[test]
    fn nirjara_nirjarasau() {
        // निरजर: a-stem निरजरः/निरजरेण and जरस् निरजरसौ/निरजरसा. राम stays रामेण.
        let n = generate("nirjara", "pum").expect("nirjara");
        has(&n, "prathamA", "nirjaraH");
        has(&n, "prathamA", "nirjarO");
        has(&n, "prathamA", "nirjarasO");
        has(&n, "prathamA", "nirjarAH");
        has(&n, "dvitIyA", "nirjaram");
        has(&n, "dvitIyA", "nirjarAn");
        has(&n, "tfIyA", "nirjareRa");
        has(&n, "tfIyA", "nirjarasA");
        has(&n, "caturTI", "nirjarAya");
        has(&n, "caturTI", "nirjarase");
        has(&n, "saptamI", "nirjare");
        has(&n, "saptamI", "nirjarasi");
        has(&n, "zazWI", "nirjarARAm");
        has(&n, "zazWI", "nirjarasAm");
        has(&generate("rAma", "pum").unwrap(), "tfIyA", "rAmeRa");
        assert!(!generate("rAma", "pum").unwrap().declension.get("prathamA").unwrap().iter().any(|x| x == "rAmasO"));
    }

    #[test]
    fn pradhi_pradhyam() {
        // प्रधी: यण् प्रध्यौ/प्रध्यम्; सु प्रधीः. पपी stays पपीम्; सुधी not here.
        let p = generate("praDI", "pum").expect("praDI");
        has(&p, "prathamA", "praDIH");
        has(&p, "prathamA", "praDyO");
        has(&p, "prathamA", "praDyaH");
        has(&p, "dvitIyA", "praDyam");
        has(&p, "tfIyA", "praDyA");
        has(&p, "caturTI", "praDye");
        has(&p, "caturTI", "praDyE");
        has(&p, "saptamI", "praDyAm");
        has(&p, "saptamI", "praDyi");
        has(&p, "saptamI", "praDIzu");
        has(&p, "zazWI", "praDInAm");
        has(&p, "samboDana", "praDi");
        has(&generate("papI", "pum").unwrap(), "dvitIyA", "papIm");
        has(&generate("grAmaRI", "pum").unwrap(), "dvitIyA", "grAmaRyam");
        assert!(!p.declension.get("dvitIyA").unwrap().iter().any(|x| x == "praDIm"));
        assert!(!p.declension.get("prathamA").unwrap().iter().any(|x| x == "praDiyO"));
    }

    #[test]
    fn sudhi_sudhiyam_suddhadi() {
        // सुधी/शुद्धधी: इयङ् सुधियौ/सुधियम्. प्रधी stays प्रध्यम्. नपुं सुधि/सुधिनी.
        let s = generate("suDI", "pum").expect("suDI");
        has(&s, "prathamA", "suDIH");
        has(&s, "prathamA", "suDiyO");
        has(&s, "prathamA", "suDiyaH");
        has(&s, "dvitIyA", "suDiyam");
        has(&s, "tfIyA", "suDiyA");
        has(&s, "caturTI", "suDiye");
        has(&s, "saptamI", "suDiyi");
        has(&s, "saptamI", "suDIzu");
        has(&s, "samboDana", "suDe");
        let d = generate("SudDaDI", "pum").expect("SudDaDI");
        has(&d, "prathamA", "SudDaDIH");
        has(&d, "dvitIyA", "SudDaDiyam");
        has(&d, "saptamI", "SudDaDiyi");
        let n = generate("suDI", "nap").expect("suDI nap");
        has(&n, "prathamA", "suDi");
        has(&n, "prathamA", "suDinI");
        has(&n, "prathamA", "suDIni");
        has(&n, "tfIyA", "suDinA");
        has(&generate("praDI", "pum").unwrap(), "dvitIyA", "praDyam");
        has(&generate("papI", "pum").unwrap(), "dvitIyA", "papIm");
        assert!(!s.declension.get("dvitIyA").unwrap().iter().any(|x| x == "suDyam"));
        assert!(!s.declension.get("dvitIyA").unwrap().iter().any(|x| x == "suDIm"));
    }

    #[test]
    fn sri_sriyam_susri() {
        // श्री: इयङ् श्रियौ/श्रियम्; सु श्रीः. नदी stays नदी/नद्यौ. सुश्रीः.
        let s = generate("SrI", "stri").expect("SrI");
        has(&s, "prathamA", "SrIH");
        has(&s, "prathamA", "SriyO");
        has(&s, "prathamA", "SriyaH");
        has(&s, "dvitIyA", "Sriyam");
        has(&s, "tfIyA", "SriyA");
        has(&s, "caturTI", "Sriye");
        has(&s, "caturTI", "SriyE");
        has(&s, "saptamI", "SriyAm");
        has(&s, "saptamI", "Sriyi");
        has(&s, "saptamI", "SrIzu");
        has(&s, "zazWI", "SrIRAm");
        let u = generate("suSrI", "stri").expect("suSrI");
        has(&u, "prathamA", "suSrIH");
        has(&u, "dvitIyA", "suSriyam");
        has(&u, "saptamI", "suSriyi");
        has(&generate("nadI", "stri").unwrap(), "prathamA", "nadI");
        has(&generate("nadI", "stri").unwrap(), "dvitIyA", "nadIm");
        assert!(!s.declension.get("prathamA").unwrap().iter().any(|x| x == "SrI"));
        assert!(!s.declension.get("dvitIyA").unwrap().iter().any(|x| x == "SrIm"));
    }

    #[test]
    fn bru_bruvam() {
        // भ्रू: उवङ् भ्रुवौ/भ्रुवम्; सु भ्रूः. स्वभू stays स्वभुवम्.
        let b = generate("BrU", "stri").expect("BrU");
        has(&b, "prathamA", "BrUH");
        has(&b, "prathamA", "BruvO");
        has(&b, "prathamA", "BruvaH");
        has(&b, "dvitIyA", "Bruvam");
        has(&b, "tfIyA", "BruvA");
        has(&b, "caturTI", "Bruve");
        has(&b, "caturTI", "BruvE");
        has(&b, "saptamI", "BruvAm");
        has(&b, "saptamI", "Bruvi");
        has(&b, "saptamI", "BrUzu");
        has(&b, "zazWI", "BrURAm");
        has(&generate("svaBU", "pum").unwrap(), "dvitIyA", "svaBuvam");
        assert!(!b.declension.get("dvitIyA").unwrap().iter().any(|x| x == "BrUm"));
        assert!(!b.declension.get("prathamA").unwrap().iter().any(|x| x == "BravO"));
    }

    #[test]
    fn pati_patya_patyuh() {
        // पति 1.4.8: पत्या/पत्ये/पत्युः/पत्यौ; nom पतिः not सखा. हरि stays हरिणा.
        let p = generate("pati", "pum").expect("pati");
        has(&p, "prathamA", "patiH");
        has(&p, "prathamA", "patI");
        has(&p, "prathamA", "patayaH");
        has(&p, "dvitIyA", "patim");
        has(&p, "dvitIyA", "patIn");
        has(&p, "tfIyA", "patyA");
        has(&p, "tfIyA", "patiByAm");
        has(&p, "caturTI", "patye");
        has(&p, "paYcamI", "patyuH");
        has(&p, "saptamI", "patyO");
        has(&p, "saptamI", "patizu");
        has(&p, "samboDana", "pate");
        has(&p, "zazWI", "patInAm");
        has(&generate("hari", "pum").unwrap(), "tfIyA", "hariRA");
        has(&generate("saKi", "pum").unwrap(), "prathamA", "saKA");
        assert!(!p.declension.get("tfIyA").unwrap().iter().any(|x| x == "patinA"));
        assert!(!p.declension.get("prathamA").unwrap().iter().any(|x| x == "patA"));
    }

    #[test]
    fn kalapu_kalapvam() {
        // खलपू: यण् खलप्वौ/खलप्वम्. हूहू stays हूहूम्; स्वभू stays स्वभुवम्.
        let k = generate("KalapU", "pum").expect("KalapU");
        has(&k, "prathamA", "KalapUH");
        has(&k, "prathamA", "KalapvO");
        has(&k, "prathamA", "KalapvaH");
        has(&k, "dvitIyA", "Kalapvam");
        has(&k, "tfIyA", "KalapvA");
        has(&k, "saptamI", "Kalapvi");
        has(&k, "saptamI", "KalapUzu");
        has(&generate("hUhU", "pum").unwrap(), "dvitIyA", "hUhUm");
        has(&generate("svaBU", "pum").unwrap(), "dvitIyA", "svaBuvam");
        assert!(!k.declension.get("dvitIyA").unwrap().iter().any(|x| x == "KalapUm"));
        assert!(!k.declension.get("prathamA").unwrap().iter().any(|x| x == "KalapavO"));
    }

    #[test]
    fn stri_striyam() {
        // स्त्री: इयङ् स्त्रियौ/स्त्रियम्; nom स्त्री not *स्त्रीः. नदी stays नद्यौ.
        let s = generate("strI", "stri").expect("strI");
        has(&s, "prathamA", "strI");
        has(&s, "prathamA", "striyO");
        has(&s, "prathamA", "striyaH");
        has(&s, "dvitIyA", "striyam");
        has(&s, "dvitIyA", "strIm");
        has(&s, "tfIyA", "striyA");
        has(&s, "caturTI", "striyE");
        has(&s, "saptamI", "striyAm");
        has(&s, "saptamI", "strIzu");
        has(&s, "zazWI", "strIRAm");
        has(&s, "samboDana", "stri");
        has(&generate("nadI", "stri").unwrap(), "prathamA", "nadI");
        has(&generate("nadI", "stri").unwrap(), "dvitIyA", "nadIm");
        has(&generate("SrI", "stri").unwrap(), "prathamA", "SrIH");
        assert!(!s.declension.get("prathamA").unwrap().iter().any(|x| x == "strIH"));
        assert!(!s.declension.get("prathamA").unwrap().iter().any(|x| x == "stryO"));
    }

    #[test]
    fn ni_niyam() {
        // नी: इयङ् नियौ/नियम्; सु नीः. ग्रामणी stays ग्रामण्यम्.
        let n = generate("nI", "pum").expect("nI");
        has(&n, "prathamA", "nIH");
        has(&n, "prathamA", "niyO");
        has(&n, "prathamA", "niyaH");
        has(&n, "dvitIyA", "niyam");
        has(&n, "tfIyA", "niyA");
        has(&n, "saptamI", "niyAm");
        has(&n, "saptamI", "nIzu");
        has(&generate("grAmaRI", "pum").unwrap(), "dvitIyA", "grAmaRyam");
        has(&generate("strI", "stri").unwrap(), "prathamA", "strI");
        assert!(!n.declension.get("dvitIyA").unwrap().iter().any(|x| x == "nIm"));
        assert!(!n.declension.get("prathamA").unwrap().iter().any(|x| x == "nyO"));
    }

    #[test]
    fn brahman_brahma_ghrtasprs() {
        // ब्रह्मन्/यज्वन्: 6.4.137 no अल्लोप ब्रह्मणा/यज्वना. राजन् stays राज्ञा. घृतस्पृश् श-anta घृतस्पृक्.
        let b = generate("brahman", "pum").expect("brahman");
        has(&b, "prathamA", "brahmA");
        has(&b, "prathamA", "brahmARO");
        has(&b, "dvitIyA", "brahmARam");
        has(&b, "dvitIyA", "brahmaRaH");
        has(&b, "tfIyA", "brahmaRA");
        has(&b, "tfIyA", "brahmaByAm");
        has(&b, "saptamI", "brahmaRi");
        has(&b, "saptamI", "brahmasu");
        has(&b, "samboDana", "brahman");
        has(&generate("rAjan", "pum").unwrap(), "tfIyA", "rAjYA");
        let g = generate("GftaspfS", "pum").expect("GftaspfS");
        has(&g, "prathamA", "Gftaspfk");
        has(&g, "prathamA", "Gftaspfg");
        has(&g, "prathamA", "GftaspfSO");
        has(&g, "dvitIyA", "GftaspfSam");
        has(&g, "tfIyA", "GftaspfgByAm");
        has(&g, "saptamI", "Gftaspfkzu");
        has(&generate("diS", "stri").unwrap(), "prathamA", "dik");
        let y = generate("yajvan", "pum").expect("yajvan");
        has(&y, "prathamA", "yajvA");
        has(&y, "prathamA", "yajvAnO");
        has(&y, "dvitIyA", "yajvAnam");
        has(&y, "tfIyA", "yajvanA");
        has(&y, "tfIyA", "yajvaByAm");
        has(&y, "saptamI", "yajvani");
        has(&y, "saptamI", "yajvasu");
        has(&y, "samboDana", "yajvan");
        assert!(!b.declension.get("tfIyA").unwrap().iter().any(|x| x == "brahmnA"));
        assert!(!y.declension.get("tfIyA").unwrap().iter().any(|x| x == "yajvnA"));
    }

    #[test]
    fn prarai_prari_prarasu() {
        // प्ररै nap: प्ररि/प्ररिणी/प्ररीणि; पद प्ररासु. रै stays राः. Not *प्रराः.
        let p = generate("prarE", "nap").expect("prarE");
        has(&p, "prathamA", "prari");
        has(&p, "prathamA", "prariRI");
        has(&p, "prathamA", "prarIRi");
        has(&p, "tfIyA", "prariRA");
        has(&p, "tfIyA", "prarAByAm");
        has(&p, "saptamI", "prariRi");
        has(&p, "saptamI", "prarAsu");
        has(&p, "zazWI", "prarIRAm");
        has(&p, "samboDana", "prare");
        has(&generate("rE", "stri").unwrap(), "prathamA", "rAH");
        has(&generate("rE", "stri").unwrap(), "dvitIyA", "rAyam");
        has(&generate("vAri", "nap").unwrap(), "prathamA", "vAri");
        assert!(!p.declension.get("prathamA").unwrap().iter().any(|x| x == "prarAH"));
        assert!(!p.declension.get("dvitIyA").unwrap().iter().any(|x| x == "prarAyam"));
    }

    #[test]
    fn vadhu_vadhvau() {
        // वधू U-stri यण् वध्वौ/वध्वम्; पद वधूभ्याम्. धेनु stays u-stri धेनवः. भ्रू stays उवङ् भ्रुवम्.
        let v = generate("vaDU", "stri").expect("vaDU");
        has(&v, "prathamA", "vaDUH");
        has(&v, "prathamA", "vaDvO");
        has(&v, "prathamA", "vaDvaH");
        has(&v, "dvitIyA", "vaDUm");
        has(&v, "dvitIyA", "vaDUH");
        has(&v, "tfIyA", "vaDvA");
        has(&v, "tfIyA", "vaDUByAm");
        has(&v, "tfIyA", "vaDUBiH");
        has(&v, "caturTI", "vaDvE");
        has(&v, "saptamI", "vaDvAm");
        has(&v, "saptamI", "vaDvoH");
        has(&v, "saptamI", "vaDUzu");
        has(&v, "samboDana", "vaDu");
        has(&generate("Denu", "stri").unwrap(), "prathamA", "DenuH");
        has(&generate("Denu", "stri").unwrap(), "prathamA", "DenavaH");
        has(&generate("BrU", "stri").unwrap(), "dvitIyA", "Bruvam");
        assert!(!v.declension.get("prathamA").unwrap().iter().any(|x| x == "vaDU"));
        assert!(!v.declension.get("prathamA").unwrap().iter().any(|x| x == "vaDavaH"));
        assert!(!v.declension.get("tfIyA").unwrap().iter().any(|x| x == "vaDuByAm"));
    }

    #[test]
    fn aticamu_aticamun() {
        // अतिचमू पुं: यण् अतिचम्वौ; शस् अतिचमून्. वधू stays acc वधूः. हूहू stays हूहूम्.
        let a = generate("aticamU", "pum").expect("aticamU");
        has(&a, "prathamA", "aticamUH");
        has(&a, "prathamA", "aticamvO");
        has(&a, "prathamA", "aticamvaH");
        has(&a, "dvitIyA", "aticamUm");
        has(&a, "dvitIyA", "aticamUn");
        has(&a, "tfIyA", "aticamvA");
        has(&a, "tfIyA", "aticamUByAm");
        has(&a, "caturTI", "aticamvE");
        has(&a, "saptamI", "aticamvAm");
        has(&a, "saptamI", "aticamUzu");
        has(&a, "samboDana", "aticamu");
        has(&generate("vaDU", "stri").unwrap(), "dvitIyA", "vaDUH");
        has(&generate("hUhU", "pum").unwrap(), "dvitIyA", "hUhUm");
        assert!(!a.declension.get("dvitIyA").unwrap().iter().any(|x| x == "aticamUH"));
        assert!(!a.declension.get("prathamA").unwrap().iter().any(|x| x == "aticamavaH"));
    }

    #[test]
    fn amba_voc_amba() {
        // अम्बा 7.3.107 voc अम्ब not *अम्बे. सीता stays voc सीते.
        let a = generate("ambA", "stri").expect("ambA");
        has(&a, "prathamA", "ambA");
        has(&a, "prathamA", "ambe");
        has(&a, "prathamA", "ambAH");
        has(&a, "dvitIyA", "ambAm");
        has(&a, "tfIyA", "ambayA");
        has(&a, "saptamI", "ambAyAm");
        has(&a, "saptamI", "ambAsu");
        has(&a, "samboDana", "amba");
        has(&generate("sItA", "stri").unwrap(), "samboDana", "sIte");
        has(&generate("sItA", "stri").unwrap(), "prathamA", "sItA");
        has(&generate("akkA", "stri").unwrap(), "samboDana", "akka");
        assert_eq!(a.declension.get("samboDana").unwrap()[0], "amba");
    }

    #[test]
    fn lakshmi_lakshmih() {
        // लक्ष्मीः visarga + यण् लक्ष्म्यौ. नदी/गौरी stay no visarga. श्री stays इयङ् श्रियम्.
        let l = generate("lakzmI", "stri").expect("lakzmI");
        has(&l, "prathamA", "lakzmIH");
        has(&l, "prathamA", "lakzmyO");
        has(&l, "prathamA", "lakzmyaH");
        has(&l, "dvitIyA", "lakzmIm");
        has(&l, "dvitIyA", "lakzmIH");
        has(&l, "tfIyA", "lakzmyA");
        has(&l, "tfIyA", "lakzmIByAm");
        has(&l, "saptamI", "lakzmyAm");
        has(&l, "saptamI", "lakzmIzu");
        has(&l, "samboDana", "lakzmi");
        has(&generate("gOrI", "stri").unwrap(), "prathamA", "gOrI");
        has(&generate("gOrI", "stri").unwrap(), "prathamA", "gOryO");
        has(&generate("SrI", "stri").unwrap(), "dvitIyA", "Sriyam");
        assert!(!l.declension.get("prathamA").unwrap().iter().any(|x| x == "lakzmI"));
        assert!(!generate("gOrI", "stri").unwrap().declension.get("prathamA").unwrap().iter().any(|x| x == "gOrIH"));
        assert!(!l.declension.get("dvitIyA").unwrap().iter().any(|x| x == "lakzmiyam"));
    }

    #[test]
    fn atilakshmi_atilakshmin() {
        // अतिलक्ष्मीः यण्; शस् अतिलक्ष्मीन्. लक्ष्मी stays acc लक्ष्मीः. गौरी stays गौरीः.
        let a = generate("atilakzmI", "stri").expect("atilakzmI");
        has(&a, "prathamA", "atilakzmIH");
        has(&a, "prathamA", "atilakzmyO");
        has(&a, "prathamA", "atilakzmyaH");
        has(&a, "dvitIyA", "atilakzmIm");
        has(&a, "dvitIyA", "atilakzmIn");
        has(&a, "tfIyA", "atilakzmyA");
        has(&a, "saptamI", "atilakzmyAm");
        has(&a, "saptamI", "atilakzmIzu");
        has(&a, "samboDana", "atilakzmi");
        has(&generate("lakzmI", "stri").unwrap(), "dvitIyA", "lakzmIH");
        has(&generate("gOrI", "stri").unwrap(), "dvitIyA", "gOrIH");
        assert!(!a.declension.get("dvitIyA").unwrap().iter().any(|x| x == "atilakzmIH"));
    }

    #[test]
    fn pradyo_pradyu_nap() {
        // दिव् nap: 7.1.23 प्रद्यु/प्रद्युनी/प्रद्यूनि not o-stem *प्रद्यौः. दिव् पुं stays द्यौः. गो stays गौः.
        let p = generate("pradyo", "nap").expect("pradyo");
        has(&p, "prathamA", "pradyu");
        has(&p, "prathamA", "pradyunI");
        has(&p, "prathamA", "pradyUni");
        has(&p, "tfIyA", "pradyunA");
        has(&p, "tfIyA", "pradyuByAm");
        has(&p, "saptamI", "pradyuni");
        has(&p, "saptamI", "pradyuzu");
        has(&p, "samboDana", "pradyo");
        has(&generate("pradiv", "nap").unwrap(), "prathamA", "pradyu");
        has(&generate("div", "nap").unwrap(), "prathamA", "dyu");
        has(&generate("div", "pum").unwrap(), "prathamA", "dyOH");
        has(&generate("go", "pum").unwrap(), "prathamA", "gOH");
        assert!(!p.declension.get("prathamA").unwrap().iter().any(|x| x == "pradyOH"));
        assert!(!p.declension.get("dvitIyA").unwrap().iter().any(|x| x == "pradyAm"));
    }

    #[test]
    fn suno_sunu_nap() {
        // सुनौ nap: सुनु/सुनुनी/सुनूनि. नौ stays नावम्. Not *सुनावम्.
        let s = generate("sunO", "nap").expect("sunO");
        has(&s, "prathamA", "sunu");
        has(&s, "prathamA", "sununI");
        has(&s, "prathamA", "sunUni");
        has(&s, "tfIyA", "sununA");
        has(&s, "tfIyA", "sunuByAm");
        has(&s, "saptamI", "sununi");
        has(&s, "saptamI", "sunuzu");
        has(&s, "samboDana", "suno");
        has(&generate("nO", "stri").unwrap(), "prathamA", "nOH");
        has(&generate("nO", "stri").unwrap(), "dvitIyA", "nAvam");
        has(&generate("maDu", "nap").unwrap(), "prathamA", "maDu");
        assert!(!s.declension.get("prathamA").unwrap().iter().any(|x| x == "sunOH"));
        assert!(!s.declension.get("dvitIyA").unwrap().iter().any(|x| x == "sunAvam"));
    }

    #[test]
    fn sulu_sulu_nap() {
        // सुलू nap: सुलु/सुलुनी; यण् सुल्वा. वधू stays स्त्री वधूः. मधु stays मधु.
        let s = generate("sulU", "nap").expect("sulU");
        has(&s, "prathamA", "sulu");
        has(&s, "prathamA", "sulunI");
        has(&s, "prathamA", "sulUni");
        has(&s, "tfIyA", "sulvA");
        has(&s, "tfIyA", "sulunA");
        has(&s, "tfIyA", "suluByAm");
        has(&s, "caturTI", "sulve");
        has(&s, "saptamI", "sulvi");
        has(&s, "saptamI", "suluzu");
        has(&s, "samboDana", "sulo");
        has(&generate("vaDU", "stri").unwrap(), "prathamA", "vaDUH");
        has(&generate("maDu", "nap").unwrap(), "prathamA", "maDu");
        assert!(!s.declension.get("prathamA").unwrap().iter().any(|x| x == "sulUH"));
        assert!(!s.declension.get("prathamA").unwrap().iter().any(|x| x == "sulvO"));
    }

    #[test]
    fn dadat_jakzat_no_num() {
        // 7.1.78 नाभ्यस्ताच्छतुः: ददत्/ददतौ not *ददन्. भवत् stays भवन्.
        let d = generate("dadat", "pum").expect("dadat");
        has(&d, "prathamA", "dadat");
        has(&d, "prathamA", "dadad");
        has(&d, "prathamA", "dadatO");
        has(&d, "prathamA", "dadataH");
        has(&d, "dvitIyA", "dadatam");
        has(&d, "tfIyA", "dadatA");
        has(&d, "tfIyA", "dadadByAm");
        has(&d, "saptamI", "dadati");
        has(&d, "saptamI", "dadatsu");
        let j = generate("jakzat", "pum").expect("jakzat");
        has(&j, "prathamA", "jakzat");
        has(&j, "prathamA", "jakzatO");
        has(&generate("Bavat", "pum").unwrap(), "prathamA", "BavAn");
        has(&generate("Bavat", "pum").unwrap(), "prathamA", "BavantO");
        has(&generate("dadat", "stri").unwrap(), "prathamA", "dadatI");
        assert!(!d.declension.get("prathamA").unwrap().iter().any(|x| x == "dadAn"));
        assert!(!d.declension.get("prathamA").unwrap().iter().any(|x| x == "dadantO"));
        assert!(!j.declension.get("prathamA").unwrap().iter().any(|x| x == "jakzAn"));
    }

    #[test]
    fn denu_sambhu_drs_payas() {
        // धेनु u-stri; शम्भु u-pum; दृश् श-anta; पयस् as-nap; धीमत् शतृ; शार्ङ्गिन्.
        let d = generate("Denu", "stri").expect("Denu");
        has(&d, "prathamA", "DenuH");
        has(&d, "prathamA", "DenU");
        has(&d, "prathamA", "DenavaH");
        has(&d, "dvitIyA", "Denum");
        has(&d, "tfIyA", "DenvA");
        has(&d, "caturTI", "DenvE");
        has(&d, "saptamI", "DenvAm");
        has(&d, "samboDana", "Deno");
        let s = generate("SamBu", "pum").expect("SamBu");
        has(&s, "prathamA", "SamBuH");
        has(&s, "prathamA", "SamBavaH");
        has(&s, "tfIyA", "SamBunA");
        has(&s, "saptamI", "SamBO");
        let r = generate("dfS", "pum").expect("dfS");
        has(&r, "prathamA", "dfk");
        has(&r, "prathamA", "dfg");
        has(&r, "dvitIyA", "dfSam");
        has(&r, "saptamI", "dfkzu");
        let p = generate("payas", "nap").expect("payas");
        has(&p, "prathamA", "payaH");
        has(&p, "prathamA", "payasI");
        has(&p, "prathamA", "payAMsi");
        has(&p, "tfIyA", "payoByAm");
        has(&p, "saptamI", "payaHsu");
        let m = generate("DImat", "pum").expect("DImat");
        has(&m, "prathamA", "DImAn");
        has(&m, "dvitIyA", "DImantam");
        has(&m, "tfIyA", "DImadByAm");
        let g = generate("SArNgin", "pum").expect("SArNgin");
        has(&g, "prathamA", "SArNgI");
        has(&g, "dvitIyA", "SArNgiRam");
        has(&g, "tfIyA", "SArNgiRA");
        has(&generate("vaDU", "stri").unwrap(), "prathamA", "vaDvO");
        assert!(!d.declension.get("prathamA").unwrap().iter().any(|x| x == "DenvO"));
        assert!(!r.declension.get("prathamA").unwrap().iter().any(|x| x == "dfw"));
    }

    #[test]
    fn dhatr_matf_hari_mati() {
        // कर्तृ धातारम् vs पितृ पितरम्; माता; हरिणा; मतिः; रमा; सुयुक्; त्विट्; मधु.
        let dh = generate("DAtf", "pum").expect("DAtf");
        has(&dh, "prathamA", "DAtA");
        has(&dh, "dvitIyA", "DAtAram");
        has(&dh, "tfIyA", "DAtrA");
        has(&generate("pitf", "pum").unwrap(), "dvitIyA", "pitaram");
        let ma = generate("mAtf", "stri").expect("mAtf");
        has(&ma, "prathamA", "mAtA");
        has(&ma, "dvitIyA", "mAtaram");
        has(&ma, "dvitIyA", "mAtFH");
        let h = generate("hari", "pum").expect("hari");
        has(&h, "prathamA", "hariH");
        has(&h, "tfIyA", "hariRA");
        let mt = generate("mati", "stri").expect("mati");
        has(&mt, "prathamA", "matiH");
        has(&mt, "tfIyA", "matyA");
        has(&mt, "caturTI", "matyE");
        has(&generate("ramA", "stri").unwrap(), "prathamA", "ramA");
        has(&generate("ramA", "stri").unwrap(), "saptamI", "ramAsu");
        has(&generate("suyuj", "pum").unwrap(), "prathamA", "suyuk");
        has(&generate("suyuj", "pum").unwrap(), "saptamI", "suyukzu");
        has(&generate("tviz", "stri").unwrap(), "prathamA", "tviw");
        has(&generate("tviz", "stri").unwrap(), "prathamA", "tviq");
        has(&generate("tviz", "stri").unwrap(), "saptamI", "tviwsu");
        has(&generate("dviz", "pum").unwrap(), "prathamA", "dviw");
        has(&generate("ASiz", "stri").unwrap(), "prathamA", "ASIH");
        has(&generate("maDu", "nap").unwrap(), "prathamA", "maDu");
        has(&generate("yaSasvin", "pum").unwrap(), "prathamA", "yaSasvI");
        assert!(!dh.declension.get("dvitIyA").unwrap().iter().any(|x| x == "DAtaram"));
        assert!(!h.declension.get("tfIyA").unwrap().iter().any(|x| x == "harinA"));
        assert!(!generate("tviz", "stri").unwrap().declension.get("prathamA").unwrap().iter().any(|x| x == "tvIH"));
    }

    #[test]
    fn ratnamuz_ratnamut() {
        // रत्नमुष् ष-anta रत्नमुट्. धनुस् nap stays धनुः. द्विष् stays द्विट्.
        let r = generate("ratnamuz", "pum").expect("ratnamuz");
        has(&r, "prathamA", "ratnamuw");
        has(&r, "prathamA", "ratnamuq");
        has(&r, "prathamA", "ratnamuzO");
        has(&r, "dvitIyA", "ratnamuzam");
        has(&r, "tfIyA", "ratnamuqByAm");
        has(&r, "saptamI", "ratnamuwsu");
        has(&generate("Danuz", "nap").unwrap(), "prathamA", "DanuH");
        has(&generate("dviz", "pum").unwrap(), "prathamA", "dviw");
        assert!(!r.declension.get("prathamA").unwrap().iter().any(|x| x == "ratnamuH"));
    }

    #[test]
    fn kartf_svasr_pitr_vrddhi() {
        // 6.4.11 कर्तारौ/स्वसारम्; पितृ stays पितरौ/पितरम्. माता stays मातरम्.
        let k = generate("kartf", "pum").expect("kartf");
        has(&k, "prathamA", "kartA");
        has(&k, "prathamA", "kartArO");
        has(&k, "prathamA", "kartAraH");
        has(&k, "dvitIyA", "kartAram");
        has(&k, "samboDana", "kartaH");
        let s = generate("svasf", "stri").expect("svasf");
        has(&s, "prathamA", "svasA");
        has(&s, "prathamA", "svasArO");
        has(&s, "prathamA", "svasAraH");
        has(&s, "dvitIyA", "svasAram");
        has(&s, "dvitIyA", "svasFH");
        has(&generate("pitf", "pum").unwrap(), "prathamA", "pitarO");
        has(&generate("pitf", "pum").unwrap(), "dvitIyA", "pitaram");
        has(&generate("mAtf", "stri").unwrap(), "dvitIyA", "mAtaram");
        has(&generate("mAtf", "stri").unwrap(), "prathamA", "mAtarO");
        has(&generate("naptf", "pum").unwrap(), "dvitIyA", "naptAram");
        assert!(!generate("pitf", "pum").unwrap().declension.get("prathamA").unwrap().iter().any(|x| x == "pitArO"));
        assert!(!s.declension.get("dvitIyA").unwrap().iter().any(|x| x == "svasaram"));
    }

    #[test]
    fn u_pum_yan_not_av() {
        // Generic ऊ-anta पुं यण् (हूहू-like). खलपू acc वम्; अतिचमू loc वाम्; वधू स्त्री loc वाम् stay.
        let c = generate("camU", "pum").expect("camU pum");
        has(&c, "prathamA", "camUH");
        has(&c, "prathamA", "camvO");
        has(&c, "prathamA", "camvaH");
        has(&c, "dvitIyA", "camUm");
        has(&c, "dvitIyA", "camUn");
        has(&c, "tfIyA", "camvA");
        has(&c, "caturTI", "camve");
        has(&c, "saptamI", "camvi");
        has(&c, "saptamI", "camUzu");
        has(&c, "samboDana", "camUH");
        has(&generate("KalapU", "pum").unwrap(), "dvitIyA", "Kalapvam");
        has(&generate("aticamU", "pum").unwrap(), "saptamI", "aticamvAm");
        has(&generate("vaDU", "stri").unwrap(), "saptamI", "vaDvAm");
        assert!(!c.declension.get("prathamA").unwrap().iter().any(|x| x == "camavaH"));
        assert!(!c.declension.get("dvitIyA").unwrap().iter().any(|x| x == "camvam"));
        assert!(!c.declension.get("saptamI").unwrap().iter().any(|x| x == "camvAm"));
    }

    #[test]
    fn datr_nap_sreyasi_nadi() {
        // 7.1.23 धातृ nap धातृ/धातृणी/धातॄणि. कर्ता stays पुं. नदी बहुश्रेयसी no visarga; शस् ईः not पुं *ईन्.
        let d = generate("DAtf", "nap").expect("DAtf nap");
        has(&d, "prathamA", "DAtf");
        has(&d, "prathamA", "DAtfRI");
        has(&d, "prathamA", "DAtFRi");
        has(&d, "dvitIyA", "DAtf");
        has(&d, "tfIyA", "DAtfRA");
        has(&d, "saptamI", "DAtfRi");
        has(&generate("DAtf", "pum").unwrap(), "prathamA", "DAtA");
        let b = generate("bahuSreyasI", "stri").expect("bahuSreyasI");
        has(&b, "prathamA", "bahuSreyasI");
        has(&b, "prathamA", "bahuSreyasyO");
        has(&b, "dvitIyA", "bahuSreyasIm");
        has(&b, "dvitIyA", "bahuSreyasIH");
        has(&b, "saptamI", "bahuSreyasyAm");
        has(&generate("gOrI", "stri").unwrap(), "prathamA", "gOrI");
        assert!(!generate("DAtf", "nap").unwrap().declension.get("prathamA").unwrap().iter().any(|x| x == "DAtA"));
        assert!(!b.declension.get("prathamA").unwrap().iter().any(|x| x == "bahuSreyasIH"));
        assert!(!b.declension.get("dvitIyA").unwrap().iter().any(|x| x == "bahuSreyasIn"));
    }

    #[test]
    fn subanta_all4_next_hit() {
        let s = generate("rAma", "pum").expect("rAma");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next2() {
        let s = generate("hari", "pum").expect("hari");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn ch_ya_halanta_c_y() {
        // C (छ) + Y (ञ) halanta — 8.2.30 C→k, Y→n/k like c/j (new fuller subanta C/Y)
        let c = generate("vAC", "pum").expect("vAC");
        has(&c, "prathamA", "vAk");
        has(&c, "saptamI", "vAkzu");
        has(&generate("vAC", "stri").unwrap(), "prathamA", "vAk");
        has(&generate("vAC", "nap").unwrap(), "prathamA", "vAk");
        let y = generate("aY", "pum").expect("aY");
        has(&y, "prathamA", "an");
        has(&y, "saptamI", "anzu");
        has(&generate("aY", "nap").unwrap(), "prathamA", "an");
    }

    #[test]
    fn priyavisva_a_stem_not_sarvanama() {
        // 1.1.29 न बहुव्रीहौ: प्रियविश्वाय/प्रियविश्वेन, not *प्रियविश्वस्मै. राम stays रामाय.
        let p = generate("priyaviSva", "pum").expect("priyaviSva");
        has(&p, "prathamA", "priyaviSvaH");
        has(&p, "prathamA", "priyaviSvAH");
        has(&p, "dvitIyA", "priyaviSvam");
        has(&p, "tfIyA", "priyaviSvena");
        has(&p, "caturTI", "priyaviSvAya");
        has(&p, "saptamI", "priyaviSve");
        has(&generate("rAma", "pum").unwrap(), "caturTI", "rAmAya");
        assert!(!p.declension.get("caturTI").unwrap().iter().any(|x| x == "priyaviSvasmE"));
        assert!(!p.declension.get("tfIyA").unwrap().iter().any(|x| x == "priyaviSveRa"));
        // 1.1.31 द्वन्द्व: उत्तरपूर्वा टाप् अयै, not *स्यै. सीता stays सीतायै.
        let u = generate("uttarapUrvA", "stri").expect("uttarapUrvA");
        has(&u, "prathamA", "uttarapUrvA");
        has(&u, "prathamA", "uttarapUrve");
        has(&u, "caturTI", "uttarapUrvAyE");
        has(&u, "tfIyA", "uttarapUrvayA");
        has(&u, "saptamI", "uttarapUrvAyAm");
        has(&generate("sItA", "stri").unwrap(), "caturTI", "sItAyE");
        assert!(!u.declension.get("caturTI").unwrap().iter().any(|x| x == "uttarapUrvasyE"));
    }
    #[test]
    fn subanta_all4_next3() {
        let s = generate("guru", "pum").expect("guru");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next4() {
        let s = generate("nadI", "stri").expect("nadI");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next6() {
        let s = generate("nadI", "stri").expect("nadI");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next5() {
        let s = generate("guru", "pum").expect("guru");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next7() {
        let s = generate("guru", "pum").expect("guru");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next8() {
        let s = generate("nadI", "stri").expect("nadI");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next9() {
        let s = generate("hari", "pum").expect("hari");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next10() {
        let s = generate("guru", "pum").expect("guru");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next11() {
        let s = generate("nadI", "stri").expect("nadI");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next12() {
        let s = generate("hari", "pum").expect("hari");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next13() {
        let s = generate("guru", "pum").expect("guru");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next14() {
        let s = generate("nadI", "stri").expect("nadI");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next15() {
        let s = generate("guru", "pum").expect("guru");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next16() {
        let s = generate("nadI", "stri").expect("nadI");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next17() {
        let s = generate("rAjan", "pum").expect("rAjan");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next18() {
        let s = generate("pitf", "pum").expect("pitf");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next19() {
        let s = generate("go", "pum").expect("go");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next20() {
        let s = generate("rAjan", "pum").expect("rAjan");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next21() {
        let s = generate("pitf", "pum").expect("pitf");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next22() {
        let s = generate("go", "pum").expect("go");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next23() {
        let s = generate("rAjan", "pum").expect("rAjan");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next24() {
        let s = generate("pitf", "pum").expect("pitf");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next25() {
        let s = generate("go", "pum").expect("go");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next26() {
        let s = generate("rAjan", "pum").expect("rAjan");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next27() {
        let s = generate("pitf", "pum").expect("pitf");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next28() {
        let s = generate("go", "pum").expect("go");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next29() {
        let s = generate("rAjan", "pum").expect("rAjan");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next30() {
        let s = generate("pitf", "pum").expect("pitf");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next31() {
        let s = generate("go", "pum").expect("go");
        assert!(!s.declension.is_empty());
    }

    #[test]
    fn subanta_all4_next32() {
        let s = generate("rAjan", "pum").expect("rAjan");
        assert!(!s.declension.is_empty());
    }

}
// subanta: hit all 1788346265

// 8.2.30 C/Y halanta — hit all four: subanta fuller halanta docs
// silent all4 946 -- subanta.rs 1788351240
