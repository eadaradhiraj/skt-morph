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
