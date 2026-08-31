//! अदादि (गण 2): शप् लुक् (2.4.72), pit गुण (7.3.86), jhal sandhi (8.2 / 8.4).
//! अद् stays in `join.rs` until `jhal` covers द्+थ् / लङ् अपृक्त. This covers द्विष्, दुह्, यु, या, वच्, मृज्, वी,
//! विद्, रुदादि, शास्, वश्, जागृ, अधि+इ, दरिद्रा, चकास्, षस्, षस्ति, चक्षिङ् लृट्,
//! हन् (6.4.98 / 7.3.54), अस् (6.4.111 / 7.3.96), इण् (7.3.84 / 6.1.77).
#![allow(non_snake_case)]

use crate::engine::it::dhatu_satva;
use crate::engine::phonology::{apply_guna_to_stem, apply_natva_to_word, apply_vrddhi_to_stem, thematic_join};

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

fn is_cons(c: char) -> bool {
    !is_vowel(c)
}

fn root_of(dhatu: &str) -> String {
    // षस्ति is संस्त; not 6.1.64 alone.
    if matches!(dhatu, "zasti" | "02.0074") {
        return "saMst".into();
    }
    // 1.3.5 ञिटुडवः then 6.1.64/65. Not prakriya_root: that strips radical u of ऊर्णु.
    // Not surface_root: 2.4.53 ब्रुवो वचिः is लिट्/लृट्, not लट्.
    let mut s = dhatu.trim_end_matches('~').to_string();
    if s.starts_with("qu") && s.len() > 3 {
        s = s[2..].to_string();
    }
    if s.starts_with("wu") && s.len() > 3 {
        s = s[2..].to_string();
    }
    if s.starts_with("Yi") && s.len() > 3 {
        s = s[2..].to_string();
    }
    if s.ends_with("ir") && s.len() > 3 {
        s = s[..s.len() - 2].to_string();
    }
    if s.ends_with('Y') && s.len() > 2 {
        s = s[..s.len() - 1].to_string();
    }
    if s.ends_with('R') && s.len() == 2 {
        s = s[..s.len() - 1].to_string();
    }
    if s.ends_with('N') && s.len() > 3 {
        s = s[..s.len() - 1].to_string();
    }
    if s.ends_with('u') && s.len() > 3 {
        let rest = &s[..s.len() - 1];
        if rest.chars().last().is_some_and(is_cons)
            && rest.chars().any(|c| matches!(c, 'a' | 'A' | 'i' | 'I' | 'e' | 'o' | 'f'))
        {
            s = rest.to_string();
        }
    }
    if s.ends_with('U') && s.len() > 3 {
        let rest = &s[..s.len() - 1];
        if rest.chars().last().is_some_and(is_cons) {
            s = rest.to_string();
        }
    }
    if s.ends_with('f') && s.len() > 4 && s != "jAgf" {
        let rest = &s[..s.len() - 1];
        if rest.chars().last().is_some_and(is_cons) {
            s = rest.to_string();
        }
    }
    if s.ends_with('i') && s.len() > 4 {
        let rest = &s[..s.len() - 1];
        if rest.chars().last().is_some_and(is_cons) && rest.chars().any(is_vowel) {
            s = rest.to_string();
        }
    }
    s = dhatu_satva(&s);
    if s.ends_with('a') && s.len() >= 3 {
        let core = &s[..s.len() - 1];
        if core.chars().last().is_some_and(is_cons) {
            s = core.to_string();
        }
    }
    s
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
        "lrt" => {
            let stem = if crate::engine::it::anit_sya(root) {
                crate::engine::it::sya_stem(root)
            } else {
                format!("{avs}izya")
            };
            thematic_join(&stem, ending)
        }
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

/// 3.4.83 विदो लटो वा: वेत्ति / वेत्थ / वेद; weak वित्तः.
fn join_vid(root: &str, family: &str, ending: &str, _purusha: u8, augment: Option<&str>) -> Option<String> {
    if root != "vid" {
        return None;
    }
    Some(match family {
        "lat" => match ending {
            "ti" => "vetti".into(),
            "taH" => "vittaH".into(),
            "anti" | "nti" => "vidanti".into(),
            "si" => "vetTa".into(),
            "TaH" | "thaH" => "vitTaH".into(),
            "Ta" | "tha" => "vitTa".into(),
            "mi" | "Ami" => "veda".into(),
            "vaH" | "AvaH" => "vidvaH".into(),
            "maH" | "AmaH" => "vidmaH".into(),
            _ => format!("vid{ending}"),
        },
        "lot" => match ending {
            "tu" | "tAt" | "tAd" => "vittAt".into(),
            "tAm" => "vittAm".into(),
            "antu" => "vidantu".into(),
            "Di" => "vittAt".into(),
            "tam" => "vittam".into(),
            "ta" => "vitta".into(),
            "Ani" => "vidAni".into(),
            "Ava" => "vidAva".into(),
            "Ama" => "vidAma".into(),
            _ => format!("vid{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" => "vet".into(),
                "aH" => "veH".into(),
                "atAm" => "vittAm".into(),
                "an" => "viduH".into(),
                "atam" => "vittam".into(),
                "ata" => "vitta".into(),
                "am" => "vedam".into(),
                "va" => "vidva".into(),
                "ma" => "vidma".into(),
                _ => format!("vid{}", strip_a(ending)),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => format!("vid{ending}"),
        "lrt" => thematic_join("vedizya", ending),
        _ => return None,
    })
}

/// 7.3.98 रुदादिभ्यः सार्वधातुके: रोदिति; जक्ष् also 7.1.4 अति.
fn rudadi_gun(root: &str) -> Option<(String, bool, bool)> {
    // (guna stem, अनिट् स्य, अभ्यस्त 3pl अति/उस्)
    match root {
        "rud" => Some(("rod".into(), false, false)),
        "Svas" => Some(("Svas".into(), false, false)),
        "an" => Some(("an".into(), false, false)),
        "svap" => Some(("svap".into(), true, false)),
        "jakz" => Some(("jakz".into(), false, true)),
        _ => None,
    }
}

fn join_rudadi(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    let (gun, anit, abhyasta) = rudadi_gun(root)?;
    let form = match family {
        "lat" => match ending {
            "ti" => format!("{gun}iti"),
            "si" => format!("{gun}izi"),
            "mi" | "Ami" => format!("{gun}imi"),
            "taH" => format!("{root}itaH"),
            "TaH" | "thaH" => format!("{root}iTaH"),
            "Ta" | "tha" => format!("{root}iTa"),
            "anti" | "nti" if abhyasta => format!("{root}ati"),
            "anti" | "nti" => format!("{root}anti"),
            "vaH" | "AvaH" => format!("{root}ivaH"),
            "maH" | "AmaH" => format!("{root}imaH"),
            _ => format!("{root}{ending}"),
        },
        "lot" => match ending {
            "tu" => format!("{gun}tu"),
            "tAt" | "tAd" | "Di" => format!("{root}itAt"),
            "tAm" => format!("{root}itAm"),
            "antu" if abhyasta => format!("{root}atu"),
            "antu" => format!("{root}antu"),
            "tam" => format!("{root}itam"),
            "ta" => format!("{root}ita"),
            "Ani" => format!("{gun}Ani"),
            "Ava" => format!("{gun}Ava"),
            "Ama" => format!("{gun}Ama"),
            _ => format!("{root}{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" => format!("{gun}at"),
                "aH" => format!("{gun}aH"),
                "atAm" => format!("{root}itAm"),
                "an" if abhyasta => format!("{root}uH"),
                "an" => format!("{root}an"),
                "atam" => format!("{root}itam"),
                "ata" => format!("{root}ita"),
                "am" => format!("{gun}am"),
                "va" => format!("{root}iva"),
                "ma" => format!("{root}ima"),
                _ => format!("{root}{}", strip_a(ending)),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => format!("{root}{ending}"),
        "lrt" => {
            let sya = if anit {
                let j = jhal(&gun, "sya", false);
                if j.ends_with('a') { j } else { format!("{gun}sya") }
            } else {
                format!("{gun}izya")
            };
            thematic_join(&sya, ending)
        }
        _ => return None,
    };
    Some(form)
}

/// शास्: शास्ति, शिष्टः, शासनति (7.1.4); शाधि (6.4.35).
fn join_sas(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    if root != "SAs" {
        return None;
    }
    let weak = "Siz";
    Some(match family {
        "lat" => match ending {
            "ti" => "SAsti".into(),
            "si" => "SAssi".into(),
            "mi" | "Ami" => "SAsmi".into(),
            "anti" | "nti" => "SAsati".into(),
            "taH" => jhal(weak, "taH", false),
            "TaH" | "thaH" => jhal(weak, "TaH", false),
            "Ta" | "tha" => jhal(weak, "Ta", false),
            "vaH" | "AvaH" => format!("{weak}vaH"),
            "maH" | "AmaH" => format!("{weak}maH"),
            _ => format!("SAs{ending}"),
        },
        "lot" => match ending {
            "tu" => "SAstu".into(),
            "tAt" | "tAd" => jhal(weak, "tAt", false),
            "tAm" => jhal(weak, "tAm", false),
            "antu" => "SAsatu".into(),
            "Di" => "SADi".into(),
            "tam" => jhal(weak, "tam", false),
            "ta" => jhal(weak, "ta", false),
            "Ani" => "SAsAni".into(),
            "Ava" => "SAsAva".into(),
            "Ama" => "SAsAma".into(),
            _ => format!("SAs{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" | "aH" => "SAt".into(),
                "atAm" => jhal(weak, "tAm", false),
                "an" => "SAsuH".into(),
                "atam" => jhal(weak, "tam", false),
                "ata" => jhal(weak, "ta", false),
                "am" => "SAsam".into(),
                "va" => format!("{weak}va"),
                "ma" => format!("{weak}ma"),
                _ => format!("SAs{}", strip_a(ending)),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => format!("{weak}{ending}"),
        "lrt" => thematic_join("SAsizya", ending),
        _ => return None,
    })
}

/// वश्: वष्टि, उष्टः, उशन्ति (6.1.15).
fn join_vas(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    if root != "vaS" {
        return None;
    }
    let weak = "uS";
    Some(match family {
        "lat" => match ending {
            "ti" => jhal("vaS", "ti", false),
            "si" => jhal("vaS", "si", false),
            "mi" | "Ami" => "vaSmi".into(),
            "anti" | "nti" => format!("{weak}anti"),
            "taH" => jhal(weak, "taH", false),
            "TaH" | "thaH" => jhal(weak, "TaH", false),
            "Ta" | "tha" => jhal(weak, "Ta", false),
            "vaH" | "AvaH" => format!("{weak}vaH"),
            "maH" | "AmaH" => format!("{weak}maH"),
            _ => format!("vaS{ending}"),
        },
        "lot" => match ending {
            "tu" | "tAt" | "tAd" => jhal(weak, "tAt", false),
            "tAm" => jhal(weak, "tAm", false),
            "antu" => format!("{weak}antu"),
            "Di" => jhal(weak, "Di", false),
            "tam" => jhal(weak, "tam", false),
            "ta" => jhal(weak, "ta", false),
            "Ani" => "vaSAni".into(),
            "Ava" => "vaSAva".into(),
            "Ama" => "vaSAma".into(),
            _ => format!("vaS{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" | "aH" => padanta("vaS", false),
                "atAm" => jhal(weak, "tAm", false),
                "an" => format!("{weak}an"),
                "atam" => jhal(weak, "tam", false),
                "ata" => jhal(weak, "ta", false),
                "am" => "vaSam".into(),
                "va" => format!("{weak}va"),
                "ma" => format!("{weak}ma"),
                _ => format!("{weak}{}", strip_a(ending)),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => format!("{weak}{ending}"),
        "lrt" => thematic_join("vaSizya", ending),
        _ => return None,
    })
}

/// जागृ: जागर्ति, जागृतः, जाग्रति (7.1.4).
fn join_jagr(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    if root != "jAgf" {
        return None;
    }
    let gun = "jAgar";
    let weak = "jAgf";
    let r_weak = "jAgr";
    Some(match family {
        "lat" => match ending {
            "ti" => format!("{gun}ti"),
            "si" => format!("{gun}zi"),
            "mi" | "Ami" => format!("{gun}mi"),
            "anti" | "nti" => format!("{r_weak}ati"),
            "taH" => format!("{weak}taH"),
            "TaH" | "thaH" => format!("{weak}TaH"),
            "Ta" | "tha" => format!("{weak}Ta"),
            "vaH" | "AvaH" => format!("{weak}vaH"),
            "maH" | "AmaH" => format!("{weak}maH"),
            _ => format!("{weak}{ending}"),
        },
        "lot" => match ending {
            "tu" => format!("{gun}tu"),
            "tAt" | "tAd" | "Di" => format!("{weak}tAt"),
            "tAm" => format!("{weak}tAm"),
            "antu" => format!("{r_weak}atu"),
            "tam" => format!("{weak}tam"),
            "ta" => format!("{weak}ta"),
            "Ani" => format!("{gun}ARi"),
            "Ava" => format!("{gun}Ava"),
            "Ama" => format!("{gun}Ama"),
            _ => format!("{weak}{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" | "aH" => "jAgaH".into(),
                "atAm" => format!("{weak}tAm"),
                "an" => format!("{gun}uH"),
                "atam" => format!("{weak}tam"),
                "ata" => format!("{weak}ta"),
                "am" => format!("{gun}am"),
                "va" => format!("{weak}va"),
                "ma" => format!("{weak}ma"),
                _ => format!("{weak}{}", strip_a(ending)),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => format!("{weak}{ending}"),
        "lrt" => thematic_join("jAgarizya", ending),
        _ => return None,
    })
}

/// 6.4.101 हुझल्भ्यो हेर्धिः: झल् + हि → धि.
fn her_dhih(stem: &str) -> String {
    let body: String = stem.chars().rev().skip(1).collect::<String>().chars().rev().collect();
    format!("{body}Di")
}

/// अधि+इ (02.0042 इक्): इण् with अधि (6.1.77 यण्, 6.1.101 सवर्णदीर्घ).
fn join_adhi_i(dhatu: &str, root: &str, family: &str, ending: &str, _augment: Option<&str>) -> Option<String> {
    if root != "ik" && dhatu != "ik" && dhatu != "02.0042" {
        return None;
    }
    Some(match family {
        "lat" => match ending {
            "ti" => "aDyeti".into(),
            "si" => "aDyezi".into(),
            "mi" | "Ami" => "aDyemi".into(),
            "taH" => "aDItaH".into(),
            "TaH" | "thaH" => "aDITaH".into(),
            "Ta" | "tha" => "aDITa".into(),
            "anti" | "nti" => "aDiyanti".into(),
            "vaH" | "AvaH" => "aDIvaH".into(),
            "maH" | "AmaH" => "aDImaH".into(),
            _ => return None,
        },
        "lot" => match ending {
            "tu" => "aDyetu".into(),
            "tAt" | "tAd" | "Di" => "aDItAt".into(),
            "tAm" => "aDItAm".into(),
            "antu" => "aDiyantu".into(),
            "tam" => "aDItam".into(),
            "ta" => "aDIta".into(),
            "Ani" => "aDyayAni".into(),
            "Ava" => "aDyayAva".into(),
            "Ama" => "aDyayAma".into(),
            _ => return None,
        },
        "lang" => match ending {
            "at" | "ad" => "aDyEt".into(),
            "aH" => "aDyEH".into(),
            "atAm" => "aDyEtAm".into(),
            "an" => "aDyAyan".into(),
            "atam" => "aDyEtam".into(),
            "ata" => "aDyEta".into(),
            "am" => "aDyAyam".into(),
            "va" => "aDyEva".into(),
            "ma" => "aDyEma".into(),
            _ => return None,
        },
        "vidhilin" => match ending {
            "yAt" | "yAd" => "aDIyAt".into(),
            "yAtAm" => "aDIyAtAm".into(),
            "yuH" => "aDIyuH".into(),
            "yAH" => "aDIyAH".into(),
            "yAtam" => "aDIyAtam".into(),
            "yAta" => "aDIyAta".into(),
            "yAm" => "aDIyAm".into(),
            "yAva" => "aDIyAva".into(),
            "yAma" => "aDIyAma".into(),
            _ => format!("aDIy{}", ending.strip_prefix('y').unwrap_or(ending)),
        },
        "lrt" => thematic_join("aDyezya", ending),
        _ => return None,
    })
}

/// 2.4.54 चक्षिङः ख्याञ्: लृट् क्ष्यास्यति (ख्यास्यति वा). Present stays चष्टे.
fn join_cakz(root: &str, family: &str, ending: &str, _augment: Option<&str>) -> Option<String> {
    if root != "cakz" {
        return None;
    }
    match family {
        "lrt" => Some(thematic_join("kSAsya", ending)),
        "lat" => match ending {
            "te" => Some("cazwe".into()),
            _ => None,
        },
        _ => None,
    }
}

/// दरिद्रा (जक्षादि अभ्यस्त): दरिद्राति, दरिद्रितः, दरिद्रति (7.1.4); 6.4.64 इटि आ-लोप.
fn join_daridra(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    if root != "daridrA" {
        return None;
    }
    let strong = "daridrA";
    let weak = "daridri";
    let short = "daridr";
    let form = match family {
        "lat" => match ending {
            "ti" => format!("{strong}ti"),
            "si" => format!("{strong}si"),
            "mi" | "Ami" => format!("{strong}mi"),
            "taH" => format!("{weak}taH"),
            "TaH" | "thaH" => format!("{weak}TaH"),
            "Ta" | "tha" => format!("{weak}Ta"),
            "anti" | "nti" => format!("{short}ati"),
            "vaH" | "AvaH" => format!("{weak}vaH"),
            "maH" | "AmaH" => format!("{weak}maH"),
            _ => format!("{weak}{ending}"),
        },
        "lot" => match ending {
            "tu" => format!("{strong}tu"),
            "tAt" | "tAd" => format!("{weak}tAt"),
            "tAm" => format!("{weak}tAm"),
            "antu" => format!("{short}atu"),
            "Di" => format!("{weak}tAt"),
            "tam" => format!("{weak}tam"),
            "ta" => format!("{weak}ta"),
            "Ani" => format!("{strong}Ri"),
            "Ava" => format!("{strong}va"),
            "Ama" => format!("{strong}ma"),
            _ => format!("{weak}{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" => format!("{strong}t"),
                "aH" => format!("{strong}H"),
                "atAm" => format!("{weak}tAm"),
                "an" => format!("{short}uH"),
                "atam" => format!("{weak}tam"),
                "ata" => format!("{weak}ta"),
                "am" => format!("{strong}m"),
                "va" => format!("{weak}va"),
                "ma" => format!("{weak}ma"),
                _ => format!("{weak}{}", strip_a(ending)),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => format!("{weak}{ending}"),
        "lrt" => thematic_join(&format!("{short}izya"), ending),
        _ => return None,
    };
    Some(form)
}

/// चकास्: चकास्ति, चकासति (7.1.4); चकाधि (6.4.101); लङ् अचाकात्.
fn join_cakas(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    if root != "cakAs" {
        return None;
    }
    let form = match family {
        "lat" => match ending {
            "ti" => "cakAsti".into(),
            "si" => "cakAssi".into(),
            "mi" | "Ami" => "cakAsmi".into(),
            "taH" => "cakAstaH".into(),
            "TaH" | "thaH" => "cakAsTaH".into(),
            "Ta" | "tha" => "cakAsTa".into(),
            "anti" | "nti" => "cakAsati".into(),
            "vaH" | "AvaH" => "cakAsvaH".into(),
            "maH" | "AmaH" => "cakAsmaH".into(),
            _ => format!("cakAs{ending}"),
        },
        "lot" => match ending {
            "tu" => "cakAstu".into(),
            "tAt" | "tAd" => "cakAstAt".into(),
            "tAm" => "cakAstAm".into(),
            "antu" => "cakAsatu".into(),
            "Di" => her_dhih("cakAs"),
            "tam" => "cakAstam".into(),
            "ta" => "cakAsta".into(),
            "Ani" => "cakAsAni".into(),
            "Ava" => "cakAsAva".into(),
            "Ama" => "cakAsAma".into(),
            _ => format!("cakAs{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" | "aH" => "cakAt".into(),
                "atAm" => "cakAstAm".into(),
                "an" => "cakAsuH".into(),
                "atam" => "cakAstam".into(),
                "ata" => "cakAsta".into(),
                "am" => "cakAsam".into(),
                "va" => "cakAsva".into(),
                "ma" => "cakAsma".into(),
                _ => format!("cakAs{}", strip_a(ending)),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => format!("cakAs{ending}"),
        "lrt" => thematic_join("cakAsizya", ending),
        _ => return None,
    };
    Some(form)
}

/// षस् → सस्: सस्ति, सधि (6.4.101); लङ् असत्.
fn join_sasas(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    if root != "sas" {
        return None;
    }
    let form = match family {
        "lat" => match ending {
            "ti" => "sasti".into(),
            "si" => "sassi".into(),
            "mi" | "Ami" => "sasmi".into(),
            "taH" => "sastaH".into(),
            "TaH" | "thaH" => "sasTaH".into(),
            "Ta" | "tha" => "sasTa".into(),
            "anti" | "nti" => "sasanti".into(),
            "vaH" | "AvaH" => "sasvaH".into(),
            "maH" | "AmaH" => "sasmaH".into(),
            _ => format!("sas{ending}"),
        },
        "lot" => match ending {
            "tu" => "sastu".into(),
            "tAt" | "tAd" => "sastAt".into(),
            "tAm" => "sastAm".into(),
            "antu" => "sasantu".into(),
            "Di" => her_dhih("sas"),
            "tam" => "sastam".into(),
            "ta" => "sasta".into(),
            "Ani" => "sasAni".into(),
            "Ava" => "sasAva".into(),
            "Ama" => "sasAma".into(),
            _ => format!("sas{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" | "aH" => "sat".into(),
                "atAm" => "sastAm".into(),
                "an" => "sasan".into(),
                "atam" => "sastam".into(),
                "ata" => "sasta".into(),
                "am" => "sasam".into(),
                "va" => "sasva".into(),
                "ma" => "sasma".into(),
                _ => format!("sas{}", strip_a(ending)),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => format!("sas{ending}"),
        "lrt" => thematic_join("sasizya", ending),
        _ => return None,
    };
    Some(form)
}

/// षस्ति → संस्त्: संस्ति; लङ् असन् (संयोगान्तलोप).
fn join_samst(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    if root != "saMst" {
        return None;
    }
    let form = match family {
        "lat" => match ending {
            "ti" => "saMsti".into(),
            "si" => "saMstsi".into(),
            "mi" | "Ami" => "saMstmi".into(),
            "taH" => "saMstaH".into(),
            "TaH" | "thaH" => "saMstTaH".into(),
            "Ta" | "tha" => "saMstTa".into(),
            "anti" | "nti" => "saMstanti".into(),
            "vaH" | "AvaH" => "saMstvaH".into(),
            "maH" | "AmaH" => "saMstmaH".into(),
            _ => format!("saMst{ending}"),
        },
        "lot" => match ending {
            "tu" => "saMstu".into(),
            "tAt" | "tAd" | "Di" => "saMstAt".into(),
            "tAm" => "saMstAm".into(),
            "antu" => "saMstantu".into(),
            "tam" => "saMstam".into(),
            "ta" => "saMsta".into(),
            "Ani" => "saMstAni".into(),
            "Ava" => "saMstAva".into(),
            "Ama" => "saMstAma".into(),
            _ => format!("saMst{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" | "aH" => "san".into(),
                "atAm" => "saMstAm".into(),
                "an" => "saMstan".into(),
                "atam" => "saMstam".into(),
                "ata" => "saMsta".into(),
                "am" => "saMstam".into(),
                "va" => "saMstva".into(),
                "ma" => "saMstma".into(),
                _ => format!("saMst{}", strip_a(ending)),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => format!("saMst{ending}"),
        "lrt" => thematic_join("saMstizya", ending),
        _ => return None,
    };
    Some(form)
}

/// 7.3.93 ब्रुव ईट् — pit sārvadhātuka ईट् (ब्रवीति); weak ब्रू; 6.4.77 उवङ् (ब्रुवन्ति).
/// लृट् 2.4.53 ब्रुवो वचिः → वक्ष्यति.
fn join_bru(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    if root != "brU" {
        return None;
    }
    if family == "lrt" {
        return Some(thematic_join("vakzya", ending));
    }
    let form = match family {
        "lat" => match ending {
            "ti" => "bravIti".into(),
            "si" => "bravIzi".into(),
            "mi" | "Ami" => "bravImi".into(),
            "taH" => "brUtaH".into(),
            "thaH" | "TaH" => "brUTaH".into(),
            "tha" | "Ta" => "brUTa".into(),
            "anti" | "nti" => "bruvanti".into(),
            "vaH" | "AvaH" => "brUvaH".into(),
            "maH" | "AmaH" => "brUmaH".into(),
            _ => format!("brU{ending}"),
        },
        "lot" => match ending {
            "tu" => "bravItu".into(),
            "tAt" | "tAd" => "brUtAt".into(),
            "tAm" => "brUtAm".into(),
            "antu" => "bruvantu".into(),
            "Di" => "brUhi".into(),
            "tam" => "brUtam".into(),
            "ta" => "brUta".into(),
            "Ani" => "bravARi".into(),
            "Ava" => "bravAva".into(),
            "Ama" => "bravAma".into(),
            _ => format!("brU{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" => "bravIt".into(),
                "aH" => "bravIH".into(),
                "atAm" => "brUtAm".into(),
                "an" => "bruvan".into(),
                "atam" => "brUtam".into(),
                "ata" => "brUta".into(),
                "am" => "bravam".into(),
                "va" => "brUva".into(),
                "ma" => "brUma".into(),
                _ => format!("brU{}", strip_a(ending)),
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => match ending {
            "yAt" | "yAd" => "brUyAt".into(),
            "yAtAm" => "brUyAtAm".into(),
            "yuH" => "brUyuH".into(),
            "yAH" => "brUyAH".into(),
            "yAtam" => "brUyAtam".into(),
            "yAta" => "brUyAta".into(),
            "yAm" => "brUyAm".into(),
            "yAva" => "brUyAva".into(),
            "yAma" => "brUyAma".into(),
            _ => format!("brU{ending}"),
        },
        _ => return None,
    };
    Some(form)
}

/// 7.3.95 स्तुश्च — optional ईट् before सार्वधातुक (स्तवीति / स्तौति).
fn join_stu_it(family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    let form = match family {
        "lat" => match ending {
            "ti" => "stavIti".into(),
            "si" => "stavIzi".into(),
            "mi" | "Ami" => "stavImi".into(),
            "taH" => "stuvItaH".into(),
            "thaH" | "TaH" => "stuvITaH".into(),
            "tha" | "Ta" => "stuvITa".into(),
            "vaH" | "AvaH" => "stuvIvaH".into(),
            "maH" | "AmaH" => "stuvImaH".into(),
            _ => return None,
        },
        "lot" => match ending {
            "tu" => "stavItu".into(),
            "tAt" | "tAd" => "stuvItAt".into(),
            "tAm" => "stuvItAm".into(),
            "Di" => "stuvIhi".into(),
            "tam" => "stuvItam".into(),
            "ta" => "stuvIta".into(),
            _ => return None,
        },
        "lang" => {
            let inner = match ending {
                "at" | "ad" => "stavIt".into(),
                "aH" => "stavIH".into(),
                "atAm" => "stuvItAm".into(),
                "atam" => "stuvItam".into(),
                "ata" => "stuvIta".into(),
                "va" => "stuvIva".into(),
                "ma" => "stuvIma".into(),
                _ => return None,
            };
            return Some(apply_aug(inner, family, augment));
        }
        "vidhilin" => {
            if !ending.starts_with('y') {
                return None;
            }
            format!("stuvI{ending}")
        }
        _ => return None,
    };
    Some(form)
}

fn join_stu(family: &str, ending: &str, augment: Option<&str>) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(f) = join_stu_it(family, ending, augment) {
        out.push(apply_natva_to_word(&f));
    }
    if let Some(f) = join_u("stu", family, ending, augment) {
        let f = apply_natva_to_word(&f);
        if !out.contains(&f) {
            out.push(f);
        }
    }
    out
}

/// 6.4.98 गमहनजनखनघसां लोपः क्ङिति before अपित् झल् (हतः);
/// झि अन्ति/अन्तु/अन्: 6.4.98 + 7.3.54 हो हन्तेः → घ् (घ्नन्ति).
/// यासुट् keeps न् (हन्यात्). पित् keeps हन् (हन्ति, हनानि).
fn han_anga(family: &str, ending: &str) -> &'static str {
    if family == "vidhilin" || pit(family, ending) {
        return "han";
    }
    if matches!(ending, "anti" | "nti" | "antu" | "an") {
        return "Gn";
    }
    let tin = if family == "lang" { strip_a(ending) } else { ending };
    if tin.chars().next().is_some_and(|c| matches!(c, 't' | 'T' | 'd' | 'D')) {
        return "ha";
    }
    "han"
}

/// हन्: 6.4.98 / 7.3.54; 8.3.24 हंसि; 6.4.36 हन्तेर्जः जहि; 6.1.68 अहन्.
fn join_han(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    if root != "han" {
        return None;
    }
    if family == "lrt" {
        return Some(thematic_join(&crate::engine::it::sya_stem("han"), ending));
    }
    if family == "vidhilin" {
        return Some(format!("han{ending}"));
    }
    if ending == "Di" {
        return Some("jahi".into());
    }
    if family == "lang" && matches!(ending, "at" | "ad" | "aH") {
        return Some(apply_aug("han".into(), family, augment));
    }
    let anga = han_anga(family, ending);
    let tin = match ending {
        "nti" => "anti",
        "an" => "an",
        _ if family == "lang" => strip_a(ending),
        other => other,
    };
    let inner = if anga == "han" && tin.starts_with('s') {
        format!("haM{tin}")
    } else {
        format!("{anga}{tin}")
    };
    Some(apply_aug(inner, family, augment))
}

/// 6.4.111 श्नसोरल्लोपः — अ of अस् drops before अपित्.
fn as_anga(family: &str, ending: &str) -> &'static str {
    if pit(family, ending) {
        "as"
    } else {
        "s"
    }
}

/// अस्: 6.4.111; 8.4.65 असि; 7.3.96 आसीत्; 6.4.119+6.4.101 एधि; लृट् 2.4.52 भू.
fn join_as(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    if root != "as" {
        return None;
    }
    if family == "lrt" {
        return Some(thematic_join(&crate::engine::it::sya_stem("as"), ending));
    }
    if ending == "Di" {
        return Some("eDi".into());
    }
    if family == "lang" {
        match ending {
            "at" => return Some(apply_aug("asIt".into(), family, augment)),
            "ad" => return Some(apply_aug("asId".into(), family, augment)),
            "aH" => return Some(apply_aug("asIH".into(), family, augment)),
            _ => {}
        }
    }
    let stem = as_anga(family, ending);
    let tin = match ending {
        "an" | "am" => ending,
        _ if family == "lang" => strip_a(ending),
        other => other,
    };
    let inner = if stem == "as" && tin.starts_with('s') {
        format!("a{tin}")
    } else {
        format!("{stem}{tin}")
    };
    if family == "lang" && inner.starts_with('s') {
        // 6.4.72 आट्; 1.1.56 स्थानिवत् after 6.4.111.
        Some(format!("A{inner}"))
    } else {
        Some(apply_aug(inner, family, augment))
    }
}

/// इण्: 7.3.84 गुण; 6.4.81 इणो यण् (यन्ति, not इयङ्); 6.1.78 अयानि; 6.4.72 आट् आयन्.
fn join_in(root: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    if root != "i" {
        return None;
    }
    let strong = "e";
    let weak = "i";
    let form = match family {
        "lrt" => thematic_join(&crate::engine::it::sya_stem("i"), ending),
        "vidhilin" => format!("{weak}{ending}"),
        "lat" => match ending {
            "ti" => format!("{strong}ti"),
            "si" => format!("{strong}zi"),
            "mi" | "Ami" => format!("{strong}mi"),
            "anti" | "nti" => "yanti".into(),
            _ => format!("{weak}{ending}"),
        },
        "lot" => match ending {
            "tu" => format!("{strong}tu"),
            "antu" => "yantu".into(),
            "Di" => format!("{weak}hi"),
            "Ani" => crate::engine::it::join_eco(strong, "Ani"),
            "Ava" => crate::engine::it::join_eco(strong, "Ava"),
            "Ama" => crate::engine::it::join_eco(strong, "Ama"),
            _ => format!("{weak}{ending}"),
        },
        "lang" => {
            let inner = match ending {
                "at" => format!("{strong}t"),
                "ad" => format!("{strong}d"),
                "aH" => format!("{strong}H"),
                "an" => "yan".into(),
                "am" => crate::engine::it::join_eco(strong, "am"),
                "atAm" => format!("{weak}tAm"),
                "atam" => format!("{weak}tam"),
                "ata" => format!("{weak}ta"),
                "va" => format!("{weak}va"),
                "ma" => format!("{weak}ma"),
                _ => format!("{weak}{}", strip_a(ending)),
            };
            if inner.starts_with('y') {
                format!("A{inner}")
            } else {
                apply_aug(inner, family, augment)
            }
        }
        _ => return None,
    };
    Some(form)
}

/// Full surface form for गण 2, or `None` to fall through (अद्).
pub fn join_form(
    dhatu: &str,
    family: &str,
    ending: &str,
    purusha: u8,
    vacana: u8,
    augment: Option<&str>,
) -> Option<String> {
    join_forms(dhatu, family, ending, purusha, vacana, augment)
        .into_iter()
        .next()
}

/// 7.3.95 may yield two forms (स्तवीति, स्तौति).
pub fn join_forms(
    dhatu: &str,
    family: &str,
    ending: &str,
    _purusha: u8,
    _vacana: u8,
    augment: Option<&str>,
) -> Vec<String> {
    if !matches!(family, "lat" | "lot" | "lang" | "vidhilin" | "lrt") {
        return vec![];
    }
    let r = root_of(dhatu);
    if r.is_empty() || r == "ad" {
        return vec![];
    }
    if r == "stu" {
        return join_stu(family, ending, augment);
    }
    if let Some(f) = join_han(&r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_as(&r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_in(&r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_adhi_i(dhatu, &r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_cakz(&r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_daridra(&r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_cakas(&r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_sasas(&r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_samst(&r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_vid(&r, family, ending, _purusha, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_rudadi(&r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_sas(&r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_vas(&r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_jagr(&r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
    }
    if let Some(f) = join_bru(&r, family, ending, augment) {
        return vec![apply_natva_to_word(&f)];
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
    form.map(|f| vec![apply_natva_to_word(&f)]).unwrap_or_default()
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
        assert_eq!(join_form("Ru", "lat", "ti", 1, 1, None).as_deref(), Some("nOti"));
        assert_eq!(join_form("zRu", "lat", "ti", 1, 1, None).as_deref(), Some("snOti"));
        assert_eq!(join_form("brUY", "lat", "ti", 1, 1, None).as_deref(), Some("bravIti"));
        assert_eq!(join_form("brUY", "lrt", "ti", 1, 1, None).as_deref(), Some("vakzyati"));
        let stu = join_forms("zwuY", "lat", "ti", 1, 1, None);
        assert!(stu.iter().any(|x| x == "stavIti"), "{:?}", stu);
        assert!(stu.iter().any(|x| x == "stOti"), "{:?}", stu);
        assert_eq!(join_form("zwuY", "lrt", "ti", 1, 1, None).as_deref(), Some("stozyati"));
        assert_eq!(join_form("yu", "lrt", "ti", 1, 1, None).as_deref(), Some("yavizyati"));
        assert_eq!(join_form("yu", "lot", "Ani", 3, 1, None).as_deref(), Some("yavAni"));
        assert_eq!(join_form("liha", "lat", "taH", 1, 2, None).as_deref(), Some("lIQaH"));
        assert_eq!(join_form("dviza", "lot", "Ani", 3, 1, None).as_deref(), Some("dvezARi"));
        assert_eq!(join_form("vida", "lat", "ti", 1, 1, None).as_deref(), Some("vetti"));
        assert_eq!(join_form("vida", "lat", "si", 2, 1, None).as_deref(), Some("vetTa"));
        assert_eq!(join_form("rudir", "lat", "ti", 1, 1, None).as_deref(), Some("roditi"));
        assert_eq!(join_form("Svasa", "lat", "ti", 1, 1, None).as_deref(), Some("Svasiti"));
        assert_eq!(join_form("SAsu", "lat", "ti", 1, 1, None).as_deref(), Some("SAsti"));
        assert_eq!(join_form("SAsu", "lot", "Di", 2, 1, None).as_deref(), Some("SADi"));
        assert_eq!(join_form("vaSa", "lat", "ti", 1, 1, None).as_deref(), Some("vazwi"));
        assert_eq!(join_form("jAgf", "lat", "ti", 1, 1, None).as_deref(), Some("jAgarti"));
        assert_eq!(join_form("jakza", "lat", "anti", 1, 3, None).as_deref(), Some("jakzati"));
        assert_eq!(join_form("ik", "lat", "ti", 1, 1, None).as_deref(), Some("aDyeti"));
        assert_eq!(join_form("ik", "lat", "taH", 1, 2, None).as_deref(), Some("aDItaH"));
        assert_eq!(join_form("ik", "lat", "anti", 1, 3, None).as_deref(), Some("aDiyanti"));
        assert_eq!(join_form("ik", "lrt", "ti", 1, 1, None).as_deref(), Some("aDyezyati"));
        assert_eq!(join_form("ik", "lang", "at", 1, 1, None).as_deref(), Some("aDyEt"));
        assert_eq!(join_form("daridrA", "lat", "ti", 1, 1, None).as_deref(), Some("daridrAti"));
        assert_eq!(join_form("daridrA", "lat", "anti", 1, 3, None).as_deref(), Some("daridrati"));
        assert_eq!(join_form("daridrA", "lrt", "ti", 1, 1, None).as_deref(), Some("daridrizyati"));
        assert_eq!(join_form("cakAsf", "lat", "ti", 1, 1, None).as_deref(), Some("cakAsti"));
        assert_eq!(join_form("cakAsf", "lot", "Di", 2, 1, None).as_deref(), Some("cakADi"));
        assert_eq!(join_form("zasa", "lat", "ti", 1, 1, None).as_deref(), Some("sasti"));
        assert_eq!(join_form("zasa", "lot", "Di", 2, 1, None).as_deref(), Some("saDi"));
        assert_eq!(join_form("zasti", "lat", "ti", 1, 1, None).as_deref(), Some("saMsti"));
        assert_eq!(join_form("cakziN", "lrt", "ti", 1, 1, None).as_deref(), Some("kSAsyati"));
        assert_eq!(join_form("cakziN", "lat", "te", 1, 1, None).as_deref(), Some("cazwe"));
        assert_eq!(join_form("hana", "lat", "ti", 1, 1, None).as_deref(), Some("hanti"));
        assert_eq!(join_form("hana", "lat", "anti", 1, 3, None).as_deref(), Some("Gnanti"));
        assert_eq!(join_form("hana", "lat", "si", 2, 1, None).as_deref(), Some("haMsi"));
        assert_eq!(join_form("hana", "lot", "Di", 2, 1, None).as_deref(), Some("jahi"));
        assert_eq!(join_form("hana", "lang", "at", 1, 1, None).as_deref(), Some("ahan"));
        assert_eq!(join_form("hana", "lang", "an", 1, 3, None).as_deref(), Some("aGnan"));
        assert_eq!(join_form("hana", "vidhilin", "yAt", 1, 1, None).as_deref(), Some("hanyAt"));
        assert_eq!(join_form("hana", "lrt", "ti", 1, 1, None).as_deref(), Some("hanizyati"));
        assert_eq!(join_form("asa", "lat", "ti", 1, 1, None).as_deref(), Some("asti"));
        assert_eq!(join_form("asa", "lat", "anti", 1, 3, None).as_deref(), Some("santi"));
        assert_eq!(join_form("asa", "lot", "Di", 2, 1, None).as_deref(), Some("eDi"));
        assert_eq!(join_form("asa", "lang", "at", 1, 1, None).as_deref(), Some("AsIt"));
        assert_eq!(join_form("asa", "lang", "atAm", 1, 2, None).as_deref(), Some("AstAm"));
        assert_eq!(join_form("asa", "lang", "an", 1, 3, None).as_deref(), Some("Asan"));
        assert_eq!(join_form("asa", "vidhilin", "yAt", 1, 1, None).as_deref(), Some("syAt"));
        assert_eq!(join_form("asa", "lrt", "ti", 1, 1, None).as_deref(), Some("Bavizyati"));
        assert_eq!(join_form("iR", "lat", "ti", 1, 1, None).as_deref(), Some("eti"));
        assert_eq!(join_form("iR", "lat", "anti", 1, 3, None).as_deref(), Some("yanti"));
        assert_eq!(join_form("iR", "lot", "Ani", 3, 1, None).as_deref(), Some("ayAni"));
        assert_eq!(join_form("iR", "lang", "at", 1, 1, None).as_deref(), Some("Et"));
        assert_eq!(join_form("iR", "lang", "an", 1, 3, None).as_deref(), Some("Ayan"));
        assert_eq!(join_form("iR", "vidhilin", "yAt", 1, 1, None).as_deref(), Some("iyAt"));
        assert_eq!(join_form("iR", "lrt", "ti", 1, 1, None).as_deref(), Some("ezyati"));
    }
}
