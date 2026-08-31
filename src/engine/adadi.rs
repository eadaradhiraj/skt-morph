//! अदादि (गण 2): शप् लुक् (2.4.72), pit गुण (7.3.86), jhal sandhi (8.2 / 8.4).
//! as / han / iṇ stay in `join.rs`; this covers द्विष्, दुह्, यु, या, वच्, मृज्, वी.
#![allow(non_snake_case)]

use crate::engine::it::surface_root;
use crate::engine::phonology::{apply_guna_to_stem, apply_natva_to_word, apply_vrddhi_to_stem, thematic_join};

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

fn is_cons(c: char) -> bool {
    !is_vowel(c)
}

fn root_of(dhatu: &str) -> String {
    // Before prakriya_root: wu-इत् + final u would strip to kz (wukzu → wukz).
    match dhatu {
        "wukzu" | "kzu" | "02.0031" => return "kzu".into(),
        "kzRu" | "02.0032" => return "kzRu".into(),
        "zRu" | "02.0033" => return "snu".into(),
        "Ru" | "02.0030" => return "nu".into(),
        "zu" | "02.0036" => return "su".into(),
        "UrRuY" | "UrRu" | "02.0034" => return "Urnu".into(),
        "mfjU" | "mfj" | "02.0061" => return "mfj".into(),
        "dviza" | "02.0003" => return "dviz".into(),
        "duha" | "02.0004" => return "duh".into(),
        "diha" | "02.0005" => return "dih".into(),
        "liha" | "02.0006" => return "lih".into(),
        "vaca" | "02.0058" => return "vac".into(),
        "ada" | "02.0001" => return "ad".into(),
        _ => {}
    }
    let mut r = surface_root(dhatu);
    match r.as_str() {
        "wukzu" | "kzu" => return "kzu".into(),
        "kzRu" => return "kzRu".into(),
        "zRu" => return "snu".into(),
        "Ru" => return "nu".into(),
        "zu" => return "su".into(),
        "UrRuY" | "UrRu" => return "Urnu".into(),
        "mfjU" | "mfj" => return "mfj".into(),
        "dviza" => return "dviz".into(),
        "duha" => return "duh".into(),
        "diha" => return "dih".into(),
        "liha" => return "lih".into(),
        "vaca" => return "vac".into(),
        "ada" => return "ad".into(),
        _ => {}
    }
    if r.starts_with("wu") && r.len() > 3 {
        r = r[2..].to_string();
    }
    if r.starts_with('z') {
        r = format!("s{}", &r[1..]);
    }
    if r.starts_with('R') {
        r = format!("n{}", &r[1..]);
    }
    if r.ends_with('a') && r.len() >= 3 {
        let core = &r[..r.len() - 1];
        if core.chars().last().is_some_and(is_cons) {
            r = core.to_string();
        }
    }
    r
}

fn pit(family: &str, ending: &str) -> bool {
    match family {
        "lat" | "lrt" => matches!(ending, "ti" | "si" | "mi" | "Ami"),
        "lot" => matches!(ending, "tu" | "Ani" | "Ava" | "Ama"),
        "lang" => matches!(ending, "at" | "ad" | "aH" | "am"),
        _ => false,
    }
}

fn strip_a(ending: &str) -> &str {
    ending.strip_prefix('a').unwrap_or(ending)
}

/// 8.2.30 coḥ kuḥ; 8.2.31 ho ḍhaḥ; 8.2.32 dāder ghah; 8.2.36 ṣaḍhoḥ kaḥ si; 8.4.41 ṣṭunā ṣṭuḥ.
fn jhal(stem: &str, suf: &str, lih: bool) -> String {
    if stem.is_empty() || suf.is_empty() {
        return format!("{stem}{suf}");
    }
    let last = stem.chars().last().unwrap();
    let first = suf.chars().next().unwrap();
    let body: String = stem.chars().rev().skip(1).collect::<String>().chars().rev().collect();
    let rest: String = suf.chars().skip(1).collect();
    if lih && last == 'Q' && matches!(first, 't' | 'T' | 'D') {
        return format!("{stem}{rest}");
    }
    match (last, first) {
        ('z', 't') | ('S', 't') => format!("{body}zw{rest}"),
        ('z', 'T') | ('S', 'T') => format!("{body}zW{rest}"),
        ('z', 's') | ('S', 's') => format!("{body}kz{rest}"),
        ('z', 'D') | ('S', 'D') => format!("{body}qQ{rest}"),
        ('c', 't') => format!("{body}kt{rest}"),
        ('c', 'T') => format!("{body}kT{rest}"),
        ('c', 's') => format!("{body}kz{rest}"),
        ('j', 't') => format!("{body}zw{rest}"),
        ('j', 'T') => format!("{body}zW{rest}"),
        ('j', 's') => format!("{body}kz{rest}"),
        ('j', 'D') => format!("{body}qQ{rest}"),
        ('h', 't') | ('h', 'T') if lih => format!("{body}Q{rest}"),
        ('h', 't') | ('h', 'T') => format!("{body}gD{rest}"),
        ('h', 's') => format!("{body}kz{rest}"),
        ('h', 'D') if lih => format!("{body}Q{rest}"),
        ('h', 'D') => format!("{body}gD{rest}"),
        ('d', 't') => format!("{body}t{suf}"),
        _ => format!("{stem}{suf}"),
    }
}

/// 8.2.39 jhalāṃ jaśo 'nte (plus 8.2.30 / 8.2.36).
fn padanta(stem: &str, lih: bool) -> String {
    let Some(last) = stem.chars().last() else {
        return stem.to_string();
    };
    let body: String = stem.chars().rev().skip(1).collect::<String>().chars().rev().collect();
    match last {
        'z' | 'S' | 'j' => format!("{body}w"),
        'h' if lih => format!("{body}w"),
        // 8.2.32 दादेर्धात्वो घः, then k at pause: दोह् → धोक्.
        'h' if stem.starts_with('d') => format!("D{}k", &body[1..]),
        'h' => format!("{body}k"),
        'c' => format!("{body}k"),
        'd' => format!("{body}t"),
        _ => stem.to_string(),
    }
}

fn lih_weak(root: &str) -> String {
    // 6.3.111 ढ्रलोपे पूर्वस्य दीर्घोऽणः: लिह् + त् → लीढ.
    let mut chars: Vec<char> = root.chars().collect();
    chars.pop();
    if let Some(last) = chars.last_mut() {
        *last = match *last {
            'i' => 'I',
            'u' => 'U',
            'a' => 'A',
            x => x,
        };
    }
    chars.into_iter().collect::<String>() + "Q"
}

fn apply_aug(form: String, family: &str, _augment: Option<&str>) -> String {
    if family != "lang" {
        return form;
    }
    // 6.4.72 आडागम with vowel sandhi (a+U → O, a+a → A).
    match form.chars().next() {
        Some('a') | Some('A') => format!("A{}", &form[1..]),
        Some('i') | Some('I') | Some('e') | Some('E') => format!("E{}", &form[1..]),
        Some('u') | Some('U') | Some('o') | Some('O') => format!("O{}", &form[1..]),
        _ => format!("a{form}"),
    }
}

fn u_final(root: &str) -> bool {
    root.ends_with('u')
}

fn av(root: &str) -> String {
    // 6.1.78 eco 'yavāyāvaḥ: यु + vowel → यव्.
    format!("{}av", &root[..root.len() - 1])
}

fn join_u(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    let body = &root[..root.len() - 1];
    let strong = format!("{body}O");
    let weak = root.to_string();
    let uv = format!("{body}uv");
    let avs = av(root);
    let form = match family {
        "lat" => match ending {
            "ti" => format!("{strong}ti"),
            "si" => format!("{strong}zi"),
            "mi" | "Ami" => format!("{strong}mi"),
            "taH" => format!("{weak}taH"),
            "thaH" | "TaH" => format!("{weak}TaH"),
            "tha" | "Ta" => format!("{weak}Ta"),
            "anti" | "nti" => format!("{uv}anti"),
            "vaH" | "AvaH" => format!("{weak}vaH"),
            "maH" | "AmaH" => format!("{weak}maH"),
            _ => format!("{weak}{ending}"),
        },
        "lot" => match ending {
            "tu" => format!("{strong}tu"),
            "tAt" | "tAd" => format!("{weak}tAt"),
            "tAm" => format!("{weak}tAm"),
            "antu" => format!("{uv}antu"),
            "Di" => format!("{weak}hi"),
            "tam" => format!("{weak}tam"),
            "ta" => format!("{weak}ta"),
            "Ani" => format!("{avs}Ani"),
            "Ava" => format!("{avs}Ava"),
            "Ama" => format!("{avs}Ama"),
            _ => format!("{weak}{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" => format!("{strong}t"),
                "aH" => format!("{strong}H"),
                "atAm" => format!("{weak}tAm"),
                "an" => format!("{uv}an"),
                "atam" => format!("{weak}tam"),
                "ata" => format!("{weak}ta"),
                "am" => format!("{avs}am"),
                "va" => format!("{weak}va"),
                "ma" => format!("{weak}ma"),
                _ => format!("{weak}{}", strip_a(ending)),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => {
            let uy = format!("{body}uy");
            match ending {
                "yAt" | "yAd" => format!("{uy}At"),
                "yAtAm" => format!("{uy}AtAm"),
                "yuH" => format!("{body}uyuH"),
                "yAH" => format!("{uy}AH"),
                "yAtam" => format!("{uy}Atam"),
                "yAta" => format!("{uy}Ata"),
                "yAm" => format!("{uy}Am"),
                "yAva" => format!("{uy}Ava"),
                "yAma" => format!("{uy}Ama"),
                _ => format!("{uy}{}", ending.strip_prefix('y').unwrap_or(ending)),
            }
        }
        "lrt" => thematic_join(&format!("{avs}izya"), ending),
        _ => return None,
    };
    Some(form)
}

fn join_a(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    let form = match family {
        "lat" => match ending {
            "anti" | "nti" => format!("{root}nti"),
            "Ami" => format!("{root}mi"),
            "AvaH" => format!("{root}vaH"),
            "AmaH" => format!("{root}maH"),
            _ => format!("{root}{ending}"),
        },
        "lot" => match ending {
            "antu" => format!("{root}ntu"),
            "Ani" => format!("{}ni", &root[..root.len() - 1]),
            "Ava" => format!("{root}va"),
            "Ama" => format!("{root}ma"),
            "Di" => format!("{root}hi"),
            _ => format!("{root}{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" => format!("{root}t"),
                "aH" => format!("{root}H"),
                "an" => format!("{root}n"),
                "am" => format!("{root}m"),
                "atAm" => format!("{root}tAm"),
                "atam" => format!("{root}tam"),
                "ata" => format!("{root}ta"),
                "va" => format!("{root}va"),
                "ma" => format!("{root}ma"),
                _ => format!("{root}{}", strip_a(ending)),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => format!("{root}{ending}"),
        "lrt" => thematic_join(&format!("{root}sya"), ending),
        _ => return None,
    };
    Some(form)
}

fn join_i(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    // वी: वेति, वीतः, वियन्ति (7.3.84; 6.4.77).
    let body = &root[..root.len() - 1];
    let strong = format!("{body}e");
    let weak = format!("{body}I");
    let iy = format!("{body}iy");
    let form = match family {
        "lat" => match ending {
            "ti" => format!("{strong}ti"),
            "si" => format!("{strong}zi"),
            "mi" | "Ami" => format!("{strong}mi"),
            "taH" => format!("{weak}taH"),
            "thaH" | "TaH" => format!("{weak}TaH"),
            "tha" | "Ta" => format!("{weak}Ta"),
            "anti" | "nti" => format!("{iy}anti"),
            "vaH" | "AvaH" => format!("{weak}vaH"),
            "maH" | "AmaH" => format!("{weak}maH"),
            _ => format!("{weak}{ending}"),
        },
        "lot" => match ending {
            "tu" | "tAt" | "tAd" | "Di" => format!("{weak}tAt"),
            "tAm" => format!("{weak}tAm"),
            "antu" => format!("{iy}antu"),
            "tam" => format!("{weak}tam"),
            "ta" => format!("{weak}ta"),
            "Ani" => format!("{strong}Ani"),
            "Ava" => format!("{strong}Ava"),
            "Ama" => format!("{strong}Ama"),
            _ => format!("{weak}{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" => format!("{strong}t"),
                "aH" => format!("{strong}H"),
                "atAm" => format!("{weak}tAm"),
                "an" => format!("{iy}an"),
                "atam" => format!("{weak}tam"),
                "ata" => format!("{weak}ta"),
                "am" => format!("{strong}am"),
                "va" => format!("{weak}va"),
                "ma" => format!("{weak}ma"),
                _ => format!("{weak}{}", strip_a(ending)),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => format!("{iy}{}", ending.strip_prefix('y').unwrap_or(ending)),
        "lrt" => thematic_join(&format!("{strong}zya"), ending),
        _ => return None,
    };
    Some(form)
}

fn cons_stems(root: &str) -> (String, String, bool) {
    let lih = root == "lih";
    if root == "mfj" {
        return (apply_vrddhi_to_stem(root), root.to_string(), false);
    }
    if lih {
        return (apply_guna_to_stem(root), lih_weak(root), true);
    }
    (apply_guna_to_stem(root), root.to_string(), lih)
}

fn join_cons(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    let (strong0, t_weak, lih) = cons_stems(root);
    let mfj = root == "mfj";
    let pl3_strong = mfj && matches!(ending, "anti" | "nti" | "antu" | "an");
    let use_strong = pit(family, ending) || pl3_strong;
    let stem = if use_strong { &strong0 } else { &t_weak };
    let gen = root;

    let form = match family {
        "lat" => match ending {
            "anti" | "nti" if !mfj => format!("{gen}anti"),
            "anti" | "nti" => format!("{strong0}anti"),
            "vaH" | "AvaH" => format!("{gen}vaH"),
            "maH" | "AmaH" => format!("{gen}maH"),
            "mi" | "Ami" => format!("{strong0}mi"),
            _ => jhal(stem, ending, lih),
        },
        "lot" => match ending {
            "antu" if mfj => format!("{strong0}antu"),
            "antu" => format!("{gen}antu"),
            "Ani" => format!("{strong0}Ani"),
            "Ava" => format!("{strong0}Ava"),
            "Ama" => format!("{strong0}Ama"),
            "Di" => jhal(&t_weak, "Di", lih),
            "tu" => jhal(&strong0, "tu", lih),
            _ => jhal(if pit(family, ending) { &strong0 } else { &t_weak }, ending, lih),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" | "aH" => padanta(&strong0, lih),
                "am" => format!("{strong0}am"),
                "an" if mfj => format!("{strong0}an"),
                "an" => format!("{gen}an"),
                "atAm" => jhal(&t_weak, "tAm", lih),
                "atam" => jhal(&t_weak, "tam", lih),
                "ata" => jhal(&t_weak, "ta", lih),
                "va" => format!("{gen}va"),
                "ma" => format!("{gen}ma"),
                _ => jhal(&t_weak, strip_a(ending), lih),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => format!("{gen}{ending}"),
        "lrt" => {
            let sya = if root == "duh" || root == "dih" {
                format!("D{}kzya", &strong0[1..strong0.len() - 1])
            } else if crate::engine::it::anit_sya(root) || matches!(root, "dviz" | "vac" | "mfj") {
                let j = jhal(&strong0, "sya", lih);
                if j.ends_with('a') {
                    j
                } else {
                    format!("{strong0}sya")
                }
            } else {
                format!("{}izya", apply_guna_to_stem(root))
            };
            thematic_join(&sya, ending)
        }
        _ => return None,
    };
    Some(form)
}

/// Full surface form for गण 2, or `None` to fall through (अस्, हन्, इण्, अधी+इ).
pub fn join_form(
    dhatu: &str,
    family: &str,
    ending: &str,
    _purusha: u8,
    _vacana: u8,
    augment: Option<&str>,
) -> Option<String> {
    if !matches!(family, "lat" | "lot" | "lang" | "vidhilin" | "lrt") {
        return None;
    }
    let r = root_of(dhatu);
    // as / han / iṇ stay in join.rs; अद् too (d+t अत्ति, लङ् आदत्).
    if r.is_empty() || matches!(r.as_str(), "as" | "han" | "i" | "ik" | "ad") {
        return None;
    }
    if r == "ik" || dhatu == "ik" || dhatu == "02.0042" {
        return None;
    }
    let form = if u_final(&r) {
        join_u(&r, family, ending, augment)
    } else if r.ends_with('A') {
        join_a(&r, family, ending, augment)
    } else if r.ends_with('I') || r == "vI" {
        join_i(&r, family, ending, augment)
    } else if r.chars().last().is_some_and(is_cons) {
        join_cons(&r, family, ending, augment)
    } else {
        None
    };
    form.map(|f| apply_natva_to_word(&f))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dviz_duh_yu_ya() {
        assert_eq!(join_form("dviza", "lat", "ti", 1, 1, None).as_deref(), Some("dvezwi"));
        assert_eq!(join_form("dviza", "lat", "taH", 1, 2, None).as_deref(), Some("dvizwaH"));
        assert_eq!(join_form("dviza", "lat", "anti", 1, 3, None).as_deref(), Some("dvizanti"));
        assert_eq!(join_form("dviza", "lat", "si", 2, 1, None).as_deref(), Some("dvekzi"));
        assert_eq!(join_form("duha", "lat", "ti", 1, 1, None).as_deref(), Some("dogDi"));
        assert_eq!(join_form("liha", "lat", "ti", 1, 1, None).as_deref(), Some("leQi"));
        assert_eq!(join_form("yu", "lat", "ti", 1, 1, None).as_deref(), Some("yOti"));
        assert_eq!(join_form("yu", "lat", "anti", 1, 3, None).as_deref(), Some("yuvanti"));
        assert_eq!(join_form("yA", "lat", "ti", 1, 1, None).as_deref(), Some("yAti"));
        assert_eq!(join_form("vaca", "lat", "ti", 1, 1, None).as_deref(), Some("vakti"));
        assert_eq!(join_form("mfjU", "lat", "ti", 1, 1, None).as_deref(), Some("mArzwi"));
        assert_eq!(join_form("dviza", "lang", "at", 1, 1, None).as_deref(), Some("advew"));
        assert_eq!(join_form("yA", "lang", "at", 1, 1, None).as_deref(), Some("ayAt"));
        assert_eq!(join_form("yA", "lot", "Di", 2, 1, None).as_deref(), Some("yAhi"));
        assert_eq!(join_form("duha", "lrt", "ti", 1, 1, None).as_deref(), Some("Dokzyati"));
        assert_eq!(join_form("UrRuY", "lang", "at", 1, 1, None).as_deref(), Some("OrROt"));
        assert_eq!(join_form("dviza", "lrt", "ti", 1, 1, None).as_deref(), Some("dvekzyati"));
        assert_eq!(join_form("dviza", "lrt", "Ami", 3, 1, None).as_deref(), Some("dvekzyAmi"));
        assert_eq!(join_form("wukzu", "lat", "ti", 1, 1, None).as_deref(), Some("kzOti"));
        assert_eq!(join_form("yu", "lrt", "ti", 1, 1, None).as_deref(), Some("yavizyati"));
        assert_eq!(join_form("yu", "lot", "Ani", 3, 1, None).as_deref(), Some("yavAni"));
        assert_eq!(join_form("liha", "lat", "taH", 1, 2, None).as_deref(), Some("lIQaH"));
        assert_eq!(join_form("dviza", "lot", "Ani", 3, 1, None).as_deref(), Some("dvezARi"));
    }
}
