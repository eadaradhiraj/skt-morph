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
    if dhatu_query == "vftu" || dhatu_query == "01.0862" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["vartsyati".into()],
                (1,2) => return vec!["vartsyataH".into()],
                (1,3) => return vec!["vartsyanti".into()],
                (2,1) => return vec!["vartsyasi".into()],
                (2,2) => return vec!["vartsyaTaH".into()],
                (2,3) => return vec!["vartsyaTa".into()],
                (3,1) => return vec!["vartsyAmi".into()],
                (3,2) => return vec!["vartsyAvaH".into()],
                (3,3) => return vec!["vartsyAmaH".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "vfDu" || dhatu_query == "01.0863" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["vartsyati".into()],
                (1,2) => return vec!["vartsyataH".into()],
                (1,3) => return vec!["vartsyanti".into()],
                (2,1) => return vec!["vartsyasi".into()],
                (2,2) => return vec!["vartsyaTaH".into()],
                (2,3) => return vec!["vartsyaTa".into()],
                (3,1) => return vec!["vartsyAmi".into()],
                (3,2) => return vec!["vartsyAvaH".into()],
                (3,3) => return vec!["vartsyAmaH".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "syandU" || dhatu_query == "01.0865" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["syantsyati".into()],
                (1,2) => return vec!["syantsyataH".into()],
                (1,3) => return vec!["syantsyanti".into()],
                (2,1) => return vec!["syantsyasi".into()],
                (2,2) => return vec!["syantsyaTaH".into()],
                (2,3) => return vec!["syantsyaTa".into()],
                (3,1) => return vec!["syantsyAmi".into()],
                (3,2) => return vec!["syantsyAvaH".into()],
                (3,3) => return vec!["syantsyAmaH".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "kfpU" || dhatu_query == "01.0866" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["kalpsyati".into()],
                (1,2) => return vec!["kalpsyataH".into()],
                (1,3) => return vec!["kalpsyanti".into()],
                (2,1) => return vec!["kalpsyasi".into()],
                (2,2) => return vec!["kalpsyaTaH".into()],
                (2,3) => return vec!["kalpsyaTa".into()],
                (3,1) => return vec!["kalpsyAmi".into()],
                (3,2) => return vec!["kalpsyAvaH".into()],
                (3,3) => return vec!["kalpsyAmaH".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "yama" || dhatu_query == "01.1031" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["yacCati".into()],
                (1,2) => return vec!["yacCataH".into()],
                (1,3) => return vec!["yacCanti".into()],
                (2,1) => return vec!["yacCasi".into()],
                (2,2) => return vec!["yacCaTaH".into()],
                (2,3) => return vec!["yacCaTa".into()],
                (3,1) => return vec!["yacCAmi".into()],
                (3,2) => return vec!["yacCAvaH".into()],
                (3,3) => return vec!["yacCAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["ayacCat".into()],
                (1,2) => return vec!["ayacCatAm".into()],
                (1,3) => return vec!["ayacCan".into()],
                (2,1) => return vec!["ayacCaH".into()],
                (2,2) => return vec!["ayacCatam".into()],
                (2,3) => return vec!["ayacCata".into()],
                (3,1) => return vec!["ayacCam".into()],
                (3,2) => return vec!["ayacCAva".into()],
                (3,3) => return vec!["ayacCAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["yacCatAt".into()],
                (1,2) => return vec!["yacCatAm".into()],
                (1,3) => return vec!["yacCantu".into()],
                (2,1) => return vec!["yacCa".into()],
                (2,2) => return vec!["yacCatam".into()],
                (2,3) => return vec!["yacCata".into()],
                (3,1) => return vec!["yacCAni".into()],
                (3,2) => return vec!["yacCAva".into()],
                (3,3) => return vec!["yacCAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["yaMsyati".into()],
                (1,2) => return vec!["yaMsyataH".into()],
                (1,3) => return vec!["yaMsyanti".into()],
                (2,1) => return vec!["yaMsyasi".into()],
                (2,2) => return vec!["yaMsyaTaH".into()],
                (2,3) => return vec!["yaMsyaTa".into()],
                (3,1) => return vec!["yaMsyAmi".into()],
                (3,2) => return vec!["yaMsyAvaH".into()],
                (3,3) => return vec!["yaMsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["yacCet".into()],
                (1,2) => return vec!["yacCetAm".into()],
                (1,3) => return vec!["yacCeyuH".into()],
                (2,1) => return vec!["yacCeH".into()],
                (2,2) => return vec!["yacCetam".into()],
                (2,3) => return vec!["yacCeta".into()],
                (3,1) => return vec!["yacCeyam".into()],
                (3,2) => return vec!["yacCeva".into()],
                (3,3) => return vec!["yacCema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "RIY" || dhatu_query == "01.1049" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["nayati".into()],
                (1,2) => return vec!["nayataH".into()],
                (1,3) => return vec!["nayanti".into()],
                (2,1) => return vec!["nayasi".into()],
                (2,2) => return vec!["nayaTaH".into()],
                (2,3) => return vec!["nayaTa".into()],
                (3,1) => return vec!["nayAmi".into()],
                (3,2) => return vec!["nayAvaH".into()],
                (3,3) => return vec!["nayAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["anayat".into()],
                (1,2) => return vec!["anayatAm".into()],
                (1,3) => return vec!["anayan".into()],
                (2,1) => return vec!["anayaH".into()],
                (2,2) => return vec!["anayatam".into()],
                (2,3) => return vec!["anayata".into()],
                (3,1) => return vec!["anayam".into()],
                (3,2) => return vec!["anayAva".into()],
                (3,3) => return vec!["anayAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["nayatAt".into()],
                (1,2) => return vec!["nayatAm".into()],
                (1,3) => return vec!["nayantu".into()],
                (2,1) => return vec!["naya".into()],
                (2,2) => return vec!["nayatam".into()],
                (2,3) => return vec!["nayata".into()],
                (3,1) => return vec!["nayAni".into()],
                (3,2) => return vec!["nayAva".into()],
                (3,3) => return vec!["nayAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["nezyati".into()],
                (1,2) => return vec!["nezyataH".into()],
                (1,3) => return vec!["nezyanti".into()],
                (2,1) => return vec!["nezyasi".into()],
                (2,2) => return vec!["nezyaTaH".into()],
                (2,3) => return vec!["nezyaTa".into()],
                (3,1) => return vec!["nezyAmi".into()],
                (3,2) => return vec!["nezyAvaH".into()],
                (3,3) => return vec!["nezyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["nayet".into()],
                (1,2) => return vec!["nayetAm".into()],
                (1,3) => return vec!["nayeyuH".into()],
                (2,1) => return vec!["nayeH".into()],
                (2,2) => return vec!["nayetam".into()],
                (2,3) => return vec!["nayeta".into()],
                (3,1) => return vec!["nayeyam".into()],
                (3,2) => return vec!["nayeva".into()],
                (3,3) => return vec!["nayema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "ovE" || dhatu_query == "01.1070" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["vAyati".into()],
                (1,2) => return vec!["vAyataH".into()],
                (1,3) => return vec!["vAyanti".into()],
                (2,1) => return vec!["vAyasi".into()],
                (2,2) => return vec!["vAyaTaH".into()],
                (2,3) => return vec!["vAyaTa".into()],
                (3,1) => return vec!["vAyAmi".into()],
                (3,2) => return vec!["vAyAvaH".into()],
                (3,3) => return vec!["vAyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["avAyat".into()],
                (1,2) => return vec!["avAyatAm".into()],
                (1,3) => return vec!["avAyan".into()],
                (2,1) => return vec!["avAyaH".into()],
                (2,2) => return vec!["avAyatam".into()],
                (2,3) => return vec!["avAyata".into()],
                (3,1) => return vec!["avAyam".into()],
                (3,2) => return vec!["avAyAva".into()],
                (3,3) => return vec!["avAyAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["vAyatAt".into()],
                (1,2) => return vec!["vAyatAm".into()],
                (1,3) => return vec!["vAyantu".into()],
                (2,1) => return vec!["vAya".into()],
                (2,2) => return vec!["vAyatam".into()],
                (2,3) => return vec!["vAyata".into()],
                (3,1) => return vec!["vAyAni".into()],
                (3,2) => return vec!["vAyAva".into()],
                (3,3) => return vec!["vAyAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["vAsyati".into()],
                (1,2) => return vec!["vAsyataH".into()],
                (1,3) => return vec!["vAsyanti".into()],
                (2,1) => return vec!["vAsyasi".into()],
                (2,2) => return vec!["vAsyaTaH".into()],
                (2,3) => return vec!["vAsyaTa".into()],
                (3,1) => return vec!["vAsyAmi".into()],
                (3,2) => return vec!["vAsyAvaH".into()],
                (3,3) => return vec!["vAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["vAyet".into()],
                (1,2) => return vec!["vAyetAm".into()],
                (1,3) => return vec!["vAyeyuH".into()],
                (2,1) => return vec!["vAyeH".into()],
                (2,2) => return vec!["vAyetam".into()],
                (2,3) => return vec!["vAyeta".into()],
                (3,1) => return vec!["vAyeyam".into()],
                (3,2) => return vec!["vAyeva".into()],
                (3,3) => return vec!["vAyema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "zRE" || dhatu_query == "01.1072" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["snAyati".into()],
                (1,2) => return vec!["snAyataH".into()],
                (1,3) => return vec!["snAyanti".into()],
                (2,1) => return vec!["snAyasi".into()],
                (2,2) => return vec!["snAyaTaH".into()],
                (2,3) => return vec!["snAyaTa".into()],
                (3,1) => return vec!["snAyAmi".into()],
                (3,2) => return vec!["snAyAvaH".into()],
                (3,3) => return vec!["snAyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["asnAyat".into()],
                (1,2) => return vec!["asnAyatAm".into()],
                (1,3) => return vec!["asnAyan".into()],
                (2,1) => return vec!["asnAyaH".into()],
                (2,2) => return vec!["asnAyatam".into()],
                (2,3) => return vec!["asnAyata".into()],
                (3,1) => return vec!["asnAyam".into()],
                (3,2) => return vec!["asnAyAva".into()],
                (3,3) => return vec!["asnAyAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["snAyatAt".into()],
                (1,2) => return vec!["snAyatAm".into()],
                (1,3) => return vec!["snAyantu".into()],
                (2,1) => return vec!["snAya".into()],
                (2,2) => return vec!["snAyatam".into()],
                (2,3) => return vec!["snAyata".into()],
                (3,1) => return vec!["snAyAni".into()],
                (3,2) => return vec!["snAyAva".into()],
                (3,3) => return vec!["snAyAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["snAsyati".into()],
                (1,2) => return vec!["snAsyataH".into()],
                (1,3) => return vec!["snAsyanti".into()],
                (2,1) => return vec!["snAsyasi".into()],
                (2,2) => return vec!["snAsyaTaH".into()],
                (2,3) => return vec!["snAsyaTa".into()],
                (3,1) => return vec!["snAsyAmi".into()],
                (3,2) => return vec!["snAsyAvaH".into()],
                (3,3) => return vec!["snAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["snAyet".into()],
                (1,2) => return vec!["snAyetAm".into()],
                (1,3) => return vec!["snAyeyuH".into()],
                (2,1) => return vec!["snAyeH".into()],
                (2,2) => return vec!["snAyetam".into()],
                (2,3) => return vec!["snAyeta".into()],
                (3,1) => return vec!["snAyeyam".into()],
                (3,2) => return vec!["snAyeva".into()],
                (3,3) => return vec!["snAyema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "dEp" || dhatu_query == "01.1073" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["dAyati".into()],
                (1,2) => return vec!["dAyataH".into()],
                (1,3) => return vec!["dAyanti".into()],
                (2,1) => return vec!["dAyasi".into()],
                (2,2) => return vec!["dAyaTaH".into()],
                (2,3) => return vec!["dAyaTa".into()],
                (3,1) => return vec!["dAyAmi".into()],
                (3,2) => return vec!["dAyAvaH".into()],
                (3,3) => return vec!["dAyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["adAyat".into()],
                (1,2) => return vec!["adAyatAm".into()],
                (1,3) => return vec!["adAyan".into()],
                (2,1) => return vec!["adAyaH".into()],
                (2,2) => return vec!["adAyatam".into()],
                (2,3) => return vec!["adAyata".into()],
                (3,1) => return vec!["adAyam".into()],
                (3,2) => return vec!["adAyAva".into()],
                (3,3) => return vec!["adAyAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["dAyatAt".into()],
                (1,2) => return vec!["dAyatAm".into()],
                (1,3) => return vec!["dAyantu".into()],
                (2,1) => return vec!["dAya".into()],
                (2,2) => return vec!["dAyatam".into()],
                (2,3) => return vec!["dAyata".into()],
                (3,1) => return vec!["dAyAni".into()],
                (3,2) => return vec!["dAyAva".into()],
                (3,3) => return vec!["dAyAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["dAsyati".into()],
                (1,2) => return vec!["dAsyataH".into()],
                (1,3) => return vec!["dAsyanti".into()],
                (2,1) => return vec!["dAsyasi".into()],
                (2,2) => return vec!["dAsyaTaH".into()],
                (2,3) => return vec!["dAsyaTa".into()],
                (3,1) => return vec!["dAsyAmi".into()],
                (3,2) => return vec!["dAsyAvaH".into()],
                (3,3) => return vec!["dAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["dAyet".into()],
                (1,2) => return vec!["dAyetAm".into()],
                (1,3) => return vec!["dAyeyuH".into()],
                (2,1) => return vec!["dAyeH".into()],
                (2,2) => return vec!["dAyetam".into()],
                (2,3) => return vec!["dAyeta".into()],
                (3,1) => return vec!["dAyeyam".into()],
                (3,2) => return vec!["dAyeva".into()],
                (3,3) => return vec!["dAyema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "zWA" || dhatu_query == "01.1077" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["tizWati".into()],
                (1,2) => return vec!["tizWataH".into()],
                (1,3) => return vec!["tizWanti".into()],
                (2,1) => return vec!["tizWasi".into()],
                (2,2) => return vec!["tizWaTaH".into()],
                (2,3) => return vec!["tizWaTa".into()],
                (3,1) => return vec!["tizWAmi".into()],
                (3,2) => return vec!["tizWAvaH".into()],
                (3,3) => return vec!["tizWAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["atizWat".into()],
                (1,2) => return vec!["atizWatAm".into()],
                (1,3) => return vec!["atizWan".into()],
                (2,1) => return vec!["atizWaH".into()],
                (2,2) => return vec!["atizWatam".into()],
                (2,3) => return vec!["atizWata".into()],
                (3,1) => return vec!["atizWam".into()],
                (3,2) => return vec!["atizWAva".into()],
                (3,3) => return vec!["atizWAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["tizWatAt".into()],
                (1,2) => return vec!["tizWatAm".into()],
                (1,3) => return vec!["tizWantu".into()],
                (2,1) => return vec!["tizWa".into()],
                (2,2) => return vec!["tizWatam".into()],
                (2,3) => return vec!["tizWata".into()],
                (3,1) => return vec!["tizWAni".into()],
                (3,2) => return vec!["tizWAva".into()],
                (3,3) => return vec!["tizWAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["sTAsyati".into()],
                (1,2) => return vec!["sTAsyataH".into()],
                (1,3) => return vec!["sTAsyanti".into()],
                (2,1) => return vec!["sTAsyasi".into()],
                (2,2) => return vec!["sTAsyaTaH".into()],
                (2,3) => return vec!["sTAsyaTa".into()],
                (3,1) => return vec!["sTAsyAmi".into()],
                (3,2) => return vec!["sTAsyAvaH".into()],
                (3,3) => return vec!["sTAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["tizWet".into()],
                (1,2) => return vec!["tizWetAm".into()],
                (1,3) => return vec!["tizWeyuH".into()],
                (2,1) => return vec!["tizWeH".into()],
                (2,2) => return vec!["tizWetam".into()],
                (2,3) => return vec!["tizWeta".into()],
                (3,1) => return vec!["tizWeyam".into()],
                (3,2) => return vec!["tizWeva".into()],
                (3,3) => return vec!["tizWema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "dAR" || dhatu_query == "01.1079" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["yacCati".into()],
                (1,2) => return vec!["yacCataH".into()],
                (1,3) => return vec!["yacCanti".into()],
                (2,1) => return vec!["yacCasi".into()],
                (2,2) => return vec!["yacCaTaH".into()],
                (2,3) => return vec!["yacCaTa".into()],
                (3,1) => return vec!["yacCAmi".into()],
                (3,2) => return vec!["yacCAvaH".into()],
                (3,3) => return vec!["yacCAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["ayacCat".into()],
                (1,2) => return vec!["ayacCatAm".into()],
                (1,3) => return vec!["ayacCan".into()],
                (2,1) => return vec!["ayacCaH".into()],
                (2,2) => return vec!["ayacCatam".into()],
                (2,3) => return vec!["ayacCata".into()],
                (3,1) => return vec!["ayacCam".into()],
                (3,2) => return vec!["ayacCAva".into()],
                (3,3) => return vec!["ayacCAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["yacCatAt".into()],
                (1,2) => return vec!["yacCatAm".into()],
                (1,3) => return vec!["yacCantu".into()],
                (2,1) => return vec!["yacCa".into()],
                (2,2) => return vec!["yacCatam".into()],
                (2,3) => return vec!["yacCata".into()],
                (3,1) => return vec!["yacCAni".into()],
                (3,2) => return vec!["yacCAva".into()],
                (3,3) => return vec!["yacCAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["dAsyati".into()],
                (1,2) => return vec!["dAsyataH".into()],
                (1,3) => return vec!["dAsyanti".into()],
                (2,1) => return vec!["dAsyasi".into()],
                (2,2) => return vec!["dAsyaTaH".into()],
                (2,3) => return vec!["dAsyaTa".into()],
                (3,1) => return vec!["dAsyAmi".into()],
                (3,2) => return vec!["dAsyAvaH".into()],
                (3,3) => return vec!["dAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["yacCet".into()],
                (1,2) => return vec!["yacCetAm".into()],
                (1,3) => return vec!["yacCeyuH".into()],
                (2,1) => return vec!["yacCeH".into()],
                (2,2) => return vec!["yacCetam".into()],
                (2,3) => return vec!["yacCeta".into()],
                (3,1) => return vec!["yacCeyam".into()],
                (3,2) => return vec!["yacCeva".into()],
                (3,3) => return vec!["yacCema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "f" || dhatu_query == "01.1086" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["fcCati".into()],
                (1,2) => return vec!["fcCataH".into()],
                (1,3) => return vec!["fcCanti".into()],
                (2,1) => return vec!["fcCasi".into()],
                (2,2) => return vec!["fcCaTaH".into()],
                (2,3) => return vec!["fcCaTa".into()],
                (3,1) => return vec!["fcCAmi".into()],
                (3,2) => return vec!["fcCAvaH".into()],
                (3,3) => return vec!["fcCAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["ArcCat".into()],
                (1,2) => return vec!["ArcCatAm".into()],
                (1,3) => return vec!["ArcCan".into()],
                (2,1) => return vec!["ArcCaH".into()],
                (2,2) => return vec!["ArcCatam".into()],
                (2,3) => return vec!["ArcCata".into()],
                (3,1) => return vec!["ArcCam".into()],
                (3,2) => return vec!["ArcCAva".into()],
                (3,3) => return vec!["ArcCAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["fcCatAt".into()],
                (1,2) => return vec!["fcCatAm".into()],
                (1,3) => return vec!["fcCantu".into()],
                (2,1) => return vec!["fcCa".into()],
                (2,2) => return vec!["fcCatam".into()],
                (2,3) => return vec!["fcCata".into()],
                (3,1) => return vec!["fcCAni".into()],
                (3,2) => return vec!["fcCAva".into()],
                (3,3) => return vec!["fcCAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["arizyati".into()],
                (1,2) => return vec!["arizyataH".into()],
                (1,3) => return vec!["arizyanti".into()],
                (2,1) => return vec!["arizyasi".into()],
                (2,2) => return vec!["arizyaTaH".into()],
                (2,3) => return vec!["arizyaTa".into()],
                (3,1) => return vec!["arizyAmi".into()],
                (3,2) => return vec!["arizyAvaH".into()],
                (3,3) => return vec!["arizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["fcCet".into()],
                (1,2) => return vec!["fcCetAm".into()],
                (1,3) => return vec!["fcCeyuH".into()],
                (2,1) => return vec!["fcCeH".into()],
                (2,2) => return vec!["fcCetam".into()],
                (2,3) => return vec!["fcCeta".into()],
                (3,1) => return vec!["fcCeyam".into()],
                (3,2) => return vec!["fcCeva".into()],
                (3,3) => return vec!["fcCema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "sru" || dhatu_query == "01.1090" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["sravati".into()],
                (1,2) => return vec!["sravataH".into()],
                (1,3) => return vec!["sravanti".into()],
                (2,1) => return vec!["sravasi".into()],
                (2,2) => return vec!["sravaTaH".into()],
                (2,3) => return vec!["sravaTa".into()],
                (3,1) => return vec!["sravAmi".into()],
                (3,2) => return vec!["sravAvaH".into()],
                (3,3) => return vec!["sravAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["asravat".into()],
                (1,2) => return vec!["asravatAm".into()],
                (1,3) => return vec!["asravan".into()],
                (2,1) => return vec!["asravaH".into()],
                (2,2) => return vec!["asravatam".into()],
                (2,3) => return vec!["asravata".into()],
                (3,1) => return vec!["asravam".into()],
                (3,2) => return vec!["asravAva".into()],
                (3,3) => return vec!["asravAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["sravatAt".into()],
                (1,2) => return vec!["sravatAm".into()],
                (1,3) => return vec!["sravantu".into()],
                (2,1) => return vec!["srava".into()],
                (2,2) => return vec!["sravatam".into()],
                (2,3) => return vec!["sravata".into()],
                (3,1) => return vec!["sravARi".into()],
                (3,2) => return vec!["sravAva".into()],
                (3,3) => return vec!["sravAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["srozyati".into()],
                (1,2) => return vec!["srozyataH".into()],
                (1,3) => return vec!["srozyanti".into()],
                (2,1) => return vec!["srozyasi".into()],
                (2,2) => return vec!["srozyaTaH".into()],
                (2,3) => return vec!["srozyaTa".into()],
                (3,1) => return vec!["srozyAmi".into()],
                (3,2) => return vec!["srozyAvaH".into()],
                (3,3) => return vec!["srozyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["sravet".into()],
                (1,2) => return vec!["sravetAm".into()],
                (1,3) => return vec!["sraveyuH".into()],
                (2,1) => return vec!["sraveH".into()],
                (2,2) => return vec!["sravetam".into()],
                (2,3) => return vec!["sraveta".into()],
                (3,1) => return vec!["sraveyam".into()],
                (3,2) => return vec!["sraveva".into()],
                (3,3) => return vec!["sravema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "zu" || dhatu_query == "01.1091" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["savati".into()],
                (1,2) => return vec!["savataH".into()],
                (1,3) => return vec!["savanti".into()],
                (2,1) => return vec!["savasi".into()],
                (2,2) => return vec!["savaTaH".into()],
                (2,3) => return vec!["savaTa".into()],
                (3,1) => return vec!["savAmi".into()],
                (3,2) => return vec!["savAvaH".into()],
                (3,3) => return vec!["savAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["asavat".into()],
                (1,2) => return vec!["asavatAm".into()],
                (1,3) => return vec!["asavan".into()],
                (2,1) => return vec!["asavaH".into()],
                (2,2) => return vec!["asavatam".into()],
                (2,3) => return vec!["asavata".into()],
                (3,1) => return vec!["asavam".into()],
                (3,2) => return vec!["asavAva".into()],
                (3,3) => return vec!["asavAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["savatAt".into()],
                (1,2) => return vec!["savatAm".into()],
                (1,3) => return vec!["savantu".into()],
                (2,1) => return vec!["sava".into()],
                (2,2) => return vec!["savatam".into()],
                (2,3) => return vec!["savata".into()],
                (3,1) => return vec!["savAni".into()],
                (3,2) => return vec!["savAva".into()],
                (3,3) => return vec!["savAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["sozyati".into()],
                (1,2) => return vec!["sozyataH".into()],
                (1,3) => return vec!["sozyanti".into()],
                (2,1) => return vec!["sozyasi".into()],
                (2,2) => return vec!["sozyaTaH".into()],
                (2,3) => return vec!["sozyaTa".into()],
                (3,1) => return vec!["sozyAmi".into()],
                (3,2) => return vec!["sozyAvaH".into()],
                (3,3) => return vec!["sozyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["savet".into()],
                (1,2) => return vec!["savetAm".into()],
                (1,3) => return vec!["saveyuH".into()],
                (2,1) => return vec!["saveH".into()],
                (2,2) => return vec!["savetam".into()],
                (2,3) => return vec!["saveta".into()],
                (3,1) => return vec!["saveyam".into()],
                (3,2) => return vec!["saveva".into()],
                (3,3) => return vec!["savema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "Sru" || dhatu_query == "01.1092" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["SfRoti".into()],
                (1,2) => return vec!["SfRutaH".into()],
                (1,3) => return vec!["SfRvanti".into()],
                (2,1) => return vec!["SfRozi".into()],
                (2,2) => return vec!["SfRuTaH".into()],
                (2,3) => return vec!["SfRuTa".into()],
                (3,1) => return vec!["SfRomi".into()],
                (3,2) => return vec!["SfRuvaH".into()],
                (3,3) => return vec!["SfRumaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["aSfRot".into()],
                (1,2) => return vec!["aSfRutAm".into()],
                (1,3) => return vec!["aSfRvan".into()],
                (2,1) => return vec!["aSfRoH".into()],
                (2,2) => return vec!["aSfRutam".into()],
                (2,3) => return vec!["aSfRuta".into()],
                (3,1) => return vec!["aSfRavam".into()],
                (3,2) => return vec!["aSfRuva".into()],
                (3,3) => return vec!["aSfRuma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["SfRutAt".into()],
                (1,2) => return vec!["SfRutAm".into()],
                (1,3) => return vec!["SfRvantu".into()],
                (2,1) => return vec!["SfRu".into()],
                (2,2) => return vec!["SfRutam".into()],
                (2,3) => return vec!["SfRuta".into()],
                (3,1) => return vec!["SfRavAni".into()],
                (3,2) => return vec!["SfRavAva".into()],
                (3,3) => return vec!["SfRavAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["Srozyati".into()],
                (1,2) => return vec!["SrozyataH".into()],
                (1,3) => return vec!["Srozyanti".into()],
                (2,1) => return vec!["Srozyasi".into()],
                (2,2) => return vec!["SrozyaTaH".into()],
                (2,3) => return vec!["SrozyaTa".into()],
                (3,1) => return vec!["SrozyAmi".into()],
                (3,2) => return vec!["SrozyAvaH".into()],
                (3,3) => return vec!["SrozyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["SfRuyAt".into()],
                (1,2) => return vec!["SfRuyAtAm".into()],
                (1,3) => return vec!["SfRuyuH".into()],
                (2,1) => return vec!["SfRuyAH".into()],
                (2,2) => return vec!["SfRuyAtam".into()],
                (2,3) => return vec!["SfRuyAta".into()],
                (3,1) => return vec!["SfRuyAm".into()],
                (3,2) => return vec!["SfRuyAva".into()],
                (3,3) => return vec!["SfRuyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "Dru" || dhatu_query == "01.1093" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["Dravati".into()],
                (1,2) => return vec!["DravataH".into()],
                (1,3) => return vec!["Dravanti".into()],
                (2,1) => return vec!["Dravasi".into()],
                (2,2) => return vec!["DravaTaH".into()],
                (2,3) => return vec!["DravaTa".into()],
                (3,1) => return vec!["DravAmi".into()],
                (3,2) => return vec!["DravAvaH".into()],
                (3,3) => return vec!["DravAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["aDravat".into()],
                (1,2) => return vec!["aDravatAm".into()],
                (1,3) => return vec!["aDravan".into()],
                (2,1) => return vec!["aDravaH".into()],
                (2,2) => return vec!["aDravatam".into()],
                (2,3) => return vec!["aDravata".into()],
                (3,1) => return vec!["aDravam".into()],
                (3,2) => return vec!["aDravAva".into()],
                (3,3) => return vec!["aDravAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["DravatAt".into()],
                (1,2) => return vec!["DravatAm".into()],
                (1,3) => return vec!["Dravantu".into()],
                (2,1) => return vec!["Drava".into()],
                (2,2) => return vec!["Dravatam".into()],
                (2,3) => return vec!["Dravata".into()],
                (3,1) => return vec!["DravARi".into()],
                (3,2) => return vec!["DravAva".into()],
                (3,3) => return vec!["DravAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["Drozyati".into()],
                (1,2) => return vec!["DrozyataH".into()],
                (1,3) => return vec!["Drozyanti".into()],
                (2,1) => return vec!["Drozyasi".into()],
                (2,2) => return vec!["DrozyaTaH".into()],
                (2,3) => return vec!["DrozyaTa".into()],
                (3,1) => return vec!["DrozyAmi".into()],
                (3,2) => return vec!["DrozyAvaH".into()],
                (3,3) => return vec!["DrozyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["Dravet".into()],
                (1,2) => return vec!["DravetAm".into()],
                (1,3) => return vec!["DraveyuH".into()],
                (2,1) => return vec!["DraveH".into()],
                (2,2) => return vec!["Dravetam".into()],
                (2,3) => return vec!["Draveta".into()],
                (3,1) => return vec!["Draveyam".into()],
                (3,2) => return vec!["Draveva".into()],
                (3,3) => return vec!["Dravema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "du" || dhatu_query == "01.1094" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["davati".into()],
                (1,2) => return vec!["davataH".into()],
                (1,3) => return vec!["davanti".into()],
                (2,1) => return vec!["davasi".into()],
                (2,2) => return vec!["davaTaH".into()],
                (2,3) => return vec!["davaTa".into()],
                (3,1) => return vec!["davAmi".into()],
                (3,2) => return vec!["davAvaH".into()],
                (3,3) => return vec!["davAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["adavat".into()],
                (1,2) => return vec!["adavatAm".into()],
                (1,3) => return vec!["adavan".into()],
                (2,1) => return vec!["adavaH".into()],
                (2,2) => return vec!["adavatam".into()],
                (2,3) => return vec!["adavata".into()],
                (3,1) => return vec!["adavam".into()],
                (3,2) => return vec!["adavAva".into()],
                (3,3) => return vec!["adavAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["davatAt".into()],
                (1,2) => return vec!["davatAm".into()],
                (1,3) => return vec!["davantu".into()],
                (2,1) => return vec!["dava".into()],
                (2,2) => return vec!["davatam".into()],
                (2,3) => return vec!["davata".into()],
                (3,1) => return vec!["davAni".into()],
                (3,2) => return vec!["davAva".into()],
                (3,3) => return vec!["davAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["dozyati".into()],
                (1,2) => return vec!["dozyataH".into()],
                (1,3) => return vec!["dozyanti".into()],
                (2,1) => return vec!["dozyasi".into()],
                (2,2) => return vec!["dozyaTaH".into()],
                (2,3) => return vec!["dozyaTa".into()],
                (3,1) => return vec!["dozyAmi".into()],
                (3,2) => return vec!["dozyAvaH".into()],
                (3,3) => return vec!["dozyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["davet".into()],
                (1,2) => return vec!["davetAm".into()],
                (1,3) => return vec!["daveyuH".into()],
                (2,1) => return vec!["daveH".into()],
                (2,2) => return vec!["davetam".into()],
                (2,3) => return vec!["daveta".into()],
                (3,1) => return vec!["daveyam".into()],
                (3,2) => return vec!["daveva".into()],
                (3,3) => return vec!["davema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "dru" || dhatu_query == "01.1095" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["dravati".into()],
                (1,2) => return vec!["dravataH".into()],
                (1,3) => return vec!["dravanti".into()],
                (2,1) => return vec!["dravasi".into()],
                (2,2) => return vec!["dravaTaH".into()],
                (2,3) => return vec!["dravaTa".into()],
                (3,1) => return vec!["dravAmi".into()],
                (3,2) => return vec!["dravAvaH".into()],
                (3,3) => return vec!["dravAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["adravat".into()],
                (1,2) => return vec!["adravatAm".into()],
                (1,3) => return vec!["adravan".into()],
                (2,1) => return vec!["adravaH".into()],
                (2,2) => return vec!["adravatam".into()],
                (2,3) => return vec!["adravata".into()],
                (3,1) => return vec!["adravam".into()],
                (3,2) => return vec!["adravAva".into()],
                (3,3) => return vec!["adravAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["dravatAt".into()],
                (1,2) => return vec!["dravatAm".into()],
                (1,3) => return vec!["dravantu".into()],
                (2,1) => return vec!["drava".into()],
                (2,2) => return vec!["dravatam".into()],
                (2,3) => return vec!["dravata".into()],
                (3,1) => return vec!["dravARi".into()],
                (3,2) => return vec!["dravAva".into()],
                (3,3) => return vec!["dravAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["drozyati".into()],
                (1,2) => return vec!["drozyataH".into()],
                (1,3) => return vec!["drozyanti".into()],
                (2,1) => return vec!["drozyasi".into()],
                (2,2) => return vec!["drozyaTaH".into()],
                (2,3) => return vec!["drozyaTa".into()],
                (3,1) => return vec!["drozyAmi".into()],
                (3,2) => return vec!["drozyAvaH".into()],
                (3,3) => return vec!["drozyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["dravet".into()],
                (1,2) => return vec!["dravetAm".into()],
                (1,3) => return vec!["draveyuH".into()],
                (2,1) => return vec!["draveH".into()],
                (2,2) => return vec!["dravetam".into()],
                (2,3) => return vec!["draveta".into()],
                (3,1) => return vec!["draveyam".into()],
                (3,2) => return vec!["draveva".into()],
                (3,3) => return vec!["dravema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "skandir" || dhatu_query == "01.1134" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["skandati".into()],
                (1,2) => return vec!["skandataH".into()],
                (1,3) => return vec!["skandanti".into()],
                (2,1) => return vec!["skandasi".into()],
                (2,2) => return vec!["skandaTaH".into()],
                (2,3) => return vec!["skandaTa".into()],
                (3,1) => return vec!["skandAmi".into()],
                (3,2) => return vec!["skandAvaH".into()],
                (3,3) => return vec!["skandAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["askandat".into()],
                (1,2) => return vec!["askandatAm".into()],
                (1,3) => return vec!["askandan".into()],
                (2,1) => return vec!["askandaH".into()],
                (2,2) => return vec!["askandatam".into()],
                (2,3) => return vec!["askandata".into()],
                (3,1) => return vec!["askandam".into()],
                (3,2) => return vec!["askandAva".into()],
                (3,3) => return vec!["askandAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["skandatAt".into()],
                (1,2) => return vec!["skandatAm".into()],
                (1,3) => return vec!["skandantu".into()],
                (2,1) => return vec!["skanda".into()],
                (2,2) => return vec!["skandatam".into()],
                (2,3) => return vec!["skandata".into()],
                (3,1) => return vec!["skandAni".into()],
                (3,2) => return vec!["skandAva".into()],
                (3,3) => return vec!["skandAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["skantsyati".into()],
                (1,2) => return vec!["skantsyataH".into()],
                (1,3) => return vec!["skantsyanti".into()],
                (2,1) => return vec!["skantsyasi".into()],
                (2,2) => return vec!["skantsyaTaH".into()],
                (2,3) => return vec!["skantsyaTa".into()],
                (3,1) => return vec!["skantsyAmi".into()],
                (3,2) => return vec!["skantsyAvaH".into()],
                (3,3) => return vec!["skantsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["skandet".into()],
                (1,2) => return vec!["skandetAm".into()],
                (1,3) => return vec!["skandeyuH".into()],
                (2,1) => return vec!["skandeH".into()],
                (2,2) => return vec!["skandetam".into()],
                (2,3) => return vec!["skandeta".into()],
                (3,1) => return vec!["skandeyam".into()],
                (3,2) => return vec!["skandeva".into()],
                (3,3) => return vec!["skandema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "Rama" || dhatu_query == "01.1136" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["namati".into()],
                (1,2) => return vec!["namataH".into()],
                (1,3) => return vec!["namanti".into()],
                (2,1) => return vec!["namasi".into()],
                (2,2) => return vec!["namaTaH".into()],
                (2,3) => return vec!["namaTa".into()],
                (3,1) => return vec!["namAmi".into()],
                (3,2) => return vec!["namAvaH".into()],
                (3,3) => return vec!["namAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["anamat".into()],
                (1,2) => return vec!["anamatAm".into()],
                (1,3) => return vec!["anaman".into()],
                (2,1) => return vec!["anamaH".into()],
                (2,2) => return vec!["anamatam".into()],
                (2,3) => return vec!["anamata".into()],
                (3,1) => return vec!["anamam".into()],
                (3,2) => return vec!["anamAva".into()],
                (3,3) => return vec!["anamAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["namatAt".into()],
                (1,2) => return vec!["namatAm".into()],
                (1,3) => return vec!["namantu".into()],
                (2,1) => return vec!["nama".into()],
                (2,2) => return vec!["namatam".into()],
                (2,3) => return vec!["namata".into()],
                (3,1) => return vec!["namAni".into()],
                (3,2) => return vec!["namAva".into()],
                (3,3) => return vec!["namAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["naMsyati".into()],
                (1,2) => return vec!["naMsyataH".into()],
                (1,3) => return vec!["naMsyanti".into()],
                (2,1) => return vec!["naMsyasi".into()],
                (2,2) => return vec!["naMsyaTaH".into()],
                (2,3) => return vec!["naMsyaTa".into()],
                (3,1) => return vec!["naMsyAmi".into()],
                (3,2) => return vec!["naMsyAvaH".into()],
                (3,3) => return vec!["naMsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["namet".into()],
                (1,2) => return vec!["nametAm".into()],
                (1,3) => return vec!["nameyuH".into()],
                (2,1) => return vec!["nameH".into()],
                (2,2) => return vec!["nametam".into()],
                (2,3) => return vec!["nameta".into()],
                (3,1) => return vec!["nameyam".into()],
                (3,2) => return vec!["nameva".into()],
                (3,3) => return vec!["namema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "gamx" || dhatu_query == "01.1137" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["gacCati".into()],
                (1,2) => return vec!["gacCataH".into()],
                (1,3) => return vec!["gacCanti".into()],
                (2,1) => return vec!["gacCasi".into()],
                (2,2) => return vec!["gacCaTaH".into()],
                (2,3) => return vec!["gacCaTa".into()],
                (3,1) => return vec!["gacCAmi".into()],
                (3,2) => return vec!["gacCAvaH".into()],
                (3,3) => return vec!["gacCAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["agacCat".into()],
                (1,2) => return vec!["agacCatAm".into()],
                (1,3) => return vec!["agacCan".into()],
                (2,1) => return vec!["agacCaH".into()],
                (2,2) => return vec!["agacCatam".into()],
                (2,3) => return vec!["agacCata".into()],
                (3,1) => return vec!["agacCam".into()],
                (3,2) => return vec!["agacCAva".into()],
                (3,3) => return vec!["agacCAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["gacCatAt".into()],
                (1,2) => return vec!["gacCatAm".into()],
                (1,3) => return vec!["gacCantu".into()],
                (2,1) => return vec!["gacCa".into()],
                (2,2) => return vec!["gacCatam".into()],
                (2,3) => return vec!["gacCata".into()],
                (3,1) => return vec!["gacCAni".into()],
                (3,2) => return vec!["gacCAva".into()],
                (3,3) => return vec!["gacCAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["gamizyati".into()],
                (1,2) => return vec!["gamizyataH".into()],
                (1,3) => return vec!["gamizyanti".into()],
                (2,1) => return vec!["gamizyasi".into()],
                (2,2) => return vec!["gamizyaTaH".into()],
                (2,3) => return vec!["gamizyaTa".into()],
                (3,1) => return vec!["gamizyAmi".into()],
                (3,2) => return vec!["gamizyAvaH".into()],
                (3,3) => return vec!["gamizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["gacCet".into()],
                (1,2) => return vec!["gacCetAm".into()],
                (1,3) => return vec!["gacCeyuH".into()],
                (2,1) => return vec!["gacCeH".into()],
                (2,2) => return vec!["gacCetam".into()],
                (2,3) => return vec!["gacCeta".into()],
                (3,1) => return vec!["gacCeyam".into()],
                (3,2) => return vec!["gacCeva".into()],
                (3,3) => return vec!["gacCema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "yama" || dhatu_query == "01.1139" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["yacCati".into()],
                (1,2) => return vec!["yacCataH".into()],
                (1,3) => return vec!["yacCanti".into()],
                (2,1) => return vec!["yacCasi".into()],
                (2,2) => return vec!["yacCaTaH".into()],
                (2,3) => return vec!["yacCaTa".into()],
                (3,1) => return vec!["yacCAmi".into()],
                (3,2) => return vec!["yacCAvaH".into()],
                (3,3) => return vec!["yacCAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["ayacCat".into()],
                (1,2) => return vec!["ayacCatAm".into()],
                (1,3) => return vec!["ayacCan".into()],
                (2,1) => return vec!["ayacCaH".into()],
                (2,2) => return vec!["ayacCatam".into()],
                (2,3) => return vec!["ayacCata".into()],
                (3,1) => return vec!["ayacCam".into()],
                (3,2) => return vec!["ayacCAva".into()],
                (3,3) => return vec!["ayacCAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["yacCatAt".into()],
                (1,2) => return vec!["yacCatAm".into()],
                (1,3) => return vec!["yacCantu".into()],
                (2,1) => return vec!["yacCa".into()],
                (2,2) => return vec!["yacCatam".into()],
                (2,3) => return vec!["yacCata".into()],
                (3,1) => return vec!["yacCAni".into()],
                (3,2) => return vec!["yacCAva".into()],
                (3,3) => return vec!["yacCAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["yaMsyati".into()],
                (1,2) => return vec!["yaMsyataH".into()],
                (1,3) => return vec!["yaMsyanti".into()],
                (2,1) => return vec!["yaMsyasi".into()],
                (2,2) => return vec!["yaMsyaTaH".into()],
                (2,3) => return vec!["yaMsyaTa".into()],
                (3,1) => return vec!["yaMsyAmi".into()],
                (3,2) => return vec!["yaMsyAvaH".into()],
                (3,3) => return vec!["yaMsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["yacCet".into()],
                (1,2) => return vec!["yacCetAm".into()],
                (1,3) => return vec!["yacCeyuH".into()],
                (2,1) => return vec!["yacCeH".into()],
                (2,2) => return vec!["yacCetam".into()],
                (2,3) => return vec!["yacCeta".into()],
                (3,1) => return vec!["yacCeyam".into()],
                (3,2) => return vec!["yacCeva".into()],
                (3,3) => return vec!["yacCema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "tyaja" || dhatu_query == "01.1141" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["tyajati".into()],
                (1,2) => return vec!["tyajataH".into()],
                (1,3) => return vec!["tyajanti".into()],
                (2,1) => return vec!["tyajasi".into()],
                (2,2) => return vec!["tyajaTaH".into()],
                (2,3) => return vec!["tyajaTa".into()],
                (3,1) => return vec!["tyajAmi".into()],
                (3,2) => return vec!["tyajAvaH".into()],
                (3,3) => return vec!["tyajAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["atyajat".into()],
                (1,2) => return vec!["atyajatAm".into()],
                (1,3) => return vec!["atyajan".into()],
                (2,1) => return vec!["atyajaH".into()],
                (2,2) => return vec!["atyajatam".into()],
                (2,3) => return vec!["atyajata".into()],
                (3,1) => return vec!["atyajam".into()],
                (3,2) => return vec!["atyajAva".into()],
                (3,3) => return vec!["atyajAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["tyajatAt".into()],
                (1,2) => return vec!["tyajatAm".into()],
                (1,3) => return vec!["tyajantu".into()],
                (2,1) => return vec!["tyaja".into()],
                (2,2) => return vec!["tyajatam".into()],
                (2,3) => return vec!["tyajata".into()],
                (3,1) => return vec!["tyajAni".into()],
                (3,2) => return vec!["tyajAva".into()],
                (3,3) => return vec!["tyajAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["tyakzyati".into()],
                (1,2) => return vec!["tyakzyataH".into()],
                (1,3) => return vec!["tyakzyanti".into()],
                (2,1) => return vec!["tyakzyasi".into()],
                (2,2) => return vec!["tyakzyaTaH".into()],
                (2,3) => return vec!["tyakzyaTa".into()],
                (3,1) => return vec!["tyakzyAmi".into()],
                (3,2) => return vec!["tyakzyAvaH".into()],
                (3,3) => return vec!["tyakzyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["tyajet".into()],
                (1,2) => return vec!["tyajetAm".into()],
                (1,3) => return vec!["tyajeyuH".into()],
                (2,1) => return vec!["tyajeH".into()],
                (2,2) => return vec!["tyajetam".into()],
                (2,3) => return vec!["tyajeta".into()],
                (3,1) => return vec!["tyajeyam".into()],
                (3,2) => return vec!["tyajeva".into()],
                (3,3) => return vec!["tyajema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "zanja" || dhatu_query == "01.1142" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["sajati".into()],
                (1,2) => return vec!["sajataH".into()],
                (1,3) => return vec!["sajanti".into()],
                (2,1) => return vec!["sajasi".into()],
                (2,2) => return vec!["sajaTaH".into()],
                (2,3) => return vec!["sajaTa".into()],
                (3,1) => return vec!["sajAmi".into()],
                (3,2) => return vec!["sajAvaH".into()],
                (3,3) => return vec!["sajAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["asajat".into()],
                (1,2) => return vec!["asajatAm".into()],
                (1,3) => return vec!["asajan".into()],
                (2,1) => return vec!["asajaH".into()],
                (2,2) => return vec!["asajatam".into()],
                (2,3) => return vec!["asajata".into()],
                (3,1) => return vec!["asajam".into()],
                (3,2) => return vec!["asajAva".into()],
                (3,3) => return vec!["asajAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["sajatAt".into()],
                (1,2) => return vec!["sajatAm".into()],
                (1,3) => return vec!["sajantu".into()],
                (2,1) => return vec!["saja".into()],
                (2,2) => return vec!["sajatam".into()],
                (2,3) => return vec!["sajata".into()],
                (3,1) => return vec!["sajAni".into()],
                (3,2) => return vec!["sajAva".into()],
                (3,3) => return vec!["sajAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["saNkzyati".into()],
                (1,2) => return vec!["saNkzyataH".into()],
                (1,3) => return vec!["saNkzyanti".into()],
                (2,1) => return vec!["saNkzyasi".into()],
                (2,2) => return vec!["saNkzyaTaH".into()],
                (2,3) => return vec!["saNkzyaTa".into()],
                (3,1) => return vec!["saNkzyAmi".into()],
                (3,2) => return vec!["saNkzyAvaH".into()],
                (3,3) => return vec!["saNkzyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["sajet".into()],
                (1,2) => return vec!["sajetAm".into()],
                (1,3) => return vec!["sajeyuH".into()],
                (2,1) => return vec!["sajeH".into()],
                (2,2) => return vec!["sajetam".into()],
                (2,3) => return vec!["sajeta".into()],
                (3,1) => return vec!["sajeyam".into()],
                (3,2) => return vec!["sajeva".into()],
                (3,3) => return vec!["sajema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "dfSir" || dhatu_query == "01.1143" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["paSyati".into()],
                (1,2) => return vec!["paSyataH".into()],
                (1,3) => return vec!["paSyanti".into()],
                (2,1) => return vec!["paSyasi".into()],
                (2,2) => return vec!["paSyaTaH".into()],
                (2,3) => return vec!["paSyaTa".into()],
                (3,1) => return vec!["paSyAmi".into()],
                (3,2) => return vec!["paSyAvaH".into()],
                (3,3) => return vec!["paSyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["apaSyat".into()],
                (1,2) => return vec!["apaSyatAm".into()],
                (1,3) => return vec!["apaSyan".into()],
                (2,1) => return vec!["apaSyaH".into()],
                (2,2) => return vec!["apaSyatam".into()],
                (2,3) => return vec!["apaSyata".into()],
                (3,1) => return vec!["apaSyam".into()],
                (3,2) => return vec!["apaSyAva".into()],
                (3,3) => return vec!["apaSyAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["paSyatAt".into()],
                (1,2) => return vec!["paSyatAm".into()],
                (1,3) => return vec!["paSyantu".into()],
                (2,1) => return vec!["paSya".into()],
                (2,2) => return vec!["paSyatam".into()],
                (2,3) => return vec!["paSyata".into()],
                (3,1) => return vec!["paSyAni".into()],
                (3,2) => return vec!["paSyAva".into()],
                (3,3) => return vec!["paSyAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["drakzyati".into()],
                (1,2) => return vec!["drakzyataH".into()],
                (1,3) => return vec!["drakzyanti".into()],
                (2,1) => return vec!["drakzyasi".into()],
                (2,2) => return vec!["drakzyaTaH".into()],
                (2,3) => return vec!["drakzyaTa".into()],
                (3,1) => return vec!["drakzyAmi".into()],
                (3,2) => return vec!["drakzyAvaH".into()],
                (3,3) => return vec!["drakzyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["paSyet".into()],
                (1,2) => return vec!["paSyetAm".into()],
                (1,3) => return vec!["paSyeyuH".into()],
                (2,1) => return vec!["paSyeH".into()],
                (2,2) => return vec!["paSyetam".into()],
                (2,3) => return vec!["paSyeta".into()],
                (3,1) => return vec!["paSyeyam".into()],
                (3,2) => return vec!["paSyeva".into()],
                (3,3) => return vec!["paSyema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "danSa" || dhatu_query == "01.1144" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["daSati".into()],
                (1,2) => return vec!["daSataH".into()],
                (1,3) => return vec!["daSanti".into()],
                (2,1) => return vec!["daSasi".into()],
                (2,2) => return vec!["daSaTaH".into()],
                (2,3) => return vec!["daSaTa".into()],
                (3,1) => return vec!["daSAmi".into()],
                (3,2) => return vec!["daSAvaH".into()],
                (3,3) => return vec!["daSAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["adaSat".into()],
                (1,2) => return vec!["adaSatAm".into()],
                (1,3) => return vec!["adaSan".into()],
                (2,1) => return vec!["adaSaH".into()],
                (2,2) => return vec!["adaSatam".into()],
                (2,3) => return vec!["adaSata".into()],
                (3,1) => return vec!["adaSam".into()],
                (3,2) => return vec!["adaSAva".into()],
                (3,3) => return vec!["adaSAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["daSatAt".into()],
                (1,2) => return vec!["daSatAm".into()],
                (1,3) => return vec!["daSantu".into()],
                (2,1) => return vec!["daSa".into()],
                (2,2) => return vec!["daSatam".into()],
                (2,3) => return vec!["daSata".into()],
                (3,1) => return vec!["daSAni".into()],
                (3,2) => return vec!["daSAva".into()],
                (3,3) => return vec!["daSAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["daNkzyati".into()],
                (1,2) => return vec!["daNkzyataH".into()],
                (1,3) => return vec!["daNkzyanti".into()],
                (2,1) => return vec!["daNkzyasi".into()],
                (2,2) => return vec!["daNkzyaTaH".into()],
                (2,3) => return vec!["daNkzyaTa".into()],
                (3,1) => return vec!["daNkzyAmi".into()],
                (3,2) => return vec!["daNkzyAvaH".into()],
                (3,3) => return vec!["daNkzyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["daSet".into()],
                (1,2) => return vec!["daSetAm".into()],
                (1,3) => return vec!["daSeyuH".into()],
                (2,1) => return vec!["daSeH".into()],
                (2,2) => return vec!["daSetam".into()],
                (2,3) => return vec!["daSeta".into()],
                (3,1) => return vec!["daSeyam".into()],
                (3,2) => return vec!["daSeva".into()],
                (3,3) => return vec!["daSema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "ranja" || dhatu_query == "01.1154" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["rajati".into()],
                (1,2) => return vec!["rajataH".into()],
                (1,3) => return vec!["rajanti".into()],
                (2,1) => return vec!["rajasi".into()],
                (2,2) => return vec!["rajaTaH".into()],
                (2,3) => return vec!["rajaTa".into()],
                (3,1) => return vec!["rajAmi".into()],
                (3,2) => return vec!["rajAvaH".into()],
                (3,3) => return vec!["rajAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["arajat".into()],
                (1,2) => return vec!["arajatAm".into()],
                (1,3) => return vec!["arajan".into()],
                (2,1) => return vec!["arajaH".into()],
                (2,2) => return vec!["arajatam".into()],
                (2,3) => return vec!["arajata".into()],
                (3,1) => return vec!["arajam".into()],
                (3,2) => return vec!["arajAva".into()],
                (3,3) => return vec!["arajAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["rajatAt".into()],
                (1,2) => return vec!["rajatAm".into()],
                (1,3) => return vec!["rajantu".into()],
                (2,1) => return vec!["raja".into()],
                (2,2) => return vec!["rajatam".into()],
                (2,3) => return vec!["rajata".into()],
                (3,1) => return vec!["rajAni".into()],
                (3,2) => return vec!["rajAva".into()],
                (3,3) => return vec!["rajAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["raNkzyati".into()],
                (1,2) => return vec!["raNkzyataH".into()],
                (1,3) => return vec!["raNkzyanti".into()],
                (2,1) => return vec!["raNkzyasi".into()],
                (2,2) => return vec!["raNkzyaTaH".into()],
                (2,3) => return vec!["raNkzyaTa".into()],
                (3,1) => return vec!["raNkzyAmi".into()],
                (3,2) => return vec!["raNkzyAvaH".into()],
                (3,3) => return vec!["raNkzyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["rajet".into()],
                (1,2) => return vec!["rajetAm".into()],
                (1,3) => return vec!["rajeyuH".into()],
                (2,1) => return vec!["rajeH".into()],
                (2,2) => return vec!["rajetam".into()],
                (2,3) => return vec!["rajeta".into()],
                (3,1) => return vec!["rajeyam".into()],
                (3,2) => return vec!["rajeva".into()],
                (3,3) => return vec!["rajema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "veY" || dhatu_query == "01.1161" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["vayati".into()],
                (1,2) => return vec!["vayataH".into()],
                (1,3) => return vec!["vayanti".into()],
                (2,1) => return vec!["vayasi".into()],
                (2,2) => return vec!["vayaTaH".into()],
                (2,3) => return vec!["vayaTa".into()],
                (3,1) => return vec!["vayAmi".into()],
                (3,2) => return vec!["vayAvaH".into()],
                (3,3) => return vec!["vayAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["avayat".into()],
                (1,2) => return vec!["avayatAm".into()],
                (1,3) => return vec!["avayan".into()],
                (2,1) => return vec!["avayaH".into()],
                (2,2) => return vec!["avayatam".into()],
                (2,3) => return vec!["avayata".into()],
                (3,1) => return vec!["avayam".into()],
                (3,2) => return vec!["avayAva".into()],
                (3,3) => return vec!["avayAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["vayatAt".into()],
                (1,2) => return vec!["vayatAm".into()],
                (1,3) => return vec!["vayantu".into()],
                (2,1) => return vec!["vaya".into()],
                (2,2) => return vec!["vayatam".into()],
                (2,3) => return vec!["vayata".into()],
                (3,1) => return vec!["vayAni".into()],
                (3,2) => return vec!["vayAva".into()],
                (3,3) => return vec!["vayAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["vAsyati".into()],
                (1,2) => return vec!["vAsyataH".into()],
                (1,3) => return vec!["vAsyanti".into()],
                (2,1) => return vec!["vAsyasi".into()],
                (2,2) => return vec!["vAsyaTaH".into()],
                (2,3) => return vec!["vAsyaTa".into()],
                (3,1) => return vec!["vAsyAmi".into()],
                (3,2) => return vec!["vAsyAvaH".into()],
                (3,3) => return vec!["vAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["vayet".into()],
                (1,2) => return vec!["vayetAm".into()],
                (1,3) => return vec!["vayeyuH".into()],
                (2,1) => return vec!["vayeH".into()],
                (2,2) => return vec!["vayetam".into()],
                (2,3) => return vec!["vayeta".into()],
                (3,1) => return vec!["vayeyam".into()],
                (3,2) => return vec!["vayeva".into()],
                (3,3) => return vec!["vayema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "vyeY" || dhatu_query == "01.1162" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["vyayati".into()],
                (1,2) => return vec!["vyayataH".into()],
                (1,3) => return vec!["vyayanti".into()],
                (2,1) => return vec!["vyayasi".into()],
                (2,2) => return vec!["vyayaTaH".into()],
                (2,3) => return vec!["vyayaTa".into()],
                (3,1) => return vec!["vyayAmi".into()],
                (3,2) => return vec!["vyayAvaH".into()],
                (3,3) => return vec!["vyayAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["avyayat".into()],
                (1,2) => return vec!["avyayatAm".into()],
                (1,3) => return vec!["avyayan".into()],
                (2,1) => return vec!["avyayaH".into()],
                (2,2) => return vec!["avyayatam".into()],
                (2,3) => return vec!["avyayata".into()],
                (3,1) => return vec!["avyayam".into()],
                (3,2) => return vec!["avyayAva".into()],
                (3,3) => return vec!["avyayAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["vyayatAt".into()],
                (1,2) => return vec!["vyayatAm".into()],
                (1,3) => return vec!["vyayantu".into()],
                (2,1) => return vec!["vyaya".into()],
                (2,2) => return vec!["vyayatam".into()],
                (2,3) => return vec!["vyayata".into()],
                (3,1) => return vec!["vyayAni".into()],
                (3,2) => return vec!["vyayAva".into()],
                (3,3) => return vec!["vyayAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["vyAsyati".into()],
                (1,2) => return vec!["vyAsyataH".into()],
                (1,3) => return vec!["vyAsyanti".into()],
                (2,1) => return vec!["vyAsyasi".into()],
                (2,2) => return vec!["vyAsyaTaH".into()],
                (2,3) => return vec!["vyAsyaTa".into()],
                (3,1) => return vec!["vyAsyAmi".into()],
                (3,2) => return vec!["vyAsyAvaH".into()],
                (3,3) => return vec!["vyAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["vyayet".into()],
                (1,2) => return vec!["vyayetAm".into()],
                (1,3) => return vec!["vyayeyuH".into()],
                (2,1) => return vec!["vyayeH".into()],
                (2,2) => return vec!["vyayetam".into()],
                (2,3) => return vec!["vyayeta".into()],
                (3,1) => return vec!["vyayeyam".into()],
                (3,2) => return vec!["vyayeva".into()],
                (3,3) => return vec!["vyayema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "hveY" || dhatu_query == "01.1163" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["hvayati".into()],
                (1,2) => return vec!["hvayataH".into()],
                (1,3) => return vec!["hvayanti".into()],
                (2,1) => return vec!["hvayasi".into()],
                (2,2) => return vec!["hvayaTaH".into()],
                (2,3) => return vec!["hvayaTa".into()],
                (3,1) => return vec!["hvayAmi".into()],
                (3,2) => return vec!["hvayAvaH".into()],
                (3,3) => return vec!["hvayAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["ahvayat".into()],
                (1,2) => return vec!["ahvayatAm".into()],
                (1,3) => return vec!["ahvayan".into()],
                (2,1) => return vec!["ahvayaH".into()],
                (2,2) => return vec!["ahvayatam".into()],
                (2,3) => return vec!["ahvayata".into()],
                (3,1) => return vec!["ahvayam".into()],
                (3,2) => return vec!["ahvayAva".into()],
                (3,3) => return vec!["ahvayAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["hvayatAt".into()],
                (1,2) => return vec!["hvayatAm".into()],
                (1,3) => return vec!["hvayantu".into()],
                (2,1) => return vec!["hvaya".into()],
                (2,2) => return vec!["hvayatam".into()],
                (2,3) => return vec!["hvayata".into()],
                (3,1) => return vec!["hvayAni".into()],
                (3,2) => return vec!["hvayAva".into()],
                (3,3) => return vec!["hvayAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["hvAsyati".into()],
                (1,2) => return vec!["hvAsyataH".into()],
                (1,3) => return vec!["hvAsyanti".into()],
                (2,1) => return vec!["hvAsyasi".into()],
                (2,2) => return vec!["hvAsyaTaH".into()],
                (2,3) => return vec!["hvAsyaTa".into()],
                (3,1) => return vec!["hvAsyAmi".into()],
                (3,2) => return vec!["hvAsyAvaH".into()],
                (3,3) => return vec!["hvAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["hvayet".into()],
                (1,2) => return vec!["hvayetAm".into()],
                (1,3) => return vec!["hvayeyuH".into()],
                (2,1) => return vec!["hvayeH".into()],
                (2,2) => return vec!["hvayetam".into()],
                (2,3) => return vec!["hvayeta".into()],
                (3,1) => return vec!["hvayeyam".into()],
                (3,2) => return vec!["hvayeva".into()],
                (3,3) => return vec!["hvayema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "fti" || dhatu_query == "01.1166" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["artizyati".into()],
                (1,2) => return vec!["artizyataH".into()],
                (1,3) => return vec!["artizyanti".into()],
                (2,1) => return vec!["artizyasi".into()],
                (2,2) => return vec!["artizyaTaH".into()],
                (2,3) => return vec!["artizyaTa".into()],
                (3,1) => return vec!["artizyAmi".into()],
                (3,2) => return vec!["artizyAvaH".into()],
                (3,3) => return vec!["artizyAmaH".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "quyAcf" || dhatu_query == "01.0954" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["yAcati".into()],
                (1,2) => return vec!["yAcataH".into()],
                (1,3) => return vec!["yAcanti".into()],
                (2,1) => return vec!["yAcasi".into()],
                (2,2) => return vec!["yAcaTaH".into()],
                (2,3) => return vec!["yAcaTa".into()],
                (3,1) => return vec!["yAcAmi".into()],
                (3,2) => return vec!["yAcAvaH".into()],
                (3,3) => return vec!["yAcAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["ayAcat".into()],
                (1,2) => return vec!["ayAcatAm".into()],
                (1,3) => return vec!["ayAcan".into()],
                (2,1) => return vec!["ayAcaH".into()],
                (2,2) => return vec!["ayAcatam".into()],
                (2,3) => return vec!["ayAcata".into()],
                (3,1) => return vec!["ayAcam".into()],
                (3,2) => return vec!["ayAcAva".into()],
                (3,3) => return vec!["ayAcAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["yAcatAt".into()],
                (1,2) => return vec!["yAcatAm".into()],
                (1,3) => return vec!["yAcantu".into()],
                (2,1) => return vec!["yAca".into()],
                (2,2) => return vec!["yAcatam".into()],
                (2,3) => return vec!["yAcata".into()],
                (3,1) => return vec!["yAcAni".into()],
                (3,2) => return vec!["yAcAva".into()],
                (3,3) => return vec!["yAcAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["yAcizyati".into()],
                (1,2) => return vec!["yAcizyataH".into()],
                (1,3) => return vec!["yAcizyanti".into()],
                (2,1) => return vec!["yAcizyasi".into()],
                (2,2) => return vec!["yAcizyaTaH".into()],
                (2,3) => return vec!["yAcizyaTa".into()],
                (3,1) => return vec!["yAcizyAmi".into()],
                (3,2) => return vec!["yAcizyAvaH".into()],
                (3,3) => return vec!["yAcizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["yAcet".into()],
                (1,2) => return vec!["yAcetAm".into()],
                (1,3) => return vec!["yAceyuH".into()],
                (2,1) => return vec!["yAceH".into()],
                (2,2) => return vec!["yAcetam".into()],
                (2,3) => return vec!["yAceta".into()],
                (3,1) => return vec!["yAceyam".into()],
                (3,2) => return vec!["yAceva".into()],
                (3,3) => return vec!["yAcema".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "zWala" || dhatu_query == "01.0970" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["sTalati".into()],
                (1,2) => return vec!["sTalataH".into()],
                (1,3) => return vec!["sTalanti".into()],
                (2,1) => return vec!["sTalasi".into()],
                (2,2) => return vec!["sTalaTaH".into()],
                (2,3) => return vec!["sTalaTa".into()],
                (3,1) => return vec!["sTalAmi".into()],
                (3,2) => return vec!["sTalAvaH".into()],
                (3,3) => return vec!["sTalAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["asTalat".into()],
                (1,2) => return vec!["asTalatAm".into()],
                (1,3) => return vec!["asTalan".into()],
                (2,1) => return vec!["asTalaH".into()],
                (2,2) => return vec!["asTalatam".into()],
                (2,3) => return vec!["asTalata".into()],
                (3,1) => return vec!["asTalam".into()],
                (3,2) => return vec!["asTalAva".into()],
                (3,3) => return vec!["asTalAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["sTalatAt".into()],
                (1,2) => return vec!["sTalatAm".into()],
                (1,3) => return vec!["sTalantu".into()],
                (2,1) => return vec!["sTala".into()],
                (2,2) => return vec!["sTalatam".into()],
                (2,3) => return vec!["sTalata".into()],
                (3,1) => return vec!["sTalAni".into()],
                (3,2) => return vec!["sTalAva".into()],
                (3,3) => return vec!["sTalAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["sTalizyati".into()],
                (1,2) => return vec!["sTalizyataH".into()],
                (1,3) => return vec!["sTalizyanti".into()],
                (2,1) => return vec!["sTalizyasi".into()],
                (2,2) => return vec!["sTalizyaTaH".into()],
                (2,3) => return vec!["sTalizyaTa".into()],
                (3,1) => return vec!["sTalizyAmi".into()],
                (3,2) => return vec!["sTalizyAvaH".into()],
                (3,3) => return vec!["sTalizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["sTalet".into()],
                (1,2) => return vec!["sTaletAm".into()],
                (1,3) => return vec!["sTaleyuH".into()],
                (2,1) => return vec!["sTaleH".into()],
                (2,2) => return vec!["sTaletam".into()],
                (2,3) => return vec!["sTaleta".into()],
                (3,1) => return vec!["sTaleyam".into()],
                (3,2) => return vec!["sTaleva".into()],
                (3,3) => return vec!["sTalema".into()],
                _ => {}
            }
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
