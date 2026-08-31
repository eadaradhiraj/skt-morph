//! Temporary per-dhātu patches where the Kaumudī prakriyā is still incomplete.
//! Prefer `live_generate`; delete a block when the sūtra is in stems/join.
//! Do not add scrape-only forms here.
#![allow(non_snake_case, unused)]

pub fn lookup_override(
    dhatu_query: &str,
    canonical: &str,
    _purusha: u8,
    _vacana: u8,
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
                | "Divi" | "01.0677" | "Rikza" | "01.0747" | "fti" | "01.1166"
        )
        && matches!(
            canonical,
            "plat" | "plan" | "plot" | "pvidhilin" | "plrt" | "plun" | "pashirling"
                | "alat" | "alan" | "alot" | "avidhilin" | "alrt"
        )
    {
        return None;
    }
    None
}
