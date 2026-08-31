//! तिङन्त: Pāṇini as arranged in the Siddhānta-Kaumudī (stem → ending → sandhi).
//! Scrape DBs are for cross-check only — they never supply a form.
use serde::{Deserialize, Serialize};
use crate::engine::lakara::{lakara_family, normalize_lakara};
use crate::engine::stems::{derive_stem, conjugation_gana};
use crate::engine::endings::family_endings;
use crate::engine::join::join_variants;
use crate::engine::upa_pada::pada_allowed_artha;

#[derive(Serialize, Deserialize, Debug)]
pub struct TinantaResult {
    pub forms: Vec<String>,
    pub dhatu: String,
    pub lakara: String,
    pub purusha: u8,
    pub vacana: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParadigmEntry {
    pub purusha: u8,
    pub vacana: u8,
    pub forms: Vec<String>,
}

fn load_dhatu_info(dhatu_query: &str) -> Option<(String, u8, String, String, String, String)> {
    Some(crate::engine::dhatu::load_or_fallback(dhatu_query))
}

pub fn generate(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8) -> TinantaResult {
    let forms = generate_all(dhatu_query, lakara, purusha, vacana);
    let (canon, _) = normalize_lakara(lakara);
    TinantaResult { forms, dhatu: dhatu_query.to_string(), lakara: canon, purusha, vacana }
}

pub fn generate_with_prefixes(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &[String]) -> TinantaResult {
    generate_with_artha(dhatu_query, lakara, purusha, vacana, prefixes, "")
}

pub fn generate_with_artha(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &[String], artha: &str) -> TinantaResult {
    let forms = generate_all_with_artha(dhatu_query, lakara, purusha, vacana, prefixes, artha);
    let (canon, _) = normalize_lakara(lakara);
    TinantaResult { forms, dhatu: dhatu_query.to_string(), lakara: canon, purusha, vacana }
}

pub fn generate_all(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8) -> Vec<String> {
    generate_all_with_prefixes(dhatu_query, lakara, purusha, vacana, &[])
}


fn attach_prefixes(prefixes: &[String], forms: Vec<String>) -> Vec<String> {
    if prefixes.is_empty() { forms }
    else { forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect() }
}

pub fn generate_all_with_prefixes(dhatu_query: &str, lakara: &str, purusha: u8, vacana: u8, prefixes: &[String]) -> Vec<String> {
    generate_all_with_artha(dhatu_query, lakara, purusha, vacana, prefixes, "")
}

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
    if !deriv.is_empty() && deriv != "shuddha" {
        return live_generate_derived(dhatu_query, deriv, &canonical, &db_lakara, purusha, vacana, prefixes, artha);
    }
    if let Some(out) = crate::engine::tinanta_overrides::lookup_override(dhatu_query, &canonical, purusha, vacana, prefixes) {
        return out;
    }
    live_generate(dhatu_query, &canonical, &db_lakara, purusha, vacana, prefixes, artha)
}

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

pub fn generate_paradigm_derived(dhatu: &str, derivation: &str, lakara: &str, prefixes: &[String]) -> Vec<ParadigmEntry> {
    generate_paradigm_derived_artha(dhatu, derivation, lakara, prefixes, "")
}

pub fn generate_paradigm_derived_artha(dhatu: &str, derivation: &str, lakara: &str, prefixes: &[String], artha: &str) -> Vec<ParadigmEntry> {
    let mut out = Vec::new();
    for p in 1..=3 {
        for v in 1..=3 {
            let forms = generate_all_derived_artha(dhatu, derivation, lakara, p, v, prefixes, artha);
            out.push(ParadigmEntry { purusha: p, vacana: v, forms });
        }
    }
    out
}

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
    if !pada_allowed_artha(&root_pada, &pada, &dhatu, prefixes, artha) {
        return vec![];
    }
    if family == "lit" {
        if let Some(forms) = crate::engine::lit::kartari(&dhatu, purusha, vacana, pada) {
            return attach_prefixes(prefixes, forms);
        }
    }
    if family == "lun" {
        if let Some(forms) = crate::engine::lun::kartari(&dhatu, purusha, vacana, pada) {
            return attach_prefixes(prefixes, forms);
        }
    }
    if family == "ashir" {
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
    if idx >= table.len() { return vec![]; }
    let (variants, _) = &table[idx];
    let forms = join_variants(&stem, variants, cgana, &family, purusha, pada, augment.as_deref(), &dhatu, vacana, &antarganas);
    attach_prefixes(prefixes, forms)
}

pub fn generate_paradigm(dhatu: &str, lakara: &str) -> Vec<ParadigmEntry> {
    let mut out = Vec::new();
    for p in 1..=3 { for v in 1..=3 {
        let forms = generate_all(dhatu, lakara, p, v);
        out.push(ParadigmEntry { purusha: p, vacana: v, forms });
    }}
    out
}

pub fn generate_paradigm_with_prefixes(dhatu: &str, lakara: &str, prefixes: &[String]) -> Vec<ParadigmEntry> {
    generate_paradigm_with_artha(dhatu, lakara, prefixes, "")
}

pub fn generate_paradigm_with_artha(dhatu: &str, lakara: &str, prefixes: &[String], artha: &str) -> Vec<ParadigmEntry> {
    let mut out = Vec::new();
    for p in 1..=3 { for v in 1..=3 {
        let forms = generate_all_with_artha(dhatu, lakara, p, v, prefixes, artha);
        out.push(ParadigmEntry { purusha: p, vacana: v, forms });
    }}
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bu_lat_prathama_ekavacana() {
        let f = generate_all("BU", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Bavati"), "{:?}", f);
    }

    #[test]
    fn gam_lat_prathama_ekavacana() {
        let f = generate_all("gam", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "gacCati"), "{:?}", f);
    }

    #[test]
    fn pra_bu_lat() {
        let prefs = vec!["pra".to_string()];
        let f = generate_all_with_prefixes("BU", "plat", 1, 1, &prefs);
        assert!(f.iter().any(|x| x == "praBavati"), "{:?}", f);
    }

    #[test]
    fn live_kakzi_vidhilin() {
        let (canon, db) = normalize_lakara("pvidhilin");
        let f = live_generate("kAkzi", &canon, &db, 1, 1, &[], "");
        assert!(f.iter().any(|x| x == "kANkzet" || x == "kANkzed"), "{:?}", f);
    }

    #[test]
    fn live_gamx_no_override() {
        assert!(crate::engine::tinanta_overrides::lookup_override("gamx", "plat", 1, 1, &[]).is_none());
        let (canon, db) = normalize_lakara("plat");
        let f = live_generate("gamx", &canon, &db, 1, 1, &[], "");
        assert!(f.iter().any(|x| x == "gacCati"), "{:?}", f);
        let (canon, db) = normalize_lakara("plrt");
        let f = live_generate("gamx", &canon, &db, 1, 1, &[], "");
        assert!(f.iter().any(|x| x == "gamizyati"), "{:?}", f);
    }

    #[test]
    fn pra_bu_lat_third_plural_no_natva() {
        let prefs = vec!["pra".to_string()];
        let f = generate_all_with_prefixes("BU", "plat", 1, 3, &prefs);
        assert!(f.iter().any(|x| x == "praBavanti"), "{:?}", f);
        assert!(!f.iter().any(|x| x == "praBavaRti"), "{:?}", f);
    }

    #[test]
    fn gam_lit_jagama() {
        let f = generate_all("gam", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "jagAma"), "{:?}", f);
        let f = generate_all("gam", "plit", 1, 3);
        assert!(f.iter().any(|x| x == "jagmuH"), "{:?}", f);
    }

    #[test]
    fn bu_lit_babhuva() {
        let f = generate_all("BU", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "baBUva"), "{:?}", f);
    }

    #[test]
    fn han_lit_jaghana() {
        let f = generate_all("hana", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "jaGAna"), "{:?}", f);
    }

    #[test]
    fn vac_yaj_lit() {
        let f = generate_all("vaca", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "uvAca"), "{:?}", f);
        let f = generate_all("yaja", "plit", 1, 2);
        assert!(f.iter().any(|x| x == "IjatuH"), "{:?}", f);
    }

    #[test]
    fn grah_ni_kf_lit() {
        let f = generate_all("graha", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "jagrAha"), "{:?}", f);
        let f = generate_all("RIY", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "ninAya"), "{:?}", f);
        let f = generate_all("qukfY", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "cakAra"), "{:?}", f);
    }

    #[test]
    fn yaj_kf_alit() {
        let f = generate_all("yaja", "alit", 1, 1);
        assert!(f.iter().any(|x| x == "Ije"), "{:?}", f);
        let f = generate_all("qukfY", "alit", 1, 3);
        assert!(f.iter().any(|x| x == "cakrire"), "{:?}", f);
        let f = generate_all("RIY", "alit", 1, 1);
        assert!(f.iter().any(|x| x == "ninye"), "{:?}", f);
    }

    #[test]
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
    fn i_as_lit() {
        let f = generate_all("iR", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "iyAya"), "{:?}", f);
        let f = generate_all("asa", "plit", 1, 1);
        assert!(f.iter().any(|x| x == "Asa"), "{:?}", f);
        let f = generate_all("iN", "alit", 1, 1);
        assert!(f.iter().any(|x| x == "Iye"), "{:?}", f);
    }

    #[test]
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
    fn gana_2_3_5_7_8_9_lat() {
        let f = generate_all("ada", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "atti"), "{:?}", f);
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
        let f = generate_all("iR", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "eti"), "{:?}", f);
    }

    #[test]
    fn lun_han_ashir_a_lrt() {
        let f = generate_all("hana", "plun", 1, 1);
        assert!(f.iter().any(|x| x == "avaDIt"), "{:?}", f);
        let f = generate_all("qukfY", "aashirling", 1, 1);
        assert!(f.iter().any(|x| x == "kfzIzwa"), "{:?}", f);
        let f = generate_all("zWA", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "sTAsyati"), "{:?}", f);
        let f = generate_all("qupacaz", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "pakzyati"), "{:?}", f);
    }

    #[test]
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
    }

    #[test]
    fn akzi_live_vidhilin_lrt() {
        assert!(crate::engine::tinanta_overrides::lookup_override("kAkzi", "pvidhilin", 1, 1, &[]).is_none());
        let f = generate_all("kAkzi", "pvidhilin", 1, 1);
        assert!(f.iter().any(|x| x == "kANkzet" || x == "kANkzed"), "{:?}", f);
        let f = generate_all("kAkzi", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "kANkzizyati"), "{:?}", f);
    }

    #[test]
    fn kri_pari_atmanepada() {
        let prefs = vec!["pari".to_string()];
        let f = generate_all_with_prefixes("qukrIY", "plat", 1, 1, &prefs);
        assert!(f.is_empty(), "{:?}", f);
        let f = generate_all_with_prefixes("qukrIY", "alat", 1, 1, &prefs);
        assert!(!f.is_empty(), "{:?}", f);
    }

    #[test]
    fn folded_g1_live() {
        let f = generate_all("RIY", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "nayati"), "{:?}", f);
        let f = generate_all("yama", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "yacCati"), "{:?}", f);
        let f = generate_all("yama", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "yaMsyati"), "{:?}", f);
        let f = generate_all("dAR", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "yacCati"), "{:?}", f);
        let f = generate_all("Sru", "plat", 1, 1);
        assert!(f.iter().any(|x| x == "Sravati"), "{:?}", f);
        let f = generate_all("Sru", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "Srozyati"), "{:?}", f);
        let f = generate_all("sru", "plrt", 1, 1);
        assert!(f.iter().any(|x| x == "srozyati"), "{:?}", f);
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
        assert!(crate::engine::tinanta_overrides::lookup_override("vftu", "plrt", 1, 1, &[]).is_none());
        assert!(crate::engine::tinanta_overrides::lookup_override("vfDu", "plrt", 1, 1, &[]).is_none());
    }

    #[test]
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
