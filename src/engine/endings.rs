//! Auto-generated from sktmorph/engine/endings.py

//! =============================================================================
//! src/engine/endings.rs: Pāṇini/Kaumudī implementation — extreme commenting pass (2026-09-01)
//! ---------------------------------------------------------------------------
//! Purpose: see inline block comments below. Every public/private block is
//! documented with sūtra reference, input/output, and edge-case notes.
//! Script: SLP1 internally; Devanagari only at demo boundary.
//! Flow: dhātu → it-strip → aṅga/vikaraṇa → lakāra/ending → sandhi → surface.
//! Gold DB is cross-check only, never source of truth.
//! =============================================================================
use crate::engine::redup::GANA3;

// ---------------------------------------------------------------------------
// fn `lat_kartari_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lat_kartari_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["ti".into()], vec!["3.4.78".into()]),
        (vec!["taH".into()], vec!["3.4.78".into()]),
        (vec!["nti".into()], vec!["3.4.78".into()]),
        (vec!["si".into()], vec!["3.4.78".into()]),
        (vec!["TaH".into()], vec!["3.4.78".into()]),
        (vec!["Ta".into()], vec!["3.4.78".into()]),
        (vec!["Ami".into()], vec!["3.4.78".into()]),
        (vec!["AvaH".into()], vec!["3.4.78".into()]),
        (vec!["AmaH".into()], vec!["3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lat_kartari_a`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lat_kartari_a() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["te".into()], vec!["3.4.78".into()]),
        (vec!["ete".into()], vec!["3.4.78".into()]),
        (vec!["ante".into()], vec!["3.4.78".into()]),
        (vec!["se".into()], vec!["3.4.78".into()]),
        (vec!["eTe".into()], vec!["3.4.78".into()]),
        (vec!["aDve".into()], vec!["3.4.78".into()]),
        (vec!["e".into()], vec!["3.4.78".into()]),
        (vec!["Avahe".into()], vec!["3.4.78".into()]),
        (vec!["Amahe".into()], vec!["3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lat_ad_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lat_ad_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["ti".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["taH".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["anti".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["si".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["TaH".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["Ta".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["mi".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["vaH".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["maH".into()], vec!["3.1.3".into(), "3.4.78".into()]),
    ]
}

/// अदादि आत्मने लट् (शप् लुक्): ते/आते/अते not thematic एते/अन्ते.
/// दुग्धे, दुहाते, दुहते; धुक्षे; दुहे.
pub fn lat_ad_a() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["te".into()], vec!["3.4.78".into()]),
        (vec!["Ate".into()], vec!["3.4.78".into()]),
        (vec!["ate".into()], vec!["3.4.78".into()]),
        (vec!["se".into()], vec!["3.4.78".into()]),
        (vec!["ATe".into()], vec!["3.4.78".into()]),
        (vec!["Dve".into()], vec!["3.4.78".into()]),
        (vec!["e".into()], vec!["3.4.78".into()]),
        (vec!["vahe".into()], vec!["3.4.78".into()]),
        (vec!["mahe".into()], vec!["3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lot_kartari_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lot_kartari_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["tAt".into(), "tAd".into(), "tu".into()], vec!["3.4.78".into()]),
        (vec!["tAm".into()], vec!["3.4.78".into()]),
        (vec!["antu".into()], vec!["3.4.78".into()]),
        (vec!["".into(), "tAt".into(), "tAd".into()], vec!["3.4.78".into()]),
        (vec!["tam".into()], vec!["3.4.78".into()]),
        (vec!["ta".into()], vec!["3.4.78".into()]),
        (vec!["Ani".into()], vec!["3.4.78".into()]),
        (vec!["Ava".into()], vec!["3.4.78".into()]),
        (vec!["Ama".into()], vec!["3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lot_kartari_a`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lot_kartari_a() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["atAm".into()], vec!["3.4.78".into()]),
        (vec!["etAm".into()], vec!["3.4.78".into()]),
        (vec!["antAm".into()], vec!["3.4.78".into()]),
        (vec!["sva".into()], vec!["3.4.78".into()]),
        (vec!["eTAm".into()], vec!["3.4.78".into()]),
        (vec!["aDvam".into()], vec!["3.4.78".into()]),
        (vec!["E".into()], vec!["3.4.78".into()]),
        (vec!["AvahE".into()], vec!["3.4.78".into()]),
        (vec!["AmahE".into()], vec!["3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lot_kartari_p_caus`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lot_kartari_p_caus() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["tAt".into(), "tAd".into(), "tu".into()], vec!["3.4.78".into()]),
        (vec!["tAm".into()], vec!["3.4.78".into()]),
        (vec!["antu".into()], vec!["3.4.78".into()]),
        (vec!["".into(), "tAt".into(), "tAd".into()], vec!["3.4.78".into()]),
        (vec!["tam".into()], vec!["3.4.78".into()]),
        (vec!["ta".into()], vec!["3.4.78".into()]),
        (vec!["Ani".into()], vec!["3.4.78".into()]),
        (vec!["Ava".into()], vec!["3.4.78".into()]),
        (vec!["Ama".into()], vec!["3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lot_ni_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lot_ni_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["Atu".into(), "ItAt".into(), "ItAd".into()], vec!["3.4.78".into()]),
        (vec!["ItAm".into()], vec!["3.4.78".into()]),
        (vec!["antu".into()], vec!["3.4.78".into()]),
        (vec!["ItAt".into(), "ItAd".into(), "Ihi".into()], vec!["3.4.78".into()]),
        (vec!["Itam".into()], vec!["3.4.78".into()]),
        (vec!["ta".into()], vec!["3.4.78".into()]),
        (vec!["Ani".into()], vec!["3.4.78".into()]),
        (vec!["Ava".into()], vec!["3.4.78".into()]),
        (vec!["Ama".into()], vec!["3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lot_nu_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lot_nu_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["utAt".into(), "utAd".into(), "otu".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["utAm".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["vantu".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["u".into(), "utAt".into(), "utAd".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["utam".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["uta".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["avAni".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["avAva".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["avAma".into()], vec!["3.1.75".into(), "3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lot_ad_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lot_ad_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["tAt".into(), "tAd".into(), "tu".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["tAm".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["antu".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["tAt".into(), "tAd".into(), "Di".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["tam".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["ta".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["Ani".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["Ava".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["Ama".into()], vec!["3.1.3".into(), "3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lrt_ad_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lrt_ad_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["ti".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["taH".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["anti".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["si".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["TaH".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["Ta".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["Ami".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["AvaH".into()], vec!["3.1.3".into(), "3.4.78".into()]),
        (vec!["AmaH".into()], vec!["3.1.3".into(), "3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lang_kartari_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lang_kartari_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["at".into(), "ad".into()], vec!["3.4.111".into()]),
        (vec!["atAm".into()], vec!["3.4.111".into()]),
        (vec!["an".into()], vec!["3.4.111".into()]),
        (vec!["aH".into()], vec!["3.4.111".into()]),
        (vec!["atam".into()], vec!["3.4.111".into()]),
        (vec!["ata".into()], vec!["3.4.111".into()]),
        (vec!["am".into()], vec!["3.4.111".into()]),
        (vec!["Ava".into()], vec!["3.4.111".into()]),
        (vec!["Ama".into()], vec!["3.4.111".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lang_kartari_a`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lang_kartari_a() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["ata".into()], vec!["3.4.111".into()]),
        (vec!["etAm".into()], vec!["3.4.111".into()]),
        (vec!["anta".into()], vec!["3.4.111".into()]),
        (vec!["aTAH".into()], vec!["3.4.111".into()]),
        (vec!["eTAm".into()], vec!["3.4.111".into()]),
        (vec!["aDvam".into()], vec!["3.4.111".into()]),
        (vec!["e".into()], vec!["3.4.111".into()]),
        (vec!["Avahi".into()], vec!["3.4.111".into()]),
        (vec!["Amahi".into()], vec!["3.4.111".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lang_ad_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lang_ad_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["at".into(), "ad".into()], vec!["3.1.3".into(), "3.4.111".into()]),
        (vec!["atAm".into()], vec!["3.1.3".into(), "3.4.111".into()]),
        (vec!["an".into()], vec!["3.1.3".into(), "3.4.111".into()]),
        (vec!["aH".into()], vec!["3.1.3".into(), "3.4.111".into()]),
        (vec!["atam".into()], vec!["3.1.3".into(), "3.4.111".into()]),
        (vec!["ata".into()], vec!["3.1.3".into(), "3.4.111".into()]),
        (vec!["am".into()], vec!["3.1.3".into(), "3.4.111".into()]),
        (vec!["va".into()], vec!["3.1.3".into(), "3.4.111".into()]),
        (vec!["ma".into()], vec!["3.1.3".into(), "3.4.111".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lang_nu_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lang_nu_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["ot".into(), "od".into()], vec!["3.1.75".into(), "3.4.111".into()]),
        (vec!["utAm".into()], vec!["3.1.75".into(), "3.4.111".into()]),
        (vec!["van".into()], vec!["3.1.75".into(), "3.4.111".into()]),
        (vec!["oH".into()], vec!["3.1.75".into(), "3.4.111".into()]),
        (vec!["utam".into()], vec!["3.1.75".into(), "3.4.111".into()]),
        (vec!["uta".into()], vec!["3.1.75".into(), "3.4.111".into()]),
        (vec!["avam".into()], vec!["3.1.75".into(), "3.4.111".into()]),
        (vec!["uva".into(), "va".into()], vec!["3.1.75".into(), "3.4.111".into()]),
        (vec!["uma".into(), "ma".into()], vec!["3.1.75".into(), "3.4.111".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lang_ni_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lang_ni_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["At".into(), "Ad".into()], vec!["3.1.81".into(), "3.4.111".into()]),
        (vec!["ItAm".into()], vec!["3.1.81".into(), "3.4.111".into()]),
        (vec!["an".into()], vec!["3.1.81".into(), "3.4.111".into()]),
        (vec!["AH".into()], vec!["3.1.81".into(), "3.4.111".into()]),
        (vec!["Itam".into()], vec!["3.1.81".into(), "3.4.111".into()]),
        (vec!["Ita".into()], vec!["3.1.81".into(), "3.4.111".into()]),
        (vec!["Am".into()], vec!["3.1.81".into(), "3.4.111".into()]),
        (vec!["Iva".into()], vec!["3.1.81".into(), "3.4.111".into()]),
        (vec!["Ima".into()], vec!["3.1.81".into(), "3.4.111".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `vidhilin_kartari_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn vidhilin_kartari_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["et".into(), "ed".into()], vec!["3.4.104".into()]),
        (vec!["etAm".into()], vec!["3.4.104".into()]),
        (vec!["eyuH".into()], vec!["3.4.104".into()]),
        (vec!["eH".into()], vec!["3.4.104".into()]),
        (vec!["etam".into()], vec!["3.4.104".into()]),
        (vec!["eta".into()], vec!["3.4.104".into()]),
        (vec!["eyam".into()], vec!["3.4.104".into()]),
        (vec!["eva".into()], vec!["3.4.104".into()]),
        (vec!["ema".into()], vec!["3.4.104".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `vidhilin_kartari_a`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn vidhilin_kartari_a() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["eta".into()], vec!["3.4.104".into()]),
        (vec!["eyAtAm".into()], vec!["3.4.104".into()]),
        (vec!["eran".into()], vec!["3.4.104".into()]),
        (vec!["eTAH".into()], vec!["3.4.104".into()]),
        (vec!["eyATAm".into()], vec!["3.4.104".into()]),
        (vec!["eDvam".into()], vec!["3.4.104".into()]),
        (vec!["eya".into()], vec!["3.4.104".into()]),
        (vec!["evahi".into()], vec!["3.4.104".into()]),
        (vec!["emahi".into()], vec!["3.4.104".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `vidhilin_ad_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn vidhilin_ad_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["yAt".into(), "yAd".into()], vec!["3.1.3".into(), "3.4.104".into()]),
        (vec!["yAtAm".into()], vec!["3.1.3".into(), "3.4.104".into()]),
        (vec!["yuH".into()], vec!["3.1.3".into(), "3.4.104".into()]),
        (vec!["yAH".into()], vec!["3.1.3".into(), "3.4.104".into()]),
        (vec!["yAtam".into()], vec!["3.1.3".into(), "3.4.104".into()]),
        (vec!["yAta".into()], vec!["3.1.3".into(), "3.4.104".into()]),
        (vec!["yAm".into()], vec!["3.1.3".into(), "3.4.104".into()]),
        (vec!["yAva".into()], vec!["3.1.3".into(), "3.4.104".into()]),
        (vec!["yAma".into()], vec!["3.1.3".into(), "3.4.104".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `vidhilin_nu_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn vidhilin_nu_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["At".into(), "Ad".into()], vec!["3.1.75".into(), "3.4.104".into()]),
        (vec!["AtAm".into()], vec!["3.1.75".into(), "3.4.104".into()]),
        (vec!["uH".into()], vec!["3.1.75".into(), "3.4.104".into()]),
        (vec!["AH".into()], vec!["3.1.75".into(), "3.4.104".into()]),
        (vec!["Atam".into()], vec!["3.1.75".into(), "3.4.104".into()]),
        (vec!["Ata".into()], vec!["3.1.75".into(), "3.4.104".into()]),
        (vec!["Am".into()], vec!["3.1.75".into(), "3.4.104".into()]),
        (vec!["Ava".into()], vec!["3.1.75".into(), "3.4.104".into()]),
        (vec!["Ama".into()], vec!["3.1.75".into(), "3.4.104".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `vidhilin_ni_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn vidhilin_ni_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["yAt".into(), "yAd".into()], vec!["3.1.81".into(), "3.4.104".into()]),
        (vec!["yAtAm".into()], vec!["3.1.81".into(), "3.4.104".into()]),
        (vec!["yuH".into()], vec!["3.1.81".into(), "3.4.104".into()]),
        (vec!["yAH".into()], vec!["3.1.81".into(), "3.4.104".into()]),
        (vec!["yAtam".into()], vec!["3.1.81".into(), "3.4.104".into()]),
        (vec!["yAta".into()], vec!["3.1.81".into(), "3.4.104".into()]),
        (vec!["yAm".into()], vec!["3.1.81".into(), "3.4.104".into()]),
        (vec!["yAva".into()], vec!["3.1.81".into(), "3.4.104".into()]),
        (vec!["yAma".into()], vec!["3.1.81".into(), "3.4.104".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lit_kartari_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lit_kartari_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["va".into()], vec!["3.2.115".into()]),
        (vec!["vatuH".into()], vec!["3.2.115".into()]),
        (vec!["vuH".into()], vec!["3.2.115".into()]),
        (vec!["viTa".into()], vec!["3.2.115".into()]),
        (vec!["vaTuH".into()], vec!["3.2.115".into()]),
        (vec!["va".into()], vec!["3.2.115".into()]),
        (vec!["va".into()], vec!["3.2.115".into()]),
        (vec!["viva".into()], vec!["3.2.115".into()]),
        (vec!["vima".into()], vec!["3.2.115".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lit_kartari_a`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lit_kartari_a() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["e".into()], vec!["3.4.78".into()]),
        (vec!["Ate".into()], vec!["3.4.78".into()]),
        (vec!["ire".into()], vec!["3.4.78".into()]),
        (vec!["iTe".into()], vec!["3.4.78".into()]),
        (vec!["ATe".into()], vec!["3.4.78".into()]),
        (vec!["iDve".into()], vec!["3.4.78".into()]),
        (vec!["e".into()], vec!["3.4.78".into()]),
        (vec!["i vahe".into()], vec!["3.4.78".into()]),
        (vec!["i mahe".into()], vec!["3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `nu_lat_kartari_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn nu_lat_kartari_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["ti".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["taH".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["nti".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["zi".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["TaH".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["Ta".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["mi".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["vaH".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["maH".into()], vec!["3.1.75".into(), "3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `nu_lat_kartari_a`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn nu_lat_kartari_a() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["te".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["vAte".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["vate".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["ze".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["vATe".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["uDve".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["ve".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["uvahe".into()], vec!["3.1.75".into(), "3.4.78".into()]),
        (vec!["vahe".into()], vec!["3.1.75".into(), "3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lrt_kartari_p`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lrt_kartari_p() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["ti".into()], vec!["3.4.78".into()]),
        (vec!["taH".into()], vec!["3.4.78".into()]),
        (vec!["nti".into()], vec!["3.4.78".into()]),
        (vec!["si".into()], vec!["3.4.78".into()]),
        (vec!["TaH".into()], vec!["3.4.78".into()]),
        (vec!["Ta".into()], vec!["3.4.78".into()]),
        (vec!["Ami".into()], vec!["3.4.78".into()]),
        (vec!["AvaH".into()], vec!["3.4.78".into()]),
        (vec!["AmaH".into()], vec!["3.4.78".into()]),
    ]
}

// ---------------------------------------------------------------------------
// fn `lrt_kartari_a`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn lrt_kartari_a() -> Vec<(Vec<String>, Vec<String>)> {
    vec![
        (vec!["te".into()], vec!["3.4.78".into()]),
        (vec!["ete".into()], vec!["3.4.78".into()]),
        (vec!["ante".into()], vec!["3.4.78".into()]),
        (vec!["se".into()], vec!["3.4.78".into()]),
        (vec!["eTe".into()], vec!["3.4.78".into()]),
        (vec!["aDve".into()], vec!["3.4.78".into()]),
        (vec!["e".into()], vec!["3.4.78".into()]),
        (vec!["Avahe".into()], vec!["3.4.78".into()]),
        (vec!["Amahe".into()], vec!["3.4.78".into()]),
    ]
}


// ---------------------------------------------------------------------------
// fn `gana_class`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn gana_class(gana: u8) -> &'static str {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if matches!(gana, 2|3) { return "ad"; }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if matches!(gana, 5|8) { return "nu"; }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if matches!(gana, 7|9) { return "thematic"; }
    "thematic"
}

// ---------------------------------------------------------------------------
// fn `family_endings` — tin/sUP endings: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn family_endings(
    family: &str,
    prayoga: &str,
    pada: &str,
    gana: u8,
    dhatu: Option<&str>,
) -> Option<Vec<(Vec<String>, Vec<String>)>> {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if prayoga != "kartari" { return None; }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana == GANA3 {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(d) = dhatu {
            let mode = crate::engine::redup::gana3_join_mode(d, None);
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if mode == "nu" {
                // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
                match family {
                    "lot" => return Some(lot_nu_p()),
                    "lang" => return Some(lang_nu_p()),
                    "vidhilin" => return Some(vidhilin_nu_p()),
                    "lat" if pada=="P" => return Some(nu_lat_kartari_p()),
                    "lat" if pada=="A" => return Some(nu_lat_kartari_a()),
                    _ => {}
                }
            }
        }
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family=="vidhilin" && pada=="P" && gana==9 { return Some(vidhilin_ni_p()); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family=="lrt" && pada=="P" && matches!(gana, 5|8) { return Some(lrt_kartari_p()); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family=="vidhilin" && pada=="P" && matches!(gana, 5|8) { return Some(vidhilin_nu_p()); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family=="lot" && pada=="P" && gana==10 { return Some(lot_kartari_p_caus()); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family=="lang" && pada=="P" && gana==4 { return Some(crate::engine::lang_ya::lang_ya_p()); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family=="lang" && pada=="P" && gana==7 { return Some(lang_ad_p()); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family=="vidhilin" && pada=="P" && gana==7 { return Some(vidhilin_ad_p()); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family=="lang" && pada=="P" && gana==9 { return Some(lang_ni_p()); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family=="lang" && pada=="P" && matches!(gana, 5|8) { return Some(lang_nu_p()); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family=="lot" && pada=="P" && gana==9 { return Some(lot_ni_p()); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if family=="lot" && pada=="P" && matches!(gana, 5|8) { return Some(lot_nu_p()); }

    let gclass = if matches!(gana, 5|8) && family=="lat" { "nu" } else { gana_class(gana) };
    let key = (family, prayoga, pada, gclass);
    let table = match key {
        ("lat","kartari","P","thematic") => lat_kartari_p(),
        ("lat","kartari","A","thematic") => lat_kartari_a(),
        ("lat","kartari","P","ad") => lat_ad_p(),
        ("lat","kartari","A","ad") => lat_ad_a(),
        ("lrt","kartari","A","ad") => lrt_kartari_a(),
        ("lot","kartari","P","thematic") => lot_kartari_p(),
        ("lot","kartari","A","thematic") => lot_kartari_a(),
        ("lot","kartari","P","ad") => lot_ad_p(),
        ("lot","kartari","P","nu") => lot_nu_p(),
        ("lrt","kartari","P","thematic") => lrt_kartari_p(),
        ("lrt","kartari","A","thematic") => lrt_kartari_a(),
        ("lrt","kartari","P","ad") => lrt_ad_p(),
        ("lang","kartari","P","thematic") => lang_kartari_p(),
        ("lang","kartari","A","thematic") => lang_kartari_a(),
        ("lang","kartari","P","ad") => lang_ad_p(),
        ("lang","kartari","P","nu") => lang_nu_p(),
        ("vidhilin","kartari","P","thematic") => vidhilin_kartari_p(),
        ("vidhilin","kartari","A","thematic") => vidhilin_kartari_a(),
        ("vidhilin","kartari","P","ad") => vidhilin_ad_p(),
        ("vidhilin","kartari","P","nu") => vidhilin_nu_p(),
        ("lit","kartari","P","thematic") => lit_kartari_p(),
        ("lit","kartari","A","thematic") => lit_kartari_a(),
        ("lat","kartari","P","nu") => nu_lat_kartari_p(),
        ("lat","kartari","A","nu") => nu_lat_kartari_a(),
        _ => return {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gclass=="nu" {
                // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
                match (family, prayoga, pada) {
                    ("lat","kartari","P") => return Some(lat_kartari_p()),
                    ("lat","kartari","A") => return Some(lat_kartari_a()),
                    _ => {}
                }
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gclass=="ad" {
                // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
                match (family, prayoga, pada, "thematic") {
                    ("lat","kartari","P","thematic") => return Some(lat_kartari_p()),
                    ("lat","kartari","A","thematic") => return Some(lat_kartari_a()),
                    ("lot","kartari","P","thematic") => return Some(lot_kartari_p()),
                    ("lot","kartari","A","thematic") => return Some(lot_kartari_a()),
                    ("lrt","kartari","P","thematic") => return Some(lrt_kartari_p()),
                    ("lang","kartari","P","thematic") => return Some(lang_kartari_p()),
                    ("lang","kartari","A","thematic") => return Some(lang_kartari_a()),
                    ("vidhilin","kartari","P","thematic") => return Some(vidhilin_kartari_p()),
                    _ => return None,
                }
            }
            None
        },
    };
    Some(table)
}


// ---------------------------------------------------------------------------
// fn `ending_table` — tin/sUP endings: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn ending_table(lakara: &str, prayoga: &str, pada: &str, gana: u8) -> Option<Vec<(String, Vec<String>)>> {
    let ( _canon, db ) = crate::engine::lakara::normalize_lakara(lakara);
    let family = crate::engine::lakara::lakara_family(&db)?;
    let table = family_endings(&family, prayoga, pada, gana, None)?;
    Some(table.into_iter().map(|(vars, sutras)| (vars[0].clone(), sutras)).collect())
}
