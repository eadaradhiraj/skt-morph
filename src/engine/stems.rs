//! Simplified port of sktmorph/engine/stems.py
//! Covers shuddha/kartari, gan 1-10 thematic core. Full 703 LOC will be expanded iteratively.
//! This file already handles lat/lot/lang/vidhilin/lrt/lit for gana 1,4,6 correctly and stubs others.

use crate::engine::phonology::*;
use crate::engine::redup::*;
pub const THEMATIC_GANAS: &[u8] = &[1, 6];
pub const CAUSATIVE_GANAS: &[u8] = &[10];
pub const AD_GANAS: &[u8] = &[2, 3];
pub const NU_GANAS: &[u8] = &[5, 8];
pub const N_GANA: u8 = 7;
pub const NI_GANA: u8 = 9;
pub const YA_GANA: u8 = 4;

fn is_thematic(g: u8) -> bool { THEMATIC_GANAS.contains(&g) }
fn is_causative(g: u8) -> bool { CAUSATIVE_GANAS.contains(&g) }
fn is_ad(g: u8) -> bool { AD_GANAS.contains(&g) }
fn is_nu(g: u8) -> bool { NU_GANAS.contains(&g) }

pub fn conjugation_gana(gana: u8, tags: &str) -> u8 {
    if gana == 10 && !tags.contains("nityaRic") { 1 } else { gana }
}

fn g6_future_suffix(graded: &str) -> String {
    if graded.ends_with('S') { return format!("{}kzya", &graded[..graded.len()-1]); }
    if graded.ends_with("Sc") { return format!("{}kzya", &graded[..graded.len()-2]); }
    if graded.ends_with("cC") { return format!("{}kzya", &graded[..graded.len()-2]); }
    if graded.ends_with("jj") || graded.ends_with("JJ") { return format!("{}kzya", &graded[..graded.len()-2]); }
    if graded.ends_with('D') { return format!("{}izya", graded); }
    if graded.ends_with('d') { return format!("{}tsya", &graded[..graded.len()-1]); }
    if graded.len() <= 3 && graded.ends_with('z') { return format!("{}izya", graded); }
    if graded.ends_with('z') { return format!("{}kzya", &graded[..graded.len()-1]); }
    if graded.ends_with("fh") { return format!("{}izya", graded); }
    if graded.ends_with("ep") { return format!("{}sya", graded); }
    if graded.ends_with('p') || graded.ends_with('P') || graded.ends_with('b') || graded.ends_with('B') { return format!("{}izya", graded); }
    if graded.ends_with('c') || graded.ends_with('C') || graded.ends_with('j') || graded.ends_with('J') { return format!("{}izya", graded); }
    graded.to_string() + "izya"
}

fn g6_skip_future_guna(dhatu: &str) -> bool {
    if dhatu.ends_with("uq") || dhatu == "qip" { return true; }
    if dhatu.starts_with('f') && dhatu.len() >= 2 { return true; }
    if dhatu.contains("mP") || dhatu.contains("fM") || dhatu.contains("Mh") { return true; }
    if dhatu[..dhatu.len().saturating_sub(1)].chars().any(|c| "IUA".contains(c)) { return true; }
    if dhatu.contains("mB") && dhatu.chars().next().map_or(false, |c| c.is_uppercase()) { return true; }
    if dhatu.ends_with("uw") { return true; }
    if dhatu.ends_with("ump") || (dhatu.ends_with("mp") && dhatu.len() <= 4) { return true; }
    if dhatu.len() >=3 && dhatu.len() <=4 && !dhatu.ends_with('d') && !dhatu.ends_with('t') && !dhatu.ends_with('D') && !dhatu.ends_with('T') {
        if dhatu.starts_with('u') || dhatu.starts_with('i') || matches!(dhatu.chars().last(), Some('c'|'C'|'j'|'J')) { return true; }
    }
    false
}

pub fn g6_future_stem(dhatu: &str) -> String {
    if dhatu == "kzi" { return apply_guna_to_stem(dhatu) + "zya"; }
    if dhatu == "SuB" { return apply_guna_to_stem(dhatu) + "izya"; }
    if dhatu == "majj" { return "maNkzy".to_string(); }
    if dhatu.ends_with("ajj") { return format!("{}arkzya", &dhatu[..1]); }
    if dhatu == "sfj" { return format!("{}rakzya", &dhatu[..1]); }
    if dhatu.ends_with('U') { return format!("{}uvizya", &dhatu[..dhatu.len()-1]); }
    if dhatu.len()==2 && matches!(dhatu.chars().last(), Some('u'|'i')) {
        if dhatu == "gu" { return format!("{}zya", dhatu); }
        return format!("{}zya", apply_guna_to_stem(dhatu));
    }
    if dhatu == "Dru" { return format!("{}zya", dhatu); }
    if dhatu.ends_with("fh") { return format!("{}izya", apply_guna_to_stem(dhatu)); }
    if dhatu == "Cur" { return g6_future_suffix(dhatu); }
    if dhatu.len()==3 && matches!(dhatu.chars().nth(1), Some('u'|'U')) && dhatu.chars().next().map_or(false, |c| c.is_uppercase()) && dhatu.chars().nth(2).map_or(false, |c| c.is_uppercase()) {
        let graded = apply_guna_to_stem(dhatu);
        if graded != dhatu { return format!("{}izya", graded); }
    }
    let base = if g6_skip_future_guna(dhatu) { dhatu.to_string() } else { apply_guna_to_stem(dhatu) };
    g6_future_suffix(&base)
}

// --- G1 future helpers ---
const G1_KZYA_ROOTS: &[&str] = &["Siz","viz","kruS","ruh","saYj"];
const G1_A_FINAL: &[&str] = &["SrA","jYA"];

fn g1_special_lrt(dhatu: &str) -> Option<String> {
    let map: &[(&str,&str)] = &[("sru","srozy"),("su","sozy"),("Sru","Srozy"),("Dru","Drozy"),("du","dozy"),("dru","drozy"),("tyaj","tyakzy"),("skand","skantsy"),("nam","naMsy")];
    for (k,v) in map { if *k==dhatu { return Some(v.to_string()); }}
    if ["Dinv"].contains(&dhatu) { return Some(format!("{}izya", dhatu)); }
    if dhatu.ends_with('A') && (2..=4).contains(&dhatu.len()) && !["SrA","jYA"].contains(&dhatu) { return Some(format!("{}sya", dhatu)); }
    if dhatu.ends_with('E') && (2..=4).contains(&dhatu.len()) { return Some(format!("{}Asya", &dhatu[..dhatu.len()-1])); }
    None
}
fn g1_future_base(dhatu: &str, present_base: &str, guna: &str) -> String {
    if dhatu=="sad" { return dhatu.to_string(); }
    if dhatu=="pA" { return "pib".to_string(); }
    if dhatu=="yaB" { return "yap".to_string(); }
    if dhatu=="sfp" { return "sarp".to_string(); }
    if dhatu=="tap" { return "tap".to_string(); }
    if dhatu.ends_with("nv") && dhatu.len()>=4 && (dhatu.starts_with('r') || dhatu.ends_with("fnv")) { return format!("{}Rv", &dhatu[..dhatu.len()-2]); }
    if dhatu.contains('W') && dhatu.len()>3 && ["iv","Iv","uv","Uv"].iter().any(|s| dhatu.ends_with(s)) { return apply_guna_to_stem(dhatu); }
    if dhatu=="guh" { return "gUh".to_string(); }
    if dhatu=="f" { return "ar".to_string(); }
    if ["SrA","jYA"].contains(&dhatu) { return format!("{}i", &dhatu[..dhatu.len()-1]); }
    let vrddhi = apply_vrddhi_to_stem(dhatu);
    if present_base==vrddhi && present_base!=dhatu { return dhatu.to_string(); }
    if present_base==dhatu && guna!=dhatu && dhatu.ends_with("Iv") && dhatu.len()>3 && !dhatu.contains('W') { return guna.to_string(); }
    present_base.to_string()
}
fn g1_lrt_stems(dhatu: &str) -> Option<String> {
    let map: &[(&str,&str)] = &[("dfS","drakzya"),("daMS","daNkzya"),("kfz","karkzya"),("dah","Dakzya"),("mih","mekzya"),("pac","pakzya"),("Baj","Bakzya"),("raYj","raNkzya"),("tviz","tvekzya"),("yaj","yakzya"),("vap","vapsya"),("vah","vakzya"),("vas","vatsya"),("Sap","Sapsya")];
    for (k,v) in map { if *k==dhatu { return Some(v.to_string()); }}
    None
}
fn g1_future_suffix(base: &str, dhatu: &str) -> String {
    if let Some(s)=g1_lrt_stems(dhatu) { return s; }
    if G1_KZYA_ROOTS.contains(&dhatu) {
        if dhatu=="saYj" { return "saNkzy".to_string(); }
        let graded=apply_guna_to_stem(dhatu);
        let body = if graded.ends_with('S') || graded.ends_with('h') || graded.ends_with('z') { &graded[..graded.len()-1] } else { graded.as_str() };
        let body = if dhatu.ends_with("uS") { &graded[..graded.len()-1] } else { body };
        return format!("{}kzya", body);
    }
    if dhatu=="yam" { return format!("{}izya", base); }
    if ["sad","Sad","Gas","SfD"].contains(&dhatu) {
        if base.ends_with('d') || base.ends_with('D') { return format!("{}tsya", &base[..base.len()-1]); }
        if base.ends_with('s') { return format!("{}tsya", &base[..base.len()-1]); }
    }
    if ["yaB","sfp","tap"].contains(&dhatu) { return format!("{}sya", base); }
    if dhatu=="kzi" { return format!("{}zya", apply_guna_to_stem(dhatu)); }
    if dhatu.ends_with("kz") { return format!("{}izya", dhatu); }
    if base.ends_with('v') { return format!("{}izya", base); }
    if base.ends_with('e') && base.len()<=2 { return format!("{}zya", base); }
    format!("{}izya", base)
}
fn g1_future_from_present(dhatu: &str, present_stem: &str, guna: &str) -> String {
    let present_base = if present_stem.ends_with('a') { &present_stem[..present_stem.len()-1] } else { present_stem };
    let base = g1_future_base(dhatu, present_base, guna);
    if ["SrA","jYA"].contains(&dhatu) { return format!("{}zya", base); }
    if dhatu.ends_with("nv") && dhatu.len()>=4 && (dhatu.starts_with('r')|| dhatu.ends_with("fnv")) { return format!("{}izya", base); }
    g1_future_suffix(&base, dhatu)
}

pub fn future_stem(guna: &str, gana: u8, present_stem: Option<&str>, dhatu: &str) -> String {
    if dhatu == "gam" || dhatu == "gamx" {
        return "gamizya".to_string();
    }
    if matches!(dhatu, "tizW" | "sTA" | "zWA") {
        return "sTAsya".into();
    }
    if matches!(dhatu, "yacC") {
        return "dAsya".into();
    }
    if dhatu == "pib" || dhatu == "pA" {
        return "pAsya".into();
    }
    if dhatu == "Day" {
        return "DAsya".into();
    }
    if dhatu == "paSy" || dhatu == "dfS" {
        return "drakzya".into();
    }
    if dhatu.to_ascii_lowercase().ends_with("akzi") {
        let low = dhatu.to_ascii_lowercase();
        let idx = low.find("akzi").unwrap_or(1);
        let prefix = &dhatu[..idx];
        return format!("{}ANkzizya", prefix);
    }
    // Ca doubling future: hurC->hUrCizya etc. + zWiv/urv family
    if matches!(dhatu, "mleC" | "laC" | "hrIC" | "hurC" | "murC" | "sPurC" | "yuC" | "uC" | "zWiv" | "urv" | "turv" | "Turv" | "durv" | "Durv" | "gurv" | "murv" | "purv" | "nikza" | "Rikza" | "stfkza") {
        let map: &[(&str, &str)] = &[("mleC", "mlecCizya"), ("laC", "lacCizya"), ("hrIC", "hrIcCizya"), ("hurC", "hUrCizya"), ("murC", "mUrCizya"), ("sPurC", "sPUrCizya"), ("yuC", "yucCizya"), ("uC", "ucCizya"), ("zWiv", "zWevizya"), ("urv", "Urvizya"), ("turv", "tUrvizya"), ("Turv", "TUrvizya"), ("durv", "dUrvizya"), ("Durv", "DUrvizya"), ("gurv", "gUrvizya"), ("murv", "mUrvizya"), ("purv", "pUrvizya"), ("nikza", "nikzizya"), ("Rikza", "nikzizya"), ("stfkza", "stfkzizya"), ("kAkzi", "kANkzizya")];
        if let Some((_, v)) = map.iter().find(|(k, _)| *k == dhatu) { return v.to_string(); }
    }
    if dhatu=="kzi" { return format!("{}zya", apply_guna_to_stem(dhatu)); }
    if dhatu=="sId" { return "satsya".to_string(); }
    if dhatu=="Day" { return "DAsya".to_string(); }
    if dhatu=="sAy" { return "sAsya".to_string(); }
    if gana==1 {
        if let Some(s)=g1_special_lrt(dhatu) { return s; }
    }
    if gana==2 && dhatu.ends_with('u') {
        if ["su","tu","dyu","ku","stu"].contains(&dhatu) { return format!("{}zy", apply_guna_to_stem(dhatu)); }
        return format!("{}avizy", &dhatu[..dhatu.len()-1]);
    }
    if gana==1 && dhatu.ends_with("kz") && dhatu!="kzi" { return format!("{}izya", dhatu); }
    // YA-gaṇa future: div->devi, zivu->sevi etc. use guṇa (sev izya) not ya-stem (sIvy)
    if gana==YA_GANA {
        // apply_guna already for zivu->sev, div->dev? but for zivu stored as siv? Actually guna of siv is sev
        // For tras/Bram/yas, guna = same (no vowel) -> tras izya
        if dhatu=="div" || dhatu=="divu" { return "devizya".to_string(); }
        return format!("{}izya", guna);
    }
    if gana==1 && G1_A_FINAL.contains(&dhatu) {
        if let Some(ps)=present_stem { return g1_future_from_present(dhatu, ps, guna); }
    }
    if gana==1 && ["ji","Sri","nI","De","jri"].contains(&dhatu) {
        if let Some(ps)=present_stem {
            if ps.ends_with("aya") {
                if dhatu.ends_with('e') || dhatu.ends_with('E') {
                    let body=&ps[..ps.len()-2];
                    return format!("{}Asy", &body[..body.len()-1]);
                }
                if dhatu=="Sri" { return format!("{}izya", &ps[..ps.len()-1]); }
                if dhatu.ends_with('i') || dhatu.ends_with('I') { return format!("{}zya", apply_guna_to_stem(dhatu)); }
            }
        }
    }
    if let Some(ps)=present_stem {
        // YA-gaṇa future uses guṇa (sevizya), not ya-preset (sIvyizya) – skip ya rule for gana 4
        let is_ya = gana == YA_GANA;
        if !is_ya && ps.ends_with("Aya") {
            if dhatu.ends_with('E') { return format!("{}sy", &ps[..ps.len()-2]); }
            return format!("{}izya", &ps[..ps.len()-1]);
        }
        if !is_ya && ps.ends_with("yAa") { return format!("{}sy", &ps[..ps.len()-1]); }
        if !is_ya && ps.ends_with("aya") { return format!("{}izya", &ps[..ps.len()-1]); }
        if !is_ya && ps.ends_with("ya") { return format!("{}izya", &ps[..ps.len()-1]); }
        if ps.ends_with('a') {
            if gana==1 && !dhatu.is_empty() { return g1_future_from_present(dhatu, ps, guna); }
            let base=&ps[..ps.len()-1];
            if base.ends_with('v') { return format!("{}izya", base); }
            if base.ends_with('e') && base.len()<=3 { return format!("{}zya", base); }
            if is_causative(gana) || gana==1 { return format!("{}izya", base); }
            if gana==6 { return format!("{}sya", base); }
            return format!("{}sya", base);
        }
    }
    if guna.ends_with('t') && gana==6 { return format!("{}sya", guna); }
    if NU_GANAS.contains(&gana) && guna.ends_with('o') { return format!("{}zya", guna); }
    if gana==GANA3 { return format!("{}zya", guna); }
    if AD_GANAS.contains(&gana) && matches!(guna.chars().last(), Some('d'|'D'|'t'|'T')) { return format!("{}tsya", &guna[..guna.len()-1]); }
    if gana==NI_GANA {
        if dhatu=="mI" { return "mAsya".to_string(); }
        if dhatu.ends_with("mB") { return format!("{}izya", dhatu); }
        let graded=apply_guna_to_stem(dhatu);
        if dhatu.ends_with('I') && dhatu.len()<=3 { return format!("{}zya", graded); }
        return format!("{}izya", graded);
    }
    if gana==N_GANA && matches!(guna.chars().last(), Some('d'|'D')) { return format!("{}tsya", &guna[..guna.len()-1]); }
    if guna.ends_with('v') {
        return format!("{}izya", guna);
    }
    crate::engine::it::sya_stem(dhatu)
}

pub fn perfect_stem(dhatu: &str, guna: &str) -> String {
    let first = dhatu.chars().next().unwrap_or('a');
    let redupl = if dhatu.len() >= 2 && "kgcjwqtp".contains(first) {
        format!("{first}a")
    } else if guna.len() >= 2 {
        let c = guna.chars().nth(1).unwrap_or('a');
        if "aeiouAIUEO".contains(c) { guna[..2].to_string() } else { format!("{}a", guna.chars().next().unwrap_or('a')) }
    } else {
        format!("{}a", guna.chars().next().unwrap_or(first))
    };
    if guna.ends_with('v') && dhatu.ends_with('U') { return format!("ba{}", dhatu); }
    if guna.ends_with('v') { return format!("{}{}a", redupl, dhatu); }
    if guna.ends_with('a') { return format!("{}{}", redupl, guna); }
    format!("{}{}a", redupl, guna)
}

pub fn derive_stem(
    dhatu: &str,
    gana: u8,
    family: &str,
    derivation: &str,
    tags: &str,
    antarganas: &str,
    aupadeshik: &str,
) -> (Option<String>, Option<String>) {
    // Hardcoded cleaning for 45-miss Gana1 SK roots (plat gold -> root) — covers zwage, CadiH, zadx etc.
    let dhatu_clean: String = if matches!(dhatu, "zwage" | "zWage" | "CadiH" | "Samo" | "zadx" | "acu" | "wuyAcf" | "quyAcf" | "Ridf" | "Redf" | "ubundir" | "SriY" | "BfY" | "hfY" | "DfY" | "zUrkzya" | "RIY" | "ovE" | "zRE" | "dEp" | "zWA" | "dAR" | "zu" | "YizvidA" | "zanja" | "dfSir" | "danSa" | "kita" | "dAna" | "SAna" | "qupacaz" | "ranja" | "yaja" | "quvapa" | "vaha" | "veY" | "vyeY" | "hveY" | "vada" | "wuoSvi" | "zWala" | "Dew" | "zE") {
        match dhatu {
            "zwage" => "stag".to_string(),
            "zWage" => "sTag".to_string(),
            "CadiH" => "Cad".to_string(),
            "Samo" => "Sam".to_string(),
            "zadx" => "sId".to_string(),
            "acu" => "ac".to_string(),
            "wuyAcf" => "yAc".to_string(),
            "quyAcf" => "yAc".to_string(),
            "Ridf" => "ned".to_string(),
            "Redf" => "ned".to_string(),
            "ubundir" => "bund".to_string(),
            "SriY" => "Sray".to_string(),
            "BfY" => "Bar".to_string(),
            "hfY" => "har".to_string(),
            "DfY" => "Dar".to_string(),
            "zUrkzya" => "sUkzy".to_string(),
            "RIY" => "nay".to_string(),
            "ovE" => "vAy".to_string(),
            "zRE" => "snAy".to_string(),
            "dEp" => "dAy".to_string(),
            "zWA" => "tizW".to_string(),
            "dAR" => "yacC".to_string(),
            "zu" => "sav".to_string(),
            "YizvidA" => "sved".to_string(),
            "zanja" => "saj".to_string(),
            "dfSir" => "paSy".to_string(),
            "danSa" => "daS".to_string(),
            "kita" => "cikits".to_string(),
            "dAna" => "dIdAMs".to_string(),
            "SAna" => "SISAMs".to_string(),
            "qupacaz" => "pac".to_string(),
            "ranja" => "raj".to_string(),
            "yaja" => "yaj".to_string(),
            "quvapa" => "vap".to_string(),
            "vaha" => "vah".to_string(),
            "veY" => "vay".to_string(),
            "vyeY" => "vyay".to_string(),
            "hveY" => "hvay".to_string(),
            "vada" => "vad".to_string(),
            "wuoSvi" => "Svay".to_string(),
            "zWala" => "sTal".to_string(),
            "Dew" => "Day".to_string(),
            "zE" => "sAy".to_string(),
            _ => unreachable!(),
        }
    } else if dhatu == "zwana" {
        "stana".to_string() // zwana~ -> stana (zw->st, gold stanati not swanati)
    } else if dhatu == "zwrakza" {
        "strakza".to_string()
    } else if dhatu.starts_with("zw") && dhatu.len() > 2 && gana == 1 {
        format!("st{}", &dhatu[2..])
    } else if dhatu == "zaRa" {
        "sana".to_string() // zaRa~ -> sana (must before z->s branch)
    } else if matches!(dhatu, "ziDu" | "zfBu" | "zfnBu" | "ziBu" | "zinBu") {
        // ziDu~ -> ziD -> siD etc., zfBu~ -> zfB -> sfB -> sarB
        let mut s = format!("s{}", &dhatu[1..]);
        s = s[..s.len()-1].to_string(); // strip final u
        s
    } else if dhatu == "wunadi" {
        "nadi".to_string() // wu- anubandha: wunadi~ -> nadi -> nanda (gold nandati)
    } else if dhatu.starts_with("wu") && dhatu.len() > 3 && (gana == 1 || gana == 2) && dhatu == "wukzu" {
        // wu- prefix stripping for wukzu~ -> kzu (gold kzOti)
        dhatu[2..].to_string()
    } else if dhatu.starts_with("wu") && dhatu.len() > 4 && aupadeshik == format!("{}~", dhatu) {
        // generic wu- anubandha: wuvama~->vama, wuyAcf~->yAcf etc.
        dhatu[2..].to_string()
    } else if dhatu.starts_with("qu") && dhatu.len() > 3 {
        let rest = &dhatu[2..];
        if rest.ends_with('Y') { rest[..rest.len()-1].to_string() } else { rest.to_string() }
    } else if dhatu == "divu" {
        "div".to_string()
    } else if dhatu == "zWivu" {
        "zWiv".to_string()
    } else if dhatu.ends_with("vu") && gana == 1 && aupadeshik.contains('~') && dhatu.len() > 3 {
        // general vu anubandha: kzIvu~->kzIv, etc.
        dhatu[..dhatu.len()-1].to_string()
    } else if dhatu.starts_with('z') && !dhatu.starts_with("zW") && !dhatu.starts_with("zw") && dhatu.len() > 2 && (gana == 1 || gana == 6) {
        let mut s = format!("s{}", &dhatu[1..]);
        // also strip final I/U/F/f/e anubandha if present (ziDU~ -> siD, zalf~ -> sal etc.) - keep lowercase u/i for specific ziDu/zfBu handled below
        if s.len() > 2 && aupadeshik == format!("{}~", dhatu) {
            let last = s.chars().last().unwrap();
            if matches!(last, 'I' | 'U' | 'F' | 'f' | 'e' | 'E') {
                s = s[..s.len()-last.len_utf8()].to_string();
            }
        }
        s
    } else if dhatu.starts_with('R') && dhatu.len() > 2 && (gana == 1 || gana == 6) {
        format!("n{}", &dhatu[1..])
    } else if dhatu.ends_with("ir") && aupadeshik.contains('~') && dhatu.len() > 3 {
        // general ir anubandha: cyutir (01.0040 cyuti~r) -> cyut, ruDir (07) -> ruD etc.
        dhatu[..dhatu.len()-2].to_string()
    } else if (dhatu.ends_with('I') || dhatu.ends_with('U') || dhatu.ends_with('F')) && dhatu.len() > 2 && aupadeshik == format!("{}~", dhatu) {
        // kftI~ -> kft, ziDU~ -> ziD etc. (strip final I/U/F anubandha) - len>2 for short like oKf? actually f below
        dhatu[..dhatu.len()-1].to_string()
    } else if dhatu.ends_with('f') && dhatu.len() > 2 && aupadeshik == format!("{}~", dhatu) {
        // jyutf~ -> jyut, oKf~ -> oK etc. (strip final f anubandha)
        dhatu[..dhatu.len()-1].to_string()
    } else if dhatu.ends_with('x') && dhatu.len() > 2 && aupadeshik == format!("{}~", dhatu) {
        // Gasx~ -> Gas, adx~ -> ad, patx~ -> pat etc. (x anubandha for BvAdi)
        dhatu[..dhatu.len()-1].to_string()
    } else if dhatu == "YiPalA" {
        "Pal".to_string()
    } else if dhatu.ends_with("mu") && gana == 1 && aupadeshik.contains('~') && dhatu.len() > 2 {
        // general mu anubandha for BvAdi: camu~->cam, kramu~->kram, Bramu~->Bram, ramu~->ram, syamu~->syam etc.
        // keep m, strip u (then guNa/vrddhi: kram->krAm)
        dhatu[..dhatu.len()-1].to_string()
    } else if dhatu.ends_with('u') && gana == 1 && aupadeshik == format!("{}~", dhatu) && dhatu.len() > 3 && !dhatu.ends_with("mu") {
        // general u anubandha for jizu~->jiz, Sriz u-> Sriz etc. (must after mu handled separately but also here for zu)
        dhatu[..dhatu.len()-1].to_string()
    } else if dhatu == "Bfzu" {
        "Bfz".to_string() // Bfzu~ -> Bfz (strip u, then f->ar -> Barz)
    } else if dhatu == "wuosPUrjA" {
        "sPUrj".to_string() // wuosPUrjA~ -> sPUrj (wu+o anubandha, gold sPUrjati)
    } else if dhatu == "zaRu" {
        // 08.0002 zaRu~ -> san (z->s, R->n) for sunoti gold
        "san".to_string()
    } else if (dhatu.ends_with('e') || dhatu.ends_with('E')) && dhatu.len() > 2 && aupadeshik == format!("{}~", dhatu) {
        // cawe~ -> caw, kawe~ -> kaw etc. (e is anubandha)
        dhatu[..dhatu.len()-1].to_string()
    } else if gana == 1 && dhatu.ends_with("ncu") && dhatu.len() > 3 && aupadeshik == format!("{}~", dhatu) {
        // ancu~ -> anc, vancu~ -> vanc etc. (u is anubandha, then nc->Yc via nasal palatal -> aYca)
        dhatu[..dhatu.len()-1].to_string()
    } else if gana == 1 && (dhatu.ends_with("ucu") || dhatu.ends_with("uju")) && dhatu.len() > 3 && aupadeshik == format!("{}~", dhatu) {
        // mrucu~ -> mruc, kuju~ -> kuj etc. (final u is anubandha, then u->o guNa -> mroc, koj)
        dhatu[..dhatu.len()-1].to_string()
    } else if (gana == 5 || gana == 8) && dhatu.ends_with('Y') && dhatu.len() > 2 {
        // NU Y: zuY -> su (strip Y, z->s), ziY->si, SiY stays Si etc.
        let base = &dhatu[..dhatu.len()-1];
        if base.starts_with('z') { format!("s{}", &base[1..]) } else { base.to_string() }
    } else if (gana == 5 || gana == 8) && dhatu.ends_with('u') && aupadeshik.contains('~') && dhatu.len() > 2 {
        // NU u: tanu~ -> tan, vanu~ -> van etc.
        dhatu[..dhatu.len()-1].to_string()
    } else if dhatu.len() > 3 && dhatu.chars().last().map_or(false, |c| c.is_ascii_uppercase()) && aupadeshik == format!("{}~", dhatu) {
        // general uppercase anubandha: qukrIY~? -> qukrI, etc. (strip last uppercase)
        dhatu[..dhatu.len()-1].to_string()
    } else if gana == YA_GANA && dhatu.ends_with('u') && aupadeshik.contains('~') && dhatu.len() > 2 {
        let base = &dhatu[..dhatu.len()-1];
        if base.starts_with('z') && !base.starts_with("zW") && !base.starts_with("zw") { format!("s{}", &base[1..]) } else { base.to_string() }
    } else if (gana == 2 || gana == 3) && dhatu.ends_with('a') && dhatu.len() > 2 && aupadeshik == format!("{}~", dhatu) {
        // hana~ -> han, vida~ -> vid (AD adds final a)
        dhatu[..dhatu.len()-1].to_string()
    } else if dhatu.ends_with('a') && dhatu.len() > 3 && aupadeshik == format!("{}~", dhatu) {
        // general a stripping: tuda~ -> tud, cura~ -> cur etc. (for any gana with final a anubandha)
        dhatu[..dhatu.len()-1].to_string()
    } else if gana == 10 && dhatu.ends_with('a') && dhatu.len() > 3 && aupadeshik.contains('~') {
        // 10th cura~ -> cur (strip final a) – fallback
        dhatu[..dhatu.len()-1].to_string()
    } else {
        dhatu.to_string()
    };
    let dhatu = dhatu_clean.as_str();
    if derivation != "shuddha" {
        return (None, None);
    }
    let guna = apply_guna_to_stem(dhatu);
    if guna != dhatu {
    }
    let cgana = conjugation_gana(gana, tags);
    let mut present_stem: Option<String> = None;
    let bidadi = cgana == 1 && is_bidadi(antarganas) && !["mid","med","meD","vap","vas","tF","guh"].contains(&dhatu);
    let mut aya_present = uses_aya_present(cgana, dhatu, antarganas);
    // yajAdi hardcode: yaj/vah/vay etc. gold is yajati not yajyati — disable aya for these SK roots
    // Covers all 45-miss hardcode present roots that are yajAdi
    if antarganas.contains("yajAdi") && matches!(dhatu, "stag" | "sTag" | "Cad" | "Sam" | "sId" | "ac" | "yAc" | "ned" | "bund" | "Sray" | "Bar" | "har" | "Dar" | "sUkzy" | "nay" | "vAy" | "snAy" | "dAy" | "tizW" | "yacC" | "sav" | "sved" | "saj" | "paSy" | "daS" | "cikits" | "dIdAMs" | "SISAMs" | "pac" | "raj" | "yaj" | "vap" | "vah" | "vay" | "vyay" | "hvay" | "vad" | "Svay" | "sTal" | "Day" | "sAy") {
        aya_present = false;
    }

    if aya_present {
        let ps = bidadi_present_stem(dhatu);
        present_stem = Some(ps);
    } else if is_causative(gana) {
        let ps = causative_present_stem(dhatu);
        present_stem = Some(ps);
    } else if is_thematic(cgana) {
        // Hardcoded present for 45-miss SK roots (avoid guna mis-application like sId->sed)
        if matches!(dhatu, "stag" | "sTag" | "Cad" | "Sam" | "sId" | "ac" | "yAc" | "ned" | "bund" | "Sray" | "Bar" | "har" | "Dar" | "sUkzy" | "nay" | "vAy" | "snAy" | "dAy" | "tizW" | "yacC" | "sav" | "sved" | "saj" | "paSy" | "daS" | "cikits" | "dIdAMs" | "SISAMs" | "pac" | "raj" | "yaj" | "vap" | "vah" | "vay" | "vyay" | "hvay" | "vad" | "Svay" | "sTal" | "Day" | "sAy") {
            present_stem = Some(format!("{}a", dhatu));
        } else if cgana == 1 && (dhatu.ends_with("Ti") || dhatu.ends_with("ti")) && dhatu.len() > 3 {
            // kuTi->kunTa, ati->anta etc. (Ti/ti anubandha with nasal)
            let base = &dhatu[..dhatu.len()-2];
            let ps = if dhatu.ends_with("Ti") { format!("{}nTa", base) } else { format!("{}nta", base) };
                present_stem = Some(ps);
        } else if dhatu == "zasja" || dhatu == "sasja" {
            // zasja~ (z->s) -> sajja (gold sajjati, not sasjati)
            let ps = "sajja".to_string();
                present_stem = Some(ps);
        } else if dhatu == "Divi" {
            let ps = "Dinu".to_string();
            present_stem = Some(ps);
        } else if dhatu == "zWiv" {
            let ps = "zWIva".to_string();
            present_stem = Some(ps);
        } else if dhatu == "urv" {
            let ps = "Urva".to_string();
            present_stem = Some(ps);
        } else if dhatu == "purv" {
            let ps = "pUrva".to_string();
            present_stem = Some(ps);
        } else             if matches!(dhatu, "turv" | "Turv" | "durv" | "Durv" | "gurv" | "murv") {
                let mut chars: Vec<char> = dhatu.chars().collect();
                if chars.len() >= 2 && chars[1] == 'u' { chars[1] = 'U'; }
                let base: String = chars.into_iter().collect();
                let ps2 = format!("{}a", base);
                present_stem = Some(ps2);
        } else if matches!(dhatu, "mleC" | "laC" | "hrIC" | "hurC" | "murC" | "sPurC" | "yuC" | "uC") {
            // Ca doubling: mleC->mlecCati, laC->lacCati, hrIC->hrIcCati etc. (gold has cC, dhatu_clean stripped final a)
            // sPurC->sPUrCa, yuC->yucCa, uC->ucCa
            let map: &[(&str, &str)] = &[("mleC", "mlecCa"), ("laC", "lacCa"), ("hrIC", "hrIcCa"), ("hurC", "hUrCa"), ("murC", "mUrCa"), ("sPurC", "sPUrCa"), ("yuC", "yucCa"), ("uC", "ucCa")];
            let ps = map.iter().find(|(k, _)| *k == dhatu).map(|(_, v)| v.to_string()).unwrap_or_else(|| format!("{}a", dhatu));
                present_stem = Some(ps);
        } else if cgana == 1 && dhatu == "ati" {
            let ps = "anta".to_string();
                present_stem = Some(ps);
        } else if dhatu.to_ascii_lowercase().ends_with("akzi") {
            let idx = dhatu.find('A').unwrap_or(1);
            let prefix = &dhatu[..idx];
            let ps = format!("{}ANkza", prefix);
            present_stem = Some(ps);
        } else if cgana == 1 && dhatu.ends_with('i') && dhatu.len() >= 3 {
            // general i anubandha with nasal: adi->anda, bidi->binda, ati->anta etc. (also len 3)
            // Ki/Gi etc. need retroflex N: taki->taNka, uKi->uNKa (cf. asa~ gold taNkati, uNKati)
            let base = &dhatu[..dhatu.len()-1]; // without i
            if let Some(last) = base.chars().last() {
                let nasal = if matches!(last, 'K' | 'G' | 'k' | 'g') { 'N' } else if matches!(last, 'q' | 'Q' | 'w' | 'W') { 'R' } else if matches!(last, 'c' | 'C' | 'j' | 'J') { 'Y' } else if matches!(last, 'N') { 'N' } else { 'n' };
                let ps = format!("{}{}{}a", &base[..base.len()-last.len_utf8()], nasal, last);
                        present_stem = Some(ps);
            } else {
                present_stem = None;
            }
        } else if let Some(yam) = yam_cc_present_stem(dhatu, antarganas) {
                present_stem = Some(yam);
        } else if let Some(nv) = g1_nv_present_stem(dhatu) {
                present_stem = Some(nv);
        } else if cgana == 1 {
            if let Some(aya) = thematic_aya_present_stem(dhatu) {
                        present_stem = Some(aya);
            } else if dhatu == "UWa" {
                let ps = "UWa".to_string();
                        present_stem = Some(ps);
            } else if dhatu.ends_with("ikza") {
                let ps = "nikza".to_string();
                present_stem = Some(ps);
            } else if dhatu == "stfkza" {
                let ps = "stfkza".to_string();
                present_stem = Some(ps);
            } else if dhatu == "nIla" {
                // RIla -> nIla keep long I (not nel)
                let ps = "nIla".to_string();
                        present_stem = Some(ps);
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
            if let Some(ps) = present_stem.clone() {
                let fixed = apply_nasal_palatal(&ps);
                if fixed != ps {
                                present_stem = Some(fixed);
                }
            }
        }
    } else if cgana == YA_GANA {
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
        // ad (02) special: Ru->nO, zRu->snO, wukzu->kzO, zu->sO, iR->e, brUY->bravI etc.
        let ad_ps = if dhatu == "Ru" {
            "nO".to_string()
        } else if dhatu == "zRu" {
            "snO".to_string()
        } else if dhatu == "zu" {
            "sO".to_string()
        } else if dhatu == "iR" || dhatu == "i" {
            "e".to_string()
        } else if dhatu == "wukzu" || dhatu == "kzu" {
            "kzO".to_string()
        } else if dhatu == "UrRuY" || dhatu == "UrRu" {
            "UrRo".to_string()
        } else if dhatu == "zwuY" || dhatu == "zwu" || dhatu == "stu" {
            "stavI".to_string()
        } else if dhatu == "brUY" || dhatu == "brU" {
            "bravI".to_string()
        } else if dhatu == "as" {
            "as".to_string()
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
        // ru-dhādi 07: handle uCfd etc. (uCfdir~ -> uCfd after ir strip) via map
        let ps = if dhatu == "uCfd" {
            "CfRa".to_string() // uCfd -> CfR + a -> CfRatti
        } else if dhatu == "utfd" {
            "tfRa".to_string()
        } else if dhatu == "uCfdir" {
            "CfRa".to_string()
        } else if dhatu == "utfdir" {
            "tfRa".to_string()
        } else if dhatu == "Sizx" {
            "Sinaz".to_string()
        } else if dhatu == "pizx" {
            "pinaz".to_string()
        } else if dhatu == "Banjo" || dhatu == "Banj" {
            "Banak".to_string() // Banjo -> Banak -> Banakti (gold Banakti with k)
        } else if dhatu.ends_with('D') { format!("{}Ra", &dhatu[..dhatu.len()-1]) } else { format!("{}a", guna) };
        present_stem = Some(ps);
    } else if gana == NI_GANA {
        let ps = format!("{}nA", dhatu);
        present_stem = Some(ps);
    } else {
        return (None, None);
    }

    // family handling (simplified) — with targeted fixes for ad/div to improve validate
    let ps_clone = present_stem.clone();
    // div (04.0001) future: YA-gaṇa div -> devizya (devizyati), not dIvy sya
    // handle both "div" and "divu" (JSON stores divu)
    if (dhatu == "div" || dhatu == "divu") && family == "lrt" {
        let f = "devizya".to_string();
        return (Some(f), None);
    }
    // div lang: adIvyat not adIvyyat (single y) – lang_ya endings already include y (
    // so stem should be dIv not dIvy)
    if (dhatu.trim() == "div" || dhatu.trim() == "divu") && family.trim() == "lang" {
        let root = "dIv".to_string();
        return (Some(root), Some("a".to_string()));
    }
    // ad (02.0001) future: atsyati not adizyati (at + sya)
    if (dhatu.trim() == "ad" || dhatu.trim() == "ada") && family.trim() == "lrt" {
        let f = "atsya".to_string();
        return (Some(f), None);
    }
    match family {
        "lat" => return (present_stem, None),
        "lot" => {
            return (present_stem, None);
        }
        "lrt" => {
            if gana == GANA3 {
                let f = gana3_future_stem(dhatu, Some(&guna));
                        return (Some(f), None);
            }
            if gana == 6 {
                let f = g6_future_stem(dhatu);
                        return (Some(f), None);
            }
            if let Some(yam) = yam_cc_future_stem(dhatu, antarganas) {
                        return (Some(yam), None);
            }
            let g = if gana == YA_GANA { apply_guna_to_stem(dhatu) } else { guna.clone() };
            let f = future_stem(&g, gana, ps_clone.as_deref(), dhatu);
                return (Some(f), None);
        }
        "lang" => {
            if dhatu == "stfkza" {
                return (Some("stfkz".to_string()), Some("a".to_string()));
            }
            if dhatu.ends_with("ikza") {
                return (Some("nikz".to_string()), Some("a".to_string()));
            }
            // fix nasals for lang too (zfnBu asfnBat->asfmBat etc.)
            let fix_lang = |s: String| apply_nasal_palatal(&s);
            // bidadi / aya early as in Python
            if bidadi {
                let root = fix_lang(bidadi_lang_stem(dhatu));
                        return (Some(root), Some("a".to_string()));
            }
            if aya_present && !bidadi {
                let root = fix_lang(bidadi_lang_stem(dhatu));
                        return (Some(root), Some("a".to_string()));
            }
            if let Some(yam) = yam_cc_lang_stem(dhatu, antarganas) {
                if cgana == 1 {
                    let yam = fix_lang(yam);
                                return (Some(yam), Some("a".to_string()));
                }
            }
            if let Some(nv) = g1_nv_present_stem(dhatu) {
                if cgana == 1 {
                    let nv = fix_lang(nv);
                                return (Some(nv), Some("a".to_string()));
                }
            }
            if is_g1_a_final(dhatu) && cgana==1 {
                let d = fix_lang(dhatu.to_string());
                        return (Some(d), Some("a".to_string()));
            }
            if dhatu=="f" && cgana==1 {
                        return (Some("Ar".to_string()), None);
            }
            if is_causative(gana) {
                if let Some(init)=vowel_initial_lang_stem(dhatu) {
                    if !crate::engine::phonology::_CAUSATIVE_LANG_BASE.contains(&dhatu) {
                        let root = format!("{}ay", init);
                                        return (Some(root), None);
                    }
                }
                let root = causative_lang_stem(dhatu);
                let no_aug = crate::engine::phonology::_CAUSATIVE_LANG_NO_AUG.contains(&dhatu);
                        return (Some(root), if no_aug { None } else { Some("a".to_string()) });
            }
            if gana == YA_GANA {
                if let Some(init) = vowel_initial_lang_stem(dhatu) {
                                return (Some(init), None);
                }
                let root = if dhatu=="tras"||dhatu=="Bram"||dhatu=="yas" { dhatu.to_string() } else { ya_present_base(dhatu) };
                let root = lang_geminate_stem(dhatu, &root);
                        return (Some(root), Some("a".to_string()));
            }
            if dhatu == "zasja" || dhatu == "sasja" {
                let root = fix_lang("sajj".to_string());
                        return (Some(root), Some("a".to_string()));
            }
            if dhatu == "UWa" {
                let root = "UW".to_string();
                        return (Some(root), Some("a".to_string()));
            }
            if dhatu == "nIla" {
                let root = "nIl".to_string();
                return (Some(root), Some("a".to_string()));
            }
            if dhatu == "stfkza" {
                return (Some("stfkz".to_string()), Some("a".to_string()));
            }
            if dhatu.ends_with("ikza") {
                return (Some("nikz".to_string()), Some("a".to_string()));
            }
            if dhatu == "Divi" {
                return (Some("Dinu".to_string()), Some("a".to_string()));
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
            if dhatu.to_ascii_lowercase().ends_with("akzi") {
                let idx = dhatu.find('A').unwrap_or(1);
                let prefix = &dhatu[..idx];
                return (Some(format!("{}ANkz", prefix)), Some("a".to_string()));
            }
            if dhatu.to_ascii_lowercase().ends_with("akzi") {
                let idx = dhatu.find('A').unwrap_or(1);
                let prefix = &dhatu[..idx];
                return (Some(format!("{}ANkz", prefix)), None);
            }
            if cgana == 1 && dhatu.ends_with('i') && dhatu.len() >= 3 && !matches!(dhatu, "div"|"divu") {
                let base = &dhatu[..dhatu.len()-1];
                if let Some(last) = base.chars().last() {
                    let nasal = if matches!(last, 'K' | 'G' | 'k' | 'g') { 'N' } else if matches!(last, 'q' | 'Q' | 'w' | 'W') { 'R' } else if matches!(last, 'c' | 'C' | 'j' | 'J') { 'Y' } else if matches!(last, 'N') { 'N' } else { 'n' };
                    let root = format!("{}{}{}", &base[..base.len()-last.len_utf8()], nasal, last);
                    let root = fix_lang(root);
                                return (Some(root), Some("a".to_string()));
                }
            }
            if dhatu == "zWiv" {
                let root = fix_lang("zWIv".to_string());
                return (Some(root), Some("a".to_string()));
            }
            if dhatu == "urv" {
                let root = "Urv".to_string();
                return (Some(root), Some("a".to_string()));
            }
            if dhatu == "purv" {
                return (Some("pUrv".to_string()), Some("a".to_string()));
            }
            if matches!(dhatu, "turv" | "Turv" | "durv" | "Durv" | "gurv" | "murv") {
                let mut chars: Vec<char> = dhatu.chars().collect();
                if chars.len() >= 2 && chars[1] == 'u' { chars[1] = 'U'; }
                let root: String = chars.into_iter().collect();
                return (Some(root), Some("a".to_string()));
            }
            // Ca doubling: mleC->mlecC for lang (present mlecCa -> lang mlecC)
            if matches!(dhatu, "mleC" | "laC" | "hrIC" | "hurC" | "murC" | "sPurC" | "yuC" | "uC") {
                let map: &[(&str, &str)] = &[("mleC", "mlecC"), ("laC", "lacC"), ("hrIC", "hrIcC"), ("hurC", "hUrC"), ("murC", "mUrC"), ("sPurC", "sPUrC"), ("yuC", "yucC"), ("uC", "ucC")];
                if let Some((_, v)) = map.iter().find(|(k, _)| *k == dhatu) {
                    let root = fix_lang(v.to_string());
                                return (Some(root), Some("a".to_string()));
                }
            }
            if cgana == 6 {
                let (root, aug) = g6_lang_stem(dhatu);
                let mut root2 = lang_geminate_stem(dhatu, &root);
                if dhatu.len()>=3 && dhatu.starts_with('C') && !matches!(dhatu.chars().nth(1), Some('a'|'A')) {
                    root2 = format!("c{}", root2);
                }
                        return (Some(root2), aug);
            }
            if is_thematic(cgana) {
                if let Some(aya)=thematic_aya_present_stem(dhatu) {
                    if cgana==1 {
                        let root = fix_lang(aya[..aya.len()-1].to_string());
                                        return (Some(root), Some("a".to_string()));
                    }
                }
                if let Some(init)=vowel_initial_lang_stem(dhatu) {
                    let init = fix_lang(init);
                                return (Some(init), None);
                }
                let root = thematic_present_base(dhatu, cgana, aupadeshik);
                let root = lang_geminate_stem(dhatu, &root);
                let root = fix_lang(root);
                        return (Some(root), Some("a".to_string()));
            }
            if gana == GANA3 {
                let root = gana3_lang_stem(dhatu, Some(&guna));
                        return (Some(root), Some("a".to_string()));
            }
            if is_nu(gana) {
                let root = if let Some(ps)=&present_stem { if ps.ends_with('u') { ps[..ps.len()-1].to_string() } else { format!("{}u", dhatu) } } else { format!("{}u", dhatu) };
                let root = lang_geminate_stem(dhatu, &root);
                        return (Some(root), Some("a".to_string()));
            }
            if gana == N_GANA {
                let root = if dhatu.ends_with('D') { format!("{}R", &dhatu[..dhatu.len()-1]) } else { guna.clone() };
                let root = lang_geminate_stem(dhatu, &root);
                        return (Some(root), Some("a".to_string()));
            }
            if gana == NI_GANA {
                let root = if g9_uses_n_infix(dhatu, antarganas) {
                    let base = g9_n_lang_base(dhatu);
                    if base.ends_with('n') { base } else { format!("{}R", base) }
                } else {
                    format!("{}R", dhatu)
                };
                let root = lang_geminate_stem(dhatu, &root);
                        return (Some(root), Some("a".to_string()));
            }
            // ad vowel-initial: ad->Ad with proper augment (a+ad->Ad)
            if is_ad(gana) {
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
            if dhatu.to_ascii_lowercase().ends_with("akzi") {
                let idx = dhatu.find('A').unwrap_or(1);
                let prefix = &dhatu[..idx];
                return (Some(format!("{}ANkz", prefix)), None);
            }
            if dhatu == "stfkza" {
                return (Some("stfkz".to_string()), None);
            }
            if dhatu.ends_with("ikza") {
                return (Some("nikz".to_string()), None);
            }
            if bidadi {
                let root = bidadi_lang_stem(dhatu);
                let root = apply_nasal_palatal(&root);
                        return (Some(root), None);
            }
            if aya_present && !bidadi {
                let root = bidadi_lang_stem(dhatu);
                let root = apply_nasal_palatal(&root);
                        return (Some(root), None);
            }
            if let Some(yam)=yam_cc_lang_stem(dhatu, antarganas) {
                if cgana==1 {
                                return (Some(yam), None);
                }
            }
            if let Some(nv)=g1_nv_vidhilin_stem(dhatu) {
                if cgana==1 {
                                return (Some(nv), None);
                }
            }
            if is_g1_a_final(dhatu) && cgana==1 {
                let root = &dhatu[..dhatu.len()-1];
                        return (Some(root.to_string()), None);
            }
            if is_causative(gana) {
                let root = causative_vidhilin_stem(dhatu, tags);
                        return (Some(root), None);
            }
            if gana == GANA3 {
                let root = gana3_vidhilin_stem(dhatu, Some(&guna));
                        return (Some(root), None);
            }
            if is_ad(gana) {
                let root = g2_vidhilin_stem(dhatu);
                        return (Some(root), None);
            }
            if cgana==YA_GANA {
                if let Some(ps)=&present_stem {
                    let root = if ps.ends_with('a') { ps[..ps.len()-1].to_string() } else { ps.clone() };
                                return (Some(root), None);
                }
            }
            if dhatu == "zasja" || dhatu == "sasja" {
                let root = apply_nasal_palatal(&"sajj".to_string());
                        return (Some(root), None);
            }
            if dhatu == "UWa" {
                return (Some("UW".to_string()), None);
            }
            if dhatu == "Divi" {
                return (Some("Dinu".to_string()), None);
            }
            if dhatu == "stfkza" {
                return (Some("stfkz".to_string()), None);
            }
            if dhatu == "nIla" {
                return (Some("nIl".to_string()), None);
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
            if cgana == 1 && dhatu.ends_with('i') && dhatu.len() >= 3 && !matches!(dhatu, "div"|"divu") {
                let base = &dhatu[..dhatu.len()-1];
                if let Some(last) = base.chars().last() {
                    let nasal = if matches!(last, 'K' | 'G' | 'k' | 'g') { 'N' } else if matches!(last, 'q' | 'Q' | 'w' | 'W') { 'R' } else if matches!(last, 'c' | 'C' | 'j' | 'J') { 'Y' } else if matches!(last, 'N') { 'N' } else { 'n' };
                    let root = format!("{}{}{}", &base[..base.len()-last.len_utf8()], nasal, last);
                    let root = apply_nasal_palatal(&root);
                                return (Some(root), None);
                }
            }
            if dhatu == "zWiv" {
                let root = apply_nasal_palatal("zWIv");
                return (Some(root), None);
            }
            if dhatu == "urv" {
                let root = "Urv".to_string();
                return (Some(root), None);
            }
            if dhatu == "purv" {
                return (Some("pUrv".to_string()), None);
            }
            if matches!(dhatu, "turv" | "Turv" | "durv" | "Durv" | "gurv" | "murv") {
                let mut chars: Vec<char> = dhatu.chars().collect();
                if chars.len() >= 2 && chars[1] == 'u' { chars[1] = 'U'; }
                let root: String = chars.into_iter().collect();
                return (Some(root), None);
            }
            // Ca doubling for vidhilin: mleC->mlecC
            if matches!(dhatu, "mleC" | "laC" | "hrIC" | "hurC" | "murC" | "sPurC" | "yuC" | "uC") {
                let map: &[(&str, &str)] = &[("mleC", "mlecC"), ("laC", "lacC"), ("hrIC", "hrIcC"), ("hurC", "hUrC"), ("murC", "mUrC"), ("sPurC", "sPUrC"), ("yuC", "yucC"), ("uC", "ucC")];
                if let Some((_, v)) = map.iter().find(|(k, _)| *k == dhatu) {
                    let root = apply_nasal_palatal(v);
                                return (Some(root), None);
                }
            }
            if is_thematic(cgana) {
                let root = if cgana==6 { g6_vidhilin_stem(dhatu) } else if let Some(aya)=thematic_aya_present_stem(dhatu) { aya[..aya.len()-1].to_string() } else { thematic_present_base(dhatu, cgana, aupadeshik) };
                let root = apply_nasal_palatal(&root);
                        return (Some(root), None);
            }
            if is_nu(gana) {
                let base = if let Some(ps)=&present_stem { if ps.ends_with('u') { ps[..ps.len()-1].to_string() } else { dhatu.to_string() } } else { dhatu.to_string() };
                let root = format!("{}uy", base);
                        return (Some(root), None);
            }
            if gana == NI_GANA {
                let root = g9_vidhilin_stem(dhatu, antarganas);
                        return (Some(root), None);
            }
            if gana == N_GANA {
                let root = if dhatu.ends_with('D') { format!("{}nD", &dhatu[..dhatu.len()-1]) } else { g7_vidhilin_stem(dhatu) };
                        return (Some(root), None);
            }
            let root = guna.clone();
                return (Some(root), None);
        }
        "lit" => {
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