//! लुङ् (3.2.110) as in the Siddhānta-Kaumudī.
//!
//! 6.4.71 लुङ्लङ्लृङ्क्ष्वडुदात्तः (अट्); 6.4.72 आडजादीनाम्;
//! 2.4.77 गातिस्थाघुपाभूभ्यः सिचः लुक्; 7.2.1 सिचि वृद्धिः परस्मैपदेषु;
//! गम् अङ् → अगमत्.

use crate::engine::lit::prakriya_root;

fn root_of(dhatu: &str) -> String {
    match prakriya_root(dhatu).as_str() {
        "RI" => "nI".into(),
        "brU" => "vac".into(),
        "zWA" => "sTA".into(),
        other => other.into(),
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

/// 6.4.71 अट्; 6.4.72 आट् + 6.1.90 आटश्च.
fn with_augment(body: &str) -> String {
    let Some(first) = body.chars().next() else {
        return format!("a{body}");
    };
    if !is_vowel(first) {
        return format!("a{body}");
    }
    let rest: String = body.chars().skip(1).collect();
    match first {
        'a' | 'A' => format!("A{rest}"),
        'i' | 'I' => format!("E{rest}"),
        'u' | 'U' => format!("O{rest}"),
        'f' | 'F' => format!("Ar{rest}"),
        'e' => format!("E{rest}"),
        'o' => format!("O{rest}"),
        other => format!("A{other}{rest}"),
    }
}

fn cell(p: u8, v: u8, forms: [&str; 9]) -> Vec<String> {
    let i = ((p - 1) * 3 + (v - 1)) as usize;
    if i >= 9 {
        vec![]
    } else {
        vec![forms[i].to_string()]
    }
}

/// 2.4.77 सिच् लुक्: भू, दा, धा, स्था, पा, गा.
fn luk(root: &str, purusha: u8, vacana: u8) -> Option<Vec<String>> {
    match root {
        "BU" => Some(cell(
            purusha,
            vacana,
            ["aBUt", "aBUtAm", "aBUvan", "aBUH", "aBUtam", "aBUta", "aBUvam", "aBUva", "aBUma"],
        )),
        "dA" => Some(cell(
            purusha,
            vacana,
            ["adAt", "adAtAm", "aduH", "adAH", "adAtam", "adAta", "adAm", "adAva", "adAma"],
        )),
        "DA" => Some(cell(
            purusha,
            vacana,
            ["aDAt", "aDAtAm", "aDuH", "aDAH", "aDAtam", "aDAta", "aDAm", "aDAva", "aDAma"],
        )),
        "sTA" => Some(cell(
            purusha,
            vacana,
            ["asTAt", "asTAtAm", "asTuH", "asTAH", "asTAtam", "asTAta", "asTAm", "asTAva", "asTAma"],
        )),
        "pA" => Some(cell(
            purusha,
            vacana,
            ["apAt", "apAtAm", "apuH", "apAH", "apAtam", "apAta", "apAm", "apAva", "apAma"],
        )),
        "gA" => Some(cell(
            purusha,
            vacana,
            ["agAt", "agAtAm", "aguH", "agAH", "agAtam", "agAta", "agAm", "agAva", "agAma"],
        )),
        _ => None,
    }
}

fn ang_thematic(stem: &str, purusha: u8, vacana: u8) -> Vec<String> {
    let a = with_augment(stem);
    match (purusha, vacana) {
        (1, 1) => vec![format!("{a}at"), format!("{a}ad")],
        (1, 2) => vec![format!("{a}atAm")],
        (1, 3) => vec![format!("{a}an")],
        (2, 1) => vec![format!("{a}aH")],
        (2, 2) => vec![format!("{a}atam")],
        (2, 3) => vec![format!("{a}ata")],
        (3, 1) => vec![format!("{a}am")],
        (3, 2) => vec![format!("{a}Ava")],
        (3, 3) => vec![format!("{a}Ama")],
        _ => vec![],
    }
}

/// सिच् + वृद्धि + ष/स्. कृ अकार्षीत्; नी अनैषीत्; इण् ऐषीत् (आट् already in Ez).
fn sic_vrddhi(body: &str, purusha: u8, vacana: u8) -> Vec<String> {
    let a = match body.chars().next() {
        Some(c) if is_vowel(c) => body.to_string(),
        _ => with_augment(body),
    };
    match (purusha, vacana) {
        (1, 1) => vec![format!("{a}It"), format!("{a}Id")],
        (1, 2) => vec![format!("{a}tAm")],
        (1, 3) => vec![format!("{a}uH")],
        (2, 1) => vec![format!("{a}IH")],
        (2, 2) => vec![format!("{a}tam")],
        (2, 3) => vec![format!("{a}ta")],
        (3, 1) => vec![format!("{a}am")],
        (3, 2) => vec![format!("{a}va")],
        (3, 3) => vec![format!("{a}ma")],
        _ => vec![],
    }
}

pub fn kartari(dhatu: &str, purusha: u8, vacana: u8, pada: &str) -> Option<Vec<String>> {
    let root = root_of(dhatu);
    if let Some(forms) = luk(&root, purusha, vacana) {
        if pada == "A" {
            return None;
        }
        return Some(forms);
    }
    match root.as_str() {
        "gam" if pada == "P" => Some(ang_thematic("gam", purusha, vacana)),
        "vac" if pada == "P" => Some(ang_thematic("voc", purusha, vacana)),
        "kf" if pada == "P" => Some(sic_vrddhi("kArz", purusha, vacana)),
        "nI" if pada == "P" => Some(sic_vrddhi("nEz", purusha, vacana)),
        "i" if pada == "P" => Some(sic_vrddhi("Ez", purusha, vacana)),
        "Sru" if pada == "P" => Some(sic_vrddhi("SrOz", purusha, vacana)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bu_da_gam_kf_lun() {
        assert_eq!(kartari("BU", 1, 1, "P").unwrap(), vec!["aBUt"]);
        assert_eq!(kartari("BU", 1, 3, "P").unwrap(), vec!["aBUvan"]);
        assert_eq!(kartari("qudAY", 1, 1, "P").unwrap(), vec!["adAt"]);
        assert_eq!(kartari("zWA", 1, 1, "P").unwrap()[0], "asTAt");
        assert!(kartari("gamx", 1, 1, "P").unwrap().iter().any(|x| x == "agamat"));
        assert_eq!(kartari("qukfY", 1, 1, "P").unwrap()[0], "akArzIt");
        assert_eq!(kartari("RIY", 1, 1, "P").unwrap()[0], "anEzIt");
        assert!(kartari("vaca", 1, 1, "P").unwrap().iter().any(|x| x == "avocat"));
    }
}
