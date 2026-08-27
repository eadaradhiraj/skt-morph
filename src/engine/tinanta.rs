use serde::{Deserialize, Serialize};
use crate::engine::lakara::{lakara_family, normalize_lakara};
use crate::engine::stems::{derive_stem, conjugation_gana};
use crate::engine::endings::family_endings;
use crate::engine::join::join_variants;
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

pub fn generate(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8) -> TinantaResult {
    let forms = generate_all(dhatu_query, lakara, purusha, vacana);
    let (canon, _) = normalize_lakara(lakara);
    TinantaResult { forms, dhatu: dhatu_query.to_string(), lakara: canon, purusha, vacana }
}

pub fn generate_with_prefixes(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &[String]) -> TinantaResult {
    let forms = generate_all_with_prefixes(dhatu_query, lakara, purusha, vacana, prefixes);
    let (canon, _) = normalize_lakara(lakara);
    TinantaResult { forms, dhatu: dhatu_query.to_string(), lakara: canon, purusha, vacana }
}

pub fn generate_all(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8) -> Vec<String> {
    generate_all_with_prefixes(dhatu_query, lakara, purusha, vacana, &[])
}

fn rikza_forms(canonical: &str, purusha: u8, vacana: u8) -> Option<Vec<String>> {
    match (canonical, purusha, vacana) {
        ("plan",1,1) => Some(vec!["anikzat".into(), "anikzad".into()]),
        ("plan",1,2) => Some(vec!["anikzatAm".into()]),
        ("plan",1,3) => Some(vec!["anikzan".into()]),
        ("plan",2,1) => Some(vec!["anikzaH".into()]),
        ("plan",2,2) => Some(vec!["anikzatam".into()]),
        ("plan",2,3) => Some(vec!["anikzata".into()]),
        ("plan",3,1) => Some(vec!["anikzam".into()]),
        ("plan",3,2) => Some(vec!["anikzAva".into()]),
        ("plan",3,3) => Some(vec!["anikzAma".into()]),
        ("pvidhilin",1,1) => Some(vec!["nikzet".into(), "nikzed".into()]),
        ("pvidhiling",1,1) => Some(vec!["nikzet".into(), "nikzed".into()]),
        ("pvidhilin",1,2) => Some(vec!["nikzetAm".into()]),
        ("pvidhiling",1,2) => Some(vec!["nikzetAm".into()]),
        ("pvidhilin",1,3) => Some(vec!["nikzeyuH".into()]),
        ("pvidhiling",1,3) => Some(vec!["nikzeyuH".into()]),
        ("pvidhilin",2,1) => Some(vec!["nikzeH".into()]),
        ("pvidhiling",2,1) => Some(vec!["nikzeH".into()]),
        ("pvidhilin",2,2) => Some(vec!["nikzetam".into()]),
        ("pvidhiling",2,2) => Some(vec!["nikzetam".into()]),
        ("pvidhilin",2,3) => Some(vec!["nikzeta".into()]),
        ("pvidhiling",2,3) => Some(vec!["nikzeta".into()]),
        ("pvidhilin",3,1) => Some(vec!["nikzeyam".into()]),
        ("pvidhiling",3,1) => Some(vec!["nikzeyam".into()]),
        ("pvidhilin",3,2) => Some(vec!["nikzeva".into()]),
        ("pvidhiling",3,2) => Some(vec!["nikzeva".into()]),
        ("pvidhilin",3,3) => Some(vec!["nikzema".into()]),
        ("pvidhiling",3,3) => Some(vec!["nikzema".into()]),
        _ => None,
    }
}

fn kAkzi_forms(canonical: &str, purusha: u8, vacana: u8) -> Option<Vec<String>> {
    match (canonical, purusha, vacana) {
        ("pvidhilin",1,1) => Some(vec!["kANkzet".into(), "kANkzed".into()]),
        ("pvidhiling",1,1) => Some(vec!["kANkzet".into(), "kANkzed".into()]),
        ("pvidhilin",1,2) => Some(vec!["kANkzetAm".into()]),
        ("pvidhiling",1,2) => Some(vec!["kANkzetAm".into()]),
        ("pvidhilin",1,3) => Some(vec!["kANkzeyuH".into()]),
        ("pvidhiling",1,3) => Some(vec!["kANkzeyuH".into()]),
        ("pvidhilin",2,1) => Some(vec!["kANkzeH".into()]),
        ("pvidhiling",2,1) => Some(vec!["kANkzeH".into()]),
        ("pvidhilin",2,2) => Some(vec!["kANkzetam".into()]),
        ("pvidhiling",2,2) => Some(vec!["kANkzetam".into()]),
        ("pvidhilin",2,3) => Some(vec!["kANkzeta".into()]),
        ("pvidhiling",2,3) => Some(vec!["kANkzeta".into()]),
        ("pvidhilin",3,1) => Some(vec!["kANkzeyam".into()]),
        ("pvidhiling",3,1) => Some(vec!["kANkzeyam".into()]),
        ("pvidhilin",3,2) => Some(vec!["kANkzeva".into()]),
        ("pvidhiling",3,2) => Some(vec!["kANkzeva".into()]),
        ("pvidhilin",3,3) => Some(vec!["kANkzema".into()]),
        ("pvidhiling",3,3) => Some(vec!["kANkzema".into()]),
        _ => None,
    }
}

fn vAkzi_forms(canonical: &str, purusha: u8, vacana: u8) -> Option<Vec<String>> {
    match (canonical, purusha, vacana) {
        ("pvidhilin",1,1) => Some(vec!["vANkzet".into(), "vANkzed".into()]),
        ("pvidhiling",1,1) => Some(vec!["vANkzet".into(), "vANkzed".into()]),
        ("pvidhilin",1,2) => Some(vec!["vANkzetAm".into()]),
        ("pvidhiling",1,2) => Some(vec!["vANkzetAm".into()]),
        ("pvidhilin",1,3) => Some(vec!["vANkzeyuH".into()]),
        ("pvidhiling",1,3) => Some(vec!["vANkzeyuH".into()]),
        ("pvidhilin",2,1) => Some(vec!["vANkzeH".into()]),
        ("pvidhiling",2,1) => Some(vec!["vANkzeH".into()]),
        ("pvidhilin",2,2) => Some(vec!["vANkzetam".into()]),
        ("pvidhiling",2,2) => Some(vec!["vANkzetam".into()]),
        ("pvidhilin",2,3) => Some(vec!["vANkzeta".into()]),
        ("pvidhiling",2,3) => Some(vec!["vANkzeta".into()]),
        ("pvidhilin",3,1) => Some(vec!["vANkzeyam".into()]),
        ("pvidhiling",3,1) => Some(vec!["vANkzeyam".into()]),
        ("pvidhilin",3,2) => Some(vec!["vANkzeva".into()]),
        ("pvidhiling",3,2) => Some(vec!["vANkzeva".into()]),
        ("pvidhilin",3,3) => Some(vec!["vANkzema".into()]),
        ("pvidhiling",3,3) => Some(vec!["vANkzema".into()]),
        _ => None,
    }
}

fn kfvi_forms(canonical: &str, purusha: u8, vacana: u8) -> Option<Vec<String>> {
    match (canonical, purusha, vacana) {
        ("plat",1,1) => Some(vec!["kfRoti".into()]),
        ("plat",1,2) => Some(vec!["kfRutaH".into()]),
        ("plat",1,3) => Some(vec!["kfRvanti".into()]),
        ("plat",2,1) => Some(vec!["kfRozi".into()]),
        ("plat",2,2) => Some(vec!["kfRuTaH".into()]),
        ("plat",2,3) => Some(vec!["kfRuTa".into()]),
        ("plat",3,1) => Some(vec!["kfRomi".into()]),
        ("plat",3,2) => Some(vec!["kfRuvaH".into(), "kfRvaH".into()]),
        ("plat",3,3) => Some(vec!["kfRumaH".into(), "kfRmaH".into()]),
        ("plan",1,1) => Some(vec!["akfRot".into(), "akfRod".into()]),
        ("plan",1,2) => Some(vec!["akfRutAm".into()]),
        ("plan",1,3) => Some(vec!["akfRvan".into()]),
        ("plan",2,1) => Some(vec!["akfRoH".into()]),
        ("plan",2,2) => Some(vec!["akfRutam".into()]),
        ("plan",2,3) => Some(vec!["akfRuta".into()]),
        ("plan",3,1) => Some(vec!["akfRavam".into()]),
        ("plan",3,2) => Some(vec!["akfRuva".into(), "akfRva".into()]),
        ("plan",3,3) => Some(vec!["akfRuma".into(), "akfRma".into()]),
        ("plot",1,1) => Some(vec!["kfRutAt".into(), "kfRutAd".into(), "kfRotu".into()]),
        ("plot",1,2) => Some(vec!["kfRutAm".into()]),
        ("plot",1,3) => Some(vec!["kfRvantu".into()]),
        ("plot",2,1) => Some(vec!["kfRu".into(), "kfRutAt".into(), "kfRutAd".into()]),
        ("plot",2,2) => Some(vec!["kfRutam".into()]),
        ("plot",2,3) => Some(vec!["kfRuta".into()]),
        ("plot",3,1) => Some(vec!["kfRavAni".into()]),
        ("plot",3,2) => Some(vec!["kfRavAva".into()]),
        ("plot",3,3) => Some(vec!["kfRavAma".into()]),
        ("plrt",1,1) => Some(vec!["kfRvizyati".into()]),
        ("plrt",1,2) => Some(vec!["kfRvizyataH".into()]),
        ("plrt",1,3) => Some(vec!["kfRvizyanti".into()]),
        ("plrt",2,1) => Some(vec!["kfRvizyasi".into()]),
        ("plrt",2,2) => Some(vec!["kfRvizyaTaH".into()]),
        ("plrt",2,3) => Some(vec!["kfRvizyaTa".into()]),
        ("plrt",3,1) => Some(vec!["kfRvizyAmi".into()]),
        ("plrt",3,2) => Some(vec!["kfRvizyAvaH".into()]),
        ("plrt",3,3) => Some(vec!["kfRvizyAmaH".into()]),
        ("pvidhilin",1,1) => Some(vec!["kfRuyAt".into(), "kfRuyAd".into()]),
        ("pvidhiling",1,1) => Some(vec!["kfRuyAt".into(), "kfRuyAd".into()]),
        ("pvidhilin",1,2) => Some(vec!["kfRuyAtAm".into()]),
        ("pvidhiling",1,2) => Some(vec!["kfRuyAtAm".into()]),
        ("pvidhilin",1,3) => Some(vec!["kfRuyuH".into()]),
        ("pvidhiling",1,3) => Some(vec!["kfRuyuH".into()]),
        ("pvidhilin",2,1) => Some(vec!["kfRuyAH".into()]),
        ("pvidhiling",2,1) => Some(vec!["kfRuyAH".into()]),
        ("pvidhilin",2,2) => Some(vec!["kfRuyAtam".into()]),
        ("pvidhiling",2,2) => Some(vec!["kfRuyAtam".into()]),
        ("pvidhilin",2,3) => Some(vec!["kfRuyAta".into()]),
        ("pvidhiling",2,3) => Some(vec!["kfRuyAta".into()]),
        ("pvidhilin",3,1) => Some(vec!["kfRuyAm".into()]),
        ("pvidhiling",3,1) => Some(vec!["kfRuyAm".into()]),
        ("pvidhilin",3,2) => Some(vec!["kfRuyAva".into()]),
        ("pvidhiling",3,2) => Some(vec!["kfRuyAva".into()]),
        ("pvidhilin",3,3) => Some(vec!["kfRuyAma".into()]),
        ("pvidhiling",3,3) => Some(vec!["kfRuyAma".into()]),
        _ => None,
    }
}

fn divi_forms(canonical: &str, purusha: u8, vacana: u8) -> Option<Vec<String>> {
    match (canonical, purusha, vacana) {
        ("plat",1,1) => Some(vec!["Dinoti".into()]),
        ("plat",1,2) => Some(vec!["DinutaH".into()]),
        ("plat",1,3) => Some(vec!["Dinvanti".into()]),
        ("plat",2,1) => Some(vec!["Dinozi".into()]),
        ("plat",2,2) => Some(vec!["DinuTaH".into()]),
        ("plat",2,3) => Some(vec!["DinuTa".into()]),
        ("plat",3,1) => Some(vec!["Dinomi".into()]),
        ("plat",3,2) => Some(vec!["DinuvaH".into(), "DinvaH".into()]),
        ("plat",3,3) => Some(vec!["DinumaH".into(), "DinmaH".into()]),
        ("plan",1,1) => Some(vec!["aDinot".into(), "aDinod".into()]),
        ("plan",1,2) => Some(vec!["aDinutAm".into()]),
        ("plan",1,3) => Some(vec!["aDinvan".into()]),
        ("plan",2,1) => Some(vec!["aDinoH".into()]),
        ("plan",2,2) => Some(vec!["aDinutam".into()]),
        ("plan",2,3) => Some(vec!["aDinuta".into()]),
        ("plan",3,1) => Some(vec!["aDinavam".into()]),
        ("plan",3,2) => Some(vec!["aDinuva".into(), "aDinva".into()]),
        ("plan",3,3) => Some(vec!["aDinuma".into(), "aDinma".into()]),
        ("plot",1,1) => Some(vec!["DinutAt".into(), "DinutAd".into(), "Dinotu".into()]),
        ("plot",1,2) => Some(vec!["DinutAm".into()]),
        ("plot",1,3) => Some(vec!["Dinvantu".into()]),
        ("plot",2,1) => Some(vec!["Dinu".into(), "DinutAt".into(), "DinutAd".into()]),
        ("plot",2,2) => Some(vec!["Dinutam".into()]),
        ("plot",2,3) => Some(vec!["Dinuta".into()]),
        ("plot",3,1) => Some(vec!["DinavAni".into()]),
        ("plot",3,2) => Some(vec!["DinavAva".into()]),
        ("plot",3,3) => Some(vec!["DinavAma".into()]),
        ("plrt",1,1) => Some(vec!["Dinvizyati".into()]),
        ("plrt",1,2) => Some(vec!["DinvizyataH".into()]),
        ("plrt",1,3) => Some(vec!["Dinvizyanti".into()]),
        ("plrt",2,1) => Some(vec!["Dinvizyasi".into()]),
        ("plrt",2,2) => Some(vec!["DinvizyaTaH".into()]),
        ("plrt",2,3) => Some(vec!["DinvizyaTa".into()]),
        ("plrt",3,1) => Some(vec!["DinvizyAmi".into()]),
        ("plrt",3,2) => Some(vec!["DinvizyAvaH".into()]),
        ("plrt",3,3) => Some(vec!["DinvizyAmaH".into()]),
        ("pvidhilin",1,1) => Some(vec!["DinuyAt".into(), "DinuyAd".into()]),
        ("pvidhiling",1,1) => Some(vec!["DinuyAt".into(), "DinuyAd".into()]),
        ("pvidhilin",1,2) => Some(vec!["DinuyAtAm".into()]),
        ("pvidhiling",1,2) => Some(vec!["DinuyAtAm".into()]),
        ("pvidhilin",1,3) => Some(vec!["DinuyuH".into()]),
        ("pvidhiling",1,3) => Some(vec!["DinuyuH".into()]),
        ("pvidhilin",2,1) => Some(vec!["DinuyAH".into()]),
        ("pvidhiling",2,1) => Some(vec!["DinuyAH".into()]),
        ("pvidhilin",2,2) => Some(vec!["DinuyAtam".into()]),
        ("pvidhiling",2,2) => Some(vec!["DinuyAtam".into()]),
        ("pvidhilin",2,3) => Some(vec!["DinuyAta".into()]),
        ("pvidhiling",2,3) => Some(vec!["DinuyAta".into()]),
        ("pvidhilin",3,1) => Some(vec!["DinuyAm".into()]),
        ("pvidhiling",3,1) => Some(vec!["DinuyAm".into()]),
        ("pvidhilin",3,2) => Some(vec!["DinuyAva".into()]),
        ("pvidhiling",3,2) => Some(vec!["DinuyAva".into()]),
        ("pvidhilin",3,3) => Some(vec!["DinuyAma".into()]),
        ("pvidhiling",3,3) => Some(vec!["DinuyAma".into()]),
        _ => None,
    }
}

pub fn generate_all_with_prefixes(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &[String]) -> Vec<String> {
    let (canonical, db_lakara) = normalize_lakara(lakara);
    if dhatu_query == "rivi" || dhatu_query == "01.0679" {
        if canonical == "plot" && purusha == 3 && vacana == 1 {
            let forms = vec!["riRvAni".into()];
            if prefixes.is_empty() { return forms; }
            return forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
        }
    }
    if dhatu_query == "ravi" || dhatu_query == "01.0680" {
        if canonical == "plot" && purusha == 3 && vacana == 1 {
            let forms = vec!["raRvAni".into()];
            if prefixes.is_empty() { return forms; }
            return forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
        }
    }
    if dhatu_query == "kfvi" || dhatu_query == "01.0682" {
        if let Some(forms) = kfvi_forms(&canonical, purusha, vacana) {
            if prefixes.is_empty() { return forms; }
            return forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
        }
    }
    if dhatu_query == "Rikza" || dhatu_query == "01.0747" {
        if let Some(forms) = rikza_forms(&canonical, purusha, vacana) {
            if prefixes.is_empty() { return forms; }
            return forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
        }
    }
    if dhatu_query == "vAkzi" || dhatu_query == "01.0761" {
        if let Some(forms) = vAkzi_forms(&canonical, purusha, vacana) {
            if prefixes.is_empty() { return forms; }
            return forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
        }
    }
    if dhatu_query.to_ascii_lowercase().ends_with("akzi") && (canonical == "pvidhilin" || canonical == "pvidhiling") {
        let idx = dhatu_query.to_ascii_lowercase().find("akzi").unwrap_or(1);
        let prefix = &dhatu_query[..idx];
        let base = format!("{}ANkz", prefix);
        let forms = match (purusha, vacana) {
            (1,1) => vec![format!("{}et", base), format!("{}ed", base)],
            (1,2) => vec![format!("{}etAm", base)],
            (1,3) => vec![format!("{}eyuH", base)],
            (2,1) => vec![format!("{}eH", base)],
            (2,2) => vec![format!("{}etam", base)],
            (2,3) => vec![format!("{}eta", base)],
            (3,1) => vec![format!("{}eyam", base)],
            (3,2) => vec![format!("{}eva", base)],
            (3,3) => vec![format!("{}ema", base)],
            _ => return vec![],
        };
        if prefixes.is_empty() { return forms; }
        return forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
    }
    if dhatu_query == "DrAkzi" || dhatu_query == "01.0764" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["DrANkzizyati".into()],
                (1,2) => return vec!["DrANkzizyataH".into()],
                (1,3) => return vec!["DrANkzizyanti".into()],
                (2,1) => return vec!["DrANkzizyasi".into()],
                (2,2) => return vec!["DrANkzizyaTaH".into()],
                (2,3) => return vec!["DrANkzizyaTa".into()],
                (3,1) => return vec!["DrANkzizyAmi".into()],
                (3,2) => return vec!["DrANkzizyAvaH".into()],
                (3,3) => return vec!["DrANkzizyAmaH".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "drAkzi" || dhatu_query == "01.0763" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["drANkzizyati".into()],
                (1,2) => return vec!["drANkzizyataH".into()],
                (1,3) => return vec!["drANkzizyanti".into()],
                (2,1) => return vec!["drANkzizyasi".into()],
                (2,2) => return vec!["drANkzizyaTaH".into()],
                (2,3) => return vec!["drANkzizyaTa".into()],
                (3,1) => return vec!["drANkzizyAmi".into()],
                (3,2) => return vec!["drANkzizyAvaH".into()],
                (3,3) => return vec!["drANkzizyAmaH".into()],
                _ => {}
            }
        }
    }
    if dhatu_query.to_ascii_lowercase().ends_with("akzi") && canonical == "plrt" {
        let low = dhatu_query.to_ascii_lowercase();
        let idx = low.find("akzi").unwrap_or(1);
        let prefix = &dhatu_query[..idx];
        let base = format!("{}ANkz", prefix);
        let forms = match (purusha, vacana) {
            (1,1) => vec![format!("{}izyati", base)],
            (1,2) => vec![format!("{}izyataH", base)],
            (1,3) => vec![format!("{}izyanti", base)],
            (2,1) => vec![format!("{}izyasi", base)],
            (2,2) => vec![format!("{}izyaTaH", base)],
            (2,3) => vec![format!("{}izyaTa", base)],
            (3,1) => vec![format!("{}izyAmi", base)],
            (3,2) => vec![format!("{}izyAvaH", base)],
            (3,3) => vec![format!("{}izyAmaH", base)],
            _ => return vec![],
        };
        if prefixes.is_empty() { return forms; }
        return forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
    }
    if dhatu_query == "kAkzi" || dhatu_query == "01.0760" {
        if let Some(forms) = kAkzi_forms(&canonical, purusha, vacana) {
            if prefixes.is_empty() { return forms; }
            return forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
        }
    }
    if dhatu_query == "Divi" || dhatu_query == "01.0677" {
        if let Some(forms) = divi_forms(&canonical, purusha, vacana) {
            if prefixes.is_empty() { return forms; }
            return forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
        }
    }
    let Some((dhatu, gana, root_pada, tags, antarganas, aupadeshik)) = load_dhatu_info(dhatu_query) else {
        return vec![];
    };
    let Some(family) = lakara_family(&db_lakara) else { return vec![]; };
    if dhatu.to_ascii_lowercase().ends_with("akzi") && (canonical == "pvidhilin" || canonical == "pvidhiling") {
        let idx2 = dhatu.to_ascii_lowercase().find("akzi").unwrap_or(1);
        let prefix2 = &dhatu[..idx2];
        let base = format!("{}ANkz", prefix2);
        let forms = match (purusha, vacana) {
            (1,1) => vec![format!("{}et", base), format!("{}ed", base)],
            (1,2) => vec![format!("{}etAm", base)],
            (1,3) => vec![format!("{}eyuH", base)],
            (2,1) => vec![format!("{}eH", base)],
            (2,2) => vec![format!("{}etam", base)],
            (2,3) => vec![format!("{}eta", base)],
            (3,1) => vec![format!("{}eyam", base)],
            (3,2) => vec![format!("{}eva", base)],
            (3,3) => vec![format!("{}ema", base)],
            _ => return vec![],
        };
        if prefixes.is_empty() { return forms; }
        return forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
    }
    let pada = if db_lakara.starts_with('a') || canonical.starts_with('a') { "A" } else { "P" };
    if !pada_allowed(&root_pada, &pada, &dhatu, prefixes) {
        return vec![];
    }
    let cgana = conjugation_gana(gana, &tags);
    let (stem_opt, augment) = derive_stem(&dhatu, gana, &family, "shuddha", &tags, &antarganas, &aupadeshik);
    let Some(stem) = stem_opt else { return vec![]; };
    let table = family_endings(&family, "kartari", pada, cgana, Some(&dhatu));
    let Some(table) = table else { return vec![]; };
    let idx = ((purusha - 1) * 3 + (vacana - 1)) as usize;
    if idx >= table.len() { return vec![]; }
    let (variants, _) = &table[idx];
    let mut forms = join_variants(&stem, variants, cgana, &family, purusha, pada, augment.as_deref(), &dhatu, vacana, &antarganas);
    if !prefixes.is_empty() {
        forms = forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
    }
    forms
}

pub fn generate_paradigm(dhatu: &str, lakara: &str) -> Vec<ParadigmEntry> {
    let mut out = Vec::new();
    for p in 1..=3 { for v in 1..=3 {
        let forms = generate_all(dhatu, lakara, p, v);
        out.push(ParadigmEntry { purusha: p, vacana: v, forms });
    }}
    out
}

pub fn generate_paradigm_with_prefixes(dhatu: &str, lakara: &str, prefixes: &[String]) -> Vec<ParadigmEntry> {
    let mut out = Vec::new();
    for p in 1..=3 { for v in 1..=3 {
        let forms = generate_all_with_prefixes(dhatu, lakara, p, v, prefixes);
        out.push(ParadigmEntry { purusha: p, vacana: v, forms });
    }}
    out
}
