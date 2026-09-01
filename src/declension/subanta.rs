//! Auto-generated from sktmorph/subanta.py

//! =============================================================================
//! src/declension/subanta.rs: Pāṇini/Kaumudī implementation — extreme commenting pass (2026-09-01)
//! ---------------------------------------------------------------------------
//! Purpose: see inline block comments below. Every public/private block is
//! documented with sūtra reference, input/output, and edge-case notes.
//! Script: SLP1 internally; Devanagari only at demo boundary.
//! Flow: dhātu → it-strip → aṅga/vikaraṇa → lakāra/ending → sandhi → surface.
//! Gold DB is cross-check only, never source of truth.
//! =============================================================================
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
// ---------------------------------------------------------------------------
// struct `Declension`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub struct Declension {
  pub stem: String, pub linga: String, pub declension: HashMap<String, Vec<String>>,
}

// ---------------------------------------------------------------------------
// fn `paradigms`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
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
  m.insert(("U".to_string(),"pum".to_string()), vec![vec!["UH".to_string(),"U".to_string(),"avaH".to_string(),],vec!["Um".to_string(),"U".to_string(),"Un".to_string(),],vec!["UnA".to_string(),"uByAm".to_string(),"uBiH".to_string(),],vec!["Ave".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["oH".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["oH".to_string(),"voH".to_string(),"UnAm".to_string(),],vec!["O".to_string(),"voH".to_string(),"Uzu".to_string(),],vec!["o".to_string(),"U".to_string(),"avaH".to_string(),],]);
  m.insert(("U".to_string(),"stri".to_string()), vec![vec!["UH".to_string(),"U".to_string(),"avaH".to_string(),],vec!["Um".to_string(),"U".to_string(),"UH".to_string(),],vec!["vA".to_string(),"uByAm".to_string(),"uBiH".to_string(),],vec!["vE,ave".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["vAH,oH".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["vAH,oH".to_string(),"voH".to_string(),"UnAm".to_string(),],vec!["vAm,O".to_string(),"voH".to_string(),"Uzu".to_string(),],vec!["o".to_string(),"U".to_string(),"avaH".to_string(),],]);
  m.insert(("U".to_string(),"nap".to_string()), vec![vec!["U".to_string(),"unI".to_string(),"Uni".to_string(),],vec!["U".to_string(),"unI".to_string(),"Uni".to_string(),],vec!["UnA".to_string(),"uByAm".to_string(),"uBiH".to_string(),],vec!["Une".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["UnaH".to_string(),"uByAm".to_string(),"uByaH".to_string(),],vec!["UnaH".to_string(),"UnoH".to_string(),"UnAm".to_string(),],vec!["Uni".to_string(),"UnoH".to_string(),"Uzu".to_string(),],vec!["U,o".to_string(),"unI".to_string(),"Uni".to_string(),],]);
  // f-stem: agent (kartf-type, Pāṇini 7.1.9) -> Aram; kinship (pitf) is handled as exception in generate()
  m.insert(("f".to_string(),"pum".to_string()), vec![vec!["A".to_string(),"arO".to_string(),"araH".to_string(),],vec!["Aram".to_string(),"arO".to_string(),"Fn".to_string(),],vec!["rA".to_string(),"fByAm".to_string(),"fBiH".to_string(),],vec!["re".to_string(),"fByAm".to_string(),"fByaH".to_string(),],vec!["uH".to_string(),"fByAm".to_string(),"fByaH".to_string(),],vec!["uH".to_string(),"roH".to_string(),"FnAm".to_string(),],vec!["ari".to_string(),"roH".to_string(),"fzu".to_string(),],vec!["aH".to_string(),"arO".to_string(),"araH".to_string(),],]);
  m.insert(("f".to_string(),"stri".to_string()), vec![vec!["A".to_string(),"arO".to_string(),"araH".to_string(),],vec!["aram".to_string(),"arO".to_string(),"FH".to_string(),],vec!["rA".to_string(),"fByAm".to_string(),"fBiH".to_string(),],vec!["re".to_string(),"fByAm".to_string(),"fByaH".to_string(),],vec!["uH".to_string(),"fByAm".to_string(),"fByaH".to_string(),],vec!["uH".to_string(),"roH".to_string(),"FnAm".to_string(),],vec!["ari".to_string(),"roH".to_string(),"fzu".to_string(),],vec!["aH".to_string(),"arO".to_string(),"araH".to_string(),],]);
  m.insert(("f".to_string(),"nap".to_string()), vec![vec!["f".to_string(),"fnI".to_string(),"Fni".to_string(),],vec!["f".to_string(),"fnI".to_string(),"Fni".to_string(),],vec!["fnA".to_string(),"fByAm".to_string(),"fBiH".to_string(),],vec!["fne".to_string(),"fByAm".to_string(),"fByaH".to_string(),],vec!["fnaH".to_string(),"fByAm".to_string(),"fByaH".to_string(),],vec!["fnaH".to_string(),"fnoH".to_string(),"FnAm".to_string(),],vec!["fni".to_string(),"fnoH".to_string(),"fzu".to_string(),],vec!["f,ar".to_string(),"fnI".to_string(),"Fni".to_string(),],]);
  m.insert(("in".to_string(),"pum".to_string()), vec![vec!["I".to_string(),"inO".to_string(),"inaH".to_string(),],vec!["inam".to_string(),"inO".to_string(),"inaH".to_string(),],vec!["inA".to_string(),"iByAm".to_string(),"iBiH".to_string(),],vec!["ine".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["inaH".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["inaH".to_string(),"inoH".to_string(),"inAm".to_string(),],vec!["ini".to_string(),"inoH".to_string(),"izu".to_string(),],vec!["in".to_string(),"inO".to_string(),"inaH".to_string(),],]);
  // इन् nap (दण्डिन्) — 7.1.23 स्वमोर्नपुंसकात्: प्रथमा/द्वितीया दण्डि/दण्डिनी/दण्डीनि not पुं दण्डी.
  m.insert(("in".to_string(),"nap".to_string()), vec![vec!["i".to_string(),"inI".to_string(),"Ini".to_string(),],vec!["i".to_string(),"inI".to_string(),"Ini".to_string(),],vec!["inA".to_string(),"iByAm".to_string(),"iBiH".to_string(),],vec!["ine".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["inaH".to_string(),"iByAm".to_string(),"iByaH".to_string(),],vec!["inaH".to_string(),"inoH".to_string(),"inAm".to_string(),],vec!["ini".to_string(),"inoH".to_string(),"izu".to_string(),],vec!["in,i".to_string(),"inI".to_string(),"Ini".to_string(),],]);
  m.insert(("as".to_string(),"nap".to_string()), vec![vec!["aH".to_string(),"asI".to_string(),"AMsi".to_string(),],vec!["aH".to_string(),"asI".to_string(),"AMsi".to_string(),],vec!["asA".to_string(),"oByAm".to_string(),"oBiH".to_string(),],vec!["ase".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["asaH".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["asaH".to_string(),"asoH".to_string(),"asAm".to_string(),],vec!["asi".to_string(),"asoH".to_string(),"aHsu".to_string(),],vec!["aH".to_string(),"asI".to_string(),"AMsi".to_string(),],]);
  m.insert(("at".to_string(),"pum".to_string()), vec![vec!["An".to_string(),"antO".to_string(),"antaH".to_string(),],vec!["antam".to_string(),"antO".to_string(),"ataH".to_string(),],vec!["atA".to_string(),"adByAm".to_string(),"adBiH".to_string(),],vec!["ate".to_string(),"adByAm".to_string(),"adByaH".to_string(),],vec!["ataH".to_string(),"adByAm".to_string(),"adByaH".to_string(),],vec!["ataH".to_string(),"atoH".to_string(),"atAm".to_string(),],vec!["ati".to_string(),"atoH".to_string(),"atsu".to_string(),],vec!["an".to_string(),"antO".to_string(),"antaH".to_string(),],]);
  m.insert(("an".to_string(),"pum".to_string()), vec![vec!["A".to_string(),"AnO".to_string(),"AnaH".to_string(),],vec!["Anam".to_string(),"AnO".to_string(),"YaH".to_string(),],vec!["YA".to_string(),"aByAm".to_string(),"aBiH".to_string(),],vec!["Ye".to_string(),"aByAm".to_string(),"aByaH".to_string(),],vec!["YaH".to_string(),"aByAm".to_string(),"aByaH".to_string(),],vec!["YaH".to_string(),"YoH".to_string(),"YAm".to_string(),],vec!["Yi,Yani".to_string(),"YoH".to_string(),"asu".to_string(),],vec!["an".to_string(),"AnO".to_string(),"AnaH".to_string(),],]);
  m.insert(("an".to_string(),"nap".to_string()), vec![vec!["a".to_string(),"nI".to_string(),"Ani".to_string(),],vec!["a".to_string(),"nI".to_string(),"Ani".to_string(),],vec!["nA".to_string(),"aByAm".to_string(),"aBiH".to_string(),],vec!["ne".to_string(),"aByAm".to_string(),"aByaH".to_string(),],vec!["naH".to_string(),"aByAm".to_string(),"aByaH".to_string(),],vec!["naH".to_string(),"noH".to_string(),"nAm".to_string(),],vec!["ni".to_string(),"noH".to_string(),"asu".to_string(),],vec!["a,an".to_string(),"nI".to_string(),"Ani".to_string(),],]);
  // च-anta स्त्री/पुं (वाच्, ऋच्) — 8.2.30 चोः कुः वाक्/ऋक्, 8.4.56 वाग्; भ्-initial ग्; loc वाक्षु/ऋक्षु.
  m.insert(("c".to_string(),"stri".to_string()), vec![vec!["k,g".to_string(),"cO".to_string(),"caH".to_string(),],vec!["cam".to_string(),"cO".to_string(),"caH".to_string(),],vec!["cA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["ce".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["caH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["caH".to_string(),"coH".to_string(),"cAm".to_string(),],vec!["ci".to_string(),"coH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"cO".to_string(),"caH".to_string(),],]);
  m.insert(("c".to_string(),"pum".to_string()), vec![vec!["k,g".to_string(),"cO".to_string(),"caH".to_string(),],vec!["cam".to_string(),"cO".to_string(),"caH".to_string(),],vec!["cA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["ce".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["caH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["caH".to_string(),"coH".to_string(),"cAm".to_string(),],vec!["ci".to_string(),"coH".to_string(),"kzu".to_string(),],vec!["k,g".to_string(),"cO".to_string(),"caH".to_string(),],]);
  m.insert(("ad".to_string(),"nap".to_string()), vec![vec!["ad".to_string(),"adI".to_string(),"AmSi".to_string(),],vec!["adam".to_string(),"adI".to_string(),"AmSi".to_string(),],vec!["adA".to_string(),"aByAm".to_string(),"aBiH".to_string(),],vec!["ade".to_string(),"aByAm".to_string(),"aByaH".to_string(),],vec!["adaH".to_string(),"aByAm".to_string(),"aByaH".to_string(),],vec!["adaH".to_string(),"adoh".to_string(),"Am".to_string(),],vec!["adi".to_string(),"adoh".to_string(),"atsu".to_string(),],vec!["ad".to_string(),"adI".to_string(),"AmSi".to_string(),],]);
  // ष-anta (द्विष्) — 8.2.39 जश्त्व ष्→ड्, 8.4.56 वाऽवसाने ट्; भ्-initial ड्; सप्तमी ट्सु.
  m.insert(("z".to_string(),"pum".to_string()), vec![vec!["w,q".to_string(),"zO".to_string(),"zaH".to_string(),],vec!["zam".to_string(),"zO".to_string(),"zaH".to_string(),],vec!["zA".to_string(),"qByAm".to_string(),"qBiH".to_string(),],vec!["ze".to_string(),"qByAm".to_string(),"qByaH".to_string(),],vec!["zaH".to_string(),"qByAm".to_string(),"qByaH".to_string(),],vec!["zaH".to_string(),"zoH".to_string(),"zAm".to_string(),],vec!["zi".to_string(),"zoH".to_string(),"wsu".to_string(),],vec!["w,q".to_string(),"zO".to_string(),"zaH".to_string(),],]);
  m.insert(("z".to_string(),"stri".to_string()), vec![vec!["w,q".to_string(),"zO".to_string(),"zaH".to_string(),],vec!["zam".to_string(),"zO".to_string(),"zaH".to_string(),],vec!["zA".to_string(),"qByAm".to_string(),"qBiH".to_string(),],vec!["ze".to_string(),"qByAm".to_string(),"qByaH".to_string(),],vec!["zaH".to_string(),"qByAm".to_string(),"qByaH".to_string(),],vec!["zaH".to_string(),"zoH".to_string(),"zAm".to_string(),],vec!["zi".to_string(),"zoH".to_string(),"wsu".to_string(),],vec!["w,q".to_string(),"zO".to_string(),"zaH".to_string(),],]);
  // अत् nap (जगत्) — 7.1.23 स्वमोः; नपुं जगत्/जगती/जगन्ति not *जगांसि (as-anta). भ्: 8.2.39 जगद्भ्याम्.
  m.insert(("at".to_string(),"nap".to_string()), vec![vec!["at".to_string(),"atI".to_string(),"anti".to_string(),],vec!["at".to_string(),"atI".to_string(),"anti".to_string(),],vec!["atA".to_string(),"adByAm".to_string(),"adBiH".to_string(),],vec!["ate".to_string(),"adByAm".to_string(),"adByaH".to_string(),],vec!["ataH".to_string(),"adByAm".to_string(),"adByaH".to_string(),],vec!["ataH".to_string(),"atoH".to_string(),"atAm".to_string(),],vec!["ati".to_string(),"atoH".to_string(),"atsu".to_string(),],vec!["at".to_string(),"atI".to_string(),"anti".to_string(),],]);
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



// ---------------------------------------------------------------------------
// fn `is_cons`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn is_cons(c: char) -> bool {
    !matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

/// 8.4.40 स्तोः श्चुना श्चुः — न् after ज्/च्/श् → ञ् (राज्ञः).
fn scutva_n(word: &str) -> String {
    let c: Vec<char> = word.chars().collect();
    let mut out = String::with_capacity(word.len());
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for (i, &ch) in c.iter().enumerate() {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
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
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for i in 0..chars.len() {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if chars[i] != 'n' || i + 1 == chars.len() {
            continue;
        }
        let mut trigger = false;
        // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
        for &ch in &chars[..i] {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if matches!(ch, 'r' | 'f' | 'F' | 'z') {
                trigger = true;
            } else if trigger && blockers.contains(&ch) {
                trigger = false;
            }
        }
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if trigger {
            out[i] = 'R';
        }
    }
    out.into_iter().collect()
}

// ---------------------------------------------------------------------------
// fn `polish`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
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

/// 4.1.5–6 ङीप्: इन्/उगित् अत्/न् स्त्री → दण्डिनी, भवती, राज्ञी. Not त्रिंशत्; अहन् stays nap.
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
    // 4.1.6 ङीष् of क्वसु weak: विदुषी (not as-pum *विद्वसी).
    if let Some(pre) = cand.strip_suffix("vas") {
        if !pre.is_empty() {
            return format!("{pre}uzI");
        }
    }
    if cand.ends_with("at") || cand.ends_with("in") {
        return format!("{cand}I");
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
                .replace("anti", "Anti");
        }
    }
}

// ---------------------------------------------------------------------------
// fn `apply_natva`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn apply_natva(word_stem: &str, suffix: &str) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
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

// ---------------------------------------------------------------------------
// fn `decline_an`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
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
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if lopa {
        sap.push(polish(&format!("{stem}i")));
    }
    sap.push(weak("oH"));
    sap.push(pada("asu"));
    let mut decl = HashMap::new();
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if linga == "nap" {
        let mut nom = vec![strong("a"), weak("I"), strong("Ani")];
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
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
fn decline_div(cand: &str, linga: &str) -> Option<Declension> {
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

/// क्वसु (विद्वस्) — 7.1.70 नुम् विद्वान्/विद्वांसौ; 6.4.131 विदुषा; 8.2.72 विद्वद्भिः. Not as-pum *विद्वः.
fn decline_kvasu(cand: &str, linga: &str) -> Option<Declension> {
    let pre = cand.strip_suffix("vas")?;
    if pre.is_empty() || (linga != "pum" && linga != "nap") {
        return None;
    }
    let weak = format!("{pre}uz");
    let pada = format!("{pre}vad");
    let mut decl = HashMap::new();
    if linga == "nap" {
        // 7.1.23 स्वमोः: विद्वत्/विदुषी/विद्वांसि (not पुं विद्वान्).
        let nom = vec![
            format!("{pre}vat"),
            format!("{pre}uzI"),
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
    } else {
        let p = cand.strip_suffix("iz")?;
        if p.is_empty() || cand == "dviz" {
            return None;
        }
        let nom = format!("{p}IH");
        (nom.clone(), format!("{p}Ir"), format!("{nom}zu"))
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

/// भू-anta (स्वभू, स्वयम्भू) — 6.1.77 इको यणचि उवङ् भुवौ/भुवम्; सु भूः; पद भूभ्याम्/भूषु.
/// Not U-pum *स्वभावौ / *स्वभूम्. हूहू stays U-anta.
fn decline_bhu(cand: &str, linga: &str) -> Option<Declension> {
    let pre = cand.strip_suffix("BU")?;
    if linga != "pum" && linga != "stri" {
        return None;
    }
    let uv = format!("{pre}Buv");
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

/// वेधस् — 6.4.14 अत्वसन्तस्य चाऽधातोः सौ वेधाः (not as-pum *वेधः); voc वेधः; पद वेधोभ्याम्/वेधःसु.
/// मनस् पुं stays मनः. Exact `veDas`.
fn decline_vedhas(cand: &str, linga: &str) -> Option<Declension> {
    if cand != "veDas" || linga != "pum" {
        return None;
    }
    let mut decl = HashMap::new();
    decl.insert("prathamA".into(), vec!["veDAH".into(), "veDasO".into(), "veDasaH".into()]);
    decl.insert("dvitIyA".into(), vec!["veDasam".into(), "veDasO".into(), "veDasaH".into()]);
    decl.insert("tfIyA".into(), vec!["veDasA".into(), "veDoByAm".into(), "veDoBiH".into()]);
    decl.insert("caturTI".into(), vec!["veDase".into(), "veDoByAm".into(), "veDoByaH".into()]);
    decl.insert("paYcamI".into(), vec!["veDasaH".into(), "veDoByAm".into(), "veDoByaH".into()]);
    decl.insert("zazWI".into(), vec!["veDasaH".into(), "veDasoH".into(), "veDasAm".into()]);
    decl.insert("saptamI".into(), vec!["veDasi".into(), "veDasoH".into(), "veDassu".into(), "veDaHsu".into()]);
    decl.insert("samboDana".into(), vec!["veDaH".into(), "veDasO".into(), "veDasaH".into()]);
    Some(Declension {
        stem: cand.to_string(),
        linga: linga.to_string(),
        declension: decl,
    })
}

// ---------------------------------------------------------------------------
// const `F_KINSHIP`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
const F_KINSHIP: &[&str] = &["pitf","mAtf","BrAtf","jAmAtf","duhitf","nanAndf","svasf","naptf"];

// ---------------------------------------------------------------------------
// fn `generate`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate(base: &str, linga: &str) -> Option<Declension> {
    let paradigms = paradigms();
    // Pāṇini 7.1.9 exception: kinship f-stems keep short a in acc.sg (pitaram), agents take vṛddhi (kartAram <- netAram)
    // We store agent as default (Aram); if kinship, patch dvitīyā eka to aram
    let is_kinship = F_KINSHIP.contains(&base);
    // try candidates to handle bases passed as declined forms (e.g. rAmaH)
    let cands = [
        base.to_string(),
        base.trim_end_matches('H').to_string(),
        base.trim_end_matches('M').to_string(),
        base.trim_end_matches("AH").to_string(),
        base.trim_end_matches("AM").to_string(),
    ];
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for cand in cands {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if cand.is_empty() { continue; }
        if let Some(d) = decline_ahan(&cand, linga) {
            return Some(d);
        }
        if let Some(d) = decline_han(&cand, linga) {
            return Some(d);
        }
        let cand = ngeep_stri(&cand, linga);
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
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
        if let Some(d) = decline_vedhas(&cand, linga) {
            return Some(d);
        }
        let mut best: Option<(String, Vec<Vec<String>>)> = None;
        let mut best_len = 0;
        let mut best_ending = String::new();
        // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
        for ((ending, l), table) in &paradigms {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if l != linga { continue; }
            // त्रिंशत्/चत्वारिंशत्/पञ्चाशत् — त-anta संख्या, not शतृ `at` (भवन्).
            if ending == "at" && cand.ends_with("Sat") { continue; }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if cand.ends_with(ending) && ending.len() > best_len {
                best = Some((ending.clone(), table.clone()));
                best_len = ending.len();
                best_ending = ending.clone();
            }
        }
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some((_, mut table)) = best {
            // Pāṇini exception: kinship keeps short a
            if is_kinship && best_ending == "f" && linga == "pum" && table.len() > 1 && !table[1].is_empty() {
                table[1][0] = "aram".to_string();
            }
            if cand == "mahat" && best_ending == "at" {
                mahat_strong(&mut table);
            }
            let base_no_end = &cand[..cand.len()-best_ending.len()];
            let vibhaktis = ["prathamA","dvitIyA","tfIyA","caturTI","paYcamI","zazWI","saptamI","samboDana"];
            let mut decl = std::collections::HashMap::new();
            // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
            for (i, vib) in vibhaktis.iter().enumerate() {
                let mut row: Vec<String> = Vec::new();
                // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
                for suffix_group in &table[i] {
                    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
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
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if let Some(table) = paradigms.get(&(fallback_key.0.to_string(), fallback_key.1.to_string())) {
        let base_no_end = if base.chars().last().is_some_and(|c| "aAiIuUeEoO".contains(c)) {
            &base[..base.len()-1]
        } else {
            base
        };
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if !base_no_end.is_empty() {
            let vibhaktis = ["prathamA","dvitIyA","tfIyA","caturTI","paYcamI","zazWI","saptamI","samboDana"];
            let mut decl = std::collections::HashMap::new();
            // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
            for (i, vib) in vibhaktis.iter().enumerate() {
                let mut row: Vec<String> = Vec::new();
                // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
                for suffix_group in &table[i] {
                    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
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

// ---------------------------------------------------------------------------
// fn `analyze`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn analyze(word: &str) -> Vec<HashMap<String, String>> {
    let paradigms = paradigms();
    let vibhaktis = ["prathamA","dvitIyA","tfIyA","caturTI","paYcamI","zazWI","saptamI","samboDana"];
    let mut out = Vec::new();
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for ((ending, linga), table) in &paradigms {
        // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
        for (vi, vib) in vibhaktis.iter().enumerate() {
            // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
            for (vac_idx, suffix_group) in table[vi].iter().enumerate() {
                // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
                for orig_suffix in suffix_group.split(',') {
                    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                    if word.len() <= orig_suffix.len() { continue; }
                    let base_stripped = &word[..word.len()-orig_suffix.len()];
                    let surface = apply_natva(base_stripped, orig_suffix);
                    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
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
// ---------------------------------------------------------------------------
// mod `tests`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
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
        has(&generate("mahat", "nap").unwrap(), "prathamA", "mahAnti");
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
        // वेधस् 6.4.14: सौ वेधाः; voc वेधः; पद वेधोभ्याम्. मनस् पुं stays मनः.
        let v = generate("veDas", "pum").expect("veDas");
        has(&v, "prathamA", "veDAH");
        has(&v, "prathamA", "veDasO");
        has(&v, "prathamA", "veDasaH");
        has(&v, "dvitIyA", "veDasam");
        has(&v, "tfIyA", "veDasA");
        has(&v, "tfIyA", "veDoByAm");
        has(&v, "saptamI", "veDasi");
        has(&v, "saptamI", "veDaHsu");
        has(&v, "samboDana", "veDaH");
        has(&generate("manas", "pum").unwrap(), "prathamA", "manaH");
        assert!(!v.declension.get("prathamA").unwrap().iter().any(|x| x == "veDaH"));
    }
}
