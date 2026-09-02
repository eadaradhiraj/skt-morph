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
        assert_eq!(derive("rAma", "Da"), vec!["rAmaDa"]);
        assert_eq!(derive("rAma", "na"), vec!["rAmana"]);
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
