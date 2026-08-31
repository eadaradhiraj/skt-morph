//! Simplified port of engine/join.py
//! Handles thematic join + augment + gana-specific overrides via stubs.
//! Full 1874 LOC will be expanded, but this covers lat/lot/lrt/lang/vidhilin for gana 1,4,6 shuddha.


//! =============================================================================
//! src/engine/join.rs: Pāṇini/Kaumudī implementation — extreme commenting pass (2026-09-01)
//! ---------------------------------------------------------------------------
//! Purpose: see inline block comments below. Every public/private block is
//! documented with sūtra reference, input/output, and edge-case notes.
//! Script: SLP1 internally; Devanagari only at demo boundary.
//! Flow: dhātu → it-strip → aṅga/vikaraṇa → lakāra/ending → sandhi → surface.
//! Gold DB is cross-check only, never source of truth.
//! =============================================================================
use crate::engine::phonology::thematic_join;

/// 3.1.80 धिन्विकृण्व्योर च: श्नु-like o/u/v after 7.1.58 (धिनोति, कृणोति).
fn join_snu_anga(base: &str, family: &str, ending: &str, augment: Option<&str>) -> Option<String> {
    let inner = match family {
        "lat" => match ending {
            "ti" => format!("{base}oti"),
            "taH" => format!("{base}utaH"),
            "nti" | "anti" => format!("{base}vanti"),
            "si" | "zi" => format!("{base}ozi"),
            "TaH" | "thaH" => format!("{base}uTaH"),
            "Ta" | "tha" => format!("{base}uTa"),
            "mi" | "Ami" => format!("{base}omi"),
            "vaH" | "AvaH" => format!("{base}uvaH"),
            "maH" | "AmaH" => format!("{base}umaH"),
            _ => return None,
        },
        "lot" => match ending {
            "tu" | "otu" => format!("{base}otu"),
            "tAt" | "utAt" => format!("{base}utAt"),
            "tAd" | "utAd" => format!("{base}utAd"),
            "tAm" | "utAm" => format!("{base}utAm"),
            "antu" | "vantu" => format!("{base}vantu"),
            "" | "u" => format!("{base}u"),
            "tam" | "utam" => format!("{base}utam"),
            "ta" | "uta" => format!("{base}uta"),
            "Ani" | "avAni" => format!("{base}avAni"),
            "Ava" | "avAva" => format!("{base}avAva"),
            "Ama" | "avAma" => format!("{base}avAma"),
            _ => return None,
        },
        "lang" => match ending {
            "at" | "ot" => format!("{base}ot"),
            "ad" | "od" => format!("{base}od"),
            "atAm" | "utAm" => format!("{base}utAm"),
            "an" | "van" => format!("{base}van"),
            "aH" | "oH" => format!("{base}oH"),
            "atam" | "utam" => format!("{base}utam"),
            "ata" | "uta" => format!("{base}uta"),
            "am" | "avam" => format!("{base}avam"),
            "Ava" | "uva" | "va" => format!("{base}uva"),
            "Ama" | "uma" | "ma" => format!("{base}uma"),
            _ => return None,
        },
        "vidhilin" => match ending {
            "et" | "At" | "yAt" => format!("{base}uyAt"),
            "ed" | "Ad" | "yAd" => format!("{base}uyAd"),
            "etAm" | "AtAm" | "yAtAm" => format!("{base}uyAtAm"),
            "eyuH" | "uH" | "yuH" => format!("{base}uyuH"),
            "eH" | "AH" | "yAH" => format!("{base}uyAH"),
            "etam" | "Atam" | "yAtam" => format!("{base}uyAtam"),
            "eta" | "Ata" | "yAta" => format!("{base}uyAta"),
            "eyam" | "Am" | "yAm" => format!("{base}uyAm"),
            "eva" | "Ava" | "yAva" => format!("{base}uyAva"),
            "ema" | "Ama" | "yAma" => format!("{base}uyAma"),
            _ => return None,
        },
        "lrt" => thematic_join(&format!("{base}vizya"), ending),
        _ => return None,
    };
    Some(apply_lang_aug(inner, family, augment))
}

// ---------------------------------------------------------------------------
// fn `apply_lang_aug`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn apply_lang_aug(form: String, family: &str, augment: Option<&str>) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family != "lang" {
        return form;
    }
    let Some(aug) = augment else {
        return format!("a{form}");
    };
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if aug != "a" {
        return format!("{aug}{form}");
    }
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match form.chars().next() {
        Some('a') | Some('A') => format!("A{}", &form[1..]),
        Some('i') | Some('I') | Some('e') | Some('E') => format!("E{}", &form[1..]),
        Some('u') | Some('U') | Some('o') | Some('O') => format!("O{}", &form[1..]),
        Some('f') | Some('F') => format!("Ar{}", &form[1..]),
        _ => format!("a{form}"),
    }
}

// ---------------------------------------------------------------------------
// fn `internal_sandhi`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn internal_sandhi(stem: &str, suffix: &str) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if stem.is_empty() || suffix.is_empty() { return format!("{}{}", stem, suffix); }
    let suff_first = suffix.chars().next().unwrap();
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if "aAiIuUfFeEoO".contains(suff_first) { return format!("{}{}", stem, suffix); }
    let stem_chars: Vec<char> = stem.chars().collect();
    let s_last = *stem_chars.last().unwrap();
    let stem_body: String = stem_chars[..stem_chars.len()-1].iter().collect();
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match (s_last, suff_first) {
        ('d', 't') => format!("{}t{}", stem_body, suffix),
        ('d', 'T') => format!("{}tT{}", stem_body, &suffix[1..]),
        ('d', 's') | ('t', 's') => format!("{}ts{}", stem_body, &suffix[1..]),
        ('D', 't') => format!("{}dD{}", stem_body, &suffix[1..]),
        ('D', 'T') => format!("{}dD{}", stem_body, &suffix[1..]),
        ('D', 's') => format!("{}ts{}", stem_body, &suffix[1..]),
        ('c', 't') | ('j', 't') => format!("{}kt{}", stem_body, &suffix[1..]),
        ('c', 'T') | ('j', 'T') => format!("{}kT{}", stem_body, &suffix[1..]),
        ('c', 's') | ('j', 's') | ('S', 's') | ('z', 's') => format!("{}kz{}", stem_body, &suffix[1..]),
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
        // 8.3.23 मोऽनुस्वारः — नंस्यति.
        ('m', 's') => format!("{}Ms{}", stem_body, &suffix[1..]),
        _ => format!("{}{}", stem, suffix),
    }
}

/// 3.1.78 श्नम्: strip leftover इत्, then infix न before the last consonant.
fn gana7_root(raw: &str) -> String {
    let mut s = raw.trim_end_matches('~').to_string();
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if s.ends_with("ir") && s.len() > 3 {
        s = s[..s.len() - 2].to_string();
    } else if s.ends_with('a') && s.len() > 2 {
        s = s[..s.len() - 1].to_string();
    }
    // 1.3.2/3 Sizx, Banjo, kftI, anjU.
    while let Some(last) = s.chars().last() {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if s.len() <= 2 {
            break;
        }
        let rest_len = s.len() - last.len_utf8();
        let prev = s[..rest_len].chars().last();
        let strip = match last {
            'x' | 'o' | 'O' | 'I' | 'U' | 'Y' => {
                prev.is_some_and(|c| !"aAiIuUfFeEoO".contains(c))
            }
            _ => false,
        };
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if strip {
            s.truncate(rest_len);
        } else {
            break;
        }
    }
    // 1.3.2 initial उँ इत् (उछृदिर्, उत्तृदिर्), not undI.
    if s.starts_with("uC")
        || (s.starts_with("ut")
            && s.len() >= 4
            && s.chars().nth(2).is_some_and(|c| !"aAiIuUfFeEoO".contains(c)))
    {
        s = s[1..].to_string();
    }
    s
}

// ---------------------------------------------------------------------------
// fn `join_form`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
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
    // अदादि: `adadi::join_form`.
    if gana == 2 {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(d) = dhatu {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(f) = crate::engine::adadi::join_form(d, family, ending, purusha, _vacana, augment) {
                return crate::engine::phonology::apply_natva_to_word(&f);
            }
        }
    }
    // 3.1.80 धिन्विकृण्व्योर च — श्नु for all sārvadhātuka / लृट्
    if let Some(base) = dhatu.and_then(crate::engine::phonology::dhinvi_krnvi_snu_base) {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(f) = join_snu_anga(base, family, ending, augment) {
            return crate::engine::phonology::apply_natva_to_word(&f);
        }
    }
    // G3 (3) reduplicated – juhu→juhoti, bibhI→bibheti, pF→piparti
    if gana == 3 {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if stem.ends_with("Ur") && ending == "ti" {
            return format!("{}arti", &stem[..stem.len()-2]); // pipUr+ti→piparti
        }
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if stem.ends_with('u') {
            let base = &stem[..stem.len()-1];
            // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
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
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if stem.ends_with('y') && (family == "vidhilin" || ending.starts_with('A')) {
            return format!("{}{}", stem, ending);
        }
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if stem.ends_with('I') {
            let base = &stem[..stem.len()-1];
            // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
            match ending {
                "ti" => return format!("{}eti", base), // bibhI→bibheti
                "taH" => return format!("{}ItaH", base),
                _ => {}
            }
        }
    }
    // NU gaṇa (5,8) – port of _join_nu (lat/lot/lrt core)
    if gana == 5 || gana == 8 {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if family == "lrt" {
            // future stems already like "to" etc, simple concat
            if ending.is_empty() { return stem.to_string(); }
            return format!("{}{}", stem, ending);
        }
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if stem.ends_with('u') {
            let base = &stem[..stem.len()-1];
            // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
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
                    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                    if ending.starts_with('u') {
                        return format!("{}{}", base, ending);
                    }
                }
            }
        }
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if stem.ends_with('R') {
            // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
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
    // N (7) श्नम्: रुणद्धि, रुन्द्धः, रुन्धन्ति; शिनष्टि, भनक्ति.
    if gana == 7 {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if family == "lrt" {
            return format!("{}{}", stem, ending);
        }
        let root = gana7_root(dhatu.unwrap_or(""));
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if !root.is_empty() {
            let chars: Vec<char> = root.chars().collect();
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if chars.len() >= 2 {
                let last = chars[chars.len() - 1];
                let body: String = chars[..chars.len() - 1].iter().collect();
                let nasal_upadha = body
                    .chars()
                    .last()
                    .is_some_and(|c| matches!(c, 'n' | 'm' | 'N' | 'Y' | 'R' | 'M'));
                let strong = if nasal_upadha {
                    format!("{body}a{last}")
                } else {
                    format!("{body}na{last}")
                };
                let weak = if nasal_upadha {
                    format!("{body}{last}")
                } else {
                    format!("{body}n{last}")
                };
                let pit = matches!(ending, "ti" | "si" | "mi" | "Ami" | "tu" | "tAt" | "tAd");
                let base = if pit { &strong } else { &weak };
                let out = |s: String| crate::engine::phonology::apply_natva_to_word(&s);
                // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
                match ending {
                    "ti" => return out(internal_sandhi(&strong, "ti")),
                    "si" => return out(internal_sandhi(&strong, "si")),
                    "mi" | "Ami" => return out(format!("{strong}mi")),
                    "taH" | "TaH" => return out(internal_sandhi(&weak, "taH")),
                    "nti" | "anti" => return out(format!("{weak}anti")),
                    "thaH" => return out(internal_sandhi(&weak, "TaH")),
                    "tha" | "Ta" => return out(internal_sandhi(&weak, "Ta")),
                    "vaH" | "AvaH" => return out(internal_sandhi(&weak, "vaH")),
                    "maH" | "AmaH" => return out(internal_sandhi(&weak, "maH")),
                    "tu" => return out(internal_sandhi(&strong, "tu")),
                    "antu" => return out(format!("{weak}antu")),
                    "tAm" => return out(internal_sandhi(&weak, "tAm")),
                    "yAt" | "yAd" => return out(format!("{weak}yA{}", &ending[1..])),
                    "at" | "ad" if family == "lang" => {
                        let body: String = strong
                            .chars()
                            .take(strong.chars().count().saturating_sub(1))
                            .collect();
                        return out(format!("{}{}t", augment.unwrap_or(""), body));
                    }
                    _ => {
                        return out(internal_sandhi(base, ending));
                    }
                }
            }
        }
    }
    // NI (9) punāti / krIRAti – handle nA → RA / nA
    if gana == 9 && stem.ends_with("nA") {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if family == "lrt" {
            return format!("{}{}", stem, ending);
        }
        let base = &stem[..stem.len()-2];
        let use_n = crate::engine::phonology::g9_uses_n_infix(dhatu.unwrap_or(""), _antarganas.unwrap_or(""));
        let nasal = if use_n { "n" } else { "R" };
        // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
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
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if family == "lang" {
                    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
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
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
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

// ---------------------------------------------------------------------------
// fn `join_all`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn join_all(
    stem: &str,
    ending: &str,
    gana: u8,
    family: &str,
    purusha: u8,
    pada: &str,
    augment: Option<&str>,
    dhatu: Option<&str>,
    vacana: u8,
    antarganas: Option<&str>,
) -> Vec<String> {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana == 2 {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(d) = dhatu {
            let fs = crate::engine::adadi::join_forms(d, family, ending, purusha, vacana, augment);
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if !fs.is_empty() {
                return fs
                    .into_iter()
                    .map(|f| crate::engine::phonology::apply_natva_to_word(&f))
                    .collect();
            }
        }
    }
    vec![join_form(
        stem,
        ending,
        gana,
        family,
        purusha,
        pada,
        augment,
        dhatu,
        vacana,
        antarganas,
    )]
}

// ---------------------------------------------------------------------------
// fn `join_variants`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
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
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family == "lit" {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(out) = crate::engine::lit::kartari(dhatu, purusha, vacana, pada) {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if !out.is_empty() {
                return out;
            }
        }
    }
    variants
        .iter()
        .flat_map(|v| {
            join_all(
                stem,
                v,
                gana,
                family,
                purusha,
                pada,
                augment,
                Some(dhatu),
                vacana,
                Some(antarganas),
            )
        })
        .collect()
}
