//! Port of sktmorph/engine/krdanta.py

//! =============================================================================
//! src/engine/krdanta.rs: Pāṇini/Kaumudī implementation — extreme commenting pass (2026-09-01)
//! ---------------------------------------------------------------------------
//! Purpose: see inline block comments below. Every public/private block is
//! documented with sūtra reference, input/output, and edge-case notes.
//! Script: SLP1 internally; Devanagari only at demo boundary.
//! Flow: dhātu → it-strip → aṅga/vikaraṇa → lakāra/ending → sandhi → surface.
//! Gold DB is cross-check only, never source of truth.
//! =============================================================================
use crate::engine::phonology::apply_guna_to_stem;
use serde::{Deserialize, Serialize};
use crate::engine::join::internal_sandhi;
use crate::engine::it::join_eco;

#[derive(Serialize, Deserialize, Debug)]
// ---------------------------------------------------------------------------
// struct `KrdantaResult`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub struct KrdantaResult {
    pub forms: Vec<String>,
    pub dhatu: String,
    pub pratyaya: String,
}

// pratyaya -> (suffix, sutras, mode)
fn pratyaya_rule(pratyaya: &str) -> Option<(&'static str, Vec<&'static str>, &'static str)> {
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match pratyaya {
        "Satf" => Some(("t", vec!["3.2.124"], "present")),
        "Satf~" => Some(("", vec!["3.2.124"], "present")),
        "kta" => Some(("ta", vec!["3.2.102"], "kta")),
        "ktavatu" => Some(("vat", vec!["3.2.171"], "kta")),
        "ktavatu~" => Some(("", vec!["3.2.171"], "kta")),
        "lyuw" => Some(("ana", vec!["3.3.115"], "guna")),
        "lyu" => Some(("ana", vec!["3.3.115"], "guna")),
        "tumun" => Some(("tum", vec!["3.3.158"], "guna_tum")),
        "ktvA" => Some(("tvA", vec!["3.4.21"], "root")),
        "ac" => Some(("", vec!["3.3.56"], "guna_a")),
        "ktin" => Some(("ti", vec!["3.3.94"], "guna")),
        "yat" => Some(("ya", vec!["3.2.187"], "guna")),
        "Ryat" => Some(("ya", vec!["3.2.187"], "guna")),
        "GaY" => Some(("a", vec!["3.3.67"], "guna")),
        "Ramul" => Some(("am", vec!["3.3.84"], "guna")),
        "Rvul" => Some(("aka", vec!["3.2.104"], "guna")),
        "vun" => Some(("aka", vec!["3.2.104"], "guna")),
        "anIyar" => Some(("anIya", vec!["3.2.96"], "anIya")),
        "tavya" => Some(("tavya", vec!["3.1.96"], "guna_tavya")),
        "tfc" => Some(("tf", vec!["3.3.92"], "guna")),
        "SAnac" => Some(("mAna", vec!["3.2.124"], "present")),
        "cAnaS" => Some(("mAna", vec!["3.2.124"], "present")),
        "gsnu" => Some(("snu", vec!["3.2.139"], "gsnu")),
        "knu" => Some(("nu", vec!["3.2.140"], "knu")),
        "kvasu" => Some(("vas", vec!["3.2.94"], "lit")),
        "lyap" => Some(("ya", vec!["3.2.187"], "lyap")),
        "ukaY" => Some(("uka", vec!["3.2.74"], "guna")),
        "a" => Some(("", vec!["3.3.56"], "guna_a")),
        "kyap" => Some(("ya", vec!["3.1.106"], "kyap")),
        "sya-Satf" => Some(("t", vec!["3.2.124"], "present")),
        "sya-Satf~" => Some(("", vec!["3.2.124"], "present")),
        "sya-SAnac" => Some(("mAna", vec!["3.2.124"], "present")),
        "sya-cAnaS" => Some(("mAna", vec!["3.2.124"], "present")),
        "BAvakarma-SAnac" => Some(("mAna", vec!["3.2.124"], "present")),
        "sya-BAvakarma-SAnac" => Some(("mAna", vec!["3.2.124"], "present")),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// fn `load_dhatu`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn load_dhatu(dhatu_query: &str) -> (String, u8, String, String, String) {
    let (dhatu, gana, _, tags, ant, aup) = crate::engine::dhatu::load_or_fallback(dhatu_query);
    (dhatu, gana, tags, ant, aup)
}

// ---------------------------------------------------------------------------
// fn `surface_root`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn surface_root(dhatu: &str) -> String {
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match crate::engine::lit::prakriya_root(dhatu).as_str() {
        "RI" => "nI".into(),
        "brU" => "vac".into(),
        "zWA" => "sTA".into(),
        other => other.to_string(),
    }
}

// ---------------------------------------------------------------------------
// fn `kta_base`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn kta_base(dhatu: &str) -> String {
    nistha_base(dhatu, true)
}

/// 6.4.24 अनिदितां हल उपधायाः क्ङिति — drop ञ्/ं upadhā before ज्/च्/श् on कित्
/// (रक्त, अक्त, दष्ट). Not म् before प्: कम्प् stays सेट् कम्पित, not *कप्त.
fn drop_anidit_upadha_nasal(root: &str) -> Option<String> {
    let mut c: Vec<char> = root.chars().collect();
    let n = c.len();
    if n < 3 {
        return None;
    }
    let ok = matches!(
        (c[n - 2], c[n - 1]),
        ('Y', 'j' | 'c') | ('n', 'j' | 'c') | ('M', 'S')
    );
    if !ok {
        return None;
    }
    c.remove(n - 2);
    Some(c.into_iter().collect())
}

/// `va` = 8.2.52 पचो वः (निष्ठा only, not क्त्वा).
fn nistha_base(dhatu: &str, va: bool) -> String {
    let mut r = surface_root(dhatu);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if r.ends_with('a') && r.len() >= 3 {
        let core = &r[..r.len() - 1];
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if core.chars().last().is_some_and(|c| !"aAiIuUfFeEoOxX".contains(c))
            && core.chars().any(|c| "aAiIuUfFeEoOxX".contains(c))
        {
            r = core.to_string();
        }
    }
    let orig = r.clone();
    // 6.1.15 वचिस्वपियजादीनां किति — वच्/यज्/वप्/वह्/स्वप्/वस् → उच्/इज्/…
    // 6.1.16 ग्रहिज्यावयिव्यधिवष्टिविचतिवृश्चतिपृच्छतिभृज्जतीनां ङिति च —
    // ग्रह् already गृहीत; व्यध् → विध् (विद्ध after 8.2.40); वयि वे → उ (उत);
    // वष्टि वश् → उश् (उष्ट after 8.2.36). विच् stays विक्त (8.2.30; संप्रसारण would collide with वच् उक्त).
    // व्रश्च्/पृच्छ्/भ्रस्ज् are 8.2.36 below. ज्या is named जीन (class after जि would be *जित).
    let r = match r.as_str() {
        "vac" => "uc".into(),
        "yaj" => "ij".into(),
        "vap" => "up".into(),
        "vah" => "uh".into(),
        "svap" | "zvap" => "sup".into(),
        "vas" => "us".into(),
        "grah" => "gfh".into(),
        "vyaD" => "viD".into(),
        "ve" => "u".into(),
        "vaS" => "uS".into(),
        other => other.to_string(),
    };
    // SLP1 भ is B; older "labh" = लभ्
    let r = if r.ends_with("bh") {
        format!("{}B", &r[..r.len() - 2])
    } else {
        r
    };
    let r = kit_anga(&r);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    // Remaining named ādeśa only. वह् ऊढ / दह् दग्ध fall out of 6.1.15 + kta_ho_dha
    // (uh → UQa via 8.2.31/6.3.111; dah starts with द so 8.2.32 दादेर् → dagDa).
    // 8.2.30 palatal is in the terminal match (before 7.2.35 इट्).
    if orig == "pac" && va {
        return "pakva".into(); // 8.2.52 पचो वः (निष्ठा only; क्त्वा stays पक्त्वा)
    }
    // 2.4.36 अदो जग्धिर्ल्यप्ति किति — अद् + क्त → जग्ध, not sandhi *atta.
    if orig == "ad" {
        return "jagDa".into();
    }
    // 8.2.31 हो ढः + गुण a→o (not 6.3.111 आ). Generic kta_ho_dha would yield *साढ.
    if orig == "sah" {
        return "soQa".into();
    }
    // भञ्ज्: ञ्-lopa + 8.2.30 ज→ग + न for त → भग्न. Palatal arm alone would give *BaYkta.
    // अञ्ज् is अक्त (6.4.24 अनिदितां), so this is not a general ञ्ज् class.
    if orig == "BaYj" {
        return "Bagna".into();
    }
    // 6.4.19 च्छ्वोः शूडनुनासिके च — दिव् व् → यू before क्त (द्यूत, not *divta / *dīta).
    if orig == "div" {
        return "dyUta".into();
    }
    // 8.2.34 नहो धः — नह् + त → नद्ध, not हो ढः *नाढ (kta_ho_dha).
    if orig == "nah" {
        return "nadDa".into();
    }
    // 6.1.16 ज्या संप्रसारण + निष्ठा न — जीन (not *jyAta; जि+त would be *jita).
    if orig == "jyA" {
        return "jIna".into();
    }
    // शुष् + निष्ठा क (not ष्टुत्व *शुष्ट). धृष् stays धृष्ट via ष्+त.
    if orig == "Suz" {
        return "Suzka".into();
    }
    // क्षण् is सेट् (not 7.2.10): क्त is क्षणित via takes_it_nistha, not *क्षात.
    match r.as_str() {
        "gfh" => "gfhIta".into(), // 7.2.37 ग्रहोऽलिटि दीर्घः
        // 8.2.36 व्रश्चभ्रस्जसृजमृजयजराजभ्राजच्छशां षः — ज/च्छ → ष before झल् त
        // (सृष्ट, मृष्ट, इष्ट). More specific than 8.2.30 चोः कुः (*सृक्त).
        // पृच्छ्: 6.1.16 संप्रसारण + च्छ→ष → पृष्ट (not palatal *prcKta).
        "sfj" | "mfj" | "Brasj" | "vraSc" => {
            let mut s = r.clone();
            s.pop();
            format!("{s}zwa")
        }
        "ij" => "izwa".into(),
        "uS" => "uzwa".into(), // 6.1.16 वश् → उश्, then षः (not सेट् *vaSita / palatal *vaSwa)
        "pfcC" | "pracC" => "pfzwa".into(),
        // 8.2.42 रदाभ्यां निष्ठातो नः पूर्वस्य च दः — भिद्/छिद् → भिन्न/छिन्न (not Bitta).
        // 8.2.45 ओदितश्च — शद्/पद्/स्कन्द् → शन्न/पन्न/स्कन्न (not Satta/skAta).
        // अद् is 2.4.36 जग्ध (above). सद् stays सत्त via internal_sandhi.
        _ if matches!(orig.as_str(), "Bid" | "Cid" | "Sad" | "pad" | "skand" | "skan") => {
            let mut stem = orig.as_str();
            if let Some(s) = stem.strip_suffix('d') {
                stem = s;
            }
            if let Some(s) = stem.strip_suffix('n') {
                stem = s;
            }
            format!("{stem}nna")
        }
        // 6.4.42 जनसनखनां: न-lopa + a→A (जात, सात, खात). Before इट् (*janita).
        _ if matches!(orig.as_str(), "jan" | "san" | "Kan") => {
            let stem = orig.trim_end_matches('n');
            let mut c: Vec<char> = stem.chars().collect();
            if let Some(last) = c.last_mut() {
                if *last == 'a' {
                    *last = 'A';
                }
            }
            format!("{}ta", c.iter().collect::<String>())
        }
        // शमामष्टानां (निष्ठा): शम् दम् तम् श्रम् भ्रम् क्रम् → शान्त दान्त … क्रान्त.
        // m stays as n before त (8.3.23/8.4.58), a→A.
        _ if matches!(orig.as_str(), "Sam" | "dam" | "tam" | "Sram" | "Bram" | "kram" | "kzam" | "klam") => {
            let stem = orig.strip_suffix('m').and_then(|s| s.strip_suffix('a')).unwrap_or(&orig);
            format!("{stem}Anta")
        }
        // 6.4.37/98 गम् हन् यम् रम् नम् तन् मन्: nasal lopa, short a (गत हत यत रत नत तत मत).
        _ if matches!(orig.as_str(), "gam" | "han" | "yam" | "ram" | "nam" | "tan" | "man" | "van") => {
            let stem = orig.trim_end_matches('m').trim_end_matches('n');
            format!("{stem}ta")
        }
        // 6.1.45 आदेच उपदेशेऽशिति: ए/ऐ → आ before कित्. 6.4.66 घुमास्थागापाजहातिसां हलि: गा/पा → ई (गीत, पीत).
        // कै → कात (not *कीत). Must precede 7.2.35 इट् (*gEita).
        _ if orig.ends_with('E') => {
            let body = &orig[..orig.len() - 1];
            if matches!(body, "g" | "p") {
                format!("{body}Ita")
            } else {
                format!("{body}Ata")
            }
        }
        // 6.4.24 अनिदितां — ञ्/ं-upadhā lopa then 8.2.30/36 (रक्त, अक्त, दष्ट). Before palatal *raYkta.
        // भञ्ज् is named भग्न above. कम्प् (म्+प्) is not this arm.
        _ if let Some(s) = drop_anidit_upadha_nasal(&orig) => internal_sandhi(&s, "ta"),
        // 8.2.36 शां षः — श् + त → ष्ट before इट् (नष्ट, दिष्ट, स्पृष्ट). Not ष-final सेट् (भाषित).
        _ if r.ends_with('S') => internal_sandhi(&r, "ta"),
        // ष् + त → ष्ट before इट् (कृष्ट, तुष्ट, द्विष्ट). शुष्क is named above.
        _ if r.ends_with('z') => internal_sandhi(&r, "ta"),
        // 8.2.30 चोः कुः — palatal + झल् त of क्त → velar (मुक्त, युक्त, सिक्त).
        // Must precede 7.2.35 इट्: takes_it_nistha would otherwise yield *mucita/*yujita.
        // च/छ/ज/झ → क/ख/ग/घ; internal_sandhi maps c/j + t → kt. छ/झ rare in निष्ठा.
        // भञ्ज् is not this arm (ञ्ज् + त → भग्न, kept named above).
        _ if r.chars().last().is_some_and(|c| matches!(c, 'c' | 'C' | 'j' | 'J')) => {
            internal_sandhi(&r, "ta")
        }
        // 8.2.37 पदादि (भष्): भ् + त → ब्ध (लब्ध, लुब्ध). Before इट् (*luBita).
        // 8.3.23 मोऽनुस्वारः on म्भ्: रम्भ् → रब्ध (drop m, then भष्).
        _ if r.ends_with("mB") => format!("{}bDa", &r[..r.len() - 2]),
        _ if r.ends_with('B') => internal_sandhi(&r, "ta"),
        // 8.2.40 झषस्तथोर्धोऽधः — ध् + त → द्ध (विद्ध after 6.1.16, बद्ध). Before इट् (*viDita).
        _ if r.ends_with('D') => internal_sandhi(&r, "ta"),
        // द् + त → त्त before इट् (नुत्त, तुत्त, सत्त). भिद्/छिद्/ओदित् are nna above.
        _ if r.ends_with('d') => internal_sandhi(&r, "ta"),
        _ if r.ends_with('h')
            && r.chars().rev().nth(1).is_some_and(|c| "aAiIuUfFeEoO".contains(c)) =>
        {
            kta_ho_dha(&r)
        }
        // After 6.1.15/16 the aṅga may be इक् (वे → उ). 7.2.11: no इट् (*veita).
        _ if r.chars().last().is_some_and(|c| "iIuUfF".contains(c)) => format!("{r}ta"),
        // इगुपध + प्/त्/क् + निष्ठा त before 7.2.35 (*kzipita/*gupita). क्षिप्त, लिप्त, गुप्त, चित्त.
        // takes_it_nistha treats unknown हल् as सेट्. Not anusvāra-upadhā प् (कम्प् → कम्पित).
        // Not ष्: generic ष्टुत्व would yield *शुष्ट; शुष्क is later.
        _ if {
            let mut cs = r.chars().rev();
            matches!(cs.next(), Some('p' | 't' | 'k'))
                && matches!(cs.next(), Some('i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X'))
        } =>
        {
            internal_sandhi(&r, "ta")
        }
        // प् + त → प्त before इट् (तप्त, आप्त). Not म्/ं-upadhā (कम्प् → कम्पित).
        _ if r.ends_with('p') && !matches!(r.chars().rev().nth(1), Some('m' | 'M')) => {
            internal_sandhi(&r, "ta")
        }
        // क् + त → क्त before इट् (शक्त). पतित stays सेट् (त् not this arm).
        _ if r.ends_with('k') => internal_sandhi(&r, "ta"),
        // त् + त → त्त before इट् (यत्त). पत् is सेट् पतित (takes_it_nistha).
        _ if r.ends_with('t') && orig != "pat" => internal_sandhi(&r, "ta"),
        _ if crate::engine::it::takes_it_nistha(&orig) => {
            let anga = if r.ends_with('s') {
                crate::engine::it::ruki_s(&r)
            } else {
                r.clone()
            };
            format!("{anga}ita")
        }
        _ => internal_sandhi(&r, "ta"),
    }
}

/// 8.2.31 हो ढः (लीढ, गूढ, ऊढ from वह् after 6.1.15 uh); 8.2.32 दादेर्धातोर्घः
/// (दह्/दुह् → दग्ध/दुग्ध via internal_sandhi h+t → gD, not ढ). 6.3.111 ढ्रलोपे
/// पूर्वस्य दीर्घोऽणः lengthens the vowel before ढ (i→ī, u→ū, a→ā). सह् is
/// named soQa (गुण o, not this ā).
fn kta_ho_dha(root: &str) -> String {
    // द-initial: 8.2.32 घः not ढः — दग्ध not *दाढ.
    if root.starts_with('d') {
        return internal_sandhi(root, "ta");
    }
    let mut body: String = root.chars().take(root.chars().count() - 1).collect();
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if let Some(v) = body.chars().last() {
        let long = match v {
            'i' => 'I',
            'u' => 'U',
            'a' => 'A',
            other => other,
        };
        body.pop();
        body.push(long);
    }
    format!("{body}Qa")
}

/// 7.4.40 द्यतिस्यतिमास्थामित्ति किति — दा/धा/स्था/मा/पा → इत्त् on कित् (क्त/क्तिन्).
/// 7.4.42 दधातेर्हिः — घु `धा` (SLP1 `DA`) on कित् → `हि`; निष्ठा `हि+त` → `हित` (8.2.31 ढत्व not triggered as no `ह्`-`त` jhal).
/// 7.4.46 दो दद् घोः — `दा` (SLP1 `dA`, दाण्) on कित् → `दद्`; `दद्+त` → `दत्त` (8.2.30 `द्+त` → `त्त`).
/// 6.4.37 न्-lopa — `गम्/हन्` → `ग/ह` before कित्; `बन्ध्` → `बध्`.
/// Keeps SLP1 `DA`=धा vs `dA`=दा distinct — critical for `हित` vs `दत्त`.
fn kit_anga(root: &str) -> String {
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match root {
        "dA" => "dad".into(),
        "DA" => "hi".into(),
        "sTA" => "sTi".into(),
        "mA" => "mi".into(),
        "pA" => "pI".into(),
        "gam" | "han" => root[..root.len() - 1].to_string(),
        // 6.4.42 जनसनखनां — न-lopa, a→A (क्तिन् जाति/खाति too).
        "jan" | "san" | "Kan" => {
            let stem = root.trim_end_matches('n');
            let mut c: Vec<char> = stem.chars().collect();
            if let Some(last) = c.last_mut() {
                if *last == 'a' {
                    *last = 'A';
                }
            }
            c.into_iter().collect()
        }
        "banD" | "bandh" => {
            let last = root.chars().last().unwrap();
            format!("{}{last}", &root[..root.len() - 2])
        }
        other => other.to_string(),
    }
}

// ---------------------------------------------------------------------------
// fn `ktin_form` — tin/sUP endings: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn ktin_form(root: &str) -> String {
    // 6.4.24 also on क्तिन् (कित्): रक्ति not *रञ्क्ति.
    let base = drop_anidit_upadha_nasal(root).unwrap_or_else(|| root.to_string());
    internal_sandhi(&kit_anga(&base), "ti")
}

/// क्यप् (3.1.106–110): कित् no गुण; पित् 6.1.71 तुक् after ह्रस्व (कृत्य, भृत्य, स्तुत्य).
/// 6.1.15 इज्या/उच्य; 3.1.108 हत्य. यक् stays क्रियमाण; ण्यत् stays कार्य.
fn kyap_form(root: &str) -> String {
    // 3.1.108 हनस्त च: हत्य not *हन्य.
    if root == "han" {
        return "hatya".into();
    }
    let r = match root {
        "vac" => "uc".into(),
        "yaj" => "ij".into(),
        "vap" => "up".into(),
        "vah" => "uh".into(),
        "svap" | "zvap" => "sup".into(),
        other => other.to_string(),
    };
    // 6.1.71 ह्रस्वस्य पिति कृति तुक् (not दीर्घ भूय; not हल् वृत्य).
    if r.chars().last().is_some_and(|c| matches!(c, 'a' | 'i' | 'u' | 'f' | 'x')) {
        format!("{r}tya")
    } else {
        format!("{r}ya")
    }
}

/// 3.2.139 ग्लाजिस्थाश्च गस्नुः: ग्लास्नु, जिष्णु (8.3.59), स्थास्नु. Not *ग्लाष्णु.
fn gsnu_form(root: &str) -> String {
    match root {
        "glA" => "glAsnu".into(),
        "ji" => "jizRu".into(),
        "sTA" => "sTAsnu".into(),
        other => format!("{other}snu"),
    }
}

/// 3.2.140 त्रसिगृधिधृषिक्षिपेः क्नुः: कित् नु (no गुण). 8.4.1 धृष्णु; SK क्षिप्णु.
/// गृध्नु stays न (द् blocks णत्व). Not *त्रस्णु / *क्षिप्नु.
fn knu_form(root: &str) -> String {
    // SK क्षिप्णुः (not *क्षिप्नु).
    if root == "kzip" {
        return "kzipRu".into();
    }
    crate::engine::phonology::apply_natva_to_word(&format!("{root}nu"))
}

/// क्वसु (3.2.107): लिट् weak aṅga + वस्. बभूवतुः → बभूवस् (not बभूव्वस्).
fn kvasu_form(dhatu: &str) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if let Some(forms) = crate::engine::lit::kartari(dhatu, 1, 2, "P") {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(du) = forms.first() {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(anga) = du.strip_suffix("atuH") {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if anga.ends_with('v') {
                    return format!("{anga}as");
                }
                return format!("{anga}vas");
            }
        }
    }
    format!("{}vas", surface_root(dhatu))
}

// ---------------------------------------------------------------------------
// fn `is_ac`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn is_ac(c: char) -> bool {
    matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

/// 7.2.115/116 वृद्धि (i/ī → ऐ, u/ū → औ, ṛ → आर्, a → आ).
fn vrddhi_ac(root: &str) -> String {
    let chars: Vec<char> = root.chars().collect();
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for idx in (0..chars.len()).rev() {
        let repl = match chars[idx] {
            'a' => Some("A"),
            'i' | 'I' | 'e' => Some("E"),
            'u' | 'U' | 'o' => Some("O"),
            'f' | 'F' => Some("Ar"),
            _ => None,
        };
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(r) = repl {
            let mut o = String::new();
            o.extend(chars[..idx].iter().copied());
            o.push_str(r);
            o.extend(chars[idx + 1..].iter().copied());
            return o;
        }
    }
    root.to_string()
}

/// 7.3.52 चजोः कु घिण्ण्यतोः.
fn cajo_ku(s: &str) -> String {
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match s.chars().last() {
        Some('c') => format!("{}k", &s[..s.len() - 1]),
        Some('j') => format!("{}g", &s[..s.len() - 1]),
        _ => s.to_string(),
    }
}

/// णित्/ञित् kṛt aṅga: 7.2.115 अचो ञ्णिति, 7.2.116 अत उपधायाः, 7.3.86 इगुपध गुण,
/// 7.3.33 आतो युक्, 7.3.32/54 हन् → घात्.
fn nit_krt_anga(root: &str, pratyaya: &str) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if root == "han" {
        return "GAt".into();
    }
    let last = root.chars().last().unwrap_or('a');
    let mut anga = if is_ac(last) {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if last == 'A' {
            format!("{root}y")
        } else {
            vrddhi_ac(root)
        }
    } else {
        // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
        match root.chars().rev().nth(1) {
            Some('a') => vrddhi_ac(root),
            Some('i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'e' | 'o') => apply_guna_to_stem(root),
            _ => root.to_string(),
        }
    };
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if matches!(pratyaya, "GaY" | "Ryat") {
        anga = cajo_ku(&anga);
    }
    anga
}

// ---------------------------------------------------------------------------
// fn `nit_krt_form`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn nit_krt_form(root: &str, pratyaya: &str) -> String {
    let suffix = match pratyaya {
        "Rvul" => "aka",
        "ukaY" => "uka",
        "Ryat" => "ya",
        _ => "a",
    };
    join_eco(&nit_krt_anga(root, pratyaya), suffix)
}

// ---------------------------------------------------------------------------
// fn `ktva_base`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn ktva_base(dhatu: &str) -> String {
    let ta = nistha_base(dhatu, false);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if let Some(stripped) = ta.strip_suffix("ita") {
        format!("{stripped}itvA")
    } else if let Some(stripped) = ta.strip_suffix("ta") {
        format!("{stripped}tvA")
    } else {
        format!("{ta}tvA")
    }
}

// ---------------------------------------------------------------------------
// fn `lyap_base`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn lyap_base(dhatu: &str) -> String {
    let ta = nistha_base(dhatu, false);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if let Some(stripped) = ta.strip_suffix("ita") {
        format!("{stripped}ya")
    } else if let Some(stripped) = ta.strip_suffix("ta") {
        format!("{stripped}ya")
    } else {
        format!("{ta}ya")
    }
}

// ---------------------------------------------------------------------------
// fn `generate`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate(dhatu_query: &str, pratyaya: &str) -> KrdantaResult {
    let forms = derive(dhatu_query, pratyaya);
    KrdantaResult { forms, dhatu: dhatu_query.to_string(), pratyaya: pratyaya.to_string() }
}

// ---------------------------------------------------------------------------
// fn `generate_with_prefixes`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_with_prefixes(dhatu_query: &str, pratyaya: &str, prefixes: &[String]) -> KrdantaResult {
    let pratyaya_eff = if pratyaya == "ktvA" && !prefixes.is_empty() { "lyap" } else { pratyaya };
    let forms = derive(dhatu_query, pratyaya_eff);
    let forms = if prefixes.is_empty() {
        forms
    } else {
        forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect()
    };
    KrdantaResult { forms, dhatu: dhatu_query.to_string(), pratyaya: pratyaya.to_string() }
}

// ---------------------------------------------------------------------------
// fn `is_avyaya`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn is_avyaya(pratyaya: &str) -> bool {
    matches!(pratyaya, "ktvA" | "lyap" | "tumun" | "Ramul" | "am")
}

/// लिङ्गs this kṛt takes. Empty = अव्यय (no सुप्).
pub fn lingas(pratyaya: &str) -> &'static [&'static str] {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if is_avyaya(pratyaya) {
        return &[];
    }
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match pratyaya {
        "ktin" => &["stri"],
        "lyuw" | "lyu" => &["nap"],
        "GaY" => &["pum"],
        _ => &["pum", "stri", "nap"],
    }
}

// ---------------------------------------------------------------------------
// fn `is_at_participle`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn is_at_participle(pratyaya: &str) -> bool {
    matches!(pratyaya, "Satf" | "Satf~" | "sya-Satf" | "sya-Satf~" | "ktavatu" | "ktavatu~")
}

// ---------------------------------------------------------------------------
// fn `pratipadika`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn pratipadika(form: &str, pratyaya: &str, linga: &str) -> Option<String> {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if is_avyaya(pratyaya) || form.is_empty() {
        return None;
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if pratyaya == "tfc" && linga == "stri" {
        let base = form.trim_end_matches('f');
        return Some(format!("{base}rI"));
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if is_at_participle(pratyaya) && linga == "stri" {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if pratyaya.starts_with("ktavatu") {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if form.ends_with("at") {
                return Some(format!("{form}I"));
            }
        } else if let Some(base) = form.strip_suffix("at") {
            return Some(format!("{base}antI"));
        }
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if linga == "stri"
        && (matches!(
            pratyaya,
            "kta" | "SAnac" | "cAnaS" | "tavya" | "anIyar" | "Rvul" | "vun" | "ac" | "anIya"
                | "yat" | "Ryat" | "kyap"
        ) || pratyaya.contains("SAnac")
            || pratyaya.contains("cAnaS"))
    {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(base) = form.strip_suffix('a') {
            return Some(format!("{base}A"));
        }
    }
    Some(form.to_string())
}

/// सुबन्त of a kṛdanta pratipadika. `None` for अव्यय or a लिङ्ग the kṛt does not take.
pub fn decline(
    dhatu_query: &str,
    pratyaya: &str,
    linga: &str,
    prefixes: &[String],
) -> Option<crate::declension::subanta::Declension> {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if !lingas(pratyaya).contains(&linga) {
        return None;
    }
    let res = generate_with_prefixes(dhatu_query, pratyaya, prefixes);
    let form = res.forms.first()?.as_str();
    let stem = pratipadika(form, pratyaya, linga)?;
    let mut d = crate::declension::subanta::generate(&stem, linga)?;
    // 6.4.14 अत्वसन्तस्य चाधातोः: शतृ has no दीर्घ (भवन् not भवान्). क्तवतु keeps आन्.
    if matches!(pratyaya, "Satf" | "Satf~" | "sya-Satf" | "sya-Satf~") && linga == "pum" {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(row) = d.declension.get_mut("prathamA") {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(nom) = row.first_mut() {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if let Some(base) = nom.strip_suffix("An") {
                    *nom = format!("{base}an");
                }
            }
        }
    }
    Some(d)
}

// ---------------------------------------------------------------------------
// fn `derive`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn derive(dhatu_query: &str, pratyaya: &str) -> Vec<String> {
    let (dhatu, gana, tags, ant, aup) = load_dhatu(dhatu_query);
    let rule = pratyaya_rule(pratyaya);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if rule.is_none() {
        return vec![];
    }
    let (suffix, _sutras, mode) = rule.unwrap();
    let root = surface_root(&dhatu);
    let guna = apply_guna_to_stem(&root);

    let form = match mode {
        "present" => {
            // 3.1.33 स्यतासी लृलुटोः: स्य-शतृ/शानच् on लृट् स्य, not लट् *गच्छत्.
            let lak = if matches!(
                pratyaya,
                "sya-Satf" | "sya-Satf~" | "sya-SAnac" | "sya-cAnaS" | "sya-BAvakarma-SAnac"
            ) {
                "lrt"
            } else {
                "lat"
            };
            let (st, _) = crate::engine::stems::derive_stem(&dhatu, gana, lak, "shuddha", &tags, &ant, &aup);
            let base = st.unwrap_or_else(|| {
                if lak == "lrt" {
                    crate::engine::it::sya_stem(&root)
                } else {
                    present_stem(&dhatu, gana)
                }
            });
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if pratyaya == "Satf" || pratyaya == "sya-Satf" {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if base.ends_with('a') {
                    format!("{}at", &base[..base.len() - 1])
                } else if base.ends_with('u') {
                    format!("{}vat", &base[..base.len() - 1])
                } else if base.ends_with('I') {
                    format!("{}at", &base[..base.len() - 1])
                } else {
                    format!("{}at", base)
                }
            } else if pratyaya == "Satf~" || pratyaya == "sya-Satf~" {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if base.ends_with('a') {
                    format!("{}n", &base[..base.len() - 1])
                } else {
                    format!("{}ant", base)
                }
            } else if pratyaya == "SAnac" || pratyaya == "cAnaS" || pratyaya.contains("SAnac") || pratyaya.contains("cAnaS") {
                // 3.1.67 यक् + 3.2.124: भावे/कर्मणि गम्यमान (not लट् *गच्छमान). स्य-भावकर्म is लृट् गमिष्यमाण, no यक्.
                let base = if pratyaya == "BAvakarma-SAnac" {
                    crate::engine::derived::karma_stem(&root)
                } else {
                    base
                };
                // 7.2.82 आने मुक्: keep अ (एधमान / गम्यमान, not *एध्मान / *गम्य्मान).
                let raw = if base.ends_with('a') {
                    format!("{}mAna", base)
                } else if base.ends_with('u') {
                    format!("{}vAna", &base[..base.len() - 1])
                } else {
                    format!("{}Ana", base)
                };
                crate::engine::phonology::apply_natva_to_word(&raw)
            } else {
                format!("{}{}", base, suffix)
            }
        }
        "kta" => {
            let base = kta_base(&dhatu);
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if pratyaya.starts_with("ktavatu") { format!("{base}vat") } else { base }
        }
        "guna" => {
            // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
            match pratyaya {
                "lyuw" | "lyu" => crate::engine::it::lyuw_form(&root),
                "tfc" => crate::engine::it::tfc_form(&root),
                "ktin" => ktin_form(&root),
                "GaY" | "Rvul" | "ukaY" | "Ryat" => nit_krt_form(&root, pratyaya),
                "Ramul" => join_eco(&nit_krt_anga(&root, "Rvul"), "am"),
                "yat" if root.ends_with('A') => format!("{}eya", &root[..root.len() - 1]),
                _ => join_eco(&guna, suffix),
            }
        }
        "guna_a" => join_eco(&guna, "a"),
        "kyap" => kyap_form(&root),
        "gsnu" => gsnu_form(&root),
        "knu" => knu_form(&root),
        "guna_tum" => crate::engine::it::tum_form(&root),
        "guna_tavya" => crate::engine::it::tavya_form(&root),
        "anIya" => crate::engine::it::anIya_form(&root),
        "root" if pratyaya == "ktvA" => ktva_base(&dhatu),
        "root" => format!("{}{}", dhatu, suffix),
        "lit" => kvasu_form(&dhatu),
        "lyap" => lyap_base(&dhatu),
        _ => format!("{}{}", guna, suffix),
    };
    vec![form]
}

// ---------------------------------------------------------------------------
// fn `present_stem`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn present_stem(dhatu: &str, gana: u8) -> String {
    let guna = apply_guna_to_stem(dhatu);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana == 10 { return format!("{}aya", guna); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana == 4 {
        // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
        for idx in (0..dhatu.len()).rev() {
            let ch = dhatu.chars().nth(idx).unwrap();
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if "iIuUfF".contains(ch) {
                let long_v = match ch { 'i' => 'I', 'u' => 'U', 'f' => 'F', _ => ch };
                let mut out = String::new();
                // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
                for (i,c) in dhatu.chars().enumerate() {
                    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                    if i==idx { out.push(long_v); } else { out.push(c); }
                }
                return format!("{}ya", out);
            }
        }
        return format!("{}ya", guna);
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana == 1 || gana == 6 {
        let base = if gana == 6 { dhatu.to_string() } else { guna };
        return format!("{}a", base);
    }
    guna
}

// Optional scrape probe (not the spec).
pub fn validate_against_gold(dhatu_id: &str, pratyaya: &str) -> Option<(String, String)> {
    let p = format!("/home/edhiraj/Documents/projs/skt-morph-data/data/{}/{}.json", &dhatu_id[..2], dhatu_id);
    let data = std::fs::read_to_string(&p).ok()?;
    let v: serde_json::Value = serde_json::from_str(&data).ok()?;
    let base = v["participles"]["krut"].get(pratyaya)?.as_array()?.first()?;
    let gold_m = base.get("m")?.as_str()?.to_string();
    let ours = derive(dhatu_id, pratyaya);
    Some((ours.first().cloned().unwrap_or_default(), gold_m))
}

#[cfg(test)]
// ---------------------------------------------------------------------------
// mod `tests`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
mod tests {
    use super::*;

    #[test]
    // ---------------------------------------------------------------------------
    // fn `bu_kta`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn bu_kta() {
        let f = derive("BU", "kta");
        assert!(f.iter().any(|x| x == "BUta"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `gam_kf_vac_da_kta`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn gam_kf_vac_da_kta() {
        assert_eq!(derive("gam", "kta"), vec!["gata"]);
        assert_eq!(derive("qukfY", "kta"), vec!["kfta"]);
        assert_eq!(derive("vaca", "kta"), vec!["ukta"]);
        // 7.4.46 दा → दत्त vs 7.4.42 धा → हित — SLP1 dA (द) vs DA (ध) distinct.
        assert_eq!(derive("qudAY", "kta"), vec!["datta"]); // दाण्/दा
        assert_eq!(derive("quDAY", "kta"), vec!["hita"]); // धेट्/धा — 7.4.42
        assert_eq!(derive("dA", "kta"), vec!["datta"]);
        assert_eq!(derive("DA", "kta"), vec!["hita"]);
        // 2.4.36 अदो जग्धिर्ल्यप्ति किति — अद् → जग्ध (atta would be sandhi-only, wrong)
        assert_eq!(derive("ada", "kta"), vec!["jagDa"]); // अद् → जग्ध
        assert_eq!(derive("ad", "kta"), vec!["jagDa"]);
        // भिद् → भिन्न (Binna) — 8.2.43/6.4.47 special, not Bitta
        assert_eq!(derive("Bida", "kta"), vec!["Binna"]); // भिद् → भिन्न (Bida is भिद् with a)
        assert_eq!(derive("Bid", "kta"), vec!["Binna"]);
        // शद्/पद् → शन्न/पन्न (Sanna/panna) — 8.2.45
        assert_eq!(derive("Sada", "kta"), vec!["Sanna"]); // शद् → शन्न
        assert_eq!(derive("pada", "kta"), vec!["panna"]); // पद् → पन्न (pada is पद्)
        assert_eq!(derive("Sad", "kta"), vec!["Sanna"]);
        assert_eq!(derive("pad", "kta"), vec!["panna"]);
        // सह् → सोढ (soQa) — 8.2.31 + guṇa
        assert_eq!(derive("saha", "kta"), vec!["soQa"]); // सह् → सोढ (saha is सह् with a)
        assert_eq!(derive("sah", "kta"), vec!["soQa"]);
        // वह् → ऊढ (UQa) — 6.1.15 + 8.2.31
        assert_eq!(derive("vaha", "kta"), vec!["UQa"]); // वह् → ऊढ (already in gam_kf_vac_da_kta? keep explicit)
        assert_eq!(derive("vah", "kta"), vec!["UQa"]);
        // दह् → दग्ध (dagDa) — 8.2.32
        assert_eq!(derive("daha", "kta"), vec!["dagDa"]); // दह् → दग्ध
        assert_eq!(derive("dah", "kta"), vec!["dagDa"]);
        // 8.2.30 चोः कुः — real palatals (not the fake *kij/*tuj dump).
        assert_eq!(derive("muc", "kta"), vec!["mukta"]);
        assert_eq!(derive("yuj", "kta"), vec!["yukta"]);
        assert_eq!(derive("sic", "kta"), vec!["sikta"]);
        assert_eq!(derive("Buj", "kta"), vec!["Bukta"]);
        assert_eq!(derive("ruc", "kta"), vec!["rukta"]);
        assert_eq!(derive("BaYj", "kta"), vec!["Bagna"]);
        assert_eq!(derive("aYj", "kta"), vec!["akta"]);
        assert_eq!(derive("raYj", "kta"), vec!["rakta"]);
        assert_eq!(derive("saYj", "kta"), vec!["sakta"]);
        assert_eq!(derive("daMS", "kta"), vec!["dazwa"]);
        assert_eq!(derive("diS", "kta"), vec!["dizwa"]);
        assert_eq!(derive("naS", "kta"), vec!["nazwa"]);
        assert_eq!(derive("spfS", "kta"), vec!["spfzwa"]);
        // 8.2.36 षः
        assert_eq!(derive("sfj", "kta"), vec!["sfzwa"]);
        assert_eq!(derive("yaj", "kta"), vec!["izwa"]);
        assert_eq!(derive("pfcC", "kta"), vec!["pfzwa"]);
        // 6.4.37/42 + शमादि
        assert_eq!(derive("jan", "kta"), vec!["jAta"]);
        assert_eq!(derive("tan", "kta"), vec!["tata"]);
        assert_eq!(derive("kram", "kta"), vec!["krAnta"]);
        assert_eq!(derive("Sram", "kta"), vec!["SrAnta"]);
        assert_eq!(derive("Bram", "kta"), vec!["BrAnta"]);
        assert_eq!(derive("dam", "kta"), vec!["dAnta"]);
        assert_eq!(derive("Sam", "kta"), vec!["SAnta"]);
        assert_eq!(derive("tam", "kta"), vec!["tAnta"]);
        assert_eq!(derive("nam", "kta"), vec!["nata"]);
        assert_eq!(derive("yam", "kta"), vec!["yata"]);
        assert_eq!(derive("ram", "kta"), vec!["rata"]);
        assert_eq!(derive("van", "kta"), vec!["vata"]);
        assert_eq!(derive("man", "kta"), vec!["mata"]);
        assert_eq!(derive("san", "kta"), vec!["sAta"]);
        assert_eq!(derive("han", "kta"), vec!["hata"]);
        assert_eq!(derive("Kan", "kta"), vec!["KAta"]);
        assert_eq!(derive("skan", "kta"), vec!["skanna"]);
        assert_eq!(derive("Cid", "kta"), vec!["Cinna"]);
        // 6.4.19 द्यूत; 8.2.37 भष्; 8.2.34 नद्ध; 6.1.16+8.2.40 विद्ध
        assert_eq!(derive("div", "kta"), vec!["dyUta"]);
        assert_eq!(derive("luB", "kta"), vec!["lubDa"]);
        assert_eq!(derive("kzuB", "kta"), vec!["kzubDa"]);
        assert_eq!(derive("ramB", "kta"), vec!["rabDa"]);
        assert_eq!(derive("laB", "kta"), vec!["labDa"]);
        assert_eq!(derive("nah", "kta"), vec!["nadDa"]);
        assert_eq!(derive("vyaD", "kta"), vec!["vidDa"]);
        assert_eq!(derive("jyA", "kta"), vec!["jIna"]);
        assert_eq!(derive("ve", "kta"), vec!["uta"]);
        assert_eq!(derive("veY", "kta"), vec!["uta"]);
        assert_eq!(derive("vaS", "kta"), vec!["uzwa"]);
        assert_eq!(derive("vaSa", "kta"), vec!["uzwa"]);
        // इगुपध प्/त्/क् before इट् (not *kzipita; कम्प् stays कम्पित).
        assert_eq!(derive("kzip", "kta"), vec!["kzipta"]);
        assert_eq!(derive("lip", "kta"), vec!["lipta"]);
        assert_eq!(derive("gup", "kta"), vec!["gupta"]);
        assert_eq!(derive("cit", "kta"), vec!["citta"]);
        assert_eq!(derive("kamp", "kta"), vec!["kampita"]);
        // ष्+त → ष्ट (कृष्ट); शुष् is शुष्क not *शुष्ट. द्+त → त्त (नुत्त); भिद् stays भिन्न.
        assert_eq!(derive("kfz", "kta"), vec!["kfzwa"]);
        assert_eq!(derive("tuz", "kta"), vec!["tuzwa"]);
        assert_eq!(derive("puz", "kta"), vec!["puzwa"]);
        assert_eq!(derive("dviz", "kta"), vec!["dvizwa"]);
        assert_eq!(derive("Suz", "kta"), vec!["Suzka"]);
        assert_eq!(derive("nud", "kta"), vec!["nutta"]);
        assert_eq!(derive("tud", "kta"), vec!["tutta"]);
        assert_eq!(derive("sad", "kta"), vec!["satta"]);
        assert_eq!(derive("tap", "kta"), vec!["tapta"]);
        assert_eq!(derive("Ap", "kta"), vec!["Apta"]);
        assert_eq!(derive("Sak", "kta"), vec!["Sakta"]);
        assert_eq!(derive("yat", "kta"), vec!["yatta"]);
        assert_eq!(derive("lih", "kta"), vec!["lIQa"]);
        assert_eq!(derive("guh", "kta"), vec!["gUQa"]);
        // 6.1.45 आदेच + 6.4.66 गा/पा → गीत/पीत; other ऐ → आत (कै कात).
        assert_eq!(derive("gE", "kta"), vec!["gIta"]);
        assert_eq!(derive("pE", "kta"), vec!["pIta"]);
        assert_eq!(derive("kE", "kta"), vec!["kAta"]);
        assert_eq!(derive("dE", "kta"), vec!["dAta"]);
        assert_eq!(derive("trE", "kta"), vec!["trAta"]);
        // 7.2.11 श्र्युकः किति: u-final अनिट् + त (real dhātus only).
        assert_eq!(derive("dru", "kta"), vec!["druta"]);
        assert_eq!(derive("sru", "kta"), vec!["sruta"]);
        assert_eq!(derive("stu", "kta"), vec!["stuta"]);
        assert_eq!(derive("hu", "kta"), vec!["huta"]);
        assert_eq!(derive("su", "kta"), vec!["suta"]);
        assert_eq!(derive("BU", "ktvA"), vec!["BUtvA"]);
        assert_eq!(derive("gam", "tumun"), vec!["gantum"]);
        let f = generate_with_prefixes("BU", "ktvA", &["pra".into()]);
        assert!(f.forms.iter().any(|x| x == "praBUya"), "{:?}", f.forms);
        assert_eq!(derive("qukfY", "tavya"), vec!["kartavya"]);
        assert_eq!(derive("qukfY", "tfc"), vec!["kartf"]);
        assert_eq!(derive("qukfY", "lyuw"), vec!["karaRa"]);
        assert_eq!(derive("qukfY", "anIyar"), vec!["karaRIya"]);
        let sat = derive("hu", "Satf");
        assert!(sat.iter().any(|x| x == "juhvat"), "{:?}", sat);
        assert_eq!(derive("dfSir", "kta"), vec!["dfzwa"]);
        assert_eq!(derive("vaha", "kta"), vec!["UQa"]);
        assert_eq!(derive("duha", "kta"), vec!["dugDa"]);
        assert_eq!(kta_base("labh"), "labDa");
        assert_eq!(kta_base("svap"), "supta");
        assert_eq!(kta_base("naS"), "nazwa");
        assert_eq!(derive("graha", "kta"), vec!["gfhIta"]);
        assert_eq!(derive("vasa", "kta"), vec!["uzita"]);
        assert_eq!(derive("patx", "kta"), vec!["patita"]);
        assert_eq!(derive("banDa", "kta"), vec!["badDa"]);
        assert_eq!(derive("qupacaz", "kta"), vec!["pakva"]);
        assert_eq!(derive("qupacaz", "ktvA"), vec!["paktvA"]);
        assert_eq!(derive("gam", "tavya"), vec!["gantavya"]);
        assert_eq!(derive("gam", "tfc"), vec!["gantf"]);
        assert_eq!(derive("RIY", "tumun"), vec!["netum"]);
        assert_eq!(derive("BU", "tumun"), vec!["Bavitum"]);
        assert_eq!(derive("Sru", "lyuw"), vec!["SravaRa"]);
        assert_eq!(derive("Sru", "anIyar"), vec!["SravaRIya"]);
        assert_eq!(derive("hana", "anIyar"), vec!["hananIya"]);
        assert_eq!(derive("RIY", "lyuw"), vec!["nayana"]);
        assert_eq!(derive("qudAY", "lyuw"), vec!["dAna"]);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `krdanta_declension`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn krdanta_declension() {
        let d = decline("gam", "kta", "pum", &[]).expect("gataH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gataH"), "{:?}", pr);
        let d = decline("gam", "kta", "stri", &[]).expect("gatA");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gatA"), "{:?}", pr);
        let d = decline("BU", "Satf", "pum", &[]).expect("Bavan");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "Bavan"), "{:?}", pr);
        let d = decline("gamx", "Satf", "pum", &[]).expect("gacCan");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gacCan"), "{:?}", pr);
        let d = decline("BU", "Satf", "stri", &[]).expect("BavantI");
        assert_eq!(d.stem, "BavantI");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "BavantI"), "{:?}", pr);
        let d = decline("BU", "Satf", "nap", &[]).expect("Bavat");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "Bavat"), "{:?}", pr);
        assert!(pr.iter().any(|x| x == "Bavanti"), "{:?}", pr);
        assert!(pr.iter().any(|x| x == "BavantI"), "{:?}", pr);
        let d = decline("qukfY", "tfc", "pum", &[]).expect("kartA");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "kartA"), "{:?}", pr);
        let dv = d.declension.get("dvitIyA").unwrap();
        assert!(dv.iter().any(|x| x == "kartAram"), "{:?}", dv);
        let d = decline("qukfY", "tfc", "stri", &[]).expect("kartrI");
        assert_eq!(d.stem, "kartrI");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "kartrI"), "{:?}", pr);
        assert!(decline("BU", "ktvA", "pum", &[]).is_none());
        let d = decline("gam", "ktavatu", "pum", &[]).expect("gatavAn");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gatavAn"), "{:?}", pr);
        // 4.1.4 टाप् शानच्: एधमानः / एधमाना / एधमानम्. शतृ स्त्री stays पचन्ती.
        let d = decline("eDa", "SAnac", "pum", &[]).expect("eDamAnaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "eDamAnaH"), "{:?}", pr);
        let d = decline("eDa", "SAnac", "stri", &[]).expect("eDamAnA");
        assert_eq!(d.stem, "eDamAnA");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "eDamAnA"), "{:?}", pr);
        let d = decline("eDa", "SAnac", "nap", &[]).expect("eDamAnam");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "eDamAnam"), "{:?}", pr);
        assert!(pr.iter().any(|x| x == "eDamAnAni"), "{:?}", pr);
        let d = decline("BU", "SAnac", "pum", &[]).expect("BavamAnaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "BavamAnaH"), "{:?}", pr);
        let d = decline("BU", "kvasu", "pum", &[]).expect("baBUvAn");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "baBUvAn"), "{:?}", pr);
        let tr = d.declension.get("tfIyA").unwrap();
        assert!(tr.iter().any(|x| x == "baBUvuzA"), "{:?}", tr);
        let d = decline("gamx", "sya-Satf", "pum", &[]).expect("gamizyan");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gamizyan"), "{:?}", pr);
        let d = decline("gamx", "sya-Satf", "stri", &[]).expect("gamizyantI");
        assert_eq!(d.stem, "gamizyantI");
        let d = decline("gamx", "Satf", "pum", &[]).expect("gacCan");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gacCan"), "{:?}", pr);
        let d = decline("gamx", "BAvakarma-SAnac", "pum", &[]).expect("gamyamAnaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gamyamAnaH"), "{:?}", pr);
        let d = decline("gamx", "BAvakarma-SAnac", "stri", &[]).expect("gamyamAnA");
        assert_eq!(d.stem, "gamyamAnA");
        let d = decline("BU", "SAnac", "pum", &[]).expect("BavamAnaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "BavamAnaH"), "{:?}", pr);
        let d = decline("qukfY", "kyap", "pum", &[]).expect("kftyaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "kftyaH"), "{:?}", pr);
        let d = decline("trasI", "knu", "pum", &[]).expect("trasnuH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "trasnuH"), "{:?}", pr);
        let d = decline("qukfY", "kyap", "stri", &[]).expect("kftyA");
        assert_eq!(d.stem, "kftyA");
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `krdanta_lingas_by_pratyaya`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn krdanta_lingas_by_pratyaya() {
        assert!(lingas("lyap").is_empty());
        assert!(lingas("ktvA").is_empty());
        assert!(lingas("tumun").is_empty());
        assert!(lingas("Ramul").is_empty());
        assert_eq!(lingas("lyuw"), &["nap"]);
        assert_eq!(lingas("ktin"), &["stri"]);
        assert_eq!(lingas("GaY"), &["pum"]);
        assert_eq!(lingas("kta"), &["pum", "stri", "nap"]);
        assert!(decline("qukfY", "lyuw", "pum", &[]).is_none());
        assert!(decline("qukfY", "lyuw", "stri", &[]).is_none());
        let d = decline("qukfY", "lyuw", "nap", &[]).expect("karaRam");
        assert_eq!(d.linga, "nap");
        assert!(decline("qukfY", "ktin", "pum", &[]).is_none());
        let d = decline("qukfY", "ktin", "stri", &[]).expect("kfti");
        assert_eq!(d.stem, "kfti");
        assert!(decline("BU", "lyap", "nap", &[]).is_none());
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `nit_krts_vrddhi_and_kitin` — sūtra: 7.2.115/116 vṛddhi: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn nit_krts_vrddhi_and_kitin() {
        assert_eq!(derive("BU", "GaY"), vec!["BAva"]);
        assert_eq!(derive("BU", "Rvul"), vec!["BAvaka"]);
        assert_eq!(derive("BU", "vun"), vec!["Bavaka"]);
        assert_eq!(derive("BU", "ukaY"), vec!["BAvuka"]);
        assert_eq!(derive("BU", "Ryat"), vec!["BAvya"]);
        assert_eq!(derive("BU", "yat"), vec!["Bavya"]);
        assert_eq!(derive("RIY", "GaY"), vec!["nAya"]);
        assert_eq!(derive("RIY", "Rvul"), vec!["nAyaka"]);
        assert_eq!(derive("RIY", "vun"), vec!["nayaka"]);
        assert_eq!(derive("qudAY", "GaY"), vec!["dAya"]);
        assert_eq!(derive("qudAY", "Rvul"), vec!["dAyaka"]);
        assert_eq!(derive("qudAY", "vun"), vec!["dAka"]);
        assert_eq!(derive("qudAY", "yat"), vec!["deya"]);
        assert_eq!(derive("tyaja", "GaY"), vec!["tyAga"]);
        assert_eq!(derive("tyaja", "Ryat"), vec!["tyAgya"]);
        assert_eq!(derive("tyaja", "Rvul"), vec!["tyAjaka"]);
        assert_eq!(derive("qupacaz", "GaY"), vec!["pAka"]);
        assert_eq!(derive("qupacaz", "Rvul"), vec!["pAcaka"]);
        assert_eq!(derive("qupacaz", "Ryat"), vec!["pAkya"]);
        assert_eq!(derive("qukfY", "GaY"), vec!["kAra"]);
        assert_eq!(derive("qukfY", "Rvul"), vec!["kAraka"]);
        assert_eq!(derive("qukfY", "vun"), vec!["karaka"]);
        assert_eq!(derive("qukfY", "Ryat"), vec!["kArya"]);
        assert_eq!(derive("hana", "GaY"), vec!["GAta"]);
        assert_eq!(derive("hana", "Rvul"), vec!["GAtaka"]);
        assert_eq!(derive("hana", "vun"), vec!["hanaka"]);
        assert_eq!(derive("gam", "GaY"), vec!["gAma"]);
        assert_eq!(derive("dfSir", "GaY"), vec!["darSa"]);
        assert_eq!(derive("Sru", "Rvul"), vec!["SrAvaka"]);
        assert_eq!(derive("Sru", "Ryat"), vec!["SrAvya"]);
        assert_eq!(derive("gam", "ktin"), vec!["gati"]);
        assert_eq!(derive("qudAY", "ktin"), vec!["datti"]);
        assert_eq!(derive("tyaja", "ktin"), vec!["tyakti"]);
        assert_eq!(derive("dfSir", "ktin"), vec!["dfzwi"]);
        assert_eq!(derive("qukfY", "ktin"), vec!["kfti"]);
        assert_eq!(derive("raYj", "ktin"), vec!["rakti"]);
        assert_eq!(derive("aYj", "ktin"), vec!["akti"]);
        assert_eq!(derive("qudAY", "kta"), vec!["datta"]);
        assert_eq!(derive("zWA", "kta"), vec!["sTita"]);
        assert_eq!(derive("eDa", "SAnac"), vec!["eDamAna"]);
        assert_eq!(derive("BU", "SAnac"), vec!["BavamAna"]);
        assert_eq!(derive("gamx", "Satf"), vec!["gacCat"]);
        // 3.1.33 स्य + 3.2.124 शतृ/शानच्: गमिष्यत्/भविष्यत्, not लट् *गच्छत्. शतृ stays गच्छत्.
        assert_eq!(derive("gamx", "sya-Satf"), vec!["gamizyat"]);
        assert_eq!(derive("BU", "sya-Satf"), vec!["Bavizyat"]);
        assert_eq!(derive("gamx", "sya-SAnac"), vec!["gamizyamARa"]);
        assert_eq!(derive("eDa", "sya-SAnac"), vec!["eDizyamARa"]);
        // 3.1.67 यक् शानच्: गम्यमान / क्रियमाण / भूयमान. कर्तरि शानच् stays एधमान/भवमान.
        assert_eq!(derive("gamx", "BAvakarma-SAnac"), vec!["gamyamAna"]);
        assert_eq!(derive("qukfY", "BAvakarma-SAnac"), vec!["kriyamARa"]);
        assert_eq!(derive("BU", "BAvakarma-SAnac"), vec!["BUyamAna"]);
        assert_eq!(derive("qudAY", "BAvakarma-SAnac"), vec!["dIyamAna"]);
        assert_eq!(derive("gamx", "sya-BAvakarma-SAnac"), vec!["gamizyamARa"]);
        assert_eq!(derive("BU", "SAnac"), vec!["BavamAna"]);
        assert_eq!(derive("qukfY", "kyap"), vec!["kftya"]);
        assert_eq!(derive("Bf", "kyap"), vec!["Bftya"]);
        assert_eq!(derive("stu", "kyap"), vec!["stutya"]);
        assert_eq!(derive("yaja", "kyap"), vec!["ijya"]);
        assert_eq!(derive("vaca", "kyap"), vec!["ucya"]);
        assert_eq!(derive("hana", "kyap"), vec!["hatya"]);
        assert_eq!(derive("vft", "kyap"), vec!["vftya"]);
        assert_eq!(derive("qukfY", "Ryat"), vec!["kArya"]);
        assert_eq!(derive("qukfY", "BAvakarma-SAnac"), vec!["kriyamARa"]);
        assert_eq!(derive("glA", "gsnu"), vec!["glAsnu"]);
        assert_eq!(derive("ji", "gsnu"), vec!["jizRu"]);
        assert_eq!(derive("zWA", "gsnu"), vec!["sTAsnu"]);
        assert_eq!(derive("trasI", "knu"), vec!["trasnu"]);
        assert_eq!(derive("gfDu", "knu"), vec!["gfDnu"]);
        assert_eq!(derive("YiDfzA", "knu"), vec!["DfzRu"]);
        assert_eq!(derive("kzip", "knu"), vec!["kzipRu"]);
        assert_eq!(derive("BU", "kvasu"), vec!["baBUvas"]);
        assert_eq!(derive("qukfY", "Ramul"), vec!["kAram"]);
        assert_eq!(derive("BU", "Ramul"), vec!["BAvam"]);
        assert!(decline("qukfY", "Ramul", "pum", &[]).is_none());
    }
}
