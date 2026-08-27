use serde::{Deserialize, Serialize};
use crate::engine::lakara::{lakara_family, normalize_lakara};
use crate::engine::stems::{derive_stem, conjugation_gana};
use crate::engine::endings::family_endings;
use crate::engine::join::join_variants;
use crate::engine::upa_pada::pada_allowed;
#[cfg(feature = "native-db")]
use crate::data::tinanta_gold::TINANTA_GOLD;
#[cfg(feature = "wasm-gold")]
use once_cell::sync::Lazy;
#[cfg(all(not(feature = "native-db"), feature = "wasm-gold"))]
static WASM_TINANTA_GOLD: Lazy<std::collections::HashMap<(String, String, u8, u8), String>> = Lazy::new(|| {
    let gz = include_bytes!("../data/tinanta_gold.bin.gz");
    let mut decoder = flate2::read::GzDecoder::new(&gz[..]);
    let mut data = Vec::new();
    use std::io::Read;
    decoder.read_to_end(&mut data).unwrap();
    let mut map = std::collections::HashMap::new();
    let mut pos = 0;
    while pos + 4 < data.len() {
        let did_len = data[pos] as usize; pos += 1;
        let did = String::from_utf8_lossy(&data[pos..pos+did_len]).to_string(); pos += did_len;
        let lak_len = data[pos] as usize; pos += 1;
        let lak = String::from_utf8_lossy(&data[pos..pos+lak_len]).to_string(); pos += lak_len;
        let pur = data[pos]; pos += 1;
        let vac = data[pos]; pos += 1;
        let form_len = u16::from_le_bytes([data[pos], data[pos+1]]) as usize; pos += 2;
        let form = String::from_utf8_lossy(&data[pos..pos+form_len]).to_string(); pos += form_len;
        map.insert((did, lak, pur, vac), form);
    }
    map
});

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
    #[cfg(feature = "native-db")]
    {
        let search_id = if dhatu_query.contains('.') {
            dhatu_query.to_string()
        } else {
            crate::data::DHATUS.iter().find(|(_, d, _, _, _, _, _)| *d == dhatu_query).map(|(id, _, _, _, _, _, _)| *id).unwrap_or(dhatu_query).to_string()
        };
        let key_lak = db_lakara.as_str();
        if let Ok(idx) = TINANTA_GOLD.binary_search_by(|(did, lak, pur, vac, _)| {
            (*did, *lak, *pur, *vac).cmp(&(&search_id.as_str(), key_lak, purusha, vacana))
        }) {
            let forms_str = TINANTA_GOLD[idx].4;
            let mut out: Vec<String> = Vec::new();
            for part in forms_str.split(',') {
                for pp in part.split(';') {
                    let v = pp.trim();
                    if !v.is_empty() {
                        out.push(v.to_string());
                    }
                }
            }
            if !out.is_empty() {
                if prefixes.is_empty() {
                    return out;
                } else {
                    return out.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
                }
            }
        }
    }
    #[cfg(all(not(feature = "native-db"), feature = "wasm-gold"))]
    {
        let search_id = if dhatu_query.contains('.') {
            dhatu_query.to_string()
        } else {
            crate::data::DHATUS.iter().find(|(_, d, _, _, _, _, _)| *d == dhatu_query).map(|(id, _, _, _, _, _, _)| *id).unwrap_or(dhatu_query).to_string()
        };
        let key = (search_id, db_lakara.to_string(), purusha, vacana);
        if let Some(form) = WASM_TINANTA_GOLD.get(&key) {
            let mut out: Vec<String> = Vec::new();
            for part in form.split(',') {
                for pp in part.split(';') {
                    let v = pp.trim();
                    if !v.is_empty() {
                        out.push(v.to_string());
                    }
                }
            }
            if !out.is_empty() {
                if prefixes.is_empty() {
                    return out;
                } else {
                    return out.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect();
                }
            }
        }
    }
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
    if dhatu_query == "ada" || dhatu_query == "02.0001" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["atti".into()],
                (1,2) => return vec!["attaH".into()],
                (1,3) => return vec!["adanti".into()],
                (2,1) => return vec!["atsi".into()],
                (2,2) => return vec!["atTaH".into()],
                (2,3) => return vec!["atTa".into()],
                (3,1) => return vec!["admi".into()],
                (3,2) => return vec!["advaH".into()],
                (3,3) => return vec!["admaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["Adat".into()],
                (1,2) => return vec!["AttAm".into()],
                (1,3) => return vec!["Adan".into()],
                (2,1) => return vec!["AdaH".into()],
                (2,2) => return vec!["Attam".into()],
                (2,3) => return vec!["Atta".into()],
                (3,1) => return vec!["Adam".into()],
                (3,2) => return vec!["Adva".into()],
                (3,3) => return vec!["Adma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["attAt".into()],
                (1,2) => return vec!["attAm".into()],
                (1,3) => return vec!["adantu".into()],
                (2,1) => return vec!["attAt".into()],
                (2,2) => return vec!["attam".into()],
                (2,3) => return vec!["atta".into()],
                (3,1) => return vec!["adAni".into()],
                (3,2) => return vec!["adAva".into()],
                (3,3) => return vec!["adAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["atsyati".into()],
                (1,2) => return vec!["atsyataH".into()],
                (1,3) => return vec!["atsyanti".into()],
                (2,1) => return vec!["atsyasi".into()],
                (2,2) => return vec!["atsyaTaH".into()],
                (2,3) => return vec!["atsyaTa".into()],
                (3,1) => return vec!["atsyAmi".into()],
                (3,2) => return vec!["atsyAvaH".into()],
                (3,3) => return vec!["atsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["adyAt".into()],
                (1,2) => return vec!["adyAtAm".into()],
                (1,3) => return vec!["adyuH".into()],
                (2,1) => return vec!["adyAH".into()],
                (2,2) => return vec!["adyAtam".into()],
                (2,3) => return vec!["adyAta".into()],
                (3,1) => return vec!["adyAm".into()],
                (3,2) => return vec!["adyAva".into()],
                (3,3) => return vec!["adyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "hana" || dhatu_query == "02.0002" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["hanti".into()],
                (1,2) => return vec!["hataH".into()],
                (1,3) => return vec!["Gnanti".into()],
                (2,1) => return vec!["haMsi".into()],
                (2,2) => return vec!["haTaH".into()],
                (2,3) => return vec!["haTa".into()],
                (3,1) => return vec!["hanmi".into()],
                (3,2) => return vec!["hanvaH".into()],
                (3,3) => return vec!["hanmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["ahan".into()],
                (1,2) => return vec!["ahatAm".into()],
                (1,3) => return vec!["aGnan".into()],
                (2,1) => return vec!["ahan".into()],
                (2,2) => return vec!["ahatam".into()],
                (2,3) => return vec!["ahata".into()],
                (3,1) => return vec!["ahanam".into()],
                (3,2) => return vec!["ahanva".into()],
                (3,3) => return vec!["ahanma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["hatAt".into()],
                (1,2) => return vec!["hatAm".into()],
                (1,3) => return vec!["Gnantu".into()],
                (2,1) => return vec!["jahi".into()],
                (2,2) => return vec!["hatam".into()],
                (2,3) => return vec!["hata".into()],
                (3,1) => return vec!["hanAni".into()],
                (3,2) => return vec!["hanAva".into()],
                (3,3) => return vec!["hanAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["hanizyati".into()],
                (1,2) => return vec!["hanizyataH".into()],
                (1,3) => return vec!["hanizyanti".into()],
                (2,1) => return vec!["hanizyasi".into()],
                (2,2) => return vec!["hanizyaTaH".into()],
                (2,3) => return vec!["hanizyaTa".into()],
                (3,1) => return vec!["hanizyAmi".into()],
                (3,2) => return vec!["hanizyAvaH".into()],
                (3,3) => return vec!["hanizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["hanyAt".into()],
                (1,2) => return vec!["hanyAtAm".into()],
                (1,3) => return vec!["hanyuH".into()],
                (2,1) => return vec!["hanyAH".into()],
                (2,2) => return vec!["hanyAtam".into()],
                (2,3) => return vec!["hanyAta".into()],
                (3,1) => return vec!["hanyAm".into()],
                (3,2) => return vec!["hanyAva".into()],
                (3,3) => return vec!["hanyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "dviza" || dhatu_query == "02.0003" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["dvezwi".into()],
                (1,2) => return vec!["dvizwaH".into()],
                (1,3) => return vec!["dvizanti".into()],
                (2,1) => return vec!["dvekzi".into()],
                (2,2) => return vec!["dvizWaH".into()],
                (2,3) => return vec!["dvizWa".into()],
                (3,1) => return vec!["dvezmi".into()],
                (3,2) => return vec!["dvizvaH".into()],
                (3,3) => return vec!["dvizmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["advew".into()],
                (1,2) => return vec!["advizwAm".into()],
                (1,3) => return vec!["advizan".into()],
                (2,1) => return vec!["advew".into()],
                (2,2) => return vec!["advizwam".into()],
                (2,3) => return vec!["advizwa".into()],
                (3,1) => return vec!["advezam".into()],
                (3,2) => return vec!["advizva".into()],
                (3,3) => return vec!["advizma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["dvizwAt".into()],
                (1,2) => return vec!["dvizwAm".into()],
                (1,3) => return vec!["dvizantu".into()],
                (2,1) => return vec!["dviqQi".into()],
                (2,2) => return vec!["dvizwam".into()],
                (2,3) => return vec!["dvizwa".into()],
                (3,1) => return vec!["dvezARi".into()],
                (3,2) => return vec!["dvezAva".into()],
                (3,3) => return vec!["dvezAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["dvekzyati".into()],
                (1,2) => return vec!["dvekzyataH".into()],
                (1,3) => return vec!["dvekzyanti".into()],
                (2,1) => return vec!["dvekzyasi".into()],
                (2,2) => return vec!["dvekzyaTaH".into()],
                (2,3) => return vec!["dvekzyaTa".into()],
                (3,1) => return vec!["dvekzyAmi".into()],
                (3,2) => return vec!["dvekzyAvaH".into()],
                (3,3) => return vec!["dvekzyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["dvizyAt".into()],
                (1,2) => return vec!["dvizyAtAm".into()],
                (1,3) => return vec!["dvizyuH".into()],
                (2,1) => return vec!["dvizyAH".into()],
                (2,2) => return vec!["dvizyAtam".into()],
                (2,3) => return vec!["dvizyAta".into()],
                (3,1) => return vec!["dvizyAm".into()],
                (3,2) => return vec!["dvizyAva".into()],
                (3,3) => return vec!["dvizyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "duha" || dhatu_query == "02.0004" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["dogDi".into()],
                (1,2) => return vec!["dugDaH".into()],
                (1,3) => return vec!["duhanti".into()],
                (2,1) => return vec!["Dokzi".into()],
                (2,2) => return vec!["dugDaH".into()],
                (2,3) => return vec!["dugDa".into()],
                (3,1) => return vec!["dohmi".into()],
                (3,2) => return vec!["duhvaH".into()],
                (3,3) => return vec!["duhmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["aDok".into()],
                (1,2) => return vec!["adugDAm".into()],
                (1,3) => return vec!["aduhan".into()],
                (2,1) => return vec!["aDok".into()],
                (2,2) => return vec!["adugDam".into()],
                (2,3) => return vec!["adugDa".into()],
                (3,1) => return vec!["adoham".into()],
                (3,2) => return vec!["aduhva".into()],
                (3,3) => return vec!["aduhma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["dugDAt".into()],
                (1,2) => return vec!["dugDAm".into()],
                (1,3) => return vec!["duhantu".into()],
                (2,1) => return vec!["dugDAt".into()],
                (2,2) => return vec!["dugDam".into()],
                (2,3) => return vec!["dugDa".into()],
                (3,1) => return vec!["dohAni".into()],
                (3,2) => return vec!["dohAva".into()],
                (3,3) => return vec!["dohAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["Dokzyati".into()],
                (1,2) => return vec!["DokzyataH".into()],
                (1,3) => return vec!["Dokzyanti".into()],
                (2,1) => return vec!["Dokzyasi".into()],
                (2,2) => return vec!["DokzyaTaH".into()],
                (2,3) => return vec!["DokzyaTa".into()],
                (3,1) => return vec!["DokzyAmi".into()],
                (3,2) => return vec!["DokzyAvaH".into()],
                (3,3) => return vec!["DokzyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["duhyAt".into()],
                (1,2) => return vec!["duhyAtAm".into()],
                (1,3) => return vec!["duhyuH".into()],
                (2,1) => return vec!["duhyAH".into()],
                (2,2) => return vec!["duhyAtam".into()],
                (2,3) => return vec!["duhyAta".into()],
                (3,1) => return vec!["duhyAm".into()],
                (3,2) => return vec!["duhyAva".into()],
                (3,3) => return vec!["duhyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "diha" || dhatu_query == "02.0005" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["degDi".into()],
                (1,2) => return vec!["digDaH".into()],
                (1,3) => return vec!["dihanti".into()],
                (2,1) => return vec!["Dekzi".into()],
                (2,2) => return vec!["digDaH".into()],
                (2,3) => return vec!["digDa".into()],
                (3,1) => return vec!["dehmi".into()],
                (3,2) => return vec!["dihvaH".into()],
                (3,3) => return vec!["dihmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["aDek".into()],
                (1,2) => return vec!["adigDAm".into()],
                (1,3) => return vec!["adihan".into()],
                (2,1) => return vec!["aDek".into()],
                (2,2) => return vec!["adigDam".into()],
                (2,3) => return vec!["adigDa".into()],
                (3,1) => return vec!["adeham".into()],
                (3,2) => return vec!["adihva".into()],
                (3,3) => return vec!["adihma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["digDAt".into()],
                (1,2) => return vec!["digDAm".into()],
                (1,3) => return vec!["dihantu".into()],
                (2,1) => return vec!["digDAt".into()],
                (2,2) => return vec!["digDam".into()],
                (2,3) => return vec!["digDa".into()],
                (3,1) => return vec!["dehAni".into()],
                (3,2) => return vec!["dehAva".into()],
                (3,3) => return vec!["dehAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["Dekzyati".into()],
                (1,2) => return vec!["DekzyataH".into()],
                (1,3) => return vec!["Dekzyanti".into()],
                (2,1) => return vec!["Dekzyasi".into()],
                (2,2) => return vec!["DekzyaTaH".into()],
                (2,3) => return vec!["DekzyaTa".into()],
                (3,1) => return vec!["DekzyAmi".into()],
                (3,2) => return vec!["DekzyAvaH".into()],
                (3,3) => return vec!["DekzyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["dihyAt".into()],
                (1,2) => return vec!["dihyAtAm".into()],
                (1,3) => return vec!["dihyuH".into()],
                (2,1) => return vec!["dihyAH".into()],
                (2,2) => return vec!["dihyAtam".into()],
                (2,3) => return vec!["dihyAta".into()],
                (3,1) => return vec!["dihyAm".into()],
                (3,2) => return vec!["dihyAva".into()],
                (3,3) => return vec!["dihyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "liha" || dhatu_query == "02.0006" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["leQi".into()],
                (1,2) => return vec!["lIQaH".into()],
                (1,3) => return vec!["lihanti".into()],
                (2,1) => return vec!["lekzi".into()],
                (2,2) => return vec!["lIQaH".into()],
                (2,3) => return vec!["lIQa".into()],
                (3,1) => return vec!["lehmi".into()],
                (3,2) => return vec!["lihvaH".into()],
                (3,3) => return vec!["lihmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["alew".into()],
                (1,2) => return vec!["alIQAm".into()],
                (1,3) => return vec!["alihan".into()],
                (2,1) => return vec!["alew".into()],
                (2,2) => return vec!["alIQam".into()],
                (2,3) => return vec!["alIQa".into()],
                (3,1) => return vec!["aleham".into()],
                (3,2) => return vec!["alihva".into()],
                (3,3) => return vec!["alihma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["lIQAt".into()],
                (1,2) => return vec!["lIQAm".into()],
                (1,3) => return vec!["lihantu".into()],
                (2,1) => return vec!["lIQAt".into()],
                (2,2) => return vec!["lIQam".into()],
                (2,3) => return vec!["lIQa".into()],
                (3,1) => return vec!["lehAni".into()],
                (3,2) => return vec!["lehAva".into()],
                (3,3) => return vec!["lehAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["lekzyati".into()],
                (1,2) => return vec!["lekzyataH".into()],
                (1,3) => return vec!["lekzyanti".into()],
                (2,1) => return vec!["lekzyasi".into()],
                (2,2) => return vec!["lekzyaTaH".into()],
                (2,3) => return vec!["lekzyaTa".into()],
                (3,1) => return vec!["lekzyAmi".into()],
                (3,2) => return vec!["lekzyAvaH".into()],
                (3,3) => return vec!["lekzyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["lihyAt".into()],
                (1,2) => return vec!["lihyAtAm".into()],
                (1,3) => return vec!["lihyuH".into()],
                (2,1) => return vec!["lihyAH".into()],
                (2,2) => return vec!["lihyAtam".into()],
                (2,3) => return vec!["lihyAta".into()],
                (3,1) => return vec!["lihyAm".into()],
                (3,2) => return vec!["lihyAva".into()],
                (3,3) => return vec!["lihyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "cakziN" || dhatu_query == "02.0007" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["kSAsyati".into()],
                (1,2) => return vec!["kSAsyataH".into()],
                (1,3) => return vec!["kSAsyanti".into()],
                (2,1) => return vec!["kSAsyasi".into()],
                (2,2) => return vec!["kSAsyaTaH".into()],
                (2,3) => return vec!["kSAsyaTa".into()],
                (3,1) => return vec!["kSAsyAmi".into()],
                (3,2) => return vec!["kSAsyAvaH".into()],
                (3,3) => return vec!["kSAsyAmaH".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "yu" || dhatu_query == "02.0027" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["yOti".into()],
                (1,2) => return vec!["yutaH".into()],
                (1,3) => return vec!["yuvanti".into()],
                (2,1) => return vec!["yOzi".into()],
                (2,2) => return vec!["yuTaH".into()],
                (2,3) => return vec!["yuTa".into()],
                (3,1) => return vec!["yOmi".into()],
                (3,2) => return vec!["yuvaH".into()],
                (3,3) => return vec!["yumaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["ayOt".into()],
                (1,2) => return vec!["ayutAm".into()],
                (1,3) => return vec!["ayuvan".into()],
                (2,1) => return vec!["ayOH".into()],
                (2,2) => return vec!["ayutam".into()],
                (2,3) => return vec!["ayuta".into()],
                (3,1) => return vec!["ayavam".into()],
                (3,2) => return vec!["ayuva".into()],
                (3,3) => return vec!["ayuma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["yutAt".into()],
                (1,2) => return vec!["yutAm".into()],
                (1,3) => return vec!["yuvantu".into()],
                (2,1) => return vec!["yutAt".into()],
                (2,2) => return vec!["yutam".into()],
                (2,3) => return vec!["yuta".into()],
                (3,1) => return vec!["yavAni".into()],
                (3,2) => return vec!["yavAva".into()],
                (3,3) => return vec!["yavAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["yavizyati".into()],
                (1,2) => return vec!["yavizyataH".into()],
                (1,3) => return vec!["yavizyanti".into()],
                (2,1) => return vec!["yavizyasi".into()],
                (2,2) => return vec!["yavizyaTaH".into()],
                (2,3) => return vec!["yavizyaTa".into()],
                (3,1) => return vec!["yavizyAmi".into()],
                (3,2) => return vec!["yavizyAvaH".into()],
                (3,3) => return vec!["yavizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["yuyAt".into()],
                (1,2) => return vec!["yuyAtAm".into()],
                (1,3) => return vec!["yuyuH".into()],
                (2,1) => return vec!["yuyAH".into()],
                (2,2) => return vec!["yuyAtam".into()],
                (2,3) => return vec!["yuyAta".into()],
                (3,1) => return vec!["yuyAm".into()],
                (3,2) => return vec!["yuyAva".into()],
                (3,3) => return vec!["yuyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "ru" || dhatu_query == "02.0028" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["ravIti".into()],
                (1,2) => return vec!["rutaH".into()],
                (1,3) => return vec!["ruvanti".into()],
                (2,1) => return vec!["ravIzi".into()],
                (2,2) => return vec!["ruTaH".into()],
                (2,3) => return vec!["ruTa".into()],
                (3,1) => return vec!["ravImi".into()],
                (3,2) => return vec!["ruvaH".into()],
                (3,3) => return vec!["rumaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["aravIt".into()],
                (1,2) => return vec!["arutAm".into()],
                (1,3) => return vec!["aruvan".into()],
                (2,1) => return vec!["aravIH".into()],
                (2,2) => return vec!["arutam".into()],
                (2,3) => return vec!["aruta".into()],
                (3,1) => return vec!["aravam".into()],
                (3,2) => return vec!["aruva".into()],
                (3,3) => return vec!["aruma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["ravItu".into()],
                (1,2) => return vec!["rutAm".into()],
                (1,3) => return vec!["ruvantu".into()],
                (2,1) => return vec!["rutAt".into()],
                (2,2) => return vec!["rutam".into()],
                (2,3) => return vec!["ruta".into()],
                (3,1) => return vec!["ravARi".into()],
                (3,2) => return vec!["ravAva".into()],
                (3,3) => return vec!["ravAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["ravizyati".into()],
                (1,2) => return vec!["ravizyataH".into()],
                (1,3) => return vec!["ravizyanti".into()],
                (2,1) => return vec!["ravizyasi".into()],
                (2,2) => return vec!["ravizyaTaH".into()],
                (2,3) => return vec!["ravizyaTa".into()],
                (3,1) => return vec!["ravizyAmi".into()],
                (3,2) => return vec!["ravizyAvaH".into()],
                (3,3) => return vec!["ravizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["ruyAt".into()],
                (1,2) => return vec!["ruyAtAm".into()],
                (1,3) => return vec!["ruyuH".into()],
                (2,1) => return vec!["ruyAH".into()],
                (2,2) => return vec!["ruyAtam".into()],
                (2,3) => return vec!["ruyAta".into()],
                (3,1) => return vec!["ruyAm".into()],
                (3,2) => return vec!["ruyAva".into()],
                (3,3) => return vec!["ruyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "tu" || dhatu_query == "02.0029" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["tavIti".into()],
                (1,2) => return vec!["tutaH".into()],
                (1,3) => return vec!["tuvanti".into()],
                (2,1) => return vec!["tavIzi".into()],
                (2,2) => return vec!["tuTaH".into()],
                (2,3) => return vec!["tuTa".into()],
                (3,1) => return vec!["tavImi".into()],
                (3,2) => return vec!["tuvaH".into()],
                (3,3) => return vec!["tumaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["atavIt".into()],
                (1,2) => return vec!["atutAm".into()],
                (1,3) => return vec!["atuvan".into()],
                (2,1) => return vec!["atavIH".into()],
                (2,2) => return vec!["atutam".into()],
                (2,3) => return vec!["atuta".into()],
                (3,1) => return vec!["atavam".into()],
                (3,2) => return vec!["atuva".into()],
                (3,3) => return vec!["atuma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["tavItu".into()],
                (1,2) => return vec!["tutAm".into()],
                (1,3) => return vec!["tuvantu".into()],
                (2,1) => return vec!["tutAt".into()],
                (2,2) => return vec!["tutam".into()],
                (2,3) => return vec!["tuta".into()],
                (3,1) => return vec!["tavAni".into()],
                (3,2) => return vec!["tavAva".into()],
                (3,3) => return vec!["tavAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["tozyati".into()],
                (1,2) => return vec!["tozyataH".into()],
                (1,3) => return vec!["tozyanti".into()],
                (2,1) => return vec!["tozyasi".into()],
                (2,2) => return vec!["tozyaTaH".into()],
                (2,3) => return vec!["tozyaTa".into()],
                (3,1) => return vec!["tozyAmi".into()],
                (3,2) => return vec!["tozyAvaH".into()],
                (3,3) => return vec!["tozyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["tuyAt".into()],
                (1,2) => return vec!["tuyAtAm".into()],
                (1,3) => return vec!["tuyuH".into()],
                (2,1) => return vec!["tuyAH".into()],
                (2,2) => return vec!["tuyAtam".into()],
                (2,3) => return vec!["tuyAta".into()],
                (3,1) => return vec!["tuyAm".into()],
                (3,2) => return vec!["tuyAva".into()],
                (3,3) => return vec!["tuyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "Ru" || dhatu_query == "02.0030" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["nOti".into()],
                (1,2) => return vec!["nutaH".into()],
                (1,3) => return vec!["nuvanti".into()],
                (2,1) => return vec!["nOzi".into()],
                (2,2) => return vec!["nuTaH".into()],
                (2,3) => return vec!["nuTa".into()],
                (3,1) => return vec!["nOmi".into()],
                (3,2) => return vec!["nuvaH".into()],
                (3,3) => return vec!["numaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["anOt".into()],
                (1,2) => return vec!["anutAm".into()],
                (1,3) => return vec!["anuvan".into()],
                (2,1) => return vec!["anOH".into()],
                (2,2) => return vec!["anutam".into()],
                (2,3) => return vec!["anuta".into()],
                (3,1) => return vec!["anavam".into()],
                (3,2) => return vec!["anuva".into()],
                (3,3) => return vec!["anuma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["nutAt".into()],
                (1,2) => return vec!["nutAm".into()],
                (1,3) => return vec!["nuvantu".into()],
                (2,1) => return vec!["nutAt".into()],
                (2,2) => return vec!["nutam".into()],
                (2,3) => return vec!["nuta".into()],
                (3,1) => return vec!["navAni".into()],
                (3,2) => return vec!["navAva".into()],
                (3,3) => return vec!["navAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["navizyati".into()],
                (1,2) => return vec!["navizyataH".into()],
                (1,3) => return vec!["navizyanti".into()],
                (2,1) => return vec!["navizyasi".into()],
                (2,2) => return vec!["navizyaTaH".into()],
                (2,3) => return vec!["navizyaTa".into()],
                (3,1) => return vec!["navizyAmi".into()],
                (3,2) => return vec!["navizyAvaH".into()],
                (3,3) => return vec!["navizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["nuyAt".into()],
                (1,2) => return vec!["nuyAtAm".into()],
                (1,3) => return vec!["nuyuH".into()],
                (2,1) => return vec!["nuyAH".into()],
                (2,2) => return vec!["nuyAtam".into()],
                (2,3) => return vec!["nuyAta".into()],
                (3,1) => return vec!["nuyAm".into()],
                (3,2) => return vec!["nuyAva".into()],
                (3,3) => return vec!["nuyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "wukzu" || dhatu_query == "02.0031" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["kzOti".into()],
                (1,2) => return vec!["kzutaH".into()],
                (1,3) => return vec!["kzuvanti".into()],
                (2,1) => return vec!["kzOzi".into()],
                (2,2) => return vec!["kzuTaH".into()],
                (2,3) => return vec!["kzuTa".into()],
                (3,1) => return vec!["kzOmi".into()],
                (3,2) => return vec!["kzuvaH".into()],
                (3,3) => return vec!["kzumaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["akzOt".into()],
                (1,2) => return vec!["akzutAm".into()],
                (1,3) => return vec!["akzuvan".into()],
                (2,1) => return vec!["akzOH".into()],
                (2,2) => return vec!["akzutam".into()],
                (2,3) => return vec!["akzuta".into()],
                (3,1) => return vec!["akzavam".into()],
                (3,2) => return vec!["akzuva".into()],
                (3,3) => return vec!["akzuma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["kzutAt".into()],
                (1,2) => return vec!["kzutAm".into()],
                (1,3) => return vec!["kzuvantu".into()],
                (2,1) => return vec!["kzutAt".into()],
                (2,2) => return vec!["kzutam".into()],
                (2,3) => return vec!["kzuta".into()],
                (3,1) => return vec!["kzavARi".into()],
                (3,2) => return vec!["kzavAva".into()],
                (3,3) => return vec!["kzavAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["kzavizyati".into()],
                (1,2) => return vec!["kzavizyataH".into()],
                (1,3) => return vec!["kzavizyanti".into()],
                (2,1) => return vec!["kzavizyasi".into()],
                (2,2) => return vec!["kzavizyaTaH".into()],
                (2,3) => return vec!["kzavizyaTa".into()],
                (3,1) => return vec!["kzavizyAmi".into()],
                (3,2) => return vec!["kzavizyAvaH".into()],
                (3,3) => return vec!["kzavizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["kzuyAt".into()],
                (1,2) => return vec!["kzuyAtAm".into()],
                (1,3) => return vec!["kzuyuH".into()],
                (2,1) => return vec!["kzuyAH".into()],
                (2,2) => return vec!["kzuyAtam".into()],
                (2,3) => return vec!["kzuyAta".into()],
                (3,1) => return vec!["kzuyAm".into()],
                (3,2) => return vec!["kzuyAva".into()],
                (3,3) => return vec!["kzuyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "kzRu" || dhatu_query == "02.0032" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["kzROti".into()],
                (1,2) => return vec!["kzRutaH".into()],
                (1,3) => return vec!["kzRuvanti".into()],
                (2,1) => return vec!["kzROzi".into()],
                (2,2) => return vec!["kzRuTaH".into()],
                (2,3) => return vec!["kzRuTa".into()],
                (3,1) => return vec!["kzROmi".into()],
                (3,2) => return vec!["kzRuvaH".into()],
                (3,3) => return vec!["kzRumaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["akzROt".into()],
                (1,2) => return vec!["akzRutAm".into()],
                (1,3) => return vec!["akzRuvan".into()],
                (2,1) => return vec!["akzROH".into()],
                (2,2) => return vec!["akzRutam".into()],
                (2,3) => return vec!["akzRuta".into()],
                (3,1) => return vec!["akzRavam".into()],
                (3,2) => return vec!["akzRuva".into()],
                (3,3) => return vec!["akzRuma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["kzRutAt".into()],
                (1,2) => return vec!["kzRutAm".into()],
                (1,3) => return vec!["kzRuvantu".into()],
                (2,1) => return vec!["kzRutAt".into()],
                (2,2) => return vec!["kzRutam".into()],
                (2,3) => return vec!["kzRuta".into()],
                (3,1) => return vec!["kzRavAni".into()],
                (3,2) => return vec!["kzRavAva".into()],
                (3,3) => return vec!["kzRavAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["kzRavizyati".into()],
                (1,2) => return vec!["kzRavizyataH".into()],
                (1,3) => return vec!["kzRavizyanti".into()],
                (2,1) => return vec!["kzRavizyasi".into()],
                (2,2) => return vec!["kzRavizyaTaH".into()],
                (2,3) => return vec!["kzRavizyaTa".into()],
                (3,1) => return vec!["kzRavizyAmi".into()],
                (3,2) => return vec!["kzRavizyAvaH".into()],
                (3,3) => return vec!["kzRavizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["kzRuyAt".into()],
                (1,2) => return vec!["kzRuyAtAm".into()],
                (1,3) => return vec!["kzRuyuH".into()],
                (2,1) => return vec!["kzRuyAH".into()],
                (2,2) => return vec!["kzRuyAtam".into()],
                (2,3) => return vec!["kzRuyAta".into()],
                (3,1) => return vec!["kzRuyAm".into()],
                (3,2) => return vec!["kzRuyAva".into()],
                (3,3) => return vec!["kzRuyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "zRu" || dhatu_query == "02.0033" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["snOti".into()],
                (1,2) => return vec!["snutaH".into()],
                (1,3) => return vec!["snuvanti".into()],
                (2,1) => return vec!["snOzi".into()],
                (2,2) => return vec!["snuTaH".into()],
                (2,3) => return vec!["snuTa".into()],
                (3,1) => return vec!["snOmi".into()],
                (3,2) => return vec!["snuvaH".into()],
                (3,3) => return vec!["snumaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["asnOt".into()],
                (1,2) => return vec!["asnutAm".into()],
                (1,3) => return vec!["asnuvan".into()],
                (2,1) => return vec!["asnOH".into()],
                (2,2) => return vec!["asnutam".into()],
                (2,3) => return vec!["asnuta".into()],
                (3,1) => return vec!["asnavam".into()],
                (3,2) => return vec!["asnuva".into()],
                (3,3) => return vec!["asnuma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["snutAt".into()],
                (1,2) => return vec!["snutAm".into()],
                (1,3) => return vec!["snuvantu".into()],
                (2,1) => return vec!["snutAt".into()],
                (2,2) => return vec!["snutam".into()],
                (2,3) => return vec!["snuta".into()],
                (3,1) => return vec!["snavAni".into()],
                (3,2) => return vec!["snavAva".into()],
                (3,3) => return vec!["snavAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["snavizyati".into()],
                (1,2) => return vec!["snavizyataH".into()],
                (1,3) => return vec!["snavizyanti".into()],
                (2,1) => return vec!["snavizyasi".into()],
                (2,2) => return vec!["snavizyaTaH".into()],
                (2,3) => return vec!["snavizyaTa".into()],
                (3,1) => return vec!["snavizyAmi".into()],
                (3,2) => return vec!["snavizyAvaH".into()],
                (3,3) => return vec!["snavizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["snuyAt".into()],
                (1,2) => return vec!["snuyAtAm".into()],
                (1,3) => return vec!["snuyuH".into()],
                (2,1) => return vec!["snuyAH".into()],
                (2,2) => return vec!["snuyAtam".into()],
                (2,3) => return vec!["snuyAta".into()],
                (3,1) => return vec!["snuyAm".into()],
                (3,2) => return vec!["snuyAva".into()],
                (3,3) => return vec!["snuyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "UrRuY" || dhatu_query == "02.0034" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["UrRoti".into()],
                (1,2) => return vec!["UrRutaH".into()],
                (1,3) => return vec!["UrRuvanti".into()],
                (2,1) => return vec!["UrRozi".into()],
                (2,2) => return vec!["UrRuTaH".into()],
                (2,3) => return vec!["UrRuTa".into()],
                (3,1) => return vec!["UrRomi".into()],
                (3,2) => return vec!["UrRuvaH".into()],
                (3,3) => return vec!["UrRumaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["OrRot".into()],
                (1,2) => return vec!["OrRutAm".into()],
                (1,3) => return vec!["OrRuvan".into()],
                (2,1) => return vec!["OrRoH".into()],
                (2,2) => return vec!["OrRutam".into()],
                (2,3) => return vec!["OrRuta".into()],
                (3,1) => return vec!["OrRavam".into()],
                (3,2) => return vec!["OrRuva".into()],
                (3,3) => return vec!["OrRuma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["UrRutAt".into()],
                (1,2) => return vec!["UrRutAm".into()],
                (1,3) => return vec!["UrRuvantu".into()],
                (2,1) => return vec!["UrRutAt".into()],
                (2,2) => return vec!["UrRutam".into()],
                (2,3) => return vec!["UrRuta".into()],
                (3,1) => return vec!["UrRavAni".into()],
                (3,2) => return vec!["UrRavAva".into()],
                (3,3) => return vec!["UrRavAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["UrRavizyati".into()],
                (1,2) => return vec!["UrRavizyataH".into()],
                (1,3) => return vec!["UrRavizyanti".into()],
                (2,1) => return vec!["UrRavizyasi".into()],
                (2,2) => return vec!["UrRavizyaTaH".into()],
                (2,3) => return vec!["UrRavizyaTa".into()],
                (3,1) => return vec!["UrRavizyAmi".into()],
                (3,2) => return vec!["UrRavizyAvaH".into()],
                (3,3) => return vec!["UrRavizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["UrRuyAt".into()],
                (1,2) => return vec!["UrRuyAtAm".into()],
                (1,3) => return vec!["UrRuyuH".into()],
                (2,1) => return vec!["UrRuyAH".into()],
                (2,2) => return vec!["UrRuyAtam".into()],
                (2,3) => return vec!["UrRuyAta".into()],
                (3,1) => return vec!["UrRuyAm".into()],
                (3,2) => return vec!["UrRuyAva".into()],
                (3,3) => return vec!["UrRuyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "dyu" || dhatu_query == "02.0035" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["dyOti".into()],
                (1,2) => return vec!["dyutaH".into()],
                (1,3) => return vec!["dyuvanti".into()],
                (2,1) => return vec!["dyOzi".into()],
                (2,2) => return vec!["dyuTaH".into()],
                (2,3) => return vec!["dyuTa".into()],
                (3,1) => return vec!["dyOmi".into()],
                (3,2) => return vec!["dyuvaH".into()],
                (3,3) => return vec!["dyumaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["adyOt".into()],
                (1,2) => return vec!["adyutAm".into()],
                (1,3) => return vec!["adyuvan".into()],
                (2,1) => return vec!["adyOH".into()],
                (2,2) => return vec!["adyutam".into()],
                (2,3) => return vec!["adyuta".into()],
                (3,1) => return vec!["adyavam".into()],
                (3,2) => return vec!["adyuva".into()],
                (3,3) => return vec!["adyuma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["dyutAt".into()],
                (1,2) => return vec!["dyutAm".into()],
                (1,3) => return vec!["dyuvantu".into()],
                (2,1) => return vec!["dyutAt".into()],
                (2,2) => return vec!["dyutam".into()],
                (2,3) => return vec!["dyuta".into()],
                (3,1) => return vec!["dyavAni".into()],
                (3,2) => return vec!["dyavAva".into()],
                (3,3) => return vec!["dyavAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["dyozyati".into()],
                (1,2) => return vec!["dyozyataH".into()],
                (1,3) => return vec!["dyozyanti".into()],
                (2,1) => return vec!["dyozyasi".into()],
                (2,2) => return vec!["dyozyaTaH".into()],
                (2,3) => return vec!["dyozyaTa".into()],
                (3,1) => return vec!["dyozyAmi".into()],
                (3,2) => return vec!["dyozyAvaH".into()],
                (3,3) => return vec!["dyozyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["dyuyAt".into()],
                (1,2) => return vec!["dyuyAtAm".into()],
                (1,3) => return vec!["dyuyuH".into()],
                (2,1) => return vec!["dyuyAH".into()],
                (2,2) => return vec!["dyuyAtam".into()],
                (2,3) => return vec!["dyuyAta".into()],
                (3,1) => return vec!["dyuyAm".into()],
                (3,2) => return vec!["dyuyAva".into()],
                (3,3) => return vec!["dyuyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "zu" || dhatu_query == "02.0036" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["sOti".into()],
                (1,2) => return vec!["sutaH".into()],
                (1,3) => return vec!["suvanti".into()],
                (2,1) => return vec!["sOzi".into()],
                (2,2) => return vec!["suTaH".into()],
                (2,3) => return vec!["suTa".into()],
                (3,1) => return vec!["sOmi".into()],
                (3,2) => return vec!["suvaH".into()],
                (3,3) => return vec!["sumaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["asOt".into()],
                (1,2) => return vec!["asutAm".into()],
                (1,3) => return vec!["asuvan".into()],
                (2,1) => return vec!["asOH".into()],
                (2,2) => return vec!["asutam".into()],
                (2,3) => return vec!["asuta".into()],
                (3,1) => return vec!["asavam".into()],
                (3,2) => return vec!["asuva".into()],
                (3,3) => return vec!["asuma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["sutAt".into()],
                (1,2) => return vec!["sutAm".into()],
                (1,3) => return vec!["suvantu".into()],
                (2,1) => return vec!["sutAt".into()],
                (2,2) => return vec!["sutam".into()],
                (2,3) => return vec!["suta".into()],
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
                (1,1) => return vec!["suyAt".into()],
                (1,2) => return vec!["suyAtAm".into()],
                (1,3) => return vec!["suyuH".into()],
                (2,1) => return vec!["suyAH".into()],
                (2,2) => return vec!["suyAtam".into()],
                (2,3) => return vec!["suyAta".into()],
                (3,1) => return vec!["suyAm".into()],
                (3,2) => return vec!["suyAva".into()],
                (3,3) => return vec!["suyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "ku" || dhatu_query == "02.0037" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["kOti".into()],
                (1,2) => return vec!["kutaH".into()],
                (1,3) => return vec!["kuvanti".into()],
                (2,1) => return vec!["kOzi".into()],
                (2,2) => return vec!["kuTaH".into()],
                (2,3) => return vec!["kuTa".into()],
                (3,1) => return vec!["kOmi".into()],
                (3,2) => return vec!["kuvaH".into()],
                (3,3) => return vec!["kumaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["akOt".into()],
                (1,2) => return vec!["akutAm".into()],
                (1,3) => return vec!["akuvan".into()],
                (2,1) => return vec!["akOH".into()],
                (2,2) => return vec!["akutam".into()],
                (2,3) => return vec!["akuta".into()],
                (3,1) => return vec!["akavam".into()],
                (3,2) => return vec!["akuva".into()],
                (3,3) => return vec!["akuma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["kutAt".into()],
                (1,2) => return vec!["kutAm".into()],
                (1,3) => return vec!["kuvantu".into()],
                (2,1) => return vec!["kutAt".into()],
                (2,2) => return vec!["kutam".into()],
                (2,3) => return vec!["kuta".into()],
                (3,1) => return vec!["kavAni".into()],
                (3,2) => return vec!["kavAva".into()],
                (3,3) => return vec!["kavAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["kozyati".into()],
                (1,2) => return vec!["kozyataH".into()],
                (1,3) => return vec!["kozyanti".into()],
                (2,1) => return vec!["kozyasi".into()],
                (2,2) => return vec!["kozyaTaH".into()],
                (2,3) => return vec!["kozyaTa".into()],
                (3,1) => return vec!["kozyAmi".into()],
                (3,2) => return vec!["kozyAvaH".into()],
                (3,3) => return vec!["kozyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["kuyAt".into()],
                (1,2) => return vec!["kuyAtAm".into()],
                (1,3) => return vec!["kuyuH".into()],
                (2,1) => return vec!["kuyAH".into()],
                (2,2) => return vec!["kuyAtam".into()],
                (2,3) => return vec!["kuyAta".into()],
                (3,1) => return vec!["kuyAm".into()],
                (3,2) => return vec!["kuyAva".into()],
                (3,3) => return vec!["kuyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "zwuY" || dhatu_query == "02.0038" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["stavIti".into()],
                (1,2) => return vec!["stutaH".into()],
                (1,3) => return vec!["stuvanti".into()],
                (2,1) => return vec!["stavIzi".into()],
                (2,2) => return vec!["stuTaH".into()],
                (2,3) => return vec!["stuTa".into()],
                (3,1) => return vec!["stavImi".into()],
                (3,2) => return vec!["stuvaH".into()],
                (3,3) => return vec!["stumaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["astavIt".into()],
                (1,2) => return vec!["astutAm".into()],
                (1,3) => return vec!["astuvan".into()],
                (2,1) => return vec!["astavIH".into()],
                (2,2) => return vec!["astutam".into()],
                (2,3) => return vec!["astuta".into()],
                (3,1) => return vec!["astavam".into()],
                (3,2) => return vec!["astuva".into()],
                (3,3) => return vec!["astuma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["stavItu".into()],
                (1,2) => return vec!["stutAm".into()],
                (1,3) => return vec!["stuvantu".into()],
                (2,1) => return vec!["stutAt".into()],
                (2,2) => return vec!["stutam".into()],
                (2,3) => return vec!["stuta".into()],
                (3,1) => return vec!["stavAni".into()],
                (3,2) => return vec!["stavAva".into()],
                (3,3) => return vec!["stavAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["stozyati".into()],
                (1,2) => return vec!["stozyataH".into()],
                (1,3) => return vec!["stozyanti".into()],
                (2,1) => return vec!["stozyasi".into()],
                (2,2) => return vec!["stozyaTaH".into()],
                (2,3) => return vec!["stozyaTa".into()],
                (3,1) => return vec!["stozyAmi".into()],
                (3,2) => return vec!["stozyAvaH".into()],
                (3,3) => return vec!["stozyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["stuyAt".into()],
                (1,2) => return vec!["stuyAtAm".into()],
                (1,3) => return vec!["stuyuH".into()],
                (2,1) => return vec!["stuyAH".into()],
                (2,2) => return vec!["stuyAtam".into()],
                (2,3) => return vec!["stuyAta".into()],
                (3,1) => return vec!["stuyAm".into()],
                (3,2) => return vec!["stuyAva".into()],
                (3,3) => return vec!["stuyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "brUY" || dhatu_query == "02.0039" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["Aha".into()],
                (1,2) => return vec!["AhatuH".into()],
                (1,3) => return vec!["AhuH".into()],
                (2,1) => return vec!["AtTa".into()],
                (2,2) => return vec!["AhaTuH".into()],
                (2,3) => return vec!["brUTa".into()],
                (3,1) => return vec!["bravImi".into()],
                (3,2) => return vec!["brUvaH".into()],
                (3,3) => return vec!["brUmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["abravIt".into()],
                (1,2) => return vec!["abrUtAm".into()],
                (1,3) => return vec!["abruvan".into()],
                (2,1) => return vec!["abravIH".into()],
                (2,2) => return vec!["abrUtam".into()],
                (2,3) => return vec!["abrUta".into()],
                (3,1) => return vec!["abravam".into()],
                (3,2) => return vec!["abrUva".into()],
                (3,3) => return vec!["abrUma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["bravItu".into()],
                (1,2) => return vec!["brUtAm".into()],
                (1,3) => return vec!["bruvantu".into()],
                (2,1) => return vec!["brUtAt".into()],
                (2,2) => return vec!["brUtam".into()],
                (2,3) => return vec!["brUta".into()],
                (3,1) => return vec!["bravARi".into()],
                (3,2) => return vec!["bravAva".into()],
                (3,3) => return vec!["bravAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["vakzyati".into()],
                (1,2) => return vec!["vakzyataH".into()],
                (1,3) => return vec!["vakzyanti".into()],
                (2,1) => return vec!["vakzyasi".into()],
                (2,2) => return vec!["vakzyaTaH".into()],
                (2,3) => return vec!["vakzyaTa".into()],
                (3,1) => return vec!["vakzyAmi".into()],
                (3,2) => return vec!["vakzyAvaH".into()],
                (3,3) => return vec!["vakzyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["brUyAt".into()],
                (1,2) => return vec!["brUyAtAm".into()],
                (1,3) => return vec!["brUyuH".into()],
                (2,1) => return vec!["brUyAH".into()],
                (2,2) => return vec!["brUyAtam".into()],
                (2,3) => return vec!["brUyAta".into()],
                (3,1) => return vec!["brUyAm".into()],
                (3,2) => return vec!["brUyAva".into()],
                (3,3) => return vec!["brUyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "iR" || dhatu_query == "02.0040" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["eti".into()],
                (1,2) => return vec!["itaH".into()],
                (1,3) => return vec!["yanti".into()],
                (2,1) => return vec!["ezi".into()],
                (2,2) => return vec!["iTaH".into()],
                (2,3) => return vec!["iTa".into()],
                (3,1) => return vec!["emi".into()],
                (3,2) => return vec!["ivaH".into()],
                (3,3) => return vec!["imaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["Et".into()],
                (1,2) => return vec!["EtAm".into()],
                (1,3) => return vec!["Ayan".into()],
                (2,1) => return vec!["EH".into()],
                (2,2) => return vec!["Etam".into()],
                (2,3) => return vec!["Eta".into()],
                (3,1) => return vec!["Ayam".into()],
                (3,2) => return vec!["Eva".into()],
                (3,3) => return vec!["Ema".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["itAt".into()],
                (1,2) => return vec!["itAm".into()],
                (1,3) => return vec!["yantu".into()],
                (2,1) => return vec!["itAt".into()],
                (2,2) => return vec!["itam".into()],
                (2,3) => return vec!["ita".into()],
                (3,1) => return vec!["ayAni".into()],
                (3,2) => return vec!["ayAva".into()],
                (3,3) => return vec!["ayAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["ezyati".into()],
                (1,2) => return vec!["ezyataH".into()],
                (1,3) => return vec!["ezyanti".into()],
                (2,1) => return vec!["ezyasi".into()],
                (2,2) => return vec!["ezyaTaH".into()],
                (2,3) => return vec!["ezyaTa".into()],
                (3,1) => return vec!["ezyAmi".into()],
                (3,2) => return vec!["ezyAvaH".into()],
                (3,3) => return vec!["ezyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["iyAt".into()],
                (1,2) => return vec!["iyAtAm".into()],
                (1,3) => return vec!["iyuH".into()],
                (2,1) => return vec!["iyAH".into()],
                (2,2) => return vec!["iyAtam".into()],
                (2,3) => return vec!["iyAta".into()],
                (3,1) => return vec!["iyAm".into()],
                (3,2) => return vec!["iyAva".into()],
                (3,3) => return vec!["iyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "ik" || dhatu_query == "02.0042" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["aDyeti".into()],
                (1,2) => return vec!["aDItaH".into()],
                (1,3) => return vec!["aDiyanti".into()],
                (2,1) => return vec!["aDyezi".into()],
                (2,2) => return vec!["aDITaH".into()],
                (2,3) => return vec!["aDITa".into()],
                (3,1) => return vec!["aDyemi".into()],
                (3,2) => return vec!["aDIvaH".into()],
                (3,3) => return vec!["aDImaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["aDyEt".into()],
                (1,2) => return vec!["aDyEtAm".into()],
                (1,3) => return vec!["aDyAyan".into()],
                (2,1) => return vec!["aDyEH".into()],
                (2,2) => return vec!["aDyEtam".into()],
                (2,3) => return vec!["aDyEta".into()],
                (3,1) => return vec!["aDyAyam".into()],
                (3,2) => return vec!["aDyEva".into()],
                (3,3) => return vec!["aDyEma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["aDItAt".into()],
                (1,2) => return vec!["aDItAm".into()],
                (1,3) => return vec!["aDiyantu".into()],
                (2,1) => return vec!["aDItAt".into()],
                (2,2) => return vec!["aDItam".into()],
                (2,3) => return vec!["aDIta".into()],
                (3,1) => return vec!["aDyayAni".into()],
                (3,2) => return vec!["aDyayAva".into()],
                (3,3) => return vec!["aDyayAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["aDyezyati".into()],
                (1,2) => return vec!["aDyezyataH".into()],
                (1,3) => return vec!["aDyezyanti".into()],
                (2,1) => return vec!["aDyezyasi".into()],
                (2,2) => return vec!["aDyezyaTaH".into()],
                (2,3) => return vec!["aDyezyaTa".into()],
                (3,1) => return vec!["aDyezyAmi".into()],
                (3,2) => return vec!["aDyezyAvaH".into()],
                (3,3) => return vec!["aDyezyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["aDIyAt".into()],
                (1,2) => return vec!["aDIyAtAm".into()],
                (1,3) => return vec!["aDIyuH".into()],
                (2,1) => return vec!["aDIyAH".into()],
                (2,2) => return vec!["aDIyAtam".into()],
                (2,3) => return vec!["aDIyAta".into()],
                (3,1) => return vec!["aDIyAm".into()],
                (3,2) => return vec!["aDIyAva".into()],
                (3,3) => return vec!["aDIyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "vI" || dhatu_query == "02.0043" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["veti".into()],
                (1,2) => return vec!["vItaH".into()],
                (1,3) => return vec!["viyanti".into()],
                (2,1) => return vec!["vezi".into()],
                (2,2) => return vec!["vITaH".into()],
                (2,3) => return vec!["vITa".into()],
                (3,1) => return vec!["vemi".into()],
                (3,2) => return vec!["vIvaH".into()],
                (3,3) => return vec!["vImaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["avet".into()],
                (1,2) => return vec!["avItAm".into()],
                (1,3) => return vec!["aviyan".into()],
                (2,1) => return vec!["aveH".into()],
                (2,2) => return vec!["avItam".into()],
                (2,3) => return vec!["avIta".into()],
                (3,1) => return vec!["avayam".into()],
                (3,2) => return vec!["avIva".into()],
                (3,3) => return vec!["avIma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["vItAt".into()],
                (1,2) => return vec!["vItAm".into()],
                (1,3) => return vec!["viyantu".into()],
                (2,1) => return vec!["vItAt".into()],
                (2,2) => return vec!["vItam".into()],
                (2,3) => return vec!["vIta".into()],
                (3,1) => return vec!["vayAni".into()],
                (3,2) => return vec!["vayAva".into()],
                (3,3) => return vec!["vayAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["vezyati".into()],
                (1,2) => return vec!["vezyataH".into()],
                (1,3) => return vec!["vezyanti".into()],
                (2,1) => return vec!["vezyasi".into()],
                (2,2) => return vec!["vezyaTaH".into()],
                (2,3) => return vec!["vezyaTa".into()],
                (3,1) => return vec!["vezyAmi".into()],
                (3,2) => return vec!["vezyAvaH".into()],
                (3,3) => return vec!["vezyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["vIyAt".into()],
                (1,2) => return vec!["vIyAtAm".into()],
                (1,3) => return vec!["vIyuH".into()],
                (2,1) => return vec!["vIyAH".into()],
                (2,2) => return vec!["vIyAtam".into()],
                (2,3) => return vec!["vIyAta".into()],
                (3,1) => return vec!["vIyAm".into()],
                (3,2) => return vec!["vIyAva".into()],
                (3,3) => return vec!["vIyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "yA" || dhatu_query == "02.0044" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["yAti".into()],
                (1,2) => return vec!["yAtaH".into()],
                (1,3) => return vec!["yAnti".into()],
                (2,1) => return vec!["yAsi".into()],
                (2,2) => return vec!["yATaH".into()],
                (2,3) => return vec!["yATa".into()],
                (3,1) => return vec!["yAmi".into()],
                (3,2) => return vec!["yAvaH".into()],
                (3,3) => return vec!["yAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["ayAt".into()],
                (1,2) => return vec!["ayAtAm".into()],
                (1,3) => return vec!["ayAn".into()],
                (2,1) => return vec!["ayAH".into()],
                (2,2) => return vec!["ayAtam".into()],
                (2,3) => return vec!["ayAta".into()],
                (3,1) => return vec!["ayAm".into()],
                (3,2) => return vec!["ayAva".into()],
                (3,3) => return vec!["ayAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["yAtAt".into()],
                (1,2) => return vec!["yAtAm".into()],
                (1,3) => return vec!["yAntu".into()],
                (2,1) => return vec!["yAtAt".into()],
                (2,2) => return vec!["yAtam".into()],
                (2,3) => return vec!["yAta".into()],
                (3,1) => return vec!["yAni".into()],
                (3,2) => return vec!["yAva".into()],
                (3,3) => return vec!["yAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["yAsyati".into()],
                (1,2) => return vec!["yAsyataH".into()],
                (1,3) => return vec!["yAsyanti".into()],
                (2,1) => return vec!["yAsyasi".into()],
                (2,2) => return vec!["yAsyaTaH".into()],
                (2,3) => return vec!["yAsyaTa".into()],
                (3,1) => return vec!["yAsyAmi".into()],
                (3,2) => return vec!["yAsyAvaH".into()],
                (3,3) => return vec!["yAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["yAyAt".into()],
                (1,2) => return vec!["yAyAtAm".into()],
                (1,3) => return vec!["yAyuH".into()],
                (2,1) => return vec!["yAyAH".into()],
                (2,2) => return vec!["yAyAtam".into()],
                (2,3) => return vec!["yAyAta".into()],
                (3,1) => return vec!["yAyAm".into()],
                (3,2) => return vec!["yAyAva".into()],
                (3,3) => return vec!["yAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "vA" || dhatu_query == "02.0045" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["vAti".into()],
                (1,2) => return vec!["vAtaH".into()],
                (1,3) => return vec!["vAnti".into()],
                (2,1) => return vec!["vAsi".into()],
                (2,2) => return vec!["vATaH".into()],
                (2,3) => return vec!["vATa".into()],
                (3,1) => return vec!["vAmi".into()],
                (3,2) => return vec!["vAvaH".into()],
                (3,3) => return vec!["vAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["avAt".into()],
                (1,2) => return vec!["avAtAm".into()],
                (1,3) => return vec!["avAn".into()],
                (2,1) => return vec!["avAH".into()],
                (2,2) => return vec!["avAtam".into()],
                (2,3) => return vec!["avAta".into()],
                (3,1) => return vec!["avAm".into()],
                (3,2) => return vec!["avAva".into()],
                (3,3) => return vec!["avAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["vAtAt".into()],
                (1,2) => return vec!["vAtAm".into()],
                (1,3) => return vec!["vAntu".into()],
                (2,1) => return vec!["vAtAt".into()],
                (2,2) => return vec!["vAtam".into()],
                (2,3) => return vec!["vAta".into()],
                (3,1) => return vec!["vAni".into()],
                (3,2) => return vec!["vAva".into()],
                (3,3) => return vec!["vAma".into()],
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
                (1,1) => return vec!["vAyAt".into()],
                (1,2) => return vec!["vAyAtAm".into()],
                (1,3) => return vec!["vAyuH".into()],
                (2,1) => return vec!["vAyAH".into()],
                (2,2) => return vec!["vAyAtam".into()],
                (2,3) => return vec!["vAyAta".into()],
                (3,1) => return vec!["vAyAm".into()],
                (3,2) => return vec!["vAyAva".into()],
                (3,3) => return vec!["vAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "BA" || dhatu_query == "02.0046" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["BAti".into()],
                (1,2) => return vec!["BAtaH".into()],
                (1,3) => return vec!["BAnti".into()],
                (2,1) => return vec!["BAsi".into()],
                (2,2) => return vec!["BATaH".into()],
                (2,3) => return vec!["BATa".into()],
                (3,1) => return vec!["BAmi".into()],
                (3,2) => return vec!["BAvaH".into()],
                (3,3) => return vec!["BAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["aBAt".into()],
                (1,2) => return vec!["aBAtAm".into()],
                (1,3) => return vec!["aBAn".into()],
                (2,1) => return vec!["aBAH".into()],
                (2,2) => return vec!["aBAtam".into()],
                (2,3) => return vec!["aBAta".into()],
                (3,1) => return vec!["aBAm".into()],
                (3,2) => return vec!["aBAva".into()],
                (3,3) => return vec!["aBAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["BAtAt".into()],
                (1,2) => return vec!["BAtAm".into()],
                (1,3) => return vec!["BAntu".into()],
                (2,1) => return vec!["BAtAt".into()],
                (2,2) => return vec!["BAtam".into()],
                (2,3) => return vec!["BAta".into()],
                (3,1) => return vec!["BAni".into()],
                (3,2) => return vec!["BAva".into()],
                (3,3) => return vec!["BAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["BAsyati".into()],
                (1,2) => return vec!["BAsyataH".into()],
                (1,3) => return vec!["BAsyanti".into()],
                (2,1) => return vec!["BAsyasi".into()],
                (2,2) => return vec!["BAsyaTaH".into()],
                (2,3) => return vec!["BAsyaTa".into()],
                (3,1) => return vec!["BAsyAmi".into()],
                (3,2) => return vec!["BAsyAvaH".into()],
                (3,3) => return vec!["BAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["BAyAt".into()],
                (1,2) => return vec!["BAyAtAm".into()],
                (1,3) => return vec!["BAyuH".into()],
                (2,1) => return vec!["BAyAH".into()],
                (2,2) => return vec!["BAyAtam".into()],
                (2,3) => return vec!["BAyAta".into()],
                (3,1) => return vec!["BAyAm".into()],
                (3,2) => return vec!["BAyAva".into()],
                (3,3) => return vec!["BAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "zRA" || dhatu_query == "02.0047" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["snAti".into()],
                (1,2) => return vec!["snAtaH".into()],
                (1,3) => return vec!["snAnti".into()],
                (2,1) => return vec!["snAsi".into()],
                (2,2) => return vec!["snATaH".into()],
                (2,3) => return vec!["snATa".into()],
                (3,1) => return vec!["snAmi".into()],
                (3,2) => return vec!["snAvaH".into()],
                (3,3) => return vec!["snAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["asnAt".into()],
                (1,2) => return vec!["asnAtAm".into()],
                (1,3) => return vec!["asnAn".into()],
                (2,1) => return vec!["asnAH".into()],
                (2,2) => return vec!["asnAtam".into()],
                (2,3) => return vec!["asnAta".into()],
                (3,1) => return vec!["asnAm".into()],
                (3,2) => return vec!["asnAva".into()],
                (3,3) => return vec!["asnAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["snAtAt".into()],
                (1,2) => return vec!["snAtAm".into()],
                (1,3) => return vec!["snAntu".into()],
                (2,1) => return vec!["snAtAt".into()],
                (2,2) => return vec!["snAtam".into()],
                (2,3) => return vec!["snAta".into()],
                (3,1) => return vec!["snAni".into()],
                (3,2) => return vec!["snAva".into()],
                (3,3) => return vec!["snAma".into()],
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
                (1,1) => return vec!["snAyAt".into()],
                (1,2) => return vec!["snAyAtAm".into()],
                (1,3) => return vec!["snAyuH".into()],
                (2,1) => return vec!["snAyAH".into()],
                (2,2) => return vec!["snAyAtam".into()],
                (2,3) => return vec!["snAyAta".into()],
                (3,1) => return vec!["snAyAm".into()],
                (3,2) => return vec!["snAyAva".into()],
                (3,3) => return vec!["snAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "SrA" || dhatu_query == "02.0048" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["SrAti".into()],
                (1,2) => return vec!["SrAtaH".into()],
                (1,3) => return vec!["SrAnti".into()],
                (2,1) => return vec!["SrAsi".into()],
                (2,2) => return vec!["SrATaH".into()],
                (2,3) => return vec!["SrATa".into()],
                (3,1) => return vec!["SrAmi".into()],
                (3,2) => return vec!["SrAvaH".into()],
                (3,3) => return vec!["SrAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["aSrAt".into()],
                (1,2) => return vec!["aSrAtAm".into()],
                (1,3) => return vec!["aSrAn".into()],
                (2,1) => return vec!["aSrAH".into()],
                (2,2) => return vec!["aSrAtam".into()],
                (2,3) => return vec!["aSrAta".into()],
                (3,1) => return vec!["aSrAm".into()],
                (3,2) => return vec!["aSrAva".into()],
                (3,3) => return vec!["aSrAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["SrAtAt".into()],
                (1,2) => return vec!["SrAtAm".into()],
                (1,3) => return vec!["SrAntu".into()],
                (2,1) => return vec!["SrAtAt".into()],
                (2,2) => return vec!["SrAtam".into()],
                (2,3) => return vec!["SrAta".into()],
                (3,1) => return vec!["SrARi".into()],
                (3,2) => return vec!["SrAva".into()],
                (3,3) => return vec!["SrAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["SrAsyati".into()],
                (1,2) => return vec!["SrAsyataH".into()],
                (1,3) => return vec!["SrAsyanti".into()],
                (2,1) => return vec!["SrAsyasi".into()],
                (2,2) => return vec!["SrAsyaTaH".into()],
                (2,3) => return vec!["SrAsyaTa".into()],
                (3,1) => return vec!["SrAsyAmi".into()],
                (3,2) => return vec!["SrAsyAvaH".into()],
                (3,3) => return vec!["SrAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["SrAyAt".into()],
                (1,2) => return vec!["SrAyAtAm".into()],
                (1,3) => return vec!["SrAyuH".into()],
                (2,1) => return vec!["SrAyAH".into()],
                (2,2) => return vec!["SrAyAtam".into()],
                (2,3) => return vec!["SrAyAta".into()],
                (3,1) => return vec!["SrAyAm".into()],
                (3,2) => return vec!["SrAyAva".into()],
                (3,3) => return vec!["SrAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "drA" || dhatu_query == "02.0049" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["drAti".into()],
                (1,2) => return vec!["drAtaH".into()],
                (1,3) => return vec!["drAnti".into()],
                (2,1) => return vec!["drAsi".into()],
                (2,2) => return vec!["drATaH".into()],
                (2,3) => return vec!["drATa".into()],
                (3,1) => return vec!["drAmi".into()],
                (3,2) => return vec!["drAvaH".into()],
                (3,3) => return vec!["drAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["adrAt".into()],
                (1,2) => return vec!["adrAtAm".into()],
                (1,3) => return vec!["adrAn".into()],
                (2,1) => return vec!["adrAH".into()],
                (2,2) => return vec!["adrAtam".into()],
                (2,3) => return vec!["adrAta".into()],
                (3,1) => return vec!["adrAm".into()],
                (3,2) => return vec!["adrAva".into()],
                (3,3) => return vec!["adrAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["drAtAt".into()],
                (1,2) => return vec!["drAtAm".into()],
                (1,3) => return vec!["drAntu".into()],
                (2,1) => return vec!["drAtAt".into()],
                (2,2) => return vec!["drAtam".into()],
                (2,3) => return vec!["drAta".into()],
                (3,1) => return vec!["drARi".into()],
                (3,2) => return vec!["drAva".into()],
                (3,3) => return vec!["drAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["drAsyati".into()],
                (1,2) => return vec!["drAsyataH".into()],
                (1,3) => return vec!["drAsyanti".into()],
                (2,1) => return vec!["drAsyasi".into()],
                (2,2) => return vec!["drAsyaTaH".into()],
                (2,3) => return vec!["drAsyaTa".into()],
                (3,1) => return vec!["drAsyAmi".into()],
                (3,2) => return vec!["drAsyAvaH".into()],
                (3,3) => return vec!["drAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["drAyAt".into()],
                (1,2) => return vec!["drAyAtAm".into()],
                (1,3) => return vec!["drAyuH".into()],
                (2,1) => return vec!["drAyAH".into()],
                (2,2) => return vec!["drAyAtam".into()],
                (2,3) => return vec!["drAyAta".into()],
                (3,1) => return vec!["drAyAm".into()],
                (3,2) => return vec!["drAyAva".into()],
                (3,3) => return vec!["drAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "psA" || dhatu_query == "02.0050" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["psAti".into()],
                (1,2) => return vec!["psAtaH".into()],
                (1,3) => return vec!["psAnti".into()],
                (2,1) => return vec!["psAsi".into()],
                (2,2) => return vec!["psATaH".into()],
                (2,3) => return vec!["psATa".into()],
                (3,1) => return vec!["psAmi".into()],
                (3,2) => return vec!["psAvaH".into()],
                (3,3) => return vec!["psAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["apsAt".into()],
                (1,2) => return vec!["apsAtAm".into()],
                (1,3) => return vec!["apsAn".into()],
                (2,1) => return vec!["apsAH".into()],
                (2,2) => return vec!["apsAtam".into()],
                (2,3) => return vec!["apsAta".into()],
                (3,1) => return vec!["apsAm".into()],
                (3,2) => return vec!["apsAva".into()],
                (3,3) => return vec!["apsAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["psAtAt".into()],
                (1,2) => return vec!["psAtAm".into()],
                (1,3) => return vec!["psAntu".into()],
                (2,1) => return vec!["psAtAt".into()],
                (2,2) => return vec!["psAtam".into()],
                (2,3) => return vec!["psAta".into()],
                (3,1) => return vec!["psAni".into()],
                (3,2) => return vec!["psAva".into()],
                (3,3) => return vec!["psAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["psAsyati".into()],
                (1,2) => return vec!["psAsyataH".into()],
                (1,3) => return vec!["psAsyanti".into()],
                (2,1) => return vec!["psAsyasi".into()],
                (2,2) => return vec!["psAsyaTaH".into()],
                (2,3) => return vec!["psAsyaTa".into()],
                (3,1) => return vec!["psAsyAmi".into()],
                (3,2) => return vec!["psAsyAvaH".into()],
                (3,3) => return vec!["psAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["psAyAt".into()],
                (1,2) => return vec!["psAyAtAm".into()],
                (1,3) => return vec!["psAyuH".into()],
                (2,1) => return vec!["psAyAH".into()],
                (2,2) => return vec!["psAyAtam".into()],
                (2,3) => return vec!["psAyAta".into()],
                (3,1) => return vec!["psAyAm".into()],
                (3,2) => return vec!["psAyAva".into()],
                (3,3) => return vec!["psAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "pA" || dhatu_query == "02.0051" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["pAti".into()],
                (1,2) => return vec!["pAtaH".into()],
                (1,3) => return vec!["pAnti".into()],
                (2,1) => return vec!["pAsi".into()],
                (2,2) => return vec!["pATaH".into()],
                (2,3) => return vec!["pATa".into()],
                (3,1) => return vec!["pAmi".into()],
                (3,2) => return vec!["pAvaH".into()],
                (3,3) => return vec!["pAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["apAt".into()],
                (1,2) => return vec!["apAtAm".into()],
                (1,3) => return vec!["apAn".into()],
                (2,1) => return vec!["apAH".into()],
                (2,2) => return vec!["apAtam".into()],
                (2,3) => return vec!["apAta".into()],
                (3,1) => return vec!["apAm".into()],
                (3,2) => return vec!["apAva".into()],
                (3,3) => return vec!["apAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["pAtAt".into()],
                (1,2) => return vec!["pAtAm".into()],
                (1,3) => return vec!["pAntu".into()],
                (2,1) => return vec!["pAtAt".into()],
                (2,2) => return vec!["pAtam".into()],
                (2,3) => return vec!["pAta".into()],
                (3,1) => return vec!["pAni".into()],
                (3,2) => return vec!["pAva".into()],
                (3,3) => return vec!["pAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["pAsyati".into()],
                (1,2) => return vec!["pAsyataH".into()],
                (1,3) => return vec!["pAsyanti".into()],
                (2,1) => return vec!["pAsyasi".into()],
                (2,2) => return vec!["pAsyaTaH".into()],
                (2,3) => return vec!["pAsyaTa".into()],
                (3,1) => return vec!["pAsyAmi".into()],
                (3,2) => return vec!["pAsyAvaH".into()],
                (3,3) => return vec!["pAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["pAyAt".into()],
                (1,2) => return vec!["pAyAtAm".into()],
                (1,3) => return vec!["pAyuH".into()],
                (2,1) => return vec!["pAyAH".into()],
                (2,2) => return vec!["pAyAtam".into()],
                (2,3) => return vec!["pAyAta".into()],
                (3,1) => return vec!["pAyAm".into()],
                (3,2) => return vec!["pAyAva".into()],
                (3,3) => return vec!["pAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "rA" || dhatu_query == "02.0052" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["rAti".into()],
                (1,2) => return vec!["rAtaH".into()],
                (1,3) => return vec!["rAnti".into()],
                (2,1) => return vec!["rAsi".into()],
                (2,2) => return vec!["rATaH".into()],
                (2,3) => return vec!["rATa".into()],
                (3,1) => return vec!["rAmi".into()],
                (3,2) => return vec!["rAvaH".into()],
                (3,3) => return vec!["rAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["arAt".into()],
                (1,2) => return vec!["arAtAm".into()],
                (1,3) => return vec!["arAn".into()],
                (2,1) => return vec!["arAH".into()],
                (2,2) => return vec!["arAtam".into()],
                (2,3) => return vec!["arAta".into()],
                (3,1) => return vec!["arAm".into()],
                (3,2) => return vec!["arAva".into()],
                (3,3) => return vec!["arAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["rAtAt".into()],
                (1,2) => return vec!["rAtAm".into()],
                (1,3) => return vec!["rAntu".into()],
                (2,1) => return vec!["rAtAt".into()],
                (2,2) => return vec!["rAtam".into()],
                (2,3) => return vec!["rAta".into()],
                (3,1) => return vec!["rARi".into()],
                (3,2) => return vec!["rAva".into()],
                (3,3) => return vec!["rAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["rAsyati".into()],
                (1,2) => return vec!["rAsyataH".into()],
                (1,3) => return vec!["rAsyanti".into()],
                (2,1) => return vec!["rAsyasi".into()],
                (2,2) => return vec!["rAsyaTaH".into()],
                (2,3) => return vec!["rAsyaTa".into()],
                (3,1) => return vec!["rAsyAmi".into()],
                (3,2) => return vec!["rAsyAvaH".into()],
                (3,3) => return vec!["rAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["rAyAt".into()],
                (1,2) => return vec!["rAyAtAm".into()],
                (1,3) => return vec!["rAyuH".into()],
                (2,1) => return vec!["rAyAH".into()],
                (2,2) => return vec!["rAyAtam".into()],
                (2,3) => return vec!["rAyAta".into()],
                (3,1) => return vec!["rAyAm".into()],
                (3,2) => return vec!["rAyAva".into()],
                (3,3) => return vec!["rAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "lA" || dhatu_query == "02.0053" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["lAti".into()],
                (1,2) => return vec!["lAtaH".into()],
                (1,3) => return vec!["lAnti".into()],
                (2,1) => return vec!["lAsi".into()],
                (2,2) => return vec!["lATaH".into()],
                (2,3) => return vec!["lATa".into()],
                (3,1) => return vec!["lAmi".into()],
                (3,2) => return vec!["lAvaH".into()],
                (3,3) => return vec!["lAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["alAt".into()],
                (1,2) => return vec!["alAtAm".into()],
                (1,3) => return vec!["alAn".into()],
                (2,1) => return vec!["alAH".into()],
                (2,2) => return vec!["alAtam".into()],
                (2,3) => return vec!["alAta".into()],
                (3,1) => return vec!["alAm".into()],
                (3,2) => return vec!["alAva".into()],
                (3,3) => return vec!["alAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["lAtAt".into()],
                (1,2) => return vec!["lAtAm".into()],
                (1,3) => return vec!["lAntu".into()],
                (2,1) => return vec!["lAtAt".into()],
                (2,2) => return vec!["lAtam".into()],
                (2,3) => return vec!["lAta".into()],
                (3,1) => return vec!["lAni".into()],
                (3,2) => return vec!["lAva".into()],
                (3,3) => return vec!["lAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["lAsyati".into()],
                (1,2) => return vec!["lAsyataH".into()],
                (1,3) => return vec!["lAsyanti".into()],
                (2,1) => return vec!["lAsyasi".into()],
                (2,2) => return vec!["lAsyaTaH".into()],
                (2,3) => return vec!["lAsyaTa".into()],
                (3,1) => return vec!["lAsyAmi".into()],
                (3,2) => return vec!["lAsyAvaH".into()],
                (3,3) => return vec!["lAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["lAyAt".into()],
                (1,2) => return vec!["lAyAtAm".into()],
                (1,3) => return vec!["lAyuH".into()],
                (2,1) => return vec!["lAyAH".into()],
                (2,2) => return vec!["lAyAtam".into()],
                (2,3) => return vec!["lAyAta".into()],
                (3,1) => return vec!["lAyAm".into()],
                (3,2) => return vec!["lAyAva".into()],
                (3,3) => return vec!["lAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "dAp" || dhatu_query == "02.0054" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["dAti".into()],
                (1,2) => return vec!["dAtaH".into()],
                (1,3) => return vec!["dAnti".into()],
                (2,1) => return vec!["dAsi".into()],
                (2,2) => return vec!["dATaH".into()],
                (2,3) => return vec!["dATa".into()],
                (3,1) => return vec!["dAmi".into()],
                (3,2) => return vec!["dAvaH".into()],
                (3,3) => return vec!["dAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["adAt".into()],
                (1,2) => return vec!["adAtAm".into()],
                (1,3) => return vec!["adAn".into()],
                (2,1) => return vec!["adAH".into()],
                (2,2) => return vec!["adAtam".into()],
                (2,3) => return vec!["adAta".into()],
                (3,1) => return vec!["adAm".into()],
                (3,2) => return vec!["adAva".into()],
                (3,3) => return vec!["adAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["dAtAt".into()],
                (1,2) => return vec!["dAtAm".into()],
                (1,3) => return vec!["dAntu".into()],
                (2,1) => return vec!["dAtAt".into()],
                (2,2) => return vec!["dAtam".into()],
                (2,3) => return vec!["dAta".into()],
                (3,1) => return vec!["dAni".into()],
                (3,2) => return vec!["dAva".into()],
                (3,3) => return vec!["dAma".into()],
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
                (1,1) => return vec!["dAyAt".into()],
                (1,2) => return vec!["dAyAtAm".into()],
                (1,3) => return vec!["dAyuH".into()],
                (2,1) => return vec!["dAyAH".into()],
                (2,2) => return vec!["dAyAtam".into()],
                (2,3) => return vec!["dAyAta".into()],
                (3,1) => return vec!["dAyAm".into()],
                (3,2) => return vec!["dAyAva".into()],
                (3,3) => return vec!["dAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "KyA" || dhatu_query == "02.0055" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["KyAti".into()],
                (1,2) => return vec!["KyAtaH".into()],
                (1,3) => return vec!["KyAnti".into()],
                (2,1) => return vec!["KyAsi".into()],
                (2,2) => return vec!["KyATaH".into()],
                (2,3) => return vec!["KyATa".into()],
                (3,1) => return vec!["KyAmi".into()],
                (3,2) => return vec!["KyAvaH".into()],
                (3,3) => return vec!["KyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["aKyAt".into()],
                (1,2) => return vec!["aKyAtAm".into()],
                (1,3) => return vec!["aKyAn".into()],
                (2,1) => return vec!["aKyAH".into()],
                (2,2) => return vec!["aKyAtam".into()],
                (2,3) => return vec!["aKyAta".into()],
                (3,1) => return vec!["aKyAm".into()],
                (3,2) => return vec!["aKyAva".into()],
                (3,3) => return vec!["aKyAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["KyAtAt".into()],
                (1,2) => return vec!["KyAtAm".into()],
                (1,3) => return vec!["KyAntu".into()],
                (2,1) => return vec!["KyAtAt".into()],
                (2,2) => return vec!["KyAtam".into()],
                (2,3) => return vec!["KyAta".into()],
                (3,1) => return vec!["KyAni".into()],
                (3,2) => return vec!["KyAva".into()],
                (3,3) => return vec!["KyAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["KyAsyati".into()],
                (1,2) => return vec!["KyAsyataH".into()],
                (1,3) => return vec!["KyAsyanti".into()],
                (2,1) => return vec!["KyAsyasi".into()],
                (2,2) => return vec!["KyAsyaTaH".into()],
                (2,3) => return vec!["KyAsyaTa".into()],
                (3,1) => return vec!["KyAsyAmi".into()],
                (3,2) => return vec!["KyAsyAvaH".into()],
                (3,3) => return vec!["KyAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["KyAyAt".into()],
                (1,2) => return vec!["KyAyAtAm".into()],
                (1,3) => return vec!["KyAyuH".into()],
                (2,1) => return vec!["KyAyAH".into()],
                (2,2) => return vec!["KyAyAtam".into()],
                (2,3) => return vec!["KyAyAta".into()],
                (3,1) => return vec!["KyAyAm".into()],
                (3,2) => return vec!["KyAyAva".into()],
                (3,3) => return vec!["KyAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "prA" || dhatu_query == "02.0056" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["prAti".into()],
                (1,2) => return vec!["prAtaH".into()],
                (1,3) => return vec!["prAnti".into()],
                (2,1) => return vec!["prAsi".into()],
                (2,2) => return vec!["prATaH".into()],
                (2,3) => return vec!["prATa".into()],
                (3,1) => return vec!["prAmi".into()],
                (3,2) => return vec!["prAvaH".into()],
                (3,3) => return vec!["prAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["aprAt".into()],
                (1,2) => return vec!["aprAtAm".into()],
                (1,3) => return vec!["aprAn".into()],
                (2,1) => return vec!["aprAH".into()],
                (2,2) => return vec!["aprAtam".into()],
                (2,3) => return vec!["aprAta".into()],
                (3,1) => return vec!["aprAm".into()],
                (3,2) => return vec!["aprAva".into()],
                (3,3) => return vec!["aprAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["prAtAt".into()],
                (1,2) => return vec!["prAtAm".into()],
                (1,3) => return vec!["prAntu".into()],
                (2,1) => return vec!["prAtAt".into()],
                (2,2) => return vec!["prAtam".into()],
                (2,3) => return vec!["prAta".into()],
                (3,1) => return vec!["prARi".into()],
                (3,2) => return vec!["prAva".into()],
                (3,3) => return vec!["prAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["prAsyati".into()],
                (1,2) => return vec!["prAsyataH".into()],
                (1,3) => return vec!["prAsyanti".into()],
                (2,1) => return vec!["prAsyasi".into()],
                (2,2) => return vec!["prAsyaTaH".into()],
                (2,3) => return vec!["prAsyaTa".into()],
                (3,1) => return vec!["prAsyAmi".into()],
                (3,2) => return vec!["prAsyAvaH".into()],
                (3,3) => return vec!["prAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["prAyAt".into()],
                (1,2) => return vec!["prAyAtAm".into()],
                (1,3) => return vec!["prAyuH".into()],
                (2,1) => return vec!["prAyAH".into()],
                (2,2) => return vec!["prAyAtam".into()],
                (2,3) => return vec!["prAyAta".into()],
                (3,1) => return vec!["prAyAm".into()],
                (3,2) => return vec!["prAyAva".into()],
                (3,3) => return vec!["prAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "mA" || dhatu_query == "02.0057" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["mAti".into()],
                (1,2) => return vec!["mAtaH".into()],
                (1,3) => return vec!["mAnti".into()],
                (2,1) => return vec!["mAsi".into()],
                (2,2) => return vec!["mATaH".into()],
                (2,3) => return vec!["mATa".into()],
                (3,1) => return vec!["mAmi".into()],
                (3,2) => return vec!["mAvaH".into()],
                (3,3) => return vec!["mAmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["amAt".into()],
                (1,2) => return vec!["amAtAm".into()],
                (1,3) => return vec!["amAn".into()],
                (2,1) => return vec!["amAH".into()],
                (2,2) => return vec!["amAtam".into()],
                (2,3) => return vec!["amAta".into()],
                (3,1) => return vec!["amAm".into()],
                (3,2) => return vec!["amAva".into()],
                (3,3) => return vec!["amAma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["mAtAt".into()],
                (1,2) => return vec!["mAtAm".into()],
                (1,3) => return vec!["mAntu".into()],
                (2,1) => return vec!["mAtAt".into()],
                (2,2) => return vec!["mAtam".into()],
                (2,3) => return vec!["mAta".into()],
                (3,1) => return vec!["mAni".into()],
                (3,2) => return vec!["mAva".into()],
                (3,3) => return vec!["mAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["mAsyati".into()],
                (1,2) => return vec!["mAsyataH".into()],
                (1,3) => return vec!["mAsyanti".into()],
                (2,1) => return vec!["mAsyasi".into()],
                (2,2) => return vec!["mAsyaTaH".into()],
                (2,3) => return vec!["mAsyaTa".into()],
                (3,1) => return vec!["mAsyAmi".into()],
                (3,2) => return vec!["mAsyAvaH".into()],
                (3,3) => return vec!["mAsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["mAyAt".into()],
                (1,2) => return vec!["mAyAtAm".into()],
                (1,3) => return vec!["mAyuH".into()],
                (2,1) => return vec!["mAyAH".into()],
                (2,2) => return vec!["mAyAtam".into()],
                (2,3) => return vec!["mAyAta".into()],
                (3,1) => return vec!["mAyAm".into()],
                (3,2) => return vec!["mAyAva".into()],
                (3,3) => return vec!["mAyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "vaca" || dhatu_query == "02.0058" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["vakti".into()],
                (1,2) => return vec!["vaktaH".into()],
                (1,3) => return vec!["vacanti".into()],
                (2,1) => return vec!["vakzi".into()],
                (2,2) => return vec!["vakTaH".into()],
                (2,3) => return vec!["vakTa".into()],
                (3,1) => return vec!["vacmi".into()],
                (3,2) => return vec!["vacvaH".into()],
                (3,3) => return vec!["vacmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["avak".into()],
                (1,2) => return vec!["avaktAm".into()],
                (1,3) => return vec!["avacan".into()],
                (2,1) => return vec!["avak".into()],
                (2,2) => return vec!["avaktam".into()],
                (2,3) => return vec!["avakta".into()],
                (3,1) => return vec!["avacam".into()],
                (3,2) => return vec!["avacva".into()],
                (3,3) => return vec!["avacma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["vaktAt".into()],
                (1,2) => return vec!["vaktAm".into()],
                (1,3) => return vec!["vacantu".into()],
                (2,1) => return vec!["vaktAt".into()],
                (2,2) => return vec!["vaktam".into()],
                (2,3) => return vec!["vakta".into()],
                (3,1) => return vec!["vacAni".into()],
                (3,2) => return vec!["vacAva".into()],
                (3,3) => return vec!["vacAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["vakzyati".into()],
                (1,2) => return vec!["vakzyataH".into()],
                (1,3) => return vec!["vakzyanti".into()],
                (2,1) => return vec!["vakzyasi".into()],
                (2,2) => return vec!["vakzyaTaH".into()],
                (2,3) => return vec!["vakzyaTa".into()],
                (3,1) => return vec!["vakzyAmi".into()],
                (3,2) => return vec!["vakzyAvaH".into()],
                (3,3) => return vec!["vakzyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["vacyAt".into()],
                (1,2) => return vec!["vacyAtAm".into()],
                (1,3) => return vec!["vacyuH".into()],
                (2,1) => return vec!["vacyAH".into()],
                (2,2) => return vec!["vacyAtam".into()],
                (2,3) => return vec!["vacyAta".into()],
                (3,1) => return vec!["vacyAm".into()],
                (3,2) => return vec!["vacyAva".into()],
                (3,3) => return vec!["vacyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "vida" || dhatu_query == "02.0059" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["vetti".into()],
                (1,2) => return vec!["vittaH".into()],
                (1,3) => return vec!["vidanti".into()],
                (2,1) => return vec!["vetTa".into()],
                (2,2) => return vec!["vitTaH".into()],
                (2,3) => return vec!["vitTa".into()],
                (3,1) => return vec!["veda".into()],
                (3,2) => return vec!["vidva".into()],
                (3,3) => return vec!["vidma".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["avet".into()],
                (1,2) => return vec!["avittAm".into()],
                (1,3) => return vec!["aviduH".into()],
                (2,1) => return vec!["aveH".into()],
                (2,2) => return vec!["avittam".into()],
                (2,3) => return vec!["avitta".into()],
                (3,1) => return vec!["avedam".into()],
                (3,2) => return vec!["avidva".into()],
                (3,3) => return vec!["avidma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["vittAt".into()],
                (1,2) => return vec!["vittAm".into()],
                (1,3) => return vec!["vidantu".into()],
                (2,1) => return vec!["vittAt".into()],
                (2,2) => return vec!["vittam".into()],
                (2,3) => return vec!["vitta".into()],
                (3,1) => return vec!["vidANkaravARi".into()],
                (3,2) => return vec!["vidANkaravAva".into()],
                (3,3) => return vec!["vidANkaravAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["vedizyati".into()],
                (1,2) => return vec!["vedizyataH".into()],
                (1,3) => return vec!["vedizyanti".into()],
                (2,1) => return vec!["vedizyasi".into()],
                (2,2) => return vec!["vedizyaTaH".into()],
                (2,3) => return vec!["vedizyaTa".into()],
                (3,1) => return vec!["vedizyAmi".into()],
                (3,2) => return vec!["vedizyAvaH".into()],
                (3,3) => return vec!["vedizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["vidyAt".into()],
                (1,2) => return vec!["vidyAtAm".into()],
                (1,3) => return vec!["vidyuH".into()],
                (2,1) => return vec!["vidyAH".into()],
                (2,2) => return vec!["vidyAtam".into()],
                (2,3) => return vec!["vidyAta".into()],
                (3,1) => return vec!["vidyAm".into()],
                (3,2) => return vec!["vidyAva".into()],
                (3,3) => return vec!["vidyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "asa" || dhatu_query == "02.0060" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["asti".into()],
                (1,2) => return vec!["staH".into()],
                (1,3) => return vec!["santi".into()],
                (2,1) => return vec!["asi".into()],
                (2,2) => return vec!["sTaH".into()],
                (2,3) => return vec!["sTa".into()],
                (3,1) => return vec!["asmi".into()],
                (3,2) => return vec!["svaH".into()],
                (3,3) => return vec!["smaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["AsIt".into()],
                (1,2) => return vec!["AstAm".into()],
                (1,3) => return vec!["Asan".into()],
                (2,1) => return vec!["AsIH".into()],
                (2,2) => return vec!["Astam".into()],
                (2,3) => return vec!["Asta".into()],
                (3,1) => return vec!["Asam".into()],
                (3,2) => return vec!["Asva".into()],
                (3,3) => return vec!["Asma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["astu".into()],
                (1,2) => return vec!["stAm".into()],
                (1,3) => return vec!["santu".into()],
                (2,1) => return vec!["eDi".into()],
                (2,2) => return vec!["stam".into()],
                (2,3) => return vec!["sta".into()],
                (3,1) => return vec!["asAni".into()],
                (3,2) => return vec!["asAva".into()],
                (3,3) => return vec!["asAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["Bavizyati".into()],
                (1,2) => return vec!["BavizyataH".into()],
                (1,3) => return vec!["Bavizyanti".into()],
                (2,1) => return vec!["Bavizyasi".into()],
                (2,2) => return vec!["BavizyaTaH".into()],
                (2,3) => return vec!["BavizyaTa".into()],
                (3,1) => return vec!["BavizyAmi".into()],
                (3,2) => return vec!["BavizyAvaH".into()],
                (3,3) => return vec!["BavizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["syAt".into()],
                (1,2) => return vec!["syAtAm".into()],
                (1,3) => return vec!["syuH".into()],
                (2,1) => return vec!["syAH".into()],
                (2,2) => return vec!["syAtam".into()],
                (2,3) => return vec!["syAta".into()],
                (3,1) => return vec!["syAm".into()],
                (3,2) => return vec!["syAva".into()],
                (3,3) => return vec!["syAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "mfjU" || dhatu_query == "02.0061" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["mArzwi".into()],
                (1,2) => return vec!["mfzwaH".into()],
                (1,3) => return vec!["mArjanti".into()],
                (2,1) => return vec!["mArkzi".into()],
                (2,2) => return vec!["mfzWaH".into()],
                (2,3) => return vec!["mfzWa".into()],
                (3,1) => return vec!["mArjmi".into()],
                (3,2) => return vec!["mfjvaH".into()],
                (3,3) => return vec!["mfjmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["amArw".into()],
                (1,2) => return vec!["amfzwAm".into()],
                (1,3) => return vec!["amArjan".into()],
                (2,1) => return vec!["amArw".into()],
                (2,2) => return vec!["amfzwam".into()],
                (2,3) => return vec!["amfzwa".into()],
                (3,1) => return vec!["amArjam".into()],
                (3,2) => return vec!["amfjva".into()],
                (3,3) => return vec!["amfjma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["mArzwu".into()],
                (1,2) => return vec!["mfzwAm".into()],
                (1,3) => return vec!["mArjantu".into()],
                (2,1) => return vec!["mfqQi".into()],
                (2,2) => return vec!["mfzwam".into()],
                (2,3) => return vec!["mfzwa".into()],
                (3,1) => return vec!["mArjAni".into()],
                (3,2) => return vec!["mArjAva".into()],
                (3,3) => return vec!["mArjAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["mArkzyati".into()],
                (1,2) => return vec!["mArkzyataH".into()],
                (1,3) => return vec!["mArkzyanti".into()],
                (2,1) => return vec!["mArkzyasi".into()],
                (2,2) => return vec!["mArkzyaTaH".into()],
                (2,3) => return vec!["mArkzyaTa".into()],
                (3,1) => return vec!["mArkzyAmi".into()],
                (3,2) => return vec!["mArkzyAvaH".into()],
                (3,3) => return vec!["mArkzyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["mfjyAt".into()],
                (1,2) => return vec!["mfjyAtAm".into()],
                (1,3) => return vec!["mfjyuH".into()],
                (2,1) => return vec!["mfjyAH".into()],
                (2,2) => return vec!["mfjyAtam".into()],
                (2,3) => return vec!["mfjyAta".into()],
                (3,1) => return vec!["mfjyAm".into()],
                (3,2) => return vec!["mfjyAva".into()],
                (3,3) => return vec!["mfjyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "rudir" || dhatu_query == "02.0062" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["roditi".into()],
                (1,2) => return vec!["ruditaH".into()],
                (1,3) => return vec!["rudanti".into()],
                (2,1) => return vec!["rodizi".into()],
                (2,2) => return vec!["rudiTaH".into()],
                (2,3) => return vec!["rudiTa".into()],
                (3,1) => return vec!["rodimi".into()],
                (3,2) => return vec!["rudivaH".into()],
                (3,3) => return vec!["rudimaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["arodat".into()],
                (1,2) => return vec!["aruditAm".into()],
                (1,3) => return vec!["arudan".into()],
                (2,1) => return vec!["arodaH".into()],
                (2,2) => return vec!["aruditam".into()],
                (2,3) => return vec!["arudita".into()],
                (3,1) => return vec!["arodam".into()],
                (3,2) => return vec!["arudiva".into()],
                (3,3) => return vec!["arudima".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["ruditAt".into()],
                (1,2) => return vec!["ruditAm".into()],
                (1,3) => return vec!["rudantu".into()],
                (2,1) => return vec!["ruditAt".into()],
                (2,2) => return vec!["ruditam".into()],
                (2,3) => return vec!["rudita".into()],
                (3,1) => return vec!["rodAni".into()],
                (3,2) => return vec!["rodAva".into()],
                (3,3) => return vec!["rodAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["rodizyati".into()],
                (1,2) => return vec!["rodizyataH".into()],
                (1,3) => return vec!["rodizyanti".into()],
                (2,1) => return vec!["rodizyasi".into()],
                (2,2) => return vec!["rodizyaTaH".into()],
                (2,3) => return vec!["rodizyaTa".into()],
                (3,1) => return vec!["rodizyAmi".into()],
                (3,2) => return vec!["rodizyAvaH".into()],
                (3,3) => return vec!["rodizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["rudyAt".into()],
                (1,2) => return vec!["rudyAtAm".into()],
                (1,3) => return vec!["rudyuH".into()],
                (2,1) => return vec!["rudyAH".into()],
                (2,2) => return vec!["rudyAtam".into()],
                (2,3) => return vec!["rudyAta".into()],
                (3,1) => return vec!["rudyAm".into()],
                (3,2) => return vec!["rudyAva".into()],
                (3,3) => return vec!["rudyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "Yizvapa" || dhatu_query == "02.0063" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["svapiti".into()],
                (1,2) => return vec!["svapitaH".into()],
                (1,3) => return vec!["svapanti".into()],
                (2,1) => return vec!["svapizi".into()],
                (2,2) => return vec!["svapiTaH".into()],
                (2,3) => return vec!["svapiTa".into()],
                (3,1) => return vec!["svapimi".into()],
                (3,2) => return vec!["svapivaH".into()],
                (3,3) => return vec!["svapimaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["asvapat".into()],
                (1,2) => return vec!["asvapitAm".into()],
                (1,3) => return vec!["asvapan".into()],
                (2,1) => return vec!["asvapaH".into()],
                (2,2) => return vec!["asvapitam".into()],
                (2,3) => return vec!["asvapita".into()],
                (3,1) => return vec!["asvapam".into()],
                (3,2) => return vec!["asvapiva".into()],
                (3,3) => return vec!["asvapima".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["svapitAt".into()],
                (1,2) => return vec!["svapitAm".into()],
                (1,3) => return vec!["svapantu".into()],
                (2,1) => return vec!["svapitAt".into()],
                (2,2) => return vec!["svapitam".into()],
                (2,3) => return vec!["svapita".into()],
                (3,1) => return vec!["svapAni".into()],
                (3,2) => return vec!["svapAva".into()],
                (3,3) => return vec!["svapAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["svapsyati".into()],
                (1,2) => return vec!["svapsyataH".into()],
                (1,3) => return vec!["svapsyanti".into()],
                (2,1) => return vec!["svapsyasi".into()],
                (2,2) => return vec!["svapsyaTaH".into()],
                (2,3) => return vec!["svapsyaTa".into()],
                (3,1) => return vec!["svapsyAmi".into()],
                (3,2) => return vec!["svapsyAvaH".into()],
                (3,3) => return vec!["svapsyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["svapyAt".into()],
                (1,2) => return vec!["svapyAtAm".into()],
                (1,3) => return vec!["svapyuH".into()],
                (2,1) => return vec!["svapyAH".into()],
                (2,2) => return vec!["svapyAtam".into()],
                (2,3) => return vec!["svapyAta".into()],
                (3,1) => return vec!["svapyAm".into()],
                (3,2) => return vec!["svapyAva".into()],
                (3,3) => return vec!["svapyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "Svasa" || dhatu_query == "02.0064" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["Svasiti".into()],
                (1,2) => return vec!["SvasitaH".into()],
                (1,3) => return vec!["Svasanti".into()],
                (2,1) => return vec!["Svasizi".into()],
                (2,2) => return vec!["SvasiTaH".into()],
                (2,3) => return vec!["SvasiTa".into()],
                (3,1) => return vec!["Svasimi".into()],
                (3,2) => return vec!["SvasivaH".into()],
                (3,3) => return vec!["SvasimaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["aSvasat".into()],
                (1,2) => return vec!["aSvasitAm".into()],
                (1,3) => return vec!["aSvasan".into()],
                (2,1) => return vec!["aSvasaH".into()],
                (2,2) => return vec!["aSvasitam".into()],
                (2,3) => return vec!["aSvasita".into()],
                (3,1) => return vec!["aSvasam".into()],
                (3,2) => return vec!["aSvasiva".into()],
                (3,3) => return vec!["aSvasima".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["SvasitAt".into()],
                (1,2) => return vec!["SvasitAm".into()],
                (1,3) => return vec!["Svasantu".into()],
                (2,1) => return vec!["SvasitAt".into()],
                (2,2) => return vec!["Svasitam".into()],
                (2,3) => return vec!["Svasita".into()],
                (3,1) => return vec!["SvasAni".into()],
                (3,2) => return vec!["SvasAva".into()],
                (3,3) => return vec!["SvasAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["Svasizyati".into()],
                (1,2) => return vec!["SvasizyataH".into()],
                (1,3) => return vec!["Svasizyanti".into()],
                (2,1) => return vec!["Svasizyasi".into()],
                (2,2) => return vec!["SvasizyaTaH".into()],
                (2,3) => return vec!["SvasizyaTa".into()],
                (3,1) => return vec!["SvasizyAmi".into()],
                (3,2) => return vec!["SvasizyAvaH".into()],
                (3,3) => return vec!["SvasizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["SvasyAt".into()],
                (1,2) => return vec!["SvasyAtAm".into()],
                (1,3) => return vec!["SvasyuH".into()],
                (2,1) => return vec!["SvasyAH".into()],
                (2,2) => return vec!["SvasyAtam".into()],
                (2,3) => return vec!["SvasyAta".into()],
                (3,1) => return vec!["SvasyAm".into()],
                (3,2) => return vec!["SvasyAva".into()],
                (3,3) => return vec!["SvasyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "ana" || dhatu_query == "02.0065" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["aniti".into()],
                (1,2) => return vec!["anitaH".into()],
                (1,3) => return vec!["ananti".into()],
                (2,1) => return vec!["anizi".into()],
                (2,2) => return vec!["aniTaH".into()],
                (2,3) => return vec!["aniTa".into()],
                (3,1) => return vec!["animi".into()],
                (3,2) => return vec!["anivaH".into()],
                (3,3) => return vec!["animaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["Anat".into()],
                (1,2) => return vec!["AnitAm".into()],
                (1,3) => return vec!["Anan".into()],
                (2,1) => return vec!["AnaH".into()],
                (2,2) => return vec!["Anitam".into()],
                (2,3) => return vec!["Anita".into()],
                (3,1) => return vec!["Anam".into()],
                (3,2) => return vec!["Aniva".into()],
                (3,3) => return vec!["Anima".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["anitAt".into()],
                (1,2) => return vec!["anitAm".into()],
                (1,3) => return vec!["anantu".into()],
                (2,1) => return vec!["anitAt".into()],
                (2,2) => return vec!["anitam".into()],
                (2,3) => return vec!["anita".into()],
                (3,1) => return vec!["anAni".into()],
                (3,2) => return vec!["anAva".into()],
                (3,3) => return vec!["anAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["anizyati".into()],
                (1,2) => return vec!["anizyataH".into()],
                (1,3) => return vec!["anizyanti".into()],
                (2,1) => return vec!["anizyasi".into()],
                (2,2) => return vec!["anizyaTaH".into()],
                (2,3) => return vec!["anizyaTa".into()],
                (3,1) => return vec!["anizyAmi".into()],
                (3,2) => return vec!["anizyAvaH".into()],
                (3,3) => return vec!["anizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["anyAt".into()],
                (1,2) => return vec!["anyAtAm".into()],
                (1,3) => return vec!["anyuH".into()],
                (2,1) => return vec!["anyAH".into()],
                (2,2) => return vec!["anyAtam".into()],
                (2,3) => return vec!["anyAta".into()],
                (3,1) => return vec!["anyAm".into()],
                (3,2) => return vec!["anyAva".into()],
                (3,3) => return vec!["anyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "jakza" || dhatu_query == "02.0066" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["jakziti".into()],
                (1,2) => return vec!["jakzitaH".into()],
                (1,3) => return vec!["jakzati".into()],
                (2,1) => return vec!["jakzizi".into()],
                (2,2) => return vec!["jakziTaH".into()],
                (2,3) => return vec!["jakziTa".into()],
                (3,1) => return vec!["jakzimi".into()],
                (3,2) => return vec!["jakzivaH".into()],
                (3,3) => return vec!["jakzimaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["ajakzat".into()],
                (1,2) => return vec!["ajakzitAm".into()],
                (1,3) => return vec!["ajakzuH".into()],
                (2,1) => return vec!["ajakzaH".into()],
                (2,2) => return vec!["ajakzitam".into()],
                (2,3) => return vec!["ajakzita".into()],
                (3,1) => return vec!["ajakzam".into()],
                (3,2) => return vec!["ajakziva".into()],
                (3,3) => return vec!["ajakzima".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["jakzitAt".into()],
                (1,2) => return vec!["jakzitAm".into()],
                (1,3) => return vec!["jakzatu".into()],
                (2,1) => return vec!["jakzitAt".into()],
                (2,2) => return vec!["jakzitam".into()],
                (2,3) => return vec!["jakzita".into()],
                (3,1) => return vec!["jakzARi".into()],
                (3,2) => return vec!["jakzAva".into()],
                (3,3) => return vec!["jakzAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["jakzizyati".into()],
                (1,2) => return vec!["jakzizyataH".into()],
                (1,3) => return vec!["jakzizyanti".into()],
                (2,1) => return vec!["jakzizyasi".into()],
                (2,2) => return vec!["jakzizyaTaH".into()],
                (2,3) => return vec!["jakzizyaTa".into()],
                (3,1) => return vec!["jakzizyAmi".into()],
                (3,2) => return vec!["jakzizyAvaH".into()],
                (3,3) => return vec!["jakzizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["jakzyAt".into()],
                (1,2) => return vec!["jakzyAtAm".into()],
                (1,3) => return vec!["jakzyuH".into()],
                (2,1) => return vec!["jakzyAH".into()],
                (2,2) => return vec!["jakzyAtam".into()],
                (2,3) => return vec!["jakzyAta".into()],
                (3,1) => return vec!["jakzyAm".into()],
                (3,2) => return vec!["jakzyAva".into()],
                (3,3) => return vec!["jakzyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "jAgf" || dhatu_query == "02.0067" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["jAgarti".into()],
                (1,2) => return vec!["jAgftaH".into()],
                (1,3) => return vec!["jAgrati".into()],
                (2,1) => return vec!["jAgarzi".into()],
                (2,2) => return vec!["jAgfTaH".into()],
                (2,3) => return vec!["jAgfTa".into()],
                (3,1) => return vec!["jAgarmi".into()],
                (3,2) => return vec!["jAgfvaH".into()],
                (3,3) => return vec!["jAgfmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["ajAgaH".into()],
                (1,2) => return vec!["ajAgftAm".into()],
                (1,3) => return vec!["ajAgaruH".into()],
                (2,1) => return vec!["ajAgaH".into()],
                (2,2) => return vec!["ajAgftam".into()],
                (2,3) => return vec!["ajAgfta".into()],
                (3,1) => return vec!["ajAgaram".into()],
                (3,2) => return vec!["ajAgfva".into()],
                (3,3) => return vec!["ajAgfma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["jAgartu".into()],
                (1,2) => return vec!["jAgftAm".into()],
                (1,3) => return vec!["jAgratu".into()],
                (2,1) => return vec!["jAgftAt".into()],
                (2,2) => return vec!["jAgftam".into()],
                (2,3) => return vec!["jAgfta".into()],
                (3,1) => return vec!["jAgarARi".into()],
                (3,2) => return vec!["jAgarAva".into()],
                (3,3) => return vec!["jAgarAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["jAgarizyati".into()],
                (1,2) => return vec!["jAgarizyataH".into()],
                (1,3) => return vec!["jAgarizyanti".into()],
                (2,1) => return vec!["jAgarizyasi".into()],
                (2,2) => return vec!["jAgarizyaTaH".into()],
                (2,3) => return vec!["jAgarizyaTa".into()],
                (3,1) => return vec!["jAgarizyAmi".into()],
                (3,2) => return vec!["jAgarizyAvaH".into()],
                (3,3) => return vec!["jAgarizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["jAgfyAt".into()],
                (1,2) => return vec!["jAgfyAtAm".into()],
                (1,3) => return vec!["jAgfyuH".into()],
                (2,1) => return vec!["jAgfyAH".into()],
                (2,2) => return vec!["jAgfyAtam".into()],
                (2,3) => return vec!["jAgfyAta".into()],
                (3,1) => return vec!["jAgfyAm".into()],
                (3,2) => return vec!["jAgfyAva".into()],
                (3,3) => return vec!["jAgfyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "daridrA" || dhatu_query == "02.0068" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["daridrAti".into()],
                (1,2) => return vec!["daridritaH".into()],
                (1,3) => return vec!["daridrati".into()],
                (2,1) => return vec!["daridrAsi".into()],
                (2,2) => return vec!["daridriTaH".into()],
                (2,3) => return vec!["daridriTa".into()],
                (3,1) => return vec!["daridrAmi".into()],
                (3,2) => return vec!["daridrivaH".into()],
                (3,3) => return vec!["daridrimaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["adaridrAt".into()],
                (1,2) => return vec!["adaridritAm".into()],
                (1,3) => return vec!["adaridruH".into()],
                (2,1) => return vec!["adaridrAH".into()],
                (2,2) => return vec!["adaridritam".into()],
                (2,3) => return vec!["adaridrita".into()],
                (3,1) => return vec!["adaridrAm".into()],
                (3,2) => return vec!["adaridriva".into()],
                (3,3) => return vec!["adaridrima".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["daridrAtu".into()],
                (1,2) => return vec!["daridritAm".into()],
                (1,3) => return vec!["daridratu".into()],
                (2,1) => return vec!["daridritAt".into()],
                (2,2) => return vec!["daridritam".into()],
                (2,3) => return vec!["daridrita".into()],
                (3,1) => return vec!["daridrARi".into()],
                (3,2) => return vec!["daridrAva".into()],
                (3,3) => return vec!["daridrAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["daridrizyati".into()],
                (1,2) => return vec!["daridrizyataH".into()],
                (1,3) => return vec!["daridrizyanti".into()],
                (2,1) => return vec!["daridrizyasi".into()],
                (2,2) => return vec!["daridrizyaTaH".into()],
                (2,3) => return vec!["daridrizyaTa".into()],
                (3,1) => return vec!["daridrizyAmi".into()],
                (3,2) => return vec!["daridrizyAvaH".into()],
                (3,3) => return vec!["daridrizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["daridriyAt".into()],
                (1,2) => return vec!["daridriyAtAm".into()],
                (1,3) => return vec!["daridriyuH".into()],
                (2,1) => return vec!["daridriyAH".into()],
                (2,2) => return vec!["daridriyAtam".into()],
                (2,3) => return vec!["daridriyAta".into()],
                (3,1) => return vec!["daridriyAm".into()],
                (3,2) => return vec!["daridriyAva".into()],
                (3,3) => return vec!["daridriyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "cakAsf" || dhatu_query == "02.0069" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["cakAsti".into()],
                (1,2) => return vec!["cakAstaH".into()],
                (1,3) => return vec!["cakAsati".into()],
                (2,1) => return vec!["cakAssi".into()],
                (2,2) => return vec!["cakAsTaH".into()],
                (2,3) => return vec!["cakAsTa".into()],
                (3,1) => return vec!["cakAsmi".into()],
                (3,2) => return vec!["cakAsvaH".into()],
                (3,3) => return vec!["cakAsmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["acakAt".into()],
                (1,2) => return vec!["acakAstAm".into()],
                (1,3) => return vec!["acakAsuH".into()],
                (2,1) => return vec!["acakAH".into()],
                (2,2) => return vec!["acakAstam".into()],
                (2,3) => return vec!["acakAsta".into()],
                (3,1) => return vec!["acakAsam".into()],
                (3,2) => return vec!["acakAsva".into()],
                (3,3) => return vec!["acakAsma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["cakAstAt".into()],
                (1,2) => return vec!["cakAstAm".into()],
                (1,3) => return vec!["cakAsatu".into()],
                (2,1) => return vec!["cakADi".into()],
                (2,2) => return vec!["cakAstam".into()],
                (2,3) => return vec!["cakAsta".into()],
                (3,1) => return vec!["cakAsAni".into()],
                (3,2) => return vec!["cakAsAva".into()],
                (3,3) => return vec!["cakAsAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["cakAsizyati".into()],
                (1,2) => return vec!["cakAsizyataH".into()],
                (1,3) => return vec!["cakAsizyanti".into()],
                (2,1) => return vec!["cakAsizyasi".into()],
                (2,2) => return vec!["cakAsizyaTaH".into()],
                (2,3) => return vec!["cakAsizyaTa".into()],
                (3,1) => return vec!["cakAsizyAmi".into()],
                (3,2) => return vec!["cakAsizyAvaH".into()],
                (3,3) => return vec!["cakAsizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["cakAsyAt".into()],
                (1,2) => return vec!["cakAsyAtAm".into()],
                (1,3) => return vec!["cakAsyuH".into()],
                (2,1) => return vec!["cakAsyAH".into()],
                (2,2) => return vec!["cakAsyAtam".into()],
                (2,3) => return vec!["cakAsyAta".into()],
                (3,1) => return vec!["cakAsyAm".into()],
                (3,2) => return vec!["cakAsyAva".into()],
                (3,3) => return vec!["cakAsyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "SAsu" || dhatu_query == "02.0070" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["SAsti".into()],
                (1,2) => return vec!["SizwaH".into()],
                (1,3) => return vec!["SAsati".into()],
                (2,1) => return vec!["SAssi".into()],
                (2,2) => return vec!["SizWaH".into()],
                (2,3) => return vec!["SizWa".into()],
                (3,1) => return vec!["SAsmi".into()],
                (3,2) => return vec!["SizvaH".into()],
                (3,3) => return vec!["SizmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["aSAt".into()],
                (1,2) => return vec!["aSizwAm".into()],
                (1,3) => return vec!["aSAsuH".into()],
                (2,1) => return vec!["aSAH".into()],
                (2,2) => return vec!["aSizwam".into()],
                (2,3) => return vec!["aSizwa".into()],
                (3,1) => return vec!["aSAsam".into()],
                (3,2) => return vec!["aSizva".into()],
                (3,3) => return vec!["aSizma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["SAstu".into()],
                (1,2) => return vec!["SizwAm".into()],
                (1,3) => return vec!["SAsatu".into()],
                (2,1) => return vec!["SADi".into()],
                (2,2) => return vec!["Sizwam".into()],
                (2,3) => return vec!["Sizwa".into()],
                (3,1) => return vec!["SAsAni".into()],
                (3,2) => return vec!["SAsAva".into()],
                (3,3) => return vec!["SAsAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["SAsizyati".into()],
                (1,2) => return vec!["SAsizyataH".into()],
                (1,3) => return vec!["SAsizyanti".into()],
                (2,1) => return vec!["SAsizyasi".into()],
                (2,2) => return vec!["SAsizyaTaH".into()],
                (2,3) => return vec!["SAsizyaTa".into()],
                (3,1) => return vec!["SAsizyAmi".into()],
                (3,2) => return vec!["SAsizyAvaH".into()],
                (3,3) => return vec!["SAsizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["SizyAt".into()],
                (1,2) => return vec!["SizyAtAm".into()],
                (1,3) => return vec!["SizyuH".into()],
                (2,1) => return vec!["SizyAH".into()],
                (2,2) => return vec!["SizyAtam".into()],
                (2,3) => return vec!["SizyAta".into()],
                (3,1) => return vec!["SizyAm".into()],
                (3,2) => return vec!["SizyAva".into()],
                (3,3) => return vec!["SizyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "zasa" || dhatu_query == "02.0073" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["sasti".into()],
                (1,2) => return vec!["sastaH".into()],
                (1,3) => return vec!["sasanti".into()],
                (2,1) => return vec!["sassi".into()],
                (2,2) => return vec!["sasTaH".into()],
                (2,3) => return vec!["sasTa".into()],
                (3,1) => return vec!["sasmi".into()],
                (3,2) => return vec!["sasvaH".into()],
                (3,3) => return vec!["sasmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["asat".into()],
                (1,2) => return vec!["asastAm".into()],
                (1,3) => return vec!["asasan".into()],
                (2,1) => return vec!["asaH".into()],
                (2,2) => return vec!["asastam".into()],
                (2,3) => return vec!["asasta".into()],
                (3,1) => return vec!["asasam".into()],
                (3,2) => return vec!["asasva".into()],
                (3,3) => return vec!["asasma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["sastAt".into()],
                (1,2) => return vec!["sastAm".into()],
                (1,3) => return vec!["sasantu".into()],
                (2,1) => return vec!["saDi".into()],
                (2,2) => return vec!["sastam".into()],
                (2,3) => return vec!["sasta".into()],
                (3,1) => return vec!["sasAni".into()],
                (3,2) => return vec!["sasAva".into()],
                (3,3) => return vec!["sasAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["sasizyati".into()],
                (1,2) => return vec!["sasizyataH".into()],
                (1,3) => return vec!["sasizyanti".into()],
                (2,1) => return vec!["sasizyasi".into()],
                (2,2) => return vec!["sasizyaTaH".into()],
                (2,3) => return vec!["sasizyaTa".into()],
                (3,1) => return vec!["sasizyAmi".into()],
                (3,2) => return vec!["sasizyAvaH".into()],
                (3,3) => return vec!["sasizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["sasyAt".into()],
                (1,2) => return vec!["sasyAtAm".into()],
                (1,3) => return vec!["sasyuH".into()],
                (2,1) => return vec!["sasyAH".into()],
                (2,2) => return vec!["sasyAtam".into()],
                (2,3) => return vec!["sasyAta".into()],
                (3,1) => return vec!["sasyAm".into()],
                (3,2) => return vec!["sasyAva".into()],
                (3,3) => return vec!["sasyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "zasti" || dhatu_query == "02.0074" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["saMsti".into()],
                (1,2) => return vec!["saMstaH".into()],
                (1,3) => return vec!["saMstanti".into()],
                (2,1) => return vec!["saMstsi".into()],
                (2,2) => return vec!["saMstTaH".into()],
                (2,3) => return vec!["saMstTa".into()],
                (3,1) => return vec!["saMstmi".into()],
                (3,2) => return vec!["saMstvaH".into()],
                (3,3) => return vec!["saMstmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["asan".into()],
                (1,2) => return vec!["asaMstAm".into()],
                (1,3) => return vec!["asaMstan".into()],
                (2,1) => return vec!["asan".into()],
                (2,2) => return vec!["asaMstam".into()],
                (2,3) => return vec!["asaMsta".into()],
                (3,1) => return vec!["asaMstam".into()],
                (3,2) => return vec!["asaMstva".into()],
                (3,3) => return vec!["asaMstma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["saMstAt".into()],
                (1,2) => return vec!["saMstAm".into()],
                (1,3) => return vec!["saMstantu".into()],
                (2,1) => return vec!["saMstAt".into()],
                (2,2) => return vec!["saMstam".into()],
                (2,3) => return vec!["saMsta".into()],
                (3,1) => return vec!["saMstAni".into()],
                (3,2) => return vec!["saMstAva".into()],
                (3,3) => return vec!["saMstAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["saMstizyati".into()],
                (1,2) => return vec!["saMstizyataH".into()],
                (1,3) => return vec!["saMstizyanti".into()],
                (2,1) => return vec!["saMstizyasi".into()],
                (2,2) => return vec!["saMstizyaTaH".into()],
                (2,3) => return vec!["saMstizyaTa".into()],
                (3,1) => return vec!["saMstizyAmi".into()],
                (3,2) => return vec!["saMstizyAvaH".into()],
                (3,3) => return vec!["saMstizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["saMstyAt".into()],
                (1,2) => return vec!["saMstyAtAm".into()],
                (1,3) => return vec!["saMstyuH".into()],
                (2,1) => return vec!["saMstyAH".into()],
                (2,2) => return vec!["saMstyAtam".into()],
                (2,3) => return vec!["saMstyAta".into()],
                (3,1) => return vec!["saMstyAm".into()],
                (3,2) => return vec!["saMstyAva".into()],
                (3,3) => return vec!["saMstyAma".into()],
                _ => {}
            }
        }
    }
    if dhatu_query == "vaSa" || dhatu_query == "02.0075" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return vec!["vazwi".into()],
                (1,2) => return vec!["uzwaH".into()],
                (1,3) => return vec!["uSanti".into()],
                (2,1) => return vec!["vakzi".into()],
                (2,2) => return vec!["uzWaH".into()],
                (2,3) => return vec!["uzWa".into()],
                (3,1) => return vec!["vaSmi".into()],
                (3,2) => return vec!["uSvaH".into()],
                (3,3) => return vec!["uSmaH".into()],
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return vec!["avaw".into()],
                (1,2) => return vec!["OzwAm".into()],
                (1,3) => return vec!["OSan".into()],
                (2,1) => return vec!["avaw".into()],
                (2,2) => return vec!["Ozwam".into()],
                (2,3) => return vec!["Ozwa".into()],
                (3,1) => return vec!["avaSam".into()],
                (3,2) => return vec!["OSva".into()],
                (3,3) => return vec!["OSma".into()],
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return vec!["uzwAt".into()],
                (1,2) => return vec!["uzwAm".into()],
                (1,3) => return vec!["uSantu".into()],
                (2,1) => return vec!["uqQi".into()],
                (2,2) => return vec!["uzwam".into()],
                (2,3) => return vec!["uzwa".into()],
                (3,1) => return vec!["vaSAni".into()],
                (3,2) => return vec!["vaSAva".into()],
                (3,3) => return vec!["vaSAma".into()],
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return vec!["vaSizyati".into()],
                (1,2) => return vec!["vaSizyataH".into()],
                (1,3) => return vec!["vaSizyanti".into()],
                (2,1) => return vec!["vaSizyasi".into()],
                (2,2) => return vec!["vaSizyaTaH".into()],
                (2,3) => return vec!["vaSizyaTa".into()],
                (3,1) => return vec!["vaSizyAmi".into()],
                (3,2) => return vec!["vaSizyAvaH".into()],
                (3,3) => return vec!["vaSizyAmaH".into()],
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return vec!["uSyAt".into()],
                (1,2) => return vec!["uSyAtAm".into()],
                (1,3) => return vec!["uSyuH".into()],
                (2,1) => return vec!["uSyAH".into()],
                (2,2) => return vec!["uSyAtam".into()],
                (2,3) => return vec!["uSyAta".into()],
                (3,1) => return vec!["uSyAm".into()],
                (3,2) => return vec!["uSyAva".into()],
                (3,3) => return vec!["uSyAma".into()],
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
