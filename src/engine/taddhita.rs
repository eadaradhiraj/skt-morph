//! तद्धित (minimal Kaumudī set): त्व, तल्, मतुप्, मयट्, इन्.
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaddhitaResult {
    pub forms: Vec<String>,
    pub pratipadika: String,
    pub pratyaya: String,
}

fn strip_visarga(s: &str) -> String {
    s.trim_end_matches('H').trim_end_matches('M').to_string()
}

fn a_stem_base(p: &str) -> String {
    let s = strip_visarga(p);
    if s.ends_with('a') {
        s[..s.len() - 1].to_string()
    } else {
        s
    }
}

pub fn derive(pratipadika: &str, pratyaya: &str) -> Vec<String> {
    let p = pratipadika.trim();
    if p.is_empty() {
        return vec![];
    }
    let s = strip_visarga(p);
    match pratyaya {
        "tva" => vec![format!("{s}tva")],
        "tal" | "tA" => vec![format!("{s}tA")],
        "matup" | "mat" => {
            if s.ends_with('a') {
                vec![format!("{}vat", &s[..s.len() - 1])]
            } else {
                vec![format!("{s}mat")]
            }
        }
        "mayaT" | "maya" => vec![format!("{s}maya")],
        "ini" | "in" => vec![format!("{}in", a_stem_base(p))],
        _ => vec![],
    }
}

pub fn generate(pratipadika: &str, pratyaya: &str) -> TaddhitaResult {
    TaddhitaResult {
        forms: derive(pratipadika, pratyaya),
        pratipadika: pratipadika.to_string(),
        pratyaya: pratyaya.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tva_tal_matup() {
        assert_eq!(derive("rAma", "tva"), vec!["rAmatva"]);
        assert_eq!(derive("rAmaH", "tal"), vec!["rAmatA"]);
        assert_eq!(derive("SrI", "matup"), vec!["SrImat"]);
        assert_eq!(derive("daRqa", "ini"), vec!["daRqin"]);
    }
}
