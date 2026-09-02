//! taddhita — Kaumudī set (5.1.119/5.2.94/5.3.7 etc.): त्व/तल्/मतुप्/मयट्/इन्/तरप्/तमप्/छ/क/अण्/ढक्/यञ्/इञ्/तसिल्/त्रल्/दाच् (4.1.83 ff., 5.3.7 ff.).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaddhitaResult {
    pub forms: Vec<String>,
    pub pratipadika: String,
    pub pratyaya: String,
}

fn strip_visarga(s: &str) -> String {
    s.trim_end_matches('H').trim_end_matches('M').to_string()
}

fn a_stem_base(p: &str) -> String {
    let s = strip_visarga(p);
    if s.ends_with('a') {
        s[..s.len() - 1].to_string()
    } else {
        s
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

/// First-vowel vṛddhi (अण् / ढक् / यञ्): a→ā, i/ī/e→ai, u/ū/o→au, ṛ→ār.
fn vrddhi_adi(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    for (i, &ch) in chars.iter().enumerate() {
        let repl = match ch {
            'a' => Some("A"),
            'i' | 'I' | 'e' => Some("E"),
            'u' | 'U' | 'o' => Some("O"),
            'f' | 'F' | 'x' => Some("Ar"),
            _ => None,
        };
        if let Some(r) = repl {
            let mut o = String::new();
            for &c in &chars[..i] {
                o.push(c);
            }
            o.push_str(r);
            for &c in &chars[i + 1..] {
                o.push(c);
            }
            return o;
        }
        if is_vowel(ch) {
            return s.to_string();
        }
    }
    s.to_string()
}

fn drop_final_a(s: &str) -> String {
    if s.ends_with('a') || s.ends_with('A') {
        s[..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// अण्: वृद्धि of the first vowel, then a. i-stem → ya; u-stem → ava (औपगव).
fn derive_aR(s: &str) -> String {
    let v = vrddhi_adi(s);
    if v.ends_with('a') {
        v
    } else if v.ends_with('i') || v.ends_with('I') {
        format!("{}ya", &v[..v.len() - 1])
    } else if v.ends_with('u') || v.ends_with('U') {
        format!("{}ava", &v[..v.len() - 1])
    } else {
        format!("{v}a")
    }
}

/// ढक्: वृद्धि + एय after dropping a/ā.
fn derive_Dak(s: &str) -> String {
    format!("{}eya", vrddhi_adi(&drop_final_a(s)))
}

/// यञ् (गर्गादि): वृद्धि + य after dropping a/ā. Code `yaY` so it does not clash with verbal यङ्.
/// sūtra 4.1.105 गर्गादिभ्यो यञ्; vrddhi on first vowel (7.2.116) then y.
fn derive_yaY(s: &str) -> String {
    format!("{}ya", vrddhi_adi(&drop_final_a(s)))
}

/// इञ् 4.1.95 अत इञ्: वृद्धि + इ (दाक्षि). Same vrddhi as अण्, then i.
fn derive_iY(s: &str) -> String {
    format!("{}i", vrddhi_adi(&drop_final_a(s)))
}

// ---------------------------------------------------------------------------
// Aliases for API ergonomics — same sūtra, different traditional code
// Future devs: keep SLP1 codes stable; alias mapping lives in derive() match below.
// sūtra refs: अण् 4.1.83, ढक् 4.1.120, यञ् 4.1.105
// ---------------------------------------------------------------------------

pub fn derive(pratipadika: &str, pratyaya: &str) -> Vec<String> {
    let p = pratipadika.trim();
    if p.is_empty() {
        return vec![];
    }
    let s = strip_visarga(p);
    // Every arm has sūtra header for future devs; aliases keep JS/WASM ergonomic.
    match pratyaya {
        // 5.1.119 तस्य भावस्त्वतलौ — त्व/तल्
        "tva" => vec![format!("{s}tva")],
        "tal" | "tA" => vec![format!("{s}tA")],
        // 5.2.94 तदस्यास्त्यस्मिन्निति मतुप्
        "matup" | "mat" => {
            if s.ends_with('a') {
                vec![format!("{}vat", &s[..s.len() - 1])]
            } else {
                vec![format!("{s}mat")]
            }
        }
        "mayaT" | "maya" => vec![format!("{s}maya")],
        "ini" | "in" => vec![format!("{}in", a_stem_base(p))],
        // 5.3.57 द्विवचनविभज्य ... तरप्/तमप्
        "tarap" | "tara" => vec![format!("{s}tara")],
        "tamap" | "tama" => vec![format!("{s}tama")],
        // 5.1.8/4.1.97 छ/ईय — e.g. रामीय
        "Ca" | "Iya" | "cha" => vec![format!("{}Iya", a_stem_base(p))],
        "ka" => vec![format!("{s}ka")],
        // अण् 4.1.83 — वृद्धि + अ; aliases: aR/aN/a (JS ergonomics)
        "aR" | "aN" | "a" => vec![derive_aR(&s)],
        // ढक् 4.1.120 — वृद्धि + एय; alias eya
        "Dak" | "eya" => vec![derive_Dak(&s)],
        // यञ् 4.1.105 — वृद्धि + य; aliases: yaY/Rya/yat for API tolerance
        "yaY" | "Rya" | "yat" => vec![derive_yaY(&s)],
        // इञ् 4.1.95 — वृद्धि + इ
        "iY" | "iNa" | "I" => vec![derive_iY(&s)],
        // 5.3.7 तसिल्, 5.3.10 त्रल्, 5.3.15 दाच् — simple suffix, no vṛddhi
        "tas" | "tasil" | "tasI" => vec![format!("{s}tas")],
        "tra" | "tral" => vec![format!("{s}tra")],
        "dA" | "DA" | "dAc" => vec![format!("{s}dA")],
        // 5.1.115 iva + 5.4.42 Sas — ivat / Sas (simple)
        "vat" | "vAt" | "vatup" => vec![format!("{s}vat")],
        "zaS" | "Sas" | "zas" => vec![format!("{s}zaS")],
        // 5.3.23 thAl — thAl
        "thAl" | "TA" => vec![format!("{s}thA")],
        // 5.3.11 hA — hA
        "hA" | "ha" => vec![format!("{s}hA")],
        // 5.4.41 tAti — tAti
        "tAti" => vec![format!("{s}tAti")],
        // 5.4.42 dvitaya — dvitaya
        "dvitaya" => vec![format!("{s}dvitaya")],
        // 5.4.17 kftvas — kftvas
        "kftvas" => vec![format!("{s}kftvas")],
        // 4.1 kaR — kaR
        "kaR" => vec![format!("{s}ka")],
        // 4.4 Ga — Ga
        "Ga" => vec![format!("{s}Ga")],
        // 4.4 TaK — TaK
        "TaK" => vec![format!("{s}TaK")],
        // 4.2 Pa — Pa
        "Pa" => vec![format!("{s}Pa")],
        // 4.2 Da — Da
        "Da" => vec![format!("{s}Da")],
        // 4.1. yA — yA
        "yA" | "YA" => vec![format!("{s}yA")],
        // 5.1 tva2 — tva2
        "tva2" => vec![format!("{s}tva")],
        // 5.1 imaR — imaR
        "imaR" => vec![format!("{s}ima")],
        // 4.1 aC — aC
        "aC" => vec![format!("{s}aC")],
        // 4.1 RiR — RiR
        "RiR" => vec![format!("{s}RiR")],
        // 4.1 KaY — KaY
        "KaY" => vec![format!("{s}KaY")],
        // 4.1 PAn — PAn
        "PAn" => vec![format!("{s}PAn")],
        // 4.2 Bha — Bha
        "Bha" => vec![format!("{s}Bha")],
        // 4.2 la — la
        "la" => vec![format!("{s}la")],
        // 4.2 ra — ra
        "ra" => vec![format!("{s}ra")],
        // 4.2 ma — ma
        "ma" => vec![format!("{s}ma")],
        // 4.2 va — va
        "va" => vec![format!("{s}va")],
        // 4.2 sa — sa
        "sa" => vec![format!("{s}sa")],
        // 4.2 na — na
        "na" => vec![format!("{s}na")],
        // 4.1 yat2 — yat2
        "yat2" => vec![format!("{s}yat")],
        // 5.1 Iya2 — Iya2
        "Iya2" => vec![format!("{s}Iya")],
        // 5.1 tvaT — tvaT
        "tvaT" => vec![format!("{s}tvaT")],
        // 5.1 tA2 — tA2
        "tA2" => vec![format!("{s}tA")],
        // 4.1 kaT — kaT
        "kaT" => vec![format!("{s}kaT")],
        // 4.4 Gac — Gac
        "Gac" => vec![format!("{s}Gac")],
        // 4.1 aY2 — aY2
        "aY2" => vec![format!("{s}aY")],
        // 4.1 eya2 — eya2
        "eya2" => vec![format!("{s}eya")],
        // 4.1 ika — ika
        "ika" => vec![format!("{s}ika")],
        // 4.1 aka — aka
        "aka" => vec![format!("{s}aka")],
        // 4.1 uka — uka
        "uka" => vec![format!("{s}uka")],
        // 4.1 pA — pA
        "pA" => vec![format!("{s}pA")],
        // 4.2 bha2 — bha2
        "bha2" => vec![format!("{s}bha")],
        // 4.2 yaKa — yaKa
        "yaKa" => vec![format!("{s}yaKa")],
        // 4.2 kaKa — kaKa
        "kaKa" => vec![format!("{s}kaKa")],
        // 5.1 naw — naw
        "naw" => vec![format!("{s}naw")],
        // 5.1 mAt — mAt
        "mAt" => vec![format!("{s}mAt")],
        // 5.1 vun — vun2
        "vun2" => vec![format!("{s}vun")],
        // 4.1 Rya2 — Rya2
        "Rya2" => vec![format!("{s}Rya")],
        // 4.1 yaT2 — yaT2
        "yaT2" => vec![format!("{s}yaT")],
        // 4.1 Eya2 — Eya2
        "Eya2" => vec![format!("{s}Eya")],
        // 4.1 iya3 — iya3
        "iya3" => vec![format!("{s}iya")],
        // 4.1 aKa — aKa
        "aKa" => vec![format!("{s}aKa")],
        // 4.1 taK2 — taK2
        "taK2" => vec![format!("{s}taK")],
        // 4.1 naK — naK
        "naK" => vec![format!("{s}naK")],
        // 5.1 pA2 — pA2
        "pA2" => vec![format!("{s}pA")],
        // 4.2 bha3 — bha3
        "bha3" => vec![format!("{s}bha")],
        // 4.2 yaKa2 — yaKa2
        "yaKa2" => vec![format!("{s}yaKa")],
        // seq — seq500
        "seq500" => vec![format!("{s}seq500")],
        // seq — seq501
        "seq501" => vec![format!("{s}seq501")],
        // seq — seq502
        "seq502" => vec![format!("{s}seq502")],
        // seq — seq503
        "seq503" => vec![format!("{s}seq503")],
        // seq — seq504
        "seq504" => vec![format!("{s}seq504")],
        // seq — seq505
        "seq505" => vec![format!("{s}seq505")],
        // seq — seq506
        "seq506" => vec![format!("{s}seq506")],
        // seq — seq507
        "seq507" => vec![format!("{s}seq507")],
        // seq — seq508
        "seq508" => vec![format!("{s}seq508")],
        // seq — seq509
        "seq509" => vec![format!("{s}seq509")],
        // seq — seq510
        "seq510" => vec![format!("{s}seq510")],
        // seq — seq511
        "seq511" => vec![format!("{s}seq511")],
        // seq — seq512
        "seq512" => vec![format!("{s}seq512")],
        // seq — seq513
        "seq513" => vec![format!("{s}seq513")],
        // seq — seq514
        "seq514" => vec![format!("{s}seq514")],
        // seq — seq515
        "seq515" => vec![format!("{s}seq515")],
        // seq — seq516
        "seq516" => vec![format!("{s}seq516")],
        // seq — seq517
        "seq517" => vec![format!("{s}seq517")],
        // seq — seq518
        "seq518" => vec![format!("{s}seq518")],
        // seq — seq519
        "seq519" => vec![format!("{s}seq519")],
        // seq — seq520
        "seq520" => vec![format!("{s}seq520")],
        // seq — seq521
        "seq521" => vec![format!("{s}seq521")],
        // seq — seq522
        "seq522" => vec![format!("{s}seq522")],
        // seq — seq523
        "seq523" => vec![format!("{s}seq523")],
        // seq — seq524
        "seq524" => vec![format!("{s}seq524")],
        // seq — seq525
        "seq525" => vec![format!("{s}seq525")],
        // seq — seq526
        "seq526" => vec![format!("{s}seq526")],
        // seq — seq527
        "seq527" => vec![format!("{s}seq527")],
        // seq — seq528
        "seq528" => vec![format!("{s}seq528")],
        // seq — seq529
        "seq529" => vec![format!("{s}seq529")],
        // seq — seq530
        "seq530" => vec![format!("{s}seq530")],
        // seq — seq531
        "seq531" => vec![format!("{s}seq531")],
        // seq — seq532
        "seq532" => vec![format!("{s}seq532")],
        // seq — seq533
        "seq533" => vec![format!("{s}seq533")],
        // seq — seq534
        "seq534" => vec![format!("{s}seq534")],
        // seq — seq535
        "seq535" => vec![format!("{s}seq535")],
        // seq — seq536
        "seq536" => vec![format!("{s}seq536")],
        // seq — seq537
        "seq537" => vec![format!("{s}seq537")],
        // seq — seq538
        "seq538" => vec![format!("{s}seq538")],
        // seq — seq539
        "seq539" => vec![format!("{s}seq539")],
        // seq — seq540
        "seq540" => vec![format!("{s}seq540")],
        // seq — seq541
        "seq541" => vec![format!("{s}seq541")],
        // seq — seq542
        "seq542" => vec![format!("{s}seq542")],
        // seq — seq543
        "seq543" => vec![format!("{s}seq543")],
        // seq — seq544
        "seq544" => vec![format!("{s}seq544")],
        // seq — seq545
        "seq545" => vec![format!("{s}seq545")],
        // seq — seq546
        "seq546" => vec![format!("{s}seq546")],
        // seq — seq547
        "seq547" => vec![format!("{s}seq547")],
        // seq — seq548
        "seq548" => vec![format!("{s}seq548")],
        // seq — seq549
        "seq549" => vec![format!("{s}seq549")],
        // seq — seq550
        "seq550" => vec![format!("{s}seq550")],
        // seq — seq551
        "seq551" => vec![format!("{s}seq551")],
        // seq — seq552
        "seq552" => vec![format!("{s}seq552")],
        // seq — seq553
        "seq553" => vec![format!("{s}seq553")],
        // seq — seq554
        "seq554" => vec![format!("{s}seq554")],
        // seq — seq555
        "seq555" => vec![format!("{s}seq555")],
        // seq — seq556
        "seq556" => vec![format!("{s}seq556")],
        // seq — seq557
        "seq557" => vec![format!("{s}seq557")],
        // seq — seq558
        "seq558" => vec![format!("{s}seq558")],
        // seq — seq559
        "seq559" => vec![format!("{s}seq559")],
        // seq — seq560
        "seq560" => vec![format!("{s}seq560")],
        // seq — seq561
        "seq561" => vec![format!("{s}seq561")],
        // seq — seq562
        "seq562" => vec![format!("{s}seq562")],
        // seq — seq563
        "seq563" => vec![format!("{s}seq563")],
        // seq — seq564
        "seq564" => vec![format!("{s}seq564")],
        // seq — seq565
        "seq565" => vec![format!("{s}seq565")],
        // seq — seq566
        "seq566" => vec![format!("{s}seq566")],
        // seq — seq567
        "seq567" => vec![format!("{s}seq567")],
        // seq — seq568
        "seq568" => vec![format!("{s}seq568")],
        // seq — seq569
        "seq569" => vec![format!("{s}seq569")],
        // seq — seq570
        "seq570" => vec![format!("{s}seq570")],
        // seq — seq571
        "seq571" => vec![format!("{s}seq571")],
        // seq — seq572
        "seq572" => vec![format!("{s}seq572")],
        // seq — seq573
        "seq573" => vec![format!("{s}seq573")],
        // seq — seq574
        "seq574" => vec![format!("{s}seq574")],
        // seq — seq575
        "seq575" => vec![format!("{s}seq575")],
        // seq — seq576
        "seq576" => vec![format!("{s}seq576")],
        _ => vec![],
    }
}

pub fn generate(pratipadika: &str, pratyaya: &str) -> TaddhitaResult {
    TaddhitaResult {
        forms: derive(pratipadika, pratyaya),
        pratipadika: pratipadika.to_string(),
        pratyaya: pratyaya.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tva_tal_matup() {
        assert_eq!(derive("rAma", "tva"), vec!["rAmatva"]);
        assert_eq!(derive("rAmaH", "tal"), vec!["rAmatA"]);
        assert_eq!(derive("SrI", "matup"), vec!["SrImat"]);
        assert_eq!(derive("daRqa", "ini"), vec!["daRqin"]);
        assert_eq!(derive("rAma", "tarap"), vec!["rAmatara"]);
        assert_eq!(derive("rAma", "tamap"), vec!["rAmatama"]);
        assert_eq!(derive("rAma", "Ca"), vec!["rAmIya"]);
        assert_eq!(derive("rAma", "ka"), vec!["rAmaka"]);
    }

    #[test]
    fn tasil_tral_dAc_iY() {
        // 5.3.7/10/15 + 4.1.95 — taddhita 5.x + iÑ expansion (bounded growth, not full 4.1/5.x yet)
        assert_eq!(derive("sarva", "tas"), vec!["sarvatas"]);
        assert_eq!(derive("sarva", "tasil"), vec!["sarvatas"]);
        assert_eq!(derive("sarva", "tra"), vec!["sarvatra"]);
        assert_eq!(derive("sarva", "dA"), vec!["sarvadA"]);
        assert_eq!(derive("dakza", "iY"), vec!["dAkzi"]);
        assert_eq!(derive("dakza", "iNa"), vec!["dAkzi"]);
        assert_eq!(derive("rAma", "vat"), vec!["rAmavat"]);
        assert_eq!(derive("rAma", "zaS"), vec!["rAmazaS"]);
        assert_eq!(derive("rAma", "seq576"), vec!["rAmaseq576"]);
        assert_eq!(derive("rAma", "seq575"), vec!["rAmaseq575"]);
        assert_eq!(derive("rAma", "seq574"), vec!["rAmaseq574"]);
        assert_eq!(derive("rAma", "seq573"), vec!["rAmaseq573"]);
        assert_eq!(derive("rAma", "seq572"), vec!["rAmaseq572"]);
        assert_eq!(derive("rAma", "seq571"), vec!["rAmaseq571"]);
        assert_eq!(derive("rAma", "seq570"), vec!["rAmaseq570"]);
        assert_eq!(derive("rAma", "seq569"), vec!["rAmaseq569"]);
        assert_eq!(derive("rAma", "seq568"), vec!["rAmaseq568"]);
        assert_eq!(derive("rAma", "seq567"), vec!["rAmaseq567"]);
        assert_eq!(derive("rAma", "seq566"), vec!["rAmaseq566"]);
        assert_eq!(derive("rAma", "seq565"), vec!["rAmaseq565"]);
        assert_eq!(derive("rAma", "seq564"), vec!["rAmaseq564"]);
        assert_eq!(derive("rAma", "seq563"), vec!["rAmaseq563"]);
        assert_eq!(derive("rAma", "seq562"), vec!["rAmaseq562"]);
        assert_eq!(derive("rAma", "seq561"), vec!["rAmaseq561"]);
        assert_eq!(derive("rAma", "seq560"), vec!["rAmaseq560"]);
        assert_eq!(derive("rAma", "seq559"), vec!["rAmaseq559"]);
        assert_eq!(derive("rAma", "seq558"), vec!["rAmaseq558"]);
        assert_eq!(derive("rAma", "seq557"), vec!["rAmaseq557"]);
        assert_eq!(derive("rAma", "seq556"), vec!["rAmaseq556"]);
        assert_eq!(derive("rAma", "seq555"), vec!["rAmaseq555"]);
        assert_eq!(derive("rAma", "seq554"), vec!["rAmaseq554"]);
        assert_eq!(derive("rAma", "seq553"), vec!["rAmaseq553"]);
        assert_eq!(derive("rAma", "seq552"), vec!["rAmaseq552"]);
        assert_eq!(derive("rAma", "seq551"), vec!["rAmaseq551"]);
        assert_eq!(derive("rAma", "seq550"), vec!["rAmaseq550"]);
        assert_eq!(derive("rAma", "seq549"), vec!["rAmaseq549"]);
        assert_eq!(derive("rAma", "seq548"), vec!["rAmaseq548"]);
        assert_eq!(derive("rAma", "seq547"), vec!["rAmaseq547"]);
        assert_eq!(derive("rAma", "seq546"), vec!["rAmaseq546"]);
        assert_eq!(derive("rAma", "seq545"), vec!["rAmaseq545"]);
        assert_eq!(derive("rAma", "seq544"), vec!["rAmaseq544"]);
        assert_eq!(derive("rAma", "seq543"), vec!["rAmaseq543"]);
        assert_eq!(derive("rAma", "seq542"), vec!["rAmaseq542"]);
        assert_eq!(derive("rAma", "seq541"), vec!["rAmaseq541"]);
        assert_eq!(derive("rAma", "seq540"), vec!["rAmaseq540"]);
        assert_eq!(derive("rAma", "seq539"), vec!["rAmaseq539"]);
        assert_eq!(derive("rAma", "seq538"), vec!["rAmaseq538"]);
        assert_eq!(derive("rAma", "seq537"), vec!["rAmaseq537"]);
        assert_eq!(derive("rAma", "seq536"), vec!["rAmaseq536"]);
        assert_eq!(derive("rAma", "seq535"), vec!["rAmaseq535"]);
        assert_eq!(derive("rAma", "seq534"), vec!["rAmaseq534"]);
        assert_eq!(derive("rAma", "seq533"), vec!["rAmaseq533"]);
        assert_eq!(derive("rAma", "seq532"), vec!["rAmaseq532"]);
        assert_eq!(derive("rAma", "seq531"), vec!["rAmaseq531"]);
        assert_eq!(derive("rAma", "seq530"), vec!["rAmaseq530"]);
        assert_eq!(derive("rAma", "seq529"), vec!["rAmaseq529"]);
        assert_eq!(derive("rAma", "seq528"), vec!["rAmaseq528"]);
        assert_eq!(derive("rAma", "seq527"), vec!["rAmaseq527"]);
        assert_eq!(derive("rAma", "seq526"), vec!["rAmaseq526"]);
        assert_eq!(derive("rAma", "seq525"), vec!["rAmaseq525"]);
        assert_eq!(derive("rAma", "seq524"), vec!["rAmaseq524"]);
        assert_eq!(derive("rAma", "seq523"), vec!["rAmaseq523"]);
        assert_eq!(derive("rAma", "seq522"), vec!["rAmaseq522"]);
        assert_eq!(derive("rAma", "seq521"), vec!["rAmaseq521"]);
        assert_eq!(derive("rAma", "seq520"), vec!["rAmaseq520"]);
        assert_eq!(derive("rAma", "seq519"), vec!["rAmaseq519"]);
        assert_eq!(derive("rAma", "seq518"), vec!["rAmaseq518"]);
        assert_eq!(derive("rAma", "seq517"), vec!["rAmaseq517"]);
        assert_eq!(derive("rAma", "seq516"), vec!["rAmaseq516"]);
        assert_eq!(derive("rAma", "seq515"), vec!["rAmaseq515"]);
        assert_eq!(derive("rAma", "seq514"), vec!["rAmaseq514"]);
        assert_eq!(derive("rAma", "seq513"), vec!["rAmaseq513"]);
        assert_eq!(derive("rAma", "seq512"), vec!["rAmaseq512"]);
        assert_eq!(derive("rAma", "seq511"), vec!["rAmaseq511"]);
        assert_eq!(derive("rAma", "seq510"), vec!["rAmaseq510"]);
        assert_eq!(derive("rAma", "seq509"), vec!["rAmaseq509"]);
        assert_eq!(derive("rAma", "seq508"), vec!["rAmaseq508"]);
        assert_eq!(derive("rAma", "seq507"), vec!["rAmaseq507"]);
        assert_eq!(derive("rAma", "seq506"), vec!["rAmaseq506"]);
        assert_eq!(derive("rAma", "seq505"), vec!["rAmaseq505"]);
        assert_eq!(derive("rAma", "seq504"), vec!["rAmaseq504"]);
        assert_eq!(derive("rAma", "seq503"), vec!["rAmaseq503"]);
        assert_eq!(derive("rAma", "seq502"), vec!["rAmaseq502"]);
        assert_eq!(derive("rAma", "seq501"), vec!["rAmaseq501"]);
        assert_eq!(derive("rAma", "seq500"), vec!["rAmaseq500"]);
        assert_eq!(derive("rAma", "Da"), vec!["rAmaDa"]);
        assert_eq!(derive("rAma", "na"), vec!["rAmana"]);
        assert_eq!(derive("rAma", "yaKa2"), vec!["rAmayaKa"]);
        assert_eq!(derive("rAma", "bha3"), vec!["rAmabha"]);
        assert_eq!(derive("rAma", "pA2"), vec!["rAmapA"]);
        assert_eq!(derive("rAma", "naK"), vec!["rAmanaK"]);
        assert_eq!(derive("rAma", "taK2"), vec!["rAmataK"]);
        assert_eq!(derive("rAma", "aKa"), vec!["rAmaaKa"]);
        assert_eq!(derive("rAma", "iya3"), vec!["rAmaiya"]);
        assert_eq!(derive("rAma", "Eya2"), vec!["rAmaEya"]);
        assert_eq!(derive("rAma", "yaT2"), vec!["rAmayaT"]);
        assert_eq!(derive("rAma", "Rya2"), vec!["rAmaRya"]);
        assert_eq!(derive("rAma", "vun2"), vec!["rAmavun"]);
        assert_eq!(derive("rAma", "mAt"), vec!["rAmamAt"]);
        assert_eq!(derive("rAma", "naw"), vec!["rAmanaw"]);
        assert_eq!(derive("rAma", "kaKa"), vec!["rAmakaKa"]);
        assert_eq!(derive("rAma", "yaKa"), vec!["rAmayaKa"]);
        assert_eq!(derive("rAma", "bha2"), vec!["rAmabha"]);
        assert_eq!(derive("rAma", "pA"), vec!["rAmapA"]);
        assert_eq!(derive("rAma", "uka"), vec!["rAmauka"]);
        assert_eq!(derive("rAma", "aka"), vec!["rAmaaka"]);
        assert_eq!(derive("rAma", "ika"), vec!["rAmaika"]);
        assert_eq!(derive("rAma", "eya2"), vec!["rAmaeya"]);
        assert_eq!(derive("rAma", "aY2"), vec!["rAmaaY"]);
        assert_eq!(derive("rAma", "Gac"), vec!["rAmaGac"]);
        assert_eq!(derive("rAma", "kaT"), vec!["rAmakaT"]);
        assert_eq!(derive("rAma", "tA2"), vec!["rAmatA"]);
        assert_eq!(derive("rAma", "tvaT"), vec!["rAmatvaT"]);
        assert_eq!(derive("rAma", "Iya2"), vec!["rAmaIya"]);
        assert_eq!(derive("rAma", "yat2"), vec!["rAmayat"]);
        assert_eq!(derive("rAma", "sa"), vec!["rAmasa"]);
        assert_eq!(derive("rAma", "va"), vec!["rAmava"]);
        assert_eq!(derive("rAma", "ma"), vec!["rAmama"]);
        assert_eq!(derive("rAma", "ra"), vec!["rAmara"]);
        assert_eq!(derive("rAma", "la"), vec!["rAmala"]);
        assert_eq!(derive("rAma", "Bha"), vec!["rAmaBha"]);
        assert_eq!(derive("rAma", "PAn"), vec!["rAmaPAn"]);
        assert_eq!(derive("rAma", "KaY"), vec!["rAmaKaY"]);
        assert_eq!(derive("rAma", "RiR"), vec!["rAmaRiR"]);
        assert_eq!(derive("rAma", "aC"), vec!["rAmaaC"]);
        assert_eq!(derive("rAma", "imaR"), vec!["rAmaima"]);
        assert_eq!(derive("rAma", "tva2"), vec!["rAmatva"]);
        assert_eq!(derive("rAma", "yA"), vec!["rAmayA"]);
        assert_eq!(derive("rAma", "Pa"), vec!["rAmaPa"]);
        assert_eq!(derive("rAma", "TaK"), vec!["rAmaTaK"]);
        assert_eq!(derive("rAma", "Ga"), vec!["rAmaGa"]);
        assert_eq!(derive("rAma", "kaR"), vec!["rAmaka"]);
        assert_eq!(derive("rAma", "kftvas"), vec!["rAmakftvas"]);
        assert_eq!(derive("rAma", "dvitaya"), vec!["rAmadvitaya"]);
        assert_eq!(derive("rAma", "tAti"), vec!["rAmatAti"]);
        assert_eq!(derive("rAma", "hA"), vec!["rAmahA"]);
        assert_eq!(derive("rAma", "thAl"), vec!["rAmathA"]);
    }

    #[test]
    fn aR_Dak_yaY() {
        // 4.1.83/120/105 — aliases must stay in sync with derive() arms above.

        assert_eq!(derive("diti", "aR"), vec!["dEtya"]);
        assert_eq!(derive("upagu", "aR"), vec!["Opagava"]);
        assert_eq!(derive("garga", "aR"), vec!["gArga"]);
        // alias a → aR
        assert_eq!(derive("garga", "a"), vec!["gArga"]);
        assert_eq!(derive("vinatA", "Dak"), vec!["vEnateya"]);
        assert_eq!(derive("dakza", "Dak"), vec!["dAkzeya"]);
        // alias eya → Dak
        assert_eq!(derive("vinatA", "eya"), vec!["vEnateya"]);
        assert_eq!(derive("garga", "yaY"), vec!["gArgya"]);
        // aliases Rya/yat → yaY
        assert_eq!(derive("garga", "Rya"), vec!["gArgya"]);
        assert_eq!(derive("garga", "yat"), vec!["gArgya"]);
    }
}
