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
    Some(crate::engine::dhatu::load_or_fallback(dhatu_query))
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


fn attach_prefixes(prefixes: &[String], forms: Vec<String>) -> Vec<String> {
    if prefixes.is_empty() { forms }
    else { forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect() }
}

fn gold_forms(dhatu_query: &str, db_lakara: &str, purusha: u8, vacana: u8) -> Option<Vec<String>> {
    #[cfg(feature = "native-db")]
    {
        let search_id = crate::engine::dhatu::resolve_id(dhatu_query);
        if let Ok(idx) = TINANTA_GOLD.binary_search_by(|(did, lak, pur, vac, _)| {
            (*did, *lak, *pur, *vac).cmp(&(&search_id.as_str(), db_lakara, purusha, vacana))
        }) {
            let mut out: Vec<String> = Vec::new();
            for part in TINANTA_GOLD[idx].4.split(',') {
                for pp in part.split(';') {
                    let v = pp.trim();
                    if !v.is_empty() { out.push(v.to_string()); }
                }
            }
            if !out.is_empty() { return Some(out); }
        }
    }
    #[cfg(all(not(feature = "native-db"), feature = "wasm-gold"))]
    {
        let search_id = crate::engine::dhatu::resolve_id(dhatu_query);
        let key = (search_id, db_lakara.to_string(), purusha, vacana);
        if let Some(form) = WASM_TINANTA_GOLD.get(&key) {
            let mut out: Vec<String> = Vec::new();
            for part in form.split(',') {
                for pp in part.split(';') {
                    let v = pp.trim();
                    if !v.is_empty() { out.push(v.to_string()); }
                }
            }
            if !out.is_empty() { return Some(out); }
        }
    }
    let _ = (dhatu_query, db_lakara, purusha, vacana);
    None
}

pub fn generate_all_with_prefixes(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &[String]) -> Vec<String> {
    let (canonical, db_lakara) = normalize_lakara(lakara);
    if let Some(out) = gold_forms(dhatu_query, &db_lakara, purusha, vacana) {
        return attach_prefixes(prefixes, out);
    }
    if let Some(out) = crate::engine::tinanta_overrides::lookup_override(dhatu_query, &canonical, purusha, vacana, prefixes) {
        return out;
    }
    live_generate(dhatu_query, &canonical, &db_lakara, purusha, vacana, prefixes)
}

/// Stem → ending → join. No gold, no per-root override table.
pub(crate) fn live_generate(
    dhatu_query: &str,
    canonical: &str,
    db_lakara: &str,
    purusha: u8,
    vacana: u8,
    prefixes: &[String],
) -> Vec<String> {
    let Some((dhatu, gana, root_pada, tags, antarganas, aupadeshik)) = load_dhatu_info(dhatu_query) else {
        return vec![];
    };
    let Some(family) = lakara_family(db_lakara) else { return vec![]; };
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
    let forms = join_variants(&stem, variants, cgana, &family, purusha, pada, augment.as_deref(), &dhatu, vacana, &antarganas);
    attach_prefixes(prefixes, forms)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bu_lat_prathama_ekavacana() {
        let f = generate_all("BU", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Bavati"), "{:?}", f);
    }

    #[test]
    fn gam_lat_prathama_ekavacana() {
        let f = generate_all("gam", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "gacCati"), "{:?}", f);
    }

    #[test]
    fn pra_bu_lat() {
        let prefs = vec!["pra".to_string()];
        let f = generate_all_with_prefixes("BU", "plat", 1, 1, &prefs);
        assert!(f.iter().any(|x| x == "praBavati"), "{:?}", f);
    }

    #[test]
    fn live_kakzi_vidhilin() {
        let (canon, db) = normalize_lakara("pvidhilin");
        let f = live_generate("kAkzi", &canon, &db, 1, 1, &[]);
        assert!(f.iter().any(|x| x == "kANkzet" || x == "kANkzed"), "{:?}", f);
    }

    #[test]
    fn live_gamx_no_override() {
        assert!(crate::engine::tinanta_overrides::lookup_override("gamx", "plat", 1, 1, &[]).is_none());
        let (canon, db) = normalize_lakara("plat");
        let f = live_generate("gamx", &canon, &db, 1, 1, &[]);
        assert!(f.iter().any(|x| x == "gacCati"), "{:?}", f);
        let (canon, db) = normalize_lakara("plrt");
        let f = live_generate("gamx", &canon, &db, 1, 1, &[]);
        assert!(f.iter().any(|x| x == "gamizyati"), "{:?}", f);
    }
}
