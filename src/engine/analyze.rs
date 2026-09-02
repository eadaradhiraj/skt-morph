//! analyze — reverse lookup (surface SLP1 → parses).
//! Builds tinanta/kṛdanta maps lazily (OnceLock); subanta/sarvanāma are live.
//! Upasargas are peeled at query time via `prefix::split_upasarga_candidates`.
//! No DB used; pure sūtra generation inverted.
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use crate::engine::prefix::split_upasarga_candidates;

/// Single parse — covers tinanta/kṛdanta/subanta/sarvanāma.
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
    pub upasarga: Option<String>,
}

/// Normalize anusvāra variants (8.3.23/8.4.58) for key equivalence.
fn normalize_nasal(s: &str) -> String {
    s.replace("saG", "saM").replace("saN", "saM").replace("saM", "sam")
}

/// Two lookup keys: raw + anusvāra-normalized.
fn keys_for(form: &str) -> [String; 2] {
    [form.to_string(), normalize_nasal(form)]
}

/// Unprefixed surface → analyses. Prefixes are peeled at query time (not stored).
/// OnceLock HashMaps built on first `analyze_word` (~7s with dhatu cache, then µs).
static TINANTA_MAP: OnceLock<HashMap<String, Vec<Analysis>>> = OnceLock::new();
static KRDANTA_MAP: OnceLock<HashMap<String, Vec<Analysis>>> = OnceLock::new();

const LAKARAS: &[&str] = &[
    "plat", "alat", "plan", "alan", "plot", "alot", "plrt", "alrt",
    "pvidhilin", "avidhilin", "plit", "alit", "plun", "alun", "pashirling", "aashirling",
];

/// KRTS indexed for reverse lookup — must stay in sync with `krdanta::pratyaya_rule`.
/// Expanded from 19 → full set so e.g. gAsnu/jizRu/sthAsnu, kvasu, lyu etc. are analyzed.
const KRTS: &[&str] = &[
    "kta", "ktavatu", "ktavatu~", "Satf", "Satf~", "SAnac", "cAnaS", "tumun", "ktvA", "lyap",
    "lyuw", "lyu", "tavya", "anIyar", "Rvul", "vun", "tfc", "ktin", "GaY", "Ramul", "ac", "a",
    "yat", "Ryat", "kyap", "ukaY", "kvasu", "Ra", "Sa", "ka",
    "gsnu", "knu", "GinuR", "kvarap", "Aluc", "kmarac", "Gurac", "varac", "itra", "zwran",
    "kurac", "kru", "klukan", "krukan", "Aru", "ra", "u", "naN", "aTuc", "Nvanip", "Takan",
    "Ryuw", "nan", "najiN", "zAkan", "zvun", "SAnan", "atfn", "vuY", "ktri", "aN", "ap",
    "sya-Satf", "sya-SAnac", "sya-cAnaS", "ini",
];

fn push_form(map: &mut HashMap<String, Vec<Analysis>>, form: &str, a: Analysis) {
    for key in keys_for(form) {
        if key.is_empty() {
            continue;
        }
        let entry = map.entry(key).or_default();
        let dup = entry.iter().any(|e| {
            e.word_type == a.word_type
                && e.dhatu_id == a.dhatu_id
                && e.lakara == a.lakara
                && e.purusha == a.purusha
                && e.vacana == a.vacana
                && e.pratyaya == a.pratyaya
        });
        if !dup {
            entry.push(a.clone());
        }
    }
}

fn build_tinanta_map() -> HashMap<String, Vec<Analysis>> {
    let mut map: HashMap<String, Vec<Analysis>> = HashMap::new();
    for (dhatu_id, dhatu, _, _, _, _, _) in crate::data::DHATUS {
        for lak in LAKARAS {
            for p in 1..=3u8 {
                for v in 1..=3u8 {
                    let bases = crate::engine::tinanta::generate_all(dhatu_id, lak, p, v);
                    for base in bases {
                        if base.is_empty() {
                            continue;
                        }
                        push_form(
                            &mut map,
                            &base,
                            Analysis {
                                word: String::new(),
                                word_type: "tinanta".to_string(),
                                dhatu: Some(dhatu.to_string()),
                                dhatu_id: Some(dhatu_id.to_string()),
                                pratyaya: None,
                                pratipadika: None,
                                linga: None,
                                vibhakti: None,
                                vacana: Some(v),
                                lakara: Some((*lak).to_string()),
                                purusha: Some(p),
                                upasarga: None,
                            },
                        );
                    }
                }
            }
        }
    }
    map
}

fn build_krdanta_map() -> HashMap<String, Vec<Analysis>> {
    let mut map: HashMap<String, Vec<Analysis>> = HashMap::new();
    for (dhatu_id, dhatu, _, _, _, _, _) in crate::data::DHATUS {
        for pr in KRTS {
            for base in crate::engine::krdanta::derive(dhatu_id, pr) {
                if base.is_empty() {
                    continue;
                }
                push_form(
                    &mut map,
                    &base,
                    Analysis {
                        word: String::new(),
                        word_type: "krdanta".to_string(),
                        dhatu: Some(dhatu.to_string()),
                        dhatu_id: Some(dhatu_id.to_string()),
                        pratyaya: Some((*pr).to_string()),
                        pratipadika: None,
                        linga: None,
                        vibhakti: None,
                        vacana: None,
                        lakara: None,
                        purusha: None,
                        upasarga: None,
                    },
                );
            }
        }
    }
    map
}

fn attach_upasarga(a: &Analysis, prefs: &[String], word: &str) -> Analysis {
    let mut b = a.clone();
    b.word = word.to_string();
    b.upasarga = if prefs.is_empty() {
        None
    } else {
        Some(prefs.join("+"))
    };
    b
}

pub fn analyze_word(word: &str) -> Vec<Analysis> {
    let w = word.trim();
    if w.is_empty() {
        return Vec::new(); // guard empty/whitespace — no panic on peel (iter 35/36)
    }
    let mut out: Vec<Analysis> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for m in crate::declension::subanta::analyze(word) {
        let key = format!(
            "subanta:{}:{}:{}:{}",
            m.get("pratipadika").map(String::as_str).unwrap_or(""),
            m.get("linga").map(String::as_str).unwrap_or(""),
            m.get("vibhakti").map(String::as_str).unwrap_or(""),
            m.get("vacana").map(String::as_str).unwrap_or("")
        );
        if seen.insert(key) {
            out.push(Analysis {
                word: word.to_string(),
                word_type: "subanta".to_string(),
                dhatu: None,
                dhatu_id: None,
                pratyaya: None,
                pratipadika: m.get("pratipadika").cloned(),
                linga: m.get("linga").cloned(),
                vibhakti: m.get("vibhakti").cloned(),
                vacana: m.get("vacana").and_then(|v| v.parse().ok()),
                lakara: None,
                purusha: None,
                upasarga: None,
            });
        }
    }
    for m in crate::declension::sarvanama::analyze(word) {
        let key = format!(
            "sarvanama:{}:{}:{}:{}",
            m.get("pratipadika").map(String::as_str).unwrap_or(""),
            m.get("linga").map(String::as_str).unwrap_or(""),
            m.get("vibhakti").map(String::as_str).unwrap_or(""),
            m.get("vacana").map(String::as_str).unwrap_or("")
        );
        if seen.insert(key) {
            out.push(Analysis {
                word: word.to_string(),
                word_type: "sarvanama".to_string(),
                dhatu: None,
                dhatu_id: None,
                pratyaya: None,
                pratipadika: m.get("pratipadika").cloned(),
                linga: m.get("linga").cloned(),
                vibhakti: m.get("vibhakti").cloned(),
                vacana: m.get("vacana").and_then(|v| v.parse().ok()),
                lakara: None,
                purusha: None,
                upasarga: None,
            });
        }
    }

    let tmap = TINANTA_MAP.get_or_init(build_tinanta_map);
    let kmap = KRDANTA_MAP.get_or_init(build_krdanta_map);
    for (prefs, rest) in split_upasarga_candidates(word) {
        for key in keys_for(&rest) {
            if let Some(v) = tmap.get(&key) {
                for a in v {
                    let sig = format!(
                        "tinanta:{}:{}:{}:{}:{:?}",
                        a.dhatu_id.as_deref().unwrap_or(""),
                        a.lakara.as_deref().unwrap_or(""),
                        a.purusha.unwrap_or(0),
                        a.vacana.unwrap_or(0),
                        prefs
                    );
                    if seen.insert(sig) {
                        out.push(attach_upasarga(a, &prefs, word));
                    }
                }
            }
            if let Some(v) = kmap.get(&key) {
                for a in v {
                    let sig = format!(
                        "krdanta:{}:{}:{:?}",
                        a.dhatu_id.as_deref().unwrap_or(""),
                        a.pratyaya.as_deref().unwrap_or(""),
                        prefs
                    );
                    if seen.insert(sig) {
                        out.push(attach_upasarga(a, &prefs, word));
                    }
                }
            }
        }
    }

    out
}

/// Prefix search for demo autocomplete — dhātus + common pratipadikas (SLP1, 1.4.59).
/// Case-insensitive for dhātu; exact prefix for stems. Fast O(n) scan (≈2k).
pub fn search_prefix(prefix: &str, limit: usize) -> Vec<String> {
    let mut results = Vec::new();
    let prefix_lower = prefix.to_lowercase();
    for (_, dhatu, _, _, _, _, _) in crate::data::DHATUS {
        if dhatu.to_lowercase().starts_with(&prefix_lower) {
            results.push(dhatu.to_string());
            if results.len() >= limit {
                return results;
            }
        }
    }
    // Common subantas for quick demo probing
    for stem in ["rAma", "hari", "guru", "nadI", "Bava", "gacC", "rAjan", "pitf", "go", "nO", "tad", "etad", "yad", "idam", "ad", "dvi", "tri", "pazcan", "wrampa"] {
        if stem.starts_with(prefix) {
            results.push(stem.to_string());
            if results.len() >= limit {
                break;
            }
        }
    }
    results.truncate(limit);
    results
}

#[cfg(test)]
mod tests {
    #[test]
    fn rame_na_is_trtiya_ekavacana() {
        let hits = crate::declension::subanta::analyze("rAmeRa");
        assert!(hits.iter().any(|m| {
            m.get("pratipadika").map(String::as_str) == Some("rAma")
                && m.get("vibhakti").map(String::as_str) == Some("tfIyA")
                && m.get("vacana").map(String::as_str) == Some("1")
        }));
    }

    #[test]
    fn empty_word_returns_empty() {
        assert!(crate::engine::analyze::analyze_word("").is_empty());
        assert!(crate::engine::analyze::analyze_word("   ").is_empty());
    }

    #[test]
    fn trampe_na_is_foreign_instrumental() {
        let hits = crate::declension::subanta::analyze("wrampeRa");
        assert!(hits.iter().any(|m| {
            m.get("pratipadika").map(String::as_str) == Some("wrampa")
                && m.get("vibhakti").map(String::as_str) == Some("tfIyA")
                && m.get("vacana").map(String::as_str) == Some("1")
        }));
    }
}
// all4 850 -- analyze
// all4 854 -- analyze
// all4 858 -- analyze
// all4 862 -- analyze
