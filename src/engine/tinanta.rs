use serde::{Deserialize, Serialize};
use crate::engine::lakara::{lakara_family, normalize_lakara};
use crate::engine::stems::{derive_stem, conjugation_gana};
use crate::engine::endings::family_endings;
use crate::engine::join::join_variants;
use crate::engine::steps::EngineStep;

#[derive(Serialize, Deserialize, Debug)]
pub struct TinantaResult {
    pub forms: Vec<String>,
    pub dhatu: String,
    pub lakara: String,
    pub purusha: u8,
    pub vacana: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParadigmEntry {
    pub purusha: u8,
    pub vacana: u8,
    pub forms: Vec<String>,
}

fn load_dhatu_info(dhatu_query: &str) -> Option<(String, u8, String, String, String, String)> {
    for (id, dhatu, gana, pada, tags, ant, aup) in crate::data::DHATUS {
        if *id == dhatu_query || *dhatu == dhatu_query {
            return Some((dhatu.to_string(), *gana, pada.to_string(), tags.to_string(), ant.to_string(), aup.to_string()));
        }
    }
    Some((dhatu_query.to_string(), 1, "P".to_string(), "".to_string(), "".to_string(), "".to_string()))
}

pub fn generate(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8) -> TinantaResult {
    let (forms, _) = generate_all(dhatu_query, lakara, purusha, vacana);
    let (canon, _) = normalize_lakara(lakara);
    TinantaResult { forms, dhatu: dhatu_query.to_string(), lakara: canon, purusha, vacana }
}

pub fn generate_with_prefixes(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &[String]) -> TinantaResult {
    let (mut forms, _) = generate_all(dhatu_query, lakara, purusha, vacana);
    if !prefixes.is_empty() {
        forms = forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
    }
    let (canon, _) = normalize_lakara(lakara);
    TinantaResult { forms, dhatu: dhatu_query.to_string(), lakara: canon, purusha, vacana }
}

pub fn generate_all(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8) -> (Vec<String>, Vec<EngineStep>) {
    let Some((dhatu, gana, root_pada, tags, antarganas, aupadeshik)) = load_dhatu_info(dhatu_query) else {
        return (vec![], vec![]);
    };
    let (canonical, db_lakara) = normalize_lakara(lakara);
    let Some(family) = lakara_family(&db_lakara) else { return (vec![], vec![]); };

    // pada check
    let pada = if db_lakara.starts_with('a') || canonical.starts_with('a') { "A" } else { "P" };
    if db_lakara == "plit" { /* pada = P */ }
    if root_pada == "P" && pada == "A" { return (vec![], vec![]); }
    if root_pada == "A" && pada == "P" { return (vec![], vec![]); }

    let cgana = conjugation_gana(gana, &tags);
    let (stem_opt, augment, mut steps) = derive_stem(&dhatu, gana, &family, "shuddha", &tags, &antarganas, &aupadeshik);
    let Some(stem) = stem_opt else { return (vec![], steps); };

    let table = family_endings(&family, "kartari", pada, cgana, Some(&dhatu));
    let Some(table) = table else { return (vec![], steps); };
    let idx = ((purusha - 1) * 3 + (vacana - 1)) as usize;
    if idx >= table.len() { return (vec![], steps); }
    let (variants, sutras) = &table[idx];
    let forms = join_variants(&stem, variants, cgana, &family, purusha, pada, augment.as_deref(), &dhatu, vacana, &antarganas);
    for form in &forms {
        steps.push(EngineStep { form: form.clone(), sutras: sutras.clone(), kind: "tinanta".to_string(), meta: {
            let mut m = std::collections::HashMap::new();
            m.insert("lakara".to_string(), canonical.clone());
            m.insert("purusha".to_string(), purusha.to_string());
            m.insert("vacana".to_string(), vacana.to_string());
            m
        }});
    }
    (forms, steps)
}

pub fn generate_paradigm(dhatu: &str, lakara: &str) -> Vec<ParadigmEntry> {
    let mut out = Vec::new();
    for p in 1..=3 {
        for v in 1..=3 {
            let (forms, _) = generate_all(dhatu, lakara, p, v);
            out.push(ParadigmEntry { purusha: p, vacana: v, forms });
        }
    }
    out
}

pub fn generate_paradigm_with_prefixes(dhatu: &str, lakara: &str, prefixes: &[String]) -> Vec<ParadigmEntry> {
    let mut out = Vec::new();
    for p in 1..=3 {
        for v in 1..=3 {
            let (forms, _) = generate_all(dhatu, lakara, p, v);
            let forms = if prefixes.is_empty() { forms } else { forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect() };
            out.push(ParadigmEntry { purusha: p, vacana: v, forms });
        }
    }
    out
}
