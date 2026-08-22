//! Simplified port of sktmorph/engine/stems.py
//! Covers shuddha/kartari, gan 1-10 thematic core. Full 703 LOC will be expanded iteratively.
//! This file already handles lat/lot/lang/vidhilin/lrt/lit for gana 1,4,6 correctly and stubs others.

use crate::engine::phonology::*;
use crate::engine::redup::*;
use crate::engine::steps::EngineStep;

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
    if graded.ends_with('z') { return format!("{}kzya", &graded[..graded.len()-1]); }
    if graded.ends_with("fh") { return format!("{}izya", graded); }
    graded.to_string() + "izya"
}

pub fn g6_future_stem(dhatu: &str) -> String {
    if dhatu == "kzi" { return apply_guna_to_stem(dhatu) + "zya"; }
    if dhatu == "SuB" { return apply_guna_to_stem(dhatu) + "izya"; }
    if dhatu == "majj" { return "maNkzy".to_string(); }
    if dhatu.ends_with("ajj") { return format!("{}arkzya", &dhatu[..1]); }
    if dhatu.ends_with('U') { return format!("{}uvizya", &dhatu[..dhatu.len()-1]); }
    let graded = apply_guna_to_stem(dhatu);
    if graded != dhatu {
        // simplified: always use suffix
        return g6_future_suffix(&graded);
    }
    g6_future_suffix(dhatu)
}

pub fn future_stem(guna: &str, gana: u8, present_stem: Option<&str>, dhatu: &str) -> String {
    if dhatu == "kzi" { return apply_guna_to_stem(dhatu) + "zya"; }
    if gana == 2 && dhatu.ends_with('u') { return format!("{}vizya", &dhatu[..dhatu.len()-1]); }
    if let Some(ps) = present_stem {
        if ps.ends_with('a') {
            let base = &ps[..ps.len()-1];
            if gana == 1 || is_causative(gana) { return format!("{}izya", base); }
            return format!("{}sya", base);
        }
    }
    if guna.ends_with('v') { return format!("{}izya", guna); }
    format!("{}izya", guna)
}

pub fn perfect_stem(dhatu: &str, guna: &str) -> String {
    let redupl = if dhatu.len() >= 2 && "kgcjwqtp".contains(dhatu.chars().next().unwrap()) {
        format!("{}a", dhatu.chars().next().unwrap())
    } else if guna.len() >= 2 {
        let c = guna.chars().nth(1).unwrap();
        if "aeiouAIUEO".contains(c) { guna[..2].to_string() } else { format!("{}a", guna.chars().next().unwrap()) }
    } else {
        format!("{}a", guna.chars().next().unwrap_or(dhatu.chars().next().unwrap()))
    };
    if guna.ends_with('v') && dhatu.ends_with('U') { return format!("ba{}", dhatu); }
    if guna.ends_with('v') { return format!("{}{}a", redupl, dhatu); }
    if guna.ends_with('a') { return format!("{}{}", redupl, guna); }
    format!("{}{}a", redupl, guna)
}

fn append_step(steps: &mut Vec<EngineStep>, form: &str, sutras: &[&str], kind: &str) {
    if steps.last().map_or(true, |s| s.form != form || s.kind != kind) {
        steps.push(EngineStep::new(form, sutras.to_vec(), kind));
    }
}

pub fn derive_stem(
    dhatu: &str,
    gana: u8,
    family: &str,
    derivation: &str,
    tags: &str,
    antarganas: &str,
    aupadeshik: &str,
) -> (Option<String>, Option<String>, Vec<EngineStep>) {
    let mut steps: Vec<EngineStep> = Vec::new();
    if derivation != "shuddha" {
        steps.push(EngineStep::new(dhatu, vec!["1.3.1"], "dhatu"));
        return (None, None, steps);
    }
    steps.push(EngineStep::new(dhatu, vec!["1.3.1"], "dhatu"));
    let guna = apply_guna_to_stem(dhatu);
    if guna != dhatu {
        append_step(&mut steps, &guna, &["7.2.115"], "guNa");
    }
    let cgana = conjugation_gana(gana, tags);
    let mut present_stem: Option<String> = None;
    let bidadi = cgana == 1 && is_bidadi(antarganas) && !["mid","med","meD","vap","vas","tF","guh"].contains(&dhatu);
    let aya_present = uses_aya_present(cgana, dhatu, antarganas);

    if aya_present {
        let ps = bidadi_present_stem(dhatu);
        append_step(&mut steps, &ps, &["3.1.33"], "yap");
        present_stem = Some(ps);
    } else if is_thematic(cgana) {
        if let Some(yam) = yam_cc_present_stem(dhatu, antarganas) {
            append_step(&mut steps, &yam, &["7.2.9"], "samprasaran");
            present_stem = Some(yam);
        } else if let Some(nv) = g1_nv_present_stem(dhatu) {
            append_step(&mut steps, &nv, &["7.3.84"], "nv_stem");
            present_stem = Some(nv);
        } else if cgana == 1 {
            if let Some(aya) = thematic_aya_present_stem(dhatu) {
                append_step(&mut steps, &aya, &["3.1.33"], "yap");
                present_stem = Some(aya);
            } else if is_g1_a_final(dhatu) {
                append_step(&mut steps, dhatu, &["3.1.68"], "sap");
                present_stem = Some(dhatu.to_string());
            } else {
                let base = if cgana == 6 { g6_plot_base(dhatu) } else { thematic_present_base(dhatu, cgana, aupadeshik) };
                if base != dhatu { append_step(&mut steps, &base, &["7.2.115"], "guNa"); }
                let ps = format!("{}a", base);
                append_step(&mut steps, &ps, &["3.1.68","3.1.69"], "sap");
                present_stem = Some(ps);
            }
        } else {
            // gana 6
            let base = g6_plot_base(dhatu);
            if base != dhatu { append_step(&mut steps, &base, &["7.2.115"], "guNa"); }
            let ps = format!("{}a", base);
            append_step(&mut steps, &ps, &["3.1.68","3.1.69"], "sap");
            present_stem = Some(ps);
        }
    } else if cgana == YA_GANA {
        if ["tras","Bram","yas"].contains(&dhatu) {
            let ps = format!("{}a", dhatu);
            append_step(&mut steps, &ps, &["3.1.68","3.1.69"], "sap");
            present_stem = Some(ps);
        } else {
            let base = ya_present_base(dhatu);
            let ps = format!("{}ya", base);
            append_step(&mut steps, &ps, &["3.1.33"], "yap");
            present_stem = Some(ps);
        }
    } else if gana == GANA3 {
        let ps = gana3_present_stem(dhatu, Some(&guna));
        append_step(&mut steps, &ps, &["6.1.1","3.1.3"], "redup");
        present_stem = Some(ps);
    } else if is_ad(gana) {
        append_step(&mut steps, &guna, &["3.1.3"], "ad");
        present_stem = Some(guna.clone());
    } else if is_nu(gana) {
        let ps = format!("{}{}", dhatu, if dhatu.ends_with('n') { "u" } else { "nu" });
        append_step(&mut steps, &ps, &["3.1.75"], "nu");
        present_stem = Some(ps);
    } else if gana == N_GANA {
        let ps = if dhatu.ends_with('D') { format!("{}Ra", &dhatu[..dhatu.len()-1]) } else { format!("{}a", guna) };
        append_step(&mut steps, &ps, &["7.3.88"], "n_gana");
        present_stem = Some(ps);
    } else if gana == NI_GANA {
        let ps = format!("{}nA", dhatu);
        append_step(&mut steps, &ps, &["3.1.81"], "nI");
        present_stem = Some(ps);
    } else if is_causative(gana) {
        let ps = causative_present_stem(dhatu);
        append_step(&mut steps, &ps, &["3.1.25"], "causal");
        present_stem = Some(ps);
    } else {
        return (None, None, steps);
    }

    // family handling (simplified)
    let ps_clone = present_stem.clone();
    match family {
        "lat" => return (present_stem, None, steps),
        "lot" => {
            if cgana == 6 {
                let root = g6_plot_base(dhatu);
                append_step(&mut steps, &root, &["3.2.69"], "plot_stem");
                return (Some(root), None, steps);
            }
            return (present_stem, None, steps);
        }
        "lrt" => {
            if gana == GANA3 {
                let f = gana3_future_stem(dhatu, Some(&guna));
                append_step(&mut steps, &f, &["3.2.135"], "lrt");
                return (Some(f), None, steps);
            }
            if gana == 6 {
                let f = g6_future_stem(dhatu);
                append_step(&mut steps, &f, &["3.2.135"], "lrt");
                return (Some(f), None, steps);
            }
            if let Some(yam) = yam_cc_future_stem(dhatu, antarganas) {
                append_step(&mut steps, &yam, &["3.2.135"], "lrt");
                return (Some(yam), None, steps);
            }
            let g = if gana == YA_GANA { apply_guna_to_stem(dhatu) } else { guna.clone() };
            let f = future_stem(&g, gana, ps_clone.as_deref(), dhatu);
            append_step(&mut steps, &f, &["3.2.135"], "lrt");
            return (Some(f), None, steps);
        }
        "lang" => {
            if let Some(yam) = yam_cc_lang_stem(dhatu, antarganas) {
                if cgana == 1 {
                    append_step(&mut steps, &yam, &["7.2.9"], "lang_stem");
                    return (Some(yam), Some("a".to_string()), steps);
                }
            }
            if gana == YA_GANA {
                if let Some(init) = vowel_initial_lang_stem(dhatu) {
                    append_step(&mut steps, &init, &["3.4.111"], "lang_stem");
                    return (Some(init), None, steps);
                }
            }
            if cgana == 6 {
                let (root, aug) = g6_lang_stem(dhatu);
                let root2 = lang_geminate_stem(dhatu, &root);
                append_step(&mut steps, &root2, &["3.4.111"], "lang_stem");
                return (Some(root2), aug, steps);
            }
            if is_causative(gana) {
                let root = causative_lang_stem(dhatu);
                append_step(&mut steps, &root, &["3.4.111"], "lang_stem");
                return (Some(root), Some("a".to_string()), steps);
            }
            // default lang: guna without 'a'
            let root = if is_ad(gana) { guna.clone() } else if let Some(ps) = &present_stem { if ps.ends_with('a') { ps[..ps.len()-1].to_string() } else { ps.clone() } } else { guna.clone() };
            let aug = if vowel_initial_lang_stem(dhatu).is_some() { None } else { Some("a".to_string()) };
            append_step(&mut steps, &root, &["3.4.111"], "lang_stem");
            return (Some(root), aug, steps);
        }
        "vidhilin" => {
            if is_causative(gana) {
                let root = causative_vidhilin_stem(dhatu, tags);
                append_step(&mut steps, &root, &["3.4.104"], "vidhilin_stem");
                return (Some(root), None, steps);
            }
            if gana == GANA3 {
                let root = gana3_vidhilin_stem(dhatu, Some(&guna));
                append_step(&mut steps, &root, &["3.4.104"], "vidhilin_stem");
                return (Some(root), None, steps);
            }
            let root = if is_ad(gana) { g2_vidhilin_stem(dhatu) } else if is_thematic(cgana) { thematic_present_base(dhatu, cgana, aupadeshik) } else { guna.clone() };
            append_step(&mut steps, &root, &["3.4.104"], "vidhilin_stem");
            return (Some(root), None, steps);
        }
        "lit" => {
            if gana == GANA3 {
                let ps = gana3_perfect_stem(dhatu, Some(&guna));
                append_step(&mut steps, &ps, &["6.1.1"], "lit");
                return (Some(ps), None, steps);
            }
            let grade = if is_thematic(cgana) && thematic_present_base(dhatu, cgana, aupadeshik) != dhatu { apply_guna_to_stem(dhatu) } else { guna.clone() };
            let ps = perfect_stem(dhatu, &grade);
            append_step(&mut steps, &ps, &["6.1.1"], "lit");
            return (Some(ps), None, steps);
        }
        _ => {}
    }
    (present_stem, None, steps)
}
