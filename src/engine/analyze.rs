use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

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

const UPASARGAS: &[&str] = &[
    "pra", "parA", "apa", "sam", "anu", "ava", "nis", "nir", "dus", "dur",
    "vi", "A", "aDi", "api", "ati", "su", "ud", "aBi", "prati", "pari", "upa", "ni",
];

fn normalize_nasal(s: &str) -> String {
    s.replace("saG", "saM").replace("saN", "saM").replace("saM", "sam")
}

fn prefix_combos() -> Vec<Vec<String>> {
    let mut combos: Vec<Vec<String>> = Vec::new();
    combos.push(vec![]);
    for &p in UPASARGAS { combos.push(vec![p.to_string()]); }
    combos.push(vec!["sam".to_string(), "A".to_string()]);
    combos.push(vec!["vi".to_string(), "parA".to_string()]);
    combos
}

// Lazy global maps: base/prefixed form -> Vec<Analysis> (without word)
// Built on first analyze call, ~2.4M forms, ~200ms in release
static TINANTA_MAP: OnceLock<HashMap<String, Vec<Analysis>>> = OnceLock::new();
static KRDANTA_MAP: OnceLock<HashMap<String, Vec<Analysis>>> = OnceLock::new();

fn build_tinanta_map() -> HashMap<String, Vec<Analysis>> {
    let mut map: HashMap<String, Vec<Analysis>> = HashMap::new();
    let combos = prefix_combos();
    let lakaras = ["plat","alat","plan","alan","plot","alot","plrt","alrt","pvidhilin","avidhilin"];
    for (dhatu_id, dhatu, _, _, _, _, _) in crate::data::DHATUS {
        for lak in lakaras {
            for p in 1..=3u8 { for v in 1..=3u8 {
                let (bases, _) = crate::engine::tinanta::generate_all(dhatu_id, lak, p, v);
                for base in bases {
                    for pref in &combos {
                        let form = if pref.is_empty() { base.clone() } else { crate::engine::prefix::apply_prefixes(pref, &base) };
                        // also store normalized variant for saM/saG
                        for key in [form.clone(), normalize_nasal(&form)] {
                            let entry = map.entry(key).or_insert_with(Vec::new);
                            // dedup by dhatu+lakara+p+v+upasarga
                            let upa = if pref.is_empty() { None } else { Some(pref.join("+")) };
                            let exists = entry.iter().any(|a: &Analysis| a.dhatu_id.as_deref()==Some(dhatu_id) && a.lakara.as_deref()==Some(lak) && a.purusha==Some(p) && a.vacana==Some(v) && a.upasarga==upa);
                            if !exists {
                                entry.push(Analysis {
                                    word: String::new(), word_type: "tinanta".to_string(),
                                    dhatu: Some(dhatu.to_string()), dhatu_id: Some(dhatu_id.to_string()),
                                    pratyaya: None, pratipadika: None, linga: None, vibhakti: None,
                                    vacana: Some(v), lakara: Some(lak.to_string()), purusha: Some(p),
                                    upasarga: upa.clone(),
                                });
                            }
                        }
                    }
                }
            }}
        }
    }
    map
}

fn build_krdanta_map() -> HashMap<String, Vec<Analysis>> {
    let mut map: HashMap<String, Vec<Analysis>> = HashMap::new();
    let combos = prefix_combos();
    let pratyayas = ["kta","ktavatu","Satf","SAnac","tumun","ktvA","lyap","lyuw","tavya","anIyar","Rvul"];
    for (dhatu_id, dhatu, _, _, _, _, _) in crate::data::DHATUS {
        for pr in pratyayas {
            let (bases, _) = crate::engine::krdanta::derive(dhatu_id, pr);
            for base in bases {
                for pref in &combos {
                    let form = if pref.is_empty() { base.clone() } else { crate::engine::prefix::apply_prefixes(pref, &base) };
                    for key in [form.clone(), normalize_nasal(&form)] {
                        let upa = if pref.is_empty() { None } else { Some(pref.join("+")) };
                        let entry = map.entry(key).or_insert_with(Vec::new);
                        if !entry.iter().any(|a| a.dhatu_id.as_deref()==Some(dhatu_id) && a.pratyaya.as_deref()==Some(pr) && a.upasarga==upa) {
                            entry.push(Analysis {
                                word: String::new(), word_type: "krdanta".to_string(),
                                dhatu: Some(dhatu.to_string()), dhatu_id: Some(dhatu_id.to_string()),
                                pratyaya: Some(pr.to_string()),
                                pratipadika: None, linga: None, vibhakti: None, vacana: None,
                                lakara: None, purusha: None, upasarga: upa.clone(),
                            });
                        }
                    }
                }
            }
        }
    }
    map
}

pub fn analyze_word(word: &str) -> Vec<Analysis> {
    let mut out: Vec<Analysis> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    // 1. Subanta/sarvanama (fast, no map)
    for m in crate::declension::subanta::analyze(word) {
        let key = format!("subanta:{}:{}:{}:{}", m.get("pratipadika").unwrap_or(&"".to_string()), m.get("linga").unwrap_or(&"".to_string()), m.get("vibhakti").unwrap_or(&"".to_string()), m.get("vacana").unwrap_or(&"".to_string()));
        if seen.insert(key) {
            out.push(Analysis {
                word: word.to_string(), word_type: "subanta".to_string(),
                dhatu: None, dhatu_id: None, pratyaya: None,
                pratipadika: m.get("pratipadika").cloned(), linga: m.get("linga").cloned(),
                vibhakti: m.get("vibhakti").cloned(), vacana: m.get("vacana").and_then(|v| v.parse().ok()),
                lakara: None, purusha: None, upasarga: None,
            });
        }
    }
    for m in crate::declension::sarvanama::analyze(word) {
        let key = format!("sarvanama:{}:{}:{}:{}", m.get("pratipadika").unwrap_or(&"".to_string()), m.get("linga").unwrap_or(&"".to_string()), m.get("vibhakti").unwrap_or(&"".to_string()), m.get("vacana").unwrap_or(&"".to_string()));
        if seen.insert(key) {
            out.push(Analysis {
                word: word.to_string(), word_type: "sarvanama".to_string(),
                dhatu: None, dhatu_id: None, pratyaya: None,
                pratipadika: m.get("pratipadika").cloned(), linga: m.get("linga").cloned(),
                vibhakti: m.get("vibhakti").cloned(), vacana: m.get("vacana").and_then(|v| v.parse().ok()),
                lakara: None, purusha: None, upasarga: None,
            });
        }
    }

    // 2. Tinanta via map (lazy)
    let tmap = TINANTA_MAP.get_or_init(build_tinanta_map);
    for key in [word.to_string(), normalize_nasal(word)] {
        if let Some(v) = tmap.get(&key) {
            for a in v {
                let sig = format!("tinanta:{}:{}:{}:{}:{:?}", a.dhatu_id.as_deref().unwrap_or(""), a.lakara.as_deref().unwrap_or(""), a.purusha.unwrap_or(0), a.vacana.unwrap_or(0), a.upasarga);
                if seen.insert(sig) {
                    let mut b = a.clone(); b.word = word.to_string(); out.push(b);
                }
            }
        }
    }
    // 3. Krdanta via map
    let kmap = KRDANTA_MAP.get_or_init(build_krdanta_map);
    for key in [word.to_string(), normalize_nasal(word)] {
        if let Some(v) = kmap.get(&key) {
            for a in v {
                let sig = format!("krdanta:{}:{}:{:?}", a.dhatu_id.as_deref().unwrap_or(""), a.pratyaya.as_deref().unwrap_or(""), a.upasarga);
                if seen.insert(sig) {
                    let mut b = a.clone(); b.word = word.to_string(); out.push(b);
                }
            }
        }
    }

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
    for stem in ["rAma", "hari", "guru", "nadI", "Bava", "gacC"] {
        if stem.starts_with(prefix) {
            results.push(stem.to_string());
            if results.len() >= limit { break; }
        }
    }
    results.truncate(limit);
    results
}
