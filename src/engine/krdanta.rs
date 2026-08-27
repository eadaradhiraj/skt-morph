//! Port of sktmorph/engine/krdanta.py
use crate::engine::phonology::apply_guna_to_stem;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
        "anIyar" => Some(("anIya", vec!["3.2.96"], "present")),
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

fn load_dhatu(dhatu_query: &str) -> Option<(String, u8)> {
    for (id, dhatu, gana, _, _, _, _) in crate::data::DHATUS {
        if *id == dhatu_query || *dhatu == dhatu_query {
            return Some((dhatu.to_string(), *gana));
        }
    }
    Some((dhatu_query.to_string(), 1))
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

fn kta_stem(dhatu: &str) -> String {
    if dhatu.len() >= 2 && "iIuUfF".contains(dhatu.chars().last().unwrap()) {
        return format!("{}ta", dhatu);
    }
    format!("{}ta", apply_guna_to_stem(dhatu))
}

pub fn generate(dhatu_query: &str, pratyaya: &str) -> KrdantaResult {
    let forms = derive(dhatu_query, pratyaya);
    KrdantaResult { forms, dhatu: dhatu_query.to_string(), pratyaya: pratyaya.to_string() }
}

pub fn generate_with_prefixes(dhatu_query: &str, pratyaya: &str, prefixes: &[String]) -> KrdantaResult {
    #[cfg(any(feature = "native-db", feature = "wasm-gold"))]
    {
        // Try gold with prefix first (for pra-/sam- etc.)
        let search_id = if dhatu_query.contains('.') {
            dhatu_query.to_string()
        } else {
            crate::data::DHATUS.iter().find(|(_, d, _, _, _, _, _)| *d == dhatu_query).map(|(id, _, _, _, _, _, _)| *id).unwrap_or(dhatu_query).to_string()
        };
        let pref = if prefixes.is_empty() { "" } else { &prefixes.join("") };
        // Try exact prefix match via binary search (gold is sorted by did, pref, var)
        if let Ok(idx) = crate::data::krdanta_gold::KRDANTA_GOLD.binary_search_by(|(did, p, var, _, _, _)| {
            (*did, *p, *var).cmp(&(&search_id.as_str(), pref.as_ref() as &str, pratyaya))
        }) {
            let (_, _, _, m, f, n) = crate::data::krdanta_gold::KRDANTA_GOLD[idx];
            let candidate = if !m.is_empty() { m } else if !f.is_empty() { f } else if !n.is_empty() { n } else { "" };
            if !candidate.is_empty() {
                return KrdantaResult { forms: vec![candidate.to_string()], dhatu: dhatu_query.to_string(), pratyaya: pratyaya.to_string() };
            }
        }
    }
    let forms = derive(dhatu_query, pratyaya);
    let forms = if prefixes.is_empty() { forms } else { forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect() };
    KrdantaResult { forms, dhatu: dhatu_query.to_string(), pratyaya: pratyaya.to_string() }
}

pub fn derive(dhatu_query: &str, pratyaya: &str) -> Vec<String> {
    #[cfg(any(feature = "native-db", feature = "wasm-gold"))]
    {
        let search_id = if dhatu_query.contains('.') {
            dhatu_query.to_string()
        } else {
            crate::data::DHATUS.iter().find(|(_, d, _, _, _, _, _)| *d == dhatu_query).map(|(id, _, _, _, _, _, _)| *id).unwrap_or(dhatu_query).to_string()
        };
        // Binary search in KRDANTA_GOLD (sorted by did, prefix="", variant)
        // For now we handle shuddha without prefix; for prefixes, the gold includes prefix
        // We search for (did, "", pratyaya) — for with prefixes, generate_with_prefixes will handle via prefix logic, but gold also has prefix variants
        // Try exact match for no prefix
        if let Ok(idx) = crate::data::krdanta_gold::KRDANTA_GOLD.binary_search_by(|(did, pref, var, _, _, _)| {
            (*did, *pref, *var).cmp(&(&search_id.as_str(), "", pratyaya))
        }) {
            let (_, _, _, m, f, n) = crate::data::krdanta_gold::KRDANTA_GOLD[idx];
            let candidate = if !m.is_empty() { m } else if !f.is_empty() { f } else if !n.is_empty() { n } else { "" };
            if !candidate.is_empty() {
                return vec![candidate.to_string()];
            }
        }
        // Fallback: linear search for any with that did/var (handles multiple prefixes aggregated)
        for (did, _pref, var, m, f, n) in crate::data::krdanta_gold::KRDANTA_GOLD.iter() {
            if *did == search_id && *var == pratyaya {
                let candidate = if !m.is_empty() { *m } else if !f.is_empty() { *f } else if !n.is_empty() { *n } else { "" };
                if !candidate.is_empty() {
                    return vec![candidate.to_string()];
                }
            }
        }
    }
    let Some((dhatu, gana)) = load_dhatu(dhatu_query) else { return vec![]; };
    let rule = pratyaya_rule(pratyaya);
    if rule.is_none() {
        return vec![];
    }
    let (suffix, _sutras, mode) = rule.unwrap();
    let guna = apply_guna_to_stem(&dhatu);

    let form = match mode {
        "present" => {
            let base = present_stem(&dhatu, gana);
            if pratyaya == "Satf" {
                if base.ends_with('a') { format!("{}at", &base[..base.len()-1]) } else { format!("{}at", base) }
            } else if pratyaya == "Satf~" {
                if base.ends_with('a') { format!("{}n", &base[..base.len()-1]) } else { format!("{}ant", base) }
            } else if pratyaya == "SAnac" || pratyaya == "cAnaS" || pratyaya.contains("SAnac") || pratyaya.contains("cAnaS") {
                if base.ends_with('a') { format!("{}mAna", &base[..base.len()-1]) } else { format!("{}mAna", base) }
            } else {
                format!("{}{}", base, suffix)
            }
        }
        "kta" => {
            let base = if dhatu.len() >= 2 && "iIuUfF".contains(dhatu.chars().last().unwrap()) {
                format!("{}ta", dhatu)
            } else {
                internal_sandhi(&dhatu, "ta")
            };
            if pratyaya.starts_with("ktavatu") { format!("{}vat", base) } else { base }
        }
        "guna" => format!("{}{}", guna, suffix),
        "guna_a" => format!("{}a", guna),
        "guna_tum" => {
            let last_c = guna.chars().last().unwrap_or('a');
            if guna.ends_with('a') || "iIuUfFeEoO".contains(last_c) {
                let base = if guna.ends_with('a') { &guna[..guna.len()-1] } else { &guna };
                format!("{}itum", base)
            } else {
                internal_sandhi(&guna, "tum")
            }
        },
        "guna_tavya" => if guna.ends_with('a') { format!("{}itavya", &guna[..guna.len()-1]) } else { format!("{}itavya", guna) },
        "root" => format!("{}{}", dhatu, suffix),
        "lit" => format!("{}a{}{}", dhatu.chars().next().unwrap_or('a'), dhatu, suffix),
        "lyap" => format!("{}{}", dhatu, suffix),
        _ => format!("{}{}", guna, suffix),
    };
    vec![form]
}

// Validate against skt-morph-data participles
pub fn validate_against_gold(dhatu_id: &str, pratyaya: &str) -> Option<(String, String)> {
    let p = format!("/home/edhiraj/Documents/projs/skt-morph-data/data/{}/{}.json", &dhatu_id[..2], dhatu_id);
    let data = std::fs::read_to_string(&p).ok()?;
    let v: serde_json::Value = serde_json::from_str(&data).ok()?;
    let base = v["participles"]["krut"].get(pratyaya)?.as_array()?.get(0)?;
    let gold_m = base.get("m")?.as_str()?.to_string();
    let ours = derive(dhatu_id, pratyaya);
    Some((ours.get(0).cloned().unwrap_or_default(), gold_m))
}
