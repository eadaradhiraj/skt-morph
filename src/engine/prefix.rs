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
            } else if w_start == 'w' || w_start == 'W' {
                result = format!("{}z{}", &prefix[..prefix.len()-1], word);
            } else if ['k','K','p','P'].contains(&w_start) {
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
        } else if w_start == 'o' {
            result = format!("{}O{}", &prefix[..prefix.len()-1], w_rest);
        } else if w_start == 'O' {
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
        result = format!("{}M{}", &prefix[..prefix.len()-1], word);
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

    // ṇatva (r/f/z triggers n->R, blocked by cCjJY etc)
    let mut chars: Vec<char> = result.chars().collect();
    let prefix_len = result.len() - word.len();
    let natva_prefixes = ["pra","parA","pari","nis","dus","antar"];
    let mut trigger = false;
    let blockers: std::collections::HashSet<char> = ['c','C','j','J','Y','S','w','W','q','Q','R','t','T','d','D','l','s','S'].iter().cloned().collect();
    for i in 0..chars.len() {
        let ch = chars[i];
        if ch == 'r' || ch == 'f' || ch == 'F' || ch == 'z' {
            if i < prefix_len && !natva_prefixes.contains(&prefix) { continue; }
            trigger = true;
        } else if trigger && ch == 'n' {
            if i != chars.len() - 1 {
                chars[i] = 'R';
            }
        } else if trigger && blockers.contains(&ch) {
            trigger = false;
        }
    }
    chars.into_iter().collect()
}

pub fn apply_prefixes(prefixes: &[String], base: &str) -> String {
    let mut cur = base.to_string();
    for p in prefixes.iter().rev() {
        cur = apply_forward_sandhi(p, &cur);
    }
    cur
}

// For analyze, we need split candidates - simplified version for generation is enough
// Full split BFS is in analyze, here just for completeness

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sam_bhu() {
        assert_eq!(apply_forward_sandhi("sam", "BUtvA"), "saMBUtvA");
    }
    #[test]
    fn test_pra_ets() {
        assert_eq!(apply_forward_sandhi("pra", "eti"), "preti");
    }
}
