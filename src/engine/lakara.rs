//! Port of engine/lakara.py
pub fn normalize_lakara(lakara: &str) -> (String, String) {
    let code = lakara.trim();
    // canonical -> db
    let map: &[(&str,&str)] = &[
        ("plat","plat"),("plrt","plrut"),("plot","plot"),("plan","plang"),("pvidhilin","pvidhiling"),("plit","plit"),
        ("plun","plun"),("pashirling","pashirling"),
        ("alat","alat"),("alrt","alrut"),("alot","alot"),("alan","alang"),("avidhilin","avidhiling"),("aling","aashirling"),("alit","alit"),
        ("alun","alun"),
    ];
    for (c,db) in map { if *c==code { return (c.to_string(), db.to_string()); } }
    // db -> canonical
    let rev: &[(&str,&str)] = &[
        ("plat","plat"),("plrut","plrt"),("plot","plot"),("plang","plan"),("pvidhiling","pvidhilin"),
        ("alat","alat"),("alrut","alrt"),("alot","alot"),("alang","alan"),("avidhiling","avidhilin"),("aashirling","aling"),("alit","alit"),
        ("plit","plit"),("plun","plun"),("alun","alun"),("pashirling","pashirling"),
    ];
    for (db,c) in rev { if *db==code { return (c.to_string(), db.to_string()); } }
    (code.to_string(), code.to_string())
}

pub fn lakara_family(db_lakara: &str) -> Option<String> {
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

pub fn pada_from_lakara(db_lakara: &str) -> String {
    if db_lakara=="plit" { return "P".into(); }
    if db_lakara.starts_with('a') { return "A".into(); }
    "P".into()
}

pub fn kartari_compatible(root_pada: &str, lakara: &str) -> bool {
    let (_, db) = normalize_lakara(lakara);
    if db.starts_with('a') || db=="alit" { return matches!(root_pada, "A"|"U"); }
    if db=="plit" { return matches!(root_pada, "P"|"U"); }
    if root_pada=="A" { return false; }
    true
}
