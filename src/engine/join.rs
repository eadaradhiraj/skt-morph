//! Simplified port of engine/join.py
//! Handles thematic join + augment + gana-specific overrides via stubs.
//! Full 1874 LOC will be expanded, but this covers lat/lot/lrt/lang/vidhilin for gana 1,4,6 shuddha.

use crate::engine::phonology::thematic_join;

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
    // Pāṇini ad-gaṇa special sandhi for "ad" (02.0001) – port of _join_ad (handle ada variant)
    if let Some(d) = dhatu {
        let d_norm = if d == "ada" { "ad" } else { d };
        if d_norm == "ad" {
            // lat/lrt: d + ti/taH/si etc -> tt assimilation
            // AD_T_INFLECT = ti,taH,si,thaH,tha,tAt,tAd,tu,tam,ta
            if matches!(ending, "ti" | "taH" | "si" | "thaH" | "tha" | "tAt" | "tAd" | "tu" | "tam" | "ta") {
                // Pāṇini 8.2.31 etc: d -> t before t/th/s; tha -> Ta
                match ending {
                    "thaH" => return "atTaH".to_string(),
                    "tha" => return "atTa".to_string(),
                    _ => return format!("at{}", ending),
                }
            }
            if family == "lang" {
                // a + ad -> Ad (augment merged, no extra 'a')
                // For ad, lang forms are Adat, Adad etc (capital A) regardless of augment flag
                match ending {
                    "at" => return "Adat".to_string(),
                    "ad" => return "Adad".to_string(),
                    "atAm" => return "AttAm".to_string(),
                    "an" => return "Adan".to_string(),
                    "aH" => return "AdaH".to_string(),
                    "atam" => return "Attam".to_string(),
                    "ata" => return "Atta".to_string(),
                    "am" => return "Adam".to_string(),
                    "va" => return "Adva".to_string(),
                    "ma" => return "Adma".to_string(),
                    _ => {}
                }
            }
            if family == "lot" {
                // ad lot: attAt etc handled above via AD_T_INFLECT; for 3rd pl etc
                if matches!(ending, "tAt"|"tAd"|"tu"|"tam"|"ta") {
                    return format!("at{}", ending);
                }
            }
        }
        // div (04.0001) lang double-y fix is handled via stems, but join also needs to avoid double y
        // Fall through to normal
    }
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
                "ti" => return format!("{}adDi", base_rur),
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
    // Core join - simplified
    let mut form = if stem.ends_with('a') && !ending.is_empty() {
        // thematic
        thematic_join(stem, ending)
    } else {
        format!("{}{}", stem, ending)
    };

    // augment handling (a- for lang)
    if let Some(aug) = augment {
        // Don't double-augment if stem already starts with vowel augment
        if !form.starts_with('A') && !form.starts_with('E') && !form.starts_with('O') {
            // gana 1 thematic: a + Bavati -> aBavat etc ; but for demo we prefix
            // Real logic checks vowel_initial etc - simplified to prefix
            if gana == 1 || gana == 6 || gana == 4 {
                // If stem was vowel-initial, augment already merged
                if !["a","A","i","I","u","U","e","E","o","O"].contains(&&stem[..1]) {
                    form = format!("{}{}", aug, form);
                }
            } else {
                form = format!("{}{}", aug, form);
            }
        }
        // lang gemination etc omitted for now
    }

    // Special case: avoid double a (Bava + anti -> Bavanti not Bavaanti) already handled by thematic_join
    form
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
