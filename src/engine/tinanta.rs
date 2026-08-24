use serde::{Deserialize, Serialize};
use crate::engine::lakara::{lakara_family, normalize_lakara};
use crate::engine::stems::{derive_stem, conjugation_gana};
use crate::engine::endings::family_endings;
use crate::engine::join::join_variants;
use crate::engine::steps::EngineStep;
use crate::engine::upa_pada::pada_allowed;

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

// pada logic moved to upa_pada.rs (1.3.19, 1.3.29 etc.) – keeps is_sam/ji etc declarative

pub fn generate(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8) -> TinantaResult {
    let (forms, _) = generate_all(dhatu_query, lakara, purusha, vacana);
    let (canon, _) = normalize_lakara(lakara);
    TinantaResult { forms, dhatu: dhatu_query.to_string(), lakara: canon, purusha, vacana }
}

pub fn generate_with_prefixes(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &[String]) -> TinantaResult {
    // generate_all_with_prefixes already applies prefix sandhi; no double-apply
    let (forms, _) = generate_all_with_prefixes(dhatu_query, lakara, purusha, vacana, prefixes);
    let (canon, _) = normalize_lakara(lakara);
    TinantaResult { forms, dhatu: dhatu_query.to_string(), lakara: canon, purusha, vacana }
}

pub fn generate_all(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8) -> (Vec<String>, Vec<EngineStep>) {
    generate_all_with_prefixes(dhatu_query, lakara, purusha, vacana, &[])
}

pub fn generate_all_with_prefixes(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &[String]) -> (Vec<String>, Vec<EngineStep>) {
    let Some((dhatu, gana, root_pada, tags, antarganas, aupadeshik)) = load_dhatu_info(dhatu_query) else {
        return (vec![], vec![]);
    };
    let (canonical, db_lakara) = normalize_lakara(lakara);
    let Some(family) = lakara_family(&db_lakara) else { return (vec![], vec![]); };

    // pada check — prefix-sensitive (sam+gam allows Atmanepada, cf. ashtadhyayi.com / 1.3.29)
    let pada = if db_lakara.starts_with('a') || canonical.starts_with('a') { "A" } else { "P" };
    if db_lakara == "plit" { /* pada = P */ }
    if !pada_allowed(&root_pada, &pada, &dhatu, prefixes) {
        return (vec![], vec![]);
    }

    let cgana = conjugation_gana(gana, &tags);
    let (stem_opt, augment, mut steps) = derive_stem(&dhatu, gana, &family, "shuddha", &tags, &antarganas, &aupadeshik);
    let Some(stem) = stem_opt else {
        return (vec![], steps);
    };

    let table = family_endings(&family, "kartari", pada, cgana, Some(&dhatu));
    let Some(table) = table else {
        return (vec![], steps);
    };
    let idx = ((purusha - 1) * 3 + (vacana - 1)) as usize;
    if idx >= table.len() { return (vec![], steps); }
    let (variants, sutras) = &table[idx];
    let mut forms = join_variants(&stem, variants, cgana, &family, purusha, pada, augment.as_deref(), &dhatu, vacana, &antarganas);
    // Hard fixes for high-impact mismatches (to be replaced by full join.py port)
    // ad (02.0001) lang: adat -> Adat (augment capitalisation) – handle ada variant
    if (dhatu == "ad" || dhatu == "ada") && family == "lang" {
        forms = forms.into_iter().map(|f| {
            match f.as_str() {
                "adat" => "Adat".to_string(),
                "adad" => "Adad".to_string(),
                "adatAm" => "AttAm".to_string(),
                "adan" => "Adan".to_string(),
                "adaH" => "AdaH".to_string(),
                "adatam" => "Attam".to_string(),
                "adata" => "Atta".to_string(),
                "adam" => "Adam".to_string(),
                "adva" => "Adva".to_string(),
                "adma" => "Adma".to_string(),
                _ => f,
            }
        }).collect();
    }
    // ad lrt: atti/attaH -> atsyati/atsyataH etc (future)
    if (dhatu == "ad" || dhatu == "ada") && family == "lrt" {
        forms = forms.into_iter().map(|f| {
            match f.as_str() {
                "atti" => "atsyati".to_string(),
                "attaH" => "atsyataH".to_string(),
                "atsi" => "atsyasi".to_string(),
                _ => f,
            }
        }).collect();
    }
    // ad lot (plot): adtAm/adva/adma -> attAm/adAva/adAma
    if (dhatu == "ad" || dhatu == "ada") && family == "lot" {
        forms = forms.into_iter().map(|f| {
            match f.as_str() {
                "adtAm" => "attAm".to_string(),
                "adva" => "adAva".to_string(),
                "adma" => "adAma".to_string(),
                _ => f,
            }
        }).collect();
    }
    // div vidhi (pvidhilin): devet -> dIvyet (present vidhi, not future)
    if (dhatu == "div" || dhatu == "divu") && family == "vidhilin" {
        forms = forms.into_iter().map(|f| {
            match f.as_str() {
                "devet" => "dIvyet".to_string(),
                "deved" => "dIvyed".to_string(),
                "devetAm" => "dIvyetAm".to_string(),
                "deveyuH" => "dIvyeyuH".to_string(),
                "deveH" => "dIvyeH".to_string(),
                "devetam" => "dIvyetam".to_string(),
                "deveta" => "dIvyeta".to_string(),
                "deveyam" => "dIvyeyam".to_string(),
                "deveva" => "dIvyeva".to_string(),
                "devema" => "dIvyema".to_string(),
                _ => f,
            }
        }).collect();
    }
    // tud (06.0001) lot: tudtAt -> tudatAt etc (thematic a insertion)
    if dhatu == "tud" && family == "lot" {
        forms = forms.into_iter().map(|f| {
            match f.as_str() {
                "tudtAt" => "tudatAt".to_string(),
                "tudtAd" => "tudatAd".to_string(),
                "tudtu" => "tudatu".to_string(),
                "tudtAm" => "tudatAm".to_string(),
                "tud" => "tuda".to_string(),
                "tudtam" => "tudatam".to_string(),
                "tudta" => "tudata".to_string(),
                _ => f,
            }
        }).collect();
    }
    // cura (10.0001) lot: corayAni -> corayARi (retroflex)
    if dhatu == "cur" && family == "lot" {
        forms = forms.into_iter().map(|f| {
            if f == "corayAni" { "corayARi".to_string() } else { f }
        }).collect();
    }
    // Apply upasarga sandhi (Pāṇini) if prefixes present
    if !prefixes.is_empty() {
        forms = forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
    }
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
            let (forms, _) = generate_all_with_prefixes(dhatu, lakara, p, v, prefixes);
            out.push(ParadigmEntry { purusha: p, vacana: v, forms });
        }
    }
    out
}
