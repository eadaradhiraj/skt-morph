//! णिच् (3.1.26), सन् (3.1.7), यङ् (3.1.22), कर्मणि यक् (3.1.67).
//! Śuddha kartari stays the default path; this module is the derived aṅga.
#![allow(non_snake_case)]

use crate::engine::endings::family_endings;
use crate::engine::it::surface_root;
use crate::engine::join::join_variants;
use crate::engine::phonology::{apply_guna_to_stem, apply_vrddhi_to_stem};

fn is_cons(c: char) -> bool {
    !matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

fn deaspirate(c: char) -> char {
    match c {
        'K' => 'k',
        'G' => 'g',
        'C' => 'c',
        'J' => 'j',
        'T' => 't',
        'D' => 'd',
        'P' => 'p',
        'B' => 'b',
        _ => c,
    }
}

fn palatalize_kuho(c: char) -> char {
    match c {
        'k' => 'c',
        'g' => 'j',
        'h' => 'j',
        _ => c,
    }
}

fn abhyasa_i(root: &str) -> String {
    let Some(c0) = root.chars().next() else {
        return "i".into();
    };
    let c = palatalize_kuho(deaspirate(c0));
    format!("{c}i")
}

fn abhyasa_guna(root: &str) -> String {
    let Some(c0) = root.chars().next() else {
        return "a".into();
    };
    let c = palatalize_kuho(deaspirate(c0));
    format!("{c}o")
}

fn root_of(dhatu: &str) -> String {
    let mut r = surface_root(dhatu);
    if r.ends_with('a') && r.len() >= 3 {
        let core = &r[..r.len() - 1];
        if core.chars().any(|c| !is_cons(c)) {
            r = core.to_string();
        }
    }
    r
}

/// मित् (6.4.92 मितां ह्रस्वः; घटादि / अमन्त).
fn is_mit(root: &str) -> bool {
    matches!(
        root,
        "gam" | "yam" | "jan" | "Kan" | "van" | "tan" | "nam" | "ram" | "kram" | "Bram" | "Sram"
            | "dam" | "Sam" | "tam" | "mad" | "Gaw" | "vyaT" | "praT"
    )
}

fn last_is_cons(root: &str) -> bool {
    root.chars().last().is_some_and(is_cons)
}

/// णिच् aṅga: 7.2.115/116 वृद्धि, 6.4.92 मित्, 7.3.36 पुक्, 7.3.86 लघूपध गुण.
pub fn nic_stem(root: &str) -> String {
    match root {
        "han" => return "GAtaya".into(),
        "i" => return "gamaya".into(),
        "pA" => return "pAyaya".into(),
        // 7.3.37 शाच्छासाह्वाव्यावेपां युक्. शो/छो/षो after 6.1.45 आदेच.
        "So" | "SA" => return "SAyaya".into(),
        "Co" | "CA" => return "CAyaya".into(),
        "zo" | "so" | "sA" => return "sAyaya".into(),
        "hvA" | "hve" => return "hvAyaya".into(),
        "vyA" | "vye" => return "vyAyaya".into(),
        _ if root.ends_with("vep") => return "vepaya".into(),
        _ => {}
    }
    if is_mit(root) {
        return format!("{root}aya");
    }
    match root.chars().last() {
        Some('A') => format!("{root}paya"),
        Some('i') | Some('I') => format!("{}Ayaya", &root[..root.len() - 1]),
        Some('u') | Some('U') => format!("{}Avaya", &root[..root.len() - 1]),
        Some('f') | Some('F') => format!("{}Araya", &root[..root.len() - 1]),
        _ if last_is_cons(root) => {
            let v = root.chars().rev().nth(1);
            match v {
                Some('i') | Some('u') | Some('f') | Some('F') => {
                    format!("{}aya", apply_guna_to_stem(root))
                }
                Some('a') => format!("{}aya", apply_vrddhi_to_stem(root)),
                _ => format!("{}aya", apply_vrddhi_to_stem(root)),
            }
        }
        _ => format!("{}aya", apply_vrddhi_to_stem(root)),
    }
}

/// सन् present aṅga. बुभूष, चिकीर्ष, जिगमिष, जिघांस, दित्स.
pub fn san_stem(root: &str) -> String {
    match root {
        "BU" => "buBUza".into(),
        "kf" => "cikIrza".into(),
        "gam" => "jigamiza".into(),
        "nI" => "ninIza".into(),
        "han" => "jiGAMsa".into(),
        "dA" => "ditsa".into(),
        "pA" => "pipAsa".into(),
        "sTA" => "tizWAsa".into(),
        "vac" => "vivakza".into(),
        "pac" => "pipakza".into(),
        "pat" => "pitsa".into(),
        "jYA" => "jijYAsa".into(),
        "dfS" => "didfkza".into(),
        "grah" => "jiGfkza".into(),
        "BI" => "biBIza".into(),
        "labh" => "lipsa".into(),
        "Sru" => "SuSrUza".into(),
        _ if crate::engine::it::anit_sya(root) && last_is_cons(root) => {
            let abh = abhyasa_i(root);
            let joined = crate::engine::join::internal_sandhi(root, "sa");
            let body = if joined.ends_with('a') {
                joined
            } else {
                format!("{joined}a")
            };
            format!("{abh}{body}")
        }
        _ => {
            let abh = abhyasa_i(root);
            format!("{abh}{root}iza")
        }
    }
}

/// यङ् present aṅga (आत्मने). बोभूय, चेक्रीय, जङ्गम्य.
pub fn yan_stem(root: &str) -> String {
    match root {
        "BU" => "boBUya".into(),
        "kf" => "cekrIya".into(),
        "gam" => "jaNgamya".into(),
        "han" => "jaNGanya".into(),
        "nI" => "nenIya".into(),
        "pac" => "pApacya".into(),
        "dA" => "dedIya".into(),
        _ => {
            let abh = abhyasa_guna(root);
            format!("{abh}{root}ya")
        }
    }
}

/// कर्मणि/भावे यक्. भूय, क्रिय, गम्य, दीय, उच्य.
pub fn karma_stem(root: &str) -> String {
    match root {
        "BU" => "BUya".into(),
        "kf" => "kriya".into(),
        "gam" => "gamya".into(),
        "dA" => "dIya".into(),
        "DA" => "DIya".into(),
        "sTA" => "sTIya".into(),
        "pA" => "pIya".into(),
        "han" => "hanya".into(),
        "nI" => "nIya".into(),
        "vac" => "ucya".into(),
        "yaj" => "ijya".into(),
        "pac" => "pacya".into(),
        "as" => "BUya".into(),
        "i" => "Iya".into(),
        _ if root.ends_with('A') => {
            let mut s = root.to_string();
            s.pop();
            format!("{s}Iya")
        }
        _ => format!("{root}ya"),
    }
}

/// 7.4.1 णौ चङ्युपधाया ह्रस्वः: णिच् aṅga without अय, then चङ् reduplication (7.4.93–94).
fn nic_shortened(root: &str) -> String {
    match root {
        "BU" => "Bav".into(),
        "kf" => "kar".into(),
        "nI" => "nay".into(),
        "dA" => "dap".into(),
        "DA" => "Dap".into(),
        "sTA" => "sTap".into(),
        "pA" => "pay".into(),
        "han" => "Gat".into(),
        "i" => "gam".into(),
        "Sru" => "Srav".into(),
        "vac" => "vac".into(),
        "pat" => "pat".into(),
        "grah" => "grah".into(),
        "dfS" => "dfS".into(),
        _ if is_mit(root) => root.to_string(),
        r if r.ends_with('A') => format!("{}p", &r[..r.len() - 1]),
        r if r.ends_with('I') || r.ends_with('i') => format!("{}ay", &r[..r.len() - 1]),
        r if r.ends_with('U') || r.ends_with('u') => apply_guna_to_stem(r),
        r if r.ends_with('f') || r.ends_with('F') => apply_guna_to_stem(r),
        r => r.to_string(),
    }
}

fn nic_cang_stem(root: &str) -> String {
    let inner = nic_shortened(root);
    let abh = abhyasa_i(&inner).chars().next().map(|c| format!("{c}I")).unwrap_or_else(|| "I".into());
    format!("{abh}{inner}")
}

fn inflect(stem: &str, family: &str, pada: &str, purusha: u8, vacana: u8) -> Vec<String> {
    let (use_stem, aug) = match family {
        "lat" | "lot" => (stem.to_string(), None),
        "lang" => {
            let base = stem.strip_suffix('a').unwrap_or(stem).to_string();
            (base, Some("a".to_string()))
        }
        "vidhilin" => (stem.strip_suffix('a').unwrap_or(stem).to_string(), None),
        "lrt" => {
            let base = stem.strip_suffix('a').unwrap_or(stem);
            (format!("{base}izya"), None)
        }
        _ => return vec![],
    };
    let Some(table) = family_endings(family, "kartari", pada, 10, None) else {
        return vec![];
    };
    let idx = ((purusha - 1) * 3 + (vacana - 1)) as usize;
    if idx >= table.len() {
        return vec![];
    }
    let (variants, _) = &table[idx];
    join_variants(
        &use_stem,
        variants,
        10,
        family,
        purusha,
        pada,
        aug.as_deref(),
        "",
        vacana,
        "",
    )
}

/// `kind`: Ric | san | yaN | karma
pub fn kartari(
    dhatu: &str,
    kind: &str,
    family: &str,
    purusha: u8,
    vacana: u8,
    pada: &str,
) -> Option<Vec<String>> {
    let root = root_of(dhatu);
    let (stem, force_pada) = match kind {
        "Ric" | "nic" | "R" => (nic_stem(&root), None),
        "san" => (san_stem(&root), None),
        "yaN" | "yan" => (yan_stem(&root), Some("A")),
        "karma" | "yak" | "BAve" => (karma_stem(&root), Some("A")),
        _ => return None,
    };
    let pada = force_pada.unwrap_or(pada);
    if family == "ashir" {
        if matches!(kind, "karma" | "yak" | "BAve") {
            return None;
        }
        return crate::engine::ashir::from_anga(&stem, purusha, vacana, pada);
    }
    if family == "lit" {
        if matches!(kind, "karma" | "yak" | "BAve") {
            return None;
        }
        let forms = crate::engine::lit::am_forms(&stem, purusha, vacana, pada);
        return if forms.is_empty() { None } else { Some(forms) };
    }
    if family == "lun" {
        let forms = match kind {
            "Ric" | "nic" | "R" => {
                crate::engine::lun::cang_kartari(&nic_cang_stem(&root), purusha, vacana, pada)
            }
            "san" => {
                let base = stem.strip_suffix('a').unwrap_or(&stem);
                if pada == "A" {
                    crate::engine::lun::sic_a(base, purusha, vacana)
                } else {
                    crate::engine::lun::sic_it_p(base, purusha, vacana)
                }
            }
            "yaN" | "yan" => {
                let base = stem.strip_suffix('a').unwrap_or(&stem);
                crate::engine::lun::cang_kartari(base, purusha, vacana, "A")
            }
            _ => return None,
        };
        return if forms.is_empty() { None } else { Some(forms) };
    }
    let forms = inflect(&stem, family, pada, purusha, vacana);
    if forms.is_empty() {
        None
    } else {
        Some(forms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nic_bu_kf_gam() {
        assert_eq!(nic_stem("BU"), "BAvaya");
        assert_eq!(nic_stem("kf"), "kAraya");
        assert_eq!(nic_stem("gam"), "gamaya");
        let f = kartari("BU", "Ric", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "BAvayati"), "{:?}", f);
        let f = kartari("qukfY", "Ric", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "kArayati"), "{:?}", f);
        let f = kartari("gamx", "Ric", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "gamayati"), "{:?}", f);
        let f = kartari("BI", "Ric", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "BAyayati"), "{:?}", f);
        let f = kartari("dfSir", "san", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "didfkzati"), "{:?}", f);
        let f = kartari("labh", "san", "lat", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x.contains("lips")), "{:?}", f);
        assert_eq!(nic_stem("pac"), "pAcaya");
        assert_eq!(nic_stem("dA"), "dApaya");
        assert_eq!(nic_stem("kzip"), "kzepaya");
        assert_eq!(nic_stem("dfS"), "darSaya");
        assert_eq!(nic_stem("Gaw"), "Gawaya");
        let f = kartari("qupacaz", "Ric", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "pAcayati"), "{:?}", f);
        let f = kartari("qupacaz", "san", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "pipakzati"), "{:?}", f);
        let f = kartari("dfSir", "Ric", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "darSayati"), "{:?}", f);
        assert_eq!(nic_stem("So"), "SAyaya");
        assert_eq!(nic_stem("Co"), "CAyaya");
        assert_eq!(nic_stem("zo"), "sAyaya");
        assert_eq!(nic_stem("hve"), "hvAyaya");
        let f = kartari("So", "Ric", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "SAyayati"), "{:?}", f);
        let f = kartari("hveY", "Ric", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "hvAyayati"), "{:?}", f);
    }

    #[test]
    fn san_yan_karma() {
        let f = kartari("BU", "san", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "buBUzati"), "{:?}", f);
        let f = kartari("qukfY", "san", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "cikIrzati"), "{:?}", f);
        let f = kartari("BU", "yaN", "lat", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "boBUyate"), "{:?}", f);
        let f = kartari("gamx", "karma", "lat", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "gamyate"), "{:?}", f);
        let f = kartari("qukfY", "karma", "lat", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "kriyate"), "{:?}", f);
        let f = kartari("BU", "karma", "lat", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "BUyate"), "{:?}", f);
    }

    #[test]
    fn nic_lit_am_lun_cang() {
        let f = kartari("BU", "Ric", "lit", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "BAvayAYcakAra"), "{:?}", f);
        let f = kartari("qukfY", "Ric", "lit", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "kArayAYcakre"), "{:?}", f);
        let f = kartari("BU", "san", "lit", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "buBUzAYcakAra"), "{:?}", f);
        assert_eq!(nic_cang_stem("kf"), "cIkar");
        assert_eq!(nic_cang_stem("BU"), "bIBav");
        assert_eq!(nic_cang_stem("gam"), "jIgam");
        let f = kartari("qukfY", "Ric", "lun", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "acIkarat" || x == "acIkarad"), "{:?}", f);
        let f = kartari("BU", "Ric", "lun", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "abIBavat" || x == "abIBavad"), "{:?}", f);
        let f = kartari("gamx", "Ric", "lun", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "ajIgamat" || x == "ajIgamad"), "{:?}", f);
        let f = kartari("BU", "Ric", "ashir", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "BAvayAt" || x == "BAvayAd"), "{:?}", f);
        let f = kartari("BU", "san", "lun", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "abuBUzIt" || x == "abuBUzId"), "{:?}", f);
        let f = kartari("BU", "yaN", "lun", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "aboBUyata"), "{:?}", f);
    }
}
