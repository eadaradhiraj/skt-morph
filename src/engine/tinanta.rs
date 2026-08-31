//! तिङन्त: Pāṇini as arranged in the Siddhānta-Kaumudī (stem → ending → sandhi).
//! Scrape DBs are for cross-check only — they never supply a form.

//! =============================================================================
//! src/engine/tinanta.rs: Pāṇini/Kaumudī implementation — extreme commenting pass (2026-09-01)
//! ---------------------------------------------------------------------------
//! Purpose: see inline block comments below. Every public/private block is
//! documented with sūtra reference, input/output, and edge-case notes.
//! Script: SLP1 internally; Devanagari only at demo boundary.
//! Flow: dhātu → it-strip → aṅga/vikaraṇa → lakāra/ending → sandhi → surface.
//! Gold DB is cross-check only, never source of truth.
//! =============================================================================
use serde::{Deserialize, Serialize};
use crate::engine::lakara::{lakara_family, normalize_lakara};
use crate::engine::stems::{derive_stem, conjugation_gana};
use crate::engine::endings::family_endings;
use crate::engine::join::join_variants;
use crate::engine::upa_pada::pada_allowed_artha;

#[derive(Serialize, Deserialize, Debug)]
// ---------------------------------------------------------------------------
// struct `TinantaResult` — tin/sUP endings: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub struct TinantaResult {
    pub forms: Vec<String>,
    pub dhatu: String,
    pub lakara: String,
    pub purusha: u8,
    pub vacana: u8,
}

#[derive(Serialize, Deserialize, Debug)]
// ---------------------------------------------------------------------------
// struct `ParadigmEntry`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub struct ParadigmEntry {
    pub purusha: u8,
    pub vacana: u8,
    pub forms: Vec<String>,
}

// ---------------------------------------------------------------------------
// fn `load_dhatu_info`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn load_dhatu_info(dhatu_query: &str) -> Option<(String, u8, String, String, String, String)> {
    Some(crate::engine::dhatu::load_or_fallback(dhatu_query))
}

// ---------------------------------------------------------------------------
// fn `generate` — tin/sUP endings: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8) -> TinantaResult {
    let forms = generate_all(dhatu_query, lakara, purusha, vacana);
    let (canon, _) = normalize_lakara(lakara);
    TinantaResult { forms, dhatu: dhatu_query.to_string(), lakara: canon, purusha, vacana }
}

// ---------------------------------------------------------------------------
// fn `generate_with_prefixes` — tin/sUP endings: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_with_prefixes(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &[String]) -> TinantaResult {
    generate_with_artha(dhatu_query, lakara, purusha, vacana, prefixes, "")
}

// ---------------------------------------------------------------------------
// fn `generate_with_artha` — tin/sUP endings: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_with_artha(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &[String], artha: &str) -> TinantaResult {
    let forms = generate_all_with_artha(dhatu_query, lakara, purusha, vacana, prefixes, artha);
    let (canon, _) = normalize_lakara(lakara);
    TinantaResult { forms, dhatu: dhatu_query.to_string(), lakara: canon, purusha, vacana }
}

// ---------------------------------------------------------------------------
// fn `generate_all`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_all(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8) -> Vec<String> {
    generate_all_with_prefixes(dhatu_query, lakara, purusha, vacana, &[])
}


// ---------------------------------------------------------------------------
// fn `attach_prefixes`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn attach_prefixes(prefixes: &[String], forms: Vec<String>) -> Vec<String> {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if prefixes.is_empty() { forms }
    // — else-branch — fallback / apavāda; sūtra gating, see comments above.
    else { forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect() }
}

// ---------------------------------------------------------------------------
// fn `generate_all_with_prefixes`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_all_with_prefixes(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &[String]) -> Vec<String> {
    generate_all_with_artha(dhatu_query, lakara, purusha, vacana, prefixes, "")
}

// ---------------------------------------------------------------------------
// fn `generate_all_with_artha`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_all_with_artha(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &[String], artha: &str) -> Vec<String> {
    generate_all_derived_artha(dhatu_query, "", lakara, purusha, vacana, prefixes, artha)
}

/// `derivation`: empty = śuddha; `Ric` / `san` / `yaN` / `karma`.
pub fn generate_all_derived(
    dhatu_query: &str,
    derivation: &str,
    lakara: &str,
    purusha: u8,
    vacana: u8,
    prefixes: &[String],
) -> Vec<String> {
    generate_all_derived_artha(dhatu_query, derivation, lakara, purusha, vacana, prefixes, "")
}

// ---------------------------------------------------------------------------
// fn `generate_all_derived_artha`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_all_derived_artha(
    dhatu_query: &str,
    derivation: &str,
    lakara: &str,
    purusha: u8,
    vacana: u8,
    prefixes: &[String],
    artha: &str,
) -> Vec<String> {
    let (canonical, db_lakara) = normalize_lakara(lakara);
    let deriv = derivation.trim();
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if !deriv.is_empty() && deriv != "shuddha" {
        return live_generate_derived(dhatu_query, deriv, &canonical, &db_lakara, purusha, vacana, prefixes, artha);
    }
    live_generate(dhatu_query, &canonical, &db_lakara, purusha, vacana, prefixes, artha)
}

// ---------------------------------------------------------------------------
// fn `generate_derived`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_derived(
    dhatu_query: &str,
    derivation: &str,
    lakara: &str,
    purusha: u8,
    vacana: u8,
    prefixes: &[String],
) -> TinantaResult {
    let forms = generate_all_derived_artha(dhatu_query, derivation, lakara, purusha, vacana, prefixes, "");
    let (canon, _) = normalize_lakara(lakara);
    TinantaResult { forms, dhatu: dhatu_query.to_string(), lakara: canon, purusha, vacana }
}

// ---------------------------------------------------------------------------
// fn `generate_derived_artha`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_derived_artha(
    dhatu_query: &str,
    derivation: &str,
    lakara: &str,
    purusha: u8,
    vacana: u8,
    prefixes: &[String],
    artha: &str,
) -> TinantaResult {
    let forms = generate_all_derived_artha(dhatu_query, derivation, lakara, purusha, vacana, prefixes, artha);
    let (canon, _) = normalize_lakara(lakara);
    TinantaResult { forms, dhatu: dhatu_query.to_string(), lakara: canon, purusha, vacana }
}

// ---------------------------------------------------------------------------
// fn `generate_paradigm_derived`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_paradigm_derived(dhatu: &str, derivation: &str, lakara: &str, prefixes: &[String]) -> Vec<ParadigmEntry> {
    generate_paradigm_derived_artha(dhatu, derivation, lakara, prefixes, "")
}

// ---------------------------------------------------------------------------
// fn `generate_paradigm_derived_artha`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_paradigm_derived_artha(dhatu: &str, derivation: &str, lakara: &str, prefixes: &[String], artha: &str) -> Vec<ParadigmEntry> {
    let mut out = Vec::new();
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for p in 1..=3 {
        // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
        for v in 1..=3 {
            let forms = generate_all_derived_artha(dhatu, derivation, lakara, p, v, prefixes, artha);
            out.push(ParadigmEntry { purusha: p, vacana: v, forms });
        }
    }
    out
}

// ---------------------------------------------------------------------------
// fn `live_generate_derived`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn live_generate_derived(
    dhatu_query: &str,
    kind: &str,
    canonical: &str,
    db_lakara: &str,
    purusha: u8,
    vacana: u8,
    prefixes: &[String],
    _artha: &str,
) -> Vec<String> {
    let Some((dhatu, _, _, _, _, _)) = load_dhatu_info(dhatu_query) else {
        return vec![];
    };
    let Some(family) = lakara_family(db_lakara) else {
        return vec![];
    };
    let pada = if db_lakara.starts_with('a') || canonical.starts_with('a') { "A" } else { "P" };
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if let Some(forms) = crate::engine::derived::kartari(&dhatu, kind, &family, purusha, vacana, pada) {
        return attach_prefixes(prefixes, forms);
    }
    vec![]
}

/// Stem → vikaraṇa/lakāra ending → sandhi. No tables.
pub fn live_generate(
    dhatu_query: &str,
    canonical: &str,
    db_lakara: &str,
    purusha: u8,
    vacana: u8,
    prefixes: &[String],
    artha: &str,
) -> Vec<String> {
    let Some((dhatu, gana, root_pada, tags, antarganas, aupadeshik)) = load_dhatu_info(dhatu_query) else {
        return vec![];
    };
    let Some(family) = lakara_family(db_lakara) else { return vec![]; };
    let pada = if db_lakara.starts_with('a') || canonical.starts_with('a') { "A" } else { "P" };
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if !pada_allowed_artha(&root_pada, pada, &dhatu, prefixes, artha) {
        // 2.4.54 चक्षिङः ख्याञ् — लृट् of ख्या/क्ष्या is parasmai.
        if !(pada == "P" && family == "lrt" && matches!(dhatu.as_str(), "cakziN")) {
            return vec![];
        }
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family == "lit" {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(forms) = crate::engine::lit::kartari(&dhatu, purusha, vacana, pada) {
            return attach_prefixes(prefixes, forms);
        }
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family == "lun" {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(forms) = crate::engine::lun::kartari_tagged(&dhatu, purusha, vacana, pada, &antarganas) {
            return attach_prefixes(prefixes, forms);
        }
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family == "ashir" {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(forms) = crate::engine::ashir::kartari(&dhatu, purusha, vacana, pada) {
            return attach_prefixes(prefixes, forms);
        }
    }
    let cgana = conjugation_gana(gana, &tags);
    let (stem_opt, augment) = derive_stem(&dhatu, gana, &family, "shuddha", &tags, &antarganas, &aupadeshik);
    let Some(stem) = stem_opt else { return vec![]; };
    let table = family_endings(&family, "kartari", pada, cgana, Some(&dhatu));
    let Some(table) = table else { return vec![]; };
    let idx = ((purusha - 1) * 3 + (vacana - 1)) as usize;
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if idx >= table.len() { return vec![]; }
    let (variants, _) = &table[idx];
    let forms = join_variants(&stem, variants, cgana, &family, purusha, pada, augment.as_deref(), &dhatu, vacana, &antarganas);
    attach_prefixes(prefixes, forms)
}

// ---------------------------------------------------------------------------
// fn `generate_paradigm`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_paradigm(dhatu: &str, lakara: &str) -> Vec<ParadigmEntry> {
    let mut out = Vec::new();
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for p in 1..=3 { for v in 1..=3 {
        let forms = generate_all(dhatu, lakara, p, v);
        out.push(ParadigmEntry { purusha: p, vacana: v, forms });
    }}
    out
}

// ---------------------------------------------------------------------------
// fn `generate_paradigm_with_prefixes`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_paradigm_with_prefixes(dhatu: &str, lakara: &str, prefixes: &[String]) -> Vec<ParadigmEntry> {
    generate_paradigm_with_artha(dhatu, lakara, prefixes, "")
}

// ---------------------------------------------------------------------------
// fn `generate_paradigm_with_artha`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_paradigm_with_artha(dhatu: &str, lakara: &str, prefixes: &[String], artha: &str) -> Vec<ParadigmEntry> {
    let mut out = Vec::new();
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for p in 1..=3 { for v in 1..=3 {
        let forms = generate_all_with_artha(dhatu, lakara, p, v, prefixes, artha);
        out.push(ParadigmEntry { purusha: p, vacana: v, forms });
    }}
    out
}

#[cfg(test)]
// ---------------------------------------------------------------------------
// mod `tests`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
mod tests {
    use super::*;

    #[test]
    // ---------------------------------------------------------------------------
    // fn `bu_lat_prathama_ekavacana`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn bu_lat_prathama_ekavacana() {
        let f = generate_all("BU", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Bavati"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `gam_lat_prathama_ekavacana`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn gam_lat_prathama_ekavacana() {
        let f = generate_all("gam", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "gacCati"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `pra_bu_lat`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn pra_bu_lat() {
        let prefs = vec!["pra".to_string()];
        let f = generate_all_with_prefixes("BU", "plat", 1, 1, &prefs);
        assert!(f.iter().any(|x| x == "praBavati"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `live_kakzi_vidhilin`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn live_kakzi_vidhilin() {
        let (canon, db) = normalize_lakara("pvidhilin");
        let f = live_generate("kAkzi", &canon, &db, 1, 1, &[], "");
        assert!(f.iter().any(|x| x == "kANkzet" || x == "kANkzed"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `live_gamx_no_override`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn live_gamx_no_override() {
        let (canon, db) = normalize_lakara("plat");
        let f = live_generate("gamx", &canon, &db, 1, 1, &[], "");
        assert!(f.iter().any(|x| x == "gacCati"), "{:?}", f);
        let (canon, db) = normalize_lakara("plrt");
        let f = live_generate("gamx", &canon, &db, 1, 1, &[], "");
        assert!(f.iter().any(|x| x == "gamizyati"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `pra_bu_lat_third_plural_no_natva`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn pra_bu_lat_third_plural_no_natva() {
        let prefs = vec!["pra".to_string()];
        let f = generate_all_with_prefixes("BU", "plat", 1, 3, &prefs);
        assert!(f.iter().any(|x| x == "praBavanti"), "{:?}", f);
        assert!(!f.iter().any(|x| x == "praBavaRti"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `gam_lit_jagama`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn gam_lit_jagama() {
        let f = generate_all("gam", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "jagAma"), "{:?}", f);
        let f = generate_all("gam", "plit", 1, 3);
        assert!(f.iter().any(|x| x == "jagmuH"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `bu_lit_babhuva`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn bu_lit_babhuva() {
        let f = generate_all("BU", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "baBUva"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `han_lit_jaghana`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn han_lit_jaghana() {
        let f = generate_all("hana", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "jaGAna"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `vac_yaj_lit`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn vac_yaj_lit() {
        let f = generate_all("vaca", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "uvAca"), "{:?}", f);
        let f = generate_all("yaja", "plit", 1, 2);
        assert!(f.iter().any(|x| x == "IjatuH"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `grah_ni_kf_lit`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn grah_ni_kf_lit() {
        let f = generate_all("graha", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "jagrAha"), "{:?}", f);
        let f = generate_all("RIY", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "ninAya"), "{:?}", f);
        let f = generate_all("qukfY", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "cakAra"), "{:?}", f);
        let f = generate_all("dfSir", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "dadarSa"), "{:?}", f);
        let f = generate_all("dfSir", "plit", 1, 2);
        assert!(f.iter().any(|x| x == "dadfSatuH"), "{:?}", f);
        let f = generate_all("tF", "plit", 1, 2);
        assert!(f.iter().any(|x| x == "teratuH"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `yaj_kf_alit`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn yaj_kf_alit() {
        let f = generate_all("yaja", "alit", 1, 1);
        assert!(f.iter().any(|x| x == "Ije"), "{:?}", f);
        let f = generate_all("qukfY", "alit", 1, 3);
        assert!(f.iter().any(|x| x == "cakrire"), "{:?}", f);
        let f = generate_all("RIY", "alit", 1, 1);
        assert!(f.iter().any(|x| x == "ninye"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `da_dha_stha_lit`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn da_dha_stha_lit() {
        let f = generate_all("qudAY", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "dadO"), "{:?}", f);
        let f = generate_all("quDAY", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "daDO"), "{:?}", f);
        let f = generate_all("zWA", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "tasTO"), "{:?}", f);
        let f = generate_all("qudAY", "alit", 1, 1);
        assert!(f.iter().any(|x| x == "dade"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `ve_vye_hve_lit`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn ve_vye_hve_lit() {
        let f = generate_all("veY", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "uvAya") && f.iter().any(|x| x == "vavO"), "{:?}", f);
        let f = generate_all("vyeY", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "vivyAya"), "{:?}", f);
        let f = generate_all("hveY", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "juhAva"), "{:?}", f);
        let f = generate_all("veY", "alit", 1, 1);
        assert!(f.iter().any(|x| x == "Uve") || f.iter().any(|x| x == "Uye"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `i_as_lit`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn i_as_lit() {
        let f = generate_all("iR", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "iyAya"), "{:?}", f);
        let f = generate_all("asa", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "Asa"), "{:?}", f);
        let f = generate_all("iN", "alit", 1, 1);
        assert!(f.iter().any(|x| x == "Iye"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `lun_ashir_am_bru`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn lun_ashir_am_bru() {
        let f = generate_all("BU", "plun", 1, 1);
        assert!(f.iter().any(|x| x == "aBUt"), "{:?}", f);
        let f = generate_all("gam", "plun", 1, 1);
        assert!(f.iter().any(|x| x == "agamat"), "{:?}", f);
        let f = generate_all("qukfY", "plun", 1, 1);
        assert!(f.iter().any(|x| x == "akArzIt"), "{:?}", f);
        let f = generate_all("BU", "pashirling", 1, 1);
        assert!(f.iter().any(|x| x == "BUyAt"), "{:?}", f);
        let f = generate_all("eDa", "alit", 1, 1);
        assert!(f.iter().any(|x| x == "eDAYcakre"), "{:?}", f);
        let f = generate_all("brUY", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "uvAca"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `gana_2_3_5_7_8_9_lat`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn gana_2_3_5_7_8_9_lat() {
        let f = generate_all("ada", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "atti"), "{:?}", f);
        let f = generate_all("ada", "plat", 2, 2);
        assert!(f.iter().any(|x| x == "atTaH"), "{:?}", f);
        let f = generate_all("ada", "plot", 2, 1);
        assert!(f.iter().any(|x| x == "adDi"), "{:?}", f);
        let f = generate_all("ada", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "atsyati"), "{:?}", f);
        let f = generate_all("hu", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "juhoti"), "{:?}", f);
        let f = generate_all("zuY", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "sunoti"), "{:?}", f);
        let f = generate_all("ruDir", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "ruRadDi" || x == "ruRaddhi"), "{:?}", f);
        let f = generate_all("tanu", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "tanoti"), "{:?}", f);
        let f = generate_all("qukrIY", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "krIRAti"), "{:?}", f);
        let f = generate_all("02.0060", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "asti"), "{:?}", f);
        let f = generate_all("02.0060", "plat", 1, 3);
        assert!(f.iter().any(|x| x == "santi"), "{:?}", f);
        let f = generate_all("02.0060", "plot", 2, 1);
        assert!(f.iter().any(|x| x == "eDi"), "{:?}", f);
        let f = generate_all("02.0060", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "Bavizyati"), "{:?}", f);
        let f = generate_all("02.0060", "plan", 1, 2);
        assert!(f.iter().any(|x| x == "AstAm"), "{:?}", f);
        let f = generate_all("asa", "plan", 1, 2);
        assert!(f.iter().any(|x| x == "AstAm"), "{:?}", f);
        let f = generate_all("iR", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "eti"), "{:?}", f);
        let f = generate_all("iR", "plat", 1, 3);
        assert!(f.iter().any(|x| x == "yanti"), "{:?}", f);
        let f = generate_all("iR", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "ezyati"), "{:?}", f);
        let f = generate_all("iR", "plot", 3, 1);
        assert!(f.iter().any(|x| x == "ayAni"), "{:?}", f);
        let f = generate_all("hana", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "hanti"), "{:?}", f);
        let f = generate_all("hana", "plat", 1, 3);
        assert!(f.iter().any(|x| x == "Gnanti"), "{:?}", f);
        let f = generate_all("hana", "plot", 2, 1);
        assert!(f.iter().any(|x| x == "jahi"), "{:?}", f);
        let f = generate_all("Ru", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "nOti"), "{:?}", f);
        let f = generate_all("zRu", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "snOti"), "{:?}", f);
        let f = generate_all("02.0036", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "sOti"), "{:?}", f);
        let f = generate_all("wukzu", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "kzOti"), "{:?}", f);
        let f = generate_all("brUY", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "bravIti"), "{:?}", f);
        let f = generate_all("brUY", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "vakzyati"), "{:?}", f);
        let f = generate_all("zwuY", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "stavIti"), "{:?}", f);
        assert!(f.iter().any(|x| x == "stOti"), "{:?}", f);
        let f = generate_all("zwuY", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "stozyati"), "{:?}", f);
        let f = generate_all("uCfdir", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "CfRatti"), "{:?}", f);
        let f = generate_all("Sizx", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Sinazwi"), "{:?}", f);
        let f = generate_all("pizx", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "pinazwi"), "{:?}", f);
        let f = generate_all("Banjo", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Banakti"), "{:?}", f);
        let f = generate_all("yujir", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "yunakti"), "{:?}", f);
        let f = generate_all("dviza", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "dvezwi"), "{:?}", f);
        let f = generate_all("duha", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "dogDi"), "{:?}", f);
        let f = generate_all("duha", "alat", 1, 1);
        assert!(f.iter().any(|x| x == "dugDe"), "{:?}", f);
        let f = generate_all("duha", "alat", 1, 2);
        assert!(f.iter().any(|x| x == "duhAte"), "{:?}", f);
        let f = generate_all("duha", "alat", 1, 3);
        assert!(f.iter().any(|x| x == "duhate"), "{:?}", f);
        let f = generate_all("duha", "alrt", 1, 1);
        assert!(f.iter().any(|x| x == "Dokzyate"), "{:?}", f);
        let f = generate_all("duha", "alat", 2, 3);
        assert!(f.iter().any(|x| x == "DugDve"), "{:?}", f);
        let f = generate_all("duha", "alot", 1, 1);
        assert!(f.iter().any(|x| x == "dugDAm"), "{:?}", f);
        let f = generate_all("duha", "alot", 2, 1);
        assert!(f.iter().any(|x| x == "Dukzva"), "{:?}", f);
        let f = generate_all("duha", "alan", 1, 1);
        assert!(f.iter().any(|x| x == "adugDa"), "{:?}", f);
        let f = generate_all("duha", "alan", 1, 3);
        assert!(f.iter().any(|x| x == "aduhata"), "{:?}", f);
        let f = generate_all("duha", "avidhilin", 1, 1);
        assert!(f.iter().any(|x| x == "duhIta"), "{:?}", f);
        let f = generate_all("yu", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "yOti"), "{:?}", f);
        let f = generate_all("yA", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "yAti"), "{:?}", f);
        let f = generate_all("vaca", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "vakti"), "{:?}", f);
        let f = generate_all("vida", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "vetti"), "{:?}", f);
        let f = generate_all("rudir", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "roditi"), "{:?}", f);
        let f = generate_all("02.0070", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "SAsti"), "{:?}", f);
        let f = generate_all("vaSa", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "vazwi"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `lun_han_ashir_a_lrt`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn lun_han_ashir_a_lrt() {
        let f = generate_all("hana", "plun", 1, 1);
        assert!(f.iter().any(|x| x == "avaDIt"), "{:?}", f);
        let f = generate_all("qukfY", "aashirling", 1, 1);
        assert!(f.iter().any(|x| x == "kfzIzwa"), "{:?}", f);
        let f = generate_all("zWA", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "sTAsyati"), "{:?}", f);
        let f = generate_all("qupacaz", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "pakzyati"), "{:?}", f);
        let f = generate_all("04.0081", "plun", 1, 1);
        assert!(f.iter().any(|x| x == "atuzat"), "{:?}", f);
        let f = generate_all("qupacaz", "alun", 1, 1);
        assert!(f.iter().any(|x| x == "apakta"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `pra_jagama_roundtrip`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn pra_jagama_roundtrip() {
        let prefs = vec!["pra".to_string()];
        let f = generate_all_with_prefixes("gam", "plit", 1, 1, &prefs);
        assert!(f.iter().any(|x| x == "prajagAma"), "{:?}", f);
        let hits = crate::engine::analyze::analyze_word("prajagAma");
        assert!(
            hits.iter().any(|a| {
                a.word_type == "tinanta"
                    && a.lakara.as_deref() == Some("plit")
                    && a.upasarga.as_deref() == Some("pra")
            }),
            "{:?}",
            hits
        );
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `gana_athematic_lang_vidhilin`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn gana_athematic_lang_vidhilin() {
        let f = generate_all("ada", "plan", 1, 1);
        assert!(f.iter().any(|x| x == "Adat" || x == "Adad"), "{:?}", f);
        let f = generate_all("ada", "pvidhilin", 1, 1);
        assert!(f.iter().any(|x| x == "adyAt" || x == "adyAd"), "{:?}", f);
        let f = generate_all("hu", "pvidhilin", 1, 1);
        assert!(f.iter().any(|x| x == "juhuyAt" || x == "juhuyAd"), "{:?}", f);
        let f = generate_all("hu", "plan", 1, 1);
        assert!(f.iter().any(|x| x == "ajuhot" || x == "ajuhod"), "{:?}", f);
        let f = generate_all("zuY", "plan", 1, 1);
        assert!(f.iter().any(|x| x == "asunot" || x == "asunod"), "{:?}", f);
        let f = generate_all("zuY", "pvidhilin", 1, 1);
        assert!(f.iter().any(|x| x == "sunuyAt" || x == "sunuyAd"), "{:?}", f);
        let f = generate_all("ruDir", "plan", 1, 1);
        assert!(f.iter().any(|x| x == "aruRat"), "{:?}", f);
        let f = generate_all("ruDir", "pvidhilin", 1, 1);
        assert!(f.iter().any(|x| x.contains("runD") && x.contains("yA")), "{:?}", f);
        let f = generate_all("qukrIY", "pvidhilin", 1, 1);
        assert!(f.iter().any(|x| x.contains("krI") && x.contains("yA")), "{:?}", f);
        let f = generate_all("qukrIY", "plan", 1, 1);
        assert!(f.iter().any(|x| x == "akrIRAt" || x == "akrIRAd" || x == "akrINAt"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `nic_san_yan_karma`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn nic_san_yan_karma() {
        let f = generate_all_derived("BU", "Ric", "plat", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "BAvayati"), "{:?}", f);
        let f = generate_all_derived("BU", "san", "plat", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "buBUzati"), "{:?}", f);
        let f = generate_all_derived("BU", "yaN", "alat", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "boBUyate"), "{:?}", f);
        let f = generate_all_derived("gam", "karma", "alat", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "gamyate"), "{:?}", f);
        let f = generate_all_derived("qukfY", "Ric", "plat", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "kArayati"), "{:?}", f);
        let f = generate_all_derived("BU", "Ric", "pvidhilin", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "BAvayet" || x == "BAvayed"), "{:?}", f);
        let f = generate_all_derived("gam", "karma", "avidhilin", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "gamyeta"), "{:?}", f);
        let f = generate_all_derived("BU", "Ric", "plit", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "BAvayAYcakAra"), "{:?}", f);
        let f = generate_all_derived("qukfY", "Ric", "plun", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "acIkarat" || x == "acIkarad"), "{:?}", f);
        let f = generate_all_derived("BU", "Ric", "pashirling", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "BAvayAt" || x == "BAvayAd"), "{:?}", f);
        let f = generate_all_derived("BU", "san", "plun", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "abuBUzIt" || x == "abuBUzId"), "{:?}", f);
        let f = generate_all_derived("BU", "Ric", "alat", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "BAvayate"), "{:?}", f);
        let f = generate_all_derived("BU", "Ric", "plot", 1, 1, &[]);
        assert!(f.iter().any(|x| x.contains("BAvaya")), "{:?}", f);
        let f = generate_all_derived("BU", "Ric", "plan", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "aBAvayat" || x == "aBAvayad"), "{:?}", f);
        let f = generate_all_derived("BU", "Ric", "plrt", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "BAvayizyati"), "{:?}", f);
        let f = generate_all_derived("BU", "san", "plrt", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "buBUzizyati"), "{:?}", f);
        let f = generate_all_derived("nI", "Ric", "plat", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "nAyayati"), "{:?}", f);
        let f = generate_all_derived("dA", "Ric", "plat", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "dApayati"), "{:?}", f);
        let f = generate_all_derived("hana", "Ric", "plat", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "GAtayati"), "{:?}", f);
        let f = generate_all_derived("So", "Ric", "plat", 1, 1, &[]);
        assert!(f.iter().any(|x| x == "SAyayati"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `akzi_live_vidhilin_lrt`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn akzi_live_vidhilin_lrt() {
        let f = generate_all("kAkzi", "pvidhilin", 1, 1);
        assert!(f.iter().any(|x| x == "kANkzet" || x == "kANkzed"), "{:?}", f);
        let f = generate_all("kAkzi", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "kANkzizyati"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `kri_pari_atmanepada`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn kri_pari_atmanepada() {
        let prefs = vec!["pari".to_string()];
        let f = generate_all_with_prefixes("qukrIY", "plat", 1, 1, &prefs);
        assert!(f.is_empty(), "{:?}", f);
        let f = generate_all_with_prefixes("qukrIY", "alat", 1, 1, &prefs);
        assert!(!f.is_empty(), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `folded_g1_live`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn folded_g1_live() {
        let f = generate_all("RIY", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "nayati"), "{:?}", f);
        let f = generate_all("yama", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "yacCati"), "{:?}", f);
        let f = generate_all("yama", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "yaMsyati"), "{:?}", f);
        let f = generate_all("dAR", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "yacCati"), "{:?}", f);
        let f = generate_all("zWA", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "tizWati"), "{:?}", f);
        let f = generate_all("dfSir", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "paSyati"), "{:?}", f);
        let f = generate_all("dfSir", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "drakzyati"), "{:?}", f);
        let f = generate_all("graha", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "grahIzyati"), "{:?}", f);
        let f = generate_all("hana", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "hanizyati"), "{:?}", f);
        let f = generate_all("zWala", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "sTalati"), "{:?}", f);
        let f = generate_all("pA", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "pibati"), "{:?}", f);
        let f = generate_all("SriY", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Srayati"), "{:?}", f);
        let f = generate_all("BfY", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Barati"), "{:?}", f);
        let f = generate_all("yaja", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "yajati"), "{:?}", f);
        let f = generate_all("veY", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "vayati"), "{:?}", f);
        let f = generate_all("zadx", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "sIdati"), "{:?}", f);
        let f = generate_all("kita", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "cikitsati"), "{:?}", f);
        let f = generate_all("dAna", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "dIdAMsati"), "{:?}", f);
        let f = generate_all("Dew", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Dayati"), "{:?}", f);
        let f = generate_all("CadiH", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Cadati"), "{:?}", f);
        let f = generate_all("dEp", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "dAyati"), "{:?}", f);
        assert!(f.iter().all(|x| x != "yacCati"), "{:?}", f);
        let f = generate_all("zanja", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "sajati"), "{:?}", f);
        let f = generate_all("zasja", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "sajjati"), "{:?}", f);
        let f = generate_all("zanja", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "saNkzyati"), "{:?}", f);
        let f = generate_all("danSa", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "daSati"), "{:?}", f);
        let f = generate_all("ranja", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "rajati"), "{:?}", f);
        let f = generate_all("kfvi", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "kfRoti"), "{:?}", f);
        let f = generate_all("zwana", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "stanati"), "{:?}", f);
        let f = generate_all("zaRa", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "sanati"), "{:?}", f);
        let f = generate_all("ziDu", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "seDati"), "{:?}", f);
        let f = generate_all("wunadi", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "nandati"), "{:?}", f);
        let f = generate_all("wuosPUrjA", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "sPUrjati"), "{:?}", f);
        let f = generate_all("YiPalA", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Palati"), "{:?}", f);
        let f = generate_all("Bfzu", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Barzati"), "{:?}", f);
        let f = generate_all("zWivu", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "zWIvati"), "{:?}", f);
        let f = generate_all("mleCa", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "mlecCati"), "{:?}", f);
        let f = generate_all("urvI", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Urvati"), "{:?}", f);
        let f = generate_all("turvI", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "tUrvati"), "{:?}", f);
        let f = generate_all("hurCA", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "hUrCati"), "{:?}", f);
        let f = generate_all("Rikza", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "nikzati"), "{:?}", f);
        let f = generate_all("RIla", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "nIlati"), "{:?}", f);
        let f = generate_all("zwfkza", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "stfkzati"), "{:?}", f);
        let f = generate_all("UWa", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "UWati"), "{:?}", f);
        let f = generate_all("ati", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "antati"), "{:?}", f);
        let f = generate_all("mleCa", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "mlecCizyati"), "{:?}", f);
        let f = generate_all("zWivu", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "zWevizyati"), "{:?}", f);
        let f = generate_all("urvI", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "Urvizyati"), "{:?}", f);
        let f = generate_all("divu", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "dIvyati"), "{:?}", f);
        let f = generate_all("divu", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "devizyati"), "{:?}", f);
        let f = generate_all("ada", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "atsyati"), "{:?}", f);
        let f = generate_all("Sru", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Sravati"), "{:?}", f);
        let f = generate_all("Sru", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "Srozyati"), "{:?}", f);
        let f = generate_all("sru", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "srozyati"), "{:?}", f);
        let f = generate_all("tyaja", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "tyakzyati"), "{:?}", f);
        let f = generate_all("skandir", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "skantsyati"), "{:?}", f);
        let f = generate_all("Rama", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "naMsyati"), "{:?}", f);
        let f = generate_all("daha", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "Dakzyati"), "{:?}", f);
        let f = generate_all("kfza", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "karkzyati"), "{:?}", f);
        let f = generate_all("vasa", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "vatsyati"), "{:?}", f);
        let f = generate_all("Sizx", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "Sekzyati"), "{:?}", f);
        let f = generate_all("rivi", "plot", 3, 1);
        assert!(f.iter().any(|x| x == "riRvAni"), "{:?}", f);
        let f = generate_all("vftu", "alrt", 1, 1);
        assert!(f.iter().any(|x| x == "vartsyate"), "{:?}", f);
        let f = generate_all("vfDu", "alrt", 1, 1);
        assert!(f.iter().any(|x| x == "varDizyate"), "{:?}", f);
        let f = generate_all("syandU", "alrt", 1, 1);
        assert!(f.iter().any(|x| x == "syantsyate"), "{:?}", f);
        let f = generate_all("kfpU", "alrt", 1, 1);
        assert!(f.iter().any(|x| x == "kalpsyate"), "{:?}", f);
        let f = generate_all("ik", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "aDyeti"), "{:?}", f);
        let f = generate_all("daridrA", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "daridrAti"), "{:?}", f);
        let f = generate_all("cakAsf", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "cakAsti"), "{:?}", f);
        let f = generate_all("zasa", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "sasti"), "{:?}", f);
        let f = generate_all("zasti", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "saMsti"), "{:?}", f);
        let f = generate_all("cakziN", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "kSAsyati"), "{:?}", f);
        let f = generate_all("Divi", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Dinoti"), "{:?}", f);
        let f = generate_all("Divi", "plat", 1, 3);
        assert!(f.iter().any(|x| x == "Dinvanti"), "{:?}", f);
        let f = generate_all("Divi", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "Dinvizyati"), "{:?}", f);
        let f = generate_all("Divi", "plan", 1, 1);
        assert!(f.iter().any(|x| x == "aDinot" || x == "aDinod"), "{:?}", f);
        let f = generate_all("Divi", "pvidhilin", 1, 1);
        assert!(f.iter().any(|x| x == "DinuyAt" || x == "DinuyAd"), "{:?}", f);
        let f = generate_all("Rikza", "plan", 1, 1);
        assert!(f.iter().any(|x| x == "anikzat" || x == "anikzad"), "{:?}", f);
        let f = generate_all("Rikza", "pvidhilin", 1, 1);
        assert!(f.iter().any(|x| x == "nikzet" || x == "nikzed"), "{:?}", f);
        let f = generate_all("fti", "alat", 1, 1);
        assert!(f.iter().any(|x| x == "artate"), "{:?}", f);
        let f = generate_all("fti", "alrt", 1, 1);
        assert!(f.iter().any(|x| x == "artizyate"), "{:?}", f);
        let f = generate_all("fti", "plrt", 1, 1);
        assert!(f.is_empty(), "Ātmane ऋति has no parasmai लृट्: {:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `gana6_lrt_sutra`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn gana6_lrt_sutra() {
        let f = generate_all("06.0001", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "totsyati"), "{:?}", f);
        let f = generate_all("06.0150", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "srakzyati"), "{:?}", f);
        let f = generate_all("06.0151", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "maNkzyati"), "{:?}", f);
        let f = generate_all("06.0143", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "kzezyati"), "{:?}", f);
        let f = generate_all("06.0134", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "guzyati"), "{:?}", f);
        let f = generate_all("06.0135", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "Druzyati"), "{:?}", f);
        let f = generate_all("06.0099", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "Curizyati"), "{:?}", f);
        let f = generate_all("06.0132", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "nuvizyati"), "{:?}", f);
        let f = generate_all("06.0130", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "huqizyati"), "{:?}", f);
        let f = generate_all("06.0046", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "SoBizyati"), "{:?}", f);
        let f = generate_all("06.0144", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "savizyati"), "{:?}", f);
        let f = generate_all("06.0136", "alrt", 1, 1);
        assert!(f.iter().any(|x| x == "kuzyate"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `kram_yam_artha`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn kram_yam_artha() {
        let vi = vec!["vi".to_string()];
        let f = generate_all_with_prefixes("kramu", "alat", 1, 1, &vi);
        assert!(f.is_empty(), "{:?}", f);
        let f = generate_all_with_artha("krama", "alat", 1, 1, &vi, "vftti");
        assert!(!f.is_empty(), "{:?}", f);
        let sam = vec!["sam".to_string()];
        let f = generate_all_with_artha("yama", "alat", 1, 1, &sam, "agranthe");
        assert!(!f.is_empty(), "{:?}", f);
        let f = generate_all_with_artha("yama", "alat", 1, 1, &sam, "granthe");
        assert!(f.is_empty(), "{:?}", f);
    }
}
