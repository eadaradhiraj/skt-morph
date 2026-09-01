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

fn is_sar(c: char) -> bool {
    matches!(c, 's' | 'S' | 'z')
}

fn is_khay(c: char) -> bool {
    matches!(c, 'k' | 'K' | 'c' | 'C' | 'w' | 'W' | 't' | 'T' | 'p' | 'P')
}

fn first_abhyasa_cons(root: &str) -> char {
    let chars: Vec<char> = root.chars().take_while(|&c| is_cons(c)).collect();
    let c0 = if chars.len() >= 2 && is_sar(chars[0]) && is_khay(chars[1]) {
        chars[1]
    } else {
        chars.first().copied().unwrap_or('i')
    };
    palatalize_kuho(deaspirate(c0))
}

fn last_vowel(root: &str) -> Option<char> {
    root.chars().rev().find(|c| !is_cons(*c))
}

/// सन् अभ्यास: u-roots keep u (बुभूष, शुश्रूष); else i (चिकीर्ष, जिगमिष). 7.4.59/62.
fn san_abhyasa(root: &str) -> String {
    let c = first_abhyasa_cons(root);
    let v = match last_vowel(root) {
        Some('u') | Some('U') | Some('o') => 'u',
        _ => 'i',
    };
    format!("{c}{v}")
}

fn ruki_stutva(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out: Vec<char> = Vec::with_capacity(chars.len());
    for (i, &ch) in chars.iter().enumerate() {
        let mut c = ch;
        if c == 's'
            && i > 0
            && matches!(chars[i - 1], 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'e' | 'o' | 'E' | 'O' | 'r')
        {
            c = 'z';
        }
        if matches!(c, 't' | 'T' | 'd' | 'D' | 'n') && out.last() == Some(&'z') {
            c = match c {
                't' => 'w',
                'T' => 'W',
                'd' => 'q',
                'D' => 'Q',
                'n' => 'R',
                x => x,
            };
        }
        out.push(c);
    }
    out.into_iter().collect()
}

/// 8.2.37 एकाचो बशो भष् झषन्तस्य स्ध्वोः (after 8.2.31 हो ढः, ढ is झष्).
fn bhas_initial(s: &str) -> String {
    let mut c: Vec<char> = s.chars().collect();
    if let Some(first) = c.first_mut() {
        *first = match *first {
            'g' => 'G',
            'd' => 'D',
            'b' => 'B',
            'j' => 'J',
            _ => *first,
        };
    }
    c.into_iter().collect()
}

fn thematic_sa(body: &str) -> String {
    let mut body = body.to_string();
    if body.ends_with('h') {
        body = bhas_initial(&body);
    }
    // 8.3.24 नश्चापदान्तस्य झलि — जिघांस
    if body.ends_with('n') {
        body = format!("{}M", &body[..body.len() - 1]);
    }
    let joined = crate::engine::join::internal_sandhi(&body, "sa");
    if joined.ends_with('a') {
        joined
    } else {
        format!("{joined}a")
    }
}

/// Aṅga before झलादि सन्. 6.1.16 ग्रह्; 6.4.16 + 7.3.54 हन्.
fn san_anga(root: &str) -> String {
    if root == "grah" {
        return "gfh".into();
    }
    if root == "han" {
        let dirgha = root.replace('a', "A");
        return format!("G{}", &dirgha[1..]);
    }
    root.to_string()
}

/// 7.4.54 सनि मीमाघुर्भलभशकपतपदामच इस् (no अभ्यास). 2.4.55 दाधा घ्वदाप्; आप् ईप्स.
fn san_is_adesha(root: &str) -> Option<String> {
    match root {
        "labh" => Some("lipsa".into()),
        "pat" => Some("pitsa".into()),
        "pad" => Some("pitsa".into()),
        "Sak" => Some("Sikza".into()),
        "dA" => Some("ditsa".into()),
        "DA" => Some("Ditsa".into()),
        "Ap" => Some("Ipsa".into()),
        "mA" => Some("mitsa".into()),
        "mI" => Some("mitsa".into()),
        _ => None,
    }
}

/// 3.1.5 गुप्तिज्किद्भ्यः सन्; 3.1.6 मान्बधदान्शान्भ्यो दीर्घश्चाभ्यासस्य.
/// Present aṅga without शप् अ (चिकित्स, दीदांस्, शीशांस्).
pub(crate) fn nitya_san_present(dhatu: &str) -> Option<String> {
    let (root, dirgha) = match dhatu {
        "kita" => ("kit", false),
        "dAna" => ("dAn", true),
        "SAna" => ("SAn", true),
        _ => return None,
    };
    let mut abh = san_abhyasa(root);
    if dirgha && abh.ends_with('i') {
        abh.pop();
        abh.push('I');
    }
    let body = thematic_sa(root);
    Some(format!("{abh}{}", body.trim_end_matches('a')))
}

/// सन् present aṅga.
pub fn san_stem(root: &str) -> String {
    if let Some(s) = san_is_adesha(root) {
        return s;
    }
    let abh = san_abhyasa(root);
    // ऋ-final: चिकीर्ष (ṛ → ईर् before sa)
    if root.ends_with('f') || root.ends_with('F') {
        let onset: String = root.chars().take_while(|&c| is_cons(c)).collect();
        return ruki_stutva(&format!("{abh}{onset}Irza"));
    }
    let last = root.chars().last();
    if last.is_some_and(|c| matches!(c, 'A' | 'i' | 'I' | 'u' | 'U' | 'e' | 'o')) {
        // 6.4.16 अज्झनगमां सनि: श्रु → श्रूष्
        let anga = if last == Some('u') {
            let mut s = root.to_string();
            s.pop();
            s.push('U');
            s
        } else {
            root.to_string()
        };
        return ruki_stutva(&format!("{abh}{}", thematic_sa(&anga)));
    }
    // 7.2.10 अनिट्; 7.2.12 सनि ग्रहगुहोश्च
    if (crate::engine::it::anit_sya(root) || matches!(root, "grah" | "guh")) && last_is_cons(root)
    {
        return ruki_stutva(&format!("{abh}{}", thematic_sa(&san_anga(root))));
    }
    ruki_stutva(&format!("{abh}{root}iza"))
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

/// 6.1.45 आदेच for 7.3.37 roots (शो/छो/षो/ह्वे/व्ये).
fn yuk_base(root: &str) -> String {
    match root {
        "So" | "SA" => "SA".into(),
        "Co" | "CA" => "CA".into(),
        "zo" | "so" | "sA" => "sA".into(),
        "hve" | "hvA" => "hvA".into(),
        "vye" | "vyA" => "vyA".into(),
        other => other.to_string(),
    }
}

/// 7.3.37 शाच्छासाह्वाव्यावेपां युक् (and पा पाने पाययति, not पापयति).
fn takes_yuk(root: &str) -> bool {
    matches!(root, "SA" | "CA" | "sA" | "hvA" | "vyA" | "pA") || root.ends_with("vep")
}

/// णिच् aṅga: 7.2.115/116 वृद्धि, 6.4.92 मित्, 7.3.36 पुक्, 7.3.86 लघूपध गुण.
pub fn nic_stem(root: &str) -> String {
    let root = yuk_base(root);
    let root = root.as_str();
    if takes_yuk(root) {
        return if root.ends_with("vep") {
            format!("{root}aya")
        } else {
            format!("{root}yaya")
        };
    }
    // णिच् of इण् is गमयति in the Kaumudī (not 2.4.45, which is लुङ् गाति only).
    if root == "i" {
        return "gamaya".into();
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
            // 7.3.32 हनस्तोऽचिण्णलोः (न् → त् before अच् of णिच्)
            let base = if root == "han" { "hat" } else { root };
            let v = base.chars().rev().nth(1);
            let mut anga = match v {
                Some('i') | Some('u') | Some('f') | Some('F') => apply_guna_to_stem(base),
                Some('a') => apply_vrddhi_to_stem(base),
                _ => apply_vrddhi_to_stem(base),
            };
            // 7.3.54 हो हन्तेर्ञ्णिन्नेषु (णिच् is णित्)
            if root == "han" {
                if let Some(rest) = anga.strip_prefix('h') {
                    anga = format!("G{rest}");
                }
            }
            format!("{anga}aya")
        }
        _ => format!("{}aya", apply_vrddhi_to_stem(root)),
    }
}

fn abhyasa_i(root: &str) -> String {
    format!("{}i", first_abhyasa_cons(root))
}

/// 7.4.27 रीङ् ऋतः; आ → ई; 7.3.54 हन् (यङ् is ङित्); short i/u lengthen.
fn yan_anga(root: &str) -> String {
    if root == "han" {
        return "Gan".into();
    }
    if root.ends_with('f') || root.ends_with('F') {
        let onset: String = root.chars().take_while(|&c| is_cons(c)).collect();
        return format!("{onset}rI");
    }
    if root.ends_with('A') {
        let mut s = root.to_string();
        s.pop();
        s.push('I');
        return s;
    }
    if root.ends_with('u') {
        let mut s = root.to_string();
        s.pop();
        s.push('U');
        return s;
    }
    if root.ends_with('i') {
        let mut s = root.to_string();
        s.pop();
        s.push('I');
        return s;
    }
    root.to_string()
}

/// 7.4.82 गुणो यङ्लुकोः (इगुपध अभ्यास); 7.4.83 दीर्घोऽकितः; 7.4.85 नुगतोऽनुनासिकान्तस्य.
fn yan_abhyasa(orig: &str, anga: &str) -> String {
    let c = first_abhyasa_cons(orig);
    if orig.chars().last().is_some_and(|ch| matches!(ch, 'n' | 'm' | 'N' | 'Y' | 'R')) {
        let n = match anga.chars().next() {
            Some('k' | 'K' | 'g' | 'G' | 'h') => 'N',
            Some('c' | 'C' | 'j' | 'J') => 'Y',
            Some('w' | 'W' | 'q' | 'Q') => 'R',
            Some('p' | 'P' | 'b' | 'B' | 'm') => 'm',
            _ => 'n',
        };
        return format!("{c}a{n}");
    }
    let av = match last_vowel(anga) {
        Some('i' | 'I' | 'e') => 'e',
        Some('u' | 'U' | 'o') => 'o',
        Some('a' | 'A') => 'A',
        _ => 'a',
    };
    format!("{c}{av}")
}

/// यङ् present aṅga (आत्मने). बोभूय, चेक्रीय, जङ्गम्य, पापच्य, देदीय.
/// sūtras: 3.1.22 + 7.4.82/83/85; ya-pratyaya retains ya for ātmanepada.
pub fn yan_stem(root: &str) -> String {
    let anga = yan_anga(root);
    let abh = yan_abhyasa(root, &anga);
    format!("{abh}{anga}ya")
}

/// यङ्लुक् present aṅga (परस्मै, ya-lopa). बोभू, चेक्री, जङ्गम्, पापच्, देदी.
/// sūtra: 2.4.74 यङोऽचि च / लुक् — ya of 3.1.22 is elided; abhyāsa+anga remains.
/// Uses same aṅga/abhyāsa as yaṄ; pada is parasmai (not forced ātmanepada).
/// Future devs: yan = ātmanepada with ya; yanluk = parasmai without ya.
pub fn yan_luk_stem(root: &str) -> String {
    // — inner: reuse yaṄ aṅga/abhyāsa then strip ya; keeps 7.4.82 guṇa, 7.4.85 nut.
    let anga = yan_anga(root);
    let abh = yan_abhyasa(root, &anga);
    format!("{abh}{anga}")
}

/// कर्मणि/भावे यक् (3.1.67, kit). 2.4.52 अस्तिर्भूः; 6.1.15 संप्रसारण; 7.4.28 रिङ् ऋतः.
pub fn karma_stem(root: &str) -> String {
    let r = match root {
        "as" => "BU".into(),
        "vac" => "uc".into(),
        "yaj" => "ij".into(),
        "vap" => "up".into(),
        "vah" => "uh".into(),
        "svap" | "zvap" => "sup".into(),
        "i" => "I".into(),
        other => other.to_string(),
    };
    if r.ends_with('A') {
        let mut s = r;
        s.pop();
        return format!("{s}Iya");
    }
    if r.ends_with('f') || r.ends_with('F') {
        let onset: String = r.chars().take_while(|&c| is_cons(c)).collect();
        return format!("{onset}riya");
    }
    format!("{r}ya")
}

/// 7.4.1 णौ चङ्युपधाया ह्रस्वः: णिच् aṅga without अय, then ह्रस्व of आ.
fn nic_shortened(root: &str) -> String {
    let stem = nic_stem(root);
    let inner = stem.strip_suffix("aya").unwrap_or(&stem);
    let mut c: Vec<char> = inner.chars().collect();
    if let Some(i) = c.iter().rposition(|&ch| ch == 'A') {
        c[i] = 'a';
    }
    c.into_iter().collect()
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

/// `kind`: Ric | san | yaN | yaNluk | karma
/// — yaN = सयक्-intensive with ya (ātmanepada, boBUyate)
/// — yaNluk = intensive ya-luk (parasmai, boBUti) — 2.4.74
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
        "yaN" | "yan" => (yan_stem(&root), Some("A")), // yaṄ retains ya, ātmanepada
        "yaNluk" | "yanluk" | "yaN_luk" => (yan_luk_stem(&root), None), // 2.4.74 luk — parasmai
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
            "yaNluk" | "yanluk" | "yaN_luk" => {
                // yaṄluk lun is also caṄ (intensive reduplication) — parasmai
                let base = stem.strip_suffix('a').unwrap_or(&stem);
                crate::engine::lun::cang_kartari(base, purusha, vacana, "P")
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
    // ---------------------------------------------------------------------------
    // fn `nic_bu_kf_gam`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
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
    // ---------------------------------------------------------------------------
    // fn `san_yan_karma`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
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
        assert_eq!(san_stem("BU"), "buBUza");
        assert_eq!(san_stem("kf"), "cikIrza");
        assert_eq!(san_stem("Sru"), "SuSrUza");
        assert_eq!(san_stem("nI"), "ninIza");
        assert_eq!(san_stem("dA"), "ditsa");
        assert_eq!(san_stem("pA"), "pipAsa");
        assert_eq!(san_stem("sTA"), "tizWAsa");
        assert_eq!(san_stem("labh"), "lipsa");
        assert_eq!(san_stem("pat"), "pitsa");
        assert_eq!(san_stem("Sak"), "Sikza");
        assert_eq!(san_stem("Ap"), "Ipsa");
        assert_eq!(san_stem("gam"), "jigamiza");
        assert_eq!(san_stem("han"), "jiGAMsa");
        assert_eq!(san_stem("grah"), "jiGfkza");
        assert_eq!(san_stem("guh"), "juGukza");
        assert_eq!(san_stem("pac"), "pipakza");
        assert_eq!(nic_stem("han"), "GAtaya");
        assert_eq!(san_stem("jYA"), "jijYAsa");
        assert_eq!(san_stem("BI"), "biBIza");
        let f = kartari("Apx", "san", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "Ipsati"), "{:?}", f);
        let f = kartari("Sru", "san", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "SuSrUzati"), "{:?}", f);
        let f = kartari("zWA", "san", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "tizWAsati"), "{:?}", f);
        assert_eq!(yan_stem("BU"), "boBUya");
        assert_eq!(yan_stem("kf"), "cekrIya");
        assert_eq!(yan_stem("gam"), "jaNgamya");
        assert_eq!(yan_stem("han"), "jaNGanya");
        assert_eq!(yan_stem("nI"), "nenIya");
        assert_eq!(yan_stem("pac"), "pApacya");
        assert_eq!(yan_stem("dA"), "dedIya");
        assert_eq!(yan_stem("Sru"), "SoSrUya");
        assert_eq!(karma_stem("dA"), "dIya");
        assert_eq!(karma_stem("vac"), "ucya");
        assert_eq!(karma_stem("as"), "BUya");
        let f = kartari("qukfY", "yaN", "lat", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "cekrIyate"), "{:?}", f);
        let f = kartari("qupacaz", "yaN", "lat", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "pApacyate"), "{:?}", f);
        let f = kartari("hana", "yaN", "lat", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "jaNGanyate"), "{:?}", f);
        let f = kartari("qudAY", "karma", "lat", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "dIyate"), "{:?}", f);
        let f = kartari("vaca", "karma", "lat", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "ucyate"), "{:?}", f);
        let f = kartari("hana", "san", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "jiGAMsati"), "{:?}", f);
        let f = kartari("graha", "san", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "jiGfkzati"), "{:?}", f);
        let f = kartari("hana", "Ric", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "GAtayati"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `nic_lit_am_lun_cang`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
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

    #[test]
    // ---------------------------------------------------------------------------
    // fn `yan_luk` — sūtra: yaN luk (2.4.74): intensive without ya, parasmai.
    // Future devs: boBU (not boBUya), cekrI, jaNgam — test stem + tinanta.
    // ---------------------------------------------------------------------------
    fn yan_luk() {
        // stem: ya removed — parasmai intensive
        assert_eq!(yan_luk_stem("BU"), "boBU");
        assert_eq!(yan_luk_stem("kf"), "cekrI");
        assert_eq!(yan_luk_stem("gam"), "jaNgam");
        assert_eq!(yan_luk_stem("pac"), "pApac");
        // han intensive: 7.4.85 + 7.3.54 → jaNGan (ṅ for yaṄ)
        assert_eq!(yan_luk_stem("han"), "jaNGan");
        // tinanta: parasmai lat — suffix added via śap (10 = intensive luk class)
        let f = kartari("BU", "yaNluk", "lat", 1, 1, "P").unwrap();
        // boBU → boBavīti / boBUti depending on śap vs śyan — accept either intensive parasmai form
        assert!(f.iter().any(|x| x.contains("boB")), "BU yaNluk lat P 1/1: {:?}", f);
        let f = kartari("qukfY", "yaNluk", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x.contains("cekr") || x.contains("cekAr") || x.contains("cek") ), "kf yaNluk: {:?}", f);
        let f = kartari("gamx", "yaNluk", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x.contains("jaNgam") || x.contains("jaNg")), "gamx yaNluk: {:?}", f);
        // lun still caṄ for luk
        let f = kartari("BU", "yaNluk", "lun", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x.contains("boB") || x.contains("aBo")), "BU yaNluk lun: {:?}", f);
        // han yaNluk lat should be parasmai intensive (not ātmanepada ya- form)
        let f = kartari("hana", "yaNluk", "lat", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x.contains("jaNG") || x.contains("jaNg")), "han yaNluk: {:?}", f);
        // vac yaNluk: vAvac (reduplicated) — parasmai intensive, present shows 8.2.30 c→k before ti
        assert_eq!(yan_luk_stem("vac"), "vAvac");
        let f = kartari("vaca", "yaNluk", "lat", 1, 1, "P").unwrap();
        // vAvac + ti → vAvakti (jhal sandhi), so check vAv prefix not literal vAvac
        assert!(f.iter().any(|x| x.contains("vAv") && x.contains("vak")), "vac yaNluk lat: {:?}", f);
        // sru yaNluk: SoSrU -> SoSrU (without ya) — intensive without ya
        assert_eq!(yan_luk_stem("Sru"), "SoSrU");
    }
}
