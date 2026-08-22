use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Analysis {
    pub word: String,
    pub word_type: String,
    pub dhatu: Option<String>,
    pub dhatu_id: Option<String>,
    pub pratyaya: Option<String>,
    pub pratipadika: Option<String>,
    pub linga: Option<String>,
    pub vibhakti: Option<String>,
    pub vacana: Option<u8>,
    pub lakara: Option<String>,
    pub purusha: Option<u8>,
}

pub fn analyze_word(word: &str) -> Vec<Analysis> {
    let mut out: Vec<Analysis> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // 1. Try subanta reverse
    for m in crate::declension::subanta::analyze(word) {
        let key = format!("subanta:{}:{}:{}:{}", m.get("pratipadika").unwrap(), m.get("linga").unwrap(), m.get("vibhakti").unwrap(), m.get("vacana").unwrap());
        if seen.insert(key) {
            out.push(Analysis {
                word: word.to_string(),
                word_type: "subanta".to_string(),
                dhatu: None, dhatu_id: None, pratyaya: None,
                pratipadika: m.get("pratipadika").cloned(),
                linga: m.get("linga").cloned(),
                vibhakti: m.get("vibhakti").cloned(),
                vacana: m.get("vacana").and_then(|v| v.parse().ok()),
                lakara: None, purusha: None,
            });
        }
    }

    // 2. Sarvanama reverse
    for m in crate::declension::sarvanama::analyze(word) {
        let key = format!("sarvanama:{}:{}:{}:{}", m.get("pratipadika").unwrap(), m.get("linga").unwrap(), m.get("vibhakti").unwrap(), m.get("vacana").unwrap());
        if seen.insert(key) {
            out.push(Analysis {
                word: word.to_string(),
                word_type: "sarvanama".to_string(),
                dhatu: None, dhatu_id: None, pratyaya: None,
                pratipadika: m.get("pratipadika").cloned(),
                linga: m.get("linga").cloned(),
                vibhakti: m.get("vibhakti").cloned(),
                vacana: m.get("vacana").and_then(|v| v.parse().ok()),
                lakara: None, purusha: None,
            });
        }
    }

    // 3. Heuristic krdanta: if word ends with known suffixes, try live krdanta reverse (tiny demo)
    // Full FST will replace this - for now we just check if word could be a participle by suffix
    let krdanta_suffixes = ["ta","tavAn","tam","ya","anIya","tavya"];
    for suf in krdanta_suffixes {
        if word.ends_with(suf) && word.len() > suf.len()+2 {
            // don't add duplicate, just hint that it could be krdanta
            // Real implementation will generate and check via FST
            break;
        }
    }

    // If nothing found, return at least an unknown analysis so UI can show "no results"
    out
}

pub fn search_prefix(prefix: &str, limit: usize) -> Vec<String> {
    let mut results = Vec::new();
    let prefix_lower = prefix.to_lowercase();
    for (_, dhatu, _, _, _, _, _) in crate::data::DHATUS {
        if dhatu.to_lowercase().starts_with(&prefix_lower) {
            results.push(dhatu.to_string());
            if results.len() >= limit { return results; }
        }
    }
    // 2. Add some common subanta stems
    for stem in ["rAma", "hari", "guru", "nadI", "Bava", "gacC"] {
        if stem.starts_with(prefix) {
            results.push(stem.to_string());
            if results.len() >= limit { break; }
        }
    }
    results.truncate(limit);
    results
}
