//! सेट् / अनिट् for आर्धधातुक (स्य, सिच्, तव्य, …) as in the Kaumudī.
//! 7.2.10 एकाच उपदेशेऽनुदात्तात्; 7.2.35 आर्धधातुकस्येड् वलादेः; 7.2.58 गमेरिट्.
#![allow(non_snake_case)]

use crate::engine::join::internal_sandhi;
use crate::engine::phonology::{apply_guna_to_stem, apply_natva_to_word};

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

/// तव्य / तृच् / तुमुन्: कर्तव्य, गन्तव्य; सेट् is भवितव्य. (निष्ठा is a different list.)
pub fn anit_tavya(root: &str) -> bool {
    matches!(root, "kf" | "gam" | "han") || (anit_sya(root) && !matches!(root, "grah"))
}

/// निष्ठा (क्त) सेट्: पतित, उषित, गृहीत. 7.2.11 श्र्युकः किति blocks भूत/श्रुत/नीत.
pub fn takes_it_nistha(root: &str) -> bool {
    if matches!(root, "vas" | "grah" | "pat") {
        return true;
    }
    if matches!(
        root,
        "kf" | "gam" | "han" | "labh" | "laB" | "naS" | "banD" | "svap" | "zvap"
    ) {
        return false;
    }
    if anit_sya(root) {
        return false;
    }
    !root.chars().last().is_some_and(|c| "iIuUfFA".contains(c))
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

fn is_ac(c: char) -> bool {
    matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

/// 6.1.78 एचोऽयवायावः; 6.1.79 वान्तो यि प्रत्यये; 6.1.101 अकः सवर्णे दीर्घः.
pub fn join_eco(stem: &str, suffix: &str) -> String {
    let Some(s0) = suffix.chars().next() else {
        return stem.to_string();
    };
    let Some(last) = stem.chars().last() else {
        return suffix.to_string();
    };
    let body: String = stem.chars().take(stem.chars().count() - 1).collect();
    if s0 == 'y' {
        return match last {
            'o' => format!("{body}av{suffix}"),
            'O' => format!("{body}Av{suffix}"),
            _ => format!("{stem}{suffix}"),
        };
    }
    if !is_ac(s0) {
        return format!("{stem}{suffix}");
    }
    match last {
        'e' => format!("{body}ay{suffix}"),
        'o' => format!("{body}av{suffix}"),
        'E' => format!("{body}Ay{suffix}"),
        'O' => format!("{body}Av{suffix}"),
        'a' | 'A' if s0 == 'a' || s0 == 'A' => format!("{body}A{}", &suffix[s0.len_utf8()..]),
        _ => format!("{stem}{suffix}"),
    }
}

/// गुण + सेट् इट् (7.2.35) or अनिट् sandhi, then णत्व. तव्य / तृच् / तुमुन्.
pub fn guna_it_join(root: &str, suffix: &str) -> String {
    let g = apply_guna_to_stem(root);
    let raw = if takes_it_tavya(root) {
        format!("{g}i{suffix}")
    } else {
        internal_sandhi(&g, suffix)
    };
    apply_natva_to_word(&raw)
}

/// गुण + vowel-initial kṛt (ल्युट् अन, अनीयर्), 6.1.78/101 then णत्व.
pub fn guna_ac_suffix(root: &str, suffix: &str) -> String {
    apply_natva_to_word(&join_eco(&apply_guna_to_stem(root), suffix))
}

pub fn tavya_form(root: &str) -> String {
    guna_it_join(root, "tavya")
}

pub fn anIya_form(root: &str) -> String {
    guna_ac_suffix(root, "anIya")
}

pub fn lyuw_form(root: &str) -> String {
    guna_ac_suffix(root, "ana")
}

pub fn tfc_form(root: &str) -> String {
    guna_it_join(root, "tf")
}

pub fn tum_form(root: &str) -> String {
    guna_it_join(root, "tum")
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
        assert_eq!(tavya_form("gam"), "gantavya");
        assert_eq!(tavya_form("nI"), "netavya");
        assert_eq!(tavya_form("Sru"), "Srotavya");
        assert_eq!(tum_form("gam"), "gantum");
        assert_eq!(tum_form("BU"), "Bavitum");
        assert_eq!(lyuw_form("kf"), "karaRa");
        assert_eq!(lyuw_form("nI"), "nayana");
        assert_eq!(lyuw_form("Sru"), "SravaRa");
        assert_eq!(anIya_form("kf"), "karaRIya");
        assert_eq!(anIya_form("han"), "hananIya");
        assert_eq!(anIya_form("Sru"), "SravaRIya");
    }
}
