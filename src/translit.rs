//! Minimal SLP1 <-> Devanagari helpers - full mapping lives in JS (sanscript.js).
//! Keep WASM ASCII-only for speed.

pub fn to_slp1(s: &str) -> String { s.to_string() }
pub fn from_slp1(s: &str) -> String { s.to_string() }
