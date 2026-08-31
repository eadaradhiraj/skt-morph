//! Upasarga-conditioned pada (Pāṇini 1.3.12-77)
//! Determines which padas (Parasmaipadī / Ātmanepadī) are valid for a given dhatu + upasarga combo.
//!
//! Phase 1 covers the reporter cases:
//!   - gam (P) bare -> P only, sam+gam / sam+A+gam -> P+A (1.3.29 vA)
//!   - ji  (P) bare -> P only, vi/parA + ji -> A only (1.3.19 nitya), other upasarga -> P
//! Design is declarative so adding further sūtras is just adding entries.

fn normalize_one(p: &str) -> String {
    let t = p.trim().trim_matches(|c| c==',' || c==';').trim();
    // handle SLP1 variants with anusvāra/ṅ etc
    match t {
        "saG" | "saM" | "saN" | "sam" | "SaM" | "SaG" | "SAM" | "sAM" => "sam".to_string(),
        "samA" | "saMA" => "samA".to_string(),
        "a" | "A" | "AG" | "AM" => "A".to_string(),
        other => other.to_string(),
    }
}

fn normalized_prefixes(prefixes: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for raw in prefixes {
        // split compound entries like "sam;A" or "sam, A" or "sam A"
        for part in raw.split(|c| c==',' || c==';' || c==' ') {
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
/// Returns Vec containing "P" and/or "A". Empty means no valid pada (shouldn't happen).
pub fn allowed_padas(root_pada: &str, dhatu: &str, prefixes: &[String]) -> Vec<String> {
    // U (ubhayapadī) -> both always
    if root_pada == "U" {
        return vec!["P".to_string(), "A".to_string()];
    }

    let d = dhatu.trim();
    let norm = normalized_prefixes(prefixes);
    let has_sam = has_prefix(&norm, "sam");
    let _has_A = has_prefix(&norm, "A");

    // --- gam ---
    // 1.3.29 sam + gam -> optional Ātmanepada (vA). Includes sam+A+gam (samAgam)
    // bare gam or A+gam alone -> P only
    if d == "gam" || d == "gamx" || d == "gamy" {
        if root_pada == "P" {
            if has_sam {
                // sam+gam and sam+A+gam both allow P and A
                return vec!["P".to_string(), "A".to_string()];
            } else {
                return vec!["P".to_string()];
            }
        } else if root_pada == "A" {
            // no gam is A by default, but keep general
            return vec!["A".to_string()];
        }
    }

    // --- ji ---
    // 1.3.19 vi / parA + ji (1P "to win") -> nitya Ātmanepada
    //   vi+ji = vijayate (A only), parA+ji = parAjayate (A only)
    //   bare ji or other upasarga (abhi, sam, etc) -> P only (jayati)
    // Cross-check: gold (ashtadhyayi.com) has ji ting only plat; vi+ji should be alat only.
    if d == "ji" {
        if root_pada == "P" {
            let has_vi = has_prefix(&norm, "vi");
            let has_parA = has_prefix(&norm, "parA");
            if norm.is_empty() {
                return vec!["P".to_string()];
            }
            if has_vi || has_parA {
                // mandatory Ātmanepada -> P not allowed
                return vec!["A".to_string()];
            } else {
                // other prefix -> still P (no rule)
                return vec!["P".to_string()];
            }
        }
    }

    // --- sTA 1.3.22 समवप्रविभ्यः स्थः ---
    if d == "sTA" || d == "zWA" {
        if has_sam || has_prefix(&norm, "ava") || has_prefix(&norm, "pra") || has_prefix(&norm, "vi") {
            return vec!["A".to_string()];
        }
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

    // --- 1.3.66 भृञो यज्ञकर्मणि? sam/ni/ud + Bf → A in non-saṃskāra; treat as vA ---
    if matches!(d, "Bf" | "BfY" | "quBfY") && (has_sam || has_prefix(&norm, "ni") || has_prefix(&norm, "ud")) {
        return vec!["P".to_string(), "A".to_string()];
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

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> { v.iter().map(|x| x.to_string()).collect() }

    #[test]
    fn gam_bare() {
        assert_eq!(allowed_padas("P","gam",&[]), vec!["P"]);
        assert!(pada_allowed("P","P","gam",&[]));
        assert!(!pada_allowed("P","A","gam",&[]));
    }
    #[test]
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
    fn ji_bare() {
        assert_eq!(allowed_padas("P","ji",&[]), vec!["P"]);
        assert!(!pada_allowed("P","A","ji",&[]));
    }
    #[test]
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
    fn ji_other_prefix_stays_p() {
        let abhi = s(&["abhi"]);
        assert_eq!(allowed_padas("P","ji",&abhi), vec!["P"]);
    }
    #[test]
    fn stha_sam_is_a() {
        let sam = s(&["sam"]);
        assert_eq!(allowed_padas("P", "zWA", &sam), vec!["A"]);
    }
    #[test]
    fn ud_car_is_a() {
        let ud = s(&["ud"]);
        assert_eq!(allowed_padas("P", "car", &ud), vec!["A"]);
    }
}
