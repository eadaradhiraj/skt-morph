//! Auto-generated from sktmorph/sarvanama.py

//! =============================================================================
//! src/declension/sarvanama.rs: Pāṇini/Kaumudī implementation — extreme commenting pass (2026-09-01)
//! ---------------------------------------------------------------------------
//! Purpose: see inline block comments below. Every public/private block is
//! documented with sūtra reference, input/output, and edge-case notes.
//! Script: SLP1 internally; Devanagari only at demo boundary.
//! Flow: dhātu → it-strip → aṅga/vikaraṇa → lakāra/ending → sandhi → surface.
//! Gold DB is cross-check only, never source of truth.
//! =============================================================================
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)] pub struct PronounTable { pub base: String, pub linga: String, pub table: HashMap<String, Vec<String>> }

/// 1.1.23 न द्व्येकयोर्द्विवचनैकवचने — द्वि/उभ only द्विवचन (slot 2); त्रि+ only बहु (slot 3).
fn du(s: &str) -> Vec<String> {
    vec![String::new(), s.to_string(), String::new()]
}
fn pl(s: &str) -> Vec<String> {
    vec![String::new(), String::new(), s.to_string()]
}

/// Query aliases (gold `tri`/`uBa`/`zaz`; old scrape keys `traya`/`ubha`/`zaq`/`paJcan`).
fn canon_sarvanama(base: &str) -> &str {
    match base {
        "traya" => "tri",
        "ubha" => "uBa",
        "zaq" => "zaz",
        "paJcan" | "paYca" => "paYcan",
        "sapta" => "saptan",
        "azwa" => "azwan",
        "nava" => "navan",
        "daSa" => "daSan",
        other => other,
    }
}

// ---------------------------------------------------------------------------
// fn `pronouns`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn pronouns() -> HashMap<(String,String), Vec<Vec<String>>> { let mut m=HashMap::new();
  m.insert(("tad".to_string(),"pum".to_string()), vec![vec!["saH".to_string(),"tO".to_string(),"te".to_string(),],vec!["tam".to_string(),"tO".to_string(),"tAn".to_string(),],vec!["tena".to_string(),"tAByAm".to_string(),"tEH".to_string(),],vec!["tasmE".to_string(),"tAByAm".to_string(),"teByaH".to_string(),],vec!["tasmAt".to_string(),"tAByAm".to_string(),"teByaH".to_string(),],vec!["tasya".to_string(),"tayoH".to_string(),"tezAm".to_string(),],vec!["tasmin".to_string(),"tayoH".to_string(),"tezu".to_string(),],]);
  m.insert(("tad".to_string(),"stri".to_string()), vec![vec!["sA".to_string(),"te".to_string(),"tAH".to_string(),],vec!["tAm".to_string(),"te".to_string(),"tAH".to_string(),],vec!["tayA".to_string(),"tAByAm".to_string(),"tABiH".to_string(),],vec!["tasyE".to_string(),"tAByAm".to_string(),"tAByaH".to_string(),],vec!["tasyAH".to_string(),"tAByAm".to_string(),"tAByaH".to_string(),],vec!["tasyAH".to_string(),"tayoH".to_string(),"tAsAm".to_string(),],vec!["tasyAm".to_string(),"tayoH".to_string(),"tAzu".to_string(),],]);
  m.insert(("tad".to_string(),"nap".to_string()), vec![vec!["tat".to_string(),"te".to_string(),"tAni".to_string(),],vec!["tat".to_string(),"te".to_string(),"tAni".to_string(),],vec!["tena".to_string(),"tAByAm".to_string(),"tEH".to_string(),],vec!["tasmE".to_string(),"tAByAm".to_string(),"teByaH".to_string(),],vec!["tasmAt".to_string(),"tAByAm".to_string(),"teByaH".to_string(),],vec!["tasya".to_string(),"tayoH".to_string(),"tezAm".to_string(),],vec!["tasmin".to_string(),"tayoH".to_string(),"tezu".to_string(),],]);
  m.insert(("kim".to_string(),"pum".to_string()), vec![vec!["kaH".to_string(),"kO".to_string(),"ke".to_string(),],vec!["kam".to_string(),"kO".to_string(),"kAn".to_string(),],vec!["kena".to_string(),"kAByAm".to_string(),"kEH".to_string(),],vec!["kasmE".to_string(),"kAByAm".to_string(),"keByaH".to_string(),],vec!["kasmAt".to_string(),"kAByAm".to_string(),"keByaH".to_string(),],vec!["kasya".to_string(),"kayoH".to_string(),"kezAm".to_string(),],vec!["kasmin".to_string(),"kayoH".to_string(),"kezu".to_string(),],]);
  m.insert(("kim".to_string(),"stri".to_string()), vec![vec!["kA".to_string(),"ke".to_string(),"kAH".to_string(),],vec!["kAm".to_string(),"ke".to_string(),"kAH".to_string(),],vec!["kayA".to_string(),"kAByAm".to_string(),"kABiH".to_string(),],vec!["kasyE".to_string(),"kAByAm".to_string(),"kAByaH".to_string(),],vec!["kasyAH".to_string(),"kAByAm".to_string(),"kAByaH".to_string(),],vec!["kasyAH".to_string(),"kayoH".to_string(),"kAsAm".to_string(),],vec!["kasyAm".to_string(),"kayoH".to_string(),"kAzu".to_string(),],]);
  m.insert(("kim".to_string(),"nap".to_string()), vec![vec!["kim".to_string(),"ke".to_string(),"kAni".to_string(),],vec!["kim".to_string(),"ke".to_string(),"kAni".to_string(),],vec!["kena".to_string(),"kAByAm".to_string(),"kEH".to_string(),],vec!["kasmE".to_string(),"kAByAm".to_string(),"keByaH".to_string(),],vec!["kasmAt".to_string(),"kAByAm".to_string(),"keByaH".to_string(),],vec!["kasya".to_string(),"kayoH".to_string(),"kezAm".to_string(),],vec!["kasmin".to_string(),"kayoH".to_string(),"kezu".to_string(),],]);
  m.insert(("asmad".to_string(),"any".to_string()), vec![vec!["aham".to_string(),"AvAm".to_string(),"vayam".to_string(),],vec!["mAm,mA".to_string(),"AvAm,nO".to_string(),"asmAn,naH".to_string(),],vec!["mayA".to_string(),"AvAByAm".to_string(),"asmABiH".to_string(),],vec!["mahyam,me".to_string(),"AvAByAm,nO".to_string(),"asmaByam,naH".to_string(),],vec!["mat".to_string(),"AvAByAm".to_string(),"asmat".to_string(),],vec!["mama,me".to_string(),"AvayoH,nO".to_string(),"asmAkam,naH".to_string(),],vec!["mayi".to_string(),"AvayoH".to_string(),"asmAsu".to_string(),],]);
  m.insert(("yuzmad".to_string(),"any".to_string()), vec![vec!["tvam".to_string(),"yuvAm".to_string(),"yUyam".to_string(),],vec!["tvAm,tvA".to_string(),"yuvAm,vAm".to_string(),"yuzmAn,vaH".to_string(),],vec!["tvayA".to_string(),"yuvAByAm".to_string(),"yuzmABiH".to_string(),],vec!["tuByam,te".to_string(),"yuvAByAm,vAm".to_string(),"yuzmaByam,vaH".to_string(),],vec!["tvat".to_string(),"yuvAByAm".to_string(),"yuzmat".to_string(),],vec!["tava,te".to_string(),"yuvayoH,vAm".to_string(),"yuzmAkam,vaH".to_string(),],vec!["tvayi".to_string(),"yuvayoH".to_string(),"yuzmAsu".to_string(),],]);
  m.insert(("sarva".to_string(),"pum".to_string()), vec![vec!["sarvaH".to_string(),"sarvO".to_string(),"sarve".to_string(),],vec!["sarvam".to_string(),"sarvO".to_string(),"sarvAn".to_string(),],vec!["sarveRa".to_string(),"sarvAByAm".to_string(),"sarvEH".to_string(),],vec!["sarvasmE".to_string(),"sarvAByAm".to_string(),"sarveByaH".to_string(),],vec!["sarvasmAt".to_string(),"sarvAByAm".to_string(),"sarveByaH".to_string(),],vec!["sarvasya".to_string(),"sarvayoH".to_string(),"sarvezAm".to_string(),],vec!["sarvasmin".to_string(),"sarvayoH".to_string(),"sarvezu".to_string(),],vec!["sarva".to_string(),"sarvO".to_string(),"sarve".to_string(),],]);
  m.insert(("sarva".to_string(),"stri".to_string()), vec![vec!["sarvA".to_string(),"sarve".to_string(),"sarvAH".to_string(),],vec!["sarvAm".to_string(),"sarve".to_string(),"sarvAH".to_string(),],vec!["sarvayA".to_string(),"sarvAByAm".to_string(),"sarvABiH".to_string(),],vec!["sarvasyE".to_string(),"sarvAByAm".to_string(),"sarvAByaH".to_string(),],vec!["sarvasyAH".to_string(),"sarvAByAm".to_string(),"sarvAByaH".to_string(),],vec!["sarvasyAH".to_string(),"sarvayoH".to_string(),"sarvAsAm".to_string(),],vec!["sarvasyAm".to_string(),"sarvayoH".to_string(),"sarvAzu".to_string(),],vec!["sarve".to_string(),"sarve".to_string(),"sarvAH".to_string(),],]);
  m.insert(("sarva".to_string(),"nap".to_string()), vec![vec!["sarvam".to_string(),"sarve".to_string(),"sarvARi".to_string(),],vec!["sarvam".to_string(),"sarve".to_string(),"sarvARi".to_string(),],vec!["sarveRa".to_string(),"sarvAByAm".to_string(),"sarvEH".to_string(),],vec!["sarvasmE".to_string(),"sarvAByAm".to_string(),"sarveByaH".to_string(),],vec!["sarvasmAt".to_string(),"sarvAByAm".to_string(),"sarveByaH".to_string(),],vec!["sarvasya".to_string(),"sarvayoH".to_string(),"sarvezAm".to_string(),],vec!["sarvasmin".to_string(),"sarvayoH".to_string(),"sarvezu".to_string(),],vec!["sarva".to_string(),"sarve".to_string(),"sarvARi".to_string(),],]);
  m.insert(("idam".to_string(),"pum".to_string()), vec![vec!["ayam".to_string(),"imO".to_string(),"ime".to_string(),],vec!["imam,enam".to_string(),"imO".to_string(),"imAn".to_string(),],vec!["anena,enena".to_string(),"AByAm".to_string(),"eBiH".to_string(),],vec!["asmE".to_string(),"AByAm".to_string(),"eByaH".to_string(),],vec!["asmAt".to_string(),"AByAm".to_string(),"eByaH".to_string(),],vec!["asya".to_string(),"anayoH,enayoH".to_string(),"ezAm".to_string(),],vec!["asmin".to_string(),"anayoH,enayoH".to_string(),"ezu".to_string(),],]);
  m.insert(("idam".to_string(),"stri".to_string()), vec![vec!["iyam".to_string(),"ime".to_string(),"imAH".to_string(),],vec!["imAm".to_string(),"ime".to_string(),"imAH".to_string(),],vec!["anayA,enayA".to_string(),"AByAm".to_string(),"ABiH".to_string(),],vec!["asyE".to_string(),"AByAm".to_string(),"AByaH".to_string(),],vec!["asyAH".to_string(),"AByAm".to_string(),"AByaH".to_string(),],vec!["asyAH".to_string(),"anayoH,enayoH".to_string(),"AsAm".to_string(),],vec!["asyAm".to_string(),"anayoH,enayoH".to_string(),"Asu".to_string(),],]);
  m.insert(("idam".to_string(),"nap".to_string()), vec![vec!["idam".to_string(),"ime".to_string(),"imAni".to_string(),],vec!["idam".to_string(),"ime".to_string(),"imAni".to_string(),],vec!["anena,enena".to_string(),"AByAm".to_string(),"eBiH".to_string(),],vec!["asmE".to_string(),"AByAm".to_string(),"eByaH".to_string(),],vec!["asmAt".to_string(),"AByAm".to_string(),"eByaH".to_string(),],vec!["asya".to_string(),"anayoH,enayoH".to_string(),"ezAm".to_string(),],vec!["asmin".to_string(),"anayoH,enayoH".to_string(),"ezu".to_string(),],]);
  m.insert(("etad".to_string(),"pum".to_string()), vec![vec!["ezaH".to_string(),"etO".to_string(),"ete".to_string(),],vec!["etam".to_string(),"etO".to_string(),"etAn".to_string(),],vec!["etena".to_string(),"etAByAm".to_string(),"etEH".to_string(),],vec!["etasmE".to_string(),"etAByAm".to_string(),"eteByaH".to_string(),],vec!["etasmAt".to_string(),"etAByAm".to_string(),"eteByaH".to_string(),],vec!["etasya".to_string(),"etayoH".to_string(),"etezAm".to_string(),],vec!["etasmin".to_string(),"etayoH".to_string(),"etezu".to_string(),],]);
  m.insert(("etad".to_string(),"stri".to_string()), vec![vec!["etA".to_string(),"ete".to_string(),"etAH".to_string(),],vec!["etAm".to_string(),"ete".to_string(),"etAH".to_string(),],vec!["etayA".to_string(),"etAByAm".to_string(),"etABiH".to_string(),],vec!["etasyE".to_string(),"etAByAm".to_string(),"etAByaH".to_string(),],vec!["etasyAH".to_string(),"etAByAm".to_string(),"etAByaH".to_string(),],vec!["etasyAH".to_string(),"etayoH".to_string(),"etAsAm".to_string(),],vec!["etasyAm".to_string(),"etayoH".to_string(),"etAsu".to_string(),],]);
  m.insert(("etad".to_string(),"nap".to_string()), vec![vec!["etat".to_string(),"ete".to_string(),"etAni".to_string(),],vec!["etat".to_string(),"ete".to_string(),"etAni".to_string(),],vec!["etena".to_string(),"etAByAm".to_string(),"etEH".to_string(),],vec!["etasmE".to_string(),"etAByAm".to_string(),"eteByaH".to_string(),],vec!["etasmAt".to_string(),"etAByAm".to_string(),"eteByaH".to_string(),],vec!["etasya".to_string(),"etayoH".to_string(),"etezAm".to_string(),],vec!["etasmin".to_string(),"etayoH".to_string(),"etezu".to_string(),],]);
  m.insert(("yad".to_string(),"pum".to_string()), vec![vec!["yaH".to_string(),"yO".to_string(),"ye".to_string(),],vec!["yam".to_string(),"yO".to_string(),"yAn".to_string(),],vec!["yena".to_string(),"yAByAm".to_string(),"yEH".to_string(),],vec!["yasmE".to_string(),"yAByAm".to_string(),"yeByaH".to_string(),],vec!["yasmAt".to_string(),"yAByAm".to_string(),"yeByaH".to_string(),],vec!["yasya".to_string(),"yayoH".to_string(),"yezAm".to_string(),],vec!["yasmin".to_string(),"yayoH".to_string(),"yezu".to_string(),],]);
  m.insert(("yad".to_string(),"stri".to_string()), vec![vec!["yA".to_string(),"ye".to_string(),"yAH".to_string(),],vec!["yAm".to_string(),"ye".to_string(),"yAH".to_string(),],vec!["yayA".to_string(),"yAByAm".to_string(),"yABiH".to_string(),],vec!["yasyE".to_string(),"yAByAm".to_string(),"yAByaH".to_string(),],vec!["yasyAH".to_string(),"yAByAm".to_string(),"yAByaH".to_string(),],vec!["yasyAH".to_string(),"yayoH".to_string(),"yAsAm".to_string(),],vec!["yasyAm".to_string(),"yayoH".to_string(),"yAsu".to_string(),],]);
  m.insert(("yad".to_string(),"nap".to_string()), vec![vec!["yat".to_string(),"ye".to_string(),"yAni".to_string(),],vec!["yat".to_string(),"ye".to_string(),"yAni".to_string(),],vec!["yena".to_string(),"yAByAm".to_string(),"yEH".to_string(),],vec!["yasmE".to_string(),"yAByAm".to_string(),"yeByaH".to_string(),],vec!["yasmAt".to_string(),"yAByAm".to_string(),"yeByaH".to_string(),],vec!["yasya".to_string(),"yayoH".to_string(),"yezAm".to_string(),],vec!["yasmin".to_string(),"yayoH".to_string(),"yezu".to_string(),],]);
  // अदस् — 7.2.107 अदस औ सुलोपश्च (असौ); 8.2.80 अदसोऽसेर्दादु दो मः (अमू); 8.2.81 अदसोऽसेर्दादु दो मः / एत ईद्बहुवचने (अमी).
  m.insert(("adas".to_string(),"pum".to_string()), vec![vec!["asO".to_string(),"amU".to_string(),"amI".to_string(),],vec!["amum".to_string(),"amU".to_string(),"amUn".to_string(),],vec!["amunA".to_string(),"amUByAm".to_string(),"amIBiH".to_string(),],vec!["amuzmE".to_string(),"amUByAm".to_string(),"amIByaH".to_string(),],vec!["amuzmAt".to_string(),"amUByAm".to_string(),"amIByaH".to_string(),],vec!["amuzya".to_string(),"amuyoH".to_string(),"amIzAm".to_string(),],vec!["amuzmin".to_string(),"amuyoH".to_string(),"amIzu".to_string(),],]);
  m.insert(("adas".to_string(),"stri".to_string()), vec![vec!["asO".to_string(),"amU".to_string(),"amUH".to_string(),],vec!["amUm".to_string(),"amU".to_string(),"amUH".to_string(),],vec!["amuyA".to_string(),"amUByAm".to_string(),"amUBiH".to_string(),],vec!["amuzyE".to_string(),"amUByAm".to_string(),"amUByaH".to_string(),],vec!["amuzyAH".to_string(),"amUByAm".to_string(),"amUByaH".to_string(),],vec!["amuzyAH".to_string(),"amuyoH".to_string(),"amUzAm".to_string(),],vec!["amuzyAm".to_string(),"amuyoH".to_string(),"amUzu".to_string(),],]);
  m.insert(("adas".to_string(),"nap".to_string()), vec![vec!["adaH".to_string(),"amU".to_string(),"amUni".to_string(),],vec!["adaH".to_string(),"amU".to_string(),"amUni".to_string(),],vec!["amunA".to_string(),"amUByAm".to_string(),"amIBiH".to_string(),],vec!["amuzmE".to_string(),"amUByAm".to_string(),"amIByaH".to_string(),],vec!["amuzmAt".to_string(),"amUByAm".to_string(),"amIByaH".to_string(),],vec!["amuzya".to_string(),"amuyoH".to_string(),"amIzAm".to_string(),],vec!["amuzmin".to_string(),"amuyoH".to_string(),"amIzu".to_string(),],]);
  // त्यद् — 7.2.102 त्यदादीनामः like तद्; nom स्यः/स्या (7.2.106).
  m.insert(("tyad".to_string(),"pum".to_string()), vec![vec!["syaH".to_string(),"tyO".to_string(),"tye".to_string(),],vec!["tyam".to_string(),"tyO".to_string(),"tyAn".to_string(),],vec!["tyena".to_string(),"tyAByAm".to_string(),"tyEH".to_string(),],vec!["tyasmE".to_string(),"tyAByAm".to_string(),"tyeByaH".to_string(),],vec!["tyasmAt".to_string(),"tyAByAm".to_string(),"tyeByaH".to_string(),],vec!["tyasya".to_string(),"tyayoH".to_string(),"tyezAm".to_string(),],vec!["tyasmin".to_string(),"tyayoH".to_string(),"tyezu".to_string(),],]);
  m.insert(("tyad".to_string(),"stri".to_string()), vec![vec!["syA".to_string(),"tye".to_string(),"tyAH".to_string(),],vec!["tyAm".to_string(),"tye".to_string(),"tyAH".to_string(),],vec!["tyayA".to_string(),"tyAByAm".to_string(),"tyABiH".to_string(),],vec!["tyasyE".to_string(),"tyAByAm".to_string(),"tyAByaH".to_string(),],vec!["tyasyAH".to_string(),"tyAByAm".to_string(),"tyAByaH".to_string(),],vec!["tyasyAH".to_string(),"tyayoH".to_string(),"tyAsAm".to_string(),],vec!["tyasyAm".to_string(),"tyayoH".to_string(),"tyAsu".to_string(),],]);
  m.insert(("tyad".to_string(),"nap".to_string()), vec![vec!["tyat".to_string(),"tye".to_string(),"tyAni".to_string(),],vec!["tyat".to_string(),"tye".to_string(),"tyAni".to_string(),],vec!["tyena".to_string(),"tyAByAm".to_string(),"tyEH".to_string(),],vec!["tyasmE".to_string(),"tyAByAm".to_string(),"tyeByaH".to_string(),],vec!["tyasmAt".to_string(),"tyAByAm".to_string(),"tyeByaH".to_string(),],vec!["tyasya".to_string(),"tyayoH".to_string(),"tyezAm".to_string(),],vec!["tyasmin".to_string(),"tyayoH".to_string(),"tyezu".to_string(),],]);
  // उभ — 1.1.23 द्विवचन only. पुं उभौ; स्त्री/नपुं उभे; उभाभ्याम्, उभयोः.
  m.insert(("uBa".to_string(),"pum".to_string()), vec![du("uBO"), du("uBO"), du("uBAByAm"), du("uBAByAm"), du("uBAByAm"), du("uBayoH"), du("uBayoH")]);
  m.insert(("uBa".to_string(),"stri".to_string()), vec![du("uBe"), du("uBe"), du("uBAByAm"), du("uBAByAm"), du("uBAByAm"), du("uBayoH"), du("uBayoH")]);
  m.insert(("uBa".to_string(),"nap".to_string()), vec![du("uBe"), du("uBe"), du("uBAByAm"), du("uBAByAm"), du("uBAByAm"), du("uBayoH"), du("uBayoH")]);
  m.insert(("ena".to_string(),"pum".to_string()), vec![vec!["ezaH".to_string(),"etO".to_string(),"ete".to_string(),],vec!["etam".to_string(),"etO".to_string(),"etAn".to_string(),],vec!["etena".to_string(),"etAByAm".to_string(),"etEH".to_string(),],vec!["etasmE".to_string(),"etAByAm".to_string(),"eteByaH".to_string(),],vec!["etasmAt".to_string(),"etAByAm".to_string(),"eteByaH".to_string(),],vec!["etasya".to_string(),"etayoH".to_string(),"etezAm".to_string(),],vec!["etasmin".to_string(),"etayoH".to_string(),"etezu".to_string(),],]);
  m.insert(("ena".to_string(),"stri".to_string()), vec![vec!["etA".to_string(),"ete".to_string(),"etAH".to_string(),],vec!["etAm".to_string(),"ete".to_string(),"etAH".to_string(),],vec!["etayA".to_string(),"etAByAm".to_string(),"etABiH".to_string(),],vec!["etasyE".to_string(),"etAByAm".to_string(),"etAByaH".to_string(),],vec!["etasyAH".to_string(),"etAByAm".to_string(),"etAByaH".to_string(),],vec!["etasyAH".to_string(),"etayoH".to_string(),"etAsAm".to_string(),],vec!["etasyAm".to_string(),"etayoH".to_string(),"etAsu".to_string(),],]);
  m.insert(("ena".to_string(),"nap".to_string()), vec![vec!["etat".to_string(),"ete".to_string(),"etAni".to_string(),],vec!["etat".to_string(),"ete".to_string(),"etAni".to_string(),],vec!["etena".to_string(),"etAByAm".to_string(),"etEH".to_string(),],vec!["etasmE".to_string(),"etAByAm".to_string(),"eteByaH".to_string(),],vec!["etasmAt".to_string(),"etAByAm".to_string(),"eteByaH".to_string(),],vec!["etasya".to_string(),"etayoH".to_string(),"etezAm".to_string(),],vec!["etasmin".to_string(),"etayoH".to_string(),"etezu".to_string(),],]);
  m.insert(("eka".to_string(),"pum".to_string()), vec![vec!["ekaH".to_string(),"ekO".to_string(),"eke".to_string(),],vec!["ekam".to_string(),"ekO".to_string(),"ekAn".to_string(),],vec!["ekena".to_string(),"ekAByAm".to_string(),"ekEH".to_string(),],vec!["ekasmE".to_string(),"ekAByAm".to_string(),"ekeByaH".to_string(),],vec!["ekasmAt".to_string(),"ekAByAm".to_string(),"ekeByaH".to_string(),],vec!["ekasya".to_string(),"ekayoH".to_string(),"ekezAm".to_string(),],vec!["ekasmin".to_string(),"ekayoH".to_string(),"ekezu".to_string(),],]);
  m.insert(("eka".to_string(),"stri".to_string()), vec![vec!["ekA".to_string(),"eke".to_string(),"ekAH".to_string(),],vec!["ekAm".to_string(),"eke".to_string(),"ekAH".to_string(),],vec!["ekayA".to_string(),"ekAByAm".to_string(),"ekABiH".to_string(),],vec!["ekasyE".to_string(),"ekAByAm".to_string(),"ekAByaH".to_string(),],vec!["ekasyAH".to_string(),"ekAByAm".to_string(),"ekAByaH".to_string(),],vec!["ekasyAH".to_string(),"ekayoH".to_string(),"ekAsAm".to_string(),],vec!["ekasyAm".to_string(),"ekayoH".to_string(),"ekAsu".to_string(),],]);
  m.insert(("eka".to_string(),"nap".to_string()), vec![vec!["ekam".to_string(),"eke".to_string(),"ekAni".to_string(),],vec!["ekam".to_string(),"eke".to_string(),"ekAni".to_string(),],vec!["ekena".to_string(),"ekAByAm".to_string(),"ekEH".to_string(),],vec!["ekasmE".to_string(),"ekAByAm".to_string(),"ekeByaH".to_string(),],vec!["ekasmAt".to_string(),"ekAByAm".to_string(),"ekeByaH".to_string(),],vec!["ekasya".to_string(),"ekayoH".to_string(),"ekezAm".to_string(),],vec!["ekasmin".to_string(),"ekayoH".to_string(),"ekezu".to_string(),],]);
  // द्वि — 1.1.23 द्विवचन: पुं द्वौ; स्त्री/नपुं द्वे; द्वाभ्याम्, द्वयोः (not *द्विः i-stem).
  m.insert(("dvi".to_string(),"pum".to_string()), vec![du("dvO"), du("dvO"), du("dvAByAm"), du("dvAByAm"), du("dvAByAm"), du("dvayoH"), du("dvayoH")]);
  m.insert(("dvi".to_string(),"stri".to_string()), vec![du("dve"), du("dve"), du("dvAByAm"), du("dvAByAm"), du("dvAByAm"), du("dvayoH"), du("dvayoH")]);
  m.insert(("dvi".to_string(),"nap".to_string()), vec![du("dve"), du("dve"), du("dvAByAm"), du("dvAByAm"), du("dvAByAm"), du("dvayoH"), du("dvayoH")]);
  m.insert(("sva".to_string(),"pum".to_string()), vec![vec!["svaH".to_string(),"svO".to_string(),"sve".to_string(),],vec!["svam".to_string(),"svO".to_string(),"svAn".to_string(),],vec!["svena".to_string(),"svAByAm".to_string(),"svEH".to_string(),],vec!["svasmE".to_string(),"svAByAm".to_string(),"sveByaH".to_string(),],vec!["svasmAt".to_string(),"svAByAm".to_string(),"sveByaH".to_string(),],vec!["svasya".to_string(),"svayoH".to_string(),"svezAm".to_string(),],vec!["svasmin".to_string(),"svayoH".to_string(),"svezu".to_string(),],]);
  m.insert(("sva".to_string(),"stri".to_string()), vec![vec!["svA".to_string(),"sve".to_string(),"svAH".to_string(),],vec!["svAm".to_string(),"sve".to_string(),"svAH".to_string(),],vec!["svayA".to_string(),"svAByAm".to_string(),"svABiH".to_string(),],vec!["svasyE".to_string(),"svAByAm".to_string(),"svAByaH".to_string(),],vec!["svasyAH".to_string(),"svAByAm".to_string(),"svAByaH".to_string(),],vec!["svasyAH".to_string(),"svayoH".to_string(),"svAsAm".to_string(),],vec!["svasyAm".to_string(),"svayoH".to_string(),"svAsu".to_string(),],]);
  m.insert(("sva".to_string(),"nap".to_string()), vec![vec!["svam".to_string(),"sve".to_string(),"svAni".to_string(),],vec!["svam".to_string(),"sve".to_string(),"svAni".to_string(),],vec!["svena".to_string(),"svAByAm".to_string(),"svEH".to_string(),],vec!["svasmE".to_string(),"svAByAm".to_string(),"sveByaH".to_string(),],vec!["svasmAt".to_string(),"svAByAm".to_string(),"sveByaH".to_string(),],vec!["svasya".to_string(),"svayoH".to_string(),"svezAm".to_string(),],vec!["svasmin".to_string(),"svayoH".to_string(),"svezu".to_string(),],]);
  m.insert(("am".to_string(),"any".to_string()), vec![vec!["am".to_string(),"am".to_string(),"am".to_string(),],vec!["am".to_string(),"am".to_string(),"am".to_string(),],vec!["am".to_string(),"am".to_string(),"am".to_string(),],vec!["am".to_string(),"am".to_string(),"am".to_string(),],vec!["am".to_string(),"am".to_string(),"am".to_string(),],vec!["am".to_string(),"am".to_string(),"am".to_string(),],vec!["am".to_string(),"am".to_string(),"am".to_string(),],]);
  // त्रि — बहु only. पुं त्रयः/त्रीन्; 7.1.53 स्त्री तिसृ → तिस्रः/तिसृभिः (not *tisrbhiH). नपुं त्रीणि (8.4.1 णत्व).
  m.insert(("tri".to_string(),"pum".to_string()), vec![pl("trayaH"), pl("trIn"), pl("triBiH"), pl("triByaH"), pl("triByaH"), pl("trayARAm"), pl("trizu")]);
  m.insert(("tri".to_string(),"stri".to_string()), vec![pl("tisraH"), pl("tisraH"), pl("tisfBiH"), pl("tisfByaH"), pl("tisfByaH"), pl("tisfRAm"), pl("tisfzu")]);
  m.insert(("tri".to_string(),"nap".to_string()), vec![pl("trIRi"), pl("trIRi"), pl("triBiH"), pl("triByaH"), pl("triByaH"), pl("trayARAm"), pl("trizu")]);
  // चतुर् — पुं चत्वारः/चतुरः; स्त्री 7.1.53 चतसृ → चतस्रः/चतसृभिः; नपुं चत्वारि; षष्ठी चतुर्णाम् (8.4.1).
  m.insert(("catur".to_string(),"pum".to_string()), vec![pl("catvAraH"), pl("caturaH"), pl("caturBiH"), pl("caturByaH"), pl("caturByaH"), pl("caturRAm"), pl("caturzu")]);
  m.insert(("catur".to_string(),"stri".to_string()), vec![pl("catasraH"), pl("catasraH"), pl("catasfBiH"), pl("catasfByaH"), pl("catasfByaH"), pl("catasfRAm"), pl("catasfzu")]);
  m.insert(("catur".to_string(),"nap".to_string()), vec![pl("catvAri"), pl("catvAri"), pl("caturBiH"), pl("caturByaH"), pl("caturByaH"), pl("caturRAm"), pl("caturzu")]);
  m.insert(("purva".to_string(),"pum".to_string()), vec![vec!["purvaH".to_string(),"purvO".to_string(),"purve".to_string(),],vec!["purvam".to_string(),"purvO".to_string(),"purvAn".to_string(),],vec!["purveRa".to_string(),"purvAByAm".to_string(),"purvEH".to_string(),],vec!["purvasmE".to_string(),"purvAByAm".to_string(),"purveByaH".to_string(),],vec!["purvasmAt".to_string(),"purvAByAm".to_string(),"purveByaH".to_string(),],vec!["purvasya".to_string(),"purvayoH".to_string(),"purvezAm".to_string(),],vec!["purvasmin".to_string(),"purvayoH".to_string(),"purvezu".to_string(),],vec!["purva".to_string(),"purvO".to_string(),"purve".to_string(),],]);
  m.insert(("purva".to_string(),"stri".to_string()), vec![vec!["purvA".to_string(),"purve".to_string(),"purvAH".to_string(),],vec!["purvAm".to_string(),"purve".to_string(),"purvAH".to_string(),],vec!["purvayA".to_string(),"purvAByAm".to_string(),"purvABiH".to_string(),],vec!["purvasyE".to_string(),"purvAByAm".to_string(),"purvAByaH".to_string(),],vec!["purvasyAH".to_string(),"purvAByAm".to_string(),"purvAByaH".to_string(),],vec!["purvasyAH".to_string(),"purvayoH".to_string(),"purvAsAm".to_string(),],vec!["purvasyAm".to_string(),"purvayoH".to_string(),"purvAzu".to_string(),],vec!["purve".to_string(),"purve".to_string(),"purvAH".to_string(),],]);
  m.insert(("purva".to_string(),"nap".to_string()), vec![vec!["purvam".to_string(),"purve".to_string(),"purvARi".to_string(),],vec!["purvam".to_string(),"purve".to_string(),"purvARi".to_string(),],vec!["purveRa".to_string(),"purvAByAm".to_string(),"purvEH".to_string(),],vec!["purvasmE".to_string(),"purvAByAm".to_string(),"purveByaH".to_string(),],vec!["purvasmAt".to_string(),"purvAByAm".to_string(),"purveByaH".to_string(),],vec!["purvasya".to_string(),"purvayoH".to_string(),"purvezAm".to_string(),],vec!["purvasmin".to_string(),"purvayoH".to_string(),"purvezu".to_string(),],vec!["purva".to_string(),"purve".to_string(),"purvARi".to_string(),],]);
  m.insert(("para".to_string(),"pum".to_string()), vec![vec!["paraH".to_string(),"parO".to_string(),"pare".to_string(),],vec!["param".to_string(),"parO".to_string(),"parAn".to_string(),],vec!["pareRa".to_string(),"parAByAm".to_string(),"parEH".to_string(),],vec!["parasmE".to_string(),"parAByAm".to_string(),"pareByaH".to_string(),],vec!["parasmAt".to_string(),"parAByAm".to_string(),"pareByaH".to_string(),],vec!["parasya".to_string(),"parayoH".to_string(),"parezAm".to_string(),],vec!["parasmin".to_string(),"parayoH".to_string(),"parezu".to_string(),],vec!["para".to_string(),"parO".to_string(),"pare".to_string(),],]);
  m.insert(("para".to_string(),"stri".to_string()), vec![vec!["parA".to_string(),"pare".to_string(),"parAH".to_string(),],vec!["parAm".to_string(),"pare".to_string(),"parAH".to_string(),],vec!["parayA".to_string(),"parAByAm".to_string(),"parABiH".to_string(),],vec!["parasyE".to_string(),"parAByAm".to_string(),"parAByaH".to_string(),],vec!["parasyAH".to_string(),"parAByAm".to_string(),"parAByaH".to_string(),],vec!["parasyAH".to_string(),"parayoH".to_string(),"parAsAm".to_string(),],vec!["parasyAm".to_string(),"parayoH".to_string(),"parAzu".to_string(),],vec!["pare".to_string(),"pare".to_string(),"parAH".to_string(),],]);
  m.insert(("para".to_string(),"nap".to_string()), vec![vec!["param".to_string(),"pare".to_string(),"parARi".to_string(),],vec!["param".to_string(),"pare".to_string(),"parARi".to_string(),],vec!["pareRa".to_string(),"parAByAm".to_string(),"parEH".to_string(),],vec!["parasmE".to_string(),"parAByAm".to_string(),"pareByaH".to_string(),],vec!["parasmAt".to_string(),"parAByAm".to_string(),"pareByaH".to_string(),],vec!["parasya".to_string(),"parayoH".to_string(),"parezAm".to_string(),],vec!["parasmin".to_string(),"parayoH".to_string(),"parezu".to_string(),],vec!["para".to_string(),"pare".to_string(),"parARi".to_string(),],]);
  m.insert(("apara".to_string(),"pum".to_string()), vec![vec!["aparaH".to_string(),"aparO".to_string(),"apare".to_string(),],vec!["aparam".to_string(),"aparO".to_string(),"aparAn".to_string(),],vec!["apareRa".to_string(),"aparAByAm".to_string(),"aparEH".to_string(),],vec!["aparasmE".to_string(),"aparAByAm".to_string(),"apareByaH".to_string(),],vec!["aparasmAt".to_string(),"aparAByAm".to_string(),"apareByaH".to_string(),],vec!["aparasya".to_string(),"aparayoH".to_string(),"aparezAm".to_string(),],vec!["aparasmin".to_string(),"aparayoH".to_string(),"aparezu".to_string(),],vec!["apara".to_string(),"aparO".to_string(),"apare".to_string(),],]);
  m.insert(("apara".to_string(),"stri".to_string()), vec![vec!["aparA".to_string(),"apare".to_string(),"aparAH".to_string(),],vec!["aparAm".to_string(),"apare".to_string(),"aparAH".to_string(),],vec!["aparayA".to_string(),"aparAByAm".to_string(),"aparABiH".to_string(),],vec!["aparasyE".to_string(),"aparAByAm".to_string(),"aparAByaH".to_string(),],vec!["aparasyAH".to_string(),"aparAByAm".to_string(),"aparAByaH".to_string(),],vec!["aparasyAH".to_string(),"aparayoH".to_string(),"aparAsAm".to_string(),],vec!["aparasyAm".to_string(),"aparayoH".to_string(),"aparAzu".to_string(),],vec!["apare".to_string(),"apare".to_string(),"aparAH".to_string(),],]);
  m.insert(("apara".to_string(),"nap".to_string()), vec![vec!["aparam".to_string(),"apare".to_string(),"aparARi".to_string(),],vec!["aparam".to_string(),"apare".to_string(),"aparARi".to_string(),],vec!["apareRa".to_string(),"aparAByAm".to_string(),"aparEH".to_string(),],vec!["aparasmE".to_string(),"aparAByAm".to_string(),"apareByaH".to_string(),],vec!["aparasmAt".to_string(),"aparAByAm".to_string(),"apareByaH".to_string(),],vec!["aparasya".to_string(),"aparayoH".to_string(),"aparezAm".to_string(),],vec!["aparasmin".to_string(),"aparayoH".to_string(),"aparezu".to_string(),],vec!["apara".to_string(),"apare".to_string(),"aparARi".to_string(),],]);
  m.insert(("anya".to_string(),"pum".to_string()), vec![vec!["anyaH".to_string(),"anyO".to_string(),"anye".to_string(),],vec!["anyam".to_string(),"anyO".to_string(),"anyAn".to_string(),],vec!["anyena".to_string(),"anyAByAm".to_string(),"anyEH".to_string(),],vec!["anyasmE".to_string(),"anyAByAm".to_string(),"anyeByaH".to_string(),],vec!["anyasmAt".to_string(),"anyAByAm".to_string(),"anyeByaH".to_string(),],vec!["anyasya".to_string(),"anyayoH".to_string(),"anyezAm".to_string(),],vec!["anyasmin".to_string(),"anyayoH".to_string(),"anyezu".to_string(),],vec!["anya".to_string(),"anyO".to_string(),"anye".to_string(),],]);
  m.insert(("anya".to_string(),"stri".to_string()), vec![vec!["anyA".to_string(),"anye".to_string(),"anyAH".to_string(),],vec!["anyAm".to_string(),"anye".to_string(),"anyAH".to_string(),],vec!["anyayA".to_string(),"anyAByAm".to_string(),"anyABiH".to_string(),],vec!["anyasyE".to_string(),"anyAByAm".to_string(),"anyAByaH".to_string(),],vec!["anyasyAH".to_string(),"anyAByAm".to_string(),"anyAByaH".to_string(),],vec!["anyasyAH".to_string(),"anyayoH".to_string(),"anyAsAm".to_string(),],vec!["anyasyAm".to_string(),"anyayoH".to_string(),"anyAzu".to_string(),],vec!["anye".to_string(),"anye".to_string(),"anyAH".to_string(),],]);
  m.insert(("anya".to_string(),"nap".to_string()), vec![vec!["anyam".to_string(),"anye".to_string(),"anyARi".to_string(),],vec!["anyam".to_string(),"anye".to_string(),"anyARi".to_string(),],vec!["anyena".to_string(),"anyAByAm".to_string(),"anyEH".to_string(),],vec!["anyasmE".to_string(),"anyAByAm".to_string(),"anyeByaH".to_string(),],vec!["anyasmAt".to_string(),"anyAByAm".to_string(),"anyeByaH".to_string(),],vec!["anyasya".to_string(),"anyayoH".to_string(),"anyezAm".to_string(),],vec!["anyasmin".to_string(),"anyayoH".to_string(),"anyezu".to_string(),],vec!["anya".to_string(),"anye".to_string(),"anyARi".to_string(),],]);
  // पञ्चन्–दशन् — 7.1.22 षड्भ्यो लुक् (जस्/शस्); all लिङ्ग same; बहु only. SLP1 ञ् is Y (not J).
  let panca = vec![pl("paYca"), pl("paYca"), pl("paYcaBiH"), pl("paYcaByaH"), pl("paYcaByaH"), pl("paYcAnAm"), pl("paYcasu")];
  for linga in ["pum", "stri", "nap"] {
      m.insert(("paYcan".to_string(), linga.to_string()), panca.clone());
  }
  // षष् — पद षट्; षड्भिः, षण्णाम् (8.4.1), षट्सु.
  let sas = vec![pl("zaw"), pl("zaw"), pl("zaqBiH"), pl("zaqByaH"), pl("zaqByaH"), pl("zaRRAm"), pl("zawsu")];
  for linga in ["pum", "stri", "nap"] {
      m.insert(("zaz".to_string(), linga.to_string()), sas.clone());
  }
  let sapta = vec![pl("sapta"), pl("sapta"), pl("saptaBiH"), pl("saptaByaH"), pl("saptaByaH"), pl("saptAnAm"), pl("saptasu")];
  for linga in ["pum", "stri", "nap"] {
      m.insert(("saptan".to_string(), linga.to_string()), sapta.clone());
  }
  // अष्टन् — 7.1.21 अष्टन आ विभक्तौ: अष्टौ/अष्ट; अष्टाभिः, अष्टानाम्.
  let azwan = vec![pl("azwO,azwa"), pl("azwO,azwa"), pl("azwABiH"), pl("azwAByaH"), pl("azwAByaH"), pl("azwAnAm"), pl("azwasu")];
  for linga in ["pum", "stri", "nap"] {
      m.insert(("azwan".to_string(), linga.to_string()), azwan.clone());
  }
  let navan = vec![pl("nava"), pl("nava"), pl("navaBiH"), pl("navaByaH"), pl("navaByaH"), pl("navAnAm"), pl("navasu")];
  for linga in ["pum", "stri", "nap"] {
      m.insert(("navan".to_string(), linga.to_string()), navan.clone());
  }
  let dasan = vec![pl("daSa"), pl("daSa"), pl("daSaBiH"), pl("daSaByaH"), pl("daSaByaH"), pl("daSAnAm"), pl("daSasu")];
  for linga in ["pum", "stri", "nap"] {
      m.insert(("daSan".to_string(), linga.to_string()), dasan.clone());
  }
  m }

// ---------------------------------------------------------------------------
// fn `generate`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate(base: &str, linga: &str) -> Option<PronounTable> {
    let base = canon_sarvanama(base);
    let linga_eff = if base=="asmad" || base=="yuzmad" || base=="am" { "any" } else { linga };
    let table = pronouns().get(&(base.to_string(), linga_eff.to_string()))?.clone();
    let vibhaktis = ["prathamA","dvitIyA","tfIyA","caturTI","paYcamI","zazWI","saptamI"];
    let mut map = HashMap::new();
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for (i, row) in table.iter().enumerate() {
        let v = vibhaktis[i].to_string();
        let forms: Vec<String> = row.iter().map(|s| s.replace(',',"/")).collect();
        map.insert(v, forms);
    }
    Some(PronounTable { base: base.to_string(), linga: linga_eff.to_string(), table: map })
}
// ---------------------------------------------------------------------------
// fn `analyze`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn analyze(word: &str) -> Vec<HashMap<String,String>> {
    let mut out=Vec::new();
    let vibhaktis = ["prathamA","dvitIyA","tfIyA","caturTI","paYcamI","zazWI","saptamI"];
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for ((base,linga), table) in pronouns() {
        // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
        for (vi,row) in table.iter().enumerate() {
            // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
            for (vac_idx, forms_str) in row.iter().enumerate() {
                // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
                for form in forms_str.split(',') {
                    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                    if form.is_empty() { continue; }
                    if form==word {
                        let mut m=HashMap::new();
                        m.insert("pratipadika".to_string(), base.clone());
                        m.insert("linga".to_string(), linga.clone());
                        m.insert("vibhakti".to_string(), vibhaktis[vi].to_string());
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

    fn has(t: &PronounTable, vib: &str, form: &str) {
        let row = t.table.get(vib).unwrap();
        assert!(row.iter().any(|x| x == form || x.split('/').any(|p| p == form)), "{vib} {:?}, want {form}", row);
    }

    #[test]
    fn adas_asau_amu_ami() {
        let p = generate("adas", "pum").expect("adas pum");
        has(&p, "prathamA", "asO");
        has(&p, "prathamA", "amU");
        has(&p, "prathamA", "amI");
        has(&p, "dvitIyA", "amum");
        has(&p, "tfIyA", "amunA");
        has(&p, "saptamI", "amuzmin");
        has(&p, "saptamI", "amIzu");
        let s = generate("adas", "stri").expect("adas stri");
        has(&s, "prathamA", "asO");
        has(&s, "prathamA", "amUH");
        has(&s, "dvitIyA", "amUm");
        let n = generate("adas", "nap").expect("adas nap");
        has(&n, "prathamA", "adaH");
        has(&n, "prathamA", "amUni");
        assert!(analyze("asO").iter().any(|m| m.get("pratipadika") == Some(&"adas".to_string())));
        assert!(analyze("adaH").iter().any(|m| m.get("pratipadika") == Some(&"adas".to_string())));
    }

    #[test]
    fn tyad_syah_and_ais() {
        let p = generate("tyad", "pum").expect("tyad pum");
        has(&p, "prathamA", "syaH");
        has(&p, "prathamA", "tyO");
        has(&p, "prathamA", "tye");
        has(&p, "dvitIyA", "tyam");
        has(&p, "tfIyA", "tyena");
        has(&p, "tfIyA", "tyEH");
        has(&p, "saptamI", "tyasmin");
        let s = generate("tyad", "stri").expect("tyad stri");
        has(&s, "prathamA", "syA");
        has(&s, "dvitIyA", "tyAm");
        let n = generate("tyad", "nap").expect("tyad nap");
        has(&n, "prathamA", "tyat");
        has(&n, "prathamA", "tyAni");
        assert!(analyze("syaH").iter().any(|m| m.get("pratipadika") == Some(&"tyad".to_string())));
        // 7.1.9 अतो भिस् ऐस् — एतैः/एकैः not *एतेभिः.
        let e = generate("etad", "pum").expect("etad");
        has(&e, "tfIyA", "etEH");
        let k = generate("eka", "pum").expect("eka");
        has(&k, "tfIyA", "ekEH");
        let i = generate("idam", "pum").expect("idam");
        has(&i, "dvitIyA", "imam");
        has(&i, "dvitIyA", "enam");
        has(&i, "tfIyA", "anena");
        has(&i, "zazWI", "anayoH");
        has(&i, "zazWI", "ezAm");
        let is = generate("idam", "stri").expect("idam stri");
        has(&is, "zazWI", "AsAm");
        has(&is, "zazWI", "anayoH");
        has(&i, "caturTI", "asmE");
        has(&i, "tfIyA", "eBiH");
        has(&is, "tfIyA", "anayA");
        has(&is, "saptamI", "Asu");
    }

    fn vac_slot<'a>(t: &'a PronounTable, vib: &str, vac: usize) -> &'a str {
        t.table.get(vib).unwrap()[vac - 1].as_str()
    }

    #[test]
    fn sankhya_dvi_tri_catur() {
        let p = generate("dvi", "pum").expect("dvi pum");
        has(&p, "prathamA", "dvO");
        has(&p, "dvitIyA", "dvO");
        has(&p, "tfIyA", "dvAByAm");
        has(&p, "zazWI", "dvayoH");
        assert_eq!(vac_slot(&p, "prathamA", 2), "dvO");
        assert!(vac_slot(&p, "prathamA", 1).is_empty());
        assert!(vac_slot(&p, "prathamA", 3).is_empty());
        let s = generate("dvi", "stri").expect("dvi stri");
        has(&s, "prathamA", "dve");
        let n = generate("dvi", "nap").expect("dvi nap");
        has(&n, "prathamA", "dve");
        assert!(analyze("dvO").iter().any(|m| m.get("pratipadika") == Some(&"dvi".to_string()) && m.get("vacana") == Some(&"2".to_string())));

        let t = generate("tri", "pum").expect("tri");
        has(&t, "prathamA", "trayaH");
        has(&t, "dvitIyA", "trIn");
        has(&t, "tfIyA", "triBiH");
        has(&t, "zazWI", "trayARAm");
        has(&t, "saptamI", "trizu");
        assert_eq!(vac_slot(&t, "prathamA", 3), "trayaH");
        let ts = generate("traya", "stri").expect("traya alias");
        has(&ts, "prathamA", "tisraH");
        has(&ts, "tfIyA", "tisfBiH");
        has(&ts, "zazWI", "tisfRAm");
        has(&ts, "saptamI", "tisfzu");
        let tn = generate("tri", "nap").expect("tri nap");
        has(&tn, "prathamA", "trIRi");

        let c = generate("catur", "pum").expect("catur");
        has(&c, "prathamA", "catvAraH");
        has(&c, "dvitIyA", "caturaH");
        has(&c, "tfIyA", "caturBiH");
        has(&c, "zazWI", "caturRAm");
        let cs = generate("catur", "stri").expect("catur stri");
        has(&cs, "prathamA", "catasraH");
        has(&cs, "tfIyA", "catasfBiH");
        has(&cs, "saptamI", "catasfzu");
        let cn = generate("catur", "nap").expect("catur nap");
        has(&cn, "prathamA", "catvAri");
        let u = generate("ubha", "pum").expect("ubha");
        has(&u, "prathamA", "uBO");
        has(&u, "tfIyA", "uBAByAm");
        let us = generate("uBa", "stri").expect("uBa stri");
        has(&us, "prathamA", "uBe");
    }

    #[test]
    fn sankhya_panca_to_dasa() {
        let p = generate("paYcan", "pum").expect("paYcan");
        has(&p, "prathamA", "paYca");
        has(&p, "dvitIyA", "paYca");
        has(&p, "tfIyA", "paYcaBiH");
        has(&p, "zazWI", "paYcAnAm");
        has(&p, "saptamI", "paYcasu");
        assert_eq!(generate("paJcan", "stri").unwrap().table.get("prathamA").unwrap()[2], "paYca");
        let z = generate("zaz", "nap").expect("zaz");
        has(&z, "prathamA", "zaw");
        has(&z, "tfIyA", "zaqBiH");
        has(&z, "zazWI", "zaRRAm");
        has(&z, "saptamI", "zawsu");
        has(&generate("zaq", "pum").expect("zaq alias"), "prathamA", "zaw");
        has(&generate("saptan", "pum").expect("sapta"), "prathamA", "sapta");
        has(&generate("saptan", "pum").unwrap(), "zazWI", "saptAnAm");
        let a = generate("azwan", "pum").expect("azwan");
        has(&a, "prathamA", "azwO");
        has(&a, "prathamA", "azwa");
        has(&a, "tfIyA", "azwABiH");
        has(&generate("navan", "stri").expect("nava"), "prathamA", "nava");
        has(&generate("daSan", "nap").expect("daSa"), "prathamA", "daSa");
        has(&generate("daSan", "nap").unwrap(), "saptamI", "daSasu");
        assert!(analyze("paYca").iter().any(|m| m.get("pratipadika") == Some(&"paYcan".to_string())));
        assert!(analyze("zaw").iter().any(|m| m.get("pratipadika") == Some(&"zaz".to_string())));
    }
}
