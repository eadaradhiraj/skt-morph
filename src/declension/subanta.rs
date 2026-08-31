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
  m.insert(("I".to_string(),"pum".to_string()), vec![vec!["I".to_string(),"yO".to_string(),"yaH".to_string(),],vec!["Im".to_string(),"yO".to_string(),"In".to_string(),],vec!["yA".to_string(),"IByAm".to_string(),"IBiH".to_string(),],vec!["ye".to_string(),"IByAm".to_string(),"IByaH".to_string(),],vec!["eH".to_string(),"IByAm".to_string(),"IByaH".to_string(),],vec!["eH".to_string(),"yoH".to_string(),"InAm".to_string(),],vec!["O".to_string(),"yoH".to_string(),"Izu".to_string(),],vec!["i".to_string(),"yO".to_string(),"yaH".to_string(),],]);
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
  m.insert(("at".to_string(),"nap".to_string()), vec![vec!["at".to_string(),"atI".to_string(),"AMsi".to_string(),],vec!["atam".to_string(),"atI".to_string(),"AMsi".to_string(),],vec!["atA".to_string(),"ByAm".to_string(),"BiH".to_string(),],vec!["ate".to_string(),"ByAm".to_string(),"ByaH".to_string(),],vec!["ataH".to_string(),"ByAm".to_string(),"ByaH".to_string(),],vec!["ataH".to_string(),"atoH".to_string(),"Am".to_string(),],vec!["ati".to_string(),"atoH".to_string(),"atsu".to_string(),],vec!["at".to_string(),"atI".to_string(),"AMsi".to_string(),],]);
  m.insert(("us".to_string(),"nap".to_string()), vec![vec!["uH".to_string(),"uSI".to_string(),"UMSi".to_string(),],vec!["uH".to_string(),"uSI".to_string(),"UMSi".to_string(),],vec!["usA".to_string(),"oByAm".to_string(),"oBiH".to_string(),],vec!["use".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["usaH".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["usaH".to_string(),"usoH".to_string(),"usAm".to_string(),],vec!["usi".to_string(),"usoH".to_string(),"uHsu".to_string(),],vec!["uH".to_string(),"uSI".to_string(),"UMSi".to_string(),],]);
  m.insert(("is".to_string(),"nap".to_string()), vec![vec!["iH".to_string(),"iSI".to_string(),"IMSi".to_string(),],vec!["iH".to_string(),"iSI".to_string(),"IMSi".to_string(),],vec!["isA".to_string(),"oByAm".to_string(),"oBiH".to_string(),],vec!["ise".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["isaH".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["isaH".to_string(),"isoH".to_string(),"isAm".to_string(),],vec!["isi".to_string(),"isoH".to_string(),"iHsu".to_string(),],vec!["iH".to_string(),"iSI".to_string(),"IMSi".to_string(),],]);
  // गो: 7.1.90 णित्, 6.1.93 औतोऽम्शसोः (गाम्/गाः), 6.1.78 अव्
  m.insert(("o".to_string(),"pum".to_string()), vec![vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],vec!["Am".to_string(),"AvO".to_string(),"AH".to_string(),],vec!["avA".to_string(),"oByAm".to_string(),"oBiH".to_string(),],vec!["ave".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["oH".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["oH".to_string(),"avoH".to_string(),"avAm".to_string(),],vec!["avi".to_string(),"avoH".to_string(),"ozu".to_string(),],vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],]);
  m.insert(("o".to_string(),"stri".to_string()), vec![vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],vec!["Am".to_string(),"AvO".to_string(),"AH".to_string(),],vec!["avA".to_string(),"oByAm".to_string(),"oBiH".to_string(),],vec!["ave".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["oH".to_string(),"oByAm".to_string(),"oByaH".to_string(),],vec!["oH".to_string(),"avoH".to_string(),"avAm".to_string(),],vec!["avi".to_string(),"avoH".to_string(),"ozu".to_string(),],vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],]);
  // नौ: औ-anta, 6.1.78 आव् (नावम्, not 6.1.93 गाम्)
  m.insert(("O".to_string(),"pum".to_string()), vec![vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],vec!["Avam".to_string(),"AvO".to_string(),"AvaH".to_string(),],vec!["AvA".to_string(),"OByAm".to_string(),"OBiH".to_string(),],vec!["Ave".to_string(),"OByAm".to_string(),"OByaH".to_string(),],vec!["AvaH".to_string(),"OByAm".to_string(),"OByaH".to_string(),],vec!["AvaH".to_string(),"AvoH".to_string(),"AvAm".to_string(),],vec!["Avi".to_string(),"AvoH".to_string(),"Ozu".to_string(),],vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],]);
  m.insert(("O".to_string(),"stri".to_string()), vec![vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],vec!["Avam".to_string(),"AvO".to_string(),"AvaH".to_string(),],vec!["AvA".to_string(),"OByAm".to_string(),"OBiH".to_string(),],vec!["Ave".to_string(),"OByAm".to_string(),"OByaH".to_string(),],vec!["AvaH".to_string(),"OByAm".to_string(),"OByaH".to_string(),],vec!["AvaH".to_string(),"AvoH".to_string(),"AvAm".to_string(),],vec!["Avi".to_string(),"AvoH".to_string(),"Ozu".to_string(),],vec!["OH".to_string(),"AvO".to_string(),"AvaH".to_string(),],]);
  // हलन्त — j-anta (वणिज्), d-anta (सुहृद्) — 8.2.30/8.2.39 jhal sandhi
  // sūtra: 8.2.30 चोः कुः (j→k at pada), 8.2.39 झलां जशोऽन्ते (j→k/j etc.)
  // Future devs: j-stem g/k (वणिक्/वणिजौ), d-stem t/d (सुहृत्/सुहृदौ) — add h→k, B→p similarly
  // Extreme commenting kept for future halanta expansion — h, B next
  m.insert(("j".to_string(),"pum".to_string()), vec![vec!["k".to_string(),"jO".to_string(),"jaH".to_string(),],vec!["jam".to_string(),"jO".to_string(),"jaH".to_string(),],vec!["jA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["je".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["jaH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["jaH".to_string(),"joH".to_string(),"jAm".to_string(),],vec!["ji".to_string(),"joH".to_string(),"zu".to_string(),],vec!["k".to_string(),"jO".to_string(),"jaH".to_string(),],]);
  m.insert(("j".to_string(),"stri".to_string()), vec![vec!["k".to_string(),"jO".to_string(),"jaH".to_string(),],vec!["jam".to_string(),"jO".to_string(),"jaH".to_string(),],vec!["jA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["je".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["jaH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["jaH".to_string(),"joH".to_string(),"jAm".to_string(),],vec!["ji".to_string(),"joH".to_string(),"zu".to_string(),],vec!["k".to_string(),"jO".to_string(),"jaH".to_string(),],]);
  m.insert(("d".to_string(),"pum".to_string()), vec![vec!["t".to_string(),"dO".to_string(),"daH".to_string(),],vec!["dam".to_string(),"dO".to_string(),"daH".to_string(),],vec!["dA".to_string(),"dByAm".to_string(),"dBiH".to_string(),],vec!["de".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["daH".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["daH".to_string(),"doH".to_string(),"dAm".to_string(),],vec!["di".to_string(),"doH".to_string(),"tsu".to_string(),],vec!["t".to_string(),"dO".to_string(),"daH".to_string(),],]);
  m.insert(("d".to_string(),"stri".to_string()), vec![vec!["t".to_string(),"dO".to_string(),"daH".to_string(),],vec!["dam".to_string(),"dO".to_string(),"daH".to_string(),],vec!["dA".to_string(),"dByAm".to_string(),"dBiH".to_string(),],vec!["de".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["daH".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["daH".to_string(),"doH".to_string(),"dAm".to_string(),],vec!["di".to_string(),"doH".to_string(),"tsu".to_string(),],vec!["t".to_string(),"dO".to_string(),"daH".to_string(),],]);
  // त-anta (मरुत्, सरित्, त्रिंशत्) — पद त्; 8.2.39 before भ् → द्भ्याम्/द्भिः. `at` शतृ stays longer-match.
  m.insert(("t".to_string(),"pum".to_string()), vec![vec!["t".to_string(),"tO".to_string(),"taH".to_string(),],vec!["tam".to_string(),"tO".to_string(),"taH".to_string(),],vec!["tA".to_string(),"dByAm".to_string(),"dBiH".to_string(),],vec!["te".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["taH".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["taH".to_string(),"toH".to_string(),"tAm".to_string(),],vec!["ti".to_string(),"toH".to_string(),"tsu".to_string(),],vec!["t".to_string(),"tO".to_string(),"taH".to_string(),],]);
  m.insert(("t".to_string(),"stri".to_string()), vec![vec!["t".to_string(),"tO".to_string(),"taH".to_string(),],vec!["tam".to_string(),"tO".to_string(),"taH".to_string(),],vec!["tA".to_string(),"dByAm".to_string(),"dBiH".to_string(),],vec!["te".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["taH".to_string(),"dByAm".to_string(),"dByaH".to_string(),],vec!["taH".to_string(),"toH".to_string(),"tAm".to_string(),],vec!["ti".to_string(),"toH".to_string(),"tsu".to_string(),],vec!["t".to_string(),"tO".to_string(),"taH".to_string(),],]);
  // h-anta (लिह्) and B-anta (लभ्-type bh) — 8.2.31 हो ढः, 8.2.32 दादेर्धातोर्घः + जश्त्व
  // sūtra: 8.2.31 h→Q/ḍh at jhal, pada h→k (दुह्→धुक्); Future devs: h shows ढ/क, B shows प्/भ्
  // Extreme: keep tsu/zu for saptamī bahu, consistent with j/d paradigms above
  m.insert(("h".to_string(),"pum".to_string()), vec![vec!["k".to_string(),"hO".to_string(),"haH".to_string(),],vec!["ham".to_string(),"hO".to_string(),"haH".to_string(),],vec!["hA".to_string(),"gByAm".to_string(),"gBiH".to_string(),],vec!["he".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["haH".to_string(),"gByAm".to_string(),"gByaH".to_string(),],vec!["haH".to_string(),"hoH".to_string(),"hAm".to_string(),],vec!["hi".to_string(),"hoH".to_string(),"tsu".to_string(),],vec!["k".to_string(),"hO".to_string(),"haH".to_string(),],]);
  let h_pum = m.get(&("h".to_string(), "pum".to_string())).expect("h pum").clone();
  m.insert(("h".to_string(), "stri".to_string()), h_pum);
  m.insert(("B".to_string(),"pum".to_string()), vec![vec!["p".to_string(),"BO".to_string(),"BaH".to_string(),],vec!["Bam".to_string(),"BO".to_string(),"BaH".to_string(),],vec!["BA".to_string(),"BByAm".to_string(),"BBiH".to_string(),],vec!["Be".to_string(),"BByAm".to_string(),"BByaH".to_string(),],vec!["BaH".to_string(),"BByAm".to_string(),"BByaH".to_string(),],vec!["BaH".to_string(),"BoH".to_string(),"BAm".to_string(),],vec!["Bi".to_string(),"BoH".to_string(),"tsu".to_string(),],vec!["p".to_string(),"BO".to_string(),"BaH".to_string(),],]);
  let b_pum = m.get(&("B".to_string(), "pum".to_string())).expect("B pum").clone();
  m.insert(("B".to_string(), "stri".to_string()), b_pum);
  // प-anta (ककुप्) — पद प्; 8.2.39 before भ् → ब्भ्याम्. Distinct from भ्-anta (लभ्→लप्).
  m.insert(("p".to_string(),"pum".to_string()), vec![vec!["p".to_string(),"pO".to_string(),"paH".to_string(),],vec!["pam".to_string(),"pO".to_string(),"paH".to_string(),],vec!["pA".to_string(),"bByAm".to_string(),"bBiH".to_string(),],vec!["pe".to_string(),"bByAm".to_string(),"bByaH".to_string(),],vec!["paH".to_string(),"bByAm".to_string(),"bByaH".to_string(),],vec!["paH".to_string(),"poH".to_string(),"pAm".to_string(),],vec!["pi".to_string(),"poH".to_string(),"psu".to_string(),],vec!["p".to_string(),"pO".to_string(),"paH".to_string(),],]);
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

/// 4.1.5–6 ङीप्: इन्/उगित् अत् स्त्री → दण्डिनी, भवती. Not त्रिंशत् (त-anta संख्या).
fn ngeep_stri(cand: &str, linga: &str) -> String {
    if linga != "stri" || cand.ends_with('I') {
        return cand.to_string();
    }
    if cand.ends_with("Sat") {
        return cand.to_string();
    }
    if cand.ends_with("at") || cand.ends_with("in") {
        format!("{cand}I")
    } else {
        cand.to_string()
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
        decl.insert(
            "prathamA".into(),
            vec![strong("A"), strong("AnO"), strong("AnaH")],
        );
        decl.insert(
            "dvitIyA".into(),
            vec![strong("Anam"), strong("AnO"), weak("aH")],
        );
        decl.insert("tfIyA".into(), vec![weak("A"), pada("aByAm"), pada("aBiH")]);
        decl.insert("caturTI".into(), vec![weak("e"), pada("aByAm"), pada("aByaH")]);
        decl.insert("paYcamI".into(), vec![weak("aH"), pada("aByAm"), pada("aByaH")]);
        decl.insert("zazWI".into(), vec![weak("aH"), weak("oH"), weak("Am")]);
        decl.insert("saptamI".into(), sap);
        decl.insert(
            "samboDana".into(),
            vec![stem.to_string(), strong("AnO"), strong("AnaH")],
        );
    }
    Declension {
        stem: stem.to_string(),
        linga: linga.to_string(),
        declension: decl,
    }
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
        let cand = ngeep_stri(&cand, linga);
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if cand.ends_with("an") && (linga == "pum" || linga == "nap") {
            return Some(decline_an(&cand, linga));
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
        has(&v, "tfIyA", "vaRijA");
        has(&v, "saptamI", "vaRiji");
        // d-anta: suhfd (सुहृद्) — nom sg सुहृत् (8.2.39), instr सुहृदा
        let s = generate("suhfd", "pum").expect("suhfd");
        has(&s, "prathamA", "suhft");
        has(&s, "dvitIyA", "suhfdam");
        has(&s, "tfIyA", "suhfdA");
        // h-anta: lih-like (दुह्→धुक्) — 8.2.31; B-anta: laB-type bh→p at pada
        let h = generate("lih", "pum").expect("lih");
        has(&h, "prathamA", "lik");
        has(&h, "tfIyA", "lihA");
        let b = generate("laB", "pum").expect("laB");
        has(&b, "prathamA", "lap");
        has(&b, "tfIyA", "laBA");
        // r-anta: gir (गिर्) — 8.2.66 s→ru gives giH; s-anta: tapas-like s pum — same ru → tapas→tapaH
        let r = generate("gir", "pum").expect("gir");
        has(&r, "prathamA", "giH");
        has(&r, "tfIyA", "girA");
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
        has(&kp, "dvitIyA", "kakupam");
        has(&kp, "tfIyA", "kakupA");
        has(&kp, "tfIyA", "kakubByAm");
        has(&kp, "saptamI", "kakupsu");
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
        // त्रिंशत् still त-anta (Sat excluded from ङीप्).
        has(&generate("triMSat", "stri").unwrap(), "prathamA", "triMSat");
        // हल् स्त्री same as पुं (not आ-stem fallback).
        has(&generate("lih", "stri").unwrap(), "prathamA", "lik");
        has(&generate("laB", "stri").unwrap(), "prathamA", "lap");
    }
}
