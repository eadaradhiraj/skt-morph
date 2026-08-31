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
