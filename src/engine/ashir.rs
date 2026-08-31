//! आशीर्लिङ् (3.3.173). यासुट् is kit: भूयात्, क्रियात्, गम्यात्, देयात्.
//! Separate from विधिलिङ् (भवेत्).

use crate::engine::lit::prakriya_root;

fn root_of(dhatu: &str) -> String {
    match prakriya_root(dhatu).as_str() {
        "RI" => "nI".into(),
        "zWA" => "sTA".into(),
        other => other.into(),
    }
}

/// Kit aṅga before यासुट्.
fn stem(root: &str) -> String {
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

fn endings(pada: &str, purusha: u8, vacana: u8) -> Option<&'static str> {
    if pada == "A" {
        return match (purusha, vacana) {
            (1, 1) => Some("sIzwa"),
            (1, 2) => Some("sIyAstAm"),
            (1, 3) => Some("sIran"),
            (2, 1) => Some("sIzWAH"),
            (2, 2) => Some("sIyAstAm"),
            (2, 3) => Some("sIDvam"),
            (3, 1) => Some("sIya"),
            (3, 2) => Some("sIvahi"),
            (3, 3) => Some("sImahi"),
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
    let st = stem(&root);
    let end = endings(pada, purusha, vacana)?;
    let form = format!("{st}{end}");
    if pada == "P" && (purusha, vacana) == (1, 1) {
        return Some(vec![form, format!("{st}yAd")]);
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
}
