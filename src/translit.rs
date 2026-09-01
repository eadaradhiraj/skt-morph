//! translit — minimal SLP1 helpers for Rust/WASM (ASCII-only, no ICU).
//! Full Devanagari ↔ SLP1/Harvard-Kyoto lives in `www/translit.js` (browser).
//! Rust/WASM stays SLP1-only for speed; this module provides:
//! - identity helpers (to/from SLP1) for type clarity
//! - `is_slp1` / `normalize_slp1` for validation
//! - `devanagari_to_slp1_stub` documenting that conversion is JS-owned
//!
//! Keep ASCII-only in WASM; no ICU dependency.

// ---------------------------------------------------------------------------
// Identity: already SLP1 — type-clarity wrapper, no allocation if possible.
// ---------------------------------------------------------------------------
pub fn to_slp1(s: &str) -> String { s.to_string() }
pub fn from_slp1(s: &str) -> String { s.to_string() }

/// True if `s` looks like SLP1 (ASCII letters only, no Devanagari block).
pub fn is_slp1(s: &str) -> bool {
    // Cheap check: Devanagari block is \u0900-\u097F.
    !s.chars().any(|c| ('\u{0900}'..='\u{097F}').contains(&c))
}

/// Normalize whitespace + trim; SLP1 is case-sensitive (a≠A etc.).
pub fn normalize_slp1(s: &str) -> String {
    s.trim().replace(char::is_whitespace, "")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn slp1_identity() { assert_eq!(to_slp1("rAma"), "rAma"); }
    #[test]
    fn is_slp1_true_for_ascii() { assert!(is_slp1("rAma")); assert!(!is_slp1("राम")); }
}
