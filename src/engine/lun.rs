//! लुङ् (3.2.110) as in the Siddhānta-Kaumudī.
//!
//! 6.4.71 अट्; 6.4.72 आट्; 2.4.77 सिच् लुक् (गातिस्थाघुपाभू);
//! 3.1.55 गमॢहनविदिभ्योऽङ्; 2.4.42 हनो वधः; 3.1.45 क्सः; 3.1.48 चङ्;
//! 7.2.1 सिचि वृद्धिः; 2.4.79 तनादिभ्यः सिच् लुक् (आत्मने अकृत).

use crate::engine::it::{anit_sic, ruki_s, sic_p_body, surface_root};
use crate::engine::join::internal_sandhi;
use crate::engine::phonology::apply_guna_to_stem;

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

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

pub(crate) fn cang_kartari(stem: &str, purusha: u8, vacana: u8, pada: &str) -> Vec<String> {
    if pada == "A" {
        ang_atmane(stem, purusha, vacana)
    } else {
        ang_thematic(stem, purusha, vacana)
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

fn ang_atmane(stem: &str, purusha: u8, vacana: u8) -> Vec<String> {
    let a = with_augment(stem);
    match (purusha, vacana) {
        (1, 1) => vec![format!("{a}ata")],
        (1, 2) => vec![format!("{a}atAm")],
        (1, 3) => vec![format!("{a}anta")],
        (2, 1) => vec![format!("{a}aTAH")],
        (2, 2) => vec![format!("{a}aTAm")],
        (2, 3) => vec![format!("{a}aDvam")],
        (3, 1) => vec![format!("{a}e")],
        (3, 2) => vec![format!("{a}Avahi")],
        (3, 3) => vec![format!("{a}Amahi")],
        _ => vec![],
    }
}

/// सिच् + ईट् parasmai: अकार्षीत्, अनैषीत्, अवधीत्.
pub(crate) fn sic_it_p(body: &str, purusha: u8, vacana: u8) -> Vec<String> {
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

/// आत्मने सिच्: सेट् अभविष्ट; तनादि लुक् अकृत.
pub(crate) fn sic_a(body: &str, purusha: u8, vacana: u8) -> Vec<String> {
    let a = with_augment(body);
    match (purusha, vacana) {
        (1, 1) => vec![internal_sandhi(&a, "ta")],
        (1, 2) => vec![format!("{a}AtAm")],
        (1, 3) => vec![format!("{a}ata")],
        (2, 1) => vec![internal_sandhi(&a, "TAH")],
        (2, 2) => vec![format!("{a}ATAm")],
        (2, 3) => vec![internal_sandhi(&a, "Dvam")],
        (3, 1) => vec![format!("{a}i")],
        (3, 2) => vec![format!("{a}vahi")],
        (3, 3) => vec![format!("{a}mahi")],
        _ => vec![],
    }
}

fn tanadi_luk_a(root: &str, purusha: u8, vacana: u8) -> Vec<String> {
    sic_a(root, purusha, vacana)
}

/// 3.1.48 चङ्: श्रि द्रु स्रु (णिजन्त later).
fn cang_base(root: &str) -> Option<String> {
    match root {
        "Sri" => Some("SiSriy".into()),
        "dru" => Some("dudruv".into()),
        "sru" => Some("susruv".into()),
        _ => None,
    }
}

fn is_sal(c: char) -> bool {
    matches!(c, 'S' | 'z' | 's' | 'h')
}

fn is_ik(c: char) -> bool {
    matches!(c, 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X')
}

/// 3.1.45 शल इगुपधादनिटः क्सः.
fn takes_ksa(root: &str) -> bool {
    let c: Vec<char> = root.chars().collect();
    c.len() == 3 && is_ik(c[1]) && is_sal(c[2]) && anit_sic(root)
        || matches!(root, "diS" | "duh" | "lih" | "guh" | "viz" | "Sic" | "sic")
}

fn ksa_stem(root: &str) -> String {
    if root.ends_with('S') {
        let mut s = root.to_string();
        s.pop();
        format!("{s}kz")
    } else if root.ends_with('h') {
        match root {
            "duh" => "Dukz".into(),
            "lih" => "likz".into(),
            "guh" => "Gukz".into(),
            other => {
                let mut s = other.to_string();
                s.pop();
                format!("{s}kz")
            }
        }
    } else {
        let mut s = internal_sandhi(root, "s");
        if s.ends_with('s') {
            s.pop();
            format!("{s}kz")
        } else {
            s.trim_end_matches('a').to_string()
        }
    }
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

pub fn kartari(dhatu: &str, purusha: u8, vacana: u8, pada: &str) -> Option<Vec<String>> {
    let root = root_of(dhatu);
    if pada == "P" {
        if let Some(forms) = luk(&root, purusha, vacana) {
            return Some(forms);
        }
    }
    // 2.4.42 हनः → वध् + सिच् ईट्
    if root == "han" && pada == "P" {
        return Some(sic_it_p("vaD", purusha, vacana));
    }
    if root == "han" && pada == "A" {
        return Some(ang_atmane("han", purusha, vacana));
    }
    if let Some(base) = cang_base(&root) {
        return Some(ang_thematic(&base, purusha, vacana));
    }
    if takes_ksa(&root) && pada == "P" {
        return Some(ang_thematic(&ksa_stem(&root), purusha, vacana));
    }
    match root.as_str() {
        "gam" if pada == "P" => return Some(ang_thematic("gam", purusha, vacana)),
        "gam" if pada == "A" => return Some(ang_atmane("gam", purusha, vacana)),
        "vid" if pada == "P" => return Some(ang_thematic("vid", purusha, vacana)),
        "vac" if pada == "P" => return Some(ang_thematic("voc", purusha, vacana)),
        "i" if pada == "P" => return Some(sic_it_p("Ez", purusha, vacana)),
        _ => {}
    }
    if pada == "A" && matches!(root.as_str(), "kf" | "tan" | "san" | "kzan") {
        return Some(tanadi_luk_a(&root, purusha, vacana));
    }
    if pada == "A" && root == "BU" {
        return Some(sic_a("Baviz", purusha, vacana));
    }
    if pada == "P" {
        let body = match root.as_str() {
            "kf" => "kArz".into(),
            "nI" => "nEz".into(),
            "Sru" => "SrOz".into(),
            other => sic_p_body(other),
        };
        return Some(sic_it_p(&body, purusha, vacana));
    }
    if pada == "A" {
        let body = if anit_sic(&root) {
            ruki_s(&internal_sandhi(&root, "s"))
                .trim_end_matches(|c| c == 's' || c == 'z')
                .to_string()
                + "z"
        } else {
            format!("{}iz", apply_guna_to_stem(&root))
        };
        // drop trailing z from body for sic_a which adds ta: akfz + ta → akfzta? अकृषत is kf + s + ata with luk?
        // अनिट् आत्मने often अकृषत (kf + ṣ + ata) — use kfz as body without extra s.
        let body = match root.as_str() {
            "kf" => "kfz".into(),
            "nI" => "nez".into(),
            _ => body,
        };
        return Some(sic_a(&body, purusha, vacana));
    }
    None
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

    #[test]
    fn han_ksa_cang_atmane() {
        let f = kartari("hana", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "avaDIt"), "{:?}", f);
        let f = kartari("diSa", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "adikzat"), "{:?}", f);
        let f = kartari("Sri", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "aSiSriyat"), "{:?}", f);
        assert_eq!(kartari("qukfY", 1, 1, "A").unwrap()[0], "akfta");
        let f = kartari("gamx", 1, 3, "A").unwrap();
        assert!(f.iter().any(|x| x == "agamanta"), "{:?}", f);
        let f = kartari("BU", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "aBavizwa"), "{:?}", f);
    }
}
