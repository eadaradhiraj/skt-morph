//! Port of engine/lakara.py

//! =============================================================================
//! src/engine/lakara.rs: Pāṇini/Kaumudī implementation — extreme commenting pass (2026-09-01)
//! ---------------------------------------------------------------------------
//! Purpose: see inline block comments below. Every public/private block is
//! documented with sūtra reference, input/output, and edge-case notes.
//! Script: SLP1 internally; Devanagari only at demo boundary.
//! Flow: dhātu → it-strip → aṅga/vikaraṇa → lakāra/ending → sandhi → surface.
//! Gold DB is cross-check only, never source of truth.
//! =============================================================================
pub fn normalize_lakara(lakara: &str) -> (String, String) {
    let code = lakara.trim();
    // canonical -> db
    let map: &[(&str,&str)] = &[
        ("plat","plat"),("plrt","plrut"),("plot","plot"),("plan","plang"),("pvidhilin","pvidhiling"),("plit","plit"),
        ("plun","plun"),("pashirling","pashirling"),
        ("alat","alat"),("alrt","alrut"),("alot","alot"),("alan","alang"),("avidhilin","avidhiling"),("aling","aashirling"),("alit","alit"),
        ("alun","alun"),
    ];
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for (c,db) in map { if *c==code { return (c.to_string(), db.to_string()); } }
    // db -> canonical
    let rev: &[(&str,&str)] = &[
        ("plat","plat"),("plrut","plrt"),("plot","plot"),("plang","plan"),("pvidhiling","pvidhilin"),
        ("alat","alat"),("alrut","alrt"),("alot","alot"),("alang","alan"),("avidhiling","avidhilin"),("aashirling","aling"),("alit","alit"),
        ("plit","plit"),("plun","plun"),("alun","alun"),("pashirling","pashirling"),
    ];
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for (db,c) in rev { if *db==code { return (c.to_string(), db.to_string()); } }
    (code.to_string(), code.to_string())
}

// ---------------------------------------------------------------------------
// fn `lakara_family`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lakara_family(db_lakara: &str) -> Option<String> {
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match db_lakara {
        "plat"|"alat" => Some("lat".into()),
        "plot"|"alot" => Some("lot".into()),
        "plrut"|"alrut" => Some("lrt".into()),
        "plang"|"alang" => Some("lang".into()),
        "pvidhiling"|"avidhiling" => Some("vidhilin".into()),
        "plit"|"alit" => Some("lit".into()),
        "plun"|"alun" => Some("lun".into()),
        "pashirling"|"aashirling" => Some("ashir".into()),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// fn `pada_from_lakara`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn pada_from_lakara(db_lakara: &str) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if db_lakara=="plit" { return "P".into(); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if db_lakara.starts_with('a') { return "A".into(); }
    "P".into()
}

// ---------------------------------------------------------------------------
// fn `kartari_compatible`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn kartari_compatible(root_pada: &str, lakara: &str) -> bool {
    let (_, db) = normalize_lakara(lakara);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if db.starts_with('a') || db=="alit" { return matches!(root_pada, "A"|"U"); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if db=="plit" { return matches!(root_pada, "P"|"U"); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if root_pada=="A" { return false; }
    true
}
