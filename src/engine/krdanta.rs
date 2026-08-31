//! Port of sktmorph/engine/krdanta.py
use crate::engine::phonology::apply_guna_to_stem;
use serde::{Deserialize, Serialize};
use crate::engine::join::internal_sandhi;

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
        "gsnu" => Some(("zRu", vec!["3.2.94"], "root")),
        "kvasu" => Some(("vas", vec!["3.2.94"], "lit")),
        "lyap" => Some(("ya", vec!["3.2.187"], "lyap")),
        "ukaY" => Some(("uka", vec!["3.2.74"], "guna")),
        "a" => Some(("", vec!["3.3.56"], "guna_a")),
        "kyap" => Some(("", vec!["3.3.56"], "guna_a")),
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
    let mut r = surface_root(dhatu);
    if r.ends_with('a') && r.len() >= 3 {
        let core = &r[..r.len() - 1];
        if core.chars().last().is_some_and(|c| !"aAiIuUfFeEoOxX".contains(c))
            && core.chars().any(|c| "aAiIuUfFeEoOxX".contains(c))
        {
            r = core.to_string();
        }
    }
    // 6.1.15 वचिस्वपियजादीनां
    let r = match r.as_str() {
        "vac" => "uc".into(),
        "yaj" => "ij".into(),
        "vap" => "up".into(),
        "vah" => "uh".into(),
        "svap" | "zvap" => "sup".into(),
        other => other.to_string(),
    };
    // SLP1 भ is B; older "labh" = लभ्
    let r = if r.ends_with("bh") {
        format!("{}B", &r[..r.len() - 2])
    } else {
        r
    };
    let r = kit_anga(&r);
    match r.as_str() {
        "grah" => "gfhIta".into(),
        "vas" => "uzita".into(),
        "pat" => "patita".into(),
        "bandh" => "badDa".into(),
        // 8.2.36 व्रश्चभ्रस्जसृजमृजयजराजभ्राजच्छशां षः
        "sfj" | "mfj" | "Brasj" | "vraSc" => {
            let mut s = r.clone();
            s.pop();
            format!("{s}zwa")
        }
        "ij" => "izwa".into(),
        _ if r.ends_with('h')
            && r.chars().rev().nth(1).is_some_and(|c| "aAiIuUfFeEoO".contains(c)) =>
        {
            kta_ho_dha(&r)
        }
        _ if r.chars().last().is_some_and(|c| "iIuUfF".contains(c)) => format!("{r}ta"),
        _ => internal_sandhi(&r, "ta"),
    }
}

/// 8.2.31 हो ढः; 8.2.32 दादेर्धातोर्घः; 6.3.111 ढ्रलोपे lengthen.
fn kta_ho_dha(root: &str) -> String {
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

/// 7.4.40 द्यतिस्यतिमास्थामित्ति किति; 7.4.42 दधातेर्हिः; 7.4.46 दो दद् घोः; 6.4.37 न्-lopa.
fn kit_anga(root: &str) -> String {
    match root {
        "dA" => "dad".into(),
        "DA" => "hi".into(),
        "sTA" => "sTi".into(),
        "mA" => "mi".into(),
        "pA" => "pI".into(),
        "gam" | "han" => root[..root.len() - 1].to_string(),
        other => other.to_string(),
    }
}

fn ktin_form(root: &str) -> String {
    internal_sandhi(&kit_anga(root), "ti")
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

/// 6.1.78 एचोऽयवायावः; 6.1.79 वान्तो यि प्रत्यये; 6.1.101 अकः सवर्णे दीर्घः.
fn join_eco(stem: &str, suffix: &str) -> String {
    let Some(s0) = suffix.chars().next() else {
        return stem.to_string();
    };
    let Some(last) = stem.chars().last() else {
        return suffix.to_string();
    };
    let body: String = stem.chars().take(stem.chars().count() - 1).collect();
    if s0 == 'y' {
        return match last {
            'o' => format!("{body}av{suffix}"),
            'O' => format!("{body}Av{suffix}"),
            _ => format!("{stem}{suffix}"),
        };
    }
    if !is_ac(s0) {
        return format!("{stem}{suffix}");
    }
    match last {
        'e' => format!("{body}ay{suffix}"),
        'o' => format!("{body}av{suffix}"),
        'E' => format!("{body}Ay{suffix}"),
        'O' => format!("{body}Av{suffix}"),
        'a' | 'A' if s0 == 'a' || s0 == 'A' => format!("{body}A{}", &suffix[s0.len_utf8()..]),
        _ => format!("{stem}{suffix}"),
    }
}

/// णित्/ञित् kṛt aṅga: 7.2.115 अचो ञ्णिति, 7.2.116 अत उपधायाः, 7.3.86 इगुपध गुण,
/// 7.3.33 आतो युक्, 7.3.32/54 हन् → घात्.
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
    let ta = kta_base(dhatu);
    if let Some(stripped) = ta.strip_suffix("ita") {
        format!("{stripped}itvA")
    } else if let Some(stripped) = ta.strip_suffix("ta") {
        format!("{stripped}tvA")
    } else {
        format!("{ta}tvA")
    }
}

fn lyap_base(dhatu: &str) -> String {
    let ta = kta_base(dhatu);
    if let Some(stripped) = ta.strip_suffix("ita") {
        format!("{stripped}ya")
    } else if let Some(stripped) = ta.strip_suffix("ta") {
        format!("{stripped}ya")
    } else {
        format!("{ta}ya")
    }
}

fn tum_base(dhatu: &str, guna: &str) -> String {
    match surface_root(dhatu).as_str() {
        "gam" => "gantum".into(),
        "kf" => "kartum".into(),
        "dA" => "dAtum".into(),
        "BU" => "Bavitum".into(),
        "nI" => "netum".into(),
        "vac" => "vaktum".into(),
        "han" => "hantum".into(),
        "sTA" => "sTAtum".into(),
        _ => {
            let last_c = guna.chars().last().unwrap_or('a');
            if guna.ends_with('a') || "iIuUfFeEoO".contains(last_c) {
                let base = if guna.ends_with('a') { &guna[..guna.len() - 1] } else { guna };
                format!("{base}itum")
            } else {
                internal_sandhi(guna, "tum")
            }
        }
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
        "lyuw" | "lyu" => &["nap"],
        "GaY" => &["pum"],
        _ => &["pum", "stri", "nap"],
    }
}

fn is_at_participle(pratyaya: &str) -> bool {
    matches!(pratyaya, "Satf" | "Satf~" | "ktavatu" | "ktavatu~")
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
        && matches!(
            pratyaya,
            "kta" | "SAnac" | "cAnaS" | "tavya" | "anIyar" | "Rvul" | "vun" | "ac" | "anIya"
                | "yat" | "Ryat"
        )
    {
        if let Some(base) = form.strip_suffix('a') {
            return Some(format!("{base}A"));
        }
    }
    Some(form.to_string())
}

fn satr_nap(stem: &str) -> Option<crate::declension::subanta::Declension> {
    let mut d = crate::declension::subanta::generate(stem, "pum")?;
    let Some(base) = stem.strip_suffix("at") else {
        d.linga = "nap".into();
        return Some(d);
    };
    let nom = vec![
        stem.to_string(),
        format!("{stem}I"),
        format!("{base}anti"),
    ];
    d.declension.insert("prathamA".into(), nom.clone());
    d.declension.insert("dvitIyA".into(), nom.clone());
    d.declension.insert("samboDana".into(), nom);
    d.linga = "nap".into();
    Some(d)
}

/// सुबन्त of a kṛdanta pratipadika. `None` for अव्यय or a लिङ्ग the kṛt does not take.
pub fn decline(
    dhatu_query: &str,
    pratyaya: &str,
    linga: &str,
    prefixes: &[String],
) -> Option<crate::declension::subanta::Declension> {
    if !lingas(pratyaya).iter().any(|&l| l == linga) {
        return None;
    }
    let res = generate_with_prefixes(dhatu_query, pratyaya, prefixes);
    let form = res.forms.first()?.as_str();
    let stem = pratipadika(form, pratyaya, linga)?;
    if is_at_participle(pratyaya) && linga == "nap" {
        return satr_nap(&stem);
    }
    crate::declension::subanta::generate(&stem, linga)
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
            let (st, _) = crate::engine::stems::derive_stem(&dhatu, gana, "lat", "shuddha", &tags, &ant, &aup);
            let base = st.unwrap_or_else(|| present_stem(&dhatu, gana));
            if pratyaya == "Satf" {
                if base.ends_with('a') {
                    format!("{}at", &base[..base.len() - 1])
                } else if base.ends_with('u') {
                    format!("{}vat", &base[..base.len() - 1])
                } else if base.ends_with('I') {
                    format!("{}at", &base[..base.len() - 1])
                } else {
                    format!("{}at", base)
                }
            } else if pratyaya == "Satf~" {
                if base.ends_with('a') {
                    format!("{}n", &base[..base.len() - 1])
                } else {
                    format!("{}ant", base)
                }
            } else if pratyaya == "SAnac" || pratyaya == "cAnaS" || pratyaya.contains("SAnac") || pratyaya.contains("cAnaS") {
                if base.ends_with('a') {
                    format!("{}mAna", &base[..base.len() - 1])
                } else if base.ends_with('u') {
                    format!("{}vAna", &base[..base.len() - 1])
                } else {
                    format!("{}mAna", base)
                }
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
                "GaY" | "Rvul" | "ukaY" | "Ryat" => nit_krt_form(&root, pratyaya),
                "yat" if root.ends_with('A') => format!("{}eya", &root[..root.len() - 1]),
                _ => join_eco(&guna, suffix),
            }
        }
        "guna_a" => join_eco(&guna, "a"),
        "guna_tum" => tum_base(&dhatu, &guna),
        "guna_tavya" => crate::engine::it::tavya_form(&root),
        "anIya" => crate::engine::it::anIya_form(&root),
        "root" if pratyaya == "ktvA" => ktva_base(&dhatu),
        "root" => format!("{}{}", dhatu, suffix),
        "lit" => format!("{}a{}{}", dhatu.chars().next().unwrap_or('a'), dhatu, suffix),
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
    let base = v["participles"]["krut"].get(pratyaya)?.as_array()?.get(0)?;
    let gold_m = base.get("m")?.as_str()?.to_string();
    let ours = derive(dhatu_id, pratyaya);
    Some((ours.get(0).cloned().unwrap_or_default(), gold_m))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bu_kta() {
        let f = derive("BU", "kta");
        assert!(f.iter().any(|x| x == "BUta"), "{:?}", f);
    }

    #[test]
    fn gam_kf_vac_da_kta() {
        assert_eq!(derive("gam", "kta"), vec!["gata"]);
        assert_eq!(derive("qukfY", "kta"), vec!["kfta"]);
        assert_eq!(derive("vaca", "kta"), vec!["ukta"]);
        assert_eq!(derive("qudAY", "kta"), vec!["datta"]);
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
    }

    #[test]
    fn krdanta_declension() {
        let d = decline("gam", "kta", "pum", &[]).expect("gataH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gataH"), "{:?}", pr);
        let d = decline("gam", "kta", "stri", &[]).expect("gatA");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gatA"), "{:?}", pr);
        let d = decline("BU", "Satf", "pum", &[]).expect("Bavan");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "BavAn"), "{:?}", pr);
        let d = decline("BU", "Satf", "stri", &[]).expect("BavantI");
        assert_eq!(d.stem, "BavantI");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "BavantI"), "{:?}", pr);
        let d = decline("BU", "Satf", "nap", &[]).expect("Bavat");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "Bavat"), "{:?}", pr);
        assert!(pr.iter().any(|x| x == "Bavanti"), "{:?}", pr);
        let d = decline("qukfY", "tfc", "pum", &[]).expect("kartA");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "kartA"), "{:?}", pr);
        let dv = d.declension.get("dvitIyA").unwrap();
        assert!(dv.iter().any(|x| x == "kartAram"), "{:?}", dv);
        assert!(decline("BU", "ktvA", "pum", &[]).is_none());
        let d = decline("gam", "ktavatu", "pum", &[]).expect("gatavAn");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gatavAn"), "{:?}", pr);
    }

    #[test]
    fn krdanta_lingas_by_pratyaya() {
        assert!(lingas("lyap").is_empty());
        assert!(lingas("ktvA").is_empty());
        assert!(lingas("tumun").is_empty());
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
        assert_eq!(derive("qudAY", "kta"), vec!["datta"]);
        assert_eq!(derive("zWA", "kta"), vec!["sTita"]);
    }
}
