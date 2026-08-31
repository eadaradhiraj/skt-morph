//! Simplified port of engine/join.py
//! Handles thematic join + augment + gana-specific overrides via stubs.
//! Full 1874 LOC will be expanded, but this covers lat/lot/lrt/lang/vidhilin for gana 1,4,6 shuddha.

use crate::engine::phonology::thematic_join;

/// धिवि (01.0677): श्नु-like o/u/v (धिनोति, धिन्वन्ति, धिनविष्यति).
fn join_divi(family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    let inner = match family {
        "lat" => match ending {
            "ti" => "Dinoti".into(),
            "taH" => "DinutaH".into(),
            "nti" | "anti" => "Dinvanti".into(),
            "si" | "zi" => "Dinozi".into(),
            "TaH" | "thaH" => "DinuTaH".into(),
            "Ta" | "tha" => "DinuTa".into(),
            "mi" | "Ami" => "Dinomi".into(),
            "vaH" | "AvaH" => "DinuvaH".into(),
            "maH" | "AmaH" => "DinumaH".into(),
            _ => return None,
        },
        "lot" => match ending {
            "tu" | "otu" => "Dinotu".into(),
            "tAt" | "utAt" => "DinutAt".into(),
            "tAd" | "utAd" => "DinutAd".into(),
            "tAm" | "utAm" => "DinutAm".into(),
            "antu" | "vantu" => "Dinvantu".into(),
            "" | "u" => "Dinu".into(),
            "tam" | "utam" => "Dinutam".into(),
            "ta" | "uta" => "Dinuta".into(),
            "Ani" | "avAni" => "DinavAni".into(),
            "Ava" | "avAva" => "DinavAva".into(),
            "Ama" | "avAma" => "DinavAma".into(),
            _ => return None,
        },
        "lang" => match ending {
            "at" | "ot" => "Dinot".into(),
            "ad" | "od" => "Dinod".into(),
            "atAm" | "utAm" => "DinutAm".into(),
            "an" | "van" => "Dinvan".into(),
            "aH" | "oH" => "DinoH".into(),
            "atam" | "utam" => "Dinutam".into(),
            "ata" | "uta" => "Dinuta".into(),
            "am" | "avam" => "Dinavam".into(),
            "Ava" | "uva" | "va" => "Dinuva".into(),
            "Ama" | "uma" | "ma" => "Dinuma".into(),
            _ => return None,
        },
        "vidhilin" => match ending {
            "et" | "At" | "yAt" => "DinuyAt".into(),
            "ed" | "Ad" | "yAd" => "DinuyAd".into(),
            "etAm" | "AtAm" | "yAtAm" => "DinuyAtAm".into(),
            "eyuH" | "uH" | "yuH" => "DinuyuH".into(),
            "eH" | "AH" | "yAH" => "DinuyAH".into(),
            "etam" | "Atam" | "yAtam" => "DinuyAtam".into(),
            "eta" | "Ata" | "yAta" => "DinuyAta".into(),
            "eyam" | "Am" | "yAm" => "DinuyAm".into(),
            "eva" | "Ava" | "yAva" => "DinuyAva".into(),
            "ema" | "Ama" | "yAma" => "DinuyAma".into(),
            _ => return None,
        },
        "lrt" => thematic_join("Dinvizya", ending),
        _ => return None,
    };
    Some(apply_lang_aug(inner, family, augment))
}

fn apply_lang_aug(form: String, family: &str, augment: Option<&str>) -> String {
    if family != "lang" {
        return form;
    }
    let Some(aug) = augment else {
        return format!("a{form}");
    };
    if aug != "a" {
        return format!("{aug}{form}");
    }
    match form.chars().next() {
        Some('a') | Some('A') => format!("A{}", &form[1..]),
        Some('i') | Some('I') | Some('e') | Some('E') => format!("E{}", &form[1..]),
        Some('u') | Some('U') | Some('o') | Some('O') => format!("O{}", &form[1..]),
        Some('f') | Some('F') => format!("Ar{}", &form[1..]),
        _ => format!("a{form}"),
    }
}

pub fn internal_sandhi(stem: &str, suffix: &str) -> String {
    if stem.is_empty() || suffix.is_empty() { return format!("{}{}", stem, suffix); }
    let suff_first = suffix.chars().next().unwrap();
    if "aAiIuUfFeEoO".contains(suff_first) { return format!("{}{}", stem, suffix); }
    let stem_chars: Vec<char> = stem.chars().collect();
    let s_last = *stem_chars.last().unwrap();
    let stem_body: String = stem_chars[..stem_chars.len()-1].iter().collect();
    match (s_last, suff_first) {
        ('d', 't') => format!("{}t{}", stem_body, suffix),
        ('d', 'T') => format!("{}tT{}", stem_body, &suffix[1..]),
        ('d', 's') | ('t', 's') => format!("{}ts{}", stem_body, &suffix[1..]),
        ('D', 't') => format!("{}dD{}", stem_body, &suffix[1..]),
        ('D', 'T') => format!("{}dD{}", stem_body, &suffix[1..]),
        ('D', 's') => format!("{}ts{}", stem_body, &suffix[1..]),
        ('c', 't') | ('j', 't') => format!("{}kt{}", stem_body, &suffix[1..]),
        ('c', 'T') | ('j', 'T') => format!("{}kT{}", stem_body, &suffix[1..]),
        ('c', 's') | ('j', 's') | ('S', 's') => format!("{}kz{}", stem_body, &suffix[1..]),
        ('z', 't') | ('S', 't') => format!("{}zw{}", stem_body, &suffix[1..]),
        ('z', 'T') | ('S', 'T') => format!("{}zW{}", stem_body, &suffix[1..]),
        ('h', 't') => format!("{}gD{}", stem_body, &suffix[1..]),
        ('h', 'T') => format!("{}gD{}", stem_body, &suffix[1..]),
        ('h', 's') => format!("{}kz{}", stem_body, &suffix[1..]),
        ('B', 't') => format!("{}bD{}", stem_body, &suffix[1..]),
        ('B', 'T') => format!("{}bD{}", stem_body, &suffix[1..]),
        // 8.3.23 मोऽनुस्वारः + 8.4.58 परसवर्णः — गन्तव्य, गन्तुम्
        ('m', 't') => format!("{}nt{}", stem_body, &suffix[1..]),
        ('m', 'T') => format!("{}nT{}", stem_body, &suffix[1..]),
        _ => format!("{}{}", stem, suffix),
    }
}

pub fn join_form(
    stem: &str,
    ending: &str,
    gana: u8,
    family: &str,
    purusha: u8,
    _pada: &str,
    augment: Option<&str>,
    dhatu: Option<&str>,
    _vacana: u8,
    _antarganas: Option<&str>,
) -> String {
    // AD (2,3) — अत्ति, अस्ति, हन्ति, एति, ब्रवीति, वक्ति
    if gana == 2 {
        if let Some(d) = dhatu {
            if let Some(f) = crate::engine::adadi::join_form(d, family, ending, purusha, _vacana, augment) {
                return crate::engine::phonology::apply_natva_to_word(&f);
            }
        }
    }
    if gana == 2 || gana == 3 {
        if let Some(d) = dhatu {
            let d = d.trim_end_matches('a');
            if d == "han" {
                if family == "lat" {
                    match ending {
                        "ti" => return "hanti".into(),
                        "taH" => return "hataH".into(),
                        "nti" | "anti" => return "Gnanti".into(),
                        "si" => return "haMsi".into(),
                        "thaH" | "TaH" => return "haTaH".into(),
                        "tha" | "Ta" => return "haTa".into(),
                        "mi" | "Ami" => return "hanmi".into(),
                        "vaH" | "AvaH" => return "hanvaH".into(),
                        "maH" | "AmaH" => return "hanmaH".into(),
                        _ => {}
                    }
                }
                if family == "lot" {
                    match ending {
                        "tu" => return "hantu".into(),
                        "tAt" => return "hatAt".into(),
                        "tAd" => return "hatAd".into(),
                        "tAm" => return "hatAm".into(),
                        "tam" => return "hatam".into(),
                        "ta" => return "hata".into(),
                        "antu" => return "Gnantu".into(),
                        "Di" => return "jahi".into(),
                        _ => {}
                    }
                }
                if family == "lang" {
                    return match ending {
                        "at" | "ad" => "ahan".into(),
                        "an" => "aGnan".into(),
                        "atAm" => "ahatAm".into(),
                        "aH" => "ahan".into(),
                        "atam" => "ahatam".into(),
                        "ata" => "ahata".into(),
                        "am" => "ahanam".into(),
                        "va" => "ahanva".into(),
                        "ma" => "ahanma".into(),
                        _ => format!("ahan{ending}"),
                    };
                }
                if family == "vidhilin" && ending.starts_with('y') {
                    return format!("han{ending}");
                }
            }
            if d == "vid" && family == "lat" {
                match (ending, purusha) {
                    ("ti", 1) => return "vetti".into(),
                    ("taH", 1) => return "vittaH".into(),
                    ("si", 2) => return "vetTa".into(),
                    _ => {}
                }
            }
            if d == "as" {
                if family == "lat" {
                    return match ending {
                        "ti" => "asti".into(),
                        "taH" => "staH".into(),
                        "anti" | "nti" => "santi".into(),
                        "si" => "asi".into(),
                        "thaH" | "TaH" => "sTaH".into(),
                        "tha" | "Ta" => "sTa".into(),
                        "mi" | "Ami" => "asmi".into(),
                        "vaH" | "AvaH" => "svaH".into(),
                        "maH" | "AmaH" => "smaH".into(),
                        _ => format!("as{ending}"),
                    };
                }
                if family == "lang" {
                    return match ending {
                        "at" | "ad" => "AsIt".into(),
                        "atAm" => "AstAm".into(),
                        "an" => "Asan".into(),
                        "aH" => "AsIH".into(),
                        "atam" => "Astam".into(),
                        "ata" => "Asta".into(),
                        "am" => "Asam".into(),
                        "va" => "Asva".into(),
                        "ma" => "Asma".into(),
                        _ => format!("As{ending}"),
                    };
                }
                if family == "vidhilin" {
                    return match ending {
                        "yAt" => "syAt".into(),
                        "yAd" => "syAd".into(),
                        "yAtAm" => "syAtAm".into(),
                        "yuH" => "syuH".into(),
                        "yAH" => "syAH".into(),
                        "yAtam" => "syAtam".into(),
                        "yAta" => "syAta".into(),
                        "yAm" => "syAm".into(),
                        "yAva" => "syAva".into(),
                        "yAma" => "syAma".into(),
                        _ => format!("s{ending}"),
                    };
                }
            }
            if d == "ad" {
                if family == "lang" {
                    return match ending {
                        "at" | "ad" => "Adat".into(),
                        "aH" => "AdaH".into(),
                        "atAm" => "AttAm".into(),
                        "atam" => "Attam".into(),
                        "ata" => "Atta".into(),
                        "an" => "Adan".into(),
                        "am" => "Adam".into(),
                        "va" => "Adva".into(),
                        "ma" => "Adma".into(),
                        _ => format!("Ad{}", ending.trim_start_matches('a')),
                    };
                }
                if family == "vidhilin" && ending.starts_with('y') {
                    return format!("ad{ending}");
                }
                if family == "lrt" {
                    return crate::engine::phonology::thematic_join("atsya", ending);
                }
                return internal_sandhi("ad", ending);
            }
            if d == "vac" {
                match ending {
                    "ti" => return "vakti".into(),
                    "taH" => return "vaktaH".into(),
                    "si" => return "vakzi".into(),
                    "anti" | "nti" => return "vacanti".into(),
                    _ => {}
                }
            }
            if d == "brU" || d == "bravI" || stem.ends_with("bravI") {
                match ending {
                    "ti" => return "bravIti".into(),
                    "taH" => return "brUtaH".into(),
                    "anti" | "nti" => return "bruvanti".into(),
                    "si" => return "bravIzi".into(),
                    "mi" | "Ami" => return "bravImi".into(),
                    _ => {}
                }
            }
            if d == "i" || d == "iR" || stem == "e" {
                match ending {
                    "ti" => return "eti".into(),
                    "taH" => return "itaH".into(),
                    "anti" | "nti" => return "yanti".into(),
                    "si" => return "ezi".into(),
                    "mi" | "Ami" => return "emi".into(),
                    "vaH" => return "ivaH".into(),
                    "maH" => return "imaH".into(),
                    _ => {}
                }
            }
        }
    }
    // धिवि (01.0677) — श्नु-like o/u/v for all sārvadhātuka / लृट्
    if dhatu == Some("Divi") && gana == 1 {
        if let Some(f) = join_divi(family, ending, augment) {
            return crate::engine::phonology::apply_natva_to_word(&f);
        }
    }
    // G3 (3) reduplicated – juhu→juhoti, bibhI→bibheti, pF→piparti
    if gana == 3 {
        if stem.ends_with("Ur") && ending == "ti" {
            return format!("{}arti", &stem[..stem.len()-2]); // pipUr+ti→piparti
        }
        if stem.ends_with('u') {
            let base = &stem[..stem.len()-1];
            match ending {
                "ti" => return format!("{}oti", base),
                "taH" => return format!("{}utaH", base),
                "nti" | "anti" => return format!("{}vati", base),
                "si" => return format!("{}oSi", base),
                "mi" => return format!("{}omi", base),
                "tu" => return format!("{}otu", base),
                "ot" | "od" => return format!("{}o{}", base, &ending[1..]),
                "utAm" => return format!("{}utAm", base),
                "van" => return format!("{}van", base),
                "At" | "Ad" => return format!("{}uyA{}", base, &ending[1..]),
                "yAt" | "yAd" => return format!("{}uyA{}", base, &ending[1..]),
                _ => {}
            }
        }
        if stem.ends_with('y') && (family == "vidhilin" || ending.starts_with('A')) {
            return format!("{}{}", stem, ending);
        }
        if stem.ends_with('I') {
            let base = &stem[..stem.len()-1];
            match ending {
                "ti" => return format!("{}eti", base), // bibhI→bibheti
                "taH" => return format!("{}ItaH", base),
                _ => {}
            }
        }
    }
    // NU gaṇa (5,8) – port of _join_nu (lat/lot/lrt core)
    if gana == 5 || gana == 8 {
        if family == "lrt" {
            // future stems already like "to" etc, simple concat
            if ending.is_empty() { return stem.to_string(); }
            return format!("{}{}", stem, ending);
        }
        if stem.ends_with('u') {
            let base = &stem[..stem.len()-1];
            match ending {
                "ti" => return format!("{}oti", base),
                "taH" => return format!("{}taH", stem),
                "nti" | "anti" => return format!("{}vanti", base),
                "si" => return format!("{}oSi", base), // slp1 S for z
                "thaH" | "TaH" => return format!("{}{}", stem, ending),
                "tha" | "Ta" => return format!("{}{}", stem, ending),
                "mi" => return format!("{}omi", base),
                "vaH" => return format!("{}vaH", stem),
                "maH" => return format!("{}maH", stem),
                "tu" => return format!("{}otu", base),
                "ot" | "od" => return format!("{}o{}", base, &ending[1..]),
                "At" | "Ad" => return format!("{}uyA{}", base, &ending[1..]),
                "tAm" => return format!("{}tAm", stem),
                "antu" | "vantu" => return format!("{}vantu", base),
                "Ani" => return format!("{}avAni", base),
                "utAt" | "utAd" => return format!("{}{}", base, ending),
                "utAm" => return format!("{}{}", base, ending),
                "utam" => return format!("{}{}", base, ending),
                "uta" => return format!("{}{}", base, ending),
                "u" => return format!("{}u", base),
                _ => {
                    if ending.starts_with('u') {
                        return format!("{}{}", base, ending);
                    }
                }
            }
        }
        if stem.ends_with('R') {
            match ending {
                "ti" => return format!("{}oti", stem),
                "taH" => return format!("{}taH", stem),
                "nti" | "anti" => return format!("{}anti", stem),
                "si" => return format!("{}oSi", stem),
                "mi" => return format!("{}omi", stem),
                _ => {}
            }
        }
    }
    // N (7) श्नम्: रुणद्धि, रुन्द्धः, रुन्धन्ति
    if gana == 7 {
        if family == "lrt" {
            return format!("{}{}", stem, ending);
        }
        let raw = dhatu.unwrap_or("");
        let root = if raw.ends_with("ir") && raw.len() > 3 {
            &raw[..raw.len() - 2]
        } else if raw.ends_with('a') && raw.len() > 2 {
            &raw[..raw.len() - 1]
        } else {
            raw
        };
        if !root.is_empty() {
            let chars: Vec<char> = root.chars().collect();
            if chars.len() >= 2 {
                let last = chars[chars.len() - 1];
                let body: String = chars[..chars.len() - 1].iter().collect();
                let strong = format!("{body}Ra{last}");
                let weak = format!("{body}n{last}");
                let pit = matches!(ending, "ti" | "si" | "mi" | "Ami" | "tu" | "tAt" | "tAd");
                let base = if pit { &strong } else { &weak };
                match ending {
                    "ti" => return internal_sandhi(&strong, "ti"),
                    "si" => return internal_sandhi(&strong, "si"),
                    "mi" | "Ami" => return format!("{strong}mi"),
                    "taH" | "TaH" => return internal_sandhi(&weak, "taH"),
                    "nti" | "anti" => return format!("{weak}anti"),
                    "thaH" => return internal_sandhi(&weak, "TaH"),
                    "tha" | "Ta" => return internal_sandhi(&weak, "Ta"),
                    "vaH" | "AvaH" => return internal_sandhi(&weak, "vaH"),
                    "maH" | "AmaH" => return internal_sandhi(&weak, "maH"),
                    "tu" => return internal_sandhi(&strong, "tu"),
                    "antu" => return format!("{weak}antu"),
                    "tAm" => return internal_sandhi(&weak, "tAm"),
                    "yAt" | "yAd" => return format!("{weak}yA{}", &ending[1..]),
                    "at" | "ad" if family == "lang" => {
                        let body: String = strong.chars().take(strong.chars().count().saturating_sub(1)).collect();
                        return format!("{}{}t", augment.unwrap_or(""), body);
                    }
                    _ => {
                        return internal_sandhi(base, ending);
                    }
                }
            }
        }
    }
    // NI (9) punāti / krIRAti – handle nA → RA / nA
    if gana == 9 && stem.ends_with("nA") {
        if family == "lrt" {
            return format!("{}{}", stem, ending);
        }
        let base = &stem[..stem.len()-2];
        let use_n = crate::engine::phonology::g9_uses_n_infix(dhatu.unwrap_or(""), _antarganas.unwrap_or(""));
        let nasal = if use_n { "n" } else { "R" };
        match ending {
            "ti" => return format!("{}{}Ati", base, nasal),
            "taH" => return format!("{}{}ItaH", base, nasal),
            "anti" | "nti" => return format!("{}{}anti", base, nasal),
            "si" => return format!("{}{}Asi", base, nasal),
            "mi" => return format!("{}{}Ami", base, nasal),
            "vaH" => return format!("{}{}IvaH", base, nasal),
            "maH" => return format!("{}{}ImaH", base, nasal),
            "yAt" | "yAd" => return format!("{}{}IyA{}", base, nasal, &ending[1..]),
            "At" | "Ad" => {
                let inner = format!("{}{}A{}", base, nasal, &ending[1..]);
                if family == "lang" {
                    if let Some(aug) = augment {
                        return format!("{aug}{inner}");
                    }
                }
                return inner;
            }
            "tu" => return format!("{}{}Atu", base, nasal),
            _ => {}
        }
    }
    // Core join - upgraded
    let mut form = if (stem.ends_with('a') || stem.ends_with('A')) && !ending.is_empty() {
        thematic_join(stem, ending)
    } else {
        internal_sandhi(stem, ending)
    };

    // augment handling (a- for lang) with proper Pāṇini vowel sandhi
    if let Some(aug) = augment {
        if aug == "a" && (form.starts_with('a') || form.starts_with('A')) {
            form = format!("A{}", &form[1..]);
        } else if aug == "a" && (form.starts_with('i') || form.starts_with('I') || form.starts_with('e') || form.starts_with('E')) {
            form = format!("E{}", &form[1..]);
        } else if aug == "a" && (form.starts_with('u') || form.starts_with('U') || form.starts_with('o') || form.starts_with('O')) {
            form = format!("O{}", &form[1..]);
        } else if aug == "a" && (form.starts_with('f') || form.starts_with('F')) {
            form = format!("Ar{}", &form[1..]);
        } else if !form.starts_with('A') && !form.starts_with('E') && !form.starts_with('O') {
            form = format!("{}{}", aug, form);
        }
    }

    crate::engine::phonology::apply_natva_to_word(&form)
}

pub fn join_variants(
    stem: &str,
    variants: &[String],
    gana: u8,
    family: &str,
    purusha: u8,
    pada: &str,
    augment: Option<&str>,
    dhatu: &str,
    vacana: u8,
    antarganas: &str,
) -> Vec<String> {
    if family == "lit" {
        if let Some(out) = crate::engine::lit::kartari(dhatu, purusha, vacana, pada) {
            if !out.is_empty() {
                return out;
            }
        }
    }
    variants.iter().map(|v| join_form(stem, v, gana, family, purusha, pada, augment, Some(dhatu), vacana, Some(antarganas))).collect()
}
