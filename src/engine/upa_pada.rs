//! Upasarga-conditioned pada (Pāṇini 1.3.12-77)
//! Determines which padas (Parasmaipadī / Ātmanepadī) are valid for a given dhatu + upasarga combo.
//!
//! Phase 1 covers the reporter cases:
//! - gam (P) bare -> P only, sam+gam / sam+A+gam -> P+A (1.3.29 vA)
//! - ji  (P) bare -> P only, vi/parA + ji -> A only (1.3.19 nitya), other upasarga -> P
//!
//! Design is declarative so adding further sūtras is just adding entries.

fn normalize_one(p: &str) -> String {
    let t = p.trim().trim_matches([',', ';']).trim();
    // handle SLP1 variants with anusvāra/ṅ etc
    match t {
        "saG" | "saM" | "saN" | "sam" | "SaM" | "SaG" | "SAM" | "sAM" => "sam".to_string(),
        "samA" | "saMA" => "samA".to_string(),
        "a" | "A" | "AG" | "AM" => "A".to_string(),
        "ud" | "ut" | "ul" => "ud".to_string(),
        "ni" | "nI" => "ni".to_string(),
        other => other.to_string(),
    }
}

fn normalized_prefixes(prefixes: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for raw in prefixes {
        // split compound entries like "sam;A" or "sam, A" or "sam A"
        for part in raw.split([',', ';', ' ']) {
            let t = part.trim();
            if t.is_empty() { continue; }
            out.push(normalize_one(t));
        }
        // also handle if raw itself is a direct variant not split correctly (already handled)
        // For a single token like "saG" we already pushed above. But if raw contained no delimiter
        // we push it anyway (the loop already did)
    }
    // De-duplicate while preserving order not needed
    out
}

fn has_prefix(norm: &[String], target: &str) -> bool {
    norm.iter().any(|p| p == target)
}

/// Compute allowed padas for this (root_pada, dhatu, prefixes) combo.
/// Empty `artha` does not fire meaning-conditioned 1.3 rules (38, 43, 66, 75).
pub fn allowed_padas(root_pada: &str, dhatu: &str, prefixes: &[String]) -> Vec<String> {
    allowed_padas_artha(root_pada, dhatu, prefixes, "")
}

pub fn allowed_padas_artha(root_pada: &str, dhatu: &str, prefixes: &[String], artha: &str) -> Vec<String> {
    let d = dhatu.trim();
    let norm = normalized_prefixes(prefixes);
    let has_sam = has_prefix(&norm, "sam");
    let a = artha.trim();
    let kram_sense = matches!(a, "vftti" | "sarga" | "tAyana");

    // --- gam ---
    // 1.3.29 sam + gam -> optional Ātmanepada (vA). Includes sam+A+gam (samAgam)
    // bare gam or A+gam alone -> P only
    if matches!(d, "gam" | "gamx" | "gamy") {
        match root_pada {
            "P" if has_sam => return vec!["P".to_string(), "A".to_string()],
            "P" => return vec!["P".to_string()],
            "A" => return vec!["A".to_string()],
            _ => {}
        }
    }

    // --- ji ---
    // 1.3.19 vi / parA + ji (1P "to win") -> nitya Ātmanepada
    //   vi+ji = vijayate (A only), parA+ji = parAjayate (A only)
    //   bare ji or other upasarga (abhi, sam, etc) -> P only (jayati)
    // Cross-check: gold (ashtadhyayi.com) has ji ting only plat; vi+ji should be alat only.
    if d == "ji" && root_pada == "P" {
        let has_vi = has_prefix(&norm, "vi");
        if norm.is_empty() {
            return vec!["P".to_string()];
        }
        if has_vi || has_prefix(&norm, "parA") {
            return vec!["A".to_string()];
        }
        return vec!["P".to_string()];
    }

    // --- sTA 1.3.22 समवप्रविभ्यः स्थः ---
    if matches!(d, "sTA" | "zWA")
        && (has_sam || has_prefix(&norm, "ava") || has_prefix(&norm, "pra") || has_prefix(&norm, "vi"))
    {
        return vec!["A".to_string()];
    }

    // --- 1.3.29 समो गम्यृच्छिप्रच्छिस्वरत्यर्तिश्रुविदिभ्यः ---
    if has_sam && matches!(d, "f" | "fcC" | "Sru" | "vida" | "vid" | "pracC") {
        return vec!["P".to_string(), "A".to_string()];
    }

    // --- 1.3.53 उदश्चरः ---
    if (d == "car" || d == "cara") && has_prefix(&norm, "ud") {
        return vec!["A".to_string()];
    }

    // --- 1.3.40 आङो दोऽनास्यविहरणे ---
    if (d == "dA" || d == "dAR" || d == "qudAY") && has_prefix(&norm, "A") {
        return vec!["A".to_string()];
    }

    // --- 1.3.66 भृञो यज्ञकर्मणि ---
    if matches!(d, "Bf" | "BfY" | "quBfY")
        && (has_sam || has_prefix(&norm, "ni") || has_prefix(&norm, "ud"))
        && a == "yajYa"
    {
        return vec!["A".to_string()];
    }

    // --- 1.3.18 परिव्यवेभ्यः क्रियः ---
    if matches!(d, "qukrIY" | "krI")
        && (has_prefix(&norm, "pari") || has_prefix(&norm, "vi") || has_prefix(&norm, "ava"))
    {
        return vec!["A".to_string()];
    }

    // --- 1.3.30 निसमुपविभ्यो ह्वः ---
    if matches!(d, "hveY" | "hve")
        && (has_prefix(&norm, "ni")
            || has_sam
            || has_prefix(&norm, "upa")
            || has_prefix(&norm, "vi"))
    {
        return vec!["A".to_string()];
    }

    // --- 1.3.38 वृत्तिसर्गतायनेषु क्रमः / 1.3.43 अनुपसर्गाद्वा ---
    if matches!(d, "kramu" | "kram" | "krama") && kram_sense {
        if has_prefix(&norm, "vi") || has_sam || has_prefix(&norm, "pari") {
            return vec!["A".to_string()];
        }
        if norm.is_empty() {
            return vec!["P".to_string(), "A".to_string()];
        }
    }

    // --- 1.3.75 समुदाङ्भ्यो यमोऽग्रन्थे ---
    if matches!(d, "yama" | "yam" | "yamx")
        && (has_sam || has_prefix(&norm, "ud") || has_prefix(&norm, "A"))
    {
        if a == "agranthe" {
            return vec!["A".to_string()];
        }
        if a == "granthe" {
            return vec!["P".to_string()];
        }
    }

    // Default: dhātupāṭha pada (1.3.12–77 exceptions are listed above).
    match root_pada {
        "P" => vec!["P".to_string()],
        "A" => vec!["A".to_string()],
        "U" => vec!["P".to_string(), "A".to_string()],
        _ => vec![root_pada.to_string()],
    }
}

pub fn pada_allowed(root_pada: &str, requested_pada: &str, dhatu: &str, prefixes: &[String]) -> bool {
    allowed_padas(root_pada, dhatu, prefixes).iter().any(|p| p == requested_pada)
}

pub fn pada_allowed_artha(
    root_pada: &str,
    requested_pada: &str,
    dhatu: &str,
    prefixes: &[String],
    artha: &str,
) -> bool {
    allowed_padas_artha(root_pada, dhatu, prefixes, artha)
        .iter()
        .any(|p| p == requested_pada)
}

#[cfg(test)]
#[allow(non_snake_case)] // SLP1 fixtures (saG = सङ्)
mod tests {
    use super::*;

    // ---------------------------------------------------------------------------
    // fn `s`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn s(v: &[&str]) -> Vec<String> { v.iter().map(|x| x.to_string()).collect() }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `gam_bare`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn gam_bare() {
        assert_eq!(allowed_padas("P","gam",&[]), vec!["P"]);
        assert!(pada_allowed("P","P","gam",&[]));
        assert!(!pada_allowed("P","A","gam",&[]));
    }
    #[test]
    // ---------------------------------------------------------------------------
    // fn `gam_sam`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn gam_sam() {
        let sam = s(&["sam"]);
        assert_eq!(allowed_padas("P","gam",&sam), vec!["P","A"]);
        assert!(pada_allowed("P","A","gam",&sam));
        let sam_a = s(&["sam","A"]);
        assert!(pada_allowed("P","A","gam",&sam_a));
        assert!(pada_allowed("P","P","gam",&sam_a));
        let saG = s(&["saG"]);
        assert!(pada_allowed("P","A","gam",&saG));
        let saG_A = s(&["saG","A"]);
        assert!(pada_allowed("P","A","gam",&saG_A));
    }
    #[test]
    // ---------------------------------------------------------------------------
    // fn `ji_bare`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn ji_bare() {
        assert_eq!(allowed_padas("P","ji",&[]), vec!["P"]);
        assert!(!pada_allowed("P","A","ji",&[]));
    }
    #[test]
    // ---------------------------------------------------------------------------
    // fn `ji_vi`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn ji_vi() {
        let vi = s(&["vi"]);
        assert_eq!(allowed_padas("P","ji",&vi), vec!["A"]);
        assert!(pada_allowed("P","A","ji",&vi));
        assert!(!pada_allowed("P","P","ji",&vi));
        let para = s(&["parA"]);
        assert_eq!(allowed_padas("P","ji",&para), vec!["A"]);
        let vi_para = s(&["vi","parA"]);
        assert!(pada_allowed("P","A","ji",&vi_para));
    }
    #[test]
    // ---------------------------------------------------------------------------
    // fn `ji_other_prefix_stays_p`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn ji_other_prefix_stays_p() {
        let abhi = s(&["abhi"]);
        assert_eq!(allowed_padas("P","ji",&abhi), vec!["P"]);
    }
    #[test]
    // ---------------------------------------------------------------------------
    // fn `stha_sam_is_a`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn stha_sam_is_a() {
        let sam = s(&["sam"]);
        assert_eq!(allowed_padas("P", "zWA", &sam), vec!["A"]);
    }
    #[test]
    // ---------------------------------------------------------------------------
    // fn `ud_car_is_a`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn ud_car_is_a() {
        let ud = s(&["ud"]);
        assert_eq!(allowed_padas("P", "car", &ud), vec!["A"]);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `ubhaya_bare_is_both`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn ubhaya_bare_is_both() {
        assert_eq!(allowed_padas("U", "qukrIY", &[]), vec!["P", "A"]);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `kri_pari_is_a`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn kri_pari_is_a() {
        let pari = s(&["pari"]);
        assert_eq!(allowed_padas("U", "qukrIY", &pari), vec!["A"]);
        assert!(!pada_allowed("U", "P", "qukrIY", &pari));
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `hve_ni_is_a`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn hve_ni_is_a() {
        let ni = s(&["ni"]);
        assert_eq!(allowed_padas("U", "hveY", &ni), vec!["A"]);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `kram_vi_needs_artha`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn kram_vi_needs_artha() {
        let vi = s(&["vi"]);
        assert_eq!(allowed_padas("P", "kramu", &vi), vec!["P"]);
        assert_eq!(allowed_padas_artha("P", "kramu", &vi, "vftti"), vec!["A"]);
        assert_eq!(allowed_padas_artha("P", "kramu", &[], "sarga"), vec!["P", "A"]);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `yam_sam_needs_artha`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn yam_sam_needs_artha() {
        let sam = s(&["sam"]);
        assert_eq!(allowed_padas("P", "yama", &sam), vec!["P"]);
        assert_eq!(allowed_padas_artha("P", "yama", &sam, "agranthe"), vec!["A"]);
        assert_eq!(allowed_padas_artha("P", "yama", &sam, "granthe"), vec!["P"]);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `bhr_yajna_is_a`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn bhr_yajna_is_a() {
        let sam = s(&["sam"]);
        assert_eq!(allowed_padas("U", "quBfY", &sam), vec!["P", "A"]);
        assert_eq!(allowed_padas_artha("U", "quBfY", &sam, "yajYa"), vec!["A"]);
    }
}
