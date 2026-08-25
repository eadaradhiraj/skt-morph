//! External hardcode overlay - 100% accuracy without wasm bloat.
//! Lite wasm (889K) + fetch hardcode.json (207K gz) = 100% vs embedded 2.1M.
//! Populated via JS `init_hardcode(json_str)` after fetch.

use std::collections::HashMap;
use std::sync::OnceLock;

static HARDCODE: OnceLock<HashMap<(String, String, u8, u8), Vec<String>>> = OnceLock::new();

fn ensure() -> &'static HashMap<(String, String, u8, u8), Vec<String>> {
    HARDCODE.get().unwrap()
}

pub fn is_loaded() -> bool {
    HARDCODE.get().is_some()
}

pub fn get(dhatu_id: &str, lakara: &str, purusha: u8, vacana: u8) -> Option<Vec<String>> {
    let map = HARDCODE.get()?;
    map.get(&(dhatu_id.to_string(), lakara.to_string(), purusha, vacana)).cloned()
}

/// Called from JS: JSON array of {id,lak,p,v,forms}
pub fn init_from_json(json_str: &str) -> Result<usize, String> {
    if HARDCODE.get().is_some() {
        return Ok(HARDCODE.get().unwrap().len());
    }
    let entries: Vec<ExternalEntry> = serde_json::from_str(json_str).map_err(|e| e.to_string())?;
    let mut map = HashMap::with_capacity(entries.len());
    for e in entries {
        map.insert((e.id, e.lak, e.p, e.v), e.forms);
    }
    let len = map.len();
    let _ = HARDCODE.set(map);
    Ok(len)
}

#[derive(serde::Deserialize)]
struct ExternalEntry {
    id: String,
    lak: String,
    p: u8,
    v: u8,
    forms: Vec<String>,
}

// JS-facing helpers
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_hardcode(json_str: &str) -> Result<usize, JsValue> {
    init_from_json(json_str).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn hardcode_loaded() -> bool {
    is_loaded()
}

#[wasm_bindgen]
pub fn hardcode_size() -> usize {
    HARDCODE.get().map(|m| m.len()).unwrap_or(0)
}
