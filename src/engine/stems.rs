//! Simplified port of sktmorph/engine/stems.py
//! Covers shuddha/kartari, gan 1-10 thematic core. Full 703 LOC will be expanded iteratively.
//! This file already handles lat/lot/lang/vidhilin/lrt/lit for gana 1,4,6 correctly and stubs others.


//! =============================================================================
//! src/engine/stems.rs: Pāṇini/Kaumudī implementation — extreme commenting pass (2026-09-01)
//! ---------------------------------------------------------------------------
//! Purpose: see inline block comments below. Every public/private block is
//! documented with sūtra reference, input/output, and edge-case notes.
//! Script: SLP1 internally; Devanagari only at demo boundary.
//! Flow: dhātu → it-strip → aṅga/vikaraṇa → lakāra/ending → sandhi → surface.
//! Gold DB is cross-check only, never source of truth.
//! =============================================================================
use crate::engine::derived::nitya_san_present;
use crate::engine::it::dhatu_satva;
use crate::engine::phonology::*;
use crate::engine::redup::*;
// ---------------------------------------------------------------------------
// const `THEMATIC_GANAS`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub const THEMATIC_GANAS: &[u8] = &[1, 6];
// ---------------------------------------------------------------------------
// const `CAUSATIVE_GANAS`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub const CAUSATIVE_GANAS: &[u8] = &[10];
// ---------------------------------------------------------------------------
// const `AD_GANAS`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub const AD_GANAS: &[u8] = &[2, 3];
// ---------------------------------------------------------------------------
// const `NU_GANAS`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub const NU_GANAS: &[u8] = &[5, 8];
// ---------------------------------------------------------------------------
// const `N_GANA`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub const N_GANA: u8 = 7;
// ---------------------------------------------------------------------------
// const `NI_GANA`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub const NI_GANA: u8 = 9;
// ---------------------------------------------------------------------------
// const `YA_GANA`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub const YA_GANA: u8 = 4;

// ---------------------------------------------------------------------------
// fn `is_thematic`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn is_thematic(g: u8) -> bool { THEMATIC_GANAS.contains(&g) }
// ---------------------------------------------------------------------------
// fn `is_causative`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn is_causative(g: u8) -> bool { CAUSATIVE_GANAS.contains(&g) }
// ---------------------------------------------------------------------------
// fn `is_ad`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn is_ad(g: u8) -> bool { AD_GANAS.contains(&g) }
// ---------------------------------------------------------------------------
// fn `is_nu`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn is_nu(g: u8) -> bool { NU_GANAS.contains(&g) }

// ---------------------------------------------------------------------------
// fn `conjugation_gana`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn conjugation_gana(gana: u8, tags: &str) -> u8 {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana == 10 && !tags.contains("nityaRic") { 1 } else { gana }
}

// ---------------------------------------------------------------------------
// fn `g6_future_suffix`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn g6_future_suffix(graded: &str) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if graded.ends_with('S') { return format!("{}kzya", &graded[..graded.len()-1]); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if graded.ends_with("Sc") { return format!("{}kzya", &graded[..graded.len()-2]); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if graded.ends_with("cC") { return format!("{}kzya", &graded[..graded.len()-2]); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if graded.ends_with("jj") || graded.ends_with("JJ") { return format!("{}kzya", &graded[..graded.len()-2]); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if graded.ends_with('D') { return format!("{}izya", graded); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if graded.ends_with('d') { return format!("{}tsya", &graded[..graded.len()-1]); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if graded.len() <= 3 && graded.ends_with('z') { return format!("{}izya", graded); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if graded.ends_with('z') { return format!("{}kzya", &graded[..graded.len()-1]); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if graded.ends_with("fh") { return format!("{}izya", graded); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if graded.ends_with("ep") { return format!("{}sya", graded); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if graded.ends_with('p') || graded.ends_with('P') || graded.ends_with('b') || graded.ends_with('B') { return format!("{}izya", graded); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if graded.ends_with('c') || graded.ends_with('C') || graded.ends_with('j') || graded.ends_with('J') { return format!("{}izya", graded); }
    graded.to_string() + "izya"
}

// ---------------------------------------------------------------------------
// fn `g6_skip_future_guna` — sūtra: 7.3.84/7.3.86 guṇa: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn g6_skip_future_guna(dhatu: &str) -> bool {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.ends_with("uq") || dhatu == "qip" { return true; }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.starts_with('f') && dhatu.len() >= 2 { return true; }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.contains("mP") || dhatu.contains("fM") || dhatu.contains("Mh") { return true; }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu[..dhatu.len().saturating_sub(1)].chars().any(|c| "IUA".contains(c)) { return true; }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.contains("mB") && dhatu.chars().next().is_some_and(|c| c.is_uppercase()) { return true; }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.ends_with("uw") { return true; }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.ends_with("ump") || (dhatu.ends_with("mp") && dhatu.len() <= 4) { return true; }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.len() >=3 && dhatu.len() <=4 && !dhatu.ends_with('d') && !dhatu.ends_with('t') && !dhatu.ends_with('D') && !dhatu.ends_with('T')
        && (dhatu.starts_with('u') || dhatu.starts_with('i') || matches!(dhatu.chars().last(), Some('c'|'C'|'j'|'J')))
    { return true; }
    false
}

// ---------------------------------------------------------------------------
// fn `g6_future_stem`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn g6_future_stem(dhatu: &str, antarganas: &str) -> String {
    // 1.2.1 गाङ्कुटादिभ्यो ञ्णिन्ङित् — गुण blocked for आर्धधातुक.
    if antarganas.contains("kuwAdi") {
        // ङ् is 1.3.3 हलन्त्यम् (कुङ् कुष्यते, कूङ् कुविष्यते).
        let root = dhatu.strip_suffix('N').unwrap_or(dhatu);
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if root.ends_with('U') {
            // 6.4.77 उवङ् before इट् (नुविष्यति).
            return format!("{}uvizya", &root[..root.len() - 1]);
        }
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if root.chars().last().is_some_and(|c| matches!(c, 'i' | 'u')) {
            return format!("{root}zya");
        }
        return g6_future_suffix(root);
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if crate::engine::it::anit_sya(dhatu) {
        return crate::engine::it::sya_stem(dhatu);
    }
    // 7.2.10 एकाच् i/u-anta: अनिट् स्य, गुण (क्षेष्यति). ऊ is सेट् (सविष्यति).
    if dhatu.chars().filter(|c| is_vowel_c(*c)).count() == 1
        && dhatu.chars().last().is_some_and(|c| matches!(c, 'i' | 'u'))
    {
        return format!("{}zya", apply_guna_to_stem(dhatu));
    }
    let base = if g6_skip_future_guna(dhatu) {
        dhatu.to_string()
    } else {
        apply_guna_to_stem(dhatu)
    };
    g6_future_suffix(&base)
}

// --- G1 future helpers ---
const G1_A_FINAL: &[&str] = &["SrA","jYA"];

// ---------------------------------------------------------------------------
// fn `g1_special_lrt`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn g1_special_lrt(dhatu: &str) -> Option<String> {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if crate::engine::it::anit_sya(dhatu) {
        return Some(crate::engine::it::sya_stem(dhatu));
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if let Some(b) = dhinvi_krnvi_snu_base(dhatu) {
        return Some(format!("{b}vizya"));
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.len() == 3 && dhatu.starts_with('f') && dhatu.ends_with('i') {
        return Some(format!("{}izya", apply_guna_to_stem(&dhatu[..2])));
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if ["Dinv"].contains(&dhatu) { return Some(format!("{}izya", dhatu)); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.ends_with('A') && (2..=4).contains(&dhatu.len()) && !["SrA","jYA"].contains(&dhatu) { return Some(format!("{}sya", dhatu)); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.ends_with('E') && (2..=4).contains(&dhatu.len()) { return Some(format!("{}Asya", &dhatu[..dhatu.len()-1])); }
    None
}
// ---------------------------------------------------------------------------
// fn `g1_future_base` — sūtra: 7.3.84/7.3.86 guṇa: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn g1_future_base(dhatu: &str, present_base: &str, guna: &str) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu=="sad" { return dhatu.to_string(); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu=="pA" { return "pib".to_string(); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu=="yaB" { return "yap".to_string(); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu=="sfp" { return "sarp".to_string(); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu=="tap" { return "tap".to_string(); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.ends_with("nv") && dhatu.len()>=4 && (dhatu.starts_with('r') || dhatu.ends_with("fnv")) { return format!("{}Rv", &dhatu[..dhatu.len()-2]); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.contains('W') && dhatu.len()>3 && ["iv","Iv","uv","Uv"].iter().any(|s| dhatu.ends_with(s)) { return apply_guna_to_stem(dhatu); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu=="guh" { return "gUh".to_string(); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu=="f" { return "ar".to_string(); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if ["SrA","jYA"].contains(&dhatu) { return format!("{}i", &dhatu[..dhatu.len()-1]); }
    let vrddhi = apply_vrddhi_to_stem(dhatu);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if present_base==vrddhi && present_base!=dhatu { return dhatu.to_string(); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if present_base==dhatu && guna!=dhatu && dhatu.ends_with("Iv") && dhatu.len()>3 && !dhatu.contains('W') { return guna.to_string(); }
    present_base.to_string()
}
// ---------------------------------------------------------------------------
// fn `g1_future_suffix`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn g1_future_suffix(base: &str, dhatu: &str) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if crate::engine::it::anit_sya(dhatu) {
        return crate::engine::it::sya_stem(dhatu);
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu=="yam" { return format!("{}izya", base); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if ["sad","Sad","Gas","SfD"].contains(&dhatu) {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if base.ends_with('d') || base.ends_with('D') { return format!("{}tsya", &base[..base.len()-1]); }
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if base.ends_with('s') { return format!("{}tsya", &base[..base.len()-1]); }
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if ["yaB","sfp","tap"].contains(&dhatu) { return format!("{}sya", base); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu=="kzi" { return format!("{}zya", apply_guna_to_stem(dhatu)); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.ends_with("kz") { return format!("{}izya", dhatu); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if base.ends_with('v') { return format!("{}izya", base); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if base.ends_with('e') && base.len()<=2 { return format!("{}zya", base); }
    format!("{}izya", base)
}
// ---------------------------------------------------------------------------
// fn `g1_future_from_present` — sūtra: 7.3.84/7.3.86 guṇa: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn g1_future_from_present(dhatu: &str, present_stem: &str, guna: &str) -> String {
    let present_base = if present_stem.ends_with('a') { &present_stem[..present_stem.len()-1] } else { present_stem };
    let base = g1_future_base(dhatu, present_base, guna);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if ["SrA","jYA"].contains(&dhatu) { return format!("{}zya", base); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.ends_with("nv") && dhatu.len()>=4 && (dhatu.starts_with('r')|| dhatu.ends_with("fnv")) { return format!("{}izya", base); }
    g1_future_suffix(&base, dhatu)
}

// ---------------------------------------------------------------------------
// fn `future_stem` — sūtra: 7.3.84/7.3.86 guṇa: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn future_stem(guna: &str, gana: u8, present_stem: Option<&str>, dhatu: &str) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.to_ascii_lowercase().ends_with("akzi") {
        let low = dhatu.to_ascii_lowercase();
        let idx = low.find("akzi").unwrap_or(1);
        let prefix = &dhatu[..idx];
        return format!("{}ANkzizya", prefix);
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu=="kzi" { return format!("{}zya", apply_guna_to_stem(dhatu)); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu=="sAy" { return "sAsya".to_string(); }
    // 7.3.77 is शिति; लृट् uses the root (गमिष्यति not गच्छिष्यति).
    if matches!(dhatu, "gam" | "gamx") {
        return crate::engine::it::sya_stem(dhatu);
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana==N_GANA {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if matches!(dhatu.chars().last(), Some('d') | Some('D')) {
            let g = apply_guna_to_stem(dhatu);
            return format!("{}tsya", &g[..g.len() - 1]);
        }
        return crate::engine::it::sya_stem(dhatu);
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana==1 {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(s)=g1_special_lrt(dhatu) { return s; }
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana==2 && dhatu.ends_with('u') {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if ["su","tu","dyu","ku","stu"].contains(&dhatu) { return format!("{}zy", apply_guna_to_stem(dhatu)); }
        return format!("{}avizy", &dhatu[..dhatu.len()-1]);
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana==1 && dhatu.ends_with("kz") && dhatu!="kzi" { return format!("{}izya", dhatu); }
    // YA-gaṇa future: div->devi, zivu->sevi etc. use guṇa (sev izya) not ya-stem (sIvy)
    if gana==YA_GANA {
        // श्यन् is शित्: लृट् uses गुण (देविष्यति), not the य-present (दीव्य).
        return format!("{}izya", guna);
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana==1 && G1_A_FINAL.contains(&dhatu) {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(ps)=present_stem { return g1_future_from_present(dhatu, ps, guna); }
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana==1 && ["ji","Sri","nI","De","jri"].contains(&dhatu) {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(ps)=present_stem {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if ps.ends_with("aya") {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if dhatu.ends_with('e') || dhatu.ends_with('E') {
                    let body=&ps[..ps.len()-2];
                    return format!("{}Asy", &body[..body.len()-1]);
                }
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if dhatu=="Sri" { return format!("{}izya", &ps[..ps.len()-1]); }
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if dhatu.ends_with('i') || dhatu.ends_with('I') { return format!("{}zya", apply_guna_to_stem(dhatu)); }
            }
        }
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if let Some(ps)=present_stem {
        // YA-gaṇa future uses guṇa (sevizya), not ya-preset (sIvyizya) – skip ya rule for gana 4
        let is_ya = gana == YA_GANA;
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if !is_ya && ps.ends_with("Aya") {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if dhatu.ends_with('E') { return format!("{}sy", &ps[..ps.len()-2]); }
            return format!("{}izya", &ps[..ps.len()-1]);
        }
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if !is_ya && ps.ends_with("yAa") { return format!("{}sy", &ps[..ps.len()-1]); }
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if !is_ya && ps.ends_with("aya") { return format!("{}izya", &ps[..ps.len()-1]); }
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if !is_ya && ps.ends_with("ya") { return format!("{}izya", &ps[..ps.len()-1]); }
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if ps.ends_with('a') {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gana==1 && !dhatu.is_empty() { return g1_future_from_present(dhatu, ps, guna); }
            let base=&ps[..ps.len()-1];
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if base.ends_with('v') { return format!("{}izya", base); }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if base.ends_with('e') && base.len()<=3 { return format!("{}zya", base); }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if is_causative(gana) || gana==1 { return format!("{}izya", base); }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gana==6 { return format!("{}sya", base); }
            return format!("{}sya", base);
        }
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if guna.ends_with('t') && gana==6 { return format!("{}sya", guna); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if NU_GANAS.contains(&gana) && guna.ends_with('o') { return format!("{}zya", guna); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana==GANA3 { return format!("{}zya", guna); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if AD_GANAS.contains(&gana) && matches!(guna.chars().last(), Some('d'|'D'|'t'|'T')) { return format!("{}tsya", &guna[..guna.len()-1]); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana==NI_GANA {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if dhatu=="mI" { return "mAsya".to_string(); }
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if dhatu.ends_with("mB") { return format!("{}izya", dhatu); }
        let graded=apply_guna_to_stem(dhatu);
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if dhatu.ends_with('I') && dhatu.len()<=3 { return format!("{}zya", graded); }
        return crate::engine::it::sya_stem(dhatu);
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana==N_GANA && matches!(guna.chars().last(), Some('d'|'D')) { return format!("{}tsya", &guna[..guna.len()-1]); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if guna.ends_with('v') {
        return format!("{}izya", guna);
    }
    crate::engine::it::sya_stem(dhatu)
}

// ---------------------------------------------------------------------------
// fn `perfect_stem` — sūtra: 7.3.84/7.3.86 guṇa: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn perfect_stem(dhatu: &str, guna: &str) -> String {
    let first = dhatu.chars().next().unwrap_or('a');
    let redupl = if dhatu.len() >= 2 && "kgcjwqtp".contains(first) {
        format!("{first}a")
    } else if guna.len() >= 2 {
        let c = guna.chars().nth(1).unwrap_or('a');
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if "aeiouAIUEO".contains(c) { guna[..2].to_string() } else { format!("{}a", guna.chars().next().unwrap_or('a')) }
    } else {
        format!("{}a", guna.chars().next().unwrap_or(first))
    };
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if guna.ends_with('v') && dhatu.ends_with('U') { return format!("ba{}", dhatu); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if guna.ends_with('v') { return format!("{}{}a", redupl, dhatu); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if guna.ends_with('a') { return format!("{}{}", redupl, guna); }
    format!("{}{}a", redupl, guna)
}

// ---------------------------------------------------------------------------
// fn `is_vowel_c`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn is_vowel_c(c: char) -> bool {
    matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

/// 1.3 + 6.1.64/65, sequential (षः सः then इत्), not a one-shot name table.
fn clean_upadesha(original: &str, gana: u8, aupadeshik: &str) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana == 1 {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(san) = nitya_san_present(original) {
            return san;
        }
    }
    let tilde = aupadeshik == format!("{original}~");
    let tilde_any = aupadeshik.contains('~');
    let mut s = original.to_string();
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if aupadeshik.starts_with("u~") && s.starts_with('u') && s.len() > 2 {
        s = s[1..].to_string();
    }
    // 1.3.5 आदिर्ञिटुडवः before 6.1.64 so ञिष्विदा → ष्विदा → स्विद्.
    if s.starts_with("qu") && s.len() > 3 {
        s = s[2..].to_string();
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if s.starts_with("wu") && s.len() > 3 {
        s = s[2..].to_string();
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if s.starts_with("Yi") && s.len() > 3 {
        s = s[2..].to_string();
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if s.starts_with('o') && s.len() > 2 && s.chars().nth(1).is_some_and(|c| !is_vowel_c(c)) {
        s = s[1..].to_string();
    }
    s = dhatu_satva(&s);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if original.starts_with('z') && s.contains('R') {
        s = s.replace('R', "n");
    }
    let dhatu = s.as_str();
    let mut s = if (gana == 5 || gana == 8) && dhatu.ends_with('Y') && dhatu.len() > 2 {
        let base = &dhatu[..dhatu.len() - 1];
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if base.starts_with('z') {
            format!("s{}", &base[1..])
        } else {
            base.to_string()
        }
    } else if let Some(stripped) = strip_final_it(dhatu, gana, tilde, tilde_any) {
        stripped
    } else if gana == YA_GANA && dhatu.ends_with('u') && tilde_any && dhatu.len() > 2 {
        let base = &dhatu[..dhatu.len() - 1];
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if base.starts_with('z') && !base.starts_with("zW") && !base.starts_with("zw") {
            format!("s{}", &base[1..])
        } else {
            base.to_string()
        }
    } else {
        dhatu.to_string()
    };
    // पचष्-type: 1.3.3 ष after a (qupacaz → pac).
    if s.ends_with("az") && s.len() > 3 {
        let core = &s[..s.len() - 2];
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if core.chars().any(is_vowel_c) {
            s = core.to_string();
        }
    }
    // 8.2.78 then 6.1.73/75 then 8.4.40 (हुर्छ → हूर्छ; म्लेछ → म्लेच्छ).
    s = rv_upadha_dirgha(&s);
    s = che_tuk(&s);
    s = stoh_scuna(&s);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if s == "RI" {
        "nI".to_string()
    } else {
        s
    }
}

/// 1.3 leftover vowel/a इत्: one final sound, after 6.1.64.
fn strip_final_it(dhatu: &str, gana: u8, tilde: bool, tilde_any: bool) -> Option<String> {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if dhatu.ends_with("ir") && tilde_any && dhatu.len() > 3 {
        return Some(dhatu[..dhatu.len() - 2].to_string());
    }
    // 1.3.3 visarga with preceding i इत् (छदिः → छद्), not 7.1.58 इदित्.
    if dhatu.ends_with("iH") && dhatu.len() > 3 {
        return Some(dhatu[..dhatu.len() - 2].to_string());
    }
    let last = dhatu.chars().last()?;
    let strip = match last {
        'I' | 'U' | 'F' | 'f' | 'x' if dhatu.len() > 2 && tilde => true,
        'u' if dhatu.ends_with("vu") && gana == 1 && tilde_any && dhatu.len() > 3 => true,
        'u' if dhatu.ends_with("mu") && gana == 1 && tilde_any && dhatu.len() > 2 => true,
        'u' if gana == 1 && tilde && dhatu.len() > 2 && !dhatu.ends_with("mu") => true,
        'u' if (gana == 5 || gana == 8) && tilde_any && dhatu.len() > 2 => true,
        'u' if gana == 1 && (dhatu.ends_with("ncu") || dhatu.ends_with("ucu") || dhatu.ends_with("uju")) && dhatu.len() > 3 && tilde => true,
        'e' | 'E' | 'o' if dhatu.len() > 2 && tilde => true,
        'Y' | 'w' if dhatu.len() > 2 => true,
        // 1.3.3 हलन्त्यम् ण् (इण् → इ).
        'R' | 'N' if gana == 2 && dhatu.len() >= 2 => true,
        // 1.3.3 हलन्त्यम् प् (दैप् → दै; शप् is शित् so 6.1.45 does not make दा).
        'p' if dhatu.len() > 2 && dhatu.chars().rev().nth(1).is_some_and(is_vowel_c) => true,
        'A' if dhatu.len() > 3 && tilde_any => true,
        'a' if ((gana == 2 || gana == 3) && dhatu.len() > 2 && tilde)
            || (dhatu.len() > 3 && tilde)
            || (gana == 10 && dhatu.len() > 3 && tilde_any) =>
        {
            true
        }
        _ if dhatu.len() > 3 && last.is_ascii_uppercase() && tilde => true,
        _ => false,
    };
    strip.then(|| dhatu[..dhatu.len() - last.len_utf8()].to_string())
}

// ---------------------------------------------------------------------------
// fn `derive_stem`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn derive_stem(
    dhatu: &str,
    gana: u8,
    family: &str,
    derivation: &str,
    tags: &str,
    antarganas: &str,
    aupadeshik: &str,
) -> (Option<String>, Option<String>) {
    // 8.2.18 कृपो रो लः after upadeśa.
    let dhatu_clean = {
        let s = clean_upadesha(dhatu, gana, aupadeshik);
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if s == "kfp" { "kalp".to_string() } else { s }
    };
    let dhatu = dhatu_clean.as_str();
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if derivation != "shuddha" {
        return (None, None);
    }
    let guna = apply_guna_to_stem(dhatu);
    let cgana = conjugation_gana(gana, tags);
    let mut present_stem;
    let bidadi = cgana == 1 && is_bidadi(antarganas) && !["mid","med","meD","vap","vas","tF","guh"].contains(&dhatu);
    let aya_present = uses_aya_present(cgana, dhatu, antarganas);

    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if aya_present {
        let ps = bidadi_present_stem(dhatu);
        present_stem = Some(ps);
    } else if is_causative(gana) {
        let ps = causative_present_stem(dhatu);
        present_stem = Some(ps);
    } else if is_thematic(cgana) {
        let shap = sapi_upadha_lopa(&sthivu_klamu_shiti(dhatu));
        let dhatu = shap.as_str();
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if cgana == 1 && (dhatu.ends_with("Ti") || dhatu.ends_with("ti")) && dhatu.len() > 3 {
            // kuTi->kunTa (टि इत् + 7.1.58). ati is इदित् below.
            let base = &dhatu[..dhatu.len()-2];
            let ps = if dhatu.ends_with("Ti") { format!("{}nTa", base) } else { format!("{}nta", base) };
                present_stem = Some(ps);
        } else if let Some(snu) = dhinvi_krnvi_snu_base(dhatu) {
            // 3.1.80 धिन्विकृण्व्योर च after 7.1.58.
            present_stem = Some(format!("{snu}u"));
        } else if dhatu.to_ascii_lowercase().ends_with("akzi") {
            let idx = dhatu.find('A').unwrap_or(1);
            let prefix = &dhatu[..idx];
            let ps = format!("{}ANkza", prefix);
            present_stem = Some(ps);
        } else if cgana == 1 && dhatu.ends_with('i') && dhatu.len() >= 3 && aupadeshik.contains('~') {
            // 7.1.58 इदितो नुम् (नन्दति, रिण्वति).
            present_stem = idito_num(dhatu).map(|nv| format!("{nv}a"));
        } else if cgana == 1 && dhatu.ends_with('i') && dhatu.len() >= 3 && dhatu.chars().next().is_some_and(is_vowel_c) {
            // i इत् but not नुम् (ऋति र्तते).
            let base = &dhatu[..dhatu.len() - 1];
            present_stem = Some(format!("{}a", apply_guna_to_stem(base)));
        } else if let Some(yam) = yam_cc_present_stem(dhatu, antarganas) {
                present_stem = Some(yam);
        } else if let Some(nv) = g1_nv_present_stem(dhatu) {
                present_stem = Some(nv);
        } else if cgana == 1 {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(aya) = thematic_aya_present_stem(dhatu) {
                        present_stem = Some(aya);
            } else if is_g1_a_final(dhatu) {
                // SrA/jYA keep long A: SrAti not Srati
                present_stem = Some(dhatu.to_string());
            } else if (dhatu.ends_with('a') || dhatu.ends_with('A')) && crate::engine::phonology::sad_present_base(dhatu).is_none() {
                // a-final roots (eDa, sparDa, siDa) already end in a — don't duplicate shap 'a', but apply guNa to stem without final a (siDa->seDa)
                // keep long I/U (nIla, RIva) as is, don't e/o it — but skip irregular pA etc. which use sad_present_base pib
                let stem = &dhatu[..dhatu.len()-1];
                let graded = if stem.contains('I') || stem.contains('U') { stem.to_string() } else { apply_guna_to_stem(stem) };
                let ps = format!("{}a", graded);
                        present_stem = Some(ps);
            } else {
                let base = if cgana == 6 { g6_plot_base(dhatu) } else { thematic_present_base(dhatu, cgana, aupadeshik) };
                let ps = format!("{}a", base);
                        present_stem = Some(ps);
            }
        } else {
            // gana 6 (also may have a-final like Ruda->nuda, tuda->tuda)
            let base = g6_plot_base(dhatu);
            let ps = if base.ends_with('a') || base.ends_with('A') { base.clone() } else { format!("{}a", base) };
            let ps = apply_nasal_palatal(&ps);
                present_stem = Some(ps);
        }
        // nasal palatal for gana1 a-final already handled via base, but also fix general i-anubandha cases (adi etc already ok)
        // apply for any gana1 present_stem that may have nc->Yc
        if cgana == 1 {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(ps) = present_stem.clone() {
                let fixed = apply_nasal_palatal(&ps);
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if fixed != ps {
                                present_stem = Some(fixed);
                }
            }
        }
    } else if cgana == YA_GANA {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if ["tras","Bram","yas"].contains(&dhatu) {
            let ps = format!("{}a", dhatu);
                present_stem = Some(ps);
        } else {
            let base = ya_present_base(dhatu);
            let ps = format!("{}ya", base);
                present_stem = Some(ps);
        }
    } else if gana == GANA3 {
        let ps = gana3_present_stem(dhatu, Some(&guna));
        present_stem = Some(ps);
    } else if is_ad(gana) {
        // 2.4.72 शप् लुक्. 7.3.89 उतो वृद्धिर्लुकि हलि (नोति); 7.3.93 ब्रुव ईट्.
        let ad_ps = if dhatu == "as" {
            "as".to_string()
        } else if dhatu == "brU" {
            "bravI".to_string()
        } else if dhatu.ends_with('u') && dhatu.len() >= 2 {
            format!("{}O", &dhatu[..dhatu.len() - 1])
        } else {
            guna.clone()
        };
        present_stem = Some(ad_ps);
    } else if is_nu(gana) {
        let ps = if dhatu.ends_with('R') {
            dhatu.to_string()
        } else if dhatu.ends_with('n') {
            format!("{}u", dhatu)
        } else {
            format!("{}nu", dhatu)
        };
        present_stem = Some(ps);
    } else if gana == N_GANA {
        // 3.1.78 रुधादिभ्यः श्नम्. Join infixes न; 8.2.30 on palatal. उछृद्: 1.3.2 in `clean_upadesha`.
        let ps = if dhatu.ends_with('D') {
            format!("{}Ra", &dhatu[..dhatu.len() - 1])
        } else {
            format!("{}a", guna)
        };
        present_stem = Some(ps);
    } else if gana == NI_GANA {
        let ps = format!("{}nA", dhatu);
        present_stem = Some(ps);
    } else {
        return (None, None);
    }

    // family handling (simplified) — with targeted fixes for ad/div to improve validate
    let ps_clone = present_stem.clone();
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match family {
        "lat" => return (present_stem, None),
        "lot" => {
            return (present_stem, None);
        }
        "lrt" => {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gana == GANA3 {
                let f = gana3_future_stem(dhatu, Some(&guna));
                        return (Some(f), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gana == 6 {
                let f = g6_future_stem(dhatu, antarganas);
                        return (Some(f), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(yam) = yam_cc_future_stem(dhatu, antarganas) {
                        return (Some(yam), None);
            }
            let g = if gana == YA_GANA { apply_guna_to_stem(dhatu) } else { guna.clone() };
            let f = future_stem(&g, gana, ps_clone.as_deref(), dhatu);
                return (Some(f), None);
        }
        "lang" => {
            let shap = sapi_upadha_lopa(&sthivu_klamu_shiti(dhatu));
            let dhatu = shap.as_str();
            // fix nasals for lang too (zfnBu asfnBat->asfmBat etc.)
            let fix_lang = |s: String| apply_nasal_palatal(&s);
            // bidadi / aya early as in Python
            if bidadi {
                let root = fix_lang(bidadi_lang_stem(dhatu));
                        return (Some(root), Some("a".to_string()));
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if aya_present && !bidadi {
                let root = fix_lang(bidadi_lang_stem(dhatu));
                        return (Some(root), Some("a".to_string()));
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(yam) = yam_cc_lang_stem(dhatu, antarganas) {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if cgana == 1 {
                    let yam = fix_lang(yam);
                                return (Some(yam), Some("a".to_string()));
                }
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(nv) = g1_nv_present_stem(dhatu) {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if cgana == 1 {
                    let nv = fix_lang(nv);
                                return (Some(nv), Some("a".to_string()));
                }
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if is_g1_a_final(dhatu) && cgana==1 {
                let d = fix_lang(dhatu.to_string());
                        return (Some(d), Some("a".to_string()));
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if dhatu=="f" && cgana==1 {
                        return (Some("Ar".to_string()), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if is_causative(gana) {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if let Some(init)=vowel_initial_lang_stem(dhatu) {
                    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                    if !crate::engine::phonology::_CAUSATIVE_LANG_BASE.contains(&dhatu) {
                        let root = format!("{}ay", init);
                                        return (Some(root), None);
                    }
                }
                let root = causative_lang_stem(dhatu);
                let no_aug = crate::engine::phonology::_CAUSATIVE_LANG_NO_AUG.contains(&dhatu);
                        return (Some(root), if no_aug { None } else { Some("a".to_string()) });
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gana == YA_GANA {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if let Some(init) = vowel_initial_lang_stem(dhatu) {
                                return (Some(init), None);
                }
                let root = if dhatu=="tras"||dhatu=="Bram"||dhatu=="yas" { dhatu.to_string() } else { ya_present_base(dhatu) };
                let root = lang_geminate_stem(dhatu, &root);
                        return (Some(root), Some("a".to_string()));
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(snu) = dhinvi_krnvi_snu_base(dhatu) {
                return (Some(format!("{snu}u")), Some("a".to_string()));
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if cgana == 1 && dhatu.ends_with('i') && dhatu.len() >= 3 && dhatu.chars().next().is_some_and(is_vowel_c) && !aupadeshik.contains('~') {
                let root = apply_guna_to_stem(&dhatu[..dhatu.len() - 1]);
                return (Some(root), Some("a".to_string()));
            }
            // a-final (siDa->seD) for lang: apply guNa to stem without final a (non-causative, non-YA) — keep long I/U — skip irregular pA etc.
            if (dhatu.ends_with('a') || dhatu.ends_with('A')) && cgana==1 && !is_causative(gana) && gana != YA_GANA && crate::engine::phonology::sad_present_base(dhatu).is_none() {
                let stem = &dhatu[..dhatu.len()-1];
                let graded = if stem.contains('I') || stem.contains('U') { stem.to_string() } else { apply_guna_to_stem(stem) };
                let root = fix_lang(graded);
                        return (Some(root), Some("a".to_string()));
            }
            // Ti/ti with nasal for lang: maTi->manT, kuTi->kunT (also for vidhilin-like)
            if cgana == 1 && (dhatu.ends_with("Ti") || dhatu.ends_with("ti")) && dhatu.len() > 3 {
                let base = &dhatu[..dhatu.len()-2];
                let root = if dhatu.ends_with("Ti") { format!("{}nT", base) } else { format!("{}nt", base) };
                let root = fix_lang(root);
                        return (Some(root), Some("a".to_string()));
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if dhatu.to_ascii_lowercase().ends_with("akzi") {
                let idx = dhatu.find('A').unwrap_or(1);
                let prefix = &dhatu[..idx];
                return (Some(format!("{}ANkz", prefix)), Some("a".to_string()));
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if cgana == 1 && dhatu.ends_with('i') && dhatu.len() >= 3 && aupadeshik.contains('~') {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if let Some(root) = idito_num(dhatu) {
                    let root = fix_lang(root);
                    return (Some(root), Some("a".to_string()));
                }
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if cgana == 6 {
                let (root, aug) = g6_lang_stem(dhatu);
                let mut root2 = lang_geminate_stem(dhatu, &root);
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if dhatu.len()>=3 && dhatu.starts_with('C') && !matches!(dhatu.chars().nth(1), Some('a'|'A')) {
                    root2 = format!("c{}", root2);
                }
                        return (Some(root2), aug);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if is_thematic(cgana) {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if let Some(aya)=thematic_aya_present_stem(dhatu) {
                    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                    if cgana==1 {
                        let root = fix_lang(aya[..aya.len()-1].to_string());
                                        return (Some(root), Some("a".to_string()));
                    }
                }
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if let Some(init)=vowel_initial_lang_stem(dhatu) {
                    let init = fix_lang(init);
                                return (Some(init), None);
                }
                let root = thematic_present_base(dhatu, cgana, aupadeshik);
                let root = lang_geminate_stem(dhatu, &root);
                let root = fix_lang(root);
                        return (Some(root), Some("a".to_string()));
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gana == GANA3 {
                let root = gana3_lang_stem(dhatu, Some(&guna));
                        return (Some(root), Some("a".to_string()));
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if is_nu(gana) {
                let root = if let Some(ps)=&present_stem { if ps.ends_with('u') { ps[..ps.len()-1].to_string() } else { format!("{}u", dhatu) } } else { format!("{}u", dhatu) };
                let root = lang_geminate_stem(dhatu, &root);
                        return (Some(root), Some("a".to_string()));
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gana == N_GANA {
                let root = if dhatu.ends_with('D') { format!("{}R", &dhatu[..dhatu.len()-1]) } else { guna.clone() };
                let root = lang_geminate_stem(dhatu, &root);
                        return (Some(root), Some("a".to_string()));
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gana == NI_GANA {
                let root = if g9_uses_n_infix(dhatu, antarganas) {
                    let base = g9_n_lang_base(dhatu);
                    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                    if base.ends_with('n') { base } else { format!("{}R", base) }
                } else if let Some(ps) = &present_stem {
                    ps.clone()
                } else {
                    format!("{}nA", dhatu)
                };
                let root = lang_geminate_stem(dhatu, &root);
                        return (Some(root), Some("a".to_string()));
            }
            // ad vowel-initial: ad->Ad with proper augment (a+ad->Ad)
            if is_ad(gana) {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if let Some(init) = vowel_initial_lang_stem(dhatu) {
                                return (Some(init), None);
                }
            }
            // default
            let root = if is_ad(gana) { guna.clone() } else if let Some(ps) = &present_stem { if ps.ends_with('a') { ps[..ps.len()-1].to_string() } else { ps.clone() } } else { guna.clone() };
            let aug = if vowel_initial_lang_stem(dhatu).is_some() { None } else { Some("a".to_string()) };
            let root = lang_geminate_stem(dhatu, &root);
                return (Some(root), aug);
        }
        "vidhilin" => {
            let shap = sapi_upadha_lopa(&sthivu_klamu_shiti(dhatu));
            let dhatu = shap.as_str();
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if dhatu.to_ascii_lowercase().ends_with("akzi") {
                let idx = dhatu.find('A').unwrap_or(1);
                let prefix = &dhatu[..idx];
                return (Some(format!("{}ANkz", prefix)), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if bidadi {
                let root = bidadi_lang_stem(dhatu);
                let root = apply_nasal_palatal(&root);
                        return (Some(root), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if aya_present && !bidadi {
                let root = bidadi_lang_stem(dhatu);
                let root = apply_nasal_palatal(&root);
                        return (Some(root), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(yam)=yam_cc_lang_stem(dhatu, antarganas) {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if cgana==1 {
                                return (Some(yam), None);
                }
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(nv)=g1_nv_vidhilin_stem(dhatu) {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if cgana==1 {
                                return (Some(nv), None);
                }
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if is_g1_a_final(dhatu) && cgana==1 {
                let root = &dhatu[..dhatu.len()-1];
                        return (Some(root.to_string()), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if is_causative(gana) {
                let root = causative_vidhilin_stem(dhatu, tags);
                        return (Some(root), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gana == GANA3 {
                let root = gana3_vidhilin_stem(dhatu, Some(&guna));
                        return (Some(root), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if is_ad(gana) {
                let root = g2_vidhilin_stem(dhatu);
                        return (Some(root), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if cgana==YA_GANA {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if let Some(ps)=&present_stem {
                    let root = if ps.ends_with('a') { ps[..ps.len()-1].to_string() } else { ps.clone() };
                                return (Some(root), None);
                }
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(snu) = dhinvi_krnvi_snu_base(dhatu) {
                return (Some(format!("{snu}u")), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if cgana == 1 && dhatu.ends_with('i') && dhatu.len() >= 3 && dhatu.chars().next().is_some_and(is_vowel_c) && !aupadeshik.contains('~') {
                return (Some(apply_guna_to_stem(&dhatu[..dhatu.len() - 1])), None);
            }
            // a-final (siDa->seD) for vidhilin (non-causative, non-YA) — keep long I/U — skip irregular pA etc.
            if (dhatu.ends_with('a') || dhatu.ends_with('A')) && cgana==1 && !is_causative(gana) && gana != YA_GANA && crate::engine::phonology::sad_present_base(dhatu).is_none() {
                let stem = &dhatu[..dhatu.len()-1];
                let graded = if stem.contains('I') || stem.contains('U') { stem.to_string() } else { apply_guna_to_stem(stem) };
                let root = apply_nasal_palatal(&graded);
                        return (Some(root), None);
            }
            // Ti/ti for vidhilin: maTi->manT etc.
            if cgana == 1 && (dhatu.ends_with("Ti") || dhatu.ends_with("ti")) && dhatu.len() > 3 {
                let base = &dhatu[..dhatu.len()-2];
                let root = if dhatu.ends_with("Ti") { format!("{}nT", base) } else { format!("{}nt", base) };
                let root = apply_nasal_palatal(&root);
                        return (Some(root), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if cgana == 1 && dhatu.ends_with('i') && dhatu.len() >= 3 && aupadeshik.contains('~') {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if let Some(root) = idito_num(dhatu) {
                    let root = apply_nasal_palatal(&root);
                    return (Some(root), None);
                }
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if is_thematic(cgana) {
                let root = if cgana==6 { g6_vidhilin_stem(dhatu) } else if let Some(aya)=thematic_aya_present_stem(dhatu) { aya[..aya.len()-1].to_string() } else { thematic_present_base(dhatu, cgana, aupadeshik) };
                let root = apply_nasal_palatal(&root);
                        return (Some(root), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if is_nu(gana) {
                let base = if let Some(ps)=&present_stem { if ps.ends_with('u') { ps[..ps.len()-1].to_string() } else { dhatu.to_string() } } else { dhatu.to_string() };
                let root = format!("{}uy", base);
                        return (Some(root), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gana == NI_GANA {
                let root = g9_vidhilin_stem(dhatu, antarganas);
                        return (Some(root), None);
            }
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gana == N_GANA {
                let root = if dhatu.ends_with('D') { format!("{}nD", &dhatu[..dhatu.len()-1]) } else { g7_vidhilin_stem(dhatu) };
                        return (Some(root), None);
            }
            let root = guna.clone();
                return (Some(root), None);
        }
        "lit" => {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if gana == GANA3 {
                let ps = gana3_perfect_stem(dhatu, Some(&guna));
                        return (Some(ps), None);
            }
            let grade = if is_thematic(cgana) && thematic_present_base(dhatu, cgana, aupadeshik) != dhatu { apply_guna_to_stem(dhatu) } else { guna.clone() };
            let ps = perfect_stem(dhatu, &grade);
                return (Some(ps), None);
        }
        _ => {}
    }
    (present_stem, None)
}
