//! Temporary per-dhātu patches where the Kaumudī prakriyā is still incomplete.
//! Prefer `live_generate`; delete a block when the sūtra is in stems/join.
//! Do not add scrape-only forms here.
#![allow(non_snake_case, unused)]

pub fn lookup_override(
    dhatu_query: &str,
    canonical: &str,
    purusha: u8,
    vacana: u8,
    prefixes: &[String],
) -> Option<Vec<String>> {
    if prefixes.is_empty()
        && matches!(
            dhatu_query,
            "ada" | "02.0001" | "hana" | "02.0002" | "hu" | "03.0001" | "zuY" | "05.0001"
                | "ruDir" | "07.0001" | "tanu" | "08.0001" | "qukrIY" | "09.0001" | "zwuY"
                | "02.0038" | "iR" | "02.0040" | "brUY" | "02.0039" | "zWA" | "01.1077"
                | "02.0060" | "asa" | "dviza" | "02.0003" | "duha" | "02.0004" | "diha"
                | "02.0005" | "liha" | "02.0006" | "yu" | "02.0027" | "ru" | "02.0028"
                | "tu" | "02.0029" | "Ru" | "02.0030" | "wukzu" | "02.0031" | "kzRu"
                | "02.0032" | "zRu" | "02.0033" | "UrRuY" | "02.0034" | "dyu" | "02.0035"
                | "zu" | "02.0036" | "ku" | "02.0037" | "vI" | "02.0043" | "yA" | "02.0044"
                | "vA" | "02.0045" | "BA" | "02.0046" | "zRA" | "02.0047" | "SrA" | "02.0048"
                | "drA" | "02.0049" | "psA" | "02.0050" | "pA" | "02.0051" | "rA" | "02.0052"
                | "lA" | "02.0053" | "dAp" | "02.0054" | "KyA" | "02.0055" | "prA" | "02.0056"
                | "mA" | "02.0057" | "vaca" | "02.0058" | "mfjU" | "02.0061" | "rivi" | "01.0679" | "ravi" | "01.0680" | "vftu" | "01.0862"
                | "vfDu" | "01.0863" | "syandU" | "01.0865" | "kfpU" | "01.0866" | "yama"
                | "01.1031" | "01.1139" | "RIY" | "01.1049" | "ovE" | "01.1070" | "zRE"
                | "01.1072" | "dEp" | "01.1073" | "dAR" | "01.1079" | "f" | "01.1086"
                | "sru" | "01.1090" | "01.1091" | "Sru" | "01.1092" | "Dru" | "01.1093"
                | "du" | "01.1094" | "dru" | "01.1095" | "skandir" | "01.1134" | "Rama"
                | "01.1136" | "tyaja" | "01.1141" | "zanja" | "01.1142" | "dfSir" | "01.1143"
                | "danSa" | "01.1144" | "ranja" | "01.1154" | "veY" | "01.1161" | "vyeY"
                | "01.1162" | "hveY" | "01.1163" | "quyAcf" | "01.0954" | "zWala" | "01.0970"
                | "vida" | "02.0059" | "rudir" | "02.0062" | "Yizvapa" | "02.0063" | "Svasa"
                | "02.0064" | "ana" | "02.0065" | "jakza" | "02.0066" | "jAgf" | "02.0067"
                | "SAsu" | "02.0070" | "vaSa" | "02.0075"
                | "cakziN" | "02.0007" | "ik" | "02.0042" | "daridrA" | "02.0068"
                | "cakAsf" | "02.0069" | "zasa" | "02.0073" | "zasti" | "02.0074"
        )
        && matches!(
            canonical,
            "plat" | "plan" | "plot" | "pvidhilin" | "plrt" | "plun" | "pashirling"
        )
    {
        return None;
    }
    if dhatu_query == "Rikza" || dhatu_query == "01.0747" {
        if let Some(forms) = rikza_forms(&canonical, purusha, vacana) {
            if prefixes.is_empty() { return Some(forms); }
            return Some(forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect());
        }
    }
    if dhatu_query == "Divi" || dhatu_query == "01.0677" {
        if let Some(forms) = divi_forms(&canonical, purusha, vacana) {
            if prefixes.is_empty() { return Some(forms); }
            return Some(forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect());
        }
    }
    if dhatu_query == "fti" || dhatu_query == "01.1166" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["artizyati".into()]),
                (1,2) => return Some(vec!["artizyataH".into()]),
                (1,3) => return Some(vec!["artizyanti".into()]),
                (2,1) => return Some(vec!["artizyasi".into()]),
                (2,2) => return Some(vec!["artizyaTaH".into()]),
                (2,3) => return Some(vec!["artizyaTa".into()]),
                (3,1) => return Some(vec!["artizyAmi".into()]),
                (3,2) => return Some(vec!["artizyAvaH".into()]),
                (3,3) => return Some(vec!["artizyAmaH".into()]),
                _ => {}
            }
        }
    }
    None
}

fn rikza_forms(canonical: &str, purusha: u8, vacana: u8) -> Option<Vec<String>> {
    match (canonical, purusha, vacana) {
        ("plan",1,1) => Some(vec!["anikzat".into(), "anikzad".into()]),
        ("plan",1,2) => Some(vec!["anikzatAm".into()]),
        ("plan",1,3) => Some(vec!["anikzan".into()]),
        ("plan",2,1) => Some(vec!["anikzaH".into()]),
        ("plan",2,2) => Some(vec!["anikzatam".into()]),
        ("plan",2,3) => Some(vec!["anikzata".into()]),
        ("plan",3,1) => Some(vec!["anikzam".into()]),
        ("plan",3,2) => Some(vec!["anikzAva".into()]),
        ("plan",3,3) => Some(vec!["anikzAma".into()]),
        ("pvidhilin",1,1) => Some(vec!["nikzet".into(), "nikzed".into()]),
        ("pvidhiling",1,1) => Some(vec!["nikzet".into(), "nikzed".into()]),
        ("pvidhilin",1,2) => Some(vec!["nikzetAm".into()]),
        ("pvidhiling",1,2) => Some(vec!["nikzetAm".into()]),
        ("pvidhilin",1,3) => Some(vec!["nikzeyuH".into()]),
        ("pvidhiling",1,3) => Some(vec!["nikzeyuH".into()]),
        ("pvidhilin",2,1) => Some(vec!["nikzeH".into()]),
        ("pvidhiling",2,1) => Some(vec!["nikzeH".into()]),
        ("pvidhilin",2,2) => Some(vec!["nikzetam".into()]),
        ("pvidhiling",2,2) => Some(vec!["nikzetam".into()]),
        ("pvidhilin",2,3) => Some(vec!["nikzeta".into()]),
        ("pvidhiling",2,3) => Some(vec!["nikzeta".into()]),
        ("pvidhilin",3,1) => Some(vec!["nikzeyam".into()]),
        ("pvidhiling",3,1) => Some(vec!["nikzeyam".into()]),
        ("pvidhilin",3,2) => Some(vec!["nikzeva".into()]),
        ("pvidhiling",3,2) => Some(vec!["nikzeva".into()]),
        ("pvidhilin",3,3) => Some(vec!["nikzema".into()]),
        ("pvidhiling",3,3) => Some(vec!["nikzema".into()]),
        _ => None,
    }
}


fn divi_forms(canonical: &str, purusha: u8, vacana: u8) -> Option<Vec<String>> {
    match (canonical, purusha, vacana) {
        ("plat",1,1) => Some(vec!["Dinoti".into()]),
        ("plat",1,2) => Some(vec!["DinutaH".into()]),
        ("plat",1,3) => Some(vec!["Dinvanti".into()]),
        ("plat",2,1) => Some(vec!["Dinozi".into()]),
        ("plat",2,2) => Some(vec!["DinuTaH".into()]),
        ("plat",2,3) => Some(vec!["DinuTa".into()]),
        ("plat",3,1) => Some(vec!["Dinomi".into()]),
        ("plat",3,2) => Some(vec!["DinuvaH".into(), "DinvaH".into()]),
        ("plat",3,3) => Some(vec!["DinumaH".into(), "DinmaH".into()]),
        ("plan",1,1) => Some(vec!["aDinot".into(), "aDinod".into()]),
        ("plan",1,2) => Some(vec!["aDinutAm".into()]),
        ("plan",1,3) => Some(vec!["aDinvan".into()]),
        ("plan",2,1) => Some(vec!["aDinoH".into()]),
        ("plan",2,2) => Some(vec!["aDinutam".into()]),
        ("plan",2,3) => Some(vec!["aDinuta".into()]),
        ("plan",3,1) => Some(vec!["aDinavam".into()]),
        ("plan",3,2) => Some(vec!["aDinuva".into(), "aDinva".into()]),
        ("plan",3,3) => Some(vec!["aDinuma".into(), "aDinma".into()]),
        ("plot",1,1) => Some(vec!["DinutAt".into(), "DinutAd".into(), "Dinotu".into()]),
        ("plot",1,2) => Some(vec!["DinutAm".into()]),
        ("plot",1,3) => Some(vec!["Dinvantu".into()]),
        ("plot",2,1) => Some(vec!["Dinu".into(), "DinutAt".into(), "DinutAd".into()]),
        ("plot",2,2) => Some(vec!["Dinutam".into()]),
        ("plot",2,3) => Some(vec!["Dinuta".into()]),
        ("plot",3,1) => Some(vec!["DinavAni".into()]),
        ("plot",3,2) => Some(vec!["DinavAva".into()]),
        ("plot",3,3) => Some(vec!["DinavAma".into()]),
        ("plrt",1,1) => Some(vec!["Dinvizyati".into()]),
        ("plrt",1,2) => Some(vec!["DinvizyataH".into()]),
        ("plrt",1,3) => Some(vec!["Dinvizyanti".into()]),
        ("plrt",2,1) => Some(vec!["Dinvizyasi".into()]),
        ("plrt",2,2) => Some(vec!["DinvizyaTaH".into()]),
        ("plrt",2,3) => Some(vec!["DinvizyaTa".into()]),
        ("plrt",3,1) => Some(vec!["DinvizyAmi".into()]),
        ("plrt",3,2) => Some(vec!["DinvizyAvaH".into()]),
        ("plrt",3,3) => Some(vec!["DinvizyAmaH".into()]),
        ("pvidhilin",1,1) => Some(vec!["DinuyAt".into(), "DinuyAd".into()]),
        ("pvidhiling",1,1) => Some(vec!["DinuyAt".into(), "DinuyAd".into()]),
        ("pvidhilin",1,2) => Some(vec!["DinuyAtAm".into()]),
        ("pvidhiling",1,2) => Some(vec!["DinuyAtAm".into()]),
        ("pvidhilin",1,3) => Some(vec!["DinuyuH".into()]),
        ("pvidhiling",1,3) => Some(vec!["DinuyuH".into()]),
        ("pvidhilin",2,1) => Some(vec!["DinuyAH".into()]),
        ("pvidhiling",2,1) => Some(vec!["DinuyAH".into()]),
        ("pvidhilin",2,2) => Some(vec!["DinuyAtam".into()]),
        ("pvidhiling",2,2) => Some(vec!["DinuyAtam".into()]),
        ("pvidhilin",2,3) => Some(vec!["DinuyAta".into()]),
        ("pvidhiling",2,3) => Some(vec!["DinuyAta".into()]),
        ("pvidhilin",3,1) => Some(vec!["DinuyAm".into()]),
        ("pvidhiling",3,1) => Some(vec!["DinuyAm".into()]),
        ("pvidhilin",3,2) => Some(vec!["DinuyAva".into()]),
        ("pvidhiling",3,2) => Some(vec!["DinuyAva".into()]),
        ("pvidhilin",3,3) => Some(vec!["DinuyAma".into()]),
        ("pvidhiling",3,3) => Some(vec!["DinuyAma".into()]),
        _ => None,
    }
}
