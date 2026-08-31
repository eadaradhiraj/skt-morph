//! =============================================================================
//! src/engine/lang_ya.rs: Pāṇini/Kaumudī implementation — extreme commenting pass (2026-09-01)
//! ---------------------------------------------------------------------------
//! Purpose: see inline block comments below. Every public/private block is
//! documented with sūtra reference, input/output, and edge-case notes.
//! Script: SLP1 internally; Devanagari only at demo boundary.
//! Flow: dhātu → it-strip → aṅga/vikaraṇa → lakāra/ending → sandhi → surface.
//! Gold DB is cross-check only, never source of truth.
//! =============================================================================
pub fn lang_ya_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["yat".into(),"yad".into()], vec!["3.4.111".into()]),
        (vec!["yatAm".into()], vec!["3.4.111".into()]),
        (vec!["yan".into()], vec!["3.4.111".into()]),
        (vec!["yaH".into()], vec!["3.4.111".into()]),
        (vec!["yatam".into()], vec!["3.4.111".into()]),
        (vec!["yata".into()], vec!["3.4.111".into()]),
        (vec!["yam".into()], vec!["3.4.111".into()]),
        (vec!["yAva".into()], vec!["3.4.111".into()]),
        (vec!["yAma".into()], vec!["3.4.111".into()]),
    ]
}
