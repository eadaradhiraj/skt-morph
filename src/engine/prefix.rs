//! Port of morphology.py UPASARGA handling + apply_forward_sandhi
//! Pāṇini sandhi for prefixes (upasarga) + verb/participle

pub fn apply_forward_sandhi(prefix: &str, word: &str) -> String {
    if prefix.is_empty() { return word.to_string(); }
    let word = word.trim_start_matches('-');
    if word.is_empty() { return prefix.to_string(); }
    let p_chars: Vec<char> = prefix.chars().collect();
    let w_chars: Vec<char> = word.chars().collect();
    let p_end = *p_chars.last().unwrap();
    let w_start = w_chars[0];
    let w_rest: String = w_chars[1..].iter().collect();
    let vowels = ['a','A','i','I','u','U','f','F','e','E','o','O'];
    let voiced_cons = ['g','G','j','J','q','Q','d','D','b','B','N','Y','R','n','m','y','r','l','v','h'];
    let unvoiced_cons = ['k','K','c','C','w','W','t','T','p','P','S','z','s'];

    let mut result = format!("{}{}", prefix, word);
    if vowels.contains(&p_end) && w_start == 'C' {
        result = format!("{}c{}", prefix, word);
    }
    if p_end == 's' {
        if prefix.ends_with("as") {
            if voiced_cons.contains(&w_start) {
                result = format!("{}o{}", &prefix[..prefix.len()-2], word);
            } else if w_start == 'a' {
                result = format!("{}o{}", &prefix[..prefix.len()-2], w_rest);
            } else if vowels.contains(&w_start) {
                result = format!("{}a{}", &prefix[..prefix.len()-2], word);
            } else if w_start == 'c' || w_start == 'C' {
                result = format!("{}S{}", &prefix[..prefix.len()-1], word);
            } else if w_start == 'w' || w_start == 'W' {
                result = format!("{}z{}", &prefix[..prefix.len()-1], word);
            } else if ['k','K','p','P'].contains(&w_start) {
                result = format!("{}H{}", &prefix[..prefix.len()-1], word);
            }
        } else if prefix.ends_with("is") || prefix.ends_with("us") {
            if voiced_cons.contains(&w_start) || vowels.contains(&w_start) {
                result = format!("{}r{}", &prefix[..prefix.len()-1], word);
            } else if w_start == 'c' || w_start == 'C' {
                result = format!("{}S{}", &prefix[..prefix.len()-1], word);
            } else if matches!(w_start, 'w' | 'W' | 'k' | 'K' | 'p' | 'P') {
                result = format!("{}z{}", &prefix[..prefix.len()-1], word);
            }
        }
    } else if p_end == 'a' || p_end == 'A' {
        if w_start == 'a' || w_start == 'A' {
            result = format!("{}A{}", &prefix[..prefix.len()-1], w_rest);
        } else if w_start == 'i' || w_start == 'I' {
            result = format!("{}e{}", &prefix[..prefix.len()-1], w_rest);
        } else if w_start == 'u' || w_start == 'U' {
            if prefix == "pra" && word.starts_with("Uh") {
                result = format!("{}O{}", &prefix[..prefix.len()-1], w_rest);
            } else {
                result = format!("{}o{}", &prefix[..prefix.len()-1], w_rest);
            }
        } else if w_start == 'f' || w_start == 'F' {
            result = format!("{}Ar{}", &prefix[..prefix.len()-1], w_rest);
        } else if w_start == 'e' {
            result = format!("{}e{}", &prefix[..prefix.len()-1], w_rest);
        } else if w_start == 'E' {
            result = format!("{}E{}", &prefix[..prefix.len()-1], w_rest);
        } else if w_start == 'o' || w_start == 'O' {
            result = format!("{}O{}", &prefix[..prefix.len()-1], w_rest);
        }
    } else if p_end == 'i' || p_end == 'I' {
        if vowels.contains(&w_start) && w_start != 'i' && w_start != 'I' {
            result = format!("{}y{}", &prefix[..prefix.len()-1], word);
        } else if w_start == 'i' || w_start == 'I' {
            result = format!("{}I{}", &prefix[..prefix.len()-1], w_rest);
        }
    } else if p_end == 'u' || p_end == 'U' {
        if vowels.contains(&w_start) && w_start != 'u' && w_start != 'U' {
            result = format!("{}v{}", &prefix[..prefix.len()-1], word);
        } else if w_start == 'u' || w_start == 'U' {
            result = format!("{}U{}", &prefix[..prefix.len()-1], w_rest);
        }
    } else if prefix.ends_with('m') && (voiced_cons.contains(&w_start) || unvoiced_cons.contains(&w_start)) {
        // 8.4.58 अनुस्वारस्य ययि परसवर्णः — ङ् before velar (सङ्करोति), ञ् before palatal.
        let nasal = match w_start {
            'k' | 'K' | 'g' | 'G' => 'N',
            'c' | 'C' | 'j' | 'J' => 'Y',
            'w' | 'W' | 'q' | 'Q' => 'R',
            't' | 'T' | 'd' | 'D' => 'n',
            _ => 'M',
        };
        result = format!("{}{}{}", &prefix[..prefix.len() - 1], nasal, word);
    } else if prefix == "ud" {
        if word.starts_with("sT") {
            result = format!("utT{}", &word[2..]);
        } else if word.starts_with("stamB") {
            result = format!("uttamB{}", &word[5..]);
        } else if word.starts_with("staB") {
            result = format!("uttaB{}", &word[4..]);
        } else if ['k','K','c','C','w','W','t','T','p','P','s','S','z'].contains(&w_start) {
            result = format!("ut{}", word);
        } else if w_start == 'h' {
            result = format!("uddh{}", w_rest);
        }
    } else if prefix == "Srat" {
        if voiced_cons.contains(&w_start) || vowels.contains(&w_start) {
            result = format!("Srad{}", word);
        } else {
            result = format!("Srat{}", word);
        }
    }
    if let Some(satva) = apply_upasarga_satva(prefix, word) {
        result = satva;
    }

    crate::engine::phonology::apply_natva_to_word(&result)
}

fn apply_upasarga_satva(prefix: &str, word: &str) -> Option<String> {
    if !matches!(prefix, "ni" | "vi" | "nI" | "su" | "anu") {
        return None;
    }
    let w_chars: Vec<char> = word.chars().collect();
    if w_chars.first() != Some(&'s') {
        return None;
    }
    let rest: String = w_chars[1..].iter().collect();
    let mutated = match w_chars.get(1).copied() {
        Some('t') => format!("zw{}", w_chars[2..].iter().collect::<String>()),
        Some('T') => format!("zW{}", w_chars[2..].iter().collect::<String>()),
        Some('d') => format!("zq{}", w_chars[2..].iter().collect::<String>()),
        Some('n') => format!("zR{}", w_chars[2..].iter().collect::<String>()),
        _ => format!("z{rest}"),
    };
    Some(format!("{prefix}{mutated}"))
}

pub fn apply_prefixes(prefixes: &[String], base: &str) -> String {
    let mut cur = base.to_string();
    for p in prefixes.iter().rev() {
        cur = apply_forward_sandhi(p, &cur);
    }
    cur
}

pub const UPASARGAS: &[&str] = &[
    "pra", "parA", "apa", "sam", "anu", "ava", "nis", "nir", "dus", "dur",
    "vi", "A", "aDi", "api", "ati", "su", "ud", "aBi", "prati", "pari", "upa", "ni",
];

/// Remainders `rest` such that `apply_forward_sandhi(prefix, rest) == word`.
pub fn unapply_prefix(prefix: &str, word: &str) -> Vec<String> {
    let mut out = Vec::new();
    if prefix.is_empty() || word.len() <= prefix.len() {
        return out;
    }
    for i in 1..word.len() {
        let rest = &word[i..];
        if rest.is_empty() {
            continue;
        }
        if apply_forward_sandhi(prefix, rest) == word {
            out.push(rest.to_string());
        }
    }
    out
}

/// `(upasargas left-to-right, bare form)` including the unprefixed word.
pub fn split_upasarga_candidates(word: &str) -> Vec<(Vec<String>, String)> {
    let mut out = vec![(Vec::new(), word.to_string())];
    for &p in UPASARGAS {
        for rest in unapply_prefix(p, word) {
            out.push((vec![p.to_string()], rest.clone()));
            for &p2 in UPASARGAS {
                if p2 == p {
                    continue;
                }
                for rest2 in unapply_prefix(p2, &rest) {
                    out.push((vec![p.to_string(), p2.to_string()], rest2));
                }
            }
        }
    }
    out
}

// For analyze, we need split candidates - simplified version for generation is enough
// Full split BFS is in analyze, here just for completeness

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // ---------------------------------------------------------------------------
    // fn `test_sam_bhu`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn test_sam_bhu() {
        assert_eq!(apply_forward_sandhi("sam", "BUtvA"), "saMBUtvA");
    }
    #[test]
    // ---------------------------------------------------------------------------
    // fn `test_pra_ets`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn test_pra_ets() {
        assert_eq!(apply_forward_sandhi("pra", "eti"), "preti");
    }
    #[test]
    // ---------------------------------------------------------------------------
    // fn `pra_bhavanti_no_natva_on_tin_ending` — tin/sUP endings: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn pra_bhavanti_no_natva_on_tin_ending() {
        assert_eq!(apply_forward_sandhi("pra", "Bavanti"), "praBavanti");
        assert_eq!(apply_forward_sandhi("pra", "Bavizyanti"), "praBavizyanti");
        assert_eq!(apply_forward_sandhi("pra", "BavAni"), "praBavAni");
        let f = apply_prefixes(&["aBi".into(), "pra".into()], "Bavanti");
        assert_eq!(f, "aBipraBavanti");
    }
    #[test]
    // ---------------------------------------------------------------------------
    // fn `pra_namati_has_natva`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn pra_namati_has_natva() {
        assert_eq!(apply_forward_sandhi("pra", "namati"), "praRamati");
    }
    #[test]
    // ---------------------------------------------------------------------------
    // fn `unapply_pra_gacchati`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn unapply_pra_gacchati() {
        let rest = unapply_prefix("pra", "pragacCati");
        assert!(rest.iter().any(|r| r == "gacCati"), "{:?}", rest);
    }
    #[test]
    // ---------------------------------------------------------------------------
    // fn `unapply_sam_bhu`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn unapply_sam_bhu() {
        let rest = unapply_prefix("sam", "saMBUtvA");
        assert!(rest.iter().any(|r| r == "BUtvA"), "{:?}", rest);
    }
    #[test]
    // ---------------------------------------------------------------------------
    // fn `sam_kf_satva_a_r`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn sam_kf_satva_a_r() {
        assert_eq!(apply_forward_sandhi("sam", "karoti"), "saNkaroti");
        assert_eq!(apply_forward_sandhi("A", "fcCati"), "ArcCati");
        assert_eq!(apply_forward_sandhi("ni", "sIdati"), "nizIdati");
        assert_eq!(apply_forward_sandhi("ni", "sTAti"), "nizWAti");
        let rest = unapply_prefix("sam", "saNkaroti");
        assert!(rest.iter().any(|r| r == "karoti"), "{:?}", rest);
    }
}
