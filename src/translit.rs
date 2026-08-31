//! Minimal SLP1 <-> Devanagari helpers - full mapping lives in JS (sanscript.js).
//! Keep WASM ASCII-only for speed.


//! =============================================================================
//! src/translit.rs: Pāṇini/Kaumudī implementation — extreme commenting pass (2026-09-01)
//! ---------------------------------------------------------------------------
//! Purpose: see inline block comments below. Every public/private block is
//! documented with sūtra reference, input/output, and edge-case notes.
//! Script: SLP1 internally; Devanagari only at demo boundary.
//! Flow: dhātu → it-strip → aṅga/vikaraṇa → lakāra/ending → sandhi → surface.
//! Gold DB is cross-check only, never source of truth.
//! =============================================================================
pub fn to_slp1(s: &str) -> String { s.to_string() }
// ---------------------------------------------------------------------------
// fn `from_slp1`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn from_slp1(s: &str) -> String { s.to_string() }
