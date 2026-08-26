//! Simplified port of engine/join.py
//! Handles thematic join + augment + gana-specific overrides via stubs.
//! Full 1874 LOC will be expanded, but this covers lat/lot/lrt/lang/vidhilin for gana 1,4,6 shuddha.

use crate::engine::phonology::thematic_join;


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
        ('d', 's') => format!("{}ts{}", stem_body, &suffix[1..]),
        ('c', 't') | ('j', 't') => format!("{}kt{}", stem_body, &suffix[1..]),
        ('z', 't') => format!("{}zw{}", stem_body, &suffix[1..]),
        ('z', 'T') => format!("{}zW{}", stem_body, &suffix[1..]),
        ('h', 't') => format!("{}gD{}", stem_body, &suffix[1..]),
        ('h', 'T') => format!("{}gD{}", stem_body, &suffix[1..]),
        ('h', 's') => format!("{}kz{}", stem_body, &suffix[1..]),
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
    // AD (2,3) irregulars: vid/as/han/vac etc. (high-impact for gana02)
    if gana == 2 || gana == 3 {
        if let Some(d) = dhatu {
            // han (02.??) – _join_han lat/lot/lang
            if d == "han" {
                if family == "lat" {
                    match ending {
                        "taH" => return "hataH".to_string(),
                        "nti" | "anti" => return "Gnanti".to_string(),
                        "si" => return "haMsi".to_string(),
                        "thaH" | "TaH" => return "haTaH".to_string(),
                        "tha" | "Ta" => return "haTa".to_string(),
                        _ => {}
                    }
                }
                if family == "lot" {
                    match ending {
                        "tu" => return "hantu".to_string(),
                        "tAm" => return "hatAm".to_string(),
                        "antu" => return "Gnantu".to_string(),
                        _ => {}
                    }
                }
                if family == "lang" {
                    match ending {
                        "an" => return "Gnan".to_string(),
                        "atAm" => return "hatAm".to_string(),
                        _ => {}
                    }
                }
            }
            if d == "vid" {
                if family == "lat" {
                    match (ending, purusha) {
                        ("ti", 1) => return "vetti".to_string(),
                        ("taH", 1) => return "vittaH".to_string(),
                        ("si", 2) => return "vetTa".to_string(),
                        _ => {}
                    }
                }
            }
            if d == "as" {
                if family == "lat" {
                    match ending {
                        "taH" => return "staH".to_string(),
                        "anti" | "nti" => return "santi".to_string(),
                        "si" => return "asi".to_string(),
                        _ => {}
                    }
                }
            }
            if d == "vac" {
                match ending {
                    "ti" => return "vakti".to_string(),
                    "taH" => return "vaktaH".to_string(),
                    "si" => return "vakzi".to_string(),
                    _ => {}
                }
            }
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
                _ => {}
            }
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
    // N (7) rudh → yunakti : handle Ru/Ra stems
    if gana == 7 {
        if family == "lrt" {
            return format!("{}{}", stem, ending);
        }
        if stem.ends_with("Ra") {
            let base_run = format!("{}n", &stem[..stem.len()-2]);
            let base_rur = &stem[..stem.len()-1];
            match ending {
                "ti" => return format!("{}atti", base_rur),
                "taH" | "TaH" => return format!("{}dDaH", base_run),
                "nti" => return format!("{}Danti", base_run),
                "si" => return format!("{}atsi", base_rur),
                "thaH" | "TaH" => return format!("{}dDaH", base_run),
                "tha" | "Ta" => return format!("{}dDa", base_run),
                "mi" => return format!("{}Dmi", stem),
                "vaH" => return format!("{}dDvaH", base_run),
                "maH" => return format!("{}dDmaH", base_run),
                "tAm" => return format!("{}dDAm", base_run),
                "tu" => return format!("{}adDu", base_rur),
                "antu" => return format!("{}Dantu", base_run),
                _ => {}
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
            _ => {}
        }
    }
    // Core join - upgraded
    let mut form = if stem.ends_with('a') && !ending.is_empty() {
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
    variants.iter().map(|v| join_form(stem, v, gana, family, purusha, pada, augment, Some(dhatu), vacana, Some(antarganas))).collect()
}
