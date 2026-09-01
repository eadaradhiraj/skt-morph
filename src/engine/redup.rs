//! Port of engine/redup.py

use crate::engine::phonology::apply_guna_to_stem;

pub const GANA3: u8 = 3;

#[derive(Debug, Clone)]
pub struct Gana3Profile {
    pub present: String,
    pub join: String, // nu | ad
    pub lang: String,
    pub vidhilin: String,
    pub future: String,
}

fn profile(dhatu: &str, guna: &str) -> Gana3Profile {
    match dhatu {
        "YiBI" => return Gana3Profile { present:"biBe".into(), join:"ad".into(), lang:"biBe".into(), vidhilin:"biBe".into(), future: format!("{}zya", guna) },
        "ohAk" => return Gana3Profile { present:"jahA".into(), join:"ad".into(), lang:"jah".into(), vidhilin:"jah".into(), future: format!("{}sya", guna) },
        "Rijir" | "Rij" => return Gana3Profile { present:"nenek".into(), join:"ad".into(), lang:"nenek".into(), vidhilin:"nenek".into(), future:"nekzya".into() },
        "vizx" => return Gana3Profile { present:"vevez".into(), join:"ad".into(), lang:"vevez".into(), vidhilin:"vevez".into(), future:"vejizya".into() },
        "Gf" => return Gana3Profile { present:"jaGar".into(), join:"ad".into(), lang:"jaGar".into(), vidhilin:"jaGar".into(), future:"Garizya".into() },
        "hu" => return Gana3Profile { present:"juhu".into(), join:"nu".into(), lang:"juh".into(), vidhilin:"juhuy".into(), future: format!("{}zya", guna) },
        "BI" => return Gana3Profile { present:"biBi".into(), join:"ad".into(), lang:"biBi".into(), vidhilin:"biBi".into(), future: format!("{}zya", guna) },
        "hrI" => return Gana3Profile { present:"jihrI".into(), join:"ad".into(), lang:"jihrI".into(), vidhilin:"jihrI".into(), future: format!("{}zya", guna) },
        "pF" | "pf" => return Gana3Profile { present:"pipUr".into(), join:"ad".into(), lang:"pipar".into(), vidhilin:"pipUr".into(), future:"parizya".into() },
        "Bf" => return Gana3Profile { present:"biBf".into(), join:"ad".into(), lang:"biBar".into(), vidhilin:"biBf".into(), future:"Barizya".into() },
        "mA" => return Gana3Profile { present:"mimI".into(), join:"ad".into(), lang:"mimI".into(), vidhilin:"mimI".into(), future: format!("{}sya", guna) },
        "hA" => {
            if guna=="hA" { return Gana3Profile { present:"jihI".into(), join:"ad".into(), lang:"jihI".into(), vidhilin:"jihI".into(), future: format!("{}sya", guna)}; }
            return Gana3Profile { present:"jahA".into(), join:"ad".into(), lang:"jah".into(), vidhilin:"jah".into(), future: format!("{}sya", guna)};
        }
        "dA" => return Gana3Profile { present:"dadA".into(), join:"ad".into(), lang:"dad".into(), vidhilin:"dad".into(), future: format!("{}sya", guna)},
        "DA" => return Gana3Profile { present:"daDA".into(), join:"ad".into(), lang:"daD".into(), vidhilin:"daD".into(), future: format!("{}sya", guna)},
        "nij" => return Gana3Profile { present:"nenij".into(), join:"ad".into(), lang:"nenij".into(), vidhilin:"nenij".into(), future:"nekzya".into()},
        "vij" => return Gana3Profile { present:"vevij".into(), join:"ad".into(), lang:"vevij".into(), vidhilin:"vevij".into(), future:"vejizya".into()},
        "viz" => return Gana3Profile { present:"veviz".into(), join:"ad".into(), lang:"veviz".into(), vidhilin:"veviz".into(), future:"vejizya".into()},
        _ => {}
    }
    if dhatu.len()==1 {
        return Gana3Profile { present: format!("j{}{}", guna, dhatu), join:"nu".into(), lang: guna.to_string(), vidhilin: format!("{}uy", guna), future: format!("{}zya", guna)};
    }
    if dhatu.len()==2 && dhatu.starts_with('h') {
        return Gana3Profile { present: format!("ji{}", guna), join:"ad".into(), lang: format!("ji{}", guna), vidhilin: format!("ji{}", guna), future: format!("{}zya", guna)};
    }
    let first = dhatu.chars().next().unwrap().to_ascii_lowercase();
    let prefix = format!("{}i{}", first, first);
    let present = if dhatu.len()>1 { format!("{}{}", prefix, &dhatu[1..]) } else { prefix.clone() };
    Gana3Profile { present: present.clone(), join:"ad".into(), lang: present.clone(), vidhilin: present, future: format!("{}zya", guna)}
}

pub fn gana3_present_stem(dhatu: &str, guna: Option<&str>) -> String {
    let g = guna.map(|s| s.to_string()).unwrap_or_else(|| apply_guna_to_stem(dhatu));
    profile(dhatu, &g).present
}
pub fn gana3_join_mode(dhatu: &str, guna: Option<&str>) -> String {
    let g = guna.map(|s| s.to_string()).unwrap_or_else(|| apply_guna_to_stem(dhatu));
    profile(dhatu, &g).join
}
pub fn gana3_lang_stem(dhatu: &str, guna: Option<&str>) -> String {
    let g = guna.map(|s| s.to_string()).unwrap_or_else(|| apply_guna_to_stem(dhatu));
    profile(dhatu, &g).lang
}
pub fn gana3_vidhilin_stem(dhatu: &str, guna: Option<&str>) -> String {
    let g = guna.map(|s| s.to_string()).unwrap_or_else(|| apply_guna_to_stem(dhatu));
    profile(dhatu, &g).vidhilin
}
pub fn gana3_future_stem(dhatu: &str, guna: Option<&str>) -> String {
    let g = guna.map(|s| s.to_string()).unwrap_or_else(|| apply_guna_to_stem(dhatu));
    profile(dhatu, &g).future
}
pub fn gana3_weak_stem(dhatu: &str, guna: &str, ending: &str, purusha: u8) -> String {
    let prof = profile(dhatu, guna);
    if ending=="ti" && purusha==1 {
        match dhatu {
            "BI" => return format!("bi{}", guna),
            "YiBI" => return "biBe".to_string(),
            "ohAk" => return "jahA".to_string(),
            "Rijir" => return "nenek".to_string(),
            "vizx" => return "vevez".to_string(),
            "Gf" => return "jaGar".to_string(),
            "hrI" => return "jihre".to_string(),
            "pF"|"pf" => return "pipa".to_string(),
            "Bf" => return "biBa".to_string(),
            _ => {}
        }
    }
    prof.present
}
pub fn gana3_perfect_stem(dhatu: &str, guna: Option<&str>) -> String { gana3_present_stem(dhatu, guna) }
