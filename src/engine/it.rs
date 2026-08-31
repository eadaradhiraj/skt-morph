//! सेट् / अनिट् for आर्धधातुक (स्य, सिच्, तव्य, …) as in the Kaumudī.
//! 7.2.10 एकाच उपदेशेऽनुदात्तात्; 7.2.35 आर्धधातुकस्येड् वलादेः; 7.2.58 गमेरिट्.
#![allow(non_snake_case)]

use crate::engine::join::internal_sandhi;
use crate::engine::phonology::apply_guna_to_stem;

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

/// अनिट् before स्य (लृट्). कृ / गम् / भू are सेट् here (करिष्यति, गमिष्यति, भविष्यति).
pub fn anit_sya(root: &str) -> bool {
    matches!(
        root,
        "pac" | "vac" | "yaj" | "vap" | "vah" | "vas" | "tyaj" | "dah" | "nI" | "Sru" | "dA"
            | "DA" | "sTA" | "pA" | "i" | "ad" | "han" | "diS" | "duh" | "lih" | "gA" | "hu"
            | "mA" | "yA" | "as" | "vid" | "pad" | "sic" | "vis" | "mfj" | "yuj" | "Baj"
            | "raYj" | "saYj" | "kfz" | "dfS" | "sfj" | "masj" | "majj" | "ruh" | "guh" | "nah"
            | "vasc" | "vraSc" | "Ced" | "Cid" | "vft" | "syand" | "kfp" | "kalp"
            | "dviz" | "dih"
    )
}

/// अनिट् सिच् (परस्मै). कृ अकार्षीत्; भू is लुक्, not this list.
pub fn anit_sic(root: &str) -> bool {
    matches!(
        root,
        "kf" | "nI" | "Sru" | "hu" | "i" | "vac" | "yaj" | "pac" | "tyaj" | "dah" | "han" | "ad"
            | "dA" | "DA" | "sTA" | "pA" | "gA" | "diS" | "duh" | "lih"
    )
}

/// तव्य / अनीयर् / तृच्: कर्तव्य, गन्तव्य; सेट् is भवितव्य.
pub fn anit_tavya(root: &str) -> bool {
    matches!(root, "kf" | "gam" | "han") || (anit_sya(root) && !matches!(root, "grah"))
}

pub fn takes_it_sya(root: &str) -> bool {
    matches!(root, "gam" | "kf" | "BU" | "pat" | "grah" | "eD") || !anit_sya(root)
}

pub fn takes_it_tavya(root: &str) -> bool {
    !anit_tavya(root)
}

/// सिचि वृद्धिः: i/ī → ai, u/ū → au, ṛ → ār, a → ā.
pub fn sic_vrddhi_grade(root: &str) -> String {
    let chars: Vec<char> = root.chars().collect();
    for idx in (0..chars.len()).rev() {
        let repl = match chars[idx] {
            'i' | 'I' | 'e' => Some("E"),
            'u' | 'U' | 'o' => Some("O"),
            'f' | 'F' => Some("Ar"),
            'a' => Some("A"),
            _ => None,
        };
        if let Some(r) = repl {
            let mut o = String::new();
            for &c in &chars[..idx] {
                o.push(c);
            }
            o.push_str(r);
            for &c in &chars[idx + 1..] {
                o.push(c);
            }
            return o;
        }
    }
    root.to_string()
}

/// रुक्: s → ṣ after i/u/ṛ/e/o/r.
pub fn ruki_s(stem: &str) -> String {
    if !stem.ends_with('s') {
        return stem.to_string();
    }
    let prev = stem.chars().rev().nth(1);
    if prev.is_some_and(|c| matches!(c, 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'e' | 'o' | 'E' | 'O' | 'r' | 'k')) {
        let mut s = stem.to_string();
        s.pop();
        s.push('z');
        return s;
    }
    stem.to_string()
}

fn last_vowel_index(s: &str) -> Option<usize> {
    s.char_indices()
        .rev()
        .find(|(_, c)| is_vowel(*c))
        .map(|(i, _)| i)
}

/// लृट् स्य stem (without the final a of thematic ति). गमिष्य, पक्ष्य, स्थास्य.
pub fn sya_stem(root: &str) -> String {
    match root {
        "gam" | "gamx" => return "gamizya".into(),
        "kf" => return "karizya".into(),
        "BU" => return "Bavizya".into(),
        "sTA" | "zWA" | "tizW" => return "sTAsya".into(),
        "dA" | "yacC" => return "dAsya".into(),
        "pA" | "pib" => return "pAsya".into(),
        "DA" | "Day" => return "DAsya".into(),
        "dfS" | "paSy" => return "drakzya".into(),
        "grah" => return "grahIzya".into(),
        "ad" => return "atsya".into(),
        "as" => return "Bavizya".into(),
        "han" => return "hanizya".into(),
        "i" => return "ezya".into(),
        "nI" => return "nezya".into(),
        "Sru" => return "Srozya".into(),
        "vac" => return "vakzya".into(),
        "pac" => return "pakzya".into(),
        "yaj" => return "yakzya".into(),
        "vft" => return "vartsya".into(),
        "syand" => return "syantsya".into(),
        "kfp" | "kalp" => return "kalpsya".into(),
        _ => {}
    }
    if takes_it_sya(root) {
        let g = apply_guna_to_stem(root);
        if g.ends_with('a') {
            format!("{}izya", &g[..g.len() - 1])
        } else {
            format!("{g}izya")
        }
    } else {
        let g = apply_guna_to_stem(root);
        let joined = internal_sandhi(&g, "sya");
        if joined.ends_with("sya") || joined.ends_with("zya") || joined.ends_with("kzya") {
            joined
        } else {
            format!("{g}sya")
        }
    }
}

/// तव्य aṅga + suffix.
pub fn tavya_form(root: &str) -> String {
    match root {
        "kf" => "kartavya".into(),
        "gam" => "gantavya".into(),
        "nI" => "netavya".into(),
        "dA" => "dAtavya".into(),
        "BU" => "Bavitavya".into(),
        "han" => "hantavya".into(),
        "vac" => "vaktavya".into(),
        "sTA" => "sTAtavya".into(),
        _ if takes_it_tavya(root) => {
            let g = apply_guna_to_stem(root);
            format!("{g}itavya")
        }
        _ => {
            let g = apply_guna_to_stem(root);
            internal_sandhi(&g, "tavya")
        }
    }
}

pub fn anIya_form(root: &str) -> String {
    match root {
        "kf" => "karaRIya".into(),
        "gam" => "gamanIya".into(),
        "nI" => "nayanIya".into(),
        "dA" => "dAnIya".into(),
        "BU" => "BavanIya".into(),
        "han" => "GAtanIya".into(),
        _ => format!("{}anIya", apply_guna_to_stem(root)),
    }
}

pub fn lyuw_form(root: &str) -> String {
    match root {
        "kf" => "karaRa".into(),
        "gam" => "gamana".into(),
        "nI" => "nayana".into(),
        "dA" => "dAna".into(),
        "BU" => "Bavana".into(),
        _ => format!("{}ana", apply_guna_to_stem(root)),
    }
}

pub fn tfc_form(root: &str) -> String {
    match root {
        "kf" => "kartf".into(),
        "nI" => "netf".into(),
        "dA" => "dAtf".into(),
        "BU" => "Bavitf".into(),
        "gam" => "gantf".into(),
        "han" => "hantf".into(),
        "vac" => "vaktf".into(),
        _ if takes_it_tavya(root) => format!("{}itf", apply_guna_to_stem(root)),
        _ => {
            let g = apply_guna_to_stem(root);
            internal_sandhi(&g, "tf")
        }
    }
}

/// सिच् parasmai body (before ईत्): कार्ष, नैष्, अत्स्.
pub fn sic_p_body(root: &str) -> String {
    if anit_sic(root) {
        let v = sic_vrddhi_grade(root);
        ruki_s(&internal_sandhi(&v, "s"))
    } else {
        let g = apply_guna_to_stem(root);
        format!("{g}iz")
    }
}

pub fn surface_root(dhatu: &str) -> String {
    match crate::engine::lit::prakriya_root(dhatu).as_str() {
        "RI" => "nI".into(),
        "brU" => "vac".into(),
        "zWA" => "sTA".into(),
        other => other.to_string(),
    }
}

/// Unused helper kept for tests of vowel scan.
#[allow(dead_code)]
fn has_vowel(s: &str) -> bool {
    last_vowel_index(s).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sya_gam_kf_pac() {
        assert_eq!(sya_stem("gam"), "gamizya");
        assert_eq!(sya_stem("kf"), "karizya");
        assert_eq!(sya_stem("pac"), "pakzya");
        assert_eq!(sya_stem("sTA"), "sTAsya");
        assert_eq!(sya_stem("vft"), "vartsya");
        assert_eq!(sya_stem("syand"), "syantsya");
        assert_eq!(sya_stem("kfp"), "kalpsya");
    }

    #[test]
    fn tavya_kf_bu() {
        assert_eq!(tavya_form("kf"), "kartavya");
        assert_eq!(tavya_form("BU"), "Bavitavya");
        assert_eq!(tfc_form("kf"), "kartf");
    }
}
