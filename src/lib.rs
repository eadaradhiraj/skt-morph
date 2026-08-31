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
pub fn init() {
    // panic hook for better errors in browser
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Generate verb conjugation paradigm (9 forms) for a dhatu + lakara
/// dhatu: SLP1 string like "BU" or dhatu_id "01.0001"
/// lakara: "plat" | "plrt" | "plot" | "plan" | "pvidhilin" | "plit" | "alat" etc.
#[wasm_bindgen]
pub fn generate_verb(dhatu: &str, lakara: &str, purusha: u8, vacana: u8) -> JsValue {
    let result = engine::tinanta::generate(dhatu, lakara, purusha, vacana);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn generate_verb_with_prefix(dhatu: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &str, artha: &str) -> JsValue {
    let prefs: Vec<String> = prefixes.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let result = engine::tinanta::generate_with_artha(dhatu, lakara, purusha, vacana, &prefs, artha);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

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

/// `derivation`: empty / `shuddha`, or `Ric` / `san` / `yaN` / `karma`.
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

/// तद्धित (त्व / तल् / मतुप् / मयट् / इन् / तरप् / तमप् / छ / क / अण् / ढक् / यञ्)
#[wasm_bindgen]
pub fn generate_taddhita(pratipadika: &str, pratyaya: &str) -> JsValue {
    let result = engine::taddhita::generate(pratipadika, pratyaya);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Generate participle / krdanta
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

/// Generate noun declension table
#[wasm_bindgen]
pub fn generate_noun(base: &str, linga: &str) -> JsValue {
    let result = declension::subanta::generate(base, linga);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Generate pronoun declension
#[wasm_bindgen]
pub fn generate_pronoun(base: &str, linga: &str) -> JsValue {
    let result = declension::sarvanama::generate(base, linga);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Analyze / search: reverse lookup
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
