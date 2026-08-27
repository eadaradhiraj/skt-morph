//! लिट् (3.2.115) as in the Siddhānta-Kaumudī.
//!
//! 6.1.8 अभ्यास; 7.4.59 ह्रस्वः; 7.4.60 हलादिः शेषः; 7.4.61 शर्पूर्वाः खयः;
//! 7.4.62 कुहोश्चुः; 7.2.116 अत उपधायाः; 6.4.98 गमहनजनखनघसां लोपः क्ङिति;
//! 6.4.120 अत एकहलमध्येऽनादेशादेर्लिटि; 6.4.77 उवङ् (ū before a vowel);
//! 6.1.15 वचिस्वपियजादीनां किति; 6.1.17 लिट्यभ्यासस्योभयेषाम्;
//! 7.2.115 अचो ञ्णिति (vṛddhi of i/u/ṛ); 6.4.77/82 iyṅ uvṅ;
//! 3.4.81 लिटस्तझयोरेशिरेच्; 3.4.82 णलतुसुस्थलथुसणल्वमाः; 7.1.91 णलुत्तमो वा.

/// Kartari forms for one cell. `purusha` 1 = प्रथम (3rd), 3 = उत्तम (1st).
/// `None` → caller’s generic stem+ending path.
pub fn kartari(dhatu: &str, purusha: u8, vacana: u8, pada: &str) -> Option<Vec<String>> {
    let root = match prakriya_root(dhatu).as_str() {
        "RI" => "nI".to_string(),
        other => other.to_string(),
    };
    let a = angas(&root)?;
    if pada == "A" {
        Some(paradigm_atmane(&a, purusha, vacana))
    } else {
        Some(paradigm(&a, purusha, vacana))
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

fn prakriya_root(dhatu: &str) -> String {
    let mut s: String = dhatu.trim_end_matches('~').into();
    if s.starts_with("qu") && s.len() > 3 {
        s = s[2..].to_string();
    }
    if s.starts_with("Yi") && s.len() > 3 {
        s = s[2..].to_string();
    }
    for it in ["ir", "x", "Y", "R", "N", "o", "A", "I", "U", "F", "e", "E", "i", "u", "f"] {
        if s.len() > it.len() && s.ends_with(it) {
            let rest = &s[..s.len() - it.len()];
            if rest.chars().any(|c| "aAiIuUfFeEoOxX".contains(c)) {
                s = rest.to_string();
                break;
            }
        }
    }
    if s.ends_with('a') && s.len() >= 4 {
        let core: String = s.chars().take(s.chars().count() - 1).collect();
        if is_cac(&core)
            || matches!(
                core.as_str(),
                "han" | "jan" | "Kan" | "Gas" | "yam" | "tap" | "vac" | "yaj" | "vap" | "vah"
                    | "vas" | "vad" | "svap" | "zvap" | "grah"
            )
        {
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
            Some(Angas {
                strong: format!("{abh}{onset}Av"),
                weak: format!("{abh}{onset}uv"),
                full: format!("{abh}{onset}av"),
                thal_anit: None,
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

fn angas(root: &str) -> Option<Angas> {
    if root == "BU" {
        return Some(Angas {
            strong: "baBUv".into(),
            weak: "baBUv".into(),
            full: "baBUv".into(),
            thal_anit: None,
        });
    }
    if let Some(a) = vacadi_angas(root) {
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
    if let Some(a) = i_u_f_angas(root) {
        return Some(a);
    }
    match root {
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
    if !is_cac(root) {
        return None;
    }
    let abh = abhyasa(root);
    let strong = format!("{}{}", abh, vrddhi_upadha(root));
    let full = format!("{abh}{root}");
    if matches!(root, "gam" | "Kan") {
        let weak = format!("{}{}", abh, lopa_upadha(root));
        let thal_anit = (root == "gam").then(|| "jaganTa".into());
        return Some(Angas {
            strong,
            weak,
            full,
            thal_anit,
        });
    }
    Some(Angas {
        strong,
        weak: e_grade_cac(root),
        full,
        thal_anit: None,
    })
}

fn paradigm(a: &Angas, purusha: u8, vacana: u8) -> Vec<String> {
    let nal = format!("{}a", a.strong);
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
}
