//! Resolve a user dhātu query (id or SLP1 name) against the compact index.

#[derive(Clone, Copy, Debug)]
pub struct DhatuRow {
    pub id: &'static str,
    pub dhatu: &'static str,
    pub gana: u8,
    pub pada: &'static str,
    pub tags: &'static str,
    pub antarganas: &'static str,
    pub aupadeshik: &'static str,
}

fn row((id, dhatu, gana, pada, tags, ant, aup): &(&'static str, &'static str, u8, &'static str, &'static str, &'static str, &'static str)) -> DhatuRow {
    DhatuRow { id, dhatu, gana: *gana, pada, tags, antarganas: ant, aupadeshik: aup }
}

use std::collections::HashMap;
use std::sync::OnceLock;

static ID_MAP: OnceLock<HashMap<&'static str, usize>> = OnceLock::new();
static NAME_MAP: OnceLock<HashMap<&'static str, Vec<usize>>> = OnceLock::new();

fn id_map() -> &'static HashMap<&'static str, usize> {
    ID_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        for (i, rec) in crate::data::DHATUS.iter().enumerate() {
            m.insert(rec.0, i);
        }
        m
    })
}
fn name_map() -> &'static HashMap<&'static str, Vec<usize>> {
    NAME_MAP.get_or_init(|| {
        let mut m: HashMap<&'static str, Vec<usize>> = HashMap::new();
        for (i, rec) in crate::data::DHATUS.iter().enumerate() {
            m.entry(rec.1).or_default().push(i);
        }
        m
    })
}

fn is_it_suffix(s: &str) -> bool {
    matches!(
        s,
        "x" | "Y" | "R" | "N" | "ir" | "o" | "A" | "I" | "U" | "F" | "e" | "E" | "i" | "u" | "f"
    )
}

/// First matching row: exact id, then exact name (file order, so bhvādi before curādi), then name+it (gam → gamx).
/// Uses OnceLock HashMaps for O(1) id/name after first call (was linear scan per lookup).
pub fn lookup(query: &str) -> Option<DhatuRow> {
    let q = query.trim();
    if q.is_empty() {
        return None;
    }
    if let Some(&idx) = id_map().get(q) {
        return Some(row(&crate::data::DHATUS[idx]));
    }
    // अस् by name: Kaumudī अस्ति (02.0060), not भ्वादि 01.1029.
    if q == "as" || q == "asa" {
        if let Some(&idx) = id_map().get("02.0060") {
            return Some(row(&crate::data::DHATUS[idx]));
        }
    }
    if let Some(idxs) = name_map().get(q) {
        for &i in idxs {
            let rec = &crate::data::DHATUS[i];
            // यम् by name: Kaumudī यच्छति (not ghaṭādi 01.0930 mittva).
            if q == "yama" && rec.5.contains("GawAdi") {
                continue;
            }
            return Some(row(rec));
        }
    }
    for rec in crate::data::DHATUS {
        let name = rec.1;
        if name.starts_with(q) && name.len() > q.len() && is_it_suffix(&name[q.len()..]) {
            return Some(row(rec));
        }
    }
    // Devanagari क्रम → SLP1 `krama` (inherent a). Retry without final a.
    if q.ends_with('a') && q.len() > 2 {
        let stem = &q[..q.len() - 1];
        for rec in crate::data::DHATUS {
            if rec.1 == stem {
                return Some(row(rec));
            }
        }
        for rec in crate::data::DHATUS {
            let name = rec.1;
            if name.starts_with(stem) && name.len() > stem.len() && is_it_suffix(&name[stem.len()..]) {
                return Some(row(rec));
            }
        }
    }
    None
}

pub fn resolve_id(query: &str) -> String {
    let q = query.trim();
    if q.contains('.') {
        return q.to_string();
    }
    lookup(q).map(|d| d.id.to_string()).unwrap_or_else(|| q.to_string())
}

/// Tuple used by tinanta / krdanta. Unknown queries stay gana 1 so foreign roots can still inflect.
pub fn load_or_fallback(query: &str) -> (String, u8, String, String, String, String) {
    if let Some(d) = lookup(query) {
        (
            d.dhatu.to_string(),
            d.gana,
            d.pada.to_string(),
            d.tags.to_string(),
            d.antarganas.to_string(),
            d.aupadeshik.to_string(),
        )
    } else {
        (query.trim().to_string(), 1, "P".to_string(), String::new(), String::new(), String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bu_by_name_is_bhavadi() {
        let d = lookup("BU").expect("BU");
        assert_eq!(d.id, "01.0001");
        assert_eq!(d.gana, 1);
    }

    #[test]
    fn bu_by_id() {
        let d = lookup("01.0001").expect("id");
        assert_eq!(d.dhatu, "BU");
    }

    #[test]
    fn gam_resolves_gamx() {
        let d = lookup("gam").expect("gam");
        assert_eq!(d.id, "01.1137");
        assert_eq!(d.dhatu, "gamx");
    }

    #[test]
    fn yama_by_name_is_yacchati() {
        let d = lookup("yama").expect("yama");
        assert_ne!(d.id, "01.0930");
        assert!(!d.antarganas.contains("GawAdi"));
    }

    #[test]
    fn krama_from_deva_is_kramu() {
        let d = lookup("krama").expect("krama");
        assert_eq!(d.dhatu, "kramu");
    }

    #[test]
    fn as_by_name_is_adadi() {
        let d = lookup("asa").expect("asa");
        assert_eq!(d.id, "02.0060");
        assert_eq!(d.gana, 2);
        let d = lookup("as").expect("as");
        assert_eq!(d.id, "02.0060");
    }
}
