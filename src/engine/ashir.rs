//! आशीर्लिङ् (3.3.173). यासुट् kit परस्मै; सीयुट् आत्मने.
//! भूयात्, क्रियात्, गम्यात्, देयात्; कृषीष्ट, भविषीष्ट.

use crate::engine::it::{ruki_s, surface_root, takes_it_tavya};
use crate::engine::phonology::apply_guna_to_stem;

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

fn root_of(dhatu: &str) -> String {
    let mut r = surface_root(dhatu);
    if r.ends_with('a') && r.len() >= 3 {
        let core = &r[..r.len() - 1];
        if core.chars().any(is_vowel) {
            r = core.to_string();
        }
    }
    r
}

/// Kit aṅga before यासुट् (परस्मै).
fn stem_p(root: &str) -> String {
    match root {
        "kf" | "f" => "kri".into(),
        "dA" | "DA" | "sTA" | "pA" | "gA" | "mA" | "yA" => {
            let mut s = root.to_string();
            s.pop();
            format!("{s}e")
        }
        "i" => "i".into(),
        _ => root.to_string(),
    }
}

/// आत्मने: कृषीष्ट, भविषीष्ट.
fn stem_a(root: &str) -> String {
    match root {
        "kf" => "kfz".into(),
        "BU" => "Baviz".into(),
        "eD" => "eDiz".into(),
        "nI" => "nez".into(),
        "gam" => "gamiz".into(),
        other if takes_it_tavya(other) => format!("{}iz", apply_guna_to_stem(other)),
        other => ruki_s(&format!("{other}s"))
            .trim_end_matches('s')
            .to_string(),
    }
}

fn endings(pada: &str, purusha: u8, vacana: u8) -> Option<&'static str> {
    if pada == "A" {
        return match (purusha, vacana) {
            (1, 1) => Some("Izwa"),
            (1, 2) => Some("IyAstAm"),
            (1, 3) => Some("Iran"),
            (2, 1) => Some("IzWAH"),
            (2, 2) => Some("IyAstAm"),
            (2, 3) => Some("IDvam"),
            (3, 1) => Some("Iya"),
            (3, 2) => Some("Ivahi"),
            (3, 3) => Some("Imahi"),
            _ => None,
        };
    }
    match (purusha, vacana) {
        (1, 1) => Some("yAt"),
        (1, 2) => Some("yAstAm"),
        (1, 3) => Some("yAsuH"),
        (2, 1) => Some("yAH"),
        (2, 2) => Some("yAstam"),
        (2, 3) => Some("yAsta"),
        (3, 1) => Some("yAsam"),
        (3, 2) => Some("yAsva"),
        (3, 3) => Some("yAsma"),
        _ => None,
    }
}

pub fn kartari(dhatu: &str, purusha: u8, vacana: u8, pada: &str) -> Option<Vec<String>> {
    let root = root_of(dhatu);
    let end = endings(pada, purusha, vacana)?;
    if pada == "A" {
        let st = stem_a(&root);
        return Some(vec![format!("{st}{end}")]);
    }
    let st = stem_p(&root);
    let form = format!("{st}{end}");
    if (purusha, vacana) == (1, 1) {
        return Some(vec![form, format!("{st}yAd")]);
    }
    Some(vec![form])
}

/// णिच्/सन् aṅga + यासुट् / सीयुट्. `anga` may end in a (भावय → भावयात्).
pub(crate) fn from_anga(anga: &str, purusha: u8, vacana: u8, pada: &str) -> Option<Vec<String>> {
    let end = endings(pada, purusha, vacana)?;
    let base = anga.strip_suffix('a').unwrap_or(anga);
    if pada == "A" {
        let st = format!("{base}iz");
        return Some(vec![format!("{st}{end}")]);
    }
    let form = if base.ends_with('y') && end.starts_with('y') {
        format!("{}{}", base, &end[1..])
    } else {
        format!("{base}{end}")
    };
    if (purusha, vacana) == (1, 1) {
        let alt = if form.ends_with('t') {
            format!("{}d", &form[..form.len() - 1])
        } else {
            format!("{form}d")
        };
        return Some(vec![form, alt]);
    }
    Some(vec![form])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bu_kf_gam_ashir() {
        let f = kartari("BU", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "BUyAt"), "{:?}", f);
        assert_eq!(kartari("qukfY", 1, 1, "P").unwrap()[0], "kriyAt");
        assert!(kartari("gamx", 1, 1, "P").unwrap().iter().any(|x| x == "gamyAt"));
        assert!(kartari("qudAY", 1, 1, "P").unwrap().iter().any(|x| x == "deyAt"));
    }

    #[test]
    fn ashir_atmane() {
        assert_eq!(kartari("qukfY", 1, 1, "A").unwrap()[0], "kfzIzwa");
        assert_eq!(kartari("BU", 1, 1, "A").unwrap()[0], "BavizIzwa");
    }
}
