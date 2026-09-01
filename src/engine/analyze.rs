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
static TINANTA_MAP: OnceLock<HashMap<String, Vec<Analysis>>> = OnceLock::new();
static KRDANTA_MAP: OnceLock<HashMap<String, Vec<Analysis>>> = OnceLock::new();

// ---------------------------------------------------------------------------
// const `LAKARAS`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
const LAKARAS: &[&str] = &[
    "plat", "alat", "plan", "alan", "plot", "alot", "plrt", "alrt",
    "pvidhilin", "avidhilin", "plit", "alit", "plun", "alun", "pashirling", "aashirling",
];

// ---------------------------------------------------------------------------
// const `KRTS`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
const KRTS: &[&str] = &[
    "kta", "ktavatu", "Satf", "SAnac", "tumun", "ktvA", "lyap", "lyuw", "tavya", "anIyar", "Rvul",
    "tfc", "ktin", "GaY", "vun", "ukaY", "Ryat", "yat", "ac",
];

// ---------------------------------------------------------------------------
// fn `push_form`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn push_form(map: &mut HashMap<String, Vec<Analysis>>, form: &str, a: Analysis) {
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for key in keys_for(form) {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
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
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if !dup {
            entry.push(a.clone());
        }
    }
}

// ---------------------------------------------------------------------------
// fn `build_tinanta_map` — tin/sUP endings: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn build_tinanta_map() -> HashMap<String, Vec<Analysis>> {
    let mut map: HashMap<String, Vec<Analysis>> = HashMap::new();
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for (dhatu_id, dhatu, _, _, _, _, _) in crate::data::DHATUS {
        // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
        for lak in LAKARAS {
            // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
            for p in 1..=3u8 {
                // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
                for v in 1..=3u8 {
                    let bases = crate::engine::tinanta::generate_all(dhatu_id, lak, p, v);
                    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
                    for base in bases {
                        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
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

// ---------------------------------------------------------------------------
// fn `build_krdanta_map`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn build_krdanta_map() -> HashMap<String, Vec<Analysis>> {
    let mut map: HashMap<String, Vec<Analysis>> = HashMap::new();
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for (dhatu_id, dhatu, _, _, _, _, _) in crate::data::DHATUS {
        // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
        for pr in KRTS {
            // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
            for base in crate::engine::krdanta::derive(dhatu_id, pr) {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
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

// ---------------------------------------------------------------------------
// fn `attach_upasarga`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
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

// ---------------------------------------------------------------------------
// fn `analyze_word`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn analyze_word(word: &str) -> Vec<Analysis> {
    let mut out: Vec<Analysis> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for m in crate::declension::subanta::analyze(word) {
        let key = format!(
            "subanta:{}:{}:{}:{}",
            m.get("pratipadika").map(String::as_str).unwrap_or(""),
            m.get("linga").map(String::as_str).unwrap_or(""),
            m.get("vibhakti").map(String::as_str).unwrap_or(""),
            m.get("vacana").map(String::as_str).unwrap_or("")
        );
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
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
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for m in crate::declension::sarvanama::analyze(word) {
        let key = format!(
            "sarvanama:{}:{}:{}:{}",
            m.get("pratipadika").map(String::as_str).unwrap_or(""),
            m.get("linga").map(String::as_str).unwrap_or(""),
            m.get("vibhakti").map(String::as_str).unwrap_or(""),
            m.get("vacana").map(String::as_str).unwrap_or("")
        );
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
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

    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for (prefs, rest) in split_upasarga_candidates(word) {
        // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
        for key in keys_for(&rest) {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(v) = tmap.get(&key) {
                // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
                for a in v {
                    let sig = format!(
                        "tinanta:{}:{}:{}:{}:{:?}",
                        a.dhatu_id.as_deref().unwrap_or(""),
                        a.lakara.as_deref().unwrap_or(""),
                        a.purusha.unwrap_or(0),
                        a.vacana.unwrap_or(0),
                        prefs
                    );
                    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                    if seen.insert(sig) {
                        out.push(attach_upasarga(a, &prefs, word));
                    }
                }
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(v) = kmap.get(&key) {
                // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
                for a in v {
                    let sig = format!(
                        "krdanta:{}:{}:{:?}",
                        a.dhatu_id.as_deref().unwrap_or(""),
                        a.pratyaya.as_deref().unwrap_or(""),
                        prefs
                    );
                    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                    if seen.insert(sig) {
                        out.push(attach_upasarga(a, &prefs, word));
                    }
                }
            }
        }
    }

    out
}

// ---------------------------------------------------------------------------
// fn `search_prefix`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn search_prefix(prefix: &str, limit: usize) -> Vec<String> {
    let mut results = Vec::new();
    let prefix_lower = prefix.to_lowercase();
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for (_, dhatu, _, _, _, _, _) in crate::data::DHATUS {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if dhatu.to_lowercase().starts_with(&prefix_lower) {
            results.push(dhatu.to_string());
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if results.len() >= limit {
                return results;
            }
        }
    }
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for stem in ["rAma", "hari", "guru", "nadI", "Bava", "gacC"] {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if stem.starts_with(prefix) {
            results.push(stem.to_string());
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if results.len() >= limit {
                break;
            }
        }
    }
    results.truncate(limit);
    results
}

#[cfg(test)]
// ---------------------------------------------------------------------------
// mod `tests`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
mod tests {
    #[test]
    // ---------------------------------------------------------------------------
    // fn `rame_na_is_trtiya_ekavacana`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn rame_na_is_trtiya_ekavacana() {
        let hits = crate::declension::subanta::analyze("rAmeRa");
        assert!(hits.iter().any(|m| {
            m.get("pratipadika").map(String::as_str) == Some("rAma")
                && m.get("vibhakti").map(String::as_str) == Some("tfIyA")
                && m.get("vacana").map(String::as_str) == Some("1")
        }));
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `trampe_na_is_foreign_instrumental`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn trampe_na_is_foreign_instrumental() {
        let hits = crate::declension::subanta::analyze("wrampeRa");
        assert!(hits.iter().any(|m| {
            m.get("pratipadika").map(String::as_str) == Some("wrampa")
                && m.get("vibhakti").map(String::as_str) == Some("tfIyA")
                && m.get("vacana").map(String::as_str) == Some("1")
        }));
    }
}
