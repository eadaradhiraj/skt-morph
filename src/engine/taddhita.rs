//! taddhita — minimal Kaumudī set (5.1.119/5.2.94 etc.): त्व/तल्/मतुप्/मयट्/इन्/तरप्/तमप्/छ/क/अण्/ढक्/यञ् (4.1.83 ff.).

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
