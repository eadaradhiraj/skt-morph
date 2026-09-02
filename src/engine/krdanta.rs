//! krdanta — kṛt pratyayas (3.1–3.4, Kaumudī). Port of sktmorph/engine/krdanta.py, sūtra-gated.

use crate::engine::phonology::apply_guna_to_stem;
use serde::{Deserialize, Serialize};
use crate::engine::join::internal_sandhi;
use crate::engine::it::join_eco;

#[derive(Serialize, Deserialize, Debug)]
pub struct KrdantaResult {
    pub forms: Vec<String>,
    pub dhatu: String,
    pub pratyaya: String,
}

// pratyaya -> (suffix, sutras, mode)
fn pratyaya_rule(pratyaya: &str) -> Option<(&'static str, Vec<&'static str>, &'static str)> {
    match pratyaya {
        "Satf" => Some(("t", vec!["3.2.124"], "present")),
        "Satf~" => Some(("", vec!["3.2.124"], "present")),
        "kta" => Some(("ta", vec!["3.2.102"], "kta")),
        "ktavatu" => Some(("vat", vec!["3.2.171"], "kta")),
        "ktavatu~" => Some(("", vec!["3.2.171"], "kta")),
        "lyuw" => Some(("ana", vec!["3.3.115"], "guna")),
        "lyu" => Some(("ana", vec!["3.1.134"], "guna")),
        "Nini" => Some(("in", vec!["3.1.134"], "nini")),
        "yuc" => Some(("ana", vec!["3.2.148", "3.2.149", "3.2.150", "3.2.151"], "yuc")),
        "tumun" => Some(("tum", vec!["3.3.158"], "guna_tum")),
        "ktvA" => Some(("tvA", vec!["3.4.21"], "root")),
        "ac" => Some(("", vec!["3.3.56"], "guna_a")),
        "ktin" => Some(("ti", vec!["3.3.94"], "guna")),
        "yat" => Some(("ya", vec!["3.1.97", "3.1.98", "3.1.99", "3.1.100", "3.1.102", "3.1.105"], "yat")),
        "Ryat" => Some(("ya", vec!["3.1.124", "3.1.125"], "guna")),
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
        "GinuR" => Some(("in", vec!["3.2.141"], "ghinun")),
        "kvarap" => Some(("vara", vec!["3.2.163"], "kvarap")),
        "Aluc" => Some(("Alu", vec!["3.2.158"], "aluc")),
        "kmarac" => Some(("mara", vec!["3.2.160"], "kmarac")),
        "Gurac" => Some(("ura", vec!["3.2.161"], "gurac")),
        "varac" => Some(("vara", vec!["3.2.175"], "varac")),
        "itra" => Some(("itra", vec!["3.2.184"], "itra")),
        "zwran" => Some(("tra", vec!["3.2.182"], "zwran")),
        "kurac" => Some(("ura", vec!["3.2.162"], "kurac")),
        "kru" => Some(("ru", vec!["3.2.174"], "kru")),
        "klukan" => Some(("luka", vec!["3.2.174"], "klukan")),
        "krukan" => Some(("ruka", vec!["3.2.174"], "krukan")),
        "Aru" => Some(("Aru", vec!["3.2.173"], "aru")),
        "ra" => Some(("ra", vec!["3.2.167"], "ra")),
        "u" => Some(("u", vec!["3.2.168"], "sanu")),
        "naN" => Some(("na", vec!["3.3.90"], "nan")),
        "aTuc" => Some(("aTu", vec!["3.3.89"], "athuc")),
        "Nvanip" => Some(("van", vec!["3.2.103"], "nvanip")),
        "Takan" => Some(("Taka", vec!["3.1.146"], "takan")),
        "Ryuw" => Some(("ana", vec!["3.1.147"], "ryuw")),
        "nan" => Some(("na", vec!["3.3.91"], "svapnan")),
        "najiN" => Some(("aj", vec!["3.2.172"], "najin")),
        "zAkan" => Some(("Aka", vec!["3.2.155"], "sakan")),
        "zvun" => Some(("aka", vec!["3.1.145"], "zvun")),
        "SAnan" => Some(("mAna", vec!["3.2.128"], "sanan")),
        "atfn" => Some(("at", vec!["3.2.104"], "atfn")),
        "vuY" => Some(("aka", vec!["3.2.146"], "vuy")),
        "ktri" => Some(("trima", vec!["3.3.88"], "ktri")),
        "aN" => Some(("A", vec!["3.3.104"], "an_stri")),
        "ap" => Some(("a", vec!["3.3.57"], "rdorap")),
        "Ra" => Some(("a", vec!["3.1.140", "3.1.141"], "jvala_ra")),
        "Sa" => Some(("a", vec!["3.1.137"], "sa_krt")),
        "ka" => Some(("a", vec!["3.1.135"], "ka_kit")),
        "kvasu" => Some(("vas", vec!["3.2.94"], "lit")),
        "lyap" => Some(("ya", vec!["7.1.37"], "lyap")),
        "ukaY" => Some(("uka", vec!["3.2.154"], "ukan")),
        "ini" => Some(("in", vec!["3.2.156", "3.2.157"], "ini")),
        "a" => Some(("", vec!["3.3.56"], "guna_a")),
        "kyap" => Some(("ya", vec!["3.1.106", "3.1.107", "3.1.108", "3.1.109", "3.1.110", "3.1.120", "3.1.121"], "kyap")),
        "sya-Satf" => Some(("t", vec!["3.2.124"], "present")),
        "sya-Satf~" => Some(("", vec!["3.2.124"], "present")),
        "sya-SAnac" => Some(("mAna", vec!["3.2.124"], "present")),
        "sya-cAnaS" => Some(("mAna", vec!["3.2.124"], "present")),
        "BAvakarma-SAnac" => Some(("mAna", vec!["3.2.124"], "present")),
        "sya-BAvakarma-SAnac" => Some(("mAna", vec!["3.2.124"], "present")),
        _ => None,
    }
}

fn load_dhatu(dhatu_query: &str) -> (String, u8, String, String, String) {
    let (dhatu, gana, _, tags, ant, aup) = crate::engine::dhatu::load_or_fallback(dhatu_query);
    (dhatu, gana, tags, ant, aup)
}

fn surface_root(dhatu: &str) -> String {
    match crate::engine::lit::prakriya_root(dhatu).as_str() {
        "RI" => "nI".into(),
        "brU" => "vac".into(),
        "zWA" => "sTA".into(),
        other => other.to_string(),
    }
}

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
    if r.ends_with('a') && r.len() >= 3 {
        let core = &r[..r.len() - 1];
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

fn ktin_form(root: &str) -> String {
    // 6.4.24 also on क्तिन् (कित्): रक्ति not *रञ्क्ति.
    let base = drop_anidit_upadha_nasal(root).unwrap_or_else(|| root.to_string());
    internal_sandhi(&kit_anga(&base), "ti")
}

/// क्यप् (3.1.106–110, 3.1.120–121): कित् no गुण; पित् 6.1.71 तुक् after ह्रस्व (कृत्य, भृत्य, स्तुत्य).
/// 6.1.15 इज्या/उच्य; 3.1.107 भूय vs यत् भव्य; 3.1.108 हत्य; 3.1.109 शिष्य (इत्त्व+षत्व).
/// 3.1.120 कृत्य/वृष्य vs ण्यत् कार्य/वर्ष्य. 3.1.121 युग्य vs ण्यत् योग्य. यक् stays क्रियमाण.
fn kyap_form(root: &str) -> String {
    // 3.1.107 भुवो भावे: भूय (दीर्घ, no तुक्). यत् stays भव्य; ण्यत् भाव्या; ल्यप् homophone भूय.
    if root == "BU" {
        return "BUya".into();
    }
    // 3.1.108 हनस्त च: हत्य not *हन्य.
    if root == "han" {
        return "hatya".into();
    }
    // 3.1.109 एतिस्तुशास्वृदृजुषः: शिष्य (6.4.34 इत्त्व + 8.3.60 षत्व), not *शास्य.
    if matches!(root, "SAs" | "SAsu") {
        return "SiSya".into();
    }
    // 3.1.110 ऋदुपधात्: वृध्य vs ण्यत् वर्ध्य. युच् stays वर्धन. क्त stays वृद्ध. Not क्लृप्/चृत् dump.
    if matches!(root, "vfD" | "vfDu") {
        return "vfDya".into();
    }
    // 3.1.121 युग्यं च पत्रे: कुत्व निपातन युग्य, not *युज्य. ण्यत् stays योग्य.
    if matches!(root, "yuj" | "yuja" | "yujir") {
        return "yugya".into();
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

/// 3.2.141 शमित्यष्टाभ्यो घिनुण्: शमी/तमी/दमी/श्रमी/भ्रमी/क्षमी/क्लमी (मित् ह्रस्व);
/// मदी is 7.2.116 मादी. इन्-anta (शमी not *शमा). Not क्त शान्त.
fn ghinun_form(root: &str) -> String {
    match root {
        "Sam" | "tam" | "dam" | "Sram" | "Bram" | "kzam" | "klam" => format!("{root}in"),
        "mad" => "mAdin".into(),
        other => format!("{other}in"),
    }
}

/// 3.2.163 इण्नश्जिसर्तिभ्यः क्वरप्: इत्वर/नश्वर/जित्वर/सृत्वर (कित् no गुण; पित् तुक्).
/// 3.2.164 गत्वरश्च: गम् → गत्वर (nasal lopa). Not *जय्वर / *गम्बर.
fn kvarap_form(root: &str) -> String {
    match root {
        "i" => "itvara".into(),
        "ji" => "jitvara".into(),
        "sf" => "sftvara".into(),
        "naS" | "RaS" => "naSvara".into(),
        "gam" => "gatvara".into(),
        other => format!("{other}vara"),
    }
}

/// 3.2.158 स्पृहिगृहिपतिदयिभ्य आलुच्: दयालु; 6.4.55 अय् → स्पृहयालु/गृहयालु/पतयालु.
fn aluc_form(root: &str) -> String {
    match root {
        "day" => "dayAlu".into(),
        "spfh" | "spfha" => "spfhayAlu".into(),
        "grah" | "gfh" | "gfha" => "gfhayAlu".into(),
        "pat" => "patayAlu".into(),
        other => format!("{other}Alu"),
    }
}

/// 3.2.160 सृघस्यदः क्मरच्: सृमर/घस्मर/अद्मर (कित् no गुण). Not *सर्मर. घस् ≠ अद्.
fn kmarac_form(root: &str) -> String {
    match root {
        "sf" => "sfmara".into(),
        "Gas" => "Gasmara".into(),
        "ad" => "admara".into(),
        other => format!("{other}mara"),
    }
}

/// 3.2.161 भञ्जभासमिदो घुरच्: भङ्गुर/भासुर/मेदुर (घित् गुण). Not *मिदुर / *भञ्जुर.
fn gurac_form(root: &str) -> String {
    match root {
        "BAs" => "BAsura".into(),
        "mid" => "medura".into(),
        "Banj" | "BaYj" => "BaNgura".into(),
        other => format!("{other}ura"),
    }
}

/// 3.2.175 स्थेशभासपिसकसो वरच्: स्थावर/ईश्वर/भास्वर/पेस्वर/कस्वर.
/// घुरच् भासुर stays *ura; क्वरप् जित्वर stays kit.
fn varac_form(root: &str) -> String {
    match root {
        "sTA" => "sTAvara".into(),
        "IS" => "ISvara".into(),
        "BAs" => "BAsvara".into(),
        "pis" | "pes" => "pesvara".into(),
        "kas" => "kasvara".into(),
        other => format!("{other}vara"),
    }
}

/// 3.2.184 अर्तिलूधूसूखनसहचर इत्रः (करण, नपुं): अरित्र/लवित्र/धुवित्र/सवित्र/खनित्र/सहित्र/चरित्र.
/// धू कुटादि उवङ् (not *धवित्र). पू is not in the sūtra.
fn itra_form(root: &str) -> String {
    match root {
        "f" => "aritra".into(),
        "lU" => "lavitra".into(),
        "DU" => "Duvitra".into(),
        "sU" | "zU" => "savitra".into(),
        "Kan" => "Kanitra".into(),
        "sah" | "zah" => "sahitra".into(),
        "car" => "caritra".into(),
        other => format!("{other}itra"),
    }
}

/// 3.2.181 धः कर्मणि ष्ट्रन्: धात्र (स्त्री धात्री). 3.2.182 दाम्नीशस्… करणे: दात्र/नेत्र/शस्त्र/योत्र/योक्त्र/
/// स्तोत्र/तोत्त्र/सेत्र/सेक्त्र/मेढ्र/पत्र/दंष्ट्र/नद्ध्र. दंष्ट्रा टाप्; others ङीष् (4.1.41).
fn zwran_form(root: &str) -> String {
    match root {
        "DA" | "De" => "DAtra".into(),
        "dA" => "dAtra".into(),
        "nI" => "netra".into(),
        "Sas" => "Sastra".into(),
        "yu" => "yotra".into(),
        "yuj" | "yuja" => "yoktra".into(),
        "stu" => "stotra".into(),
        "tud" | "tuda" => "tottra".into(),
        "si" | "zu" => "setra".into(),
        "sic" | "zic" => "sektra".into(),
        "mih" | "miha" => "meQra".into(),
        "pat" => "patra".into(),
        "danS" | "danSa" | "daMS" => "daMzwra".into(),
        "nah" | "Rah" => "nadDra".into(),
        other => format!("{other}tra"),
    }
}

/// 3.2.162 विदिभिदिच्छिदेः कुरच्: विदुर/भिदुर/छिदुर (कित् no गुण). घुरच् मेदुर stays गुण.
fn kurac_form(root: &str) -> String {
    match root {
        "vid" | "vida" => "vidura".into(),
        "Bid" => "Bidura".into(),
        "Cid" => "Cidura".into(),
        other => format!("{other}ura"),
    }
}

/// 3.2.174 भियः क्रुक्लुकनौ: भीरु/भीलुक (कित् no गुण). वार्तिक क्रुकन् भीरुक. Not *भेरु.
fn bhi_kru_form(root: &str, kind: &str) -> String {
    if root != "BI" {
        return match kind {
            "kru" => format!("{root}ru"),
            "klukan" => format!("{root}luka"),
            _ => format!("{root}ruka"),
        };
    }
    match kind {
        "kru" => "BIru".into(),
        "klukan" => "BIluka".into(),
        _ => "BIruka".into(),
    }
}

/// 3.2.173 शृवन्द्योरारुः: शरारु (गुण); वन्दारु (वन्द्). Not *शृारु.
fn aru_form(root: &str) -> String {
    match root {
        "SF" | "Sf" => "SarAru".into(),
        "vad" | "vadi" | "vand" => "vandAru".into(),
        other => format!("{other}Aru"),
    }
}

/// 3.2.167 नमिकम्पिस्म्यजसकमहिंसदीपो रः: नम्र/कम्प्र/स्मेर/जस्र/कम्र/हिंस्र/दीप्र.
/// स्मेर has गुण; नश् क्वरप् stays नश्वर.
fn ra_form(root: &str) -> String {
    match root {
        "nam" | "Ram" => "namra".into(),
        "kamp" | "kap" | "kapi" => "kampra".into(),
        "smi" | "zmi" => "smera".into(),
        "jas" => "jasra".into(),
        "kam" => "kamra".into(),
        "hims" | "hiMs" | "his" => "himsra".into(),
        "dIp" => "dIpra".into(),
        other => format!("{other}ra"),
    }
}

/// 3.2.168 सनाशंसभिक्ष उः: भिक्षु; 3.1.5 गुप्तिज्किद् जुगुप्सु/तितिक्षु/चिकित्सु;
/// 3.1.6 मीमांसु/बीभत्सु; generic सन् चिकीर्षु. Not षणु-धातु.
fn san_u_form(dhatu: &str, root: &str) -> String {
    match root {
        "Bikz" | "Bikza" => return "Bikzu".into(),
        "gup" | "gupa" => return "jugupsu".into(),
        "tij" | "tija" => return "titikzu".into(),
        "mAn" | "mAna" => return "mImAMsu".into(),
        "baD" => return "bIBatsu".into(),
        _ => {}
    }
    let stem = crate::engine::derived::nitya_san_present(dhatu)
        .unwrap_or_else(|| crate::engine::derived::san_stem(root));
    format!("{}u", stem.trim_end_matches('a'))
}

/// 3.3.90 यजयाच्यतविच्छप्रच्छरक्षो नङ्: यज्ञ/याच्ञा/यत्न/विश्न/प्रश्न/रक्ष्ण (ङित् no गुण).
fn nan_form(root: &str) -> String {
    match root {
        "yaj" => "yajYa".into(),
        "yAc" => "yAcYA".into(),
        "yat" => "yatna".into(),
        "vicC" | "viC" | "viCa" => "viSna".into(),
        "pracC" | "praC" | "praCa" => "praSna".into(),
        "rakz" | "rakza" => "rakzRa".into(),
        other => format!("{other}na"),
    }
}

/// 3.3.89 ट्वितोऽथुच्: वेपथु/भाजथु. 7.1.58 इदितो नुम्: नन्दथु not *नदथु.
fn athuc_form(root: &str) -> String {
    if matches!(root, "nad" | "nand" | "nadi") {
        return "nandaTu".into();
    }
    format!("{root}aTu")
}

/// 3.2.103 सुयजोर्ङ्वनिप्: यज्वन्/सुत्वन् (ङित् no गुण). Not *याज्वन्. अन-stem यज्वा/यज्वना.
fn nvanip_form(root: &str) -> String {
    match root {
        "yaj" => "yajvan".into(),
        "su" | "zu" => "sutvan".into(),
        other => format!("{other}van"),
    }
}

/// 3.1.146 गस्थकन्: गै → गाथक (आत्व). स्त्री गाथिका (not टाप् *गाथका).
fn takan_form(root: &str) -> String {
    match root {
        "gE" | "gA" => "gATaka".into(),
        other => format!("{other}Taka"),
    }
}

/// 3.1.147 ण्युट् च: गायन (7.3.33 युक्). 3.1.148 हाः: हायन. थकन् stays गाथक.
fn ryuw_form(root: &str) -> String {
    match root {
        "gE" | "gA" => "gAyana".into(),
        "hA" => "hAyana".into(),
        other => format!("{other}ana"),
    }
}

/// 3.3.91 स्वपो नन्: स्वप्न. नङ् यज्ञ stays. नजिङ् स्वप्नज् stays.
fn svapnan_form(root: &str) -> String {
    match root {
        "svap" | "zvap" => "svapna".into(),
        other => format!("{other}na"),
    }
}

/// 3.2.172 स्वपितृषोर्नजिङ्: स्वप्नज्/तृष्णज् (ङित्; 8.2.30 कुत्व स्वप्नक्). नन् stays स्वप्न.
fn najin_form(root: &str) -> String {
    match root {
        "svap" | "zvap" => "svapnaj".into(),
        "tfz" => "tfzRaj".into(),
        other => format!("{other}aj"),
    }
}

/// 3.2.155 जल्पभिक्षकुट्टलुण्टवृङां षाकन्: जल्पाक/भिक्षाक/कुट्टाक/लुण्टाक/वराक (गुण).
/// षिद् → 4.1.41 ङीष् जल्पाकी. उः भिक्षु stays. Not *जल्पक.
fn sakan_form(root: &str) -> String {
    match root {
        "jalp" | "jalpa" => "jalpAka".into(),
        "Bikz" | "Bikza" => "BikzAka".into(),
        "kuww" | "kuwwa" => "kuwwAka".into(),
        "lunw" | "lunwa" | "luRw" | "luRwa" => "luRwAka".into(),
        "vf" => "varAka".into(),
        other => format!("{other}Aka"),
    }
}

/// 3.1.145 शिल्पिनि ष्वुन्: नर्तक/खनक/रजक (रञ्जेर्नलोप). षिद् ङीष् नर्तकी not ण्वुल् *नर्तिका.
fn zvun_form(root: &str) -> String {
    match root {
        "nft" | "nftI" => "nartaka".into(),
        "Kan" | "Kanu" => "Kanaka".into(),
        "raYj" | "ranj" | "ranja" => "rajaka".into(),
        other => format!("{other}aka"),
    }
}

/// 3.2.128 पूङ् यजः शानन्: पवमान/यजमान (कर्तरि कृत्, not लादेश शानच्).
/// पूञ् पुनाति stays शानच्; एधमान stays शानच्.
fn sanan_form(dhatu: &str, root: &str) -> String {
    let d = dhatu.trim_end_matches('~');
    if d == "pUN" {
        return "pavamAna".into();
    }
    if matches!(root, "yaj") || d == "yaja" {
        return "yajamAna".into();
    }
    format!("{root}mAna")
}

/// 3.2.104 जीर्यतेरतृन्: जॄ → जरत् (गुण; उगित् नुम् जरन्). स्त्री जरती not शतृ *जरन्ती. क्त stays जीर्ण.
fn atfn_form(root: &str) -> String {
    match root {
        "jF" | "jFz" => "jarat".into(),
        other => format!("{}at", apply_guna_to_stem(other)),
    }
}

/// 3.2.146 निन्दहिंसक्लिशखाद…वुञ्: निन्दक/हिंसक/क्लेशक/खादक (ञित् अक).
/// ण्वुल् कारक stays. वुन् is not this list. स्त्री खादिका (not टाप् *खादका).
fn vuy_form(root: &str) -> String {
    match root {
        "KAd" | "KAda" => "KAdaka".into(),
        "nind" | "ninda" | "Rid" | "Ridi" => "nindaka".into(),
        "his" | "hisi" | "hims" | "hiMs" => "hiMsaka".into(),
        "kliS" | "kliSa" => "kleSaka".into(),
        "naS" | "RaS" | "RaSa" => "nASaka".into(),
        "kzip" | "kzipa" => "kzepaka".into(),
        "raw" | "rawa" => "rAwaka".into(),
        "vad" | "vada" => "vAdaka".into(),
        "BAz" | "BAza" => "BAzaka".into(),
        "asUy" | "asUya" => "asUyaka".into(),
        other => format!("{other}aka"),
    }
}

/// 3.3.88 ड्वितः क्त्रिः + 4.4.20 क्त्रेर्मम्: कृत्रिम/पक्त्रिम (कित्; not निष्ठा पक्व).
/// याच् is सेट् याचित्रिम (not palatal *याक्त्रिम).
fn ktri_form(dhatu: &str) -> String {
    let d = dhatu.trim_end_matches('~');
    if !d.starts_with("qu") {
        return format!("{}trima", surface_root(dhatu));
    }
    let root = surface_root(dhatu);
    if matches!(root.as_str(), "yAc") {
        return "yAcitrima".into();
    }
    let ta = nistha_base(dhatu, false);
    if let Some(base) = ta.strip_suffix('a') {
        format!("{base}rima")
    } else {
        format!("{ta}rima")
    }
}

/// 3.3.104 षिद्भिदादिभ्योऽङ्: स्त्री त्रपा/क्षमा/भिदा/छिदा/जरा. अतृन् stays जरत्. कुरच् stays भिदुर.
fn an_stri_form(root: &str) -> String {
    match root {
        "trap" | "trapU" | "trapUz" => "trapA".into(),
        "kzam" | "kzamU" | "kzamUz" => "kzamA".into(),
        "Bid" | "Bidir" => "BidA".into(),
        "Cid" | "Cidir" => "CidA".into(),
        "jF" | "jFz" => "jarA".into(),
        "kzip" => "kzipA".into(),
        "vid" | "vida" => "vidA".into(),
        "guh" => "guhA".into(),
        "kfp" | "krap" => "kfpA".into(),
        other => format!("{other}A"),
    }
}

/// 3.3.57 ॠदोरप्: ऋ/उ-धातु गुण+अ — कर/भव (not घञ् कारक/भाव). एरच् stays separately.
fn rdorap_form(root: &str) -> String {
    match root.chars().last() {
        Some('f' | 'F' | 'u' | 'U') => join_eco(&apply_guna_to_stem(root), "a"),
        _ => format!("{root}a"),
    }
}

/// 3.2.148 चलनशब्दार्थादकर्मकाद् युच्: चलन/चोपन/शब्दन/रवण (7.1.1 अन).
/// 3.2.149 अनुदात्तैतश्च हलादेः: वर्तन/वर्धन. Not सन् *जुगुप्सन dump.
/// 3.2.150 जुचङ्क्रम्यदन्द्रम्यसृगृधिज्वलशुचलषपतपदः: सरण/गर्धन/ज्वलन/शोचन/लषण/पतन/पदन.
/// 3.2.151 क्रुधमण्डार्थेभ्यश्च: क्रोधन/रोषण/मण्डन/भूषण.
/// ण stays ज्वाल. ल्यु stays नन्दन. क्नु stays गृध्नु. Do not dump यङ चङ्क्रमण.
fn yuc_form(root: &str) -> String {
    match root {
        "cal" | "cala" => "calana".into(),
        "cup" | "cupa" => "copana".into(),
        "Sabd" | "Sabda" => "Sabdana".into(),
        "ru" => "ravaRa".into(),
        "vft" | "vftu" => "vartana".into(),
        "vfD" | "vfDu" => "varDana".into(),
        "sf" => "saraRa".into(),
        "gfD" | "gfDa" | "gfDu" => "garDana".into(),
        "jval" | "jvala" => "jvalana".into(),
        "Suc" | "Suca" => "Socana".into(),
        "laz" | "laza" => "lazaRa".into(),
        "pat" | "pata" | "patx" => "patana".into(),
        "pad" | "pada" => "padana".into(),
        "kruD" | "kruDa" => "kroDana".into(),
        "ruz" | "ruza" => "rozaRa".into(),
        "maq" | "maqi" => "maRqana".into(),
        "BUz" | "BUza" => "BUzaRa".into(),
        other => format!("{other}ana"),
    }
}

/// 3.2.154 लषपतपदस्थाभूवृषहनकमगमशॄभ्य उकञ्: लाषुक/पातुक/पादुक/स्थायुक/भावुक/वर्षुक/घातुक/कामुक/गामुक/शारुक.
/// युच् stays लषण/पतन. वरच् stays स्थावर. कः stays स्थ. णिनि stays स्थायी. आरु stays शरारु.
fn ukan_form(root: &str) -> String {
    match root {
        "laz" | "laza" => "lAzuka".into(),
        "pat" | "pata" | "patx" => "pAtuka".into(),
        "pad" | "pada" => "pAduka".into(),
        "sTA" => "sTAyuka".into(),
        "BU" => "BAvuka".into(),
        "vfz" | "vfza" | "vfzu" => "varzuka".into(),
        "han" => "GAtuka".into(),
        "kam" | "kamu" => "kAmuka".into(),
        "gam" | "gamx" => "gAmuka".into(),
        "SF" | "SFY" => "SAruka".into(),
        other => nit_krt_form(other, "ukaY"),
    }
}

/// 3.2.156 प्रजोरिनिः: जविन् (प्रजवी with उपसर्ग). सौत्र जु, not in धातुपाठ.
/// 3.2.157 जिदृक्षिविश्रीण्वमाव्यथाभ्यमपरिभूप्रसूभ्यश्च: जयिन्/दरिन्/क्षयिन्/श्रयिन्/अयिन्/वमिन्;
/// अव्यथिन् निपातन नञ्. णिनि stays ग्राहिन्. क्वरप् stays जित्वर. घिनुण् stays शमिन्.
fn ini_form(root: &str) -> String {
    match root {
        "ju" => "javin".into(),
        "ji" => "jayin".into(),
        "df" => "darin".into(),
        "kzi" => "ksayin".into(),
        "Sri" => "Srayin".into(),
        "i" => "ayin".into(),
        "vam" | "vama" => "vamin".into(),
        "vyaT" | "vyaTa" => "avyaTin".into(),
        "am" | "ama" => "amin".into(),
        "BU" => "Bavin".into(),
        "sU" | "zU" => "savin".into(),
        other => format!("{other}in"),
    }
}

/// 3.1.97 अचो यत्: चेय/जेय; आ → एय देय. 6.1.45+6.4.65 ऐ → गेय/पेय not *गैय.
/// 3.1.98 पोरदुपधात्: शप्य/लभ्य/आप्य (no वृद्धि). ण्यत् stays पाक्य.
/// 3.1.99 शकिसहोश्च: शक्य/सह्य. 3.1.100 गदमदचरयमश्चानुपसर्गे: गद्य/मद्य/चर्य/यम्य.
/// 3.1.102 वह्यं करणम्: वह्य vs ण्यत् वाह्य. क्यप् stays ऊह्य. क्त stays ऊढ.
/// 3.1.105 अजेर् यत्: अज्य vs ण्यत् आग्य (7.3.52 कु). क्त stays अक्त. घञ् stays आग.
/// थकन् stays गाथक. ण्युट् stays गायन. क्त stays गीत. श stays पिब. घिनुण् stays मादिन्.
/// थकन् stays गाथक. ण्युट् stays गायन. क्त stays गीत. श stays पिब. घिनुण् stays मादिन्.
fn yat_form(root: &str) -> String {
    match root {
        "gE" => "geya".into(),
        "pE" => "peya".into(),
        "Sap" | "Sapa" => "Sapya".into(),
        "laB" | "laBa" | "laBaz" => "laBya".into(),
        "Ap" | "Apx" => "Apya".into(),
        "Sak" | "Saka" => "Sakya".into(),
        "sah" | "saha" | "zah" | "zaha" => "sahya".into(),
        "gad" | "gada" => "gadya".into(),
        "mad" | "mada" => "madya".into(),
        "car" | "cara" => "carya".into(),
        "yam" | "yama" => "yamya".into(),
        "vah" | "vaha" => "vahya".into(),
        "aj" | "aja" => "ajya".into(),
        r if r.ends_with('A') => format!("{}eya", &r[..r.len() - 1]),
        other => join_eco(&apply_guna_to_stem(other), "ya"),
    }
}

/// 3.1.134 ग्रह्यादेर् णिनिः: ग्राही/स्थायी/मन्त्री (णित् वृद्धि; आतो युक्; इदित् नुम्).
/// ल्यु stays नन्दन. आलुच् stays गृहयालु. वरच् stays स्थावर. कः stays स्थ. घिनुण् stays शमिन्.
fn nini_form(root: &str) -> String {
    match root {
        "grah" | "graha" => "grAhin".into(),
        "sTA" => "sTAyin".into(),
        "matr" | "matri" | "mantr" => "mantrin".into(),
        other => format!("{other}in"),
    }
}

/// 3.1.140 ज्वलितिकसन्तेभ्यो णः: ज्वाल/चाल; वार्तिक तन् → तान.
/// 3.1.141 श्याद्व्यधास्रुसंस्र्वतीणवसावहृलिहश्लिषश्वसश्च: व्याध/लेह/श्लेष/श्वास/स्राव/श्याय.
/// घञ्/अप् stay कार/कर. श stays पिब. कः stays स्थ. Do not dump gold ण.
fn jvala_ra_form(root: &str) -> String {
    match root {
        "jval" | "jvala" | "cal" | "cala" | "tan" | "tanu" | "sru" => {
            join_eco(&nit_krt_anga(root, "Ra"), "a")
        }
        // इत्-a often kept (लिह् ≠ CaC); णित् on the stripped aṅga, not *लिहा.
        "vyaD" | "vyaDa" => "vyADa".into(),
        "lih" | "liha" => "leha".into(),
        "Sliz" | "Sliza" => "Sleza".into(),
        "Svas" | "Svasa" => "SvAsa".into(),
        "SyE" | "SyEN" | "SyA" => "SyAya".into(),
        other => format!("{other}a"),
    }
}

/// 3.1.137 पाघ्राध्माधेट्दृशः शः: पिब/जिघ्र/धम/धय/पश्य (शित् → 7.3.78).
/// 3.1.139 ददातिदधात्योर्विभाषा: दद/दध (श्लु). क्त stays पीत/दत्त/हित. घञ् stays दाय.
fn sa_krt_form(dhatu: &str, root: &str) -> String {
    let d = dhatu.trim_end_matches('~');
    if d == "Dew" || root == "De" {
        return "Daya".into();
    }
    if d.starts_with("qudA") || d == "dAY" {
        return "dada".into();
    }
    if d.starts_with("quDA") || d == "DAY" {
        return "daDa".into();
    }
    match root {
        "pA" => "piba".into(),
        "GrA" => "jiGra".into(),
        "DmA" => "Dama".into(),
        "dfS" => "paSya".into(),
        other => format!("{other}a"),
    }
}

/// 3.1.135 इगुपधज्ञाप्रीकिरः कः: ज्ञ/प्रिय/किर/बुध/कृश/क्षिप (कित् no गुण).
/// 3.1.136 आतश्चोपसर्गे: ग्ल/स्थ (आ-lopa). श stays पिब. गस्नु stays ग्लास्नु. वुञ् stays क्षेपक.
fn ka_kit_form(root: &str) -> String {
    match root {
        "jYA" => "jYa".into(),
        "prI" => "priya".into(),
        "kF" => "kira".into(),
        "buD" | "buDa" => "buDa".into(),
        "kfS" | "kfSa" => "kfSa".into(),
        "kzip" | "kzipa" => "kzipa".into(),
        "liK" | "liKa" => "liKa".into(),
        "glA" | "glE" => "gla".into(),
        "mlA" | "mlE" => "mla".into(),
        "sTA" => "sTa".into(),
        other => format!("{other}a"),
    }
}

/// क्वसु (3.2.107): लिट् weak aṅga + वस्. बभूवतुः → बभूवस् (not बभूव्वस्).
fn kvasu_form(dhatu: &str) -> String {
    if let Some(forms) = crate::engine::lit::kartari(dhatu, 1, 2, "P") {
        if let Some(du) = forms.first() {
            if let Some(anga) = du.strip_suffix("atuH") {
                if anga.ends_with('v') {
                    return format!("{anga}as");
                }
                return format!("{anga}vas");
            }
        }
    }
    format!("{}vas", surface_root(dhatu))
}

fn is_ac(c: char) -> bool {
    matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

/// 7.2.115/116 वृद्धि (i/ī → ऐ, u/ū → औ, ṛ → आर्, a → आ).
fn vrddhi_ac(root: &str) -> String {
    let chars: Vec<char> = root.chars().collect();
    for idx in (0..chars.len()).rev() {
        let repl = match chars[idx] {
            'a' => Some("A"),
            'i' | 'I' | 'e' => Some("E"),
            'u' | 'U' | 'o' => Some("O"),
            'f' | 'F' => Some("Ar"),
            _ => None,
        };
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
    match s.chars().last() {
        Some('c') => format!("{}k", &s[..s.len() - 1]),
        Some('j') => format!("{}g", &s[..s.len() - 1]),
        _ => s.to_string(),
    }
}

/// णित्/ञित् kṛt aṅga: 7.2.115 अचो ञ्णिति, 7.2.116 अत उपधायाः, 7.3.86 इगुपध गुण,
/// 7.3.33 आतो युक्, 7.3.32/54 हन् → घात्.
/// 3.1.124 ऋहलोर्ण्यत्: कार्य/हार्य/धार्य/वाक्य/पाक्य. 3.1.125 ओरावश्यके: लाव्य vs यत् लव्य.
fn nit_krt_anga(root: &str, pratyaya: &str) -> String {
    if root == "han" {
        return "GAt".into();
    }
    let last = root.chars().last().unwrap_or('a');
    let mut anga = if is_ac(last) {
        if last == 'A' {
            format!("{root}y")
        } else {
            vrddhi_ac(root)
        }
    } else {
        match root.chars().rev().nth(1) {
            Some('a') => vrddhi_ac(root),
            Some('i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'e' | 'o') => apply_guna_to_stem(root),
            _ => root.to_string(),
        }
    };
    if matches!(pratyaya, "GaY" | "Ryat") {
        anga = cajo_ku(&anga);
    }
    anga
}

fn nit_krt_form(root: &str, pratyaya: &str) -> String {
    let suffix = match pratyaya {
        "Rvul" => "aka",
        "ukaY" => "uka",
        "Ryat" => "ya",
        _ => "a",
    };
    join_eco(&nit_krt_anga(root, pratyaya), suffix)
}

fn ktva_base(dhatu: &str) -> String {
    let ta = nistha_base(dhatu, false);
    if let Some(stripped) = ta.strip_suffix("ita") {
        format!("{stripped}itvA")
    } else if let Some(stripped) = ta.strip_suffix("ta") {
        format!("{stripped}tvA")
    } else {
        format!("{ta}tvA")
    }
}

/// 7.1.37 क्त्वो ल्यप् + 6.1.71 तुक् after ह्रस्व: कृत्य/गत्य (not *कृय/*गय).
/// दीर्घ भूय; सेट् इत→य गृह्य. क्त्वा without उपसर्ग stays त्वा.
fn lyap_base(dhatu: &str) -> String {
    let ta = nistha_base(dhatu, false);
    if let Some(stripped) = ta.strip_suffix("ita") {
        format!("{stripped}ya")
    } else if let Some(x) = ta.strip_suffix("ta") {
        if x.chars().last().is_some_and(|c| matches!(c, 'a' | 'i' | 'u' | 'f' | 'x')) {
            format!("{x}tya")
        } else {
            format!("{x}ya")
        }
    } else {
        format!("{ta}ya")
    }
}

pub fn generate(dhatu_query: &str, pratyaya: &str) -> KrdantaResult {
    let forms = derive(dhatu_query, pratyaya);
    KrdantaResult { forms, dhatu: dhatu_query.to_string(), pratyaya: pratyaya.to_string() }
}

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

pub fn is_avyaya(pratyaya: &str) -> bool {
    matches!(pratyaya, "ktvA" | "lyap" | "tumun" | "Ramul" | "am")
}

/// लिङ्गs this kṛt takes. Empty = अव्यय (no सुप्).
pub fn lingas(pratyaya: &str) -> &'static [&'static str] {
    if is_avyaya(pratyaya) {
        return &[];
    }
    match pratyaya {
        "ktin" => &["stri"],
        "aN" => &["stri"],
        "lyuw" => &["nap"],
        "itra" => &["nap"],
        "GaY" => &["pum"],
        _ => &["pum", "stri", "nap"],
    }
}

fn is_at_participle(pratyaya: &str) -> bool {
    matches!(pratyaya, "Satf" | "Satf~" | "sya-Satf" | "sya-Satf~" | "ktavatu" | "ktavatu~")
}

fn pratipadika(form: &str, pratyaya: &str, linga: &str) -> Option<String> {
    if is_avyaya(pratyaya) || form.is_empty() {
        return None;
    }
    if pratyaya == "tfc" && linga == "stri" {
        let base = form.trim_end_matches('f');
        return Some(format!("{base}rI"));
    }
    if is_at_participle(pratyaya) && linga == "stri" {
        if pratyaya.starts_with("ktavatu") {
            if form.ends_with("at") {
                return Some(format!("{form}I"));
            }
        } else if let Some(base) = form.strip_suffix("at") {
            return Some(format!("{base}antI"));
        }
    }
    if linga == "stri"
        && (matches!(
            pratyaya,
            "kta" | "SAnac" | "cAnaS" | "tavya" | "anIyar" | "ac" | "anIya"
                | "yat" | "Ryat" | "kyap" | "kmarac" | "Gurac" | "kurac" | "klukan" | "krukan"
                | "ra" | "naN" | "SAnan" | "ktri" | "ap" | "Ra" | "Sa" | "ka" | "lyu" | "yuc"
                | "ukaY"
        ) || pratyaya.contains("SAnac")
            || pratyaya.contains("cAnaS"))
    {
        if let Some(base) = form.strip_suffix('a') {
            return Some(format!("{base}A"));
        }
    }
    // 3.1.146 थकन् स्त्री गाथिका; 3.1.147 टित् ण्युट् गायनी.
    if pratyaya == "Takan" && linga == "stri" {
        if let Some(base) = form.strip_suffix("aka") {
            return Some(format!("{base}ikA"));
        }
    }
    if pratyaya == "Ryuw" && linga == "stri" {
        if let Some(base) = form.strip_suffix('a') {
            return Some(format!("{base}I"));
        }
    }
    if pratyaya == "kvarap" && linga == "stri" {
        if let Some(base) = form.strip_suffix('a') {
            return Some(format!("{base}I"));
        }
    }
    // 4.1.41 षिद् षाकन्: जल्पाकी not टाप् *जल्पाका.
    if pratyaya == "zAkan" && linga == "stri" {
        if let Some(base) = form.strip_suffix('a') {
            return Some(format!("{base}I"));
        }
    }
    // 4.1.41 षिद् ष्वुन्: नर्तकी/खनकी/रजकी not टाप् *नर्तका / ण्वुल् *नर्तिका.
    if pratyaya == "zvun" && linga == "stri" {
        if let Some(base) = form.strip_suffix("aka") {
            return Some(format!("{base}akI"));
        }
    }
    // 4.1.41 षिद् ष्ट्रन्: नेत्री not टाप् *नेत्रा. दंष्ट्रा is अजादि टाप्.
    if pratyaya == "zwran" && linga == "stri" {
        if form == "daMzwra" {
            return Some("daMzwrA".into());
        }
        if let Some(base) = form.strip_suffix('a') {
            return Some(format!("{base}I"));
        }
    }
    // 3.2.146 वुञ् / 3.1.133 ण्वुल् / वुन् स्त्री: 7.3.44 कात् पूर्वस्य इत् → कारिका not टाप् *कारका.
    // ष्वुन् stays नर्तकी (ङीष्, already above).
    if matches!(pratyaya, "vuY" | "Rvul" | "vun") && linga == "stri" {
        if let Some(base) = form.strip_suffix("aka") {
            return Some(format!("{base}ikA"));
        }
    }
    // 3.2.104 अतृन् स्त्री जरती (ङीप् on अत), not शतृ भवन्ती.
    if pratyaya == "atfn" && linga == "stri" && form.ends_with("at") {
        return Some(format!("{form}I"));
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
    if !lingas(pratyaya).contains(&linga) {
        return None;
    }
    let res = generate_with_prefixes(dhatu_query, pratyaya, prefixes);
    let form = res.forms.first()?.as_str();
    let stem = pratipadika(form, pratyaya, linga)?;
    let mut d = crate::declension::subanta::generate(&stem, linga)?;
    // 6.4.14 अत्वसन्तस्य चाधातोः: शतृ has no दीर्घ (भवन् not भवान्). क्तवतु keeps आन्.
    if matches!(
        pratyaya,
        "Satf" | "Satf~" | "sya-Satf" | "sya-Satf~" | "atfn"
    ) && linga == "pum" {
        if let Some(row) = d.declension.get_mut("prathamA") {
            if let Some(nom) = row.first_mut() {
                if let Some(base) = nom.strip_suffix("An") {
                    *nom = format!("{base}an");
                }
            }
        }
    }
    Some(d)
}

pub fn derive(dhatu_query: &str, pratyaya: &str) -> Vec<String> {
    let (dhatu, gana, tags, ant, aup) = load_dhatu(dhatu_query);
    let rule = pratyaya_rule(pratyaya);
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
            if pratyaya == "Satf" || pratyaya == "sya-Satf" {
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
            if pratyaya.starts_with("ktavatu") { format!("{base}vat") } else { base }
        }
        "guna" => {
            match pratyaya {
                "lyuw" | "lyu" => crate::engine::it::lyuw_form(&root),
                "tfc" => crate::engine::it::tfc_form(&root),
                "ktin" => ktin_form(&root),
                "GaY" | "Rvul" | "Ryat" => nit_krt_form(&root, pratyaya),
                "Ramul" => join_eco(&nit_krt_anga(&root, "Rvul"), "am"),
                _ => join_eco(&guna, suffix),
            }
        }
        "guna_a" => join_eco(&guna, "a"),
        "kyap" => kyap_form(&root),
        "gsnu" => gsnu_form(&root),
        "knu" => knu_form(&root),
        "ghinun" => ghinun_form(&root),
        "kvarap" => kvarap_form(&root),
        "aluc" => aluc_form(&root),
        "kmarac" => kmarac_form(&root),
        "gurac" => gurac_form(&root),
        "varac" => varac_form(&root),
        "itra" => itra_form(&root),
        "zwran" => zwran_form(&root),
        "kurac" => kurac_form(&root),
        "kru" => bhi_kru_form(&root, "kru"),
        "klukan" => bhi_kru_form(&root, "klukan"),
        "krukan" => bhi_kru_form(&root, "krukan"),
        "aru" => aru_form(&root),
        "ra" => ra_form(&root),
        "sanu" => san_u_form(&dhatu, &root),
        "nan" => nan_form(&root),
        "athuc" => athuc_form(&root),
        "nvanip" => nvanip_form(&root),
        "takan" => takan_form(&root),
        "ryuw" => ryuw_form(&root),
        "svapnan" => svapnan_form(&root),
        "najin" => najin_form(&root),
        "sakan" => sakan_form(&root),
        "zvun" => zvun_form(&root),
        "sanan" => sanan_form(&dhatu, &root),
        "atfn" => atfn_form(&root),
        "vuy" => vuy_form(&root),
        "ktri" => ktri_form(&dhatu),
        "an_stri" => an_stri_form(&root),
        "rdorap" => rdorap_form(&root),
        "jvala_ra" => jvala_ra_form(&root),
        "sa_krt" => sa_krt_form(&dhatu, &root),
        "ka_kit" => ka_kit_form(&root),
        "nini" => nini_form(&root),
        "yuc" => yuc_form(&root),
        "ukan" => ukan_form(&root),
        "ini" => ini_form(&root),
        "yat" => yat_form(&root),
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

fn present_stem(dhatu: &str, gana: u8) -> String {
    let guna = apply_guna_to_stem(dhatu);
    if gana == 10 { return format!("{}aya", guna); }
    if gana == 4 {
        for idx in (0..dhatu.len()).rev() {
            let ch = dhatu.chars().nth(idx).unwrap();
            if "iIuUfF".contains(ch) {
                let long_v = match ch { 'i' => 'I', 'u' => 'U', 'f' => 'F', _ => ch };
                let mut out = String::new();
                for (i,c) in dhatu.chars().enumerate() {
                    if i==idx { out.push(long_v); } else { out.push(c); }
                }
                return format!("{}ya", out);
            }
        }
        return format!("{}ya", guna);
    }
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
        // 7.1.37 ल्यप् from क्त्वा: कृत्य/गत्य not *कृय/*गय. क्त्वा stays कृत्वा.
        assert_eq!(derive("qukfY", "lyap"), vec!["kftya"]);
        assert_eq!(derive("gam", "lyap"), vec!["gatya"]);
        assert_eq!(derive("BU", "lyap"), vec!["BUya"]);
        assert_eq!(derive("qukfY", "ktvA"), vec!["kftvA"]);
        let f = generate_with_prefixes("qukfY", "ktvA", &["pra".into()]);
        assert!(f.forms.iter().any(|x| x == "prakftya"), "{:?}", f.forms);
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
        let d = decline("Samu", "GinuR", "pum", &[]).expect("SamI");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "SamI"), "{:?}", pr);
        let d = decline("Samu", "GinuR", "stri", &[]).expect("SaminI");
        assert_eq!(d.stem, "SaminI");
        let d = decline("ji", "kvarap", "pum", &[]).expect("jitvaraH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "jitvaraH"), "{:?}", pr);
        let d = decline("ji", "kvarap", "stri", &[]).expect("jitvarI");
        assert_eq!(d.stem, "jitvarI");
        let d = decline("daya", "Aluc", "pum", &[]).expect("dayAluH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "dayAluH"), "{:?}", pr);
        let d = decline("ad", "kmarac", "pum", &[]).expect("admaraH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "admaraH"), "{:?}", pr);
        let d = decline("ad", "kmarac", "stri", &[]).expect("admarA");
        assert_eq!(d.stem, "admarA");
        let d = decline("BAsf", "Gurac", "pum", &[]).expect("BAsuraH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "BAsuraH"), "{:?}", pr);
        let d = decline("ISa", "varac", "pum", &[]).expect("ISvaraH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "ISvaraH"), "{:?}", pr);
        let d = decline("RIY", "zwran", "nap", &[]).expect("netram");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "netram"), "{:?}", pr);
        let d = decline("RIY", "zwran", "stri", &[]).expect("netrI");
        assert_eq!(d.stem, "netrI");
        let d = decline("danSa", "zwran", "stri", &[]).expect("daMzwrA");
        assert_eq!(d.stem, "daMzwrA");
        let d = decline("vida", "kurac", "pum", &[]).expect("viduraH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "viduraH"), "{:?}", pr);
        let d = decline("vida", "kurac", "stri", &[]).expect("vidurA");
        assert_eq!(d.stem, "vidurA");
        let d = decline("YiBI", "kru", "pum", &[]).expect("BIruH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "BIruH"), "{:?}", pr);
        let d = decline("vadi", "Aru", "pum", &[]).expect("vandAruH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "vandAruH"), "{:?}", pr);
        let d = decline("Rama", "ra", "pum", &[]).expect("namraH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "namraH"), "{:?}", pr);
        let d = decline("Bikza", "u", "pum", &[]).expect("BikzuH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "BikzuH"), "{:?}", pr);
        let d = decline("yaja", "naN", "pum", &[]).expect("yajYaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "yajYaH"), "{:?}", pr);
        let d = decline("yaja", "Nvanip", "pum", &[]).expect("yajvA");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "yajvA"), "{:?}", pr);
        let tr = d.declension.get("tfIyA").unwrap();
        assert!(tr.iter().any(|x| x == "yajvanA"), "{:?}", tr);
        let d = decline("gE", "Takan", "pum", &[]).expect("gATakaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gATakaH"), "{:?}", pr);
        let d = decline("gE", "Takan", "stri", &[]).expect("gATikA");
        assert_eq!(d.stem, "gATikA");
        let d = decline("gE", "Ryuw", "pum", &[]).expect("gAyanaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gAyanaH"), "{:?}", pr);
        let d = decline("gE", "Ryuw", "stri", &[]).expect("gAyanI");
        assert_eq!(d.stem, "gAyanI");
        let d = decline("Yizvapa", "nan", "pum", &[]).expect("svapnaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "svapnaH"), "{:?}", pr);
        let d = decline("Yizvapa", "najiN", "pum", &[]).expect("svapnak");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "svapnak"), "{:?}", pr);
        let d = decline("jalpa", "zAkan", "pum", &[]).expect("jalpAkaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "jalpAkaH"), "{:?}", pr);
        let d = decline("jalpa", "zAkan", "stri", &[]).expect("jalpAkI");
        assert_eq!(d.stem, "jalpAkI");
        let d = decline("nftI", "zvun", "pum", &[]).expect("nartakaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "nartakaH"), "{:?}", pr);
        let d = decline("nftI", "zvun", "stri", &[]).expect("nartakI");
        assert_eq!(d.stem, "nartakI");
        let d = decline("Kanu", "zvun", "stri", &[]).expect("KanakI");
        assert_eq!(d.stem, "KanakI");
        let d = decline("pUN", "SAnan", "pum", &[]).expect("pavamAnaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "pavamAnaH"), "{:?}", pr);
        let d = decline("pUN", "SAnan", "stri", &[]).expect("pavamAnA");
        assert_eq!(d.stem, "pavamAnA");
        let d = decline("yaja", "SAnan", "pum", &[]).expect("yajamAnaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "yajamAnaH"), "{:?}", pr);
        let d = decline("jFz", "atfn", "pum", &[]).expect("jaran");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "jaran"), "{:?}", pr);
        let d = decline("jFz", "atfn", "stri", &[]).expect("jaratI");
        assert_eq!(d.stem, "jaratI");
        let d = decline("KAdf", "vuY", "pum", &[]).expect("KAdakaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "KAdakaH"), "{:?}", pr);
        let d = decline("KAdf", "vuY", "stri", &[]).expect("KAdikA");
        assert_eq!(d.stem, "KAdikA");
        let d = decline("qukfY", "Rvul", "pum", &[]).expect("kArakaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "kArakaH"), "{:?}", pr);
        let d = decline("qukfY", "Rvul", "stri", &[]).expect("kArikA");
        assert_eq!(d.stem, "kArikA");
        let d = decline("BU", "vun", "stri", &[]).expect("BavikA");
        assert_eq!(d.stem, "BavikA");
        let d = decline("nftI", "zvun", "stri", &[]).expect("nartakI");
        assert_eq!(d.stem, "nartakI");
        let d = decline("qukfY", "ktri", "pum", &[]).expect("kftrimaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "kftrimaH"), "{:?}", pr);
        let d = decline("qukfY", "ktri", "stri", &[]).expect("kftrimA");
        assert_eq!(d.stem, "kftrimA");
        let d = decline("trapUz", "aN", "stri", &[]).expect("trapA");
        assert_eq!(d.stem, "trapA");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "trapA"), "{:?}", pr);
        let d = decline("qukfY", "ap", "pum", &[]).expect("karaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "karaH"), "{:?}", pr);
        let d = decline("jvala", "Ra", "pum", &[]).expect("jvAlaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "jvAlaH"), "{:?}", pr);
        let d = decline("vyaDa", "Ra", "pum", &[]).expect("vyADaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "vyADaH"), "{:?}", pr);
        let d = decline("pA", "Sa", "pum", &[]).expect("pibaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "pibaH"), "{:?}", pr);
        let d = decline("pA", "Sa", "stri", &[]).expect("pibA");
        assert_eq!(d.stem, "pibA");
        let d = decline("jYA", "ka", "pum", &[]).expect("jYaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "jYaH"), "{:?}", pr);
        let d = decline("wunadi", "aTuc", "pum", &[]).expect("nandaTuH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "nandaTuH"), "{:?}", pr);
        let d = decline("wunadi", "lyu", "pum", &[]).expect("nandanaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "nandanaH"), "{:?}", pr);
        let d = decline("wunadi", "lyu", "stri", &[]).expect("nandanA");
        assert_eq!(d.stem, "nandanA");
        let d = decline("graha", "Nini", "pum", &[]).expect("grAhI");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "grAhI"), "{:?}", pr);
        let d = decline("graha", "Nini", "stri", &[]).expect("grAhinI");
        assert_eq!(d.stem, "grAhinI");
        let d = decline("cala", "yuc", "pum", &[]).expect("calanaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "calanaH"), "{:?}", pr);
        let d = decline("cala", "yuc", "stri", &[]).expect("calanA");
        assert_eq!(d.stem, "calanA");
        let d = decline("kamu", "ukaY", "pum", &[]).expect("kAmukaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "kAmukaH"), "{:?}", pr);
        let d = decline("kamu", "ukaY", "stri", &[]).expect("kAmukA");
        assert_eq!(d.stem, "kAmukA");
        let d = decline("ji", "ini", "pum", &[]).expect("jayI");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "jayI"), "{:?}", pr);
        let d = decline("ji", "ini", "stri", &[]).expect("jayinI");
        assert_eq!(d.stem, "jayinI");
        let d = decline("SAsu", "kyap", "pum", &[]).expect("SiSyaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "SiSyaH"), "{:?}", pr);
        let d = decline("SAsu", "kyap", "stri", &[]).expect("SiSyA");
        assert_eq!(d.stem, "SiSyA");
        let d = decline("gE", "yat", "pum", &[]).expect("geyaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "geyaH"), "{:?}", pr);
        let d = decline("gE", "yat", "nap", &[]).expect("geyam");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "geyam"), "{:?}", pr);
        let d = decline("Saka", "yat", "nap", &[]).expect("Sakyam");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "Sakyam"), "{:?}", pr);
        let d = decline("hfY", "Ryat", "pum", &[]).expect("hAryaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "hAryaH"), "{:?}", pr);
        let d = decline("qukfY", "kyap", "stri", &[]).expect("kftyA");
        assert_eq!(d.stem, "kftyA");
        let d = decline("yujir", "kyap", "pum", &[]).expect("yugyaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "yugyaH"), "{:?}", pr);
        let d = decline("BU", "kyap", "pum", &[]).expect("BUyaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "BUyaH"), "{:?}", pr);
        let d = decline("vaha", "yat", "nap", &[]).expect("vahyam");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "vahyam"), "{:?}", pr);
        let d = decline("aja", "yat", "nap", &[]).expect("ajyam");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "ajyam"), "{:?}", pr);
        let d = decline("vfDu", "kyap", "pum", &[]).expect("vfDyaH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "vfDyaH"), "{:?}", pr);
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
        assert_eq!(lingas("lyu"), &["pum", "stri", "nap"]);
        assert_eq!(lingas("itra"), &["nap"]);
        assert!(decline("f", "itra", "pum", &[]).is_none());
        let d = decline("f", "itra", "nap", &[]).expect("aritram");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "aritram"), "{:?}", pr);
        assert_eq!(lingas("ktin"), &["stri"]);
        assert_eq!(lingas("aN"), &["stri"]);
        assert!(decline("trapUz", "aN", "pum", &[]).is_none());
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
        assert_eq!(derive("gE", "yat"), vec!["geya"]);
        assert_eq!(derive("pE", "yat"), vec!["peya"]);
        assert_eq!(derive("ciY", "yat"), vec!["ceya"]);
        assert_eq!(derive("ji", "yat"), vec!["jeya"]);
        assert_eq!(derive("gE", "Takan"), vec!["gATaka"]);
        assert_eq!(derive("gE", "Ryuw"), vec!["gAyana"]);
        assert_eq!(derive("gE", "kta"), vec!["gIta"]);
        assert_eq!(derive("pA", "Sa"), vec!["piba"]);
        assert_eq!(derive("Sapa", "yat"), vec!["Sapya"]);
        assert_eq!(derive("qulaBaz", "yat"), vec!["laBya"]);
        assert_eq!(derive("Apx", "yat"), vec!["Apya"]);
        assert_eq!(derive("Saka", "yat"), vec!["Sakya"]);
        assert_eq!(derive("zaha", "yat"), vec!["sahya"]);
        assert_eq!(derive("gada", "yat"), vec!["gadya"]);
        assert_eq!(derive("madI", "yat"), vec!["madya"]);
        assert_eq!(derive("cara", "yat"), vec!["carya"]);
        assert_eq!(derive("yama", "yat"), vec!["yamya"]);
        assert_eq!(derive("qupacaz", "Ryat"), vec!["pAkya"]);
        assert_eq!(derive("madI", "GinuR"), vec!["mAdin"]);
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
        assert_eq!(derive("hfY", "Ryat"), vec!["hArya"]);
        assert_eq!(derive("DfY", "Ryat"), vec!["DArya"]);
        assert_eq!(derive("vaca", "Ryat"), vec!["vAkya"]);
        assert_eq!(derive("lUY", "Ryat"), vec!["lAvya"]);
        assert_eq!(derive("pUY", "Ryat"), vec!["pAvya"]);
        assert_eq!(derive("lUY", "yat"), vec!["lavya"]);
        assert_eq!(derive("pUY", "yat"), vec!["pavya"]);
        assert_eq!(derive("qukfY", "kyap"), vec!["kftya"]);
        assert_eq!(derive("vfzu", "kyap"), vec!["vfzya"]);
        assert_eq!(derive("vfzu", "Ryat"), vec!["varzya"]);
        assert_eq!(derive("yujir", "kyap"), vec!["yugya"]);
        assert_eq!(derive("yujir", "Ryat"), vec!["yogya"]);
        assert_eq!(derive("yujir", "kta"), vec!["yukta"]);
        assert_eq!(derive("yujir", "GaY"), vec!["yoga"]);
        assert_eq!(derive("BU", "kyap"), vec!["BUya"]);
        assert_eq!(derive("BU", "yat"), vec!["Bavya"]);
        assert_eq!(derive("BU", "Ryat"), vec!["BAvya"]);
        assert_eq!(derive("BU", "lyap"), vec!["BUya"]);
        assert_eq!(derive("BU", "ukaY"), vec!["BAvuka"]);
        assert_eq!(derive("BU", "ini"), vec!["Bavin"]);
        let f = generate_with_prefixes("BU", "kyap", &["pra".into()]);
        assert!(f.forms.iter().any(|x| x == "praBUya"), "{:?}", f.forms);
        let f = generate_with_prefixes("BU", "yat", &["pra".into()]);
        assert!(f.forms.iter().any(|x| x == "praBavya"), "{:?}", f.forms);
        assert_eq!(derive("vaha", "yat"), vec!["vahya"]);
        assert_eq!(derive("vaha", "Ryat"), vec!["vAhya"]);
        assert_eq!(derive("vaha", "kta"), vec!["UQa"]);
        assert_eq!(derive("vaha", "kyap"), vec!["uhya"]);
        assert_eq!(derive("vaha", "GaY"), vec!["vAha"]);
        assert_eq!(derive("aja", "yat"), vec!["ajya"]);
        assert_eq!(derive("aja", "Ryat"), vec!["Agya"]);
        assert_eq!(derive("aja", "kta"), vec!["akta"]);
        assert_eq!(derive("aja", "GaY"), vec!["Aga"]);
        assert_eq!(derive("vfDu", "kyap"), vec!["vfDya"]);
        assert_eq!(derive("vfDu", "Ryat"), vec!["varDya"]);
        assert_eq!(derive("vfDu", "yuc"), vec!["varDana"]);
        assert_eq!(derive("vfDu", "kta"), vec!["vfdDa"]);
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
        assert_eq!(derive("SAsu", "kyap"), vec!["SiSya"]);
        assert_eq!(derive("iR", "kyap"), vec!["itya"]);
        assert_eq!(derive("juzI", "kyap"), vec!["juzya"]);
        assert_eq!(derive("dfN", "kyap"), vec!["dftya"]);
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
        assert_eq!(derive("Samu", "GinuR"), vec!["Samin"]);
        assert_eq!(derive("tamu", "GinuR"), vec!["tamin"]);
        assert_eq!(derive("madI", "GinuR"), vec!["mAdin"]);
        assert_eq!(derive("kzamU", "GinuR"), vec!["kzamin"]);
        assert_eq!(derive("ji", "kvarap"), vec!["jitvara"]);
        assert_eq!(derive("sf", "kvarap"), vec!["sftvara"]);
        assert_eq!(derive("iR", "kvarap"), vec!["itvara"]);
        assert_eq!(derive("naS", "kvarap"), vec!["naSvara"]);
        assert_eq!(derive("gam", "kvarap"), vec!["gatvara"]);
        assert_eq!(derive("daya", "Aluc"), vec!["dayAlu"]);
        assert_eq!(derive("spfha", "Aluc"), vec!["spfhayAlu"]);
        assert_eq!(derive("graha", "Aluc"), vec!["gfhayAlu"]);
        assert_eq!(derive("patx", "Aluc"), vec!["patayAlu"]);
        assert_eq!(derive("sf", "kmarac"), vec!["sfmara"]);
        assert_eq!(derive("Gas", "kmarac"), vec!["Gasmara"]);
        assert_eq!(derive("ad", "kmarac"), vec!["admara"]);
        assert_eq!(derive("BAsf", "Gurac"), vec!["BAsura"]);
        assert_eq!(derive("YimidA", "Gurac"), vec!["medura"]);
        assert_eq!(derive("Banjo", "Gurac"), vec!["BaNgura"]);
        assert_eq!(derive("zWA", "varac"), vec!["sTAvara"]);
        assert_eq!(derive("ISa", "varac"), vec!["ISvara"]);
        assert_eq!(derive("BAsf", "varac"), vec!["BAsvara"]);
        assert_eq!(derive("pisf", "varac"), vec!["pesvara"]);
        assert_eq!(derive("kasa", "varac"), vec!["kasvara"]);
        assert_eq!(derive("f", "itra"), vec!["aritra"]);
        assert_eq!(derive("cara", "itra"), vec!["caritra"]);
        assert_eq!(derive("Kanu", "itra"), vec!["Kanitra"]);
        assert_eq!(derive("zaha", "itra"), vec!["sahitra"]);
        assert_eq!(derive("lUY", "itra"), vec!["lavitra"]);
        assert_eq!(derive("DU", "itra"), vec!["Duvitra"]);
        assert_eq!(derive("zUN", "itra"), vec!["savitra"]);
        assert_eq!(derive("RIY", "zwran"), vec!["netra"]);
        assert_eq!(derive("quDAY", "zwran"), vec!["DAtra"]);
        assert_eq!(derive("qudAY", "zwran"), vec!["dAtra"]);
        assert_eq!(derive("Sasu", "zwran"), vec!["Sastra"]);
        assert_eq!(derive("miha", "zwran"), vec!["meQra"]);
        assert_eq!(derive("yuja", "zwran"), vec!["yoktra"]);
        assert_eq!(derive("danSa", "zwran"), vec!["daMzwra"]);
        assert_eq!(derive("vida", "kurac"), vec!["vidura"]);
        assert_eq!(derive("Bidir", "kurac"), vec!["Bidura"]);
        assert_eq!(derive("Cidir", "kurac"), vec!["Cidura"]);
        assert_eq!(derive("YiBI", "kru"), vec!["BIru"]);
        assert_eq!(derive("YiBI", "klukan"), vec!["BIluka"]);
        assert_eq!(derive("YiBI", "krukan"), vec!["BIruka"]);
        assert_eq!(derive("SFY", "Aru"), vec!["SarAru"]);
        assert_eq!(derive("vadi", "Aru"), vec!["vandAru"]);
        assert_eq!(derive("Rama", "ra"), vec!["namra"]);
        assert_eq!(derive("kapi", "ra"), vec!["kampra"]);
        assert_eq!(derive("zmiN", "ra"), vec!["smera"]);
        assert_eq!(derive("jasu", "ra"), vec!["jasra"]);
        assert_eq!(derive("dIpI", "ra"), vec!["dIpra"]);
        assert_eq!(derive("Bikza", "u"), vec!["Bikzu"]);
        assert_eq!(derive("gupa", "u"), vec!["jugupsu"]);
        assert_eq!(derive("tija", "u"), vec!["titikzu"]);
        assert_eq!(derive("kita", "u"), vec!["cikitsu"]);
        assert_eq!(derive("qukfY", "u"), vec!["cikIrzu"]);
        assert_eq!(derive("mAna", "u"), vec!["mImAMsu"]);
        assert_eq!(derive("yaja", "naN"), vec!["yajYa"]);
        assert_eq!(derive("yatI", "naN"), vec!["yatna"]);
        assert_eq!(derive("quyAcf", "naN"), vec!["yAcYA"]);
        assert_eq!(derive("praCa", "naN"), vec!["praSna"]);
        assert_eq!(derive("rakza", "naN"), vec!["rakzRa"]);
        assert_eq!(derive("wunadi", "aTuc"), vec!["nandaTu"]);
        assert_eq!(derive("wunadi", "lyu"), vec!["nandana"]);
        assert_eq!(derive("wunadi", "lyuw"), vec!["nandana"]);
        assert_eq!(derive("qukfY", "lyuw"), vec!["karaRa"]);
        assert_eq!(derive("graha", "Nini"), vec!["grAhin"]);
        assert_eq!(derive("zWA", "Nini"), vec!["sTAyin"]);
        assert_eq!(derive("matri", "Nini"), vec!["mantrin"]);
        assert_eq!(derive("graha", "Aluc"), vec!["gfhayAlu"]);
        assert_eq!(derive("zWA", "varac"), vec!["sTAvara"]);
        assert_eq!(derive("zWA", "ka"), vec!["sTa"]);
        assert_eq!(derive("Samu", "GinuR"), vec!["Samin"]);
        assert_eq!(derive("cala", "yuc"), vec!["calana"]);
        assert_eq!(derive("cupa", "yuc"), vec!["copana"]);
        assert_eq!(derive("Sabda", "yuc"), vec!["Sabdana"]);
        assert_eq!(derive("ru", "yuc"), vec!["ravaRa"]);
        assert_eq!(derive("sf", "yuc"), vec!["saraRa"]);
        assert_eq!(derive("gfDu", "yuc"), vec!["garDana"]);
        assert_eq!(derive("jvala", "yuc"), vec!["jvalana"]);
        assert_eq!(derive("Suca", "yuc"), vec!["Socana"]);
        assert_eq!(derive("laza", "yuc"), vec!["lazaRa"]);
        assert_eq!(derive("patx", "yuc"), vec!["patana"]);
        assert_eq!(derive("pada", "yuc"), vec!["padana"]);
        assert_eq!(derive("vftu", "yuc"), vec!["vartana"]);
        assert_eq!(derive("vfDu", "yuc"), vec!["varDana"]);
        assert_eq!(derive("kruDa", "yuc"), vec!["kroDana"]);
        assert_eq!(derive("ruza", "yuc"), vec!["rozaRa"]);
        assert_eq!(derive("maqi", "yuc"), vec!["maRqana"]);
        assert_eq!(derive("BUza", "yuc"), vec!["BUzaRa"]);
        assert_eq!(derive("laza", "ukaY"), vec!["lAzuka"]);
        assert_eq!(derive("patx", "ukaY"), vec!["pAtuka"]);
        assert_eq!(derive("pada", "ukaY"), vec!["pAduka"]);
        assert_eq!(derive("zWA", "ukaY"), vec!["sTAyuka"]);
        assert_eq!(derive("vfzu", "ukaY"), vec!["varzuka"]);
        assert_eq!(derive("hana", "ukaY"), vec!["GAtuka"]);
        assert_eq!(derive("kamu", "ukaY"), vec!["kAmuka"]);
        assert_eq!(derive("gamx", "ukaY"), vec!["gAmuka"]);
        assert_eq!(derive("SFY", "ukaY"), vec!["SAruka"]);
        assert_eq!(derive("laza", "yuc"), vec!["lazaRa"]);
        assert_eq!(derive("zWA", "Nini"), vec!["sTAyin"]);
        assert_eq!(derive("SFY", "Aru"), vec!["SarAru"]);
        assert_eq!(derive("ju", "ini"), vec!["javin"]);
        assert_eq!(derive("ji", "ini"), vec!["jayin"]);
        assert_eq!(derive("dfN", "ini"), vec!["darin"]);
        assert_eq!(derive("kzi", "ini"), vec!["ksayin"]);
        assert_eq!(derive("SriY", "ini"), vec!["Srayin"]);
        assert_eq!(derive("iR", "ini"), vec!["ayin"]);
        assert_eq!(derive("wuvama", "ini"), vec!["vamin"]);
        assert_eq!(derive("vyaTa", "ini"), vec!["avyaTin"]);
        assert_eq!(derive("ama", "ini"), vec!["amin"]);
        assert_eq!(derive("BU", "ini"), vec!["Bavin"]);
        assert_eq!(derive("zU", "ini"), vec!["savin"]);
        let f = generate_with_prefixes("ju", "ini", &["pra".into()]);
        assert!(f.forms.iter().any(|x| x == "prajavin"), "{:?}", f.forms);
        let f = generate_with_prefixes("iR", "ini", &["ati".into()]);
        assert!(f.forms.iter().any(|x| x == "atyayin"), "{:?}", f.forms);
        let f = generate_with_prefixes("SriY", "ini", &["vi".into()]);
        assert!(f.forms.iter().any(|x| x == "viSrayin"), "{:?}", f.forms);
        let f = generate_with_prefixes("BU", "ini", &["pari".into()]);
        assert!(f.forms.iter().any(|x| x == "pariBavin"), "{:?}", f.forms);
        let f = generate_with_prefixes("zU", "ini", &["pra".into()]);
        assert!(f.forms.iter().any(|x| x == "prasavin"), "{:?}", f.forms);
        let f = generate_with_prefixes("ama", "ini", &["aBi".into()]);
        assert!(f.forms.iter().any(|x| x == "aByamin"), "{:?}", f.forms);
        assert_eq!(derive("ji", "kvarap"), vec!["jitvara"]);
        assert_eq!(derive("graha", "Nini"), vec!["grAhin"]);
        assert_eq!(derive("Samu", "GinuR"), vec!["Samin"]);
        assert_eq!(derive("jvala", "Ra"), vec!["jvAla"]);
        assert_eq!(derive("cala", "Ra"), vec!["cAla"]);
        assert_eq!(derive("wunadi", "lyu"), vec!["nandana"]);
        assert_eq!(derive("gfDu", "knu"), vec!["gfDnu"]);
        assert_eq!(derive("patx", "Aluc"), vec!["patayAlu"]);
        assert_eq!(derive("wuvepf", "aTuc"), vec!["vepaTu"]);
        assert_eq!(derive("wuBrAjf", "aTuc"), vec!["BrAjaTu"]);
        assert_eq!(derive("yaja", "Nvanip"), vec!["yajvan"]);
        assert_eq!(derive("zuY", "Nvanip"), vec!["sutvan"]);
        assert_eq!(derive("gE", "Takan"), vec!["gATaka"]);
        assert_eq!(derive("gE", "Ryuw"), vec!["gAyana"]);
        assert_eq!(derive("ohAk", "Ryuw"), vec!["hAyana"]);
        assert_eq!(derive("Yizvapa", "nan"), vec!["svapna"]);
        assert_eq!(derive("Yizvapa", "najiN"), vec!["svapnaj"]);
        assert_eq!(derive("YitfzA", "najiN"), vec!["tfzRaj"]);
        assert_eq!(derive("jalpa", "zAkan"), vec!["jalpAka"]);
        assert_eq!(derive("Bikza", "zAkan"), vec!["BikzAka"]);
        assert_eq!(derive("Bikza", "u"), vec!["Bikzu"]);
        assert_eq!(derive("kuwwa", "zAkan"), vec!["kuwwAka"]);
        assert_eq!(derive("lunwa", "zAkan"), vec!["luRwAka"]);
        assert_eq!(derive("vfN", "zAkan"), vec!["varAka"]);
        assert_eq!(derive("nftI", "zvun"), vec!["nartaka"]);
        assert_eq!(derive("Kanu", "zvun"), vec!["Kanaka"]);
        assert_eq!(derive("Kanu", "itra"), vec!["Kanitra"]);
        assert_eq!(derive("ranja", "zvun"), vec!["rajaka"]);
        assert_eq!(derive("raYj", "zvun"), vec!["rajaka"]);
        assert_eq!(derive("pUN", "SAnan"), vec!["pavamAna"]);
        assert_eq!(derive("yaja", "SAnan"), vec!["yajamAna"]);
        assert_eq!(derive("jFz", "atfn"), vec!["jarat"]);
        assert_eq!(derive("jF", "atfn"), vec!["jarat"]);
        assert_eq!(derive("KAdf", "vuY"), vec!["KAdaka"]);
        assert_eq!(derive("Ridi", "vuY"), vec!["nindaka"]);
        assert_eq!(derive("hisi", "vuY"), vec!["hiMsaka"]);
        assert_eq!(derive("kliSa", "vuY"), vec!["kleSaka"]);
        assert_eq!(derive("qukfY", "Rvul"), vec!["kAraka"]);
        assert_eq!(derive("qukfY", "ktri"), vec!["kftrima"]);
        assert_eq!(derive("qupacaz", "ktri"), vec!["paktrima"]);
        assert_eq!(derive("qupacaz", "kta"), vec!["pakva"]);
        assert_eq!(derive("quyAcf", "ktri"), vec!["yAcitrima"]);
        assert_eq!(derive("qulaBaz", "ktri"), vec!["labDrima"]);
        assert_eq!(derive("quvapa", "ktri"), vec!["uptrima"]);
        assert_eq!(derive("quDAY", "ktri"), vec!["hitrima"]);
        assert_eq!(derive("qudAY", "ktri"), vec!["dattrima"]);
        assert_eq!(derive("quBfY", "ktri"), vec!["Bftrima"]);
        assert_eq!(derive("qumiY", "ktri"), vec!["mitrima"]);
        assert_eq!(derive("qukrIY", "ktri"), vec!["krItrima"]);
        assert_eq!(derive("trapUz", "aN"), vec!["trapA"]);
        assert_eq!(derive("kzamUz", "aN"), vec!["kzamA"]);
        assert_eq!(derive("Bidir", "aN"), vec!["BidA"]);
        assert_eq!(derive("Cidir", "aN"), vec!["CidA"]);
        assert_eq!(derive("jFz", "aN"), vec!["jarA"]);
        assert_eq!(derive("jFz", "atfn"), vec!["jarat"]);
        assert_eq!(derive("Bidir", "kurac"), vec!["Bidura"]);
        assert_eq!(derive("qukfY", "ap"), vec!["kara"]);
        assert_eq!(derive("BU", "ap"), vec!["Bava"]);
        assert_eq!(derive("qukfY", "GaY"), vec!["kAra"]);
        assert_eq!(derive("jvala", "Ra"), vec!["jvAla"]);
        assert_eq!(derive("cala", "Ra"), vec!["cAla"]);
        assert_eq!(derive("tanu", "Ra"), vec!["tAna"]);
        assert_eq!(derive("vyaDa", "Ra"), vec!["vyADa"]);
        assert_eq!(derive("liha", "Ra"), vec!["leha"]);
        assert_eq!(derive("Sliza", "Ra"), vec!["Sleza"]);
        assert_eq!(derive("Svasa", "Ra"), vec!["SvAsa"]);
        assert_eq!(derive("sru", "Ra"), vec!["srAva"]);
        assert_eq!(derive("SyEN", "Ra"), vec!["SyAya"]);
        assert_eq!(derive("pA", "Sa"), vec!["piba"]);
        assert_eq!(derive("GrA", "Sa"), vec!["jiGra"]);
        assert_eq!(derive("DmA", "Sa"), vec!["Dama"]);
        assert_eq!(derive("Dew", "Sa"), vec!["Daya"]);
        assert_eq!(derive("dfSir", "Sa"), vec!["paSya"]);
        assert_eq!(derive("qudAY", "Sa"), vec!["dada"]);
        assert_eq!(derive("quDAY", "Sa"), vec!["daDa"]);
        assert_eq!(derive("pA", "kta"), vec!["pIta"]);
        assert_eq!(derive("qudAY", "kta"), vec!["datta"]);
        assert_eq!(derive("quDAY", "kta"), vec!["hita"]);
        assert_eq!(derive("qudAY", "GaY"), vec!["dAya"]);
        assert_eq!(derive("jYA", "ka"), vec!["jYa"]);
        assert_eq!(derive("prIY", "ka"), vec!["priya"]);
        assert_eq!(derive("kF", "ka"), vec!["kira"]);
        assert_eq!(derive("buDa", "ka"), vec!["buDa"]);
        assert_eq!(derive("kfSa", "ka"), vec!["kfSa"]);
        assert_eq!(derive("kzipa", "ka"), vec!["kzipa"]);
        assert_eq!(derive("kzipa", "vuY"), vec!["kzepaka"]);
        assert_eq!(derive("kzip", "knu"), vec!["kzipRu"]);
        assert_eq!(derive("liKa", "ka"), vec!["liKa"]);
        assert_eq!(derive("glE", "ka"), vec!["gla"]);
        assert_eq!(derive("zWA", "ka"), vec!["sTa"]);
        assert_eq!(derive("pA", "Sa"), vec!["piba"]);
        assert_eq!(derive("naS", "kvarap"), vec!["naSvara"]);
        assert_eq!(derive("gamx", "Satf"), vec!["gacCat"]);
        assert_eq!(derive("eDa", "SAnac"), vec!["eDamAna"]);
        assert_eq!(derive("Yizvapa", "kta"), vec!["supta"]);
        assert_eq!(derive("yaja", "naN"), vec!["yajYa"]);
        assert_eq!(derive("naS", "kvarap"), vec!["naSvara"]);
        assert_eq!(derive("YimidA", "Gurac"), vec!["medura"]);
        assert_eq!(derive("BAsf", "Gurac"), vec!["BAsura"]);
        assert_eq!(derive("BU", "kvasu"), vec!["baBUvas"]);
        assert_eq!(derive("qukfY", "Ramul"), vec!["kAram"]);
        assert_eq!(derive("BU", "Ramul"), vec!["BAvam"]);
        assert!(decline("qukfY", "Ramul", "pum", &[]).is_none());
    }
}
// all4 849 -- krdanta
// all4 853 -- krdanta
// all4 857 -- krdanta
// all4 861 -- krdanta
// all4 865 -- krdanta
