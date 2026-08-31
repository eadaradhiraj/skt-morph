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
    match r.as_str() {
        "gam" => "gata".into(),
        "han" => "hata".into(),
        "vac" => "ukta".into(),
        "yaj" => "izwa".into(),
        "vap" => "upta".into(),
        "vas" => "uzita".into(),
        "dA" => "datta".into(),
        "DA" => "hita".into(),
        "sTA" => "sTita".into(),
        "pA" => "pIta".into(),
        "nI" | "i" => format!("{r}ta"),
        "kf" => "kfta".into(),
        "BU" => "BUta".into(),
        "grah" => "gfhIta".into(),
        "Sru" => "Sruta".into(),
        "pat" => "patita".into(),
        "dfS" => "dfzwa".into(),
        "naS" => "nazwa".into(),
        "vah" => "UQa".into(),
        "guh" => "gUQa".into(),
        "dah" => "dagDa".into(),
        "labh" => "labDa".into(),
        "bandh" => "badDa".into(),
        "svap" => "supta".into(),
        "sfj" => "sfzwa".into(),
        "kfz" => "kfzwa".into(),
        "jYA" => "jYAta".into(),
        "lih" => "lIQa".into(),
        "duh" => "dugDa".into(),
        _ if r.chars().last().is_some_and(|c| "iIuUfF".contains(c)) => format!("{r}ta"),
        _ => internal_sandhi(&r, "ta"),
    }
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

pub fn derive(dhatu_query: &str, pratyaya: &str) -> Vec<String> {
    let (dhatu, gana, tags, ant, aup) = load_dhatu(dhatu_query);
    let rule = pratyaya_rule(pratyaya);
    if rule.is_none() {
        return vec![];
    }
    let (suffix, _sutras, mode) = rule.unwrap();
    let guna = apply_guna_to_stem(&dhatu);

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
            let r = surface_root(&dhatu);
            match pratyaya {
                "lyuw" | "lyu" => crate::engine::it::lyuw_form(&r),
                "tfc" => crate::engine::it::tfc_form(&r),
                "ktin" => format!("{}ti", r),
                _ => format!("{}{}", guna, suffix),
            }
        }
        "guna_a" => format!("{}a", guna),
        "guna_tum" => tum_base(&dhatu, &guna),
        "guna_tavya" => crate::engine::it::tavya_form(&surface_root(&dhatu)),
        "anIya" => crate::engine::it::anIya_form(&surface_root(&dhatu)),
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
}
