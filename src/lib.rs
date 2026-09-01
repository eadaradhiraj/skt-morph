//! skt-morph — Pāṇini as ordered in the Siddhānta-Kaumudī.
//! * `data`      — dhātupāṭha + gold cross-check (never source of truth).
//! * `engine`    — tinanta / kṛdanta / taddhita live generation (sūtra-driven).
//! * `declension`— subanta + sarvanāma (ending-class, not site scrape).
//! * `translit`  — SLP1 internally; Devanagari only at JS boundary (www/translit.js).
//!
//! Flow: dhātu → it-strip (1.3.2–9) → aṅga/vikaraṇa → lakāra/ending → sandhi.
#![deny(warnings)]
use wasm_bindgen::prelude::*;

pub mod data;
pub mod declension;
pub mod engine;
pub mod translit;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn init() {}

/// Generate single tinanta form. SLP1 dhatu/id + lakara (plat/plrt/…), puruṣa/vacana 1–3 (clamped).
#[wasm_bindgen]
pub fn generate_verb(dhatu: &str, lakara: &str, purusha: u8, vacana: u8) -> JsValue {
    let result = engine::tinanta::generate(dhatu, lakara, purusha, vacana);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Tinanta with upasargas (comma-separated SLP1) + optional artha (1.3).
#[wasm_bindgen]
pub fn generate_verb_with_prefix(dhatu: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &str, artha: &str) -> JsValue {
    let prefs: Vec<String> = prefixes.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let result = engine::tinanta::generate_with_artha(dhatu, lakara, purusha, vacana, &prefs, artha);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Full 9-form tinanta paradigm (3 puruṣa × 3 vacana).
#[wasm_bindgen]
pub fn generate_verb_paradigm(dhatu: &str, lakara: &str) -> JsValue {
    let result = engine::tinanta::generate_paradigm(dhatu, lakara);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn generate_verb_paradigm_with_prefix(dhatu: &str, lakara: &str, prefixes: &str, artha: &str) -> JsValue {
    let prefs: Vec<String> = prefixes.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let result = engine::tinanta::generate_paradigm_with_artha(dhatu, lakara, &prefs, artha);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Derived tinanta: `Ric`/`san`/`yaN`/`yaNluk`/`karma` (see `engine::derived` + `www` dropdown, Kaumudī 3.1.26/3.1.22/2.4.74).
/// yaNluk = 2.4.74 ya-luk intensive (parasmai boBUti vs yaN boBUyate ātmane).
#[wasm_bindgen]
pub fn generate_verb_derived(dhatu: &str, derivation: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &str, artha: &str) -> JsValue {
    let prefs: Vec<String> = prefixes.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let result = engine::tinanta::generate_derived_artha(dhatu, derivation, lakara, purusha, vacana, &prefs, artha);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn generate_verb_paradigm_derived(dhatu: &str, derivation: &str, lakara: &str, prefixes: &str, artha: &str) -> JsValue {
    let prefs: Vec<String> = prefixes.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let result = engine::tinanta::generate_paradigm_derived_artha(dhatu, derivation, lakara, &prefs, artha);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Taddhita (4.1): tva/tal/matup/mayaT/in/tarap/tamap/Ca/ka/aN/Dak/yaY etc.
#[wasm_bindgen]
pub fn generate_taddhita(pratipadika: &str, pratyaya: &str) -> JsValue {
    let result = engine::taddhita::generate(pratipadika, pratyaya);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Kṛdanta generation (3.1–3.4): kta/ktavatu/…/kyap etc.
#[wasm_bindgen]
pub fn generate_krdanta(dhatu: &str, pratyaya: &str) -> JsValue {
    let result = engine::krdanta::generate(dhatu, pratyaya);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn generate_krdanta_with_prefix(dhatu: &str, pratyaya: &str, prefixes: &str) -> JsValue {
    let prefs: Vec<String> = prefixes.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let result = engine::krdanta::generate_with_prefixes(dhatu, pratyaya, &prefs);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Liṅgas a kṛt takes (pum/stri/nap); empty = avyaya (ktvA/tumun/lyap).
#[wasm_bindgen]
pub fn krdanta_lingas(pratyaya: &str) -> JsValue {
    serde_wasm_bindgen::to_value(engine::krdanta::lingas(pratyaya)).unwrap_or(JsValue::NULL)
}

/// Decline kṛdanta where it takes sup (kta/śatṛ/tṛc …); avyaya → null.
#[wasm_bindgen]
pub fn generate_krdanta_declension(dhatu: &str, pratyaya: &str, linga: &str, prefixes: &str) -> JsValue {
    let prefs: Vec<String> = prefixes.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    match engine::krdanta::decline(dhatu, pratyaya, linga, &prefs) {
        Some(d) => serde_wasm_bindgen::to_value(&d).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Subanta generation (sup 4.1.2, halanta via 8.2.30/39/66).
#[wasm_bindgen]
pub fn generate_noun(base: &str, linga: &str) -> JsValue {
    let result = declension::subanta::generate(base, linga);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Sarvanāma generation (1.1.27 etc.).
#[wasm_bindgen]
pub fn generate_pronoun(base: &str, linga: &str) -> JsValue {
    let result = declension::sarvanama::generate(base, linga);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Reverse lookup: one surface form (SLP1) → every valid parse (tinanta/kṛdanta/subanta/sarvanāma, upasarga-peeled, Kaumudī).
#[wasm_bindgen]
pub fn analyze(word: &str) -> JsValue {
    let result = engine::analyze::analyze_word(word);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn search(prefix: &str, limit: Option<usize>) -> JsValue {
    let result = engine::analyze::search_prefix(prefix, limit.unwrap_or(10));
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}
