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
                | "02.0060"
        )
        && matches!(
            canonical,
            "plat" | "plan" | "plot" | "pvidhilin" | "plrt" | "plun" | "pashirling"
        )
    {
        return None;
    }
    if dhatu_query == "rivi" || dhatu_query == "01.0679" {
        if canonical == "plot" && purusha == 3 && vacana == 1 {
            let forms = vec!["riRvAni".into()];
            if prefixes.is_empty() { return Some(forms); }
            return Some(forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect());
        }
    }
    if dhatu_query == "ravi" || dhatu_query == "01.0680" {
        if canonical == "plot" && purusha == 3 && vacana == 1 {
            let forms = vec!["raRvAni".into()];
            if prefixes.is_empty() { return Some(forms); }
            return Some(forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect());
        }
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
    if dhatu_query == "vftu" || dhatu_query == "01.0862" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vartsyati".into()]),
                (1,2) => return Some(vec!["vartsyataH".into()]),
                (1,3) => return Some(vec!["vartsyanti".into()]),
                (2,1) => return Some(vec!["vartsyasi".into()]),
                (2,2) => return Some(vec!["vartsyaTaH".into()]),
                (2,3) => return Some(vec!["vartsyaTa".into()]),
                (3,1) => return Some(vec!["vartsyAmi".into()]),
                (3,2) => return Some(vec!["vartsyAvaH".into()]),
                (3,3) => return Some(vec!["vartsyAmaH".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "vfDu" || dhatu_query == "01.0863" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vartsyati".into()]),
                (1,2) => return Some(vec!["vartsyataH".into()]),
                (1,3) => return Some(vec!["vartsyanti".into()]),
                (2,1) => return Some(vec!["vartsyasi".into()]),
                (2,2) => return Some(vec!["vartsyaTaH".into()]),
                (2,3) => return Some(vec!["vartsyaTa".into()]),
                (3,1) => return Some(vec!["vartsyAmi".into()]),
                (3,2) => return Some(vec!["vartsyAvaH".into()]),
                (3,3) => return Some(vec!["vartsyAmaH".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "syandU" || dhatu_query == "01.0865" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["syantsyati".into()]),
                (1,2) => return Some(vec!["syantsyataH".into()]),
                (1,3) => return Some(vec!["syantsyanti".into()]),
                (2,1) => return Some(vec!["syantsyasi".into()]),
                (2,2) => return Some(vec!["syantsyaTaH".into()]),
                (2,3) => return Some(vec!["syantsyaTa".into()]),
                (3,1) => return Some(vec!["syantsyAmi".into()]),
                (3,2) => return Some(vec!["syantsyAvaH".into()]),
                (3,3) => return Some(vec!["syantsyAmaH".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "kfpU" || dhatu_query == "01.0866" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kalpsyati".into()]),
                (1,2) => return Some(vec!["kalpsyataH".into()]),
                (1,3) => return Some(vec!["kalpsyanti".into()]),
                (2,1) => return Some(vec!["kalpsyasi".into()]),
                (2,2) => return Some(vec!["kalpsyaTaH".into()]),
                (2,3) => return Some(vec!["kalpsyaTa".into()]),
                (3,1) => return Some(vec!["kalpsyAmi".into()]),
                (3,2) => return Some(vec!["kalpsyAvaH".into()]),
                (3,3) => return Some(vec!["kalpsyAmaH".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "yama" || dhatu_query == "01.1031" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yacCati".into()]),
                (1,2) => return Some(vec!["yacCataH".into()]),
                (1,3) => return Some(vec!["yacCanti".into()]),
                (2,1) => return Some(vec!["yacCasi".into()]),
                (2,2) => return Some(vec!["yacCaTaH".into()]),
                (2,3) => return Some(vec!["yacCaTa".into()]),
                (3,1) => return Some(vec!["yacCAmi".into()]),
                (3,2) => return Some(vec!["yacCAvaH".into()]),
                (3,3) => return Some(vec!["yacCAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ayacCat".into()]),
                (1,2) => return Some(vec!["ayacCatAm".into()]),
                (1,3) => return Some(vec!["ayacCan".into()]),
                (2,1) => return Some(vec!["ayacCaH".into()]),
                (2,2) => return Some(vec!["ayacCatam".into()]),
                (2,3) => return Some(vec!["ayacCata".into()]),
                (3,1) => return Some(vec!["ayacCam".into()]),
                (3,2) => return Some(vec!["ayacCAva".into()]),
                (3,3) => return Some(vec!["ayacCAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yacCatAt".into()]),
                (1,2) => return Some(vec!["yacCatAm".into()]),
                (1,3) => return Some(vec!["yacCantu".into()]),
                (2,1) => return Some(vec!["yacCa".into()]),
                (2,2) => return Some(vec!["yacCatam".into()]),
                (2,3) => return Some(vec!["yacCata".into()]),
                (3,1) => return Some(vec!["yacCAni".into()]),
                (3,2) => return Some(vec!["yacCAva".into()]),
                (3,3) => return Some(vec!["yacCAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yaMsyati".into()]),
                (1,2) => return Some(vec!["yaMsyataH".into()]),
                (1,3) => return Some(vec!["yaMsyanti".into()]),
                (2,1) => return Some(vec!["yaMsyasi".into()]),
                (2,2) => return Some(vec!["yaMsyaTaH".into()]),
                (2,3) => return Some(vec!["yaMsyaTa".into()]),
                (3,1) => return Some(vec!["yaMsyAmi".into()]),
                (3,2) => return Some(vec!["yaMsyAvaH".into()]),
                (3,3) => return Some(vec!["yaMsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yacCet".into()]),
                (1,2) => return Some(vec!["yacCetAm".into()]),
                (1,3) => return Some(vec!["yacCeyuH".into()]),
                (2,1) => return Some(vec!["yacCeH".into()]),
                (2,2) => return Some(vec!["yacCetam".into()]),
                (2,3) => return Some(vec!["yacCeta".into()]),
                (3,1) => return Some(vec!["yacCeyam".into()]),
                (3,2) => return Some(vec!["yacCeva".into()]),
                (3,3) => return Some(vec!["yacCema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "RIY" || dhatu_query == "01.1049" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["nayati".into()]),
                (1,2) => return Some(vec!["nayataH".into()]),
                (1,3) => return Some(vec!["nayanti".into()]),
                (2,1) => return Some(vec!["nayasi".into()]),
                (2,2) => return Some(vec!["nayaTaH".into()]),
                (2,3) => return Some(vec!["nayaTa".into()]),
                (3,1) => return Some(vec!["nayAmi".into()]),
                (3,2) => return Some(vec!["nayAvaH".into()]),
                (3,3) => return Some(vec!["nayAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["anayat".into()]),
                (1,2) => return Some(vec!["anayatAm".into()]),
                (1,3) => return Some(vec!["anayan".into()]),
                (2,1) => return Some(vec!["anayaH".into()]),
                (2,2) => return Some(vec!["anayatam".into()]),
                (2,3) => return Some(vec!["anayata".into()]),
                (3,1) => return Some(vec!["anayam".into()]),
                (3,2) => return Some(vec!["anayAva".into()]),
                (3,3) => return Some(vec!["anayAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["nayatAt".into()]),
                (1,2) => return Some(vec!["nayatAm".into()]),
                (1,3) => return Some(vec!["nayantu".into()]),
                (2,1) => return Some(vec!["naya".into()]),
                (2,2) => return Some(vec!["nayatam".into()]),
                (2,3) => return Some(vec!["nayata".into()]),
                (3,1) => return Some(vec!["nayAni".into()]),
                (3,2) => return Some(vec!["nayAva".into()]),
                (3,3) => return Some(vec!["nayAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["nezyati".into()]),
                (1,2) => return Some(vec!["nezyataH".into()]),
                (1,3) => return Some(vec!["nezyanti".into()]),
                (2,1) => return Some(vec!["nezyasi".into()]),
                (2,2) => return Some(vec!["nezyaTaH".into()]),
                (2,3) => return Some(vec!["nezyaTa".into()]),
                (3,1) => return Some(vec!["nezyAmi".into()]),
                (3,2) => return Some(vec!["nezyAvaH".into()]),
                (3,3) => return Some(vec!["nezyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["nayet".into()]),
                (1,2) => return Some(vec!["nayetAm".into()]),
                (1,3) => return Some(vec!["nayeyuH".into()]),
                (2,1) => return Some(vec!["nayeH".into()]),
                (2,2) => return Some(vec!["nayetam".into()]),
                (2,3) => return Some(vec!["nayeta".into()]),
                (3,1) => return Some(vec!["nayeyam".into()]),
                (3,2) => return Some(vec!["nayeva".into()]),
                (3,3) => return Some(vec!["nayema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "ovE" || dhatu_query == "01.1070" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vAyati".into()]),
                (1,2) => return Some(vec!["vAyataH".into()]),
                (1,3) => return Some(vec!["vAyanti".into()]),
                (2,1) => return Some(vec!["vAyasi".into()]),
                (2,2) => return Some(vec!["vAyaTaH".into()]),
                (2,3) => return Some(vec!["vAyaTa".into()]),
                (3,1) => return Some(vec!["vAyAmi".into()]),
                (3,2) => return Some(vec!["vAyAvaH".into()]),
                (3,3) => return Some(vec!["vAyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["avAyat".into()]),
                (1,2) => return Some(vec!["avAyatAm".into()]),
                (1,3) => return Some(vec!["avAyan".into()]),
                (2,1) => return Some(vec!["avAyaH".into()]),
                (2,2) => return Some(vec!["avAyatam".into()]),
                (2,3) => return Some(vec!["avAyata".into()]),
                (3,1) => return Some(vec!["avAyam".into()]),
                (3,2) => return Some(vec!["avAyAva".into()]),
                (3,3) => return Some(vec!["avAyAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vAyatAt".into()]),
                (1,2) => return Some(vec!["vAyatAm".into()]),
                (1,3) => return Some(vec!["vAyantu".into()]),
                (2,1) => return Some(vec!["vAya".into()]),
                (2,2) => return Some(vec!["vAyatam".into()]),
                (2,3) => return Some(vec!["vAyata".into()]),
                (3,1) => return Some(vec!["vAyAni".into()]),
                (3,2) => return Some(vec!["vAyAva".into()]),
                (3,3) => return Some(vec!["vAyAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vAsyati".into()]),
                (1,2) => return Some(vec!["vAsyataH".into()]),
                (1,3) => return Some(vec!["vAsyanti".into()]),
                (2,1) => return Some(vec!["vAsyasi".into()]),
                (2,2) => return Some(vec!["vAsyaTaH".into()]),
                (2,3) => return Some(vec!["vAsyaTa".into()]),
                (3,1) => return Some(vec!["vAsyAmi".into()]),
                (3,2) => return Some(vec!["vAsyAvaH".into()]),
                (3,3) => return Some(vec!["vAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vAyet".into()]),
                (1,2) => return Some(vec!["vAyetAm".into()]),
                (1,3) => return Some(vec!["vAyeyuH".into()]),
                (2,1) => return Some(vec!["vAyeH".into()]),
                (2,2) => return Some(vec!["vAyetam".into()]),
                (2,3) => return Some(vec!["vAyeta".into()]),
                (3,1) => return Some(vec!["vAyeyam".into()]),
                (3,2) => return Some(vec!["vAyeva".into()]),
                (3,3) => return Some(vec!["vAyema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "zRE" || dhatu_query == "01.1072" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["snAyati".into()]),
                (1,2) => return Some(vec!["snAyataH".into()]),
                (1,3) => return Some(vec!["snAyanti".into()]),
                (2,1) => return Some(vec!["snAyasi".into()]),
                (2,2) => return Some(vec!["snAyaTaH".into()]),
                (2,3) => return Some(vec!["snAyaTa".into()]),
                (3,1) => return Some(vec!["snAyAmi".into()]),
                (3,2) => return Some(vec!["snAyAvaH".into()]),
                (3,3) => return Some(vec!["snAyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["asnAyat".into()]),
                (1,2) => return Some(vec!["asnAyatAm".into()]),
                (1,3) => return Some(vec!["asnAyan".into()]),
                (2,1) => return Some(vec!["asnAyaH".into()]),
                (2,2) => return Some(vec!["asnAyatam".into()]),
                (2,3) => return Some(vec!["asnAyata".into()]),
                (3,1) => return Some(vec!["asnAyam".into()]),
                (3,2) => return Some(vec!["asnAyAva".into()]),
                (3,3) => return Some(vec!["asnAyAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["snAyatAt".into()]),
                (1,2) => return Some(vec!["snAyatAm".into()]),
                (1,3) => return Some(vec!["snAyantu".into()]),
                (2,1) => return Some(vec!["snAya".into()]),
                (2,2) => return Some(vec!["snAyatam".into()]),
                (2,3) => return Some(vec!["snAyata".into()]),
                (3,1) => return Some(vec!["snAyAni".into()]),
                (3,2) => return Some(vec!["snAyAva".into()]),
                (3,3) => return Some(vec!["snAyAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["snAsyati".into()]),
                (1,2) => return Some(vec!["snAsyataH".into()]),
                (1,3) => return Some(vec!["snAsyanti".into()]),
                (2,1) => return Some(vec!["snAsyasi".into()]),
                (2,2) => return Some(vec!["snAsyaTaH".into()]),
                (2,3) => return Some(vec!["snAsyaTa".into()]),
                (3,1) => return Some(vec!["snAsyAmi".into()]),
                (3,2) => return Some(vec!["snAsyAvaH".into()]),
                (3,3) => return Some(vec!["snAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["snAyet".into()]),
                (1,2) => return Some(vec!["snAyetAm".into()]),
                (1,3) => return Some(vec!["snAyeyuH".into()]),
                (2,1) => return Some(vec!["snAyeH".into()]),
                (2,2) => return Some(vec!["snAyetam".into()]),
                (2,3) => return Some(vec!["snAyeta".into()]),
                (3,1) => return Some(vec!["snAyeyam".into()]),
                (3,2) => return Some(vec!["snAyeva".into()]),
                (3,3) => return Some(vec!["snAyema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "dEp" || dhatu_query == "01.1073" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dAyati".into()]),
                (1,2) => return Some(vec!["dAyataH".into()]),
                (1,3) => return Some(vec!["dAyanti".into()]),
                (2,1) => return Some(vec!["dAyasi".into()]),
                (2,2) => return Some(vec!["dAyaTaH".into()]),
                (2,3) => return Some(vec!["dAyaTa".into()]),
                (3,1) => return Some(vec!["dAyAmi".into()]),
                (3,2) => return Some(vec!["dAyAvaH".into()]),
                (3,3) => return Some(vec!["dAyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["adAyat".into()]),
                (1,2) => return Some(vec!["adAyatAm".into()]),
                (1,3) => return Some(vec!["adAyan".into()]),
                (2,1) => return Some(vec!["adAyaH".into()]),
                (2,2) => return Some(vec!["adAyatam".into()]),
                (2,3) => return Some(vec!["adAyata".into()]),
                (3,1) => return Some(vec!["adAyam".into()]),
                (3,2) => return Some(vec!["adAyAva".into()]),
                (3,3) => return Some(vec!["adAyAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dAyatAt".into()]),
                (1,2) => return Some(vec!["dAyatAm".into()]),
                (1,3) => return Some(vec!["dAyantu".into()]),
                (2,1) => return Some(vec!["dAya".into()]),
                (2,2) => return Some(vec!["dAyatam".into()]),
                (2,3) => return Some(vec!["dAyata".into()]),
                (3,1) => return Some(vec!["dAyAni".into()]),
                (3,2) => return Some(vec!["dAyAva".into()]),
                (3,3) => return Some(vec!["dAyAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dAsyati".into()]),
                (1,2) => return Some(vec!["dAsyataH".into()]),
                (1,3) => return Some(vec!["dAsyanti".into()]),
                (2,1) => return Some(vec!["dAsyasi".into()]),
                (2,2) => return Some(vec!["dAsyaTaH".into()]),
                (2,3) => return Some(vec!["dAsyaTa".into()]),
                (3,1) => return Some(vec!["dAsyAmi".into()]),
                (3,2) => return Some(vec!["dAsyAvaH".into()]),
                (3,3) => return Some(vec!["dAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dAyet".into()]),
                (1,2) => return Some(vec!["dAyetAm".into()]),
                (1,3) => return Some(vec!["dAyeyuH".into()]),
                (2,1) => return Some(vec!["dAyeH".into()]),
                (2,2) => return Some(vec!["dAyetam".into()]),
                (2,3) => return Some(vec!["dAyeta".into()]),
                (3,1) => return Some(vec!["dAyeyam".into()]),
                (3,2) => return Some(vec!["dAyeva".into()]),
                (3,3) => return Some(vec!["dAyema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "zWA" || dhatu_query == "01.1077" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["tizWati".into()]),
                (1,2) => return Some(vec!["tizWataH".into()]),
                (1,3) => return Some(vec!["tizWanti".into()]),
                (2,1) => return Some(vec!["tizWasi".into()]),
                (2,2) => return Some(vec!["tizWaTaH".into()]),
                (2,3) => return Some(vec!["tizWaTa".into()]),
                (3,1) => return Some(vec!["tizWAmi".into()]),
                (3,2) => return Some(vec!["tizWAvaH".into()]),
                (3,3) => return Some(vec!["tizWAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["atizWat".into()]),
                (1,2) => return Some(vec!["atizWatAm".into()]),
                (1,3) => return Some(vec!["atizWan".into()]),
                (2,1) => return Some(vec!["atizWaH".into()]),
                (2,2) => return Some(vec!["atizWatam".into()]),
                (2,3) => return Some(vec!["atizWata".into()]),
                (3,1) => return Some(vec!["atizWam".into()]),
                (3,2) => return Some(vec!["atizWAva".into()]),
                (3,3) => return Some(vec!["atizWAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["tizWatAt".into()]),
                (1,2) => return Some(vec!["tizWatAm".into()]),
                (1,3) => return Some(vec!["tizWantu".into()]),
                (2,1) => return Some(vec!["tizWa".into()]),
                (2,2) => return Some(vec!["tizWatam".into()]),
                (2,3) => return Some(vec!["tizWata".into()]),
                (3,1) => return Some(vec!["tizWAni".into()]),
                (3,2) => return Some(vec!["tizWAva".into()]),
                (3,3) => return Some(vec!["tizWAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sTAsyati".into()]),
                (1,2) => return Some(vec!["sTAsyataH".into()]),
                (1,3) => return Some(vec!["sTAsyanti".into()]),
                (2,1) => return Some(vec!["sTAsyasi".into()]),
                (2,2) => return Some(vec!["sTAsyaTaH".into()]),
                (2,3) => return Some(vec!["sTAsyaTa".into()]),
                (3,1) => return Some(vec!["sTAsyAmi".into()]),
                (3,2) => return Some(vec!["sTAsyAvaH".into()]),
                (3,3) => return Some(vec!["sTAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["tizWet".into()]),
                (1,2) => return Some(vec!["tizWetAm".into()]),
                (1,3) => return Some(vec!["tizWeyuH".into()]),
                (2,1) => return Some(vec!["tizWeH".into()]),
                (2,2) => return Some(vec!["tizWetam".into()]),
                (2,3) => return Some(vec!["tizWeta".into()]),
                (3,1) => return Some(vec!["tizWeyam".into()]),
                (3,2) => return Some(vec!["tizWeva".into()]),
                (3,3) => return Some(vec!["tizWema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "dAR" || dhatu_query == "01.1079" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yacCati".into()]),
                (1,2) => return Some(vec!["yacCataH".into()]),
                (1,3) => return Some(vec!["yacCanti".into()]),
                (2,1) => return Some(vec!["yacCasi".into()]),
                (2,2) => return Some(vec!["yacCaTaH".into()]),
                (2,3) => return Some(vec!["yacCaTa".into()]),
                (3,1) => return Some(vec!["yacCAmi".into()]),
                (3,2) => return Some(vec!["yacCAvaH".into()]),
                (3,3) => return Some(vec!["yacCAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ayacCat".into()]),
                (1,2) => return Some(vec!["ayacCatAm".into()]),
                (1,3) => return Some(vec!["ayacCan".into()]),
                (2,1) => return Some(vec!["ayacCaH".into()]),
                (2,2) => return Some(vec!["ayacCatam".into()]),
                (2,3) => return Some(vec!["ayacCata".into()]),
                (3,1) => return Some(vec!["ayacCam".into()]),
                (3,2) => return Some(vec!["ayacCAva".into()]),
                (3,3) => return Some(vec!["ayacCAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yacCatAt".into()]),
                (1,2) => return Some(vec!["yacCatAm".into()]),
                (1,3) => return Some(vec!["yacCantu".into()]),
                (2,1) => return Some(vec!["yacCa".into()]),
                (2,2) => return Some(vec!["yacCatam".into()]),
                (2,3) => return Some(vec!["yacCata".into()]),
                (3,1) => return Some(vec!["yacCAni".into()]),
                (3,2) => return Some(vec!["yacCAva".into()]),
                (3,3) => return Some(vec!["yacCAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dAsyati".into()]),
                (1,2) => return Some(vec!["dAsyataH".into()]),
                (1,3) => return Some(vec!["dAsyanti".into()]),
                (2,1) => return Some(vec!["dAsyasi".into()]),
                (2,2) => return Some(vec!["dAsyaTaH".into()]),
                (2,3) => return Some(vec!["dAsyaTa".into()]),
                (3,1) => return Some(vec!["dAsyAmi".into()]),
                (3,2) => return Some(vec!["dAsyAvaH".into()]),
                (3,3) => return Some(vec!["dAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yacCet".into()]),
                (1,2) => return Some(vec!["yacCetAm".into()]),
                (1,3) => return Some(vec!["yacCeyuH".into()]),
                (2,1) => return Some(vec!["yacCeH".into()]),
                (2,2) => return Some(vec!["yacCetam".into()]),
                (2,3) => return Some(vec!["yacCeta".into()]),
                (3,1) => return Some(vec!["yacCeyam".into()]),
                (3,2) => return Some(vec!["yacCeva".into()]),
                (3,3) => return Some(vec!["yacCema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "f" || dhatu_query == "01.1086" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["fcCati".into()]),
                (1,2) => return Some(vec!["fcCataH".into()]),
                (1,3) => return Some(vec!["fcCanti".into()]),
                (2,1) => return Some(vec!["fcCasi".into()]),
                (2,2) => return Some(vec!["fcCaTaH".into()]),
                (2,3) => return Some(vec!["fcCaTa".into()]),
                (3,1) => return Some(vec!["fcCAmi".into()]),
                (3,2) => return Some(vec!["fcCAvaH".into()]),
                (3,3) => return Some(vec!["fcCAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ArcCat".into()]),
                (1,2) => return Some(vec!["ArcCatAm".into()]),
                (1,3) => return Some(vec!["ArcCan".into()]),
                (2,1) => return Some(vec!["ArcCaH".into()]),
                (2,2) => return Some(vec!["ArcCatam".into()]),
                (2,3) => return Some(vec!["ArcCata".into()]),
                (3,1) => return Some(vec!["ArcCam".into()]),
                (3,2) => return Some(vec!["ArcCAva".into()]),
                (3,3) => return Some(vec!["ArcCAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["fcCatAt".into()]),
                (1,2) => return Some(vec!["fcCatAm".into()]),
                (1,3) => return Some(vec!["fcCantu".into()]),
                (2,1) => return Some(vec!["fcCa".into()]),
                (2,2) => return Some(vec!["fcCatam".into()]),
                (2,3) => return Some(vec!["fcCata".into()]),
                (3,1) => return Some(vec!["fcCAni".into()]),
                (3,2) => return Some(vec!["fcCAva".into()]),
                (3,3) => return Some(vec!["fcCAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["arizyati".into()]),
                (1,2) => return Some(vec!["arizyataH".into()]),
                (1,3) => return Some(vec!["arizyanti".into()]),
                (2,1) => return Some(vec!["arizyasi".into()]),
                (2,2) => return Some(vec!["arizyaTaH".into()]),
                (2,3) => return Some(vec!["arizyaTa".into()]),
                (3,1) => return Some(vec!["arizyAmi".into()]),
                (3,2) => return Some(vec!["arizyAvaH".into()]),
                (3,3) => return Some(vec!["arizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["fcCet".into()]),
                (1,2) => return Some(vec!["fcCetAm".into()]),
                (1,3) => return Some(vec!["fcCeyuH".into()]),
                (2,1) => return Some(vec!["fcCeH".into()]),
                (2,2) => return Some(vec!["fcCetam".into()]),
                (2,3) => return Some(vec!["fcCeta".into()]),
                (3,1) => return Some(vec!["fcCeyam".into()]),
                (3,2) => return Some(vec!["fcCeva".into()]),
                (3,3) => return Some(vec!["fcCema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "sru" || dhatu_query == "01.1090" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sravati".into()]),
                (1,2) => return Some(vec!["sravataH".into()]),
                (1,3) => return Some(vec!["sravanti".into()]),
                (2,1) => return Some(vec!["sravasi".into()]),
                (2,2) => return Some(vec!["sravaTaH".into()]),
                (2,3) => return Some(vec!["sravaTa".into()]),
                (3,1) => return Some(vec!["sravAmi".into()]),
                (3,2) => return Some(vec!["sravAvaH".into()]),
                (3,3) => return Some(vec!["sravAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["asravat".into()]),
                (1,2) => return Some(vec!["asravatAm".into()]),
                (1,3) => return Some(vec!["asravan".into()]),
                (2,1) => return Some(vec!["asravaH".into()]),
                (2,2) => return Some(vec!["asravatam".into()]),
                (2,3) => return Some(vec!["asravata".into()]),
                (3,1) => return Some(vec!["asravam".into()]),
                (3,2) => return Some(vec!["asravAva".into()]),
                (3,3) => return Some(vec!["asravAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sravatAt".into()]),
                (1,2) => return Some(vec!["sravatAm".into()]),
                (1,3) => return Some(vec!["sravantu".into()]),
                (2,1) => return Some(vec!["srava".into()]),
                (2,2) => return Some(vec!["sravatam".into()]),
                (2,3) => return Some(vec!["sravata".into()]),
                (3,1) => return Some(vec!["sravARi".into()]),
                (3,2) => return Some(vec!["sravAva".into()]),
                (3,3) => return Some(vec!["sravAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["srozyati".into()]),
                (1,2) => return Some(vec!["srozyataH".into()]),
                (1,3) => return Some(vec!["srozyanti".into()]),
                (2,1) => return Some(vec!["srozyasi".into()]),
                (2,2) => return Some(vec!["srozyaTaH".into()]),
                (2,3) => return Some(vec!["srozyaTa".into()]),
                (3,1) => return Some(vec!["srozyAmi".into()]),
                (3,2) => return Some(vec!["srozyAvaH".into()]),
                (3,3) => return Some(vec!["srozyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sravet".into()]),
                (1,2) => return Some(vec!["sravetAm".into()]),
                (1,3) => return Some(vec!["sraveyuH".into()]),
                (2,1) => return Some(vec!["sraveH".into()]),
                (2,2) => return Some(vec!["sravetam".into()]),
                (2,3) => return Some(vec!["sraveta".into()]),
                (3,1) => return Some(vec!["sraveyam".into()]),
                (3,2) => return Some(vec!["sraveva".into()]),
                (3,3) => return Some(vec!["sravema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "zu" || dhatu_query == "01.1091" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["savati".into()]),
                (1,2) => return Some(vec!["savataH".into()]),
                (1,3) => return Some(vec!["savanti".into()]),
                (2,1) => return Some(vec!["savasi".into()]),
                (2,2) => return Some(vec!["savaTaH".into()]),
                (2,3) => return Some(vec!["savaTa".into()]),
                (3,1) => return Some(vec!["savAmi".into()]),
                (3,2) => return Some(vec!["savAvaH".into()]),
                (3,3) => return Some(vec!["savAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["asavat".into()]),
                (1,2) => return Some(vec!["asavatAm".into()]),
                (1,3) => return Some(vec!["asavan".into()]),
                (2,1) => return Some(vec!["asavaH".into()]),
                (2,2) => return Some(vec!["asavatam".into()]),
                (2,3) => return Some(vec!["asavata".into()]),
                (3,1) => return Some(vec!["asavam".into()]),
                (3,2) => return Some(vec!["asavAva".into()]),
                (3,3) => return Some(vec!["asavAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["savatAt".into()]),
                (1,2) => return Some(vec!["savatAm".into()]),
                (1,3) => return Some(vec!["savantu".into()]),
                (2,1) => return Some(vec!["sava".into()]),
                (2,2) => return Some(vec!["savatam".into()]),
                (2,3) => return Some(vec!["savata".into()]),
                (3,1) => return Some(vec!["savAni".into()]),
                (3,2) => return Some(vec!["savAva".into()]),
                (3,3) => return Some(vec!["savAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sozyati".into()]),
                (1,2) => return Some(vec!["sozyataH".into()]),
                (1,3) => return Some(vec!["sozyanti".into()]),
                (2,1) => return Some(vec!["sozyasi".into()]),
                (2,2) => return Some(vec!["sozyaTaH".into()]),
                (2,3) => return Some(vec!["sozyaTa".into()]),
                (3,1) => return Some(vec!["sozyAmi".into()]),
                (3,2) => return Some(vec!["sozyAvaH".into()]),
                (3,3) => return Some(vec!["sozyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["savet".into()]),
                (1,2) => return Some(vec!["savetAm".into()]),
                (1,3) => return Some(vec!["saveyuH".into()]),
                (2,1) => return Some(vec!["saveH".into()]),
                (2,2) => return Some(vec!["savetam".into()]),
                (2,3) => return Some(vec!["saveta".into()]),
                (3,1) => return Some(vec!["saveyam".into()]),
                (3,2) => return Some(vec!["saveva".into()]),
                (3,3) => return Some(vec!["savema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "Sru" || dhatu_query == "01.1092" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["SfRoti".into()]),
                (1,2) => return Some(vec!["SfRutaH".into()]),
                (1,3) => return Some(vec!["SfRvanti".into()]),
                (2,1) => return Some(vec!["SfRozi".into()]),
                (2,2) => return Some(vec!["SfRuTaH".into()]),
                (2,3) => return Some(vec!["SfRuTa".into()]),
                (3,1) => return Some(vec!["SfRomi".into()]),
                (3,2) => return Some(vec!["SfRuvaH".into()]),
                (3,3) => return Some(vec!["SfRumaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aSfRot".into()]),
                (1,2) => return Some(vec!["aSfRutAm".into()]),
                (1,3) => return Some(vec!["aSfRvan".into()]),
                (2,1) => return Some(vec!["aSfRoH".into()]),
                (2,2) => return Some(vec!["aSfRutam".into()]),
                (2,3) => return Some(vec!["aSfRuta".into()]),
                (3,1) => return Some(vec!["aSfRavam".into()]),
                (3,2) => return Some(vec!["aSfRuva".into()]),
                (3,3) => return Some(vec!["aSfRuma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["SfRutAt".into()]),
                (1,2) => return Some(vec!["SfRutAm".into()]),
                (1,3) => return Some(vec!["SfRvantu".into()]),
                (2,1) => return Some(vec!["SfRu".into()]),
                (2,2) => return Some(vec!["SfRutam".into()]),
                (2,3) => return Some(vec!["SfRuta".into()]),
                (3,1) => return Some(vec!["SfRavAni".into()]),
                (3,2) => return Some(vec!["SfRavAva".into()]),
                (3,3) => return Some(vec!["SfRavAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["Srozyati".into()]),
                (1,2) => return Some(vec!["SrozyataH".into()]),
                (1,3) => return Some(vec!["Srozyanti".into()]),
                (2,1) => return Some(vec!["Srozyasi".into()]),
                (2,2) => return Some(vec!["SrozyaTaH".into()]),
                (2,3) => return Some(vec!["SrozyaTa".into()]),
                (3,1) => return Some(vec!["SrozyAmi".into()]),
                (3,2) => return Some(vec!["SrozyAvaH".into()]),
                (3,3) => return Some(vec!["SrozyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["SfRuyAt".into()]),
                (1,2) => return Some(vec!["SfRuyAtAm".into()]),
                (1,3) => return Some(vec!["SfRuyuH".into()]),
                (2,1) => return Some(vec!["SfRuyAH".into()]),
                (2,2) => return Some(vec!["SfRuyAtam".into()]),
                (2,3) => return Some(vec!["SfRuyAta".into()]),
                (3,1) => return Some(vec!["SfRuyAm".into()]),
                (3,2) => return Some(vec!["SfRuyAva".into()]),
                (3,3) => return Some(vec!["SfRuyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "Dru" || dhatu_query == "01.1093" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["Dravati".into()]),
                (1,2) => return Some(vec!["DravataH".into()]),
                (1,3) => return Some(vec!["Dravanti".into()]),
                (2,1) => return Some(vec!["Dravasi".into()]),
                (2,2) => return Some(vec!["DravaTaH".into()]),
                (2,3) => return Some(vec!["DravaTa".into()]),
                (3,1) => return Some(vec!["DravAmi".into()]),
                (3,2) => return Some(vec!["DravAvaH".into()]),
                (3,3) => return Some(vec!["DravAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aDravat".into()]),
                (1,2) => return Some(vec!["aDravatAm".into()]),
                (1,3) => return Some(vec!["aDravan".into()]),
                (2,1) => return Some(vec!["aDravaH".into()]),
                (2,2) => return Some(vec!["aDravatam".into()]),
                (2,3) => return Some(vec!["aDravata".into()]),
                (3,1) => return Some(vec!["aDravam".into()]),
                (3,2) => return Some(vec!["aDravAva".into()]),
                (3,3) => return Some(vec!["aDravAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["DravatAt".into()]),
                (1,2) => return Some(vec!["DravatAm".into()]),
                (1,3) => return Some(vec!["Dravantu".into()]),
                (2,1) => return Some(vec!["Drava".into()]),
                (2,2) => return Some(vec!["Dravatam".into()]),
                (2,3) => return Some(vec!["Dravata".into()]),
                (3,1) => return Some(vec!["DravARi".into()]),
                (3,2) => return Some(vec!["DravAva".into()]),
                (3,3) => return Some(vec!["DravAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["Drozyati".into()]),
                (1,2) => return Some(vec!["DrozyataH".into()]),
                (1,3) => return Some(vec!["Drozyanti".into()]),
                (2,1) => return Some(vec!["Drozyasi".into()]),
                (2,2) => return Some(vec!["DrozyaTaH".into()]),
                (2,3) => return Some(vec!["DrozyaTa".into()]),
                (3,1) => return Some(vec!["DrozyAmi".into()]),
                (3,2) => return Some(vec!["DrozyAvaH".into()]),
                (3,3) => return Some(vec!["DrozyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["Dravet".into()]),
                (1,2) => return Some(vec!["DravetAm".into()]),
                (1,3) => return Some(vec!["DraveyuH".into()]),
                (2,1) => return Some(vec!["DraveH".into()]),
                (2,2) => return Some(vec!["Dravetam".into()]),
                (2,3) => return Some(vec!["Draveta".into()]),
                (3,1) => return Some(vec!["Draveyam".into()]),
                (3,2) => return Some(vec!["Draveva".into()]),
                (3,3) => return Some(vec!["Dravema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "du" || dhatu_query == "01.1094" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["davati".into()]),
                (1,2) => return Some(vec!["davataH".into()]),
                (1,3) => return Some(vec!["davanti".into()]),
                (2,1) => return Some(vec!["davasi".into()]),
                (2,2) => return Some(vec!["davaTaH".into()]),
                (2,3) => return Some(vec!["davaTa".into()]),
                (3,1) => return Some(vec!["davAmi".into()]),
                (3,2) => return Some(vec!["davAvaH".into()]),
                (3,3) => return Some(vec!["davAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["adavat".into()]),
                (1,2) => return Some(vec!["adavatAm".into()]),
                (1,3) => return Some(vec!["adavan".into()]),
                (2,1) => return Some(vec!["adavaH".into()]),
                (2,2) => return Some(vec!["adavatam".into()]),
                (2,3) => return Some(vec!["adavata".into()]),
                (3,1) => return Some(vec!["adavam".into()]),
                (3,2) => return Some(vec!["adavAva".into()]),
                (3,3) => return Some(vec!["adavAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["davatAt".into()]),
                (1,2) => return Some(vec!["davatAm".into()]),
                (1,3) => return Some(vec!["davantu".into()]),
                (2,1) => return Some(vec!["dava".into()]),
                (2,2) => return Some(vec!["davatam".into()]),
                (2,3) => return Some(vec!["davata".into()]),
                (3,1) => return Some(vec!["davAni".into()]),
                (3,2) => return Some(vec!["davAva".into()]),
                (3,3) => return Some(vec!["davAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dozyati".into()]),
                (1,2) => return Some(vec!["dozyataH".into()]),
                (1,3) => return Some(vec!["dozyanti".into()]),
                (2,1) => return Some(vec!["dozyasi".into()]),
                (2,2) => return Some(vec!["dozyaTaH".into()]),
                (2,3) => return Some(vec!["dozyaTa".into()]),
                (3,1) => return Some(vec!["dozyAmi".into()]),
                (3,2) => return Some(vec!["dozyAvaH".into()]),
                (3,3) => return Some(vec!["dozyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["davet".into()]),
                (1,2) => return Some(vec!["davetAm".into()]),
                (1,3) => return Some(vec!["daveyuH".into()]),
                (2,1) => return Some(vec!["daveH".into()]),
                (2,2) => return Some(vec!["davetam".into()]),
                (2,3) => return Some(vec!["daveta".into()]),
                (3,1) => return Some(vec!["daveyam".into()]),
                (3,2) => return Some(vec!["daveva".into()]),
                (3,3) => return Some(vec!["davema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "dru" || dhatu_query == "01.1095" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dravati".into()]),
                (1,2) => return Some(vec!["dravataH".into()]),
                (1,3) => return Some(vec!["dravanti".into()]),
                (2,1) => return Some(vec!["dravasi".into()]),
                (2,2) => return Some(vec!["dravaTaH".into()]),
                (2,3) => return Some(vec!["dravaTa".into()]),
                (3,1) => return Some(vec!["dravAmi".into()]),
                (3,2) => return Some(vec!["dravAvaH".into()]),
                (3,3) => return Some(vec!["dravAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["adravat".into()]),
                (1,2) => return Some(vec!["adravatAm".into()]),
                (1,3) => return Some(vec!["adravan".into()]),
                (2,1) => return Some(vec!["adravaH".into()]),
                (2,2) => return Some(vec!["adravatam".into()]),
                (2,3) => return Some(vec!["adravata".into()]),
                (3,1) => return Some(vec!["adravam".into()]),
                (3,2) => return Some(vec!["adravAva".into()]),
                (3,3) => return Some(vec!["adravAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dravatAt".into()]),
                (1,2) => return Some(vec!["dravatAm".into()]),
                (1,3) => return Some(vec!["dravantu".into()]),
                (2,1) => return Some(vec!["drava".into()]),
                (2,2) => return Some(vec!["dravatam".into()]),
                (2,3) => return Some(vec!["dravata".into()]),
                (3,1) => return Some(vec!["dravARi".into()]),
                (3,2) => return Some(vec!["dravAva".into()]),
                (3,3) => return Some(vec!["dravAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["drozyati".into()]),
                (1,2) => return Some(vec!["drozyataH".into()]),
                (1,3) => return Some(vec!["drozyanti".into()]),
                (2,1) => return Some(vec!["drozyasi".into()]),
                (2,2) => return Some(vec!["drozyaTaH".into()]),
                (2,3) => return Some(vec!["drozyaTa".into()]),
                (3,1) => return Some(vec!["drozyAmi".into()]),
                (3,2) => return Some(vec!["drozyAvaH".into()]),
                (3,3) => return Some(vec!["drozyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dravet".into()]),
                (1,2) => return Some(vec!["dravetAm".into()]),
                (1,3) => return Some(vec!["draveyuH".into()]),
                (2,1) => return Some(vec!["draveH".into()]),
                (2,2) => return Some(vec!["dravetam".into()]),
                (2,3) => return Some(vec!["draveta".into()]),
                (3,1) => return Some(vec!["draveyam".into()]),
                (3,2) => return Some(vec!["draveva".into()]),
                (3,3) => return Some(vec!["dravema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "skandir" || dhatu_query == "01.1134" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["skandati".into()]),
                (1,2) => return Some(vec!["skandataH".into()]),
                (1,3) => return Some(vec!["skandanti".into()]),
                (2,1) => return Some(vec!["skandasi".into()]),
                (2,2) => return Some(vec!["skandaTaH".into()]),
                (2,3) => return Some(vec!["skandaTa".into()]),
                (3,1) => return Some(vec!["skandAmi".into()]),
                (3,2) => return Some(vec!["skandAvaH".into()]),
                (3,3) => return Some(vec!["skandAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["askandat".into()]),
                (1,2) => return Some(vec!["askandatAm".into()]),
                (1,3) => return Some(vec!["askandan".into()]),
                (2,1) => return Some(vec!["askandaH".into()]),
                (2,2) => return Some(vec!["askandatam".into()]),
                (2,3) => return Some(vec!["askandata".into()]),
                (3,1) => return Some(vec!["askandam".into()]),
                (3,2) => return Some(vec!["askandAva".into()]),
                (3,3) => return Some(vec!["askandAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["skandatAt".into()]),
                (1,2) => return Some(vec!["skandatAm".into()]),
                (1,3) => return Some(vec!["skandantu".into()]),
                (2,1) => return Some(vec!["skanda".into()]),
                (2,2) => return Some(vec!["skandatam".into()]),
                (2,3) => return Some(vec!["skandata".into()]),
                (3,1) => return Some(vec!["skandAni".into()]),
                (3,2) => return Some(vec!["skandAva".into()]),
                (3,3) => return Some(vec!["skandAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["skantsyati".into()]),
                (1,2) => return Some(vec!["skantsyataH".into()]),
                (1,3) => return Some(vec!["skantsyanti".into()]),
                (2,1) => return Some(vec!["skantsyasi".into()]),
                (2,2) => return Some(vec!["skantsyaTaH".into()]),
                (2,3) => return Some(vec!["skantsyaTa".into()]),
                (3,1) => return Some(vec!["skantsyAmi".into()]),
                (3,2) => return Some(vec!["skantsyAvaH".into()]),
                (3,3) => return Some(vec!["skantsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["skandet".into()]),
                (1,2) => return Some(vec!["skandetAm".into()]),
                (1,3) => return Some(vec!["skandeyuH".into()]),
                (2,1) => return Some(vec!["skandeH".into()]),
                (2,2) => return Some(vec!["skandetam".into()]),
                (2,3) => return Some(vec!["skandeta".into()]),
                (3,1) => return Some(vec!["skandeyam".into()]),
                (3,2) => return Some(vec!["skandeva".into()]),
                (3,3) => return Some(vec!["skandema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "Rama" || dhatu_query == "01.1136" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["namati".into()]),
                (1,2) => return Some(vec!["namataH".into()]),
                (1,3) => return Some(vec!["namanti".into()]),
                (2,1) => return Some(vec!["namasi".into()]),
                (2,2) => return Some(vec!["namaTaH".into()]),
                (2,3) => return Some(vec!["namaTa".into()]),
                (3,1) => return Some(vec!["namAmi".into()]),
                (3,2) => return Some(vec!["namAvaH".into()]),
                (3,3) => return Some(vec!["namAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["anamat".into()]),
                (1,2) => return Some(vec!["anamatAm".into()]),
                (1,3) => return Some(vec!["anaman".into()]),
                (2,1) => return Some(vec!["anamaH".into()]),
                (2,2) => return Some(vec!["anamatam".into()]),
                (2,3) => return Some(vec!["anamata".into()]),
                (3,1) => return Some(vec!["anamam".into()]),
                (3,2) => return Some(vec!["anamAva".into()]),
                (3,3) => return Some(vec!["anamAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["namatAt".into()]),
                (1,2) => return Some(vec!["namatAm".into()]),
                (1,3) => return Some(vec!["namantu".into()]),
                (2,1) => return Some(vec!["nama".into()]),
                (2,2) => return Some(vec!["namatam".into()]),
                (2,3) => return Some(vec!["namata".into()]),
                (3,1) => return Some(vec!["namAni".into()]),
                (3,2) => return Some(vec!["namAva".into()]),
                (3,3) => return Some(vec!["namAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["naMsyati".into()]),
                (1,2) => return Some(vec!["naMsyataH".into()]),
                (1,3) => return Some(vec!["naMsyanti".into()]),
                (2,1) => return Some(vec!["naMsyasi".into()]),
                (2,2) => return Some(vec!["naMsyaTaH".into()]),
                (2,3) => return Some(vec!["naMsyaTa".into()]),
                (3,1) => return Some(vec!["naMsyAmi".into()]),
                (3,2) => return Some(vec!["naMsyAvaH".into()]),
                (3,3) => return Some(vec!["naMsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["namet".into()]),
                (1,2) => return Some(vec!["nametAm".into()]),
                (1,3) => return Some(vec!["nameyuH".into()]),
                (2,1) => return Some(vec!["nameH".into()]),
                (2,2) => return Some(vec!["nametam".into()]),
                (2,3) => return Some(vec!["nameta".into()]),
                (3,1) => return Some(vec!["nameyam".into()]),
                (3,2) => return Some(vec!["nameva".into()]),
                (3,3) => return Some(vec!["namema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "yama" || dhatu_query == "01.1139" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yacCati".into()]),
                (1,2) => return Some(vec!["yacCataH".into()]),
                (1,3) => return Some(vec!["yacCanti".into()]),
                (2,1) => return Some(vec!["yacCasi".into()]),
                (2,2) => return Some(vec!["yacCaTaH".into()]),
                (2,3) => return Some(vec!["yacCaTa".into()]),
                (3,1) => return Some(vec!["yacCAmi".into()]),
                (3,2) => return Some(vec!["yacCAvaH".into()]),
                (3,3) => return Some(vec!["yacCAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ayacCat".into()]),
                (1,2) => return Some(vec!["ayacCatAm".into()]),
                (1,3) => return Some(vec!["ayacCan".into()]),
                (2,1) => return Some(vec!["ayacCaH".into()]),
                (2,2) => return Some(vec!["ayacCatam".into()]),
                (2,3) => return Some(vec!["ayacCata".into()]),
                (3,1) => return Some(vec!["ayacCam".into()]),
                (3,2) => return Some(vec!["ayacCAva".into()]),
                (3,3) => return Some(vec!["ayacCAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yacCatAt".into()]),
                (1,2) => return Some(vec!["yacCatAm".into()]),
                (1,3) => return Some(vec!["yacCantu".into()]),
                (2,1) => return Some(vec!["yacCa".into()]),
                (2,2) => return Some(vec!["yacCatam".into()]),
                (2,3) => return Some(vec!["yacCata".into()]),
                (3,1) => return Some(vec!["yacCAni".into()]),
                (3,2) => return Some(vec!["yacCAva".into()]),
                (3,3) => return Some(vec!["yacCAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yaMsyati".into()]),
                (1,2) => return Some(vec!["yaMsyataH".into()]),
                (1,3) => return Some(vec!["yaMsyanti".into()]),
                (2,1) => return Some(vec!["yaMsyasi".into()]),
                (2,2) => return Some(vec!["yaMsyaTaH".into()]),
                (2,3) => return Some(vec!["yaMsyaTa".into()]),
                (3,1) => return Some(vec!["yaMsyAmi".into()]),
                (3,2) => return Some(vec!["yaMsyAvaH".into()]),
                (3,3) => return Some(vec!["yaMsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yacCet".into()]),
                (1,2) => return Some(vec!["yacCetAm".into()]),
                (1,3) => return Some(vec!["yacCeyuH".into()]),
                (2,1) => return Some(vec!["yacCeH".into()]),
                (2,2) => return Some(vec!["yacCetam".into()]),
                (2,3) => return Some(vec!["yacCeta".into()]),
                (3,1) => return Some(vec!["yacCeyam".into()]),
                (3,2) => return Some(vec!["yacCeva".into()]),
                (3,3) => return Some(vec!["yacCema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "tyaja" || dhatu_query == "01.1141" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["tyajati".into()]),
                (1,2) => return Some(vec!["tyajataH".into()]),
                (1,3) => return Some(vec!["tyajanti".into()]),
                (2,1) => return Some(vec!["tyajasi".into()]),
                (2,2) => return Some(vec!["tyajaTaH".into()]),
                (2,3) => return Some(vec!["tyajaTa".into()]),
                (3,1) => return Some(vec!["tyajAmi".into()]),
                (3,2) => return Some(vec!["tyajAvaH".into()]),
                (3,3) => return Some(vec!["tyajAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["atyajat".into()]),
                (1,2) => return Some(vec!["atyajatAm".into()]),
                (1,3) => return Some(vec!["atyajan".into()]),
                (2,1) => return Some(vec!["atyajaH".into()]),
                (2,2) => return Some(vec!["atyajatam".into()]),
                (2,3) => return Some(vec!["atyajata".into()]),
                (3,1) => return Some(vec!["atyajam".into()]),
                (3,2) => return Some(vec!["atyajAva".into()]),
                (3,3) => return Some(vec!["atyajAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["tyajatAt".into()]),
                (1,2) => return Some(vec!["tyajatAm".into()]),
                (1,3) => return Some(vec!["tyajantu".into()]),
                (2,1) => return Some(vec!["tyaja".into()]),
                (2,2) => return Some(vec!["tyajatam".into()]),
                (2,3) => return Some(vec!["tyajata".into()]),
                (3,1) => return Some(vec!["tyajAni".into()]),
                (3,2) => return Some(vec!["tyajAva".into()]),
                (3,3) => return Some(vec!["tyajAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["tyakzyati".into()]),
                (1,2) => return Some(vec!["tyakzyataH".into()]),
                (1,3) => return Some(vec!["tyakzyanti".into()]),
                (2,1) => return Some(vec!["tyakzyasi".into()]),
                (2,2) => return Some(vec!["tyakzyaTaH".into()]),
                (2,3) => return Some(vec!["tyakzyaTa".into()]),
                (3,1) => return Some(vec!["tyakzyAmi".into()]),
                (3,2) => return Some(vec!["tyakzyAvaH".into()]),
                (3,3) => return Some(vec!["tyakzyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["tyajet".into()]),
                (1,2) => return Some(vec!["tyajetAm".into()]),
                (1,3) => return Some(vec!["tyajeyuH".into()]),
                (2,1) => return Some(vec!["tyajeH".into()]),
                (2,2) => return Some(vec!["tyajetam".into()]),
                (2,3) => return Some(vec!["tyajeta".into()]),
                (3,1) => return Some(vec!["tyajeyam".into()]),
                (3,2) => return Some(vec!["tyajeva".into()]),
                (3,3) => return Some(vec!["tyajema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "zanja" || dhatu_query == "01.1142" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sajati".into()]),
                (1,2) => return Some(vec!["sajataH".into()]),
                (1,3) => return Some(vec!["sajanti".into()]),
                (2,1) => return Some(vec!["sajasi".into()]),
                (2,2) => return Some(vec!["sajaTaH".into()]),
                (2,3) => return Some(vec!["sajaTa".into()]),
                (3,1) => return Some(vec!["sajAmi".into()]),
                (3,2) => return Some(vec!["sajAvaH".into()]),
                (3,3) => return Some(vec!["sajAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["asajat".into()]),
                (1,2) => return Some(vec!["asajatAm".into()]),
                (1,3) => return Some(vec!["asajan".into()]),
                (2,1) => return Some(vec!["asajaH".into()]),
                (2,2) => return Some(vec!["asajatam".into()]),
                (2,3) => return Some(vec!["asajata".into()]),
                (3,1) => return Some(vec!["asajam".into()]),
                (3,2) => return Some(vec!["asajAva".into()]),
                (3,3) => return Some(vec!["asajAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sajatAt".into()]),
                (1,2) => return Some(vec!["sajatAm".into()]),
                (1,3) => return Some(vec!["sajantu".into()]),
                (2,1) => return Some(vec!["saja".into()]),
                (2,2) => return Some(vec!["sajatam".into()]),
                (2,3) => return Some(vec!["sajata".into()]),
                (3,1) => return Some(vec!["sajAni".into()]),
                (3,2) => return Some(vec!["sajAva".into()]),
                (3,3) => return Some(vec!["sajAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["saNkzyati".into()]),
                (1,2) => return Some(vec!["saNkzyataH".into()]),
                (1,3) => return Some(vec!["saNkzyanti".into()]),
                (2,1) => return Some(vec!["saNkzyasi".into()]),
                (2,2) => return Some(vec!["saNkzyaTaH".into()]),
                (2,3) => return Some(vec!["saNkzyaTa".into()]),
                (3,1) => return Some(vec!["saNkzyAmi".into()]),
                (3,2) => return Some(vec!["saNkzyAvaH".into()]),
                (3,3) => return Some(vec!["saNkzyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sajet".into()]),
                (1,2) => return Some(vec!["sajetAm".into()]),
                (1,3) => return Some(vec!["sajeyuH".into()]),
                (2,1) => return Some(vec!["sajeH".into()]),
                (2,2) => return Some(vec!["sajetam".into()]),
                (2,3) => return Some(vec!["sajeta".into()]),
                (3,1) => return Some(vec!["sajeyam".into()]),
                (3,2) => return Some(vec!["sajeva".into()]),
                (3,3) => return Some(vec!["sajema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "dfSir" || dhatu_query == "01.1143" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["paSyati".into()]),
                (1,2) => return Some(vec!["paSyataH".into()]),
                (1,3) => return Some(vec!["paSyanti".into()]),
                (2,1) => return Some(vec!["paSyasi".into()]),
                (2,2) => return Some(vec!["paSyaTaH".into()]),
                (2,3) => return Some(vec!["paSyaTa".into()]),
                (3,1) => return Some(vec!["paSyAmi".into()]),
                (3,2) => return Some(vec!["paSyAvaH".into()]),
                (3,3) => return Some(vec!["paSyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["apaSyat".into()]),
                (1,2) => return Some(vec!["apaSyatAm".into()]),
                (1,3) => return Some(vec!["apaSyan".into()]),
                (2,1) => return Some(vec!["apaSyaH".into()]),
                (2,2) => return Some(vec!["apaSyatam".into()]),
                (2,3) => return Some(vec!["apaSyata".into()]),
                (3,1) => return Some(vec!["apaSyam".into()]),
                (3,2) => return Some(vec!["apaSyAva".into()]),
                (3,3) => return Some(vec!["apaSyAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["paSyatAt".into()]),
                (1,2) => return Some(vec!["paSyatAm".into()]),
                (1,3) => return Some(vec!["paSyantu".into()]),
                (2,1) => return Some(vec!["paSya".into()]),
                (2,2) => return Some(vec!["paSyatam".into()]),
                (2,3) => return Some(vec!["paSyata".into()]),
                (3,1) => return Some(vec!["paSyAni".into()]),
                (3,2) => return Some(vec!["paSyAva".into()]),
                (3,3) => return Some(vec!["paSyAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["drakzyati".into()]),
                (1,2) => return Some(vec!["drakzyataH".into()]),
                (1,3) => return Some(vec!["drakzyanti".into()]),
                (2,1) => return Some(vec!["drakzyasi".into()]),
                (2,2) => return Some(vec!["drakzyaTaH".into()]),
                (2,3) => return Some(vec!["drakzyaTa".into()]),
                (3,1) => return Some(vec!["drakzyAmi".into()]),
                (3,2) => return Some(vec!["drakzyAvaH".into()]),
                (3,3) => return Some(vec!["drakzyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["paSyet".into()]),
                (1,2) => return Some(vec!["paSyetAm".into()]),
                (1,3) => return Some(vec!["paSyeyuH".into()]),
                (2,1) => return Some(vec!["paSyeH".into()]),
                (2,2) => return Some(vec!["paSyetam".into()]),
                (2,3) => return Some(vec!["paSyeta".into()]),
                (3,1) => return Some(vec!["paSyeyam".into()]),
                (3,2) => return Some(vec!["paSyeva".into()]),
                (3,3) => return Some(vec!["paSyema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "danSa" || dhatu_query == "01.1144" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["daSati".into()]),
                (1,2) => return Some(vec!["daSataH".into()]),
                (1,3) => return Some(vec!["daSanti".into()]),
                (2,1) => return Some(vec!["daSasi".into()]),
                (2,2) => return Some(vec!["daSaTaH".into()]),
                (2,3) => return Some(vec!["daSaTa".into()]),
                (3,1) => return Some(vec!["daSAmi".into()]),
                (3,2) => return Some(vec!["daSAvaH".into()]),
                (3,3) => return Some(vec!["daSAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["adaSat".into()]),
                (1,2) => return Some(vec!["adaSatAm".into()]),
                (1,3) => return Some(vec!["adaSan".into()]),
                (2,1) => return Some(vec!["adaSaH".into()]),
                (2,2) => return Some(vec!["adaSatam".into()]),
                (2,3) => return Some(vec!["adaSata".into()]),
                (3,1) => return Some(vec!["adaSam".into()]),
                (3,2) => return Some(vec!["adaSAva".into()]),
                (3,3) => return Some(vec!["adaSAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["daSatAt".into()]),
                (1,2) => return Some(vec!["daSatAm".into()]),
                (1,3) => return Some(vec!["daSantu".into()]),
                (2,1) => return Some(vec!["daSa".into()]),
                (2,2) => return Some(vec!["daSatam".into()]),
                (2,3) => return Some(vec!["daSata".into()]),
                (3,1) => return Some(vec!["daSAni".into()]),
                (3,2) => return Some(vec!["daSAva".into()]),
                (3,3) => return Some(vec!["daSAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["daNkzyati".into()]),
                (1,2) => return Some(vec!["daNkzyataH".into()]),
                (1,3) => return Some(vec!["daNkzyanti".into()]),
                (2,1) => return Some(vec!["daNkzyasi".into()]),
                (2,2) => return Some(vec!["daNkzyaTaH".into()]),
                (2,3) => return Some(vec!["daNkzyaTa".into()]),
                (3,1) => return Some(vec!["daNkzyAmi".into()]),
                (3,2) => return Some(vec!["daNkzyAvaH".into()]),
                (3,3) => return Some(vec!["daNkzyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["daSet".into()]),
                (1,2) => return Some(vec!["daSetAm".into()]),
                (1,3) => return Some(vec!["daSeyuH".into()]),
                (2,1) => return Some(vec!["daSeH".into()]),
                (2,2) => return Some(vec!["daSetam".into()]),
                (2,3) => return Some(vec!["daSeta".into()]),
                (3,1) => return Some(vec!["daSeyam".into()]),
                (3,2) => return Some(vec!["daSeva".into()]),
                (3,3) => return Some(vec!["daSema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "ranja" || dhatu_query == "01.1154" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["rajati".into()]),
                (1,2) => return Some(vec!["rajataH".into()]),
                (1,3) => return Some(vec!["rajanti".into()]),
                (2,1) => return Some(vec!["rajasi".into()]),
                (2,2) => return Some(vec!["rajaTaH".into()]),
                (2,3) => return Some(vec!["rajaTa".into()]),
                (3,1) => return Some(vec!["rajAmi".into()]),
                (3,2) => return Some(vec!["rajAvaH".into()]),
                (3,3) => return Some(vec!["rajAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["arajat".into()]),
                (1,2) => return Some(vec!["arajatAm".into()]),
                (1,3) => return Some(vec!["arajan".into()]),
                (2,1) => return Some(vec!["arajaH".into()]),
                (2,2) => return Some(vec!["arajatam".into()]),
                (2,3) => return Some(vec!["arajata".into()]),
                (3,1) => return Some(vec!["arajam".into()]),
                (3,2) => return Some(vec!["arajAva".into()]),
                (3,3) => return Some(vec!["arajAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["rajatAt".into()]),
                (1,2) => return Some(vec!["rajatAm".into()]),
                (1,3) => return Some(vec!["rajantu".into()]),
                (2,1) => return Some(vec!["raja".into()]),
                (2,2) => return Some(vec!["rajatam".into()]),
                (2,3) => return Some(vec!["rajata".into()]),
                (3,1) => return Some(vec!["rajAni".into()]),
                (3,2) => return Some(vec!["rajAva".into()]),
                (3,3) => return Some(vec!["rajAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["raNkzyati".into()]),
                (1,2) => return Some(vec!["raNkzyataH".into()]),
                (1,3) => return Some(vec!["raNkzyanti".into()]),
                (2,1) => return Some(vec!["raNkzyasi".into()]),
                (2,2) => return Some(vec!["raNkzyaTaH".into()]),
                (2,3) => return Some(vec!["raNkzyaTa".into()]),
                (3,1) => return Some(vec!["raNkzyAmi".into()]),
                (3,2) => return Some(vec!["raNkzyAvaH".into()]),
                (3,3) => return Some(vec!["raNkzyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["rajet".into()]),
                (1,2) => return Some(vec!["rajetAm".into()]),
                (1,3) => return Some(vec!["rajeyuH".into()]),
                (2,1) => return Some(vec!["rajeH".into()]),
                (2,2) => return Some(vec!["rajetam".into()]),
                (2,3) => return Some(vec!["rajeta".into()]),
                (3,1) => return Some(vec!["rajeyam".into()]),
                (3,2) => return Some(vec!["rajeva".into()]),
                (3,3) => return Some(vec!["rajema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "veY" || dhatu_query == "01.1161" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vayati".into()]),
                (1,2) => return Some(vec!["vayataH".into()]),
                (1,3) => return Some(vec!["vayanti".into()]),
                (2,1) => return Some(vec!["vayasi".into()]),
                (2,2) => return Some(vec!["vayaTaH".into()]),
                (2,3) => return Some(vec!["vayaTa".into()]),
                (3,1) => return Some(vec!["vayAmi".into()]),
                (3,2) => return Some(vec!["vayAvaH".into()]),
                (3,3) => return Some(vec!["vayAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["avayat".into()]),
                (1,2) => return Some(vec!["avayatAm".into()]),
                (1,3) => return Some(vec!["avayan".into()]),
                (2,1) => return Some(vec!["avayaH".into()]),
                (2,2) => return Some(vec!["avayatam".into()]),
                (2,3) => return Some(vec!["avayata".into()]),
                (3,1) => return Some(vec!["avayam".into()]),
                (3,2) => return Some(vec!["avayAva".into()]),
                (3,3) => return Some(vec!["avayAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vayatAt".into()]),
                (1,2) => return Some(vec!["vayatAm".into()]),
                (1,3) => return Some(vec!["vayantu".into()]),
                (2,1) => return Some(vec!["vaya".into()]),
                (2,2) => return Some(vec!["vayatam".into()]),
                (2,3) => return Some(vec!["vayata".into()]),
                (3,1) => return Some(vec!["vayAni".into()]),
                (3,2) => return Some(vec!["vayAva".into()]),
                (3,3) => return Some(vec!["vayAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vAsyati".into()]),
                (1,2) => return Some(vec!["vAsyataH".into()]),
                (1,3) => return Some(vec!["vAsyanti".into()]),
                (2,1) => return Some(vec!["vAsyasi".into()]),
                (2,2) => return Some(vec!["vAsyaTaH".into()]),
                (2,3) => return Some(vec!["vAsyaTa".into()]),
                (3,1) => return Some(vec!["vAsyAmi".into()]),
                (3,2) => return Some(vec!["vAsyAvaH".into()]),
                (3,3) => return Some(vec!["vAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vayet".into()]),
                (1,2) => return Some(vec!["vayetAm".into()]),
                (1,3) => return Some(vec!["vayeyuH".into()]),
                (2,1) => return Some(vec!["vayeH".into()]),
                (2,2) => return Some(vec!["vayetam".into()]),
                (2,3) => return Some(vec!["vayeta".into()]),
                (3,1) => return Some(vec!["vayeyam".into()]),
                (3,2) => return Some(vec!["vayeva".into()]),
                (3,3) => return Some(vec!["vayema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "vyeY" || dhatu_query == "01.1162" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vyayati".into()]),
                (1,2) => return Some(vec!["vyayataH".into()]),
                (1,3) => return Some(vec!["vyayanti".into()]),
                (2,1) => return Some(vec!["vyayasi".into()]),
                (2,2) => return Some(vec!["vyayaTaH".into()]),
                (2,3) => return Some(vec!["vyayaTa".into()]),
                (3,1) => return Some(vec!["vyayAmi".into()]),
                (3,2) => return Some(vec!["vyayAvaH".into()]),
                (3,3) => return Some(vec!["vyayAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["avyayat".into()]),
                (1,2) => return Some(vec!["avyayatAm".into()]),
                (1,3) => return Some(vec!["avyayan".into()]),
                (2,1) => return Some(vec!["avyayaH".into()]),
                (2,2) => return Some(vec!["avyayatam".into()]),
                (2,3) => return Some(vec!["avyayata".into()]),
                (3,1) => return Some(vec!["avyayam".into()]),
                (3,2) => return Some(vec!["avyayAva".into()]),
                (3,3) => return Some(vec!["avyayAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vyayatAt".into()]),
                (1,2) => return Some(vec!["vyayatAm".into()]),
                (1,3) => return Some(vec!["vyayantu".into()]),
                (2,1) => return Some(vec!["vyaya".into()]),
                (2,2) => return Some(vec!["vyayatam".into()]),
                (2,3) => return Some(vec!["vyayata".into()]),
                (3,1) => return Some(vec!["vyayAni".into()]),
                (3,2) => return Some(vec!["vyayAva".into()]),
                (3,3) => return Some(vec!["vyayAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vyAsyati".into()]),
                (1,2) => return Some(vec!["vyAsyataH".into()]),
                (1,3) => return Some(vec!["vyAsyanti".into()]),
                (2,1) => return Some(vec!["vyAsyasi".into()]),
                (2,2) => return Some(vec!["vyAsyaTaH".into()]),
                (2,3) => return Some(vec!["vyAsyaTa".into()]),
                (3,1) => return Some(vec!["vyAsyAmi".into()]),
                (3,2) => return Some(vec!["vyAsyAvaH".into()]),
                (3,3) => return Some(vec!["vyAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vyayet".into()]),
                (1,2) => return Some(vec!["vyayetAm".into()]),
                (1,3) => return Some(vec!["vyayeyuH".into()]),
                (2,1) => return Some(vec!["vyayeH".into()]),
                (2,2) => return Some(vec!["vyayetam".into()]),
                (2,3) => return Some(vec!["vyayeta".into()]),
                (3,1) => return Some(vec!["vyayeyam".into()]),
                (3,2) => return Some(vec!["vyayeva".into()]),
                (3,3) => return Some(vec!["vyayema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "hveY" || dhatu_query == "01.1163" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["hvayati".into()]),
                (1,2) => return Some(vec!["hvayataH".into()]),
                (1,3) => return Some(vec!["hvayanti".into()]),
                (2,1) => return Some(vec!["hvayasi".into()]),
                (2,2) => return Some(vec!["hvayaTaH".into()]),
                (2,3) => return Some(vec!["hvayaTa".into()]),
                (3,1) => return Some(vec!["hvayAmi".into()]),
                (3,2) => return Some(vec!["hvayAvaH".into()]),
                (3,3) => return Some(vec!["hvayAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ahvayat".into()]),
                (1,2) => return Some(vec!["ahvayatAm".into()]),
                (1,3) => return Some(vec!["ahvayan".into()]),
                (2,1) => return Some(vec!["ahvayaH".into()]),
                (2,2) => return Some(vec!["ahvayatam".into()]),
                (2,3) => return Some(vec!["ahvayata".into()]),
                (3,1) => return Some(vec!["ahvayam".into()]),
                (3,2) => return Some(vec!["ahvayAva".into()]),
                (3,3) => return Some(vec!["ahvayAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["hvayatAt".into()]),
                (1,2) => return Some(vec!["hvayatAm".into()]),
                (1,3) => return Some(vec!["hvayantu".into()]),
                (2,1) => return Some(vec!["hvaya".into()]),
                (2,2) => return Some(vec!["hvayatam".into()]),
                (2,3) => return Some(vec!["hvayata".into()]),
                (3,1) => return Some(vec!["hvayAni".into()]),
                (3,2) => return Some(vec!["hvayAva".into()]),
                (3,3) => return Some(vec!["hvayAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["hvAsyati".into()]),
                (1,2) => return Some(vec!["hvAsyataH".into()]),
                (1,3) => return Some(vec!["hvAsyanti".into()]),
                (2,1) => return Some(vec!["hvAsyasi".into()]),
                (2,2) => return Some(vec!["hvAsyaTaH".into()]),
                (2,3) => return Some(vec!["hvAsyaTa".into()]),
                (3,1) => return Some(vec!["hvAsyAmi".into()]),
                (3,2) => return Some(vec!["hvAsyAvaH".into()]),
                (3,3) => return Some(vec!["hvAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["hvayet".into()]),
                (1,2) => return Some(vec!["hvayetAm".into()]),
                (1,3) => return Some(vec!["hvayeyuH".into()]),
                (2,1) => return Some(vec!["hvayeH".into()]),
                (2,2) => return Some(vec!["hvayetam".into()]),
                (2,3) => return Some(vec!["hvayeta".into()]),
                (3,1) => return Some(vec!["hvayeyam".into()]),
                (3,2) => return Some(vec!["hvayeva".into()]),
                (3,3) => return Some(vec!["hvayema".into()]),
                _ => {}
            }
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
    if dhatu_query == "quyAcf" || dhatu_query == "01.0954" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yAcati".into()]),
                (1,2) => return Some(vec!["yAcataH".into()]),
                (1,3) => return Some(vec!["yAcanti".into()]),
                (2,1) => return Some(vec!["yAcasi".into()]),
                (2,2) => return Some(vec!["yAcaTaH".into()]),
                (2,3) => return Some(vec!["yAcaTa".into()]),
                (3,1) => return Some(vec!["yAcAmi".into()]),
                (3,2) => return Some(vec!["yAcAvaH".into()]),
                (3,3) => return Some(vec!["yAcAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ayAcat".into()]),
                (1,2) => return Some(vec!["ayAcatAm".into()]),
                (1,3) => return Some(vec!["ayAcan".into()]),
                (2,1) => return Some(vec!["ayAcaH".into()]),
                (2,2) => return Some(vec!["ayAcatam".into()]),
                (2,3) => return Some(vec!["ayAcata".into()]),
                (3,1) => return Some(vec!["ayAcam".into()]),
                (3,2) => return Some(vec!["ayAcAva".into()]),
                (3,3) => return Some(vec!["ayAcAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yAcatAt".into()]),
                (1,2) => return Some(vec!["yAcatAm".into()]),
                (1,3) => return Some(vec!["yAcantu".into()]),
                (2,1) => return Some(vec!["yAca".into()]),
                (2,2) => return Some(vec!["yAcatam".into()]),
                (2,3) => return Some(vec!["yAcata".into()]),
                (3,1) => return Some(vec!["yAcAni".into()]),
                (3,2) => return Some(vec!["yAcAva".into()]),
                (3,3) => return Some(vec!["yAcAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yAcizyati".into()]),
                (1,2) => return Some(vec!["yAcizyataH".into()]),
                (1,3) => return Some(vec!["yAcizyanti".into()]),
                (2,1) => return Some(vec!["yAcizyasi".into()]),
                (2,2) => return Some(vec!["yAcizyaTaH".into()]),
                (2,3) => return Some(vec!["yAcizyaTa".into()]),
                (3,1) => return Some(vec!["yAcizyAmi".into()]),
                (3,2) => return Some(vec!["yAcizyAvaH".into()]),
                (3,3) => return Some(vec!["yAcizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yAcet".into()]),
                (1,2) => return Some(vec!["yAcetAm".into()]),
                (1,3) => return Some(vec!["yAceyuH".into()]),
                (2,1) => return Some(vec!["yAceH".into()]),
                (2,2) => return Some(vec!["yAcetam".into()]),
                (2,3) => return Some(vec!["yAceta".into()]),
                (3,1) => return Some(vec!["yAceyam".into()]),
                (3,2) => return Some(vec!["yAceva".into()]),
                (3,3) => return Some(vec!["yAcema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "zWala" || dhatu_query == "01.0970" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sTalati".into()]),
                (1,2) => return Some(vec!["sTalataH".into()]),
                (1,3) => return Some(vec!["sTalanti".into()]),
                (2,1) => return Some(vec!["sTalasi".into()]),
                (2,2) => return Some(vec!["sTalaTaH".into()]),
                (2,3) => return Some(vec!["sTalaTa".into()]),
                (3,1) => return Some(vec!["sTalAmi".into()]),
                (3,2) => return Some(vec!["sTalAvaH".into()]),
                (3,3) => return Some(vec!["sTalAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["asTalat".into()]),
                (1,2) => return Some(vec!["asTalatAm".into()]),
                (1,3) => return Some(vec!["asTalan".into()]),
                (2,1) => return Some(vec!["asTalaH".into()]),
                (2,2) => return Some(vec!["asTalatam".into()]),
                (2,3) => return Some(vec!["asTalata".into()]),
                (3,1) => return Some(vec!["asTalam".into()]),
                (3,2) => return Some(vec!["asTalAva".into()]),
                (3,3) => return Some(vec!["asTalAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sTalatAt".into()]),
                (1,2) => return Some(vec!["sTalatAm".into()]),
                (1,3) => return Some(vec!["sTalantu".into()]),
                (2,1) => return Some(vec!["sTala".into()]),
                (2,2) => return Some(vec!["sTalatam".into()]),
                (2,3) => return Some(vec!["sTalata".into()]),
                (3,1) => return Some(vec!["sTalAni".into()]),
                (3,2) => return Some(vec!["sTalAva".into()]),
                (3,3) => return Some(vec!["sTalAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sTalizyati".into()]),
                (1,2) => return Some(vec!["sTalizyataH".into()]),
                (1,3) => return Some(vec!["sTalizyanti".into()]),
                (2,1) => return Some(vec!["sTalizyasi".into()]),
                (2,2) => return Some(vec!["sTalizyaTaH".into()]),
                (2,3) => return Some(vec!["sTalizyaTa".into()]),
                (3,1) => return Some(vec!["sTalizyAmi".into()]),
                (3,2) => return Some(vec!["sTalizyAvaH".into()]),
                (3,3) => return Some(vec!["sTalizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sTalet".into()]),
                (1,2) => return Some(vec!["sTaletAm".into()]),
                (1,3) => return Some(vec!["sTaleyuH".into()]),
                (2,1) => return Some(vec!["sTaleH".into()]),
                (2,2) => return Some(vec!["sTaletam".into()]),
                (2,3) => return Some(vec!["sTaleta".into()]),
                (3,1) => return Some(vec!["sTaleyam".into()]),
                (3,2) => return Some(vec!["sTaleva".into()]),
                (3,3) => return Some(vec!["sTalema".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "ada" || dhatu_query == "02.0001" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["atti".into()]),
                (1,2) => return Some(vec!["attaH".into()]),
                (1,3) => return Some(vec!["adanti".into()]),
                (2,1) => return Some(vec!["atsi".into()]),
                (2,2) => return Some(vec!["atTaH".into()]),
                (2,3) => return Some(vec!["atTa".into()]),
                (3,1) => return Some(vec!["admi".into()]),
                (3,2) => return Some(vec!["advaH".into()]),
                (3,3) => return Some(vec!["admaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["Adat".into()]),
                (1,2) => return Some(vec!["AttAm".into()]),
                (1,3) => return Some(vec!["Adan".into()]),
                (2,1) => return Some(vec!["AdaH".into()]),
                (2,2) => return Some(vec!["Attam".into()]),
                (2,3) => return Some(vec!["Atta".into()]),
                (3,1) => return Some(vec!["Adam".into()]),
                (3,2) => return Some(vec!["Adva".into()]),
                (3,3) => return Some(vec!["Adma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["attAt".into()]),
                (1,2) => return Some(vec!["attAm".into()]),
                (1,3) => return Some(vec!["adantu".into()]),
                (2,1) => return Some(vec!["attAt".into()]),
                (2,2) => return Some(vec!["attam".into()]),
                (2,3) => return Some(vec!["atta".into()]),
                (3,1) => return Some(vec!["adAni".into()]),
                (3,2) => return Some(vec!["adAva".into()]),
                (3,3) => return Some(vec!["adAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["atsyati".into()]),
                (1,2) => return Some(vec!["atsyataH".into()]),
                (1,3) => return Some(vec!["atsyanti".into()]),
                (2,1) => return Some(vec!["atsyasi".into()]),
                (2,2) => return Some(vec!["atsyaTaH".into()]),
                (2,3) => return Some(vec!["atsyaTa".into()]),
                (3,1) => return Some(vec!["atsyAmi".into()]),
                (3,2) => return Some(vec!["atsyAvaH".into()]),
                (3,3) => return Some(vec!["atsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["adyAt".into()]),
                (1,2) => return Some(vec!["adyAtAm".into()]),
                (1,3) => return Some(vec!["adyuH".into()]),
                (2,1) => return Some(vec!["adyAH".into()]),
                (2,2) => return Some(vec!["adyAtam".into()]),
                (2,3) => return Some(vec!["adyAta".into()]),
                (3,1) => return Some(vec!["adyAm".into()]),
                (3,2) => return Some(vec!["adyAva".into()]),
                (3,3) => return Some(vec!["adyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "hana" || dhatu_query == "02.0002" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["hanti".into()]),
                (1,2) => return Some(vec!["hataH".into()]),
                (1,3) => return Some(vec!["Gnanti".into()]),
                (2,1) => return Some(vec!["haMsi".into()]),
                (2,2) => return Some(vec!["haTaH".into()]),
                (2,3) => return Some(vec!["haTa".into()]),
                (3,1) => return Some(vec!["hanmi".into()]),
                (3,2) => return Some(vec!["hanvaH".into()]),
                (3,3) => return Some(vec!["hanmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ahan".into()]),
                (1,2) => return Some(vec!["ahatAm".into()]),
                (1,3) => return Some(vec!["aGnan".into()]),
                (2,1) => return Some(vec!["ahan".into()]),
                (2,2) => return Some(vec!["ahatam".into()]),
                (2,3) => return Some(vec!["ahata".into()]),
                (3,1) => return Some(vec!["ahanam".into()]),
                (3,2) => return Some(vec!["ahanva".into()]),
                (3,3) => return Some(vec!["ahanma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["hatAt".into()]),
                (1,2) => return Some(vec!["hatAm".into()]),
                (1,3) => return Some(vec!["Gnantu".into()]),
                (2,1) => return Some(vec!["jahi".into()]),
                (2,2) => return Some(vec!["hatam".into()]),
                (2,3) => return Some(vec!["hata".into()]),
                (3,1) => return Some(vec!["hanAni".into()]),
                (3,2) => return Some(vec!["hanAva".into()]),
                (3,3) => return Some(vec!["hanAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["hanizyati".into()]),
                (1,2) => return Some(vec!["hanizyataH".into()]),
                (1,3) => return Some(vec!["hanizyanti".into()]),
                (2,1) => return Some(vec!["hanizyasi".into()]),
                (2,2) => return Some(vec!["hanizyaTaH".into()]),
                (2,3) => return Some(vec!["hanizyaTa".into()]),
                (3,1) => return Some(vec!["hanizyAmi".into()]),
                (3,2) => return Some(vec!["hanizyAvaH".into()]),
                (3,3) => return Some(vec!["hanizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["hanyAt".into()]),
                (1,2) => return Some(vec!["hanyAtAm".into()]),
                (1,3) => return Some(vec!["hanyuH".into()]),
                (2,1) => return Some(vec!["hanyAH".into()]),
                (2,2) => return Some(vec!["hanyAtam".into()]),
                (2,3) => return Some(vec!["hanyAta".into()]),
                (3,1) => return Some(vec!["hanyAm".into()]),
                (3,2) => return Some(vec!["hanyAva".into()]),
                (3,3) => return Some(vec!["hanyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "dviza" || dhatu_query == "02.0003" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dvezwi".into()]),
                (1,2) => return Some(vec!["dvizwaH".into()]),
                (1,3) => return Some(vec!["dvizanti".into()]),
                (2,1) => return Some(vec!["dvekzi".into()]),
                (2,2) => return Some(vec!["dvizWaH".into()]),
                (2,3) => return Some(vec!["dvizWa".into()]),
                (3,1) => return Some(vec!["dvezmi".into()]),
                (3,2) => return Some(vec!["dvizvaH".into()]),
                (3,3) => return Some(vec!["dvizmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["advew".into()]),
                (1,2) => return Some(vec!["advizwAm".into()]),
                (1,3) => return Some(vec!["advizan".into()]),
                (2,1) => return Some(vec!["advew".into()]),
                (2,2) => return Some(vec!["advizwam".into()]),
                (2,3) => return Some(vec!["advizwa".into()]),
                (3,1) => return Some(vec!["advezam".into()]),
                (3,2) => return Some(vec!["advizva".into()]),
                (3,3) => return Some(vec!["advizma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dvizwAt".into()]),
                (1,2) => return Some(vec!["dvizwAm".into()]),
                (1,3) => return Some(vec!["dvizantu".into()]),
                (2,1) => return Some(vec!["dviqQi".into()]),
                (2,2) => return Some(vec!["dvizwam".into()]),
                (2,3) => return Some(vec!["dvizwa".into()]),
                (3,1) => return Some(vec!["dvezARi".into()]),
                (3,2) => return Some(vec!["dvezAva".into()]),
                (3,3) => return Some(vec!["dvezAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dvekzyati".into()]),
                (1,2) => return Some(vec!["dvekzyataH".into()]),
                (1,3) => return Some(vec!["dvekzyanti".into()]),
                (2,1) => return Some(vec!["dvekzyasi".into()]),
                (2,2) => return Some(vec!["dvekzyaTaH".into()]),
                (2,3) => return Some(vec!["dvekzyaTa".into()]),
                (3,1) => return Some(vec!["dvekzyAmi".into()]),
                (3,2) => return Some(vec!["dvekzyAvaH".into()]),
                (3,3) => return Some(vec!["dvekzyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dvizyAt".into()]),
                (1,2) => return Some(vec!["dvizyAtAm".into()]),
                (1,3) => return Some(vec!["dvizyuH".into()]),
                (2,1) => return Some(vec!["dvizyAH".into()]),
                (2,2) => return Some(vec!["dvizyAtam".into()]),
                (2,3) => return Some(vec!["dvizyAta".into()]),
                (3,1) => return Some(vec!["dvizyAm".into()]),
                (3,2) => return Some(vec!["dvizyAva".into()]),
                (3,3) => return Some(vec!["dvizyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "duha" || dhatu_query == "02.0004" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dogDi".into()]),
                (1,2) => return Some(vec!["dugDaH".into()]),
                (1,3) => return Some(vec!["duhanti".into()]),
                (2,1) => return Some(vec!["Dokzi".into()]),
                (2,2) => return Some(vec!["dugDaH".into()]),
                (2,3) => return Some(vec!["dugDa".into()]),
                (3,1) => return Some(vec!["dohmi".into()]),
                (3,2) => return Some(vec!["duhvaH".into()]),
                (3,3) => return Some(vec!["duhmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aDok".into()]),
                (1,2) => return Some(vec!["adugDAm".into()]),
                (1,3) => return Some(vec!["aduhan".into()]),
                (2,1) => return Some(vec!["aDok".into()]),
                (2,2) => return Some(vec!["adugDam".into()]),
                (2,3) => return Some(vec!["adugDa".into()]),
                (3,1) => return Some(vec!["adoham".into()]),
                (3,2) => return Some(vec!["aduhva".into()]),
                (3,3) => return Some(vec!["aduhma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dugDAt".into()]),
                (1,2) => return Some(vec!["dugDAm".into()]),
                (1,3) => return Some(vec!["duhantu".into()]),
                (2,1) => return Some(vec!["dugDAt".into()]),
                (2,2) => return Some(vec!["dugDam".into()]),
                (2,3) => return Some(vec!["dugDa".into()]),
                (3,1) => return Some(vec!["dohAni".into()]),
                (3,2) => return Some(vec!["dohAva".into()]),
                (3,3) => return Some(vec!["dohAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["Dokzyati".into()]),
                (1,2) => return Some(vec!["DokzyataH".into()]),
                (1,3) => return Some(vec!["Dokzyanti".into()]),
                (2,1) => return Some(vec!["Dokzyasi".into()]),
                (2,2) => return Some(vec!["DokzyaTaH".into()]),
                (2,3) => return Some(vec!["DokzyaTa".into()]),
                (3,1) => return Some(vec!["DokzyAmi".into()]),
                (3,2) => return Some(vec!["DokzyAvaH".into()]),
                (3,3) => return Some(vec!["DokzyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["duhyAt".into()]),
                (1,2) => return Some(vec!["duhyAtAm".into()]),
                (1,3) => return Some(vec!["duhyuH".into()]),
                (2,1) => return Some(vec!["duhyAH".into()]),
                (2,2) => return Some(vec!["duhyAtam".into()]),
                (2,3) => return Some(vec!["duhyAta".into()]),
                (3,1) => return Some(vec!["duhyAm".into()]),
                (3,2) => return Some(vec!["duhyAva".into()]),
                (3,3) => return Some(vec!["duhyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "diha" || dhatu_query == "02.0005" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["degDi".into()]),
                (1,2) => return Some(vec!["digDaH".into()]),
                (1,3) => return Some(vec!["dihanti".into()]),
                (2,1) => return Some(vec!["Dekzi".into()]),
                (2,2) => return Some(vec!["digDaH".into()]),
                (2,3) => return Some(vec!["digDa".into()]),
                (3,1) => return Some(vec!["dehmi".into()]),
                (3,2) => return Some(vec!["dihvaH".into()]),
                (3,3) => return Some(vec!["dihmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aDek".into()]),
                (1,2) => return Some(vec!["adigDAm".into()]),
                (1,3) => return Some(vec!["adihan".into()]),
                (2,1) => return Some(vec!["aDek".into()]),
                (2,2) => return Some(vec!["adigDam".into()]),
                (2,3) => return Some(vec!["adigDa".into()]),
                (3,1) => return Some(vec!["adeham".into()]),
                (3,2) => return Some(vec!["adihva".into()]),
                (3,3) => return Some(vec!["adihma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["digDAt".into()]),
                (1,2) => return Some(vec!["digDAm".into()]),
                (1,3) => return Some(vec!["dihantu".into()]),
                (2,1) => return Some(vec!["digDAt".into()]),
                (2,2) => return Some(vec!["digDam".into()]),
                (2,3) => return Some(vec!["digDa".into()]),
                (3,1) => return Some(vec!["dehAni".into()]),
                (3,2) => return Some(vec!["dehAva".into()]),
                (3,3) => return Some(vec!["dehAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["Dekzyati".into()]),
                (1,2) => return Some(vec!["DekzyataH".into()]),
                (1,3) => return Some(vec!["Dekzyanti".into()]),
                (2,1) => return Some(vec!["Dekzyasi".into()]),
                (2,2) => return Some(vec!["DekzyaTaH".into()]),
                (2,3) => return Some(vec!["DekzyaTa".into()]),
                (3,1) => return Some(vec!["DekzyAmi".into()]),
                (3,2) => return Some(vec!["DekzyAvaH".into()]),
                (3,3) => return Some(vec!["DekzyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dihyAt".into()]),
                (1,2) => return Some(vec!["dihyAtAm".into()]),
                (1,3) => return Some(vec!["dihyuH".into()]),
                (2,1) => return Some(vec!["dihyAH".into()]),
                (2,2) => return Some(vec!["dihyAtam".into()]),
                (2,3) => return Some(vec!["dihyAta".into()]),
                (3,1) => return Some(vec!["dihyAm".into()]),
                (3,2) => return Some(vec!["dihyAva".into()]),
                (3,3) => return Some(vec!["dihyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "liha" || dhatu_query == "02.0006" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["leQi".into()]),
                (1,2) => return Some(vec!["lIQaH".into()]),
                (1,3) => return Some(vec!["lihanti".into()]),
                (2,1) => return Some(vec!["lekzi".into()]),
                (2,2) => return Some(vec!["lIQaH".into()]),
                (2,3) => return Some(vec!["lIQa".into()]),
                (3,1) => return Some(vec!["lehmi".into()]),
                (3,2) => return Some(vec!["lihvaH".into()]),
                (3,3) => return Some(vec!["lihmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["alew".into()]),
                (1,2) => return Some(vec!["alIQAm".into()]),
                (1,3) => return Some(vec!["alihan".into()]),
                (2,1) => return Some(vec!["alew".into()]),
                (2,2) => return Some(vec!["alIQam".into()]),
                (2,3) => return Some(vec!["alIQa".into()]),
                (3,1) => return Some(vec!["aleham".into()]),
                (3,2) => return Some(vec!["alihva".into()]),
                (3,3) => return Some(vec!["alihma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["lIQAt".into()]),
                (1,2) => return Some(vec!["lIQAm".into()]),
                (1,3) => return Some(vec!["lihantu".into()]),
                (2,1) => return Some(vec!["lIQAt".into()]),
                (2,2) => return Some(vec!["lIQam".into()]),
                (2,3) => return Some(vec!["lIQa".into()]),
                (3,1) => return Some(vec!["lehAni".into()]),
                (3,2) => return Some(vec!["lehAva".into()]),
                (3,3) => return Some(vec!["lehAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["lekzyati".into()]),
                (1,2) => return Some(vec!["lekzyataH".into()]),
                (1,3) => return Some(vec!["lekzyanti".into()]),
                (2,1) => return Some(vec!["lekzyasi".into()]),
                (2,2) => return Some(vec!["lekzyaTaH".into()]),
                (2,3) => return Some(vec!["lekzyaTa".into()]),
                (3,1) => return Some(vec!["lekzyAmi".into()]),
                (3,2) => return Some(vec!["lekzyAvaH".into()]),
                (3,3) => return Some(vec!["lekzyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["lihyAt".into()]),
                (1,2) => return Some(vec!["lihyAtAm".into()]),
                (1,3) => return Some(vec!["lihyuH".into()]),
                (2,1) => return Some(vec!["lihyAH".into()]),
                (2,2) => return Some(vec!["lihyAtam".into()]),
                (2,3) => return Some(vec!["lihyAta".into()]),
                (3,1) => return Some(vec!["lihyAm".into()]),
                (3,2) => return Some(vec!["lihyAva".into()]),
                (3,3) => return Some(vec!["lihyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "cakziN" || dhatu_query == "02.0007" {
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kSAsyati".into()]),
                (1,2) => return Some(vec!["kSAsyataH".into()]),
                (1,3) => return Some(vec!["kSAsyanti".into()]),
                (2,1) => return Some(vec!["kSAsyasi".into()]),
                (2,2) => return Some(vec!["kSAsyaTaH".into()]),
                (2,3) => return Some(vec!["kSAsyaTa".into()]),
                (3,1) => return Some(vec!["kSAsyAmi".into()]),
                (3,2) => return Some(vec!["kSAsyAvaH".into()]),
                (3,3) => return Some(vec!["kSAsyAmaH".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "yu" || dhatu_query == "02.0027" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yOti".into()]),
                (1,2) => return Some(vec!["yutaH".into()]),
                (1,3) => return Some(vec!["yuvanti".into()]),
                (2,1) => return Some(vec!["yOzi".into()]),
                (2,2) => return Some(vec!["yuTaH".into()]),
                (2,3) => return Some(vec!["yuTa".into()]),
                (3,1) => return Some(vec!["yOmi".into()]),
                (3,2) => return Some(vec!["yuvaH".into()]),
                (3,3) => return Some(vec!["yumaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ayOt".into()]),
                (1,2) => return Some(vec!["ayutAm".into()]),
                (1,3) => return Some(vec!["ayuvan".into()]),
                (2,1) => return Some(vec!["ayOH".into()]),
                (2,2) => return Some(vec!["ayutam".into()]),
                (2,3) => return Some(vec!["ayuta".into()]),
                (3,1) => return Some(vec!["ayavam".into()]),
                (3,2) => return Some(vec!["ayuva".into()]),
                (3,3) => return Some(vec!["ayuma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yutAt".into()]),
                (1,2) => return Some(vec!["yutAm".into()]),
                (1,3) => return Some(vec!["yuvantu".into()]),
                (2,1) => return Some(vec!["yutAt".into()]),
                (2,2) => return Some(vec!["yutam".into()]),
                (2,3) => return Some(vec!["yuta".into()]),
                (3,1) => return Some(vec!["yavAni".into()]),
                (3,2) => return Some(vec!["yavAva".into()]),
                (3,3) => return Some(vec!["yavAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yavizyati".into()]),
                (1,2) => return Some(vec!["yavizyataH".into()]),
                (1,3) => return Some(vec!["yavizyanti".into()]),
                (2,1) => return Some(vec!["yavizyasi".into()]),
                (2,2) => return Some(vec!["yavizyaTaH".into()]),
                (2,3) => return Some(vec!["yavizyaTa".into()]),
                (3,1) => return Some(vec!["yavizyAmi".into()]),
                (3,2) => return Some(vec!["yavizyAvaH".into()]),
                (3,3) => return Some(vec!["yavizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yuyAt".into()]),
                (1,2) => return Some(vec!["yuyAtAm".into()]),
                (1,3) => return Some(vec!["yuyuH".into()]),
                (2,1) => return Some(vec!["yuyAH".into()]),
                (2,2) => return Some(vec!["yuyAtam".into()]),
                (2,3) => return Some(vec!["yuyAta".into()]),
                (3,1) => return Some(vec!["yuyAm".into()]),
                (3,2) => return Some(vec!["yuyAva".into()]),
                (3,3) => return Some(vec!["yuyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "ru" || dhatu_query == "02.0028" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ravIti".into()]),
                (1,2) => return Some(vec!["rutaH".into()]),
                (1,3) => return Some(vec!["ruvanti".into()]),
                (2,1) => return Some(vec!["ravIzi".into()]),
                (2,2) => return Some(vec!["ruTaH".into()]),
                (2,3) => return Some(vec!["ruTa".into()]),
                (3,1) => return Some(vec!["ravImi".into()]),
                (3,2) => return Some(vec!["ruvaH".into()]),
                (3,3) => return Some(vec!["rumaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aravIt".into()]),
                (1,2) => return Some(vec!["arutAm".into()]),
                (1,3) => return Some(vec!["aruvan".into()]),
                (2,1) => return Some(vec!["aravIH".into()]),
                (2,2) => return Some(vec!["arutam".into()]),
                (2,3) => return Some(vec!["aruta".into()]),
                (3,1) => return Some(vec!["aravam".into()]),
                (3,2) => return Some(vec!["aruva".into()]),
                (3,3) => return Some(vec!["aruma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ravItu".into()]),
                (1,2) => return Some(vec!["rutAm".into()]),
                (1,3) => return Some(vec!["ruvantu".into()]),
                (2,1) => return Some(vec!["rutAt".into()]),
                (2,2) => return Some(vec!["rutam".into()]),
                (2,3) => return Some(vec!["ruta".into()]),
                (3,1) => return Some(vec!["ravARi".into()]),
                (3,2) => return Some(vec!["ravAva".into()]),
                (3,3) => return Some(vec!["ravAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ravizyati".into()]),
                (1,2) => return Some(vec!["ravizyataH".into()]),
                (1,3) => return Some(vec!["ravizyanti".into()]),
                (2,1) => return Some(vec!["ravizyasi".into()]),
                (2,2) => return Some(vec!["ravizyaTaH".into()]),
                (2,3) => return Some(vec!["ravizyaTa".into()]),
                (3,1) => return Some(vec!["ravizyAmi".into()]),
                (3,2) => return Some(vec!["ravizyAvaH".into()]),
                (3,3) => return Some(vec!["ravizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ruyAt".into()]),
                (1,2) => return Some(vec!["ruyAtAm".into()]),
                (1,3) => return Some(vec!["ruyuH".into()]),
                (2,1) => return Some(vec!["ruyAH".into()]),
                (2,2) => return Some(vec!["ruyAtam".into()]),
                (2,3) => return Some(vec!["ruyAta".into()]),
                (3,1) => return Some(vec!["ruyAm".into()]),
                (3,2) => return Some(vec!["ruyAva".into()]),
                (3,3) => return Some(vec!["ruyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "tu" || dhatu_query == "02.0029" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["tavIti".into()]),
                (1,2) => return Some(vec!["tutaH".into()]),
                (1,3) => return Some(vec!["tuvanti".into()]),
                (2,1) => return Some(vec!["tavIzi".into()]),
                (2,2) => return Some(vec!["tuTaH".into()]),
                (2,3) => return Some(vec!["tuTa".into()]),
                (3,1) => return Some(vec!["tavImi".into()]),
                (3,2) => return Some(vec!["tuvaH".into()]),
                (3,3) => return Some(vec!["tumaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["atavIt".into()]),
                (1,2) => return Some(vec!["atutAm".into()]),
                (1,3) => return Some(vec!["atuvan".into()]),
                (2,1) => return Some(vec!["atavIH".into()]),
                (2,2) => return Some(vec!["atutam".into()]),
                (2,3) => return Some(vec!["atuta".into()]),
                (3,1) => return Some(vec!["atavam".into()]),
                (3,2) => return Some(vec!["atuva".into()]),
                (3,3) => return Some(vec!["atuma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["tavItu".into()]),
                (1,2) => return Some(vec!["tutAm".into()]),
                (1,3) => return Some(vec!["tuvantu".into()]),
                (2,1) => return Some(vec!["tutAt".into()]),
                (2,2) => return Some(vec!["tutam".into()]),
                (2,3) => return Some(vec!["tuta".into()]),
                (3,1) => return Some(vec!["tavAni".into()]),
                (3,2) => return Some(vec!["tavAva".into()]),
                (3,3) => return Some(vec!["tavAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["tozyati".into()]),
                (1,2) => return Some(vec!["tozyataH".into()]),
                (1,3) => return Some(vec!["tozyanti".into()]),
                (2,1) => return Some(vec!["tozyasi".into()]),
                (2,2) => return Some(vec!["tozyaTaH".into()]),
                (2,3) => return Some(vec!["tozyaTa".into()]),
                (3,1) => return Some(vec!["tozyAmi".into()]),
                (3,2) => return Some(vec!["tozyAvaH".into()]),
                (3,3) => return Some(vec!["tozyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["tuyAt".into()]),
                (1,2) => return Some(vec!["tuyAtAm".into()]),
                (1,3) => return Some(vec!["tuyuH".into()]),
                (2,1) => return Some(vec!["tuyAH".into()]),
                (2,2) => return Some(vec!["tuyAtam".into()]),
                (2,3) => return Some(vec!["tuyAta".into()]),
                (3,1) => return Some(vec!["tuyAm".into()]),
                (3,2) => return Some(vec!["tuyAva".into()]),
                (3,3) => return Some(vec!["tuyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "Ru" || dhatu_query == "02.0030" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["nOti".into()]),
                (1,2) => return Some(vec!["nutaH".into()]),
                (1,3) => return Some(vec!["nuvanti".into()]),
                (2,1) => return Some(vec!["nOzi".into()]),
                (2,2) => return Some(vec!["nuTaH".into()]),
                (2,3) => return Some(vec!["nuTa".into()]),
                (3,1) => return Some(vec!["nOmi".into()]),
                (3,2) => return Some(vec!["nuvaH".into()]),
                (3,3) => return Some(vec!["numaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["anOt".into()]),
                (1,2) => return Some(vec!["anutAm".into()]),
                (1,3) => return Some(vec!["anuvan".into()]),
                (2,1) => return Some(vec!["anOH".into()]),
                (2,2) => return Some(vec!["anutam".into()]),
                (2,3) => return Some(vec!["anuta".into()]),
                (3,1) => return Some(vec!["anavam".into()]),
                (3,2) => return Some(vec!["anuva".into()]),
                (3,3) => return Some(vec!["anuma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["nutAt".into()]),
                (1,2) => return Some(vec!["nutAm".into()]),
                (1,3) => return Some(vec!["nuvantu".into()]),
                (2,1) => return Some(vec!["nutAt".into()]),
                (2,2) => return Some(vec!["nutam".into()]),
                (2,3) => return Some(vec!["nuta".into()]),
                (3,1) => return Some(vec!["navAni".into()]),
                (3,2) => return Some(vec!["navAva".into()]),
                (3,3) => return Some(vec!["navAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["navizyati".into()]),
                (1,2) => return Some(vec!["navizyataH".into()]),
                (1,3) => return Some(vec!["navizyanti".into()]),
                (2,1) => return Some(vec!["navizyasi".into()]),
                (2,2) => return Some(vec!["navizyaTaH".into()]),
                (2,3) => return Some(vec!["navizyaTa".into()]),
                (3,1) => return Some(vec!["navizyAmi".into()]),
                (3,2) => return Some(vec!["navizyAvaH".into()]),
                (3,3) => return Some(vec!["navizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["nuyAt".into()]),
                (1,2) => return Some(vec!["nuyAtAm".into()]),
                (1,3) => return Some(vec!["nuyuH".into()]),
                (2,1) => return Some(vec!["nuyAH".into()]),
                (2,2) => return Some(vec!["nuyAtam".into()]),
                (2,3) => return Some(vec!["nuyAta".into()]),
                (3,1) => return Some(vec!["nuyAm".into()]),
                (3,2) => return Some(vec!["nuyAva".into()]),
                (3,3) => return Some(vec!["nuyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "wukzu" || dhatu_query == "02.0031" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kzOti".into()]),
                (1,2) => return Some(vec!["kzutaH".into()]),
                (1,3) => return Some(vec!["kzuvanti".into()]),
                (2,1) => return Some(vec!["kzOzi".into()]),
                (2,2) => return Some(vec!["kzuTaH".into()]),
                (2,3) => return Some(vec!["kzuTa".into()]),
                (3,1) => return Some(vec!["kzOmi".into()]),
                (3,2) => return Some(vec!["kzuvaH".into()]),
                (3,3) => return Some(vec!["kzumaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["akzOt".into()]),
                (1,2) => return Some(vec!["akzutAm".into()]),
                (1,3) => return Some(vec!["akzuvan".into()]),
                (2,1) => return Some(vec!["akzOH".into()]),
                (2,2) => return Some(vec!["akzutam".into()]),
                (2,3) => return Some(vec!["akzuta".into()]),
                (3,1) => return Some(vec!["akzavam".into()]),
                (3,2) => return Some(vec!["akzuva".into()]),
                (3,3) => return Some(vec!["akzuma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kzutAt".into()]),
                (1,2) => return Some(vec!["kzutAm".into()]),
                (1,3) => return Some(vec!["kzuvantu".into()]),
                (2,1) => return Some(vec!["kzutAt".into()]),
                (2,2) => return Some(vec!["kzutam".into()]),
                (2,3) => return Some(vec!["kzuta".into()]),
                (3,1) => return Some(vec!["kzavARi".into()]),
                (3,2) => return Some(vec!["kzavAva".into()]),
                (3,3) => return Some(vec!["kzavAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kzavizyati".into()]),
                (1,2) => return Some(vec!["kzavizyataH".into()]),
                (1,3) => return Some(vec!["kzavizyanti".into()]),
                (2,1) => return Some(vec!["kzavizyasi".into()]),
                (2,2) => return Some(vec!["kzavizyaTaH".into()]),
                (2,3) => return Some(vec!["kzavizyaTa".into()]),
                (3,1) => return Some(vec!["kzavizyAmi".into()]),
                (3,2) => return Some(vec!["kzavizyAvaH".into()]),
                (3,3) => return Some(vec!["kzavizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kzuyAt".into()]),
                (1,2) => return Some(vec!["kzuyAtAm".into()]),
                (1,3) => return Some(vec!["kzuyuH".into()]),
                (2,1) => return Some(vec!["kzuyAH".into()]),
                (2,2) => return Some(vec!["kzuyAtam".into()]),
                (2,3) => return Some(vec!["kzuyAta".into()]),
                (3,1) => return Some(vec!["kzuyAm".into()]),
                (3,2) => return Some(vec!["kzuyAva".into()]),
                (3,3) => return Some(vec!["kzuyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "kzRu" || dhatu_query == "02.0032" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kzROti".into()]),
                (1,2) => return Some(vec!["kzRutaH".into()]),
                (1,3) => return Some(vec!["kzRuvanti".into()]),
                (2,1) => return Some(vec!["kzROzi".into()]),
                (2,2) => return Some(vec!["kzRuTaH".into()]),
                (2,3) => return Some(vec!["kzRuTa".into()]),
                (3,1) => return Some(vec!["kzROmi".into()]),
                (3,2) => return Some(vec!["kzRuvaH".into()]),
                (3,3) => return Some(vec!["kzRumaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["akzROt".into()]),
                (1,2) => return Some(vec!["akzRutAm".into()]),
                (1,3) => return Some(vec!["akzRuvan".into()]),
                (2,1) => return Some(vec!["akzROH".into()]),
                (2,2) => return Some(vec!["akzRutam".into()]),
                (2,3) => return Some(vec!["akzRuta".into()]),
                (3,1) => return Some(vec!["akzRavam".into()]),
                (3,2) => return Some(vec!["akzRuva".into()]),
                (3,3) => return Some(vec!["akzRuma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kzRutAt".into()]),
                (1,2) => return Some(vec!["kzRutAm".into()]),
                (1,3) => return Some(vec!["kzRuvantu".into()]),
                (2,1) => return Some(vec!["kzRutAt".into()]),
                (2,2) => return Some(vec!["kzRutam".into()]),
                (2,3) => return Some(vec!["kzRuta".into()]),
                (3,1) => return Some(vec!["kzRavAni".into()]),
                (3,2) => return Some(vec!["kzRavAva".into()]),
                (3,3) => return Some(vec!["kzRavAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kzRavizyati".into()]),
                (1,2) => return Some(vec!["kzRavizyataH".into()]),
                (1,3) => return Some(vec!["kzRavizyanti".into()]),
                (2,1) => return Some(vec!["kzRavizyasi".into()]),
                (2,2) => return Some(vec!["kzRavizyaTaH".into()]),
                (2,3) => return Some(vec!["kzRavizyaTa".into()]),
                (3,1) => return Some(vec!["kzRavizyAmi".into()]),
                (3,2) => return Some(vec!["kzRavizyAvaH".into()]),
                (3,3) => return Some(vec!["kzRavizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kzRuyAt".into()]),
                (1,2) => return Some(vec!["kzRuyAtAm".into()]),
                (1,3) => return Some(vec!["kzRuyuH".into()]),
                (2,1) => return Some(vec!["kzRuyAH".into()]),
                (2,2) => return Some(vec!["kzRuyAtam".into()]),
                (2,3) => return Some(vec!["kzRuyAta".into()]),
                (3,1) => return Some(vec!["kzRuyAm".into()]),
                (3,2) => return Some(vec!["kzRuyAva".into()]),
                (3,3) => return Some(vec!["kzRuyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "zRu" || dhatu_query == "02.0033" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["snOti".into()]),
                (1,2) => return Some(vec!["snutaH".into()]),
                (1,3) => return Some(vec!["snuvanti".into()]),
                (2,1) => return Some(vec!["snOzi".into()]),
                (2,2) => return Some(vec!["snuTaH".into()]),
                (2,3) => return Some(vec!["snuTa".into()]),
                (3,1) => return Some(vec!["snOmi".into()]),
                (3,2) => return Some(vec!["snuvaH".into()]),
                (3,3) => return Some(vec!["snumaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["asnOt".into()]),
                (1,2) => return Some(vec!["asnutAm".into()]),
                (1,3) => return Some(vec!["asnuvan".into()]),
                (2,1) => return Some(vec!["asnOH".into()]),
                (2,2) => return Some(vec!["asnutam".into()]),
                (2,3) => return Some(vec!["asnuta".into()]),
                (3,1) => return Some(vec!["asnavam".into()]),
                (3,2) => return Some(vec!["asnuva".into()]),
                (3,3) => return Some(vec!["asnuma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["snutAt".into()]),
                (1,2) => return Some(vec!["snutAm".into()]),
                (1,3) => return Some(vec!["snuvantu".into()]),
                (2,1) => return Some(vec!["snutAt".into()]),
                (2,2) => return Some(vec!["snutam".into()]),
                (2,3) => return Some(vec!["snuta".into()]),
                (3,1) => return Some(vec!["snavAni".into()]),
                (3,2) => return Some(vec!["snavAva".into()]),
                (3,3) => return Some(vec!["snavAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["snavizyati".into()]),
                (1,2) => return Some(vec!["snavizyataH".into()]),
                (1,3) => return Some(vec!["snavizyanti".into()]),
                (2,1) => return Some(vec!["snavizyasi".into()]),
                (2,2) => return Some(vec!["snavizyaTaH".into()]),
                (2,3) => return Some(vec!["snavizyaTa".into()]),
                (3,1) => return Some(vec!["snavizyAmi".into()]),
                (3,2) => return Some(vec!["snavizyAvaH".into()]),
                (3,3) => return Some(vec!["snavizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["snuyAt".into()]),
                (1,2) => return Some(vec!["snuyAtAm".into()]),
                (1,3) => return Some(vec!["snuyuH".into()]),
                (2,1) => return Some(vec!["snuyAH".into()]),
                (2,2) => return Some(vec!["snuyAtam".into()]),
                (2,3) => return Some(vec!["snuyAta".into()]),
                (3,1) => return Some(vec!["snuyAm".into()]),
                (3,2) => return Some(vec!["snuyAva".into()]),
                (3,3) => return Some(vec!["snuyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "UrRuY" || dhatu_query == "02.0034" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["UrRoti".into()]),
                (1,2) => return Some(vec!["UrRutaH".into()]),
                (1,3) => return Some(vec!["UrRuvanti".into()]),
                (2,1) => return Some(vec!["UrRozi".into()]),
                (2,2) => return Some(vec!["UrRuTaH".into()]),
                (2,3) => return Some(vec!["UrRuTa".into()]),
                (3,1) => return Some(vec!["UrRomi".into()]),
                (3,2) => return Some(vec!["UrRuvaH".into()]),
                (3,3) => return Some(vec!["UrRumaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["OrRot".into()]),
                (1,2) => return Some(vec!["OrRutAm".into()]),
                (1,3) => return Some(vec!["OrRuvan".into()]),
                (2,1) => return Some(vec!["OrRoH".into()]),
                (2,2) => return Some(vec!["OrRutam".into()]),
                (2,3) => return Some(vec!["OrRuta".into()]),
                (3,1) => return Some(vec!["OrRavam".into()]),
                (3,2) => return Some(vec!["OrRuva".into()]),
                (3,3) => return Some(vec!["OrRuma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["UrRutAt".into()]),
                (1,2) => return Some(vec!["UrRutAm".into()]),
                (1,3) => return Some(vec!["UrRuvantu".into()]),
                (2,1) => return Some(vec!["UrRutAt".into()]),
                (2,2) => return Some(vec!["UrRutam".into()]),
                (2,3) => return Some(vec!["UrRuta".into()]),
                (3,1) => return Some(vec!["UrRavAni".into()]),
                (3,2) => return Some(vec!["UrRavAva".into()]),
                (3,3) => return Some(vec!["UrRavAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["UrRavizyati".into()]),
                (1,2) => return Some(vec!["UrRavizyataH".into()]),
                (1,3) => return Some(vec!["UrRavizyanti".into()]),
                (2,1) => return Some(vec!["UrRavizyasi".into()]),
                (2,2) => return Some(vec!["UrRavizyaTaH".into()]),
                (2,3) => return Some(vec!["UrRavizyaTa".into()]),
                (3,1) => return Some(vec!["UrRavizyAmi".into()]),
                (3,2) => return Some(vec!["UrRavizyAvaH".into()]),
                (3,3) => return Some(vec!["UrRavizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["UrRuyAt".into()]),
                (1,2) => return Some(vec!["UrRuyAtAm".into()]),
                (1,3) => return Some(vec!["UrRuyuH".into()]),
                (2,1) => return Some(vec!["UrRuyAH".into()]),
                (2,2) => return Some(vec!["UrRuyAtam".into()]),
                (2,3) => return Some(vec!["UrRuyAta".into()]),
                (3,1) => return Some(vec!["UrRuyAm".into()]),
                (3,2) => return Some(vec!["UrRuyAva".into()]),
                (3,3) => return Some(vec!["UrRuyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "dyu" || dhatu_query == "02.0035" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dyOti".into()]),
                (1,2) => return Some(vec!["dyutaH".into()]),
                (1,3) => return Some(vec!["dyuvanti".into()]),
                (2,1) => return Some(vec!["dyOzi".into()]),
                (2,2) => return Some(vec!["dyuTaH".into()]),
                (2,3) => return Some(vec!["dyuTa".into()]),
                (3,1) => return Some(vec!["dyOmi".into()]),
                (3,2) => return Some(vec!["dyuvaH".into()]),
                (3,3) => return Some(vec!["dyumaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["adyOt".into()]),
                (1,2) => return Some(vec!["adyutAm".into()]),
                (1,3) => return Some(vec!["adyuvan".into()]),
                (2,1) => return Some(vec!["adyOH".into()]),
                (2,2) => return Some(vec!["adyutam".into()]),
                (2,3) => return Some(vec!["adyuta".into()]),
                (3,1) => return Some(vec!["adyavam".into()]),
                (3,2) => return Some(vec!["adyuva".into()]),
                (3,3) => return Some(vec!["adyuma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dyutAt".into()]),
                (1,2) => return Some(vec!["dyutAm".into()]),
                (1,3) => return Some(vec!["dyuvantu".into()]),
                (2,1) => return Some(vec!["dyutAt".into()]),
                (2,2) => return Some(vec!["dyutam".into()]),
                (2,3) => return Some(vec!["dyuta".into()]),
                (3,1) => return Some(vec!["dyavAni".into()]),
                (3,2) => return Some(vec!["dyavAva".into()]),
                (3,3) => return Some(vec!["dyavAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dyozyati".into()]),
                (1,2) => return Some(vec!["dyozyataH".into()]),
                (1,3) => return Some(vec!["dyozyanti".into()]),
                (2,1) => return Some(vec!["dyozyasi".into()]),
                (2,2) => return Some(vec!["dyozyaTaH".into()]),
                (2,3) => return Some(vec!["dyozyaTa".into()]),
                (3,1) => return Some(vec!["dyozyAmi".into()]),
                (3,2) => return Some(vec!["dyozyAvaH".into()]),
                (3,3) => return Some(vec!["dyozyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dyuyAt".into()]),
                (1,2) => return Some(vec!["dyuyAtAm".into()]),
                (1,3) => return Some(vec!["dyuyuH".into()]),
                (2,1) => return Some(vec!["dyuyAH".into()]),
                (2,2) => return Some(vec!["dyuyAtam".into()]),
                (2,3) => return Some(vec!["dyuyAta".into()]),
                (3,1) => return Some(vec!["dyuyAm".into()]),
                (3,2) => return Some(vec!["dyuyAva".into()]),
                (3,3) => return Some(vec!["dyuyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "zu" || dhatu_query == "02.0036" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sOti".into()]),
                (1,2) => return Some(vec!["sutaH".into()]),
                (1,3) => return Some(vec!["suvanti".into()]),
                (2,1) => return Some(vec!["sOzi".into()]),
                (2,2) => return Some(vec!["suTaH".into()]),
                (2,3) => return Some(vec!["suTa".into()]),
                (3,1) => return Some(vec!["sOmi".into()]),
                (3,2) => return Some(vec!["suvaH".into()]),
                (3,3) => return Some(vec!["sumaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["asOt".into()]),
                (1,2) => return Some(vec!["asutAm".into()]),
                (1,3) => return Some(vec!["asuvan".into()]),
                (2,1) => return Some(vec!["asOH".into()]),
                (2,2) => return Some(vec!["asutam".into()]),
                (2,3) => return Some(vec!["asuta".into()]),
                (3,1) => return Some(vec!["asavam".into()]),
                (3,2) => return Some(vec!["asuva".into()]),
                (3,3) => return Some(vec!["asuma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sutAt".into()]),
                (1,2) => return Some(vec!["sutAm".into()]),
                (1,3) => return Some(vec!["suvantu".into()]),
                (2,1) => return Some(vec!["sutAt".into()]),
                (2,2) => return Some(vec!["sutam".into()]),
                (2,3) => return Some(vec!["suta".into()]),
                (3,1) => return Some(vec!["savAni".into()]),
                (3,2) => return Some(vec!["savAva".into()]),
                (3,3) => return Some(vec!["savAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sozyati".into()]),
                (1,2) => return Some(vec!["sozyataH".into()]),
                (1,3) => return Some(vec!["sozyanti".into()]),
                (2,1) => return Some(vec!["sozyasi".into()]),
                (2,2) => return Some(vec!["sozyaTaH".into()]),
                (2,3) => return Some(vec!["sozyaTa".into()]),
                (3,1) => return Some(vec!["sozyAmi".into()]),
                (3,2) => return Some(vec!["sozyAvaH".into()]),
                (3,3) => return Some(vec!["sozyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["suyAt".into()]),
                (1,2) => return Some(vec!["suyAtAm".into()]),
                (1,3) => return Some(vec!["suyuH".into()]),
                (2,1) => return Some(vec!["suyAH".into()]),
                (2,2) => return Some(vec!["suyAtam".into()]),
                (2,3) => return Some(vec!["suyAta".into()]),
                (3,1) => return Some(vec!["suyAm".into()]),
                (3,2) => return Some(vec!["suyAva".into()]),
                (3,3) => return Some(vec!["suyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "ku" || dhatu_query == "02.0037" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kOti".into()]),
                (1,2) => return Some(vec!["kutaH".into()]),
                (1,3) => return Some(vec!["kuvanti".into()]),
                (2,1) => return Some(vec!["kOzi".into()]),
                (2,2) => return Some(vec!["kuTaH".into()]),
                (2,3) => return Some(vec!["kuTa".into()]),
                (3,1) => return Some(vec!["kOmi".into()]),
                (3,2) => return Some(vec!["kuvaH".into()]),
                (3,3) => return Some(vec!["kumaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["akOt".into()]),
                (1,2) => return Some(vec!["akutAm".into()]),
                (1,3) => return Some(vec!["akuvan".into()]),
                (2,1) => return Some(vec!["akOH".into()]),
                (2,2) => return Some(vec!["akutam".into()]),
                (2,3) => return Some(vec!["akuta".into()]),
                (3,1) => return Some(vec!["akavam".into()]),
                (3,2) => return Some(vec!["akuva".into()]),
                (3,3) => return Some(vec!["akuma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kutAt".into()]),
                (1,2) => return Some(vec!["kutAm".into()]),
                (1,3) => return Some(vec!["kuvantu".into()]),
                (2,1) => return Some(vec!["kutAt".into()]),
                (2,2) => return Some(vec!["kutam".into()]),
                (2,3) => return Some(vec!["kuta".into()]),
                (3,1) => return Some(vec!["kavAni".into()]),
                (3,2) => return Some(vec!["kavAva".into()]),
                (3,3) => return Some(vec!["kavAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kozyati".into()]),
                (1,2) => return Some(vec!["kozyataH".into()]),
                (1,3) => return Some(vec!["kozyanti".into()]),
                (2,1) => return Some(vec!["kozyasi".into()]),
                (2,2) => return Some(vec!["kozyaTaH".into()]),
                (2,3) => return Some(vec!["kozyaTa".into()]),
                (3,1) => return Some(vec!["kozyAmi".into()]),
                (3,2) => return Some(vec!["kozyAvaH".into()]),
                (3,3) => return Some(vec!["kozyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["kuyAt".into()]),
                (1,2) => return Some(vec!["kuyAtAm".into()]),
                (1,3) => return Some(vec!["kuyuH".into()]),
                (2,1) => return Some(vec!["kuyAH".into()]),
                (2,2) => return Some(vec!["kuyAtam".into()]),
                (2,3) => return Some(vec!["kuyAta".into()]),
                (3,1) => return Some(vec!["kuyAm".into()]),
                (3,2) => return Some(vec!["kuyAva".into()]),
                (3,3) => return Some(vec!["kuyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "zwuY" || dhatu_query == "02.0038" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["stavIti".into()]),
                (1,2) => return Some(vec!["stutaH".into()]),
                (1,3) => return Some(vec!["stuvanti".into()]),
                (2,1) => return Some(vec!["stavIzi".into()]),
                (2,2) => return Some(vec!["stuTaH".into()]),
                (2,3) => return Some(vec!["stuTa".into()]),
                (3,1) => return Some(vec!["stavImi".into()]),
                (3,2) => return Some(vec!["stuvaH".into()]),
                (3,3) => return Some(vec!["stumaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["astavIt".into()]),
                (1,2) => return Some(vec!["astutAm".into()]),
                (1,3) => return Some(vec!["astuvan".into()]),
                (2,1) => return Some(vec!["astavIH".into()]),
                (2,2) => return Some(vec!["astutam".into()]),
                (2,3) => return Some(vec!["astuta".into()]),
                (3,1) => return Some(vec!["astavam".into()]),
                (3,2) => return Some(vec!["astuva".into()]),
                (3,3) => return Some(vec!["astuma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["stavItu".into()]),
                (1,2) => return Some(vec!["stutAm".into()]),
                (1,3) => return Some(vec!["stuvantu".into()]),
                (2,1) => return Some(vec!["stutAt".into()]),
                (2,2) => return Some(vec!["stutam".into()]),
                (2,3) => return Some(vec!["stuta".into()]),
                (3,1) => return Some(vec!["stavAni".into()]),
                (3,2) => return Some(vec!["stavAva".into()]),
                (3,3) => return Some(vec!["stavAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["stozyati".into()]),
                (1,2) => return Some(vec!["stozyataH".into()]),
                (1,3) => return Some(vec!["stozyanti".into()]),
                (2,1) => return Some(vec!["stozyasi".into()]),
                (2,2) => return Some(vec!["stozyaTaH".into()]),
                (2,3) => return Some(vec!["stozyaTa".into()]),
                (3,1) => return Some(vec!["stozyAmi".into()]),
                (3,2) => return Some(vec!["stozyAvaH".into()]),
                (3,3) => return Some(vec!["stozyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["stuyAt".into()]),
                (1,2) => return Some(vec!["stuyAtAm".into()]),
                (1,3) => return Some(vec!["stuyuH".into()]),
                (2,1) => return Some(vec!["stuyAH".into()]),
                (2,2) => return Some(vec!["stuyAtam".into()]),
                (2,3) => return Some(vec!["stuyAta".into()]),
                (3,1) => return Some(vec!["stuyAm".into()]),
                (3,2) => return Some(vec!["stuyAva".into()]),
                (3,3) => return Some(vec!["stuyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "brUY" || dhatu_query == "02.0039" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["Aha".into()]),
                (1,2) => return Some(vec!["AhatuH".into()]),
                (1,3) => return Some(vec!["AhuH".into()]),
                (2,1) => return Some(vec!["AtTa".into()]),
                (2,2) => return Some(vec!["AhaTuH".into()]),
                (2,3) => return Some(vec!["brUTa".into()]),
                (3,1) => return Some(vec!["bravImi".into()]),
                (3,2) => return Some(vec!["brUvaH".into()]),
                (3,3) => return Some(vec!["brUmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["abravIt".into()]),
                (1,2) => return Some(vec!["abrUtAm".into()]),
                (1,3) => return Some(vec!["abruvan".into()]),
                (2,1) => return Some(vec!["abravIH".into()]),
                (2,2) => return Some(vec!["abrUtam".into()]),
                (2,3) => return Some(vec!["abrUta".into()]),
                (3,1) => return Some(vec!["abravam".into()]),
                (3,2) => return Some(vec!["abrUva".into()]),
                (3,3) => return Some(vec!["abrUma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["bravItu".into()]),
                (1,2) => return Some(vec!["brUtAm".into()]),
                (1,3) => return Some(vec!["bruvantu".into()]),
                (2,1) => return Some(vec!["brUtAt".into()]),
                (2,2) => return Some(vec!["brUtam".into()]),
                (2,3) => return Some(vec!["brUta".into()]),
                (3,1) => return Some(vec!["bravARi".into()]),
                (3,2) => return Some(vec!["bravAva".into()]),
                (3,3) => return Some(vec!["bravAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vakzyati".into()]),
                (1,2) => return Some(vec!["vakzyataH".into()]),
                (1,3) => return Some(vec!["vakzyanti".into()]),
                (2,1) => return Some(vec!["vakzyasi".into()]),
                (2,2) => return Some(vec!["vakzyaTaH".into()]),
                (2,3) => return Some(vec!["vakzyaTa".into()]),
                (3,1) => return Some(vec!["vakzyAmi".into()]),
                (3,2) => return Some(vec!["vakzyAvaH".into()]),
                (3,3) => return Some(vec!["vakzyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["brUyAt".into()]),
                (1,2) => return Some(vec!["brUyAtAm".into()]),
                (1,3) => return Some(vec!["brUyuH".into()]),
                (2,1) => return Some(vec!["brUyAH".into()]),
                (2,2) => return Some(vec!["brUyAtam".into()]),
                (2,3) => return Some(vec!["brUyAta".into()]),
                (3,1) => return Some(vec!["brUyAm".into()]),
                (3,2) => return Some(vec!["brUyAva".into()]),
                (3,3) => return Some(vec!["brUyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "iR" || dhatu_query == "02.0040" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["eti".into()]),
                (1,2) => return Some(vec!["itaH".into()]),
                (1,3) => return Some(vec!["yanti".into()]),
                (2,1) => return Some(vec!["ezi".into()]),
                (2,2) => return Some(vec!["iTaH".into()]),
                (2,3) => return Some(vec!["iTa".into()]),
                (3,1) => return Some(vec!["emi".into()]),
                (3,2) => return Some(vec!["ivaH".into()]),
                (3,3) => return Some(vec!["imaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["Et".into()]),
                (1,2) => return Some(vec!["EtAm".into()]),
                (1,3) => return Some(vec!["Ayan".into()]),
                (2,1) => return Some(vec!["EH".into()]),
                (2,2) => return Some(vec!["Etam".into()]),
                (2,3) => return Some(vec!["Eta".into()]),
                (3,1) => return Some(vec!["Ayam".into()]),
                (3,2) => return Some(vec!["Eva".into()]),
                (3,3) => return Some(vec!["Ema".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["itAt".into()]),
                (1,2) => return Some(vec!["itAm".into()]),
                (1,3) => return Some(vec!["yantu".into()]),
                (2,1) => return Some(vec!["itAt".into()]),
                (2,2) => return Some(vec!["itam".into()]),
                (2,3) => return Some(vec!["ita".into()]),
                (3,1) => return Some(vec!["ayAni".into()]),
                (3,2) => return Some(vec!["ayAva".into()]),
                (3,3) => return Some(vec!["ayAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ezyati".into()]),
                (1,2) => return Some(vec!["ezyataH".into()]),
                (1,3) => return Some(vec!["ezyanti".into()]),
                (2,1) => return Some(vec!["ezyasi".into()]),
                (2,2) => return Some(vec!["ezyaTaH".into()]),
                (2,3) => return Some(vec!["ezyaTa".into()]),
                (3,1) => return Some(vec!["ezyAmi".into()]),
                (3,2) => return Some(vec!["ezyAvaH".into()]),
                (3,3) => return Some(vec!["ezyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["iyAt".into()]),
                (1,2) => return Some(vec!["iyAtAm".into()]),
                (1,3) => return Some(vec!["iyuH".into()]),
                (2,1) => return Some(vec!["iyAH".into()]),
                (2,2) => return Some(vec!["iyAtam".into()]),
                (2,3) => return Some(vec!["iyAta".into()]),
                (3,1) => return Some(vec!["iyAm".into()]),
                (3,2) => return Some(vec!["iyAva".into()]),
                (3,3) => return Some(vec!["iyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "ik" || dhatu_query == "02.0042" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aDyeti".into()]),
                (1,2) => return Some(vec!["aDItaH".into()]),
                (1,3) => return Some(vec!["aDiyanti".into()]),
                (2,1) => return Some(vec!["aDyezi".into()]),
                (2,2) => return Some(vec!["aDITaH".into()]),
                (2,3) => return Some(vec!["aDITa".into()]),
                (3,1) => return Some(vec!["aDyemi".into()]),
                (3,2) => return Some(vec!["aDIvaH".into()]),
                (3,3) => return Some(vec!["aDImaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aDyEt".into()]),
                (1,2) => return Some(vec!["aDyEtAm".into()]),
                (1,3) => return Some(vec!["aDyAyan".into()]),
                (2,1) => return Some(vec!["aDyEH".into()]),
                (2,2) => return Some(vec!["aDyEtam".into()]),
                (2,3) => return Some(vec!["aDyEta".into()]),
                (3,1) => return Some(vec!["aDyAyam".into()]),
                (3,2) => return Some(vec!["aDyEva".into()]),
                (3,3) => return Some(vec!["aDyEma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aDItAt".into()]),
                (1,2) => return Some(vec!["aDItAm".into()]),
                (1,3) => return Some(vec!["aDiyantu".into()]),
                (2,1) => return Some(vec!["aDItAt".into()]),
                (2,2) => return Some(vec!["aDItam".into()]),
                (2,3) => return Some(vec!["aDIta".into()]),
                (3,1) => return Some(vec!["aDyayAni".into()]),
                (3,2) => return Some(vec!["aDyayAva".into()]),
                (3,3) => return Some(vec!["aDyayAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aDyezyati".into()]),
                (1,2) => return Some(vec!["aDyezyataH".into()]),
                (1,3) => return Some(vec!["aDyezyanti".into()]),
                (2,1) => return Some(vec!["aDyezyasi".into()]),
                (2,2) => return Some(vec!["aDyezyaTaH".into()]),
                (2,3) => return Some(vec!["aDyezyaTa".into()]),
                (3,1) => return Some(vec!["aDyezyAmi".into()]),
                (3,2) => return Some(vec!["aDyezyAvaH".into()]),
                (3,3) => return Some(vec!["aDyezyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aDIyAt".into()]),
                (1,2) => return Some(vec!["aDIyAtAm".into()]),
                (1,3) => return Some(vec!["aDIyuH".into()]),
                (2,1) => return Some(vec!["aDIyAH".into()]),
                (2,2) => return Some(vec!["aDIyAtam".into()]),
                (2,3) => return Some(vec!["aDIyAta".into()]),
                (3,1) => return Some(vec!["aDIyAm".into()]),
                (3,2) => return Some(vec!["aDIyAva".into()]),
                (3,3) => return Some(vec!["aDIyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "vI" || dhatu_query == "02.0043" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["veti".into()]),
                (1,2) => return Some(vec!["vItaH".into()]),
                (1,3) => return Some(vec!["viyanti".into()]),
                (2,1) => return Some(vec!["vezi".into()]),
                (2,2) => return Some(vec!["vITaH".into()]),
                (2,3) => return Some(vec!["vITa".into()]),
                (3,1) => return Some(vec!["vemi".into()]),
                (3,2) => return Some(vec!["vIvaH".into()]),
                (3,3) => return Some(vec!["vImaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["avet".into()]),
                (1,2) => return Some(vec!["avItAm".into()]),
                (1,3) => return Some(vec!["aviyan".into()]),
                (2,1) => return Some(vec!["aveH".into()]),
                (2,2) => return Some(vec!["avItam".into()]),
                (2,3) => return Some(vec!["avIta".into()]),
                (3,1) => return Some(vec!["avayam".into()]),
                (3,2) => return Some(vec!["avIva".into()]),
                (3,3) => return Some(vec!["avIma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vItAt".into()]),
                (1,2) => return Some(vec!["vItAm".into()]),
                (1,3) => return Some(vec!["viyantu".into()]),
                (2,1) => return Some(vec!["vItAt".into()]),
                (2,2) => return Some(vec!["vItam".into()]),
                (2,3) => return Some(vec!["vIta".into()]),
                (3,1) => return Some(vec!["vayAni".into()]),
                (3,2) => return Some(vec!["vayAva".into()]),
                (3,3) => return Some(vec!["vayAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vezyati".into()]),
                (1,2) => return Some(vec!["vezyataH".into()]),
                (1,3) => return Some(vec!["vezyanti".into()]),
                (2,1) => return Some(vec!["vezyasi".into()]),
                (2,2) => return Some(vec!["vezyaTaH".into()]),
                (2,3) => return Some(vec!["vezyaTa".into()]),
                (3,1) => return Some(vec!["vezyAmi".into()]),
                (3,2) => return Some(vec!["vezyAvaH".into()]),
                (3,3) => return Some(vec!["vezyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vIyAt".into()]),
                (1,2) => return Some(vec!["vIyAtAm".into()]),
                (1,3) => return Some(vec!["vIyuH".into()]),
                (2,1) => return Some(vec!["vIyAH".into()]),
                (2,2) => return Some(vec!["vIyAtam".into()]),
                (2,3) => return Some(vec!["vIyAta".into()]),
                (3,1) => return Some(vec!["vIyAm".into()]),
                (3,2) => return Some(vec!["vIyAva".into()]),
                (3,3) => return Some(vec!["vIyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "yA" || dhatu_query == "02.0044" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yAti".into()]),
                (1,2) => return Some(vec!["yAtaH".into()]),
                (1,3) => return Some(vec!["yAnti".into()]),
                (2,1) => return Some(vec!["yAsi".into()]),
                (2,2) => return Some(vec!["yATaH".into()]),
                (2,3) => return Some(vec!["yATa".into()]),
                (3,1) => return Some(vec!["yAmi".into()]),
                (3,2) => return Some(vec!["yAvaH".into()]),
                (3,3) => return Some(vec!["yAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ayAt".into()]),
                (1,2) => return Some(vec!["ayAtAm".into()]),
                (1,3) => return Some(vec!["ayAn".into()]),
                (2,1) => return Some(vec!["ayAH".into()]),
                (2,2) => return Some(vec!["ayAtam".into()]),
                (2,3) => return Some(vec!["ayAta".into()]),
                (3,1) => return Some(vec!["ayAm".into()]),
                (3,2) => return Some(vec!["ayAva".into()]),
                (3,3) => return Some(vec!["ayAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yAtAt".into()]),
                (1,2) => return Some(vec!["yAtAm".into()]),
                (1,3) => return Some(vec!["yAntu".into()]),
                (2,1) => return Some(vec!["yAtAt".into()]),
                (2,2) => return Some(vec!["yAtam".into()]),
                (2,3) => return Some(vec!["yAta".into()]),
                (3,1) => return Some(vec!["yAni".into()]),
                (3,2) => return Some(vec!["yAva".into()]),
                (3,3) => return Some(vec!["yAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yAsyati".into()]),
                (1,2) => return Some(vec!["yAsyataH".into()]),
                (1,3) => return Some(vec!["yAsyanti".into()]),
                (2,1) => return Some(vec!["yAsyasi".into()]),
                (2,2) => return Some(vec!["yAsyaTaH".into()]),
                (2,3) => return Some(vec!["yAsyaTa".into()]),
                (3,1) => return Some(vec!["yAsyAmi".into()]),
                (3,2) => return Some(vec!["yAsyAvaH".into()]),
                (3,3) => return Some(vec!["yAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["yAyAt".into()]),
                (1,2) => return Some(vec!["yAyAtAm".into()]),
                (1,3) => return Some(vec!["yAyuH".into()]),
                (2,1) => return Some(vec!["yAyAH".into()]),
                (2,2) => return Some(vec!["yAyAtam".into()]),
                (2,3) => return Some(vec!["yAyAta".into()]),
                (3,1) => return Some(vec!["yAyAm".into()]),
                (3,2) => return Some(vec!["yAyAva".into()]),
                (3,3) => return Some(vec!["yAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "vA" || dhatu_query == "02.0045" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vAti".into()]),
                (1,2) => return Some(vec!["vAtaH".into()]),
                (1,3) => return Some(vec!["vAnti".into()]),
                (2,1) => return Some(vec!["vAsi".into()]),
                (2,2) => return Some(vec!["vATaH".into()]),
                (2,3) => return Some(vec!["vATa".into()]),
                (3,1) => return Some(vec!["vAmi".into()]),
                (3,2) => return Some(vec!["vAvaH".into()]),
                (3,3) => return Some(vec!["vAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["avAt".into()]),
                (1,2) => return Some(vec!["avAtAm".into()]),
                (1,3) => return Some(vec!["avAn".into()]),
                (2,1) => return Some(vec!["avAH".into()]),
                (2,2) => return Some(vec!["avAtam".into()]),
                (2,3) => return Some(vec!["avAta".into()]),
                (3,1) => return Some(vec!["avAm".into()]),
                (3,2) => return Some(vec!["avAva".into()]),
                (3,3) => return Some(vec!["avAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vAtAt".into()]),
                (1,2) => return Some(vec!["vAtAm".into()]),
                (1,3) => return Some(vec!["vAntu".into()]),
                (2,1) => return Some(vec!["vAtAt".into()]),
                (2,2) => return Some(vec!["vAtam".into()]),
                (2,3) => return Some(vec!["vAta".into()]),
                (3,1) => return Some(vec!["vAni".into()]),
                (3,2) => return Some(vec!["vAva".into()]),
                (3,3) => return Some(vec!["vAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vAsyati".into()]),
                (1,2) => return Some(vec!["vAsyataH".into()]),
                (1,3) => return Some(vec!["vAsyanti".into()]),
                (2,1) => return Some(vec!["vAsyasi".into()]),
                (2,2) => return Some(vec!["vAsyaTaH".into()]),
                (2,3) => return Some(vec!["vAsyaTa".into()]),
                (3,1) => return Some(vec!["vAsyAmi".into()]),
                (3,2) => return Some(vec!["vAsyAvaH".into()]),
                (3,3) => return Some(vec!["vAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vAyAt".into()]),
                (1,2) => return Some(vec!["vAyAtAm".into()]),
                (1,3) => return Some(vec!["vAyuH".into()]),
                (2,1) => return Some(vec!["vAyAH".into()]),
                (2,2) => return Some(vec!["vAyAtam".into()]),
                (2,3) => return Some(vec!["vAyAta".into()]),
                (3,1) => return Some(vec!["vAyAm".into()]),
                (3,2) => return Some(vec!["vAyAva".into()]),
                (3,3) => return Some(vec!["vAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "BA" || dhatu_query == "02.0046" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["BAti".into()]),
                (1,2) => return Some(vec!["BAtaH".into()]),
                (1,3) => return Some(vec!["BAnti".into()]),
                (2,1) => return Some(vec!["BAsi".into()]),
                (2,2) => return Some(vec!["BATaH".into()]),
                (2,3) => return Some(vec!["BATa".into()]),
                (3,1) => return Some(vec!["BAmi".into()]),
                (3,2) => return Some(vec!["BAvaH".into()]),
                (3,3) => return Some(vec!["BAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aBAt".into()]),
                (1,2) => return Some(vec!["aBAtAm".into()]),
                (1,3) => return Some(vec!["aBAn".into()]),
                (2,1) => return Some(vec!["aBAH".into()]),
                (2,2) => return Some(vec!["aBAtam".into()]),
                (2,3) => return Some(vec!["aBAta".into()]),
                (3,1) => return Some(vec!["aBAm".into()]),
                (3,2) => return Some(vec!["aBAva".into()]),
                (3,3) => return Some(vec!["aBAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["BAtAt".into()]),
                (1,2) => return Some(vec!["BAtAm".into()]),
                (1,3) => return Some(vec!["BAntu".into()]),
                (2,1) => return Some(vec!["BAtAt".into()]),
                (2,2) => return Some(vec!["BAtam".into()]),
                (2,3) => return Some(vec!["BAta".into()]),
                (3,1) => return Some(vec!["BAni".into()]),
                (3,2) => return Some(vec!["BAva".into()]),
                (3,3) => return Some(vec!["BAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["BAsyati".into()]),
                (1,2) => return Some(vec!["BAsyataH".into()]),
                (1,3) => return Some(vec!["BAsyanti".into()]),
                (2,1) => return Some(vec!["BAsyasi".into()]),
                (2,2) => return Some(vec!["BAsyaTaH".into()]),
                (2,3) => return Some(vec!["BAsyaTa".into()]),
                (3,1) => return Some(vec!["BAsyAmi".into()]),
                (3,2) => return Some(vec!["BAsyAvaH".into()]),
                (3,3) => return Some(vec!["BAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["BAyAt".into()]),
                (1,2) => return Some(vec!["BAyAtAm".into()]),
                (1,3) => return Some(vec!["BAyuH".into()]),
                (2,1) => return Some(vec!["BAyAH".into()]),
                (2,2) => return Some(vec!["BAyAtam".into()]),
                (2,3) => return Some(vec!["BAyAta".into()]),
                (3,1) => return Some(vec!["BAyAm".into()]),
                (3,2) => return Some(vec!["BAyAva".into()]),
                (3,3) => return Some(vec!["BAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "zRA" || dhatu_query == "02.0047" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["snAti".into()]),
                (1,2) => return Some(vec!["snAtaH".into()]),
                (1,3) => return Some(vec!["snAnti".into()]),
                (2,1) => return Some(vec!["snAsi".into()]),
                (2,2) => return Some(vec!["snATaH".into()]),
                (2,3) => return Some(vec!["snATa".into()]),
                (3,1) => return Some(vec!["snAmi".into()]),
                (3,2) => return Some(vec!["snAvaH".into()]),
                (3,3) => return Some(vec!["snAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["asnAt".into()]),
                (1,2) => return Some(vec!["asnAtAm".into()]),
                (1,3) => return Some(vec!["asnAn".into()]),
                (2,1) => return Some(vec!["asnAH".into()]),
                (2,2) => return Some(vec!["asnAtam".into()]),
                (2,3) => return Some(vec!["asnAta".into()]),
                (3,1) => return Some(vec!["asnAm".into()]),
                (3,2) => return Some(vec!["asnAva".into()]),
                (3,3) => return Some(vec!["asnAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["snAtAt".into()]),
                (1,2) => return Some(vec!["snAtAm".into()]),
                (1,3) => return Some(vec!["snAntu".into()]),
                (2,1) => return Some(vec!["snAtAt".into()]),
                (2,2) => return Some(vec!["snAtam".into()]),
                (2,3) => return Some(vec!["snAta".into()]),
                (3,1) => return Some(vec!["snAni".into()]),
                (3,2) => return Some(vec!["snAva".into()]),
                (3,3) => return Some(vec!["snAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["snAsyati".into()]),
                (1,2) => return Some(vec!["snAsyataH".into()]),
                (1,3) => return Some(vec!["snAsyanti".into()]),
                (2,1) => return Some(vec!["snAsyasi".into()]),
                (2,2) => return Some(vec!["snAsyaTaH".into()]),
                (2,3) => return Some(vec!["snAsyaTa".into()]),
                (3,1) => return Some(vec!["snAsyAmi".into()]),
                (3,2) => return Some(vec!["snAsyAvaH".into()]),
                (3,3) => return Some(vec!["snAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["snAyAt".into()]),
                (1,2) => return Some(vec!["snAyAtAm".into()]),
                (1,3) => return Some(vec!["snAyuH".into()]),
                (2,1) => return Some(vec!["snAyAH".into()]),
                (2,2) => return Some(vec!["snAyAtam".into()]),
                (2,3) => return Some(vec!["snAyAta".into()]),
                (3,1) => return Some(vec!["snAyAm".into()]),
                (3,2) => return Some(vec!["snAyAva".into()]),
                (3,3) => return Some(vec!["snAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "SrA" || dhatu_query == "02.0048" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["SrAti".into()]),
                (1,2) => return Some(vec!["SrAtaH".into()]),
                (1,3) => return Some(vec!["SrAnti".into()]),
                (2,1) => return Some(vec!["SrAsi".into()]),
                (2,2) => return Some(vec!["SrATaH".into()]),
                (2,3) => return Some(vec!["SrATa".into()]),
                (3,1) => return Some(vec!["SrAmi".into()]),
                (3,2) => return Some(vec!["SrAvaH".into()]),
                (3,3) => return Some(vec!["SrAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aSrAt".into()]),
                (1,2) => return Some(vec!["aSrAtAm".into()]),
                (1,3) => return Some(vec!["aSrAn".into()]),
                (2,1) => return Some(vec!["aSrAH".into()]),
                (2,2) => return Some(vec!["aSrAtam".into()]),
                (2,3) => return Some(vec!["aSrAta".into()]),
                (3,1) => return Some(vec!["aSrAm".into()]),
                (3,2) => return Some(vec!["aSrAva".into()]),
                (3,3) => return Some(vec!["aSrAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["SrAtAt".into()]),
                (1,2) => return Some(vec!["SrAtAm".into()]),
                (1,3) => return Some(vec!["SrAntu".into()]),
                (2,1) => return Some(vec!["SrAtAt".into()]),
                (2,2) => return Some(vec!["SrAtam".into()]),
                (2,3) => return Some(vec!["SrAta".into()]),
                (3,1) => return Some(vec!["SrARi".into()]),
                (3,2) => return Some(vec!["SrAva".into()]),
                (3,3) => return Some(vec!["SrAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["SrAsyati".into()]),
                (1,2) => return Some(vec!["SrAsyataH".into()]),
                (1,3) => return Some(vec!["SrAsyanti".into()]),
                (2,1) => return Some(vec!["SrAsyasi".into()]),
                (2,2) => return Some(vec!["SrAsyaTaH".into()]),
                (2,3) => return Some(vec!["SrAsyaTa".into()]),
                (3,1) => return Some(vec!["SrAsyAmi".into()]),
                (3,2) => return Some(vec!["SrAsyAvaH".into()]),
                (3,3) => return Some(vec!["SrAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["SrAyAt".into()]),
                (1,2) => return Some(vec!["SrAyAtAm".into()]),
                (1,3) => return Some(vec!["SrAyuH".into()]),
                (2,1) => return Some(vec!["SrAyAH".into()]),
                (2,2) => return Some(vec!["SrAyAtam".into()]),
                (2,3) => return Some(vec!["SrAyAta".into()]),
                (3,1) => return Some(vec!["SrAyAm".into()]),
                (3,2) => return Some(vec!["SrAyAva".into()]),
                (3,3) => return Some(vec!["SrAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "drA" || dhatu_query == "02.0049" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["drAti".into()]),
                (1,2) => return Some(vec!["drAtaH".into()]),
                (1,3) => return Some(vec!["drAnti".into()]),
                (2,1) => return Some(vec!["drAsi".into()]),
                (2,2) => return Some(vec!["drATaH".into()]),
                (2,3) => return Some(vec!["drATa".into()]),
                (3,1) => return Some(vec!["drAmi".into()]),
                (3,2) => return Some(vec!["drAvaH".into()]),
                (3,3) => return Some(vec!["drAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["adrAt".into()]),
                (1,2) => return Some(vec!["adrAtAm".into()]),
                (1,3) => return Some(vec!["adrAn".into()]),
                (2,1) => return Some(vec!["adrAH".into()]),
                (2,2) => return Some(vec!["adrAtam".into()]),
                (2,3) => return Some(vec!["adrAta".into()]),
                (3,1) => return Some(vec!["adrAm".into()]),
                (3,2) => return Some(vec!["adrAva".into()]),
                (3,3) => return Some(vec!["adrAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["drAtAt".into()]),
                (1,2) => return Some(vec!["drAtAm".into()]),
                (1,3) => return Some(vec!["drAntu".into()]),
                (2,1) => return Some(vec!["drAtAt".into()]),
                (2,2) => return Some(vec!["drAtam".into()]),
                (2,3) => return Some(vec!["drAta".into()]),
                (3,1) => return Some(vec!["drARi".into()]),
                (3,2) => return Some(vec!["drAva".into()]),
                (3,3) => return Some(vec!["drAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["drAsyati".into()]),
                (1,2) => return Some(vec!["drAsyataH".into()]),
                (1,3) => return Some(vec!["drAsyanti".into()]),
                (2,1) => return Some(vec!["drAsyasi".into()]),
                (2,2) => return Some(vec!["drAsyaTaH".into()]),
                (2,3) => return Some(vec!["drAsyaTa".into()]),
                (3,1) => return Some(vec!["drAsyAmi".into()]),
                (3,2) => return Some(vec!["drAsyAvaH".into()]),
                (3,3) => return Some(vec!["drAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["drAyAt".into()]),
                (1,2) => return Some(vec!["drAyAtAm".into()]),
                (1,3) => return Some(vec!["drAyuH".into()]),
                (2,1) => return Some(vec!["drAyAH".into()]),
                (2,2) => return Some(vec!["drAyAtam".into()]),
                (2,3) => return Some(vec!["drAyAta".into()]),
                (3,1) => return Some(vec!["drAyAm".into()]),
                (3,2) => return Some(vec!["drAyAva".into()]),
                (3,3) => return Some(vec!["drAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "psA" || dhatu_query == "02.0050" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["psAti".into()]),
                (1,2) => return Some(vec!["psAtaH".into()]),
                (1,3) => return Some(vec!["psAnti".into()]),
                (2,1) => return Some(vec!["psAsi".into()]),
                (2,2) => return Some(vec!["psATaH".into()]),
                (2,3) => return Some(vec!["psATa".into()]),
                (3,1) => return Some(vec!["psAmi".into()]),
                (3,2) => return Some(vec!["psAvaH".into()]),
                (3,3) => return Some(vec!["psAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["apsAt".into()]),
                (1,2) => return Some(vec!["apsAtAm".into()]),
                (1,3) => return Some(vec!["apsAn".into()]),
                (2,1) => return Some(vec!["apsAH".into()]),
                (2,2) => return Some(vec!["apsAtam".into()]),
                (2,3) => return Some(vec!["apsAta".into()]),
                (3,1) => return Some(vec!["apsAm".into()]),
                (3,2) => return Some(vec!["apsAva".into()]),
                (3,3) => return Some(vec!["apsAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["psAtAt".into()]),
                (1,2) => return Some(vec!["psAtAm".into()]),
                (1,3) => return Some(vec!["psAntu".into()]),
                (2,1) => return Some(vec!["psAtAt".into()]),
                (2,2) => return Some(vec!["psAtam".into()]),
                (2,3) => return Some(vec!["psAta".into()]),
                (3,1) => return Some(vec!["psAni".into()]),
                (3,2) => return Some(vec!["psAva".into()]),
                (3,3) => return Some(vec!["psAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["psAsyati".into()]),
                (1,2) => return Some(vec!["psAsyataH".into()]),
                (1,3) => return Some(vec!["psAsyanti".into()]),
                (2,1) => return Some(vec!["psAsyasi".into()]),
                (2,2) => return Some(vec!["psAsyaTaH".into()]),
                (2,3) => return Some(vec!["psAsyaTa".into()]),
                (3,1) => return Some(vec!["psAsyAmi".into()]),
                (3,2) => return Some(vec!["psAsyAvaH".into()]),
                (3,3) => return Some(vec!["psAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["psAyAt".into()]),
                (1,2) => return Some(vec!["psAyAtAm".into()]),
                (1,3) => return Some(vec!["psAyuH".into()]),
                (2,1) => return Some(vec!["psAyAH".into()]),
                (2,2) => return Some(vec!["psAyAtam".into()]),
                (2,3) => return Some(vec!["psAyAta".into()]),
                (3,1) => return Some(vec!["psAyAm".into()]),
                (3,2) => return Some(vec!["psAyAva".into()]),
                (3,3) => return Some(vec!["psAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "pA" || dhatu_query == "02.0051" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["pAti".into()]),
                (1,2) => return Some(vec!["pAtaH".into()]),
                (1,3) => return Some(vec!["pAnti".into()]),
                (2,1) => return Some(vec!["pAsi".into()]),
                (2,2) => return Some(vec!["pATaH".into()]),
                (2,3) => return Some(vec!["pATa".into()]),
                (3,1) => return Some(vec!["pAmi".into()]),
                (3,2) => return Some(vec!["pAvaH".into()]),
                (3,3) => return Some(vec!["pAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["apAt".into()]),
                (1,2) => return Some(vec!["apAtAm".into()]),
                (1,3) => return Some(vec!["apAn".into()]),
                (2,1) => return Some(vec!["apAH".into()]),
                (2,2) => return Some(vec!["apAtam".into()]),
                (2,3) => return Some(vec!["apAta".into()]),
                (3,1) => return Some(vec!["apAm".into()]),
                (3,2) => return Some(vec!["apAva".into()]),
                (3,3) => return Some(vec!["apAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["pAtAt".into()]),
                (1,2) => return Some(vec!["pAtAm".into()]),
                (1,3) => return Some(vec!["pAntu".into()]),
                (2,1) => return Some(vec!["pAtAt".into()]),
                (2,2) => return Some(vec!["pAtam".into()]),
                (2,3) => return Some(vec!["pAta".into()]),
                (3,1) => return Some(vec!["pAni".into()]),
                (3,2) => return Some(vec!["pAva".into()]),
                (3,3) => return Some(vec!["pAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["pAsyati".into()]),
                (1,2) => return Some(vec!["pAsyataH".into()]),
                (1,3) => return Some(vec!["pAsyanti".into()]),
                (2,1) => return Some(vec!["pAsyasi".into()]),
                (2,2) => return Some(vec!["pAsyaTaH".into()]),
                (2,3) => return Some(vec!["pAsyaTa".into()]),
                (3,1) => return Some(vec!["pAsyAmi".into()]),
                (3,2) => return Some(vec!["pAsyAvaH".into()]),
                (3,3) => return Some(vec!["pAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["pAyAt".into()]),
                (1,2) => return Some(vec!["pAyAtAm".into()]),
                (1,3) => return Some(vec!["pAyuH".into()]),
                (2,1) => return Some(vec!["pAyAH".into()]),
                (2,2) => return Some(vec!["pAyAtam".into()]),
                (2,3) => return Some(vec!["pAyAta".into()]),
                (3,1) => return Some(vec!["pAyAm".into()]),
                (3,2) => return Some(vec!["pAyAva".into()]),
                (3,3) => return Some(vec!["pAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "rA" || dhatu_query == "02.0052" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["rAti".into()]),
                (1,2) => return Some(vec!["rAtaH".into()]),
                (1,3) => return Some(vec!["rAnti".into()]),
                (2,1) => return Some(vec!["rAsi".into()]),
                (2,2) => return Some(vec!["rATaH".into()]),
                (2,3) => return Some(vec!["rATa".into()]),
                (3,1) => return Some(vec!["rAmi".into()]),
                (3,2) => return Some(vec!["rAvaH".into()]),
                (3,3) => return Some(vec!["rAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["arAt".into()]),
                (1,2) => return Some(vec!["arAtAm".into()]),
                (1,3) => return Some(vec!["arAn".into()]),
                (2,1) => return Some(vec!["arAH".into()]),
                (2,2) => return Some(vec!["arAtam".into()]),
                (2,3) => return Some(vec!["arAta".into()]),
                (3,1) => return Some(vec!["arAm".into()]),
                (3,2) => return Some(vec!["arAva".into()]),
                (3,3) => return Some(vec!["arAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["rAtAt".into()]),
                (1,2) => return Some(vec!["rAtAm".into()]),
                (1,3) => return Some(vec!["rAntu".into()]),
                (2,1) => return Some(vec!["rAtAt".into()]),
                (2,2) => return Some(vec!["rAtam".into()]),
                (2,3) => return Some(vec!["rAta".into()]),
                (3,1) => return Some(vec!["rARi".into()]),
                (3,2) => return Some(vec!["rAva".into()]),
                (3,3) => return Some(vec!["rAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["rAsyati".into()]),
                (1,2) => return Some(vec!["rAsyataH".into()]),
                (1,3) => return Some(vec!["rAsyanti".into()]),
                (2,1) => return Some(vec!["rAsyasi".into()]),
                (2,2) => return Some(vec!["rAsyaTaH".into()]),
                (2,3) => return Some(vec!["rAsyaTa".into()]),
                (3,1) => return Some(vec!["rAsyAmi".into()]),
                (3,2) => return Some(vec!["rAsyAvaH".into()]),
                (3,3) => return Some(vec!["rAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["rAyAt".into()]),
                (1,2) => return Some(vec!["rAyAtAm".into()]),
                (1,3) => return Some(vec!["rAyuH".into()]),
                (2,1) => return Some(vec!["rAyAH".into()]),
                (2,2) => return Some(vec!["rAyAtam".into()]),
                (2,3) => return Some(vec!["rAyAta".into()]),
                (3,1) => return Some(vec!["rAyAm".into()]),
                (3,2) => return Some(vec!["rAyAva".into()]),
                (3,3) => return Some(vec!["rAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "lA" || dhatu_query == "02.0053" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["lAti".into()]),
                (1,2) => return Some(vec!["lAtaH".into()]),
                (1,3) => return Some(vec!["lAnti".into()]),
                (2,1) => return Some(vec!["lAsi".into()]),
                (2,2) => return Some(vec!["lATaH".into()]),
                (2,3) => return Some(vec!["lATa".into()]),
                (3,1) => return Some(vec!["lAmi".into()]),
                (3,2) => return Some(vec!["lAvaH".into()]),
                (3,3) => return Some(vec!["lAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["alAt".into()]),
                (1,2) => return Some(vec!["alAtAm".into()]),
                (1,3) => return Some(vec!["alAn".into()]),
                (2,1) => return Some(vec!["alAH".into()]),
                (2,2) => return Some(vec!["alAtam".into()]),
                (2,3) => return Some(vec!["alAta".into()]),
                (3,1) => return Some(vec!["alAm".into()]),
                (3,2) => return Some(vec!["alAva".into()]),
                (3,3) => return Some(vec!["alAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["lAtAt".into()]),
                (1,2) => return Some(vec!["lAtAm".into()]),
                (1,3) => return Some(vec!["lAntu".into()]),
                (2,1) => return Some(vec!["lAtAt".into()]),
                (2,2) => return Some(vec!["lAtam".into()]),
                (2,3) => return Some(vec!["lAta".into()]),
                (3,1) => return Some(vec!["lAni".into()]),
                (3,2) => return Some(vec!["lAva".into()]),
                (3,3) => return Some(vec!["lAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["lAsyati".into()]),
                (1,2) => return Some(vec!["lAsyataH".into()]),
                (1,3) => return Some(vec!["lAsyanti".into()]),
                (2,1) => return Some(vec!["lAsyasi".into()]),
                (2,2) => return Some(vec!["lAsyaTaH".into()]),
                (2,3) => return Some(vec!["lAsyaTa".into()]),
                (3,1) => return Some(vec!["lAsyAmi".into()]),
                (3,2) => return Some(vec!["lAsyAvaH".into()]),
                (3,3) => return Some(vec!["lAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["lAyAt".into()]),
                (1,2) => return Some(vec!["lAyAtAm".into()]),
                (1,3) => return Some(vec!["lAyuH".into()]),
                (2,1) => return Some(vec!["lAyAH".into()]),
                (2,2) => return Some(vec!["lAyAtam".into()]),
                (2,3) => return Some(vec!["lAyAta".into()]),
                (3,1) => return Some(vec!["lAyAm".into()]),
                (3,2) => return Some(vec!["lAyAva".into()]),
                (3,3) => return Some(vec!["lAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "dAp" || dhatu_query == "02.0054" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dAti".into()]),
                (1,2) => return Some(vec!["dAtaH".into()]),
                (1,3) => return Some(vec!["dAnti".into()]),
                (2,1) => return Some(vec!["dAsi".into()]),
                (2,2) => return Some(vec!["dATaH".into()]),
                (2,3) => return Some(vec!["dATa".into()]),
                (3,1) => return Some(vec!["dAmi".into()]),
                (3,2) => return Some(vec!["dAvaH".into()]),
                (3,3) => return Some(vec!["dAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["adAt".into()]),
                (1,2) => return Some(vec!["adAtAm".into()]),
                (1,3) => return Some(vec!["adAn".into()]),
                (2,1) => return Some(vec!["adAH".into()]),
                (2,2) => return Some(vec!["adAtam".into()]),
                (2,3) => return Some(vec!["adAta".into()]),
                (3,1) => return Some(vec!["adAm".into()]),
                (3,2) => return Some(vec!["adAva".into()]),
                (3,3) => return Some(vec!["adAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dAtAt".into()]),
                (1,2) => return Some(vec!["dAtAm".into()]),
                (1,3) => return Some(vec!["dAntu".into()]),
                (2,1) => return Some(vec!["dAtAt".into()]),
                (2,2) => return Some(vec!["dAtam".into()]),
                (2,3) => return Some(vec!["dAta".into()]),
                (3,1) => return Some(vec!["dAni".into()]),
                (3,2) => return Some(vec!["dAva".into()]),
                (3,3) => return Some(vec!["dAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dAsyati".into()]),
                (1,2) => return Some(vec!["dAsyataH".into()]),
                (1,3) => return Some(vec!["dAsyanti".into()]),
                (2,1) => return Some(vec!["dAsyasi".into()]),
                (2,2) => return Some(vec!["dAsyaTaH".into()]),
                (2,3) => return Some(vec!["dAsyaTa".into()]),
                (3,1) => return Some(vec!["dAsyAmi".into()]),
                (3,2) => return Some(vec!["dAsyAvaH".into()]),
                (3,3) => return Some(vec!["dAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["dAyAt".into()]),
                (1,2) => return Some(vec!["dAyAtAm".into()]),
                (1,3) => return Some(vec!["dAyuH".into()]),
                (2,1) => return Some(vec!["dAyAH".into()]),
                (2,2) => return Some(vec!["dAyAtam".into()]),
                (2,3) => return Some(vec!["dAyAta".into()]),
                (3,1) => return Some(vec!["dAyAm".into()]),
                (3,2) => return Some(vec!["dAyAva".into()]),
                (3,3) => return Some(vec!["dAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "KyA" || dhatu_query == "02.0055" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["KyAti".into()]),
                (1,2) => return Some(vec!["KyAtaH".into()]),
                (1,3) => return Some(vec!["KyAnti".into()]),
                (2,1) => return Some(vec!["KyAsi".into()]),
                (2,2) => return Some(vec!["KyATaH".into()]),
                (2,3) => return Some(vec!["KyATa".into()]),
                (3,1) => return Some(vec!["KyAmi".into()]),
                (3,2) => return Some(vec!["KyAvaH".into()]),
                (3,3) => return Some(vec!["KyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aKyAt".into()]),
                (1,2) => return Some(vec!["aKyAtAm".into()]),
                (1,3) => return Some(vec!["aKyAn".into()]),
                (2,1) => return Some(vec!["aKyAH".into()]),
                (2,2) => return Some(vec!["aKyAtam".into()]),
                (2,3) => return Some(vec!["aKyAta".into()]),
                (3,1) => return Some(vec!["aKyAm".into()]),
                (3,2) => return Some(vec!["aKyAva".into()]),
                (3,3) => return Some(vec!["aKyAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["KyAtAt".into()]),
                (1,2) => return Some(vec!["KyAtAm".into()]),
                (1,3) => return Some(vec!["KyAntu".into()]),
                (2,1) => return Some(vec!["KyAtAt".into()]),
                (2,2) => return Some(vec!["KyAtam".into()]),
                (2,3) => return Some(vec!["KyAta".into()]),
                (3,1) => return Some(vec!["KyAni".into()]),
                (3,2) => return Some(vec!["KyAva".into()]),
                (3,3) => return Some(vec!["KyAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["KyAsyati".into()]),
                (1,2) => return Some(vec!["KyAsyataH".into()]),
                (1,3) => return Some(vec!["KyAsyanti".into()]),
                (2,1) => return Some(vec!["KyAsyasi".into()]),
                (2,2) => return Some(vec!["KyAsyaTaH".into()]),
                (2,3) => return Some(vec!["KyAsyaTa".into()]),
                (3,1) => return Some(vec!["KyAsyAmi".into()]),
                (3,2) => return Some(vec!["KyAsyAvaH".into()]),
                (3,3) => return Some(vec!["KyAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["KyAyAt".into()]),
                (1,2) => return Some(vec!["KyAyAtAm".into()]),
                (1,3) => return Some(vec!["KyAyuH".into()]),
                (2,1) => return Some(vec!["KyAyAH".into()]),
                (2,2) => return Some(vec!["KyAyAtam".into()]),
                (2,3) => return Some(vec!["KyAyAta".into()]),
                (3,1) => return Some(vec!["KyAyAm".into()]),
                (3,2) => return Some(vec!["KyAyAva".into()]),
                (3,3) => return Some(vec!["KyAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "prA" || dhatu_query == "02.0056" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["prAti".into()]),
                (1,2) => return Some(vec!["prAtaH".into()]),
                (1,3) => return Some(vec!["prAnti".into()]),
                (2,1) => return Some(vec!["prAsi".into()]),
                (2,2) => return Some(vec!["prATaH".into()]),
                (2,3) => return Some(vec!["prATa".into()]),
                (3,1) => return Some(vec!["prAmi".into()]),
                (3,2) => return Some(vec!["prAvaH".into()]),
                (3,3) => return Some(vec!["prAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aprAt".into()]),
                (1,2) => return Some(vec!["aprAtAm".into()]),
                (1,3) => return Some(vec!["aprAn".into()]),
                (2,1) => return Some(vec!["aprAH".into()]),
                (2,2) => return Some(vec!["aprAtam".into()]),
                (2,3) => return Some(vec!["aprAta".into()]),
                (3,1) => return Some(vec!["aprAm".into()]),
                (3,2) => return Some(vec!["aprAva".into()]),
                (3,3) => return Some(vec!["aprAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["prAtAt".into()]),
                (1,2) => return Some(vec!["prAtAm".into()]),
                (1,3) => return Some(vec!["prAntu".into()]),
                (2,1) => return Some(vec!["prAtAt".into()]),
                (2,2) => return Some(vec!["prAtam".into()]),
                (2,3) => return Some(vec!["prAta".into()]),
                (3,1) => return Some(vec!["prARi".into()]),
                (3,2) => return Some(vec!["prAva".into()]),
                (3,3) => return Some(vec!["prAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["prAsyati".into()]),
                (1,2) => return Some(vec!["prAsyataH".into()]),
                (1,3) => return Some(vec!["prAsyanti".into()]),
                (2,1) => return Some(vec!["prAsyasi".into()]),
                (2,2) => return Some(vec!["prAsyaTaH".into()]),
                (2,3) => return Some(vec!["prAsyaTa".into()]),
                (3,1) => return Some(vec!["prAsyAmi".into()]),
                (3,2) => return Some(vec!["prAsyAvaH".into()]),
                (3,3) => return Some(vec!["prAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["prAyAt".into()]),
                (1,2) => return Some(vec!["prAyAtAm".into()]),
                (1,3) => return Some(vec!["prAyuH".into()]),
                (2,1) => return Some(vec!["prAyAH".into()]),
                (2,2) => return Some(vec!["prAyAtam".into()]),
                (2,3) => return Some(vec!["prAyAta".into()]),
                (3,1) => return Some(vec!["prAyAm".into()]),
                (3,2) => return Some(vec!["prAyAva".into()]),
                (3,3) => return Some(vec!["prAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "mA" || dhatu_query == "02.0057" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["mAti".into()]),
                (1,2) => return Some(vec!["mAtaH".into()]),
                (1,3) => return Some(vec!["mAnti".into()]),
                (2,1) => return Some(vec!["mAsi".into()]),
                (2,2) => return Some(vec!["mATaH".into()]),
                (2,3) => return Some(vec!["mATa".into()]),
                (3,1) => return Some(vec!["mAmi".into()]),
                (3,2) => return Some(vec!["mAvaH".into()]),
                (3,3) => return Some(vec!["mAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["amAt".into()]),
                (1,2) => return Some(vec!["amAtAm".into()]),
                (1,3) => return Some(vec!["amAn".into()]),
                (2,1) => return Some(vec!["amAH".into()]),
                (2,2) => return Some(vec!["amAtam".into()]),
                (2,3) => return Some(vec!["amAta".into()]),
                (3,1) => return Some(vec!["amAm".into()]),
                (3,2) => return Some(vec!["amAva".into()]),
                (3,3) => return Some(vec!["amAma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["mAtAt".into()]),
                (1,2) => return Some(vec!["mAtAm".into()]),
                (1,3) => return Some(vec!["mAntu".into()]),
                (2,1) => return Some(vec!["mAtAt".into()]),
                (2,2) => return Some(vec!["mAtam".into()]),
                (2,3) => return Some(vec!["mAta".into()]),
                (3,1) => return Some(vec!["mAni".into()]),
                (3,2) => return Some(vec!["mAva".into()]),
                (3,3) => return Some(vec!["mAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["mAsyati".into()]),
                (1,2) => return Some(vec!["mAsyataH".into()]),
                (1,3) => return Some(vec!["mAsyanti".into()]),
                (2,1) => return Some(vec!["mAsyasi".into()]),
                (2,2) => return Some(vec!["mAsyaTaH".into()]),
                (2,3) => return Some(vec!["mAsyaTa".into()]),
                (3,1) => return Some(vec!["mAsyAmi".into()]),
                (3,2) => return Some(vec!["mAsyAvaH".into()]),
                (3,3) => return Some(vec!["mAsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["mAyAt".into()]),
                (1,2) => return Some(vec!["mAyAtAm".into()]),
                (1,3) => return Some(vec!["mAyuH".into()]),
                (2,1) => return Some(vec!["mAyAH".into()]),
                (2,2) => return Some(vec!["mAyAtam".into()]),
                (2,3) => return Some(vec!["mAyAta".into()]),
                (3,1) => return Some(vec!["mAyAm".into()]),
                (3,2) => return Some(vec!["mAyAva".into()]),
                (3,3) => return Some(vec!["mAyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "vaca" || dhatu_query == "02.0058" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vakti".into()]),
                (1,2) => return Some(vec!["vaktaH".into()]),
                (1,3) => return Some(vec!["vacanti".into()]),
                (2,1) => return Some(vec!["vakzi".into()]),
                (2,2) => return Some(vec!["vakTaH".into()]),
                (2,3) => return Some(vec!["vakTa".into()]),
                (3,1) => return Some(vec!["vacmi".into()]),
                (3,2) => return Some(vec!["vacvaH".into()]),
                (3,3) => return Some(vec!["vacmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["avak".into()]),
                (1,2) => return Some(vec!["avaktAm".into()]),
                (1,3) => return Some(vec!["avacan".into()]),
                (2,1) => return Some(vec!["avak".into()]),
                (2,2) => return Some(vec!["avaktam".into()]),
                (2,3) => return Some(vec!["avakta".into()]),
                (3,1) => return Some(vec!["avacam".into()]),
                (3,2) => return Some(vec!["avacva".into()]),
                (3,3) => return Some(vec!["avacma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vaktAt".into()]),
                (1,2) => return Some(vec!["vaktAm".into()]),
                (1,3) => return Some(vec!["vacantu".into()]),
                (2,1) => return Some(vec!["vaktAt".into()]),
                (2,2) => return Some(vec!["vaktam".into()]),
                (2,3) => return Some(vec!["vakta".into()]),
                (3,1) => return Some(vec!["vacAni".into()]),
                (3,2) => return Some(vec!["vacAva".into()]),
                (3,3) => return Some(vec!["vacAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vakzyati".into()]),
                (1,2) => return Some(vec!["vakzyataH".into()]),
                (1,3) => return Some(vec!["vakzyanti".into()]),
                (2,1) => return Some(vec!["vakzyasi".into()]),
                (2,2) => return Some(vec!["vakzyaTaH".into()]),
                (2,3) => return Some(vec!["vakzyaTa".into()]),
                (3,1) => return Some(vec!["vakzyAmi".into()]),
                (3,2) => return Some(vec!["vakzyAvaH".into()]),
                (3,3) => return Some(vec!["vakzyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vacyAt".into()]),
                (1,2) => return Some(vec!["vacyAtAm".into()]),
                (1,3) => return Some(vec!["vacyuH".into()]),
                (2,1) => return Some(vec!["vacyAH".into()]),
                (2,2) => return Some(vec!["vacyAtam".into()]),
                (2,3) => return Some(vec!["vacyAta".into()]),
                (3,1) => return Some(vec!["vacyAm".into()]),
                (3,2) => return Some(vec!["vacyAva".into()]),
                (3,3) => return Some(vec!["vacyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "vida" || dhatu_query == "02.0059" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vetti".into()]),
                (1,2) => return Some(vec!["vittaH".into()]),
                (1,3) => return Some(vec!["vidanti".into()]),
                (2,1) => return Some(vec!["vetTa".into()]),
                (2,2) => return Some(vec!["vitTaH".into()]),
                (2,3) => return Some(vec!["vitTa".into()]),
                (3,1) => return Some(vec!["veda".into()]),
                (3,2) => return Some(vec!["vidva".into()]),
                (3,3) => return Some(vec!["vidma".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["avet".into()]),
                (1,2) => return Some(vec!["avittAm".into()]),
                (1,3) => return Some(vec!["aviduH".into()]),
                (2,1) => return Some(vec!["aveH".into()]),
                (2,2) => return Some(vec!["avittam".into()]),
                (2,3) => return Some(vec!["avitta".into()]),
                (3,1) => return Some(vec!["avedam".into()]),
                (3,2) => return Some(vec!["avidva".into()]),
                (3,3) => return Some(vec!["avidma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vittAt".into()]),
                (1,2) => return Some(vec!["vittAm".into()]),
                (1,3) => return Some(vec!["vidantu".into()]),
                (2,1) => return Some(vec!["vittAt".into()]),
                (2,2) => return Some(vec!["vittam".into()]),
                (2,3) => return Some(vec!["vitta".into()]),
                (3,1) => return Some(vec!["vidANkaravARi".into()]),
                (3,2) => return Some(vec!["vidANkaravAva".into()]),
                (3,3) => return Some(vec!["vidANkaravAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vedizyati".into()]),
                (1,2) => return Some(vec!["vedizyataH".into()]),
                (1,3) => return Some(vec!["vedizyanti".into()]),
                (2,1) => return Some(vec!["vedizyasi".into()]),
                (2,2) => return Some(vec!["vedizyaTaH".into()]),
                (2,3) => return Some(vec!["vedizyaTa".into()]),
                (3,1) => return Some(vec!["vedizyAmi".into()]),
                (3,2) => return Some(vec!["vedizyAvaH".into()]),
                (3,3) => return Some(vec!["vedizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vidyAt".into()]),
                (1,2) => return Some(vec!["vidyAtAm".into()]),
                (1,3) => return Some(vec!["vidyuH".into()]),
                (2,1) => return Some(vec!["vidyAH".into()]),
                (2,2) => return Some(vec!["vidyAtam".into()]),
                (2,3) => return Some(vec!["vidyAta".into()]),
                (3,1) => return Some(vec!["vidyAm".into()]),
                (3,2) => return Some(vec!["vidyAva".into()]),
                (3,3) => return Some(vec!["vidyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "asa" || dhatu_query == "02.0060" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["asti".into()]),
                (1,2) => return Some(vec!["staH".into()]),
                (1,3) => return Some(vec!["santi".into()]),
                (2,1) => return Some(vec!["asi".into()]),
                (2,2) => return Some(vec!["sTaH".into()]),
                (2,3) => return Some(vec!["sTa".into()]),
                (3,1) => return Some(vec!["asmi".into()]),
                (3,2) => return Some(vec!["svaH".into()]),
                (3,3) => return Some(vec!["smaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["AsIt".into()]),
                (1,2) => return Some(vec!["AstAm".into()]),
                (1,3) => return Some(vec!["Asan".into()]),
                (2,1) => return Some(vec!["AsIH".into()]),
                (2,2) => return Some(vec!["Astam".into()]),
                (2,3) => return Some(vec!["Asta".into()]),
                (3,1) => return Some(vec!["Asam".into()]),
                (3,2) => return Some(vec!["Asva".into()]),
                (3,3) => return Some(vec!["Asma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["astu".into()]),
                (1,2) => return Some(vec!["stAm".into()]),
                (1,3) => return Some(vec!["santu".into()]),
                (2,1) => return Some(vec!["eDi".into()]),
                (2,2) => return Some(vec!["stam".into()]),
                (2,3) => return Some(vec!["sta".into()]),
                (3,1) => return Some(vec!["asAni".into()]),
                (3,2) => return Some(vec!["asAva".into()]),
                (3,3) => return Some(vec!["asAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["Bavizyati".into()]),
                (1,2) => return Some(vec!["BavizyataH".into()]),
                (1,3) => return Some(vec!["Bavizyanti".into()]),
                (2,1) => return Some(vec!["Bavizyasi".into()]),
                (2,2) => return Some(vec!["BavizyaTaH".into()]),
                (2,3) => return Some(vec!["BavizyaTa".into()]),
                (3,1) => return Some(vec!["BavizyAmi".into()]),
                (3,2) => return Some(vec!["BavizyAvaH".into()]),
                (3,3) => return Some(vec!["BavizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["syAt".into()]),
                (1,2) => return Some(vec!["syAtAm".into()]),
                (1,3) => return Some(vec!["syuH".into()]),
                (2,1) => return Some(vec!["syAH".into()]),
                (2,2) => return Some(vec!["syAtam".into()]),
                (2,3) => return Some(vec!["syAta".into()]),
                (3,1) => return Some(vec!["syAm".into()]),
                (3,2) => return Some(vec!["syAva".into()]),
                (3,3) => return Some(vec!["syAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "mfjU" || dhatu_query == "02.0061" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["mArzwi".into()]),
                (1,2) => return Some(vec!["mfzwaH".into()]),
                (1,3) => return Some(vec!["mArjanti".into()]),
                (2,1) => return Some(vec!["mArkzi".into()]),
                (2,2) => return Some(vec!["mfzWaH".into()]),
                (2,3) => return Some(vec!["mfzWa".into()]),
                (3,1) => return Some(vec!["mArjmi".into()]),
                (3,2) => return Some(vec!["mfjvaH".into()]),
                (3,3) => return Some(vec!["mfjmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["amArw".into()]),
                (1,2) => return Some(vec!["amfzwAm".into()]),
                (1,3) => return Some(vec!["amArjan".into()]),
                (2,1) => return Some(vec!["amArw".into()]),
                (2,2) => return Some(vec!["amfzwam".into()]),
                (2,3) => return Some(vec!["amfzwa".into()]),
                (3,1) => return Some(vec!["amArjam".into()]),
                (3,2) => return Some(vec!["amfjva".into()]),
                (3,3) => return Some(vec!["amfjma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["mArzwu".into()]),
                (1,2) => return Some(vec!["mfzwAm".into()]),
                (1,3) => return Some(vec!["mArjantu".into()]),
                (2,1) => return Some(vec!["mfqQi".into()]),
                (2,2) => return Some(vec!["mfzwam".into()]),
                (2,3) => return Some(vec!["mfzwa".into()]),
                (3,1) => return Some(vec!["mArjAni".into()]),
                (3,2) => return Some(vec!["mArjAva".into()]),
                (3,3) => return Some(vec!["mArjAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["mArkzyati".into()]),
                (1,2) => return Some(vec!["mArkzyataH".into()]),
                (1,3) => return Some(vec!["mArkzyanti".into()]),
                (2,1) => return Some(vec!["mArkzyasi".into()]),
                (2,2) => return Some(vec!["mArkzyaTaH".into()]),
                (2,3) => return Some(vec!["mArkzyaTa".into()]),
                (3,1) => return Some(vec!["mArkzyAmi".into()]),
                (3,2) => return Some(vec!["mArkzyAvaH".into()]),
                (3,3) => return Some(vec!["mArkzyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["mfjyAt".into()]),
                (1,2) => return Some(vec!["mfjyAtAm".into()]),
                (1,3) => return Some(vec!["mfjyuH".into()]),
                (2,1) => return Some(vec!["mfjyAH".into()]),
                (2,2) => return Some(vec!["mfjyAtam".into()]),
                (2,3) => return Some(vec!["mfjyAta".into()]),
                (3,1) => return Some(vec!["mfjyAm".into()]),
                (3,2) => return Some(vec!["mfjyAva".into()]),
                (3,3) => return Some(vec!["mfjyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "rudir" || dhatu_query == "02.0062" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["roditi".into()]),
                (1,2) => return Some(vec!["ruditaH".into()]),
                (1,3) => return Some(vec!["rudanti".into()]),
                (2,1) => return Some(vec!["rodizi".into()]),
                (2,2) => return Some(vec!["rudiTaH".into()]),
                (2,3) => return Some(vec!["rudiTa".into()]),
                (3,1) => return Some(vec!["rodimi".into()]),
                (3,2) => return Some(vec!["rudivaH".into()]),
                (3,3) => return Some(vec!["rudimaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["arodat".into()]),
                (1,2) => return Some(vec!["aruditAm".into()]),
                (1,3) => return Some(vec!["arudan".into()]),
                (2,1) => return Some(vec!["arodaH".into()]),
                (2,2) => return Some(vec!["aruditam".into()]),
                (2,3) => return Some(vec!["arudita".into()]),
                (3,1) => return Some(vec!["arodam".into()]),
                (3,2) => return Some(vec!["arudiva".into()]),
                (3,3) => return Some(vec!["arudima".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ruditAt".into()]),
                (1,2) => return Some(vec!["ruditAm".into()]),
                (1,3) => return Some(vec!["rudantu".into()]),
                (2,1) => return Some(vec!["ruditAt".into()]),
                (2,2) => return Some(vec!["ruditam".into()]),
                (2,3) => return Some(vec!["rudita".into()]),
                (3,1) => return Some(vec!["rodAni".into()]),
                (3,2) => return Some(vec!["rodAva".into()]),
                (3,3) => return Some(vec!["rodAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["rodizyati".into()]),
                (1,2) => return Some(vec!["rodizyataH".into()]),
                (1,3) => return Some(vec!["rodizyanti".into()]),
                (2,1) => return Some(vec!["rodizyasi".into()]),
                (2,2) => return Some(vec!["rodizyaTaH".into()]),
                (2,3) => return Some(vec!["rodizyaTa".into()]),
                (3,1) => return Some(vec!["rodizyAmi".into()]),
                (3,2) => return Some(vec!["rodizyAvaH".into()]),
                (3,3) => return Some(vec!["rodizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["rudyAt".into()]),
                (1,2) => return Some(vec!["rudyAtAm".into()]),
                (1,3) => return Some(vec!["rudyuH".into()]),
                (2,1) => return Some(vec!["rudyAH".into()]),
                (2,2) => return Some(vec!["rudyAtam".into()]),
                (2,3) => return Some(vec!["rudyAta".into()]),
                (3,1) => return Some(vec!["rudyAm".into()]),
                (3,2) => return Some(vec!["rudyAva".into()]),
                (3,3) => return Some(vec!["rudyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "Yizvapa" || dhatu_query == "02.0063" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["svapiti".into()]),
                (1,2) => return Some(vec!["svapitaH".into()]),
                (1,3) => return Some(vec!["svapanti".into()]),
                (2,1) => return Some(vec!["svapizi".into()]),
                (2,2) => return Some(vec!["svapiTaH".into()]),
                (2,3) => return Some(vec!["svapiTa".into()]),
                (3,1) => return Some(vec!["svapimi".into()]),
                (3,2) => return Some(vec!["svapivaH".into()]),
                (3,3) => return Some(vec!["svapimaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["asvapat".into()]),
                (1,2) => return Some(vec!["asvapitAm".into()]),
                (1,3) => return Some(vec!["asvapan".into()]),
                (2,1) => return Some(vec!["asvapaH".into()]),
                (2,2) => return Some(vec!["asvapitam".into()]),
                (2,3) => return Some(vec!["asvapita".into()]),
                (3,1) => return Some(vec!["asvapam".into()]),
                (3,2) => return Some(vec!["asvapiva".into()]),
                (3,3) => return Some(vec!["asvapima".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["svapitAt".into()]),
                (1,2) => return Some(vec!["svapitAm".into()]),
                (1,3) => return Some(vec!["svapantu".into()]),
                (2,1) => return Some(vec!["svapitAt".into()]),
                (2,2) => return Some(vec!["svapitam".into()]),
                (2,3) => return Some(vec!["svapita".into()]),
                (3,1) => return Some(vec!["svapAni".into()]),
                (3,2) => return Some(vec!["svapAva".into()]),
                (3,3) => return Some(vec!["svapAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["svapsyati".into()]),
                (1,2) => return Some(vec!["svapsyataH".into()]),
                (1,3) => return Some(vec!["svapsyanti".into()]),
                (2,1) => return Some(vec!["svapsyasi".into()]),
                (2,2) => return Some(vec!["svapsyaTaH".into()]),
                (2,3) => return Some(vec!["svapsyaTa".into()]),
                (3,1) => return Some(vec!["svapsyAmi".into()]),
                (3,2) => return Some(vec!["svapsyAvaH".into()]),
                (3,3) => return Some(vec!["svapsyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["svapyAt".into()]),
                (1,2) => return Some(vec!["svapyAtAm".into()]),
                (1,3) => return Some(vec!["svapyuH".into()]),
                (2,1) => return Some(vec!["svapyAH".into()]),
                (2,2) => return Some(vec!["svapyAtam".into()]),
                (2,3) => return Some(vec!["svapyAta".into()]),
                (3,1) => return Some(vec!["svapyAm".into()]),
                (3,2) => return Some(vec!["svapyAva".into()]),
                (3,3) => return Some(vec!["svapyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "Svasa" || dhatu_query == "02.0064" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["Svasiti".into()]),
                (1,2) => return Some(vec!["SvasitaH".into()]),
                (1,3) => return Some(vec!["Svasanti".into()]),
                (2,1) => return Some(vec!["Svasizi".into()]),
                (2,2) => return Some(vec!["SvasiTaH".into()]),
                (2,3) => return Some(vec!["SvasiTa".into()]),
                (3,1) => return Some(vec!["Svasimi".into()]),
                (3,2) => return Some(vec!["SvasivaH".into()]),
                (3,3) => return Some(vec!["SvasimaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aSvasat".into()]),
                (1,2) => return Some(vec!["aSvasitAm".into()]),
                (1,3) => return Some(vec!["aSvasan".into()]),
                (2,1) => return Some(vec!["aSvasaH".into()]),
                (2,2) => return Some(vec!["aSvasitam".into()]),
                (2,3) => return Some(vec!["aSvasita".into()]),
                (3,1) => return Some(vec!["aSvasam".into()]),
                (3,2) => return Some(vec!["aSvasiva".into()]),
                (3,3) => return Some(vec!["aSvasima".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["SvasitAt".into()]),
                (1,2) => return Some(vec!["SvasitAm".into()]),
                (1,3) => return Some(vec!["Svasantu".into()]),
                (2,1) => return Some(vec!["SvasitAt".into()]),
                (2,2) => return Some(vec!["Svasitam".into()]),
                (2,3) => return Some(vec!["Svasita".into()]),
                (3,1) => return Some(vec!["SvasAni".into()]),
                (3,2) => return Some(vec!["SvasAva".into()]),
                (3,3) => return Some(vec!["SvasAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["Svasizyati".into()]),
                (1,2) => return Some(vec!["SvasizyataH".into()]),
                (1,3) => return Some(vec!["Svasizyanti".into()]),
                (2,1) => return Some(vec!["Svasizyasi".into()]),
                (2,2) => return Some(vec!["SvasizyaTaH".into()]),
                (2,3) => return Some(vec!["SvasizyaTa".into()]),
                (3,1) => return Some(vec!["SvasizyAmi".into()]),
                (3,2) => return Some(vec!["SvasizyAvaH".into()]),
                (3,3) => return Some(vec!["SvasizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["SvasyAt".into()]),
                (1,2) => return Some(vec!["SvasyAtAm".into()]),
                (1,3) => return Some(vec!["SvasyuH".into()]),
                (2,1) => return Some(vec!["SvasyAH".into()]),
                (2,2) => return Some(vec!["SvasyAtam".into()]),
                (2,3) => return Some(vec!["SvasyAta".into()]),
                (3,1) => return Some(vec!["SvasyAm".into()]),
                (3,2) => return Some(vec!["SvasyAva".into()]),
                (3,3) => return Some(vec!["SvasyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "ana" || dhatu_query == "02.0065" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aniti".into()]),
                (1,2) => return Some(vec!["anitaH".into()]),
                (1,3) => return Some(vec!["ananti".into()]),
                (2,1) => return Some(vec!["anizi".into()]),
                (2,2) => return Some(vec!["aniTaH".into()]),
                (2,3) => return Some(vec!["aniTa".into()]),
                (3,1) => return Some(vec!["animi".into()]),
                (3,2) => return Some(vec!["anivaH".into()]),
                (3,3) => return Some(vec!["animaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["Anat".into()]),
                (1,2) => return Some(vec!["AnitAm".into()]),
                (1,3) => return Some(vec!["Anan".into()]),
                (2,1) => return Some(vec!["AnaH".into()]),
                (2,2) => return Some(vec!["Anitam".into()]),
                (2,3) => return Some(vec!["Anita".into()]),
                (3,1) => return Some(vec!["Anam".into()]),
                (3,2) => return Some(vec!["Aniva".into()]),
                (3,3) => return Some(vec!["Anima".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["anitAt".into()]),
                (1,2) => return Some(vec!["anitAm".into()]),
                (1,3) => return Some(vec!["anantu".into()]),
                (2,1) => return Some(vec!["anitAt".into()]),
                (2,2) => return Some(vec!["anitam".into()]),
                (2,3) => return Some(vec!["anita".into()]),
                (3,1) => return Some(vec!["anAni".into()]),
                (3,2) => return Some(vec!["anAva".into()]),
                (3,3) => return Some(vec!["anAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["anizyati".into()]),
                (1,2) => return Some(vec!["anizyataH".into()]),
                (1,3) => return Some(vec!["anizyanti".into()]),
                (2,1) => return Some(vec!["anizyasi".into()]),
                (2,2) => return Some(vec!["anizyaTaH".into()]),
                (2,3) => return Some(vec!["anizyaTa".into()]),
                (3,1) => return Some(vec!["anizyAmi".into()]),
                (3,2) => return Some(vec!["anizyAvaH".into()]),
                (3,3) => return Some(vec!["anizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["anyAt".into()]),
                (1,2) => return Some(vec!["anyAtAm".into()]),
                (1,3) => return Some(vec!["anyuH".into()]),
                (2,1) => return Some(vec!["anyAH".into()]),
                (2,2) => return Some(vec!["anyAtam".into()]),
                (2,3) => return Some(vec!["anyAta".into()]),
                (3,1) => return Some(vec!["anyAm".into()]),
                (3,2) => return Some(vec!["anyAva".into()]),
                (3,3) => return Some(vec!["anyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "jakza" || dhatu_query == "02.0066" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["jakziti".into()]),
                (1,2) => return Some(vec!["jakzitaH".into()]),
                (1,3) => return Some(vec!["jakzati".into()]),
                (2,1) => return Some(vec!["jakzizi".into()]),
                (2,2) => return Some(vec!["jakziTaH".into()]),
                (2,3) => return Some(vec!["jakziTa".into()]),
                (3,1) => return Some(vec!["jakzimi".into()]),
                (3,2) => return Some(vec!["jakzivaH".into()]),
                (3,3) => return Some(vec!["jakzimaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ajakzat".into()]),
                (1,2) => return Some(vec!["ajakzitAm".into()]),
                (1,3) => return Some(vec!["ajakzuH".into()]),
                (2,1) => return Some(vec!["ajakzaH".into()]),
                (2,2) => return Some(vec!["ajakzitam".into()]),
                (2,3) => return Some(vec!["ajakzita".into()]),
                (3,1) => return Some(vec!["ajakzam".into()]),
                (3,2) => return Some(vec!["ajakziva".into()]),
                (3,3) => return Some(vec!["ajakzima".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["jakzitAt".into()]),
                (1,2) => return Some(vec!["jakzitAm".into()]),
                (1,3) => return Some(vec!["jakzatu".into()]),
                (2,1) => return Some(vec!["jakzitAt".into()]),
                (2,2) => return Some(vec!["jakzitam".into()]),
                (2,3) => return Some(vec!["jakzita".into()]),
                (3,1) => return Some(vec!["jakzARi".into()]),
                (3,2) => return Some(vec!["jakzAva".into()]),
                (3,3) => return Some(vec!["jakzAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["jakzizyati".into()]),
                (1,2) => return Some(vec!["jakzizyataH".into()]),
                (1,3) => return Some(vec!["jakzizyanti".into()]),
                (2,1) => return Some(vec!["jakzizyasi".into()]),
                (2,2) => return Some(vec!["jakzizyaTaH".into()]),
                (2,3) => return Some(vec!["jakzizyaTa".into()]),
                (3,1) => return Some(vec!["jakzizyAmi".into()]),
                (3,2) => return Some(vec!["jakzizyAvaH".into()]),
                (3,3) => return Some(vec!["jakzizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["jakzyAt".into()]),
                (1,2) => return Some(vec!["jakzyAtAm".into()]),
                (1,3) => return Some(vec!["jakzyuH".into()]),
                (2,1) => return Some(vec!["jakzyAH".into()]),
                (2,2) => return Some(vec!["jakzyAtam".into()]),
                (2,3) => return Some(vec!["jakzyAta".into()]),
                (3,1) => return Some(vec!["jakzyAm".into()]),
                (3,2) => return Some(vec!["jakzyAva".into()]),
                (3,3) => return Some(vec!["jakzyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "jAgf" || dhatu_query == "02.0067" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["jAgarti".into()]),
                (1,2) => return Some(vec!["jAgftaH".into()]),
                (1,3) => return Some(vec!["jAgrati".into()]),
                (2,1) => return Some(vec!["jAgarzi".into()]),
                (2,2) => return Some(vec!["jAgfTaH".into()]),
                (2,3) => return Some(vec!["jAgfTa".into()]),
                (3,1) => return Some(vec!["jAgarmi".into()]),
                (3,2) => return Some(vec!["jAgfvaH".into()]),
                (3,3) => return Some(vec!["jAgfmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["ajAgaH".into()]),
                (1,2) => return Some(vec!["ajAgftAm".into()]),
                (1,3) => return Some(vec!["ajAgaruH".into()]),
                (2,1) => return Some(vec!["ajAgaH".into()]),
                (2,2) => return Some(vec!["ajAgftam".into()]),
                (2,3) => return Some(vec!["ajAgfta".into()]),
                (3,1) => return Some(vec!["ajAgaram".into()]),
                (3,2) => return Some(vec!["ajAgfva".into()]),
                (3,3) => return Some(vec!["ajAgfma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["jAgartu".into()]),
                (1,2) => return Some(vec!["jAgftAm".into()]),
                (1,3) => return Some(vec!["jAgratu".into()]),
                (2,1) => return Some(vec!["jAgftAt".into()]),
                (2,2) => return Some(vec!["jAgftam".into()]),
                (2,3) => return Some(vec!["jAgfta".into()]),
                (3,1) => return Some(vec!["jAgarARi".into()]),
                (3,2) => return Some(vec!["jAgarAva".into()]),
                (3,3) => return Some(vec!["jAgarAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["jAgarizyati".into()]),
                (1,2) => return Some(vec!["jAgarizyataH".into()]),
                (1,3) => return Some(vec!["jAgarizyanti".into()]),
                (2,1) => return Some(vec!["jAgarizyasi".into()]),
                (2,2) => return Some(vec!["jAgarizyaTaH".into()]),
                (2,3) => return Some(vec!["jAgarizyaTa".into()]),
                (3,1) => return Some(vec!["jAgarizyAmi".into()]),
                (3,2) => return Some(vec!["jAgarizyAvaH".into()]),
                (3,3) => return Some(vec!["jAgarizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["jAgfyAt".into()]),
                (1,2) => return Some(vec!["jAgfyAtAm".into()]),
                (1,3) => return Some(vec!["jAgfyuH".into()]),
                (2,1) => return Some(vec!["jAgfyAH".into()]),
                (2,2) => return Some(vec!["jAgfyAtam".into()]),
                (2,3) => return Some(vec!["jAgfyAta".into()]),
                (3,1) => return Some(vec!["jAgfyAm".into()]),
                (3,2) => return Some(vec!["jAgfyAva".into()]),
                (3,3) => return Some(vec!["jAgfyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "daridrA" || dhatu_query == "02.0068" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["daridrAti".into()]),
                (1,2) => return Some(vec!["daridritaH".into()]),
                (1,3) => return Some(vec!["daridrati".into()]),
                (2,1) => return Some(vec!["daridrAsi".into()]),
                (2,2) => return Some(vec!["daridriTaH".into()]),
                (2,3) => return Some(vec!["daridriTa".into()]),
                (3,1) => return Some(vec!["daridrAmi".into()]),
                (3,2) => return Some(vec!["daridrivaH".into()]),
                (3,3) => return Some(vec!["daridrimaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["adaridrAt".into()]),
                (1,2) => return Some(vec!["adaridritAm".into()]),
                (1,3) => return Some(vec!["adaridruH".into()]),
                (2,1) => return Some(vec!["adaridrAH".into()]),
                (2,2) => return Some(vec!["adaridritam".into()]),
                (2,3) => return Some(vec!["adaridrita".into()]),
                (3,1) => return Some(vec!["adaridrAm".into()]),
                (3,2) => return Some(vec!["adaridriva".into()]),
                (3,3) => return Some(vec!["adaridrima".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["daridrAtu".into()]),
                (1,2) => return Some(vec!["daridritAm".into()]),
                (1,3) => return Some(vec!["daridratu".into()]),
                (2,1) => return Some(vec!["daridritAt".into()]),
                (2,2) => return Some(vec!["daridritam".into()]),
                (2,3) => return Some(vec!["daridrita".into()]),
                (3,1) => return Some(vec!["daridrARi".into()]),
                (3,2) => return Some(vec!["daridrAva".into()]),
                (3,3) => return Some(vec!["daridrAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["daridrizyati".into()]),
                (1,2) => return Some(vec!["daridrizyataH".into()]),
                (1,3) => return Some(vec!["daridrizyanti".into()]),
                (2,1) => return Some(vec!["daridrizyasi".into()]),
                (2,2) => return Some(vec!["daridrizyaTaH".into()]),
                (2,3) => return Some(vec!["daridrizyaTa".into()]),
                (3,1) => return Some(vec!["daridrizyAmi".into()]),
                (3,2) => return Some(vec!["daridrizyAvaH".into()]),
                (3,3) => return Some(vec!["daridrizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["daridriyAt".into()]),
                (1,2) => return Some(vec!["daridriyAtAm".into()]),
                (1,3) => return Some(vec!["daridriyuH".into()]),
                (2,1) => return Some(vec!["daridriyAH".into()]),
                (2,2) => return Some(vec!["daridriyAtam".into()]),
                (2,3) => return Some(vec!["daridriyAta".into()]),
                (3,1) => return Some(vec!["daridriyAm".into()]),
                (3,2) => return Some(vec!["daridriyAva".into()]),
                (3,3) => return Some(vec!["daridriyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "cakAsf" || dhatu_query == "02.0069" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["cakAsti".into()]),
                (1,2) => return Some(vec!["cakAstaH".into()]),
                (1,3) => return Some(vec!["cakAsati".into()]),
                (2,1) => return Some(vec!["cakAssi".into()]),
                (2,2) => return Some(vec!["cakAsTaH".into()]),
                (2,3) => return Some(vec!["cakAsTa".into()]),
                (3,1) => return Some(vec!["cakAsmi".into()]),
                (3,2) => return Some(vec!["cakAsvaH".into()]),
                (3,3) => return Some(vec!["cakAsmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["acakAt".into()]),
                (1,2) => return Some(vec!["acakAstAm".into()]),
                (1,3) => return Some(vec!["acakAsuH".into()]),
                (2,1) => return Some(vec!["acakAH".into()]),
                (2,2) => return Some(vec!["acakAstam".into()]),
                (2,3) => return Some(vec!["acakAsta".into()]),
                (3,1) => return Some(vec!["acakAsam".into()]),
                (3,2) => return Some(vec!["acakAsva".into()]),
                (3,3) => return Some(vec!["acakAsma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["cakAstAt".into()]),
                (1,2) => return Some(vec!["cakAstAm".into()]),
                (1,3) => return Some(vec!["cakAsatu".into()]),
                (2,1) => return Some(vec!["cakADi".into()]),
                (2,2) => return Some(vec!["cakAstam".into()]),
                (2,3) => return Some(vec!["cakAsta".into()]),
                (3,1) => return Some(vec!["cakAsAni".into()]),
                (3,2) => return Some(vec!["cakAsAva".into()]),
                (3,3) => return Some(vec!["cakAsAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["cakAsizyati".into()]),
                (1,2) => return Some(vec!["cakAsizyataH".into()]),
                (1,3) => return Some(vec!["cakAsizyanti".into()]),
                (2,1) => return Some(vec!["cakAsizyasi".into()]),
                (2,2) => return Some(vec!["cakAsizyaTaH".into()]),
                (2,3) => return Some(vec!["cakAsizyaTa".into()]),
                (3,1) => return Some(vec!["cakAsizyAmi".into()]),
                (3,2) => return Some(vec!["cakAsizyAvaH".into()]),
                (3,3) => return Some(vec!["cakAsizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["cakAsyAt".into()]),
                (1,2) => return Some(vec!["cakAsyAtAm".into()]),
                (1,3) => return Some(vec!["cakAsyuH".into()]),
                (2,1) => return Some(vec!["cakAsyAH".into()]),
                (2,2) => return Some(vec!["cakAsyAtam".into()]),
                (2,3) => return Some(vec!["cakAsyAta".into()]),
                (3,1) => return Some(vec!["cakAsyAm".into()]),
                (3,2) => return Some(vec!["cakAsyAva".into()]),
                (3,3) => return Some(vec!["cakAsyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "SAsu" || dhatu_query == "02.0070" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["SAsti".into()]),
                (1,2) => return Some(vec!["SizwaH".into()]),
                (1,3) => return Some(vec!["SAsati".into()]),
                (2,1) => return Some(vec!["SAssi".into()]),
                (2,2) => return Some(vec!["SizWaH".into()]),
                (2,3) => return Some(vec!["SizWa".into()]),
                (3,1) => return Some(vec!["SAsmi".into()]),
                (3,2) => return Some(vec!["SizvaH".into()]),
                (3,3) => return Some(vec!["SizmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["aSAt".into()]),
                (1,2) => return Some(vec!["aSizwAm".into()]),
                (1,3) => return Some(vec!["aSAsuH".into()]),
                (2,1) => return Some(vec!["aSAH".into()]),
                (2,2) => return Some(vec!["aSizwam".into()]),
                (2,3) => return Some(vec!["aSizwa".into()]),
                (3,1) => return Some(vec!["aSAsam".into()]),
                (3,2) => return Some(vec!["aSizva".into()]),
                (3,3) => return Some(vec!["aSizma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["SAstu".into()]),
                (1,2) => return Some(vec!["SizwAm".into()]),
                (1,3) => return Some(vec!["SAsatu".into()]),
                (2,1) => return Some(vec!["SADi".into()]),
                (2,2) => return Some(vec!["Sizwam".into()]),
                (2,3) => return Some(vec!["Sizwa".into()]),
                (3,1) => return Some(vec!["SAsAni".into()]),
                (3,2) => return Some(vec!["SAsAva".into()]),
                (3,3) => return Some(vec!["SAsAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["SAsizyati".into()]),
                (1,2) => return Some(vec!["SAsizyataH".into()]),
                (1,3) => return Some(vec!["SAsizyanti".into()]),
                (2,1) => return Some(vec!["SAsizyasi".into()]),
                (2,2) => return Some(vec!["SAsizyaTaH".into()]),
                (2,3) => return Some(vec!["SAsizyaTa".into()]),
                (3,1) => return Some(vec!["SAsizyAmi".into()]),
                (3,2) => return Some(vec!["SAsizyAvaH".into()]),
                (3,3) => return Some(vec!["SAsizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["SizyAt".into()]),
                (1,2) => return Some(vec!["SizyAtAm".into()]),
                (1,3) => return Some(vec!["SizyuH".into()]),
                (2,1) => return Some(vec!["SizyAH".into()]),
                (2,2) => return Some(vec!["SizyAtam".into()]),
                (2,3) => return Some(vec!["SizyAta".into()]),
                (3,1) => return Some(vec!["SizyAm".into()]),
                (3,2) => return Some(vec!["SizyAva".into()]),
                (3,3) => return Some(vec!["SizyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "zasa" || dhatu_query == "02.0073" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sasti".into()]),
                (1,2) => return Some(vec!["sastaH".into()]),
                (1,3) => return Some(vec!["sasanti".into()]),
                (2,1) => return Some(vec!["sassi".into()]),
                (2,2) => return Some(vec!["sasTaH".into()]),
                (2,3) => return Some(vec!["sasTa".into()]),
                (3,1) => return Some(vec!["sasmi".into()]),
                (3,2) => return Some(vec!["sasvaH".into()]),
                (3,3) => return Some(vec!["sasmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["asat".into()]),
                (1,2) => return Some(vec!["asastAm".into()]),
                (1,3) => return Some(vec!["asasan".into()]),
                (2,1) => return Some(vec!["asaH".into()]),
                (2,2) => return Some(vec!["asastam".into()]),
                (2,3) => return Some(vec!["asasta".into()]),
                (3,1) => return Some(vec!["asasam".into()]),
                (3,2) => return Some(vec!["asasva".into()]),
                (3,3) => return Some(vec!["asasma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sastAt".into()]),
                (1,2) => return Some(vec!["sastAm".into()]),
                (1,3) => return Some(vec!["sasantu".into()]),
                (2,1) => return Some(vec!["saDi".into()]),
                (2,2) => return Some(vec!["sastam".into()]),
                (2,3) => return Some(vec!["sasta".into()]),
                (3,1) => return Some(vec!["sasAni".into()]),
                (3,2) => return Some(vec!["sasAva".into()]),
                (3,3) => return Some(vec!["sasAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sasizyati".into()]),
                (1,2) => return Some(vec!["sasizyataH".into()]),
                (1,3) => return Some(vec!["sasizyanti".into()]),
                (2,1) => return Some(vec!["sasizyasi".into()]),
                (2,2) => return Some(vec!["sasizyaTaH".into()]),
                (2,3) => return Some(vec!["sasizyaTa".into()]),
                (3,1) => return Some(vec!["sasizyAmi".into()]),
                (3,2) => return Some(vec!["sasizyAvaH".into()]),
                (3,3) => return Some(vec!["sasizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["sasyAt".into()]),
                (1,2) => return Some(vec!["sasyAtAm".into()]),
                (1,3) => return Some(vec!["sasyuH".into()]),
                (2,1) => return Some(vec!["sasyAH".into()]),
                (2,2) => return Some(vec!["sasyAtam".into()]),
                (2,3) => return Some(vec!["sasyAta".into()]),
                (3,1) => return Some(vec!["sasyAm".into()]),
                (3,2) => return Some(vec!["sasyAva".into()]),
                (3,3) => return Some(vec!["sasyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "zasti" || dhatu_query == "02.0074" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["saMsti".into()]),
                (1,2) => return Some(vec!["saMstaH".into()]),
                (1,3) => return Some(vec!["saMstanti".into()]),
                (2,1) => return Some(vec!["saMstsi".into()]),
                (2,2) => return Some(vec!["saMstTaH".into()]),
                (2,3) => return Some(vec!["saMstTa".into()]),
                (3,1) => return Some(vec!["saMstmi".into()]),
                (3,2) => return Some(vec!["saMstvaH".into()]),
                (3,3) => return Some(vec!["saMstmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["asan".into()]),
                (1,2) => return Some(vec!["asaMstAm".into()]),
                (1,3) => return Some(vec!["asaMstan".into()]),
                (2,1) => return Some(vec!["asan".into()]),
                (2,2) => return Some(vec!["asaMstam".into()]),
                (2,3) => return Some(vec!["asaMsta".into()]),
                (3,1) => return Some(vec!["asaMstam".into()]),
                (3,2) => return Some(vec!["asaMstva".into()]),
                (3,3) => return Some(vec!["asaMstma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["saMstAt".into()]),
                (1,2) => return Some(vec!["saMstAm".into()]),
                (1,3) => return Some(vec!["saMstantu".into()]),
                (2,1) => return Some(vec!["saMstAt".into()]),
                (2,2) => return Some(vec!["saMstam".into()]),
                (2,3) => return Some(vec!["saMsta".into()]),
                (3,1) => return Some(vec!["saMstAni".into()]),
                (3,2) => return Some(vec!["saMstAva".into()]),
                (3,3) => return Some(vec!["saMstAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["saMstizyati".into()]),
                (1,2) => return Some(vec!["saMstizyataH".into()]),
                (1,3) => return Some(vec!["saMstizyanti".into()]),
                (2,1) => return Some(vec!["saMstizyasi".into()]),
                (2,2) => return Some(vec!["saMstizyaTaH".into()]),
                (2,3) => return Some(vec!["saMstizyaTa".into()]),
                (3,1) => return Some(vec!["saMstizyAmi".into()]),
                (3,2) => return Some(vec!["saMstizyAvaH".into()]),
                (3,3) => return Some(vec!["saMstizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["saMstyAt".into()]),
                (1,2) => return Some(vec!["saMstyAtAm".into()]),
                (1,3) => return Some(vec!["saMstyuH".into()]),
                (2,1) => return Some(vec!["saMstyAH".into()]),
                (2,2) => return Some(vec!["saMstyAtam".into()]),
                (2,3) => return Some(vec!["saMstyAta".into()]),
                (3,1) => return Some(vec!["saMstyAm".into()]),
                (3,2) => return Some(vec!["saMstyAva".into()]),
                (3,3) => return Some(vec!["saMstyAma".into()]),
                _ => {}
            }
        }
    }
    if dhatu_query == "vaSa" || dhatu_query == "02.0075" {
        if canonical == "plat" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vazwi".into()]),
                (1,2) => return Some(vec!["uzwaH".into()]),
                (1,3) => return Some(vec!["uSanti".into()]),
                (2,1) => return Some(vec!["vakzi".into()]),
                (2,2) => return Some(vec!["uzWaH".into()]),
                (2,3) => return Some(vec!["uzWa".into()]),
                (3,1) => return Some(vec!["vaSmi".into()]),
                (3,2) => return Some(vec!["uSvaH".into()]),
                (3,3) => return Some(vec!["uSmaH".into()]),
                _ => {}
            }
        }
        if canonical == "plan" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["avaw".into()]),
                (1,2) => return Some(vec!["OzwAm".into()]),
                (1,3) => return Some(vec!["OSan".into()]),
                (2,1) => return Some(vec!["avaw".into()]),
                (2,2) => return Some(vec!["Ozwam".into()]),
                (2,3) => return Some(vec!["Ozwa".into()]),
                (3,1) => return Some(vec!["avaSam".into()]),
                (3,2) => return Some(vec!["OSva".into()]),
                (3,3) => return Some(vec!["OSma".into()]),
                _ => {}
            }
        }
        if canonical == "plot" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["uzwAt".into()]),
                (1,2) => return Some(vec!["uzwAm".into()]),
                (1,3) => return Some(vec!["uSantu".into()]),
                (2,1) => return Some(vec!["uqQi".into()]),
                (2,2) => return Some(vec!["uzwam".into()]),
                (2,3) => return Some(vec!["uzwa".into()]),
                (3,1) => return Some(vec!["vaSAni".into()]),
                (3,2) => return Some(vec!["vaSAva".into()]),
                (3,3) => return Some(vec!["vaSAma".into()]),
                _ => {}
            }
        }
        if canonical == "plrt" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["vaSizyati".into()]),
                (1,2) => return Some(vec!["vaSizyataH".into()]),
                (1,3) => return Some(vec!["vaSizyanti".into()]),
                (2,1) => return Some(vec!["vaSizyasi".into()]),
                (2,2) => return Some(vec!["vaSizyaTaH".into()]),
                (2,3) => return Some(vec!["vaSizyaTa".into()]),
                (3,1) => return Some(vec!["vaSizyAmi".into()]),
                (3,2) => return Some(vec!["vaSizyAvaH".into()]),
                (3,3) => return Some(vec!["vaSizyAmaH".into()]),
                _ => {}
            }
        }
        if canonical == "pvidhilin" {
            match (purusha, vacana) {
                (1,1) => return Some(vec!["uSyAt".into()]),
                (1,2) => return Some(vec!["uSyAtAm".into()]),
                (1,3) => return Some(vec!["uSyuH".into()]),
                (2,1) => return Some(vec!["uSyAH".into()]),
                (2,2) => return Some(vec!["uSyAtam".into()]),
                (2,3) => return Some(vec!["uSyAta".into()]),
                (3,1) => return Some(vec!["uSyAm".into()]),
                (3,2) => return Some(vec!["uSyAva".into()]),
                (3,3) => return Some(vec!["uSyAma".into()]),
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

fn kfvi_forms(canonical: &str, purusha: u8, vacana: u8) -> Option<Vec<String>> {
    match (canonical, purusha, vacana) {
        ("plat",1,1) => Some(vec!["kfRoti".into()]),
        ("plat",1,2) => Some(vec!["kfRutaH".into()]),
        ("plat",1,3) => Some(vec!["kfRvanti".into()]),
        ("plat",2,1) => Some(vec!["kfRozi".into()]),
        ("plat",2,2) => Some(vec!["kfRuTaH".into()]),
        ("plat",2,3) => Some(vec!["kfRuTa".into()]),
        ("plat",3,1) => Some(vec!["kfRomi".into()]),
        ("plat",3,2) => Some(vec!["kfRuvaH".into(), "kfRvaH".into()]),
        ("plat",3,3) => Some(vec!["kfRumaH".into(), "kfRmaH".into()]),
        ("plan",1,1) => Some(vec!["akfRot".into(), "akfRod".into()]),
        ("plan",1,2) => Some(vec!["akfRutAm".into()]),
        ("plan",1,3) => Some(vec!["akfRvan".into()]),
        ("plan",2,1) => Some(vec!["akfRoH".into()]),
        ("plan",2,2) => Some(vec!["akfRutam".into()]),
        ("plan",2,3) => Some(vec!["akfRuta".into()]),
        ("plan",3,1) => Some(vec!["akfRavam".into()]),
        ("plan",3,2) => Some(vec!["akfRuva".into(), "akfRva".into()]),
        ("plan",3,3) => Some(vec!["akfRuma".into(), "akfRma".into()]),
        ("plot",1,1) => Some(vec!["kfRutAt".into(), "kfRutAd".into(), "kfRotu".into()]),
        ("plot",1,2) => Some(vec!["kfRutAm".into()]),
        ("plot",1,3) => Some(vec!["kfRvantu".into()]),
        ("plot",2,1) => Some(vec!["kfRu".into(), "kfRutAt".into(), "kfRutAd".into()]),
        ("plot",2,2) => Some(vec!["kfRutam".into()]),
        ("plot",2,3) => Some(vec!["kfRuta".into()]),
        ("plot",3,1) => Some(vec!["kfRavAni".into()]),
        ("plot",3,2) => Some(vec!["kfRavAva".into()]),
        ("plot",3,3) => Some(vec!["kfRavAma".into()]),
        ("plrt",1,1) => Some(vec!["kfRvizyati".into()]),
        ("plrt",1,2) => Some(vec!["kfRvizyataH".into()]),
        ("plrt",1,3) => Some(vec!["kfRvizyanti".into()]),
        ("plrt",2,1) => Some(vec!["kfRvizyasi".into()]),
        ("plrt",2,2) => Some(vec!["kfRvizyaTaH".into()]),
        ("plrt",2,3) => Some(vec!["kfRvizyaTa".into()]),
        ("plrt",3,1) => Some(vec!["kfRvizyAmi".into()]),
        ("plrt",3,2) => Some(vec!["kfRvizyAvaH".into()]),
        ("plrt",3,3) => Some(vec!["kfRvizyAmaH".into()]),
        ("pvidhilin",1,1) => Some(vec!["kfRuyAt".into(), "kfRuyAd".into()]),
        ("pvidhiling",1,1) => Some(vec!["kfRuyAt".into(), "kfRuyAd".into()]),
        ("pvidhilin",1,2) => Some(vec!["kfRuyAtAm".into()]),
        ("pvidhiling",1,2) => Some(vec!["kfRuyAtAm".into()]),
        ("pvidhilin",1,3) => Some(vec!["kfRuyuH".into()]),
        ("pvidhiling",1,3) => Some(vec!["kfRuyuH".into()]),
        ("pvidhilin",2,1) => Some(vec!["kfRuyAH".into()]),
        ("pvidhiling",2,1) => Some(vec!["kfRuyAH".into()]),
        ("pvidhilin",2,2) => Some(vec!["kfRuyAtam".into()]),
        ("pvidhiling",2,2) => Some(vec!["kfRuyAtam".into()]),
        ("pvidhilin",2,3) => Some(vec!["kfRuyAta".into()]),
        ("pvidhiling",2,3) => Some(vec!["kfRuyAta".into()]),
        ("pvidhilin",3,1) => Some(vec!["kfRuyAm".into()]),
        ("pvidhiling",3,1) => Some(vec!["kfRuyAm".into()]),
        ("pvidhilin",3,2) => Some(vec!["kfRuyAva".into()]),
        ("pvidhiling",3,2) => Some(vec!["kfRuyAva".into()]),
        ("pvidhilin",3,3) => Some(vec!["kfRuyAma".into()]),
        ("pvidhiling",3,3) => Some(vec!["kfRuyAma".into()]),
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
