//! लिट् (3.2.115) as in the Siddhānta-Kaumudī.
//!
//! 6.1.8 अभ्यास; 7.4.59 ह्रस्वः; 7.4.60 हलादिः शेषः; 7.4.61 शर्पूर्वाः खयः;
//! 7.4.62 कुहोश्चुः; 7.2.116 अत उपधायाः; 6.4.98 गमहनजनखनघसां लोपः क्ङिति;
//! 6.4.120 अत एकहलमध्येऽनादेशादेर्लिटि; 6.4.77 उवङ् (ū before a vowel);
//! 6.1.15 वचिस्वपियजादीनां किति; 6.1.17 लिट्यभ्यासस्योभयेषाम्;
//! 7.2.115 अचो ञ्णिति (vṛddhi of i/u/ṛ); 6.4.77/82 iyṅ uvṅ;
//! 7.1.34 आत औ णलः; 6.4.64 आतो लोप इटि च; 6.1.45 आदेच उपदेशेऽशिति;
//! 6.1.64 धात्वादेः षः सः; 2.4.41 वेञो वयिः; 6.1.37 लिटि वयो यः;
//! 6.1.38 वश्चास्यान्यतरस्यां किति; 6.1.39 वेञः; 6.1.33 अभ्यस्तस्य च (ह्वे);
//! 6.1.46 न व्यो लिटि; 7.4.69 दीर्घ इणः किति; 7.4.70 अत आदेः; 7.4.71 तस्मान्नुड् द्विहलः;
//! 7.4.66 उरत्; 6.4.122 तृफलभजत्रपश्च; 6.4.78 अभ्यासस्यासवर्णे; 6.4.81 इणो यण्;
//! 2.4.53 ब्रुवो वचिः; 3.1.35–36 आम्;
//! 3.4.81 लिटस्तझयोरेशिरेच्; 3.4.82 णलतुसुस्थलथुसणल्वमाः; 7.1.91 णलुत्तमो वा.

use crate::engine::phonology::apply_guna_to_stem;

/// Kartari forms for one cell. `purusha` 1 = प्रथम (3rd), 3 = उत्तम (1st).
/// `None` → caller’s generic stem+ending path.
pub fn kartari(dhatu: &str, purusha: u8, vacana: u8, pada: &str) -> Option<Vec<String>> {
    let root = match prakriya_root(dhatu).as_str() {
        "RI" => "nI".to_string(),
        "brU" => "vac".to_string(),
        other => other.to_string(),
    };
    if takes_am(&root) {
        return Some(am_forms(&root, purusha, vacana, pada));
    }
    let list = all_angas(&root)?;
    let mut out = Vec::new();
    for a in &list {
        let forms = if pada == "A" {
            paradigm_atmane(a, purusha, vacana)
        } else {
            paradigm(a, purusha, vacana)
        };
        for f in forms {
            if !out.contains(&f) {
                out.push(f);
            }
        }
    }
    Some(out)
}

/// वे: 2.4.41 वयिः (उवाय, ऊयतुः / 6.1.38 ऊवतुः) and 6.1.39 आत्व (ववौ).
/// व्ये: 6.1.46 no आत्व → विव्याय. ह्वे: 6.1.33 → हु → जुहाव.
fn all_angas(root: &str) -> Option<Vec<Angas>> {
    match root {
        "ve" => Some(vec![
            Angas {
                strong: "uvAy".into(),
                weak: "Uy".into(),
                full: "uvay".into(),
                thal_anit: None,
            },
            Angas {
                strong: "uvAy".into(),
                weak: "Uv".into(),
                full: "uvay".into(),
                thal_anit: None,
            },
            a_final_angas("vA"),
        ]),
        "vye" => Some(vec![Angas {
            strong: "vivyAy".into(),
            weak: "vivy".into(),
            full: "vivyay".into(),
            thal_anit: None,
        }]),
        "hve" => {
            let mut a = i_u_f_angas("hu")?;
            a.thal_anit = Some("juhoTa".into());
            Some(vec![a])
        }
        _ => angas(root).map(|a| vec![a]),
    }
}

struct Angas {
    /// णल्: jagAm, papAt, baBUv, jaGAn
    strong: String,
    /// kit weak: jagm, pet, baBUv, jaGn
    weak: String,
    /// no lopa, no vṛddhi: jagam, papat, baBUv, jaGan
    full: String,
    thal_anit: Option<String>,
}

pub(crate) fn prakriya_root(dhatu: &str) -> String {
    let mut s: String = dhatu.trim_end_matches('~').into();
    if s.starts_with("qu") && s.len() > 3 {
        s = s[2..].to_string();
    }
    if s.starts_with("Yi") && s.len() > 3 {
        s = s[2..].to_string();
    }
    // ओकार इत् (ohAk → hAk, ovE → vE).
    if s.starts_with('o') && s.len() > 2 && s.chars().nth(1).is_some_and(is_cons) {
        s = s[1..].to_string();
    }
    // जक्षादि: जागृ / दरिद्रा keep the final vowel; it is not इत्.
    let skip_it = matches!(s.as_str(), "jAgf" | "daridrA");
    if !skip_it {
        for it in ["ir", "x", "Y", "R", "N", "k", "o", "A", "I", "U", "F", "e", "E", "i", "u", "f"] {
            if s.len() > it.len() && s.ends_with(it) {
                let rest = &s[..s.len() - it.len()];
                if rest.chars().any(|c| "aAiIuUfFeEoOxX".contains(c)) {
                    s = rest.to_string();
                    break;
                }
            }
        }
    }
    if s.ends_with('a') && s.len() >= 3 {
        let core: String = s.chars().take(s.chars().count() - 1).collect();
        if is_cac(&core)
            || is_a_plus_cons(&core)
            || is_cluster_cac(&core)
            || ic_adi(core.chars().next())
            || matches!(
                core.as_str(),
                "han" | "jan" | "Kan" | "Gas" | "yam" | "tap" | "vac" | "yaj" | "vap" | "vah"
                    | "vas" | "vad" | "svap" | "zvap" | "grah"
            )
        {
            return core;
        }
    }
    // पचष्-type: इत् ष after a (qupacaz → pac).
    if s.ends_with("az") && s.len() > 3 {
        let core: String = s.chars().take(s.chars().count() - 2).collect();
        if is_cac(&core) || is_cluster_cac(&core) {
            return core;
        }
    }
    s
}

fn is_cons(c: char) -> bool {
    !matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

fn is_cac(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    c.len() == 3 && is_cons(c[0]) && c[1] == 'a' && is_cons(c[2])
}

fn is_a_plus_cons(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    c.len() == 2 && c[0] == 'a' && is_cons(c[1])
}

fn ic_adi(c: Option<char>) -> bool {
    matches!(c, Some('i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'o' | 'E' | 'O'))
}

fn is_guru_root(root: &str) -> bool {
    let c: Vec<char> = root.chars().collect();
    for (i, &ch) in c.iter().enumerate() {
        if matches!(ch, 'A' | 'I' | 'U' | 'F' | 'e' | 'o' | 'E' | 'O') {
            return true;
        }
        if !is_cons(ch) && i + 2 < c.len() && is_cons(c[i + 1]) && is_cons(c[i + 2]) {
            return true;
        }
    }
    false
}

/// 3.1.35 कास्प्रत्ययादाममन्त्रे लिटि; 3.1.36 इजादेश्च गुरुमतोऽनृच्छः.
fn takes_am(root: &str) -> bool {
    if matches!(root, "kAs" | "As") {
        return true;
    }
    if matches!(root, "fcC" | "fC" | "f") {
        return false;
    }
    ic_adi(root.chars().next()) && is_guru_root(root)
}

fn join_am(am: &str, aux: &str) -> String {
    if aux.starts_with('c') || aux.starts_with('C') {
        let base: String = am.chars().take(am.chars().count().saturating_sub(1)).collect();
        return format!("{base}Y{aux}");
    }
    format!("{am}{aux}")
}

/// णिच्/सन्/यङ् लिट्: aṅga + आम् + कृ/अस्/भू. `anga` may end in a (भावय → भावयाम्).
pub(crate) fn am_forms(anga: &str, purusha: u8, vacana: u8, pada: &str) -> Vec<String> {
    let base = anga.strip_suffix('a').unwrap_or(anga);
    let am = format!("{base}Am");
    let mut out = Vec::new();
    let push_aux = |out: &mut Vec<String>, aux_dhatu: &str, p: &str| {
        if let Some(forms) = kartari(aux_dhatu, purusha, vacana, p) {
            for f in forms {
                let j = join_am(&am, &f);
                if !out.contains(&j) {
                    out.push(j);
                }
            }
        }
    };
    push_aux(&mut out, "qukfY", pada);
    push_aux(&mut out, "asa", "P");
    push_aux(&mut out, "BU", "P");
    if pada == "A" {
        push_aux(&mut out, "BU", "A");
    }
    out
}

fn is_sar(c: char) -> bool {
    matches!(c, 's' | 'S' | 'z')
}

fn is_khay(c: char) -> bool {
    matches!(c, 'k' | 'K' | 'c' | 'C' | 'w' | 'W' | 't' | 'T' | 'p' | 'P')
}

fn deaspirate(c: char) -> char {
    match c {
        'K' => 'k',
        'G' => 'g',
        'C' => 'c',
        'J' => 'j',
        'W' => 'w',
        'Q' => 'q',
        'T' => 't',
        'D' => 'd',
        'P' => 'p',
        'B' => 'b',
        _ => c,
    }
}

/// 7.4.62 कुहोश्चुः after deaspiration (ख → च, not छ).
fn palatalize_kuho(c: char) -> char {
    match c {
        'k' => 'c',
        'g' => 'j',
        'h' => 'j',
        _ => c,
    }
}

fn abhyasa(root: &str) -> String {
    let chars: Vec<char> = root.chars().collect();
    let c0 = if chars.len() >= 2 && is_sar(chars[0]) && is_khay(chars[1]) {
        chars[1]
    } else {
        chars[0]
    };
    let c = palatalize_kuho(deaspirate(c0));
    format!("{c}a")
}

fn vrddhi_upadha(root: &str) -> String {
    let mut out = String::new();
    let mut done = false;
    for ch in root.chars() {
        if ch == 'a' && !done {
            out.push('A');
            done = true;
        } else {
            out.push(ch);
        }
    }
    out
}

fn lopa_upadha(root: &str) -> String {
    root.chars().filter(|&c| c != 'a').collect()
}

fn e_grade_cac(root: &str) -> String {
    let c: Vec<char> = root.chars().collect();
    format!("{}e{}", c[0], c[2])
}

/// 6.1.45 आदेच उपदेशेऽशिति — e/o/ai/au → ā, except वे/व्ये/ह्वे (यजादि).
fn adech(root: &str) -> String {
    if matches!(root, "ve" | "vye" | "hve") {
        return root.to_string();
    }
    let mut c: Vec<char> = root.chars().collect();
    if let Some(last) = c.last_mut() {
        if matches!(*last, 'e' | 'o' | 'E' | 'O') {
            *last = 'A';
        }
    }
    c.into_iter().collect()
}

/// 8.3.59 आदेशप्रत्यययोः षत्व after iṇ; 8.4.41 ष्टुना ष्टुः.
fn satva_stutva(s: &str) -> String {
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

/// 6.1.64 धात्वादेः षः सः, and undo ष्टुत्व on the next consonant (ष्ठा → स्था, ष्णा → स्ना).
fn dhatvadeh_sas(root: &str) -> String {
    let mut c: Vec<char> = root.chars().collect();
    if c.first() != Some(&'z') {
        return root.to_string();
    }
    c[0] = 's';
    if let Some(second) = c.get_mut(1) {
        *second = match *second {
            'w' => 't',
            'W' => 'T',
            'q' => 'd',
            'Q' => 'D',
            'R' => 'n',
            'z' => 's',
            x => x,
        };
    }
    c.into_iter().collect()
}

/// 7.1.34 आत औ णलः; 6.4.64 आ-lopa in kit / before iṭ.
fn a_final_angas(root: &str) -> Angas {
    let abh = abhyasa(root);
    let strong = format!("{abh}{root}");
    let weak: String = strong.chars().take(strong.chars().count() - 1).collect();
    Angas {
        strong: strong.clone(),
        weak: weak.clone(),
        full: weak,
        thal_anit: Some(format!("{strong}Ta")),
    }
}

/// णल् surface: CaC gets अ; आ-anta gets औ (7.1.34).
fn nal_form(a: &Angas) -> String {
    if a.strong.ends_with('A') {
        let mut s = a.strong.clone();
        s.pop();
        s.push('O');
        s
    } else {
        format!("{}a", a.strong)
    }
}

/// 6.1.15: vaC / yaC → uC / iC (व्/य् + अ → उ/इ).
fn samprasarana(root: &str) -> Option<String> {
    let c: Vec<char> = root.chars().collect();
    if c.len() == 3 && c[1] == 'a' {
        if c[0] == 'v' {
            return Some(format!("u{}", c[2]));
        }
        if c[0] == 'y' {
            return Some(format!("i{}", c[2]));
        }
    }
    if root == "svap" || root == "zvap" {
        return Some("sup".into());
    }
    None
}

fn vacadi_angas(root: &str) -> Option<Angas> {
    if root == "svap" || root == "zvap" {
        return Some(Angas {
            strong: "suzvAp".into(),
            weak: "suzup".into(),
            full: "suzvap".into(),
            thal_anit: None,
        });
    }
    let samp = samprasarana(root)?;
    if !matches!(root, "vac" | "vap" | "vah" | "vad" | "vas" | "yaj") {
        return None;
    }
    let u_or_i = samp.chars().next()?;
    let strong = format!("{}{}", u_or_i, vrddhi_upadha(root));
    let mut weak_coda: String = samp.chars().skip(1).collect();
    if root == "vas" {
        weak_coda = "z".into();
    }
    let weak = match u_or_i {
        'u' => format!("U{}", weak_coda),
        'i' => format!("I{}", weak_coda),
        _ => return None,
    };
    let full = format!("{}{}", u_or_i, root);
    let thal_anit = (root == "vac").then(|| "uvakTa".into());
    Some(Angas {
        strong,
        weak,
        full,
        thal_anit,
    })
}

/// अभ्यास vowel: i/u stay; ṛ → a (7.4.66 उरत्).
fn abhyasa_ik(root: &str) -> String {
    let onset: String = root.chars().take_while(|&c| is_cons(c)).collect();
    if onset.is_empty() {
        return abhyasa(root);
    }
    let cons = first_abhyasa_cons(&onset);
    let v = match root.chars().find(|&c| !is_cons(c)) {
        Some('i') | Some('I') => 'i',
        Some('u') | Some('U') => 'u',
        Some('f') | Some('F') => 'a',
        _ => 'a',
    };
    format!("{cons}{v}")
}

/// इगुपध हल्-anta: णल् गुण (ददर्श), kit no गुण (ददृशतुः). Not 7.2.115 (aṅga is not अजन्त).
fn ig_upadha_halanta(root: &str) -> Option<Angas> {
    let c: Vec<char> = root.chars().collect();
    if c.len() < 2 || !c.last().is_some_and(|&ch| is_cons(ch)) {
        return None;
    }
    let vowels: Vec<(usize, char)> = c
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, ch)| !is_cons(*ch))
        .collect();
    if vowels.len() != 1 {
        return None;
    }
    let (vi, v) = vowels[0];
    if vi == 0 || !matches!(v, 'i' | 'u' | 'f') {
        return None;
    }
    let abh = abhyasa_ik(root);
    let strong = satva_stutva(&format!("{}{}", abh, apply_guna_to_stem(root)));
    let weak = satva_stutva(&format!("{abh}{root}"));
    Some(Angas {
        strong,
        weak: weak.clone(),
        full: weak,
        thal_anit: None,
    })
}

fn first_abhyasa_cons(onset: &str) -> char {
    let chars: Vec<char> = onset.chars().collect();
    let c0 = if chars.len() >= 2 && is_sar(chars[0]) && is_khay(chars[1]) {
        chars[1]
    } else {
        chars[0]
    };
    palatalize_kuho(deaspirate(c0))
}

fn split_onset_vowel(root: &str) -> Option<(String, char)> {
    let mut c: Vec<char> = root.chars().collect();
    let v = *c.last()?;
    if !matches!(v, 'i' | 'I' | 'u' | 'U' | 'f' | 'F') {
        return None;
    }
    c.pop();
    if c.is_empty() {
        return None;
    }
    Some((c.into_iter().collect(), v))
}

/// ī/i → āy / y; u → āv / uv; ṛ → ār / r. अभ्यास vowel is hrasva (7.4.59).
fn i_u_f_angas(root: &str) -> Option<Angas> {
    if root == "i" {
        // 7.4.69 दीर्घ इणः किति; 6.4.78 इयङ्; थल् वेट् इयेथ.
        return Some(Angas {
            strong: "iyAy".into(),
            weak: "Iy".into(),
            full: "iyay".into(),
            thal_anit: Some("iyeTa".into()),
        });
    }
    if root == "f" {
        return Some(Angas {
            strong: "Ar".into(),
            weak: "Ar".into(),
            full: "Ar".into(),
            thal_anit: None,
        });
    }
    // 6.4.122 तृफलभजत्रपश्च — तॄ kit is तेर, not तत्र.
    if root == "tF" || root == "tf" {
        return Some(Angas {
            strong: "tatAr".into(),
            weak: "ter".into(),
            full: "tatar".into(),
            thal_anit: None,
        });
    }
    if root == "ji" {
        return Some(Angas {
            strong: "jigAy".into(),
            weak: "jigy".into(),
            full: "jigay".into(),
            thal_anit: None,
        });
    }
    let (onset, v) = split_onset_vowel(root)?;
    match v {
        'I' | 'i' => {
            let abh = format!("{}i", first_abhyasa_cons(&onset));
            Some(Angas {
                strong: format!("{abh}{onset}Ay"),
                weak: format!("{abh}{onset}y"),
                full: format!("{abh}{onset}ay"),
                thal_anit: None,
            })
        }
        'u' | 'U' => {
            let abh = format!("{}u", first_abhyasa_cons(&onset));
            let strong = satva_stutva(&format!("{abh}{onset}Av"));
            let weak = satva_stutva(&format!("{abh}{onset}uv"));
            let full = satva_stutva(&format!("{abh}{onset}av"));
            Some(Angas {
                strong: strong.clone(),
                weak,
                full: full.clone(),
                thal_anit: Some(format!("{full}Ta")),
            })
        }
        'f' | 'F' => {
            let abh = format!("{}a", first_abhyasa_cons(&onset));
            Some(Angas {
                strong: format!("{abh}{onset}Ar"),
                weak: format!("{abh}{onset}r"),
                full: format!("{abh}{onset}ar"),
                thal_anit: None,
            })
        }
        _ => None,
    }
}

/// 7.4.70 अत आदेः; 7.4.71 तस्मान्नुड् द्विहलः.
fn a_initial_angas(root: &str) -> Option<Angas> {
    let c: Vec<char> = root.chars().collect();
    if c.len() == 2 && c[0] == 'a' && is_cons(c[1]) {
        let stem = format!("A{}", c[1]);
        return Some(Angas {
            strong: stem.clone(),
            weak: stem.clone(),
            full: stem,
            thal_anit: None,
        });
    }
    if c.len() == 3 && c[0] == 'a' && is_cons(c[1]) && is_cons(c[2]) {
        let c1 = if c[1] == 'n' && matches!(c[2], 'c' | 'C' | 'j' | 'J') {
            'Y'
        } else {
            c[1]
        };
        let stem = format!("An{}{}", c1, c[2]);
        return Some(Angas {
            strong: stem.clone(),
            weak: stem.clone(),
            full: stem,
            thal_anit: None,
        });
    }
    None
}

fn is_cluster_cac(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    c.len() >= 4
        && is_cons(c[c.len() - 1])
        && c[c.len() - 2] == 'a'
        && c[..c.len() - 2].iter().all(|&x| is_cons(x))
}

fn angas(root: &str) -> Option<Angas> {
    let root = adech(root);
    let root = dhatvadeh_sas(&root);
    let root = if root.ends_with('a') && root.len() >= 3 {
        let core = &root[..root.len() - 1];
        if core.chars().last().is_some_and(is_cons) && core.chars().any(|c| !is_cons(c)) {
            core.to_string()
        } else {
            root
        }
    } else {
        root
    };
    if root == "BU" {
        return Some(Angas {
            strong: "baBUv".into(),
            weak: "baBUv".into(),
            full: "baBUv".into(),
            thal_anit: None,
        });
    }
    if root == "jAgf" {
        return Some(Angas {
            strong: "jAgAr".into(),
            weak: "jAgar".into(),
            full: "jAgar".into(),
            thal_anit: None,
        });
    }
    if root == "daridrA" {
        return Some(Angas {
            strong: "daridrA".into(),
            weak: "daridr".into(),
            full: "daridr".into(),
            thal_anit: Some("daridrATa".into()),
        });
    }
    if root == "cakAs" {
        return Some(Angas {
            strong: "cakAs".into(),
            weak: "cakAs".into(),
            full: "cakAs".into(),
            thal_anit: Some("cakAsTa".into()),
        });
    }
    if let Some(a) = vacadi_angas(&root) {
        return Some(a);
    }
    if root == "grah" {
        return Some(Angas {
            strong: "jagrAh".into(),
            weak: "jagfh".into(),
            full: "jagrah".into(),
            thal_anit: None,
        });
    }
    if let Some(a) = i_u_f_angas(&root) {
        return Some(a);
    }
    if root.ends_with('A') && root.chars().count() >= 2 {
        return Some(a_final_angas(&root));
    }
    if let Some(a) = a_initial_angas(&root) {
        return Some(a);
    }
    match root.as_str() {
        "han" => {
            return Some(Angas {
                strong: "jaGAn".into(),
                weak: "jaGn".into(),
                full: "jaGan".into(),
                thal_anit: Some("jaGanTa".into()),
            });
        }
        "jan" => {
            return Some(Angas {
                strong: "jajAn".into(),
                weak: "jajY".into(),
                full: "jajan".into(),
                thal_anit: None,
            });
        }
        "Gas" => {
            return Some(Angas {
                strong: "jaGAs".into(),
                weak: "jakz".into(),
                full: "jaGas".into(),
                thal_anit: None,
            });
        }
        _ => {}
    }
    if let Some(a) = ig_upadha_halanta(&root) {
        return Some(a);
    }
    if is_cluster_cac(&root) {
        let abh = abhyasa(&root);
        let strong = format!("{}{}", abh, vrddhi_upadha(&root));
        let full = format!("{abh}{root}");
        return Some(Angas {
            strong,
            weak: full.clone(),
            full,
            thal_anit: None,
        });
    }
    // गाध् / नाथ्: CāC (7.4.59 ह्रस्वः in abhyāsa, 7.4.62 कुहोश्चुः).
    let c: Vec<char> = root.chars().collect();
    if c.len() == 3 && is_cons(c[0]) && c[1] == 'A' && is_cons(c[2]) {
        let abh = abhyasa(&root);
        let full = format!("{abh}{root}");
        return Some(Angas {
            strong: full.clone(),
            weak: full.clone(),
            full,
            thal_anit: Some(format!("{abh}{root}Ta")),
        });
    }
    if is_cac(&root) {
        let abh = abhyasa(&root);
        let strong = format!("{}{}", abh, vrddhi_upadha(&root));
        let full = format!("{abh}{root}");
        if matches!(root.as_str(), "gam" | "Kan") {
            let weak = format!("{}{}", abh, lopa_upadha(&root));
            let thal_anit = (root == "gam").then(|| "jaganTa".into());
            return Some(Angas {
                strong,
                weak,
                full,
                thal_anit,
            });
        }
        let thal = match root.as_str() {
            "pac" => Some("papakTa".into()),
            "tyaj" => Some("tatyakTa".into()),
            _ => Some(format!("{full}Ta")),
        };
        return Some(Angas {
            strong,
            weak: e_grade_cac(&root),
            full,
            thal_anit: thal,
        });
    }
    // स्पर्ध् etc.: अभ्यास + root when no other aṅga rule fired.
    if root.chars().any(is_cons) {
        let abh = abhyasa(&root);
        let full = format!("{abh}{root}");
        return Some(Angas {
            strong: full.clone(),
            weak: full.clone(),
            full,
            thal_anit: None,
        });
    }
    None
}

fn paradigm(a: &Angas, purusha: u8, vacana: u8) -> Vec<String> {
    let nal = nal_form(a);
    match (purusha, vacana) {
        (1, 1) => vec![nal],
        (1, 2) => vec![format!("{}atuH", a.weak)],
        (1, 3) => vec![format!("{}uH", a.weak)],
        (2, 1) => {
            let mut v = vec![format!("{}iTa", a.full)];
            if let Some(t) = &a.thal_anit {
                v.push(t.clone());
            }
            v
        }
        (2, 2) => vec![format!("{}aTuH", a.weak)],
        (2, 3) => vec![format!("{}a", a.weak)],
        (3, 1) => {
            let alt = format!("{}a", a.full);
            if alt == nal {
                vec![nal]
            } else {
                vec![nal, alt]
            }
        }
        (3, 2) => vec![format!("{}iva", a.weak)],
        (3, 3) => vec![format!("{}ima", a.weak)],
        _ => vec![],
    }
}

/// Ātmanepada: all kit, so the weak aṅga + एश्/आते/इरेच् … (3.4.81).
fn paradigm_atmane(a: &Angas, purusha: u8, vacana: u8) -> Vec<String> {
    let w = &a.weak;
    match (purusha, vacana) {
        (1, 1) => vec![format!("{w}e")],
        (1, 2) => vec![format!("{w}Ate")],
        (1, 3) => vec![format!("{w}ire")],
        (2, 1) => vec![format!("{w}ize")],
        (2, 2) => vec![format!("{w}ATe")],
        (2, 3) => vec![format!("{w}iDve")],
        (3, 1) => vec![format!("{w}e")],
        (3, 2) => vec![format!("{w}ivahe")],
        (3, 3) => vec![format!("{w}imahe")],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gam_lit_prathama_ekavacana_is_jagama() {
        let f = kartari("gamx", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "jagAma"), "{:?}", f);
    }

    #[test]
    fn gam_weak_and_thal() {
        assert_eq!(kartari("gamx", 1, 3, "P").unwrap(), vec!["jagmuH"]);
        let t2 = kartari("gamx", 2, 1, "P").unwrap();
        assert!(t2.iter().any(|x| x == "jagamiTa"), "{:?}", t2);
        assert!(t2.iter().any(|x| x == "jaganTa"), "{:?}", t2);
        let u1 = kartari("gamx", 3, 1, "P").unwrap();
        assert!(u1.iter().any(|x| x == "jagAma") && u1.iter().any(|x| x == "jagama"), "{:?}", u1);
    }

    #[test]
    fn bu_lit_babhuva() {
        let f = kartari("BU", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "baBUva"), "{:?}", f);
        assert_eq!(kartari("BU", 1, 2, "P").unwrap(), vec!["baBUvatuH"]);
    }

    #[test]
    fn han_lit_jaghana() {
        let f = kartari("hana", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "jaGAna"), "{:?}", f);
        assert_eq!(kartari("hana", 1, 2, "P").unwrap(), vec!["jaGnatuH"]);
    }

    #[test]
    fn pat_lit_e_grade() {
        assert!(kartari("patx", 1, 1, "P").unwrap().iter().any(|x| x == "papAta"));
        assert_eq!(kartari("patx", 1, 2, "P").unwrap(), vec!["petatuH"]);
    }

    #[test]
    fn khan_lit() {
        let f = kartari("Kanu", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "caKAna"), "{:?}", f);
        assert_eq!(kartari("Kanu", 1, 2, "P").unwrap(), vec!["caKnatuH"]);
    }

    #[test]
    fn abhyasa_kuho() {
        assert_eq!(abhyasa("gam"), "ja");
        assert_eq!(abhyasa("Kan"), "ca");
        assert_eq!(abhyasa("pat"), "pa");
        assert_eq!(abhyasa("kram"), "ca");
        assert_eq!(abhyasa("gAD"), "ja");
        assert_eq!(abhyasa("sparD"), "pa");
    }

    #[test]
    fn gadh_nath_spardh_atmane() {
        let f = kartari("gADf", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "jagADe"), "{:?}", f);
        let f = kartari("nATf", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "nanATe"), "{:?}", f);
        let f = kartari("sparDa", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "pasparDe"), "{:?}", f);
    }

    #[test]
    fn vac_lit_uvaca() {
        let f = kartari("vaca", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "uvAca"), "{:?}", f);
        assert_eq!(kartari("vaca", 1, 2, "P").unwrap(), vec!["UcatuH"]);
        assert_eq!(kartari("vaca", 1, 3, "P").unwrap(), vec!["UcuH"]);
    }

    #[test]
    fn yaj_lit_iyaja() {
        assert!(kartari("yaja", 1, 1, "P").unwrap().iter().any(|x| x == "iyAja"));
        assert_eq!(kartari("yaja", 1, 2, "P").unwrap(), vec!["IjatuH"]);
    }

    #[test]
    fn vap_vas_vad_svap() {
        assert!(kartari("quvapa", 1, 1, "P").unwrap().iter().any(|x| x == "uvApa"));
        assert_eq!(kartari("quvapa", 1, 2, "P").unwrap(), vec!["UpatuH"]);
        assert_eq!(kartari("vasa", 1, 2, "P").unwrap(), vec!["UzatuH"]);
        assert_eq!(kartari("vada", 1, 2, "P").unwrap(), vec!["UdatuH"]);
        assert!(kartari("Yizvapa", 1, 1, "P").unwrap().iter().any(|x| x == "suzvApa"));
        assert_eq!(kartari("Yizvapa", 1, 2, "P").unwrap(), vec!["suzupatuH"]);
    }

    #[test]
    fn grah_lit() {
        assert!(kartari("graha", 1, 1, "P").unwrap().iter().any(|x| x == "jagrAha"));
        assert_eq!(kartari("graha", 1, 2, "P").unwrap(), vec!["jagfhatuH"]);
    }

    #[test]
    fn ni_kf_sru_lit() {
        assert!(kartari("RIY", 1, 1, "P").unwrap().iter().any(|x| x == "ninAya"));
        assert_eq!(kartari("RIY", 1, 2, "P").unwrap(), vec!["ninyatuH"]);
        assert!(kartari("qukfY", 1, 1, "P").unwrap().iter().any(|x| x == "cakAra"));
        assert_eq!(kartari("qukfY", 1, 2, "P").unwrap(), vec!["cakratuH"]);
        assert!(kartari("Sru", 1, 1, "P").unwrap().iter().any(|x| x == "SuSrAva"));
        assert_eq!(kartari("Sru", 1, 2, "P").unwrap(), vec!["SuSruvatuH"]);
        assert!(kartari("YiBI", 1, 1, "P").unwrap().iter().any(|x| x == "biBAya"));
    }

    #[test]
    fn atmanepada_lit() {
        assert_eq!(kartari("yaja", 1, 1, "A").unwrap(), vec!["Ije"]);
        assert_eq!(kartari("yaja", 1, 3, "A").unwrap(), vec!["Ijire"]);
        assert_eq!(kartari("qukfY", 1, 1, "A").unwrap(), vec!["cakre"]);
        assert_eq!(kartari("RIY", 1, 1, "A").unwrap(), vec!["ninye"]);
        assert_eq!(kartari("graha", 1, 1, "A").unwrap(), vec!["jagfhe"]);
        assert_eq!(kartari("BU", 1, 1, "A").unwrap(), vec!["baBUve"]);
        assert_eq!(kartari("gamx", 1, 1, "A").unwrap(), vec!["jagme"]);
    }

    #[test]
    fn da_dha_stha_lit() {
        assert_eq!(kartari("qudAY", 1, 1, "P").unwrap(), vec!["dadO"]);
        assert_eq!(kartari("qudAY", 1, 2, "P").unwrap(), vec!["dadatuH"]);
        assert_eq!(kartari("qudAY", 1, 3, "P").unwrap(), vec!["daduH"]);
        let u1 = kartari("qudAY", 3, 1, "P").unwrap();
        assert!(u1.iter().any(|x| x == "dadO") && u1.iter().any(|x| x == "dada"), "{:?}", u1);
        let t2 = kartari("qudAY", 2, 1, "P").unwrap();
        assert!(t2.iter().any(|x| x == "dadiTa") && t2.iter().any(|x| x == "dadATa"), "{:?}", t2);
        assert_eq!(kartari("qudAY", 1, 1, "A").unwrap(), vec!["dade"]);
        assert_eq!(kartari("quDAY", 1, 1, "P").unwrap(), vec!["daDO"]);
        assert_eq!(kartari("quDAY", 1, 2, "P").unwrap(), vec!["daDatuH"]);
        assert_eq!(kartari("quDAY", 1, 1, "A").unwrap(), vec!["daDe"]);
        assert_eq!(kartari("zWA", 1, 1, "P").unwrap(), vec!["tasTO"]);
        assert_eq!(kartari("zWA", 1, 2, "P").unwrap(), vec!["tasTatuH"]);
        assert_eq!(kartari("pA", 1, 1, "P").unwrap(), vec!["papO"]);
        assert_eq!(kartari("gA", 1, 1, "P").unwrap(), vec!["jagO"]);
        assert_eq!(kartari("ohAk", 1, 1, "P").unwrap(), vec!["jahO"]);
        assert_eq!(kartari("glE", 1, 1, "P").unwrap(), vec!["jaglO"]);
        assert_eq!(kartari("mAN", 1, 1, "A").unwrap(), vec!["mame"]);
    }

    #[test]
    fn ve_vye_hve_lit() {
        let f = kartari("veY", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "uvAya"), "{:?}", f);
        assert!(f.iter().any(|x| x == "vavO"), "{:?}", f);
        let f = kartari("veY", 1, 2, "P").unwrap();
        assert!(f.iter().any(|x| x == "UyatuH"), "{:?}", f);
        assert!(f.iter().any(|x| x == "UvatuH"), "{:?}", f);
        assert!(f.iter().any(|x| x == "vavatuH"), "{:?}", f);
        let f = kartari("veY", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "Uye"), "{:?}", f);
        assert!(f.iter().any(|x| x == "Uve"), "{:?}", f);
        assert!(f.iter().any(|x| x == "vave"), "{:?}", f);
        assert_eq!(kartari("vyeY", 1, 1, "P").unwrap(), vec!["vivyAya"]);
        assert_eq!(kartari("vyeY", 1, 2, "P").unwrap(), vec!["vivyatuH"]);
        assert_eq!(kartari("vyeY", 1, 1, "A").unwrap(), vec!["vivye"]);
        assert_eq!(kartari("hveY", 1, 1, "P").unwrap(), vec!["juhAva"]);
        assert_eq!(kartari("hveY", 1, 2, "P").unwrap(), vec!["juhuvatuH"]);
        assert_eq!(kartari("hveY", 1, 1, "A").unwrap(), vec!["juhuve"]);
        let t2 = kartari("hveY", 2, 1, "P").unwrap();
        assert!(t2.iter().any(|x| x == "juhaviTa") && t2.iter().any(|x| x == "juhoTa"), "{:?}", t2);
    }

    #[test]
    fn i_as_lit() {
        assert_eq!(kartari("iR", 1, 1, "P").unwrap(), vec!["iyAya"]);
        assert_eq!(kartari("iR", 1, 2, "P").unwrap(), vec!["IyatuH"]);
        assert_eq!(kartari("iR", 1, 3, "P").unwrap(), vec!["IyuH"]);
        let t2 = kartari("iR", 2, 1, "P").unwrap();
        assert!(t2.iter().any(|x| x == "iyayiTa") && t2.iter().any(|x| x == "iyeTa"), "{:?}", t2);
        let u1 = kartari("iR", 3, 1, "P").unwrap();
        assert!(u1.iter().any(|x| x == "iyAya") && u1.iter().any(|x| x == "iyaya"), "{:?}", u1);
        assert_eq!(kartari("iN", 1, 1, "A").unwrap(), vec!["Iye"]);
        assert_eq!(kartari("asa", 1, 1, "P").unwrap(), vec!["Asa"]);
        assert_eq!(kartari("asa", 1, 2, "P").unwrap(), vec!["AsatuH"]);
        assert_eq!(kartari("asa", 1, 3, "P").unwrap(), vec!["AsuH"]);
        assert_eq!(kartari("ada", 1, 1, "P").unwrap(), vec!["Ada"]);
        assert_eq!(kartari("awa", 1, 1, "P").unwrap(), vec!["Awa"]);
        assert_eq!(kartari("f", 1, 1, "P").unwrap(), vec!["Ara"]);
    }

    #[test]
    fn am_bru_jagf_nut() {
        let f = kartari("eDa", 1, 1, "A").unwrap();
        assert!(f.iter().any(|x| x == "eDAYcakre"), "{:?}", f);
        assert!(f.iter().any(|x| x == "eDAmAsa"), "{:?}", f);
        let f = kartari("brUY", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "uvAca"), "{:?}", f);
        assert_eq!(kartari("jAgf", 1, 1, "P").unwrap(), vec!["jAgAra"]);
        assert_eq!(kartari("jAgf", 1, 2, "P").unwrap(), vec!["jAgaratuH"]);
        assert_eq!(kartari("anjU", 1, 1, "P").unwrap(), vec!["AnYja"]);
        assert!(kartari("tyaja", 1, 1, "P").unwrap().iter().any(|x| x == "tatyAja"));
        assert_eq!(kartari("daridrA", 1, 1, "P").unwrap(), vec!["daridrO"]);
    }

    #[test]
    fn ig_upadha_and_tr_lit() {
        assert!(kartari("dfSir", 1, 1, "P").unwrap().iter().any(|x| x == "dadarSa"), "{:?}", kartari("dfSir", 1, 1, "P"));
        assert_eq!(kartari("dfSir", 1, 2, "P").unwrap(), vec!["dadfSatuH"]);
        assert_eq!(kartari("dfSir", 1, 3, "P").unwrap(), vec!["dadfSuH"]);
        assert_eq!(kartari("dfSir", 1, 1, "A").unwrap(), vec!["dadfSe"]);
        assert!(kartari("kfza", 1, 1, "P").unwrap().iter().any(|x| x == "cakarza"), "{:?}", kartari("kfza", 1, 1, "P"));
        assert_eq!(kartari("kfza", 1, 2, "P").unwrap(), vec!["cakfzatuH"]);
        assert!(kartari("vida", 1, 1, "P").unwrap().iter().any(|x| x == "viveda"), "{:?}", kartari("vida", 1, 1, "P"));
        assert_eq!(kartari("vida", 1, 2, "P").unwrap(), vec!["vividatuH"]);
        assert!(kartari("buDa", 1, 1, "P").unwrap().iter().any(|x| x == "buboDa"), "{:?}", kartari("buDa", 1, 1, "P"));
        assert_eq!(kartari("buDa", 1, 2, "P").unwrap(), vec!["bubuDatuH"]);
        assert!(kartari("diSa", 1, 1, "P").unwrap().iter().any(|x| x == "dideSa"), "{:?}", kartari("diSa", 1, 1, "P"));
        assert!(kartari("tF", 1, 1, "P").unwrap().iter().any(|x| x == "tatAra"), "{:?}", kartari("tF", 1, 1, "P"));
        assert_eq!(kartari("tF", 1, 2, "P").unwrap(), vec!["teratuH"]);
        assert!(kartari("Pala", 1, 1, "P").unwrap().iter().any(|x| x == "paPAla"), "{:?}", kartari("Pala", 1, 1, "P"));
        assert_eq!(kartari("Pala", 1, 2, "P").unwrap(), vec!["PelatuH"]);
        assert!(kartari("Baja", 1, 1, "P").unwrap().iter().any(|x| x == "baBAja"), "{:?}", kartari("Baja", 1, 1, "P"));
        assert_eq!(kartari("Baja", 1, 2, "P").unwrap(), vec!["BejatuH"]);
    }

    #[test]
    fn stu_cakas_thal() {
        let f = kartari("zwuY", 1, 1, "P").unwrap();
        assert!(f.iter().any(|x| x == "tuzwAva"), "{:?}", f);
        assert_eq!(kartari("cakAs", 1, 1, "P").unwrap(), vec!["cakAsa"]);
        let t = kartari("qupacaz", 2, 1, "P").unwrap();
        assert!(t.iter().any(|x| x == "papaciTa"), "{:?}", t);
        assert!(t.iter().any(|x| x == "papakTa"), "{:?}", t);
    }
}
