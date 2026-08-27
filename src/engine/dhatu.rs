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

fn is_it_suffix(s: &str) -> bool {
    matches!(
        s,
        "x" | "Y" | "R" | "N" | "ir" | "o" | "A" | "I" | "U" | "F" | "e" | "E" | "i" | "u" | "f"
    )
}

/// First matching row: exact id, then exact name (file order, so bhvādi before curādi), then name+it (gam → gamx).
pub fn lookup(query: &str) -> Option<DhatuRow> {
    let q = query.trim();
    if q.is_empty() {
        return None;
    }
    for rec in crate::data::DHATUS {
        if rec.0 == q {
            return Some(row(rec));
        }
    }
    for rec in crate::data::DHATUS {
        if rec.1 == q {
            return Some(row(rec));
        }
    }
    for rec in crate::data::DHATUS {
        let name = rec.1;
        if name.starts_with(q) && name.len() > q.len() && is_it_suffix(&name[q.len()..]) {
            return Some(row(rec));
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
}
