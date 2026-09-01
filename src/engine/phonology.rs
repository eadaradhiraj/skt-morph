//! phonology — guṇa/vṛddhi (7.3.84 ff.), sandhi, ṇatva/ṣatva, śnu/siz etc.
//! Port of sktmorph/engine/phonology.py — SLP1 ASCII, char-safe.
//! All helpers are sūtra-gated; no DB fallback.
pub const VOWEL_FINAL: &str = "aeiouAIUEOfF";

/// 1.1.3 vowel check — SLP1 vowel set.
fn is_vowel_final(c: char) -> bool {
    VOWEL_FINAL.contains(c)
}

/// True if stem ends in vowel (a/A/i/I/u/U/f/F …) — for guṇa eligibility.
pub fn ends_with_vowel(stem: &str) -> bool {
    stem.chars().last().is_some_and(is_vowel_final)
}

/// 7.3.84 sārvadhātuka guṇa ( + 7.3.86 laghūpadha) — last vowel → guṇa.
/// e.g. ci→ce, hu→ho, kṛ→kar. Idempotent if already guṇita. Handles 7.3.84/86 in one pass.
pub fn apply_guna_to_stem(stem: &str) -> String {
    let chars: Vec<char> = stem.chars().collect();
    for idx in (0..chars.len()).rev() {
        let ch = chars[idx];
        let repl: Option<&str> = match ch {
            'i' => Some("e"),
            'I' => Some("e"),
            'u' => Some("o"),
            'U' => Some("av"),
            'f' => Some("ar"),
            'F' => Some("ar"),
            'A' => Some("A"),
            'a' => Some("a"),
            _ => None,
        };
        if let Some(r) = repl {
            let mut out = String::new();
            for &c in &chars[0..idx] { out.push(c); }
            out.push_str(r);
            for &c in &chars[idx + 1..] { out.push(c); }
            return out;
        }
    }
    stem.to_string()
}

/// Causative grade (ṇic 7.3.86 + 7.2.115 vṛddhi) — for ṇic/san stems.
pub fn apply_causative_grade(stem: &str) -> String {
    let chars: Vec<char> = stem.chars().collect();
    for idx in (0..chars.len()).rev() {
        let ch = chars[idx];
        if !is_vowel_final(ch) { continue; }
        let trailing: String = chars[idx+1..].iter().collect();
        if trailing.len() > 1 { return stem.to_string(); }
        if ch == 'a' || ch == 'A' {
            let mut out = String::new();
            for &c in &chars[0..idx] { out.push(c); }
            out.push('A');
            out.push_str(&trailing);
            return out;
        }
        if matches!(ch, 'I'|'U'|'F') {
            return stem.to_string();
        }
        // guna map for causative (same as above without IUUF already handled)
        let repl: Option<&str> = match ch {
            'i' => Some("e"),
            'u' => Some("o"),
            'U' => Some("av"),
            'f' => Some("ar"),
            'F' => Some("ar"),
            'a' => Some("a"),
            'A' => Some("A"),
            _ => None,
        };
        if let Some(r) = repl {
            let mut out = String::new();
            for &c in &chars[0..idx] { out.push(c); }
            out.push_str(r);
            for &c in &chars[idx+1..] { out.push(c); }
            return out;
        }
    }
    stem.to_string()
}

pub fn vowel_initial_lang_stem(dhatu: &str) -> Option<String> {
    let mut chars = dhatu.chars();
    let first = chars.next()?;
    let rest: String = chars.collect();
    match first {
        'a' | 'A' => Some(format!("A{}", rest)),
        'i' | 'I' => Some(format!("E{}", rest)),
        'u' | 'U' => Some(format!("O{}", rest)),
        'f' | 'F' => Some(format!("Ar{}", rest)),
        'e' | 'E' => Some(format!("E{}", rest)),
        'o' | 'O' => Some(format!("O{}", rest)),
        _ => None,
    }
}

pub fn bidadi_present_stem(dhatu: &str) -> String {
    if dhatu.ends_with('i') || dhatu.ends_with('I') || dhatu.ends_with('u') || dhatu.ends_with('U') || dhatu.ends_with('e') || dhatu.ends_with('E') {
        let mut s = dhatu.chars().collect::<Vec<_>>();
        s.pop();
        let base: String = s.into_iter().collect();
        format!("{}aya", base)
    } else {
        format!("{}ya", dhatu)
    }
}

pub fn bidadi_lang_stem(dhatu: &str) -> String {
    if dhatu.ends_with('i') || dhatu.ends_with('I') || dhatu.ends_with('u') || dhatu.ends_with('U') || dhatu.ends_with('e') || dhatu.ends_with('E') {
        let mut s = dhatu.chars().collect::<Vec<_>>();
        s.pop();
        let base: String = s.into_iter().collect();
        format!("{}ay", base)
    } else {
        format!("{}ay", dhatu)
    }
}

pub fn bidadi_vidhilin_stem(dhatu: &str) -> String {
    bidadi_lang_stem(dhatu)
}

pub fn is_bidadi(antarganas: &str) -> bool { antarganas.contains("BidAdi") }
pub fn is_yajadi(antarganas: &str) -> bool { antarganas.contains("yajAdi") }
pub fn is_gawadi(antarganas: &str) -> bool { antarganas.contains("GawAdi") }

pub fn g1_rv_nv_present_base(dhatu: &str) -> Option<String> {
    if dhatu.ends_with("nv") && dhatu.len() >= 4 && dhatu.starts_with('r') {
        Some(format!("{}Rv", &dhatu[..dhatu.len()-2]))
    } else { None }
}

const G1_NV_ROOTS: &[&str] = &["Dinv"];
pub fn g1_nv_present_stem(dhatu: &str) -> Option<String> {
    if G1_NV_ROOTS.contains(&dhatu) {
        Some(format!("{}no", &dhatu[..dhatu.len()-2]))
    } else { None }
}
pub fn g1_nv_vidhilin_stem(dhatu: &str) -> Option<String> {
    if G1_NV_ROOTS.contains(&dhatu) {
        Some(format!("{}nu", &dhatu[..dhatu.len()-2]))
    } else { None }
}

/// 7.3.77 इषुगमियमां छः — यम् only here (घटादि blocks छ); दाण् is 7.3.78 in `sad_present_base`.
pub fn yam_cc_present_stem(dhatu: &str, antarganas: &str) -> Option<String> {
    if dhatu=="yam" && !is_gawadi(antarganas) { Some("yacCa".to_string()) } else { None }
}
pub fn yam_cc_lang_stem(dhatu: &str, antarganas: &str) -> Option<String> {
    if dhatu=="yam" && !is_gawadi(antarganas) { Some("yacC".to_string()) } else { None }
}
pub fn yam_cc_future_stem(dhatu: &str, antarganas: &str) -> Option<String> {
    if dhatu=="yam" && !is_gawadi(antarganas) { Some("yaMsya".to_string()) } else { None }
}

/// 6.1.78 एचोऽयवायावः after गुण: श्रयति, नयति, धयति; वे/व्ये/ह्वे/श्वि.
const G1_AYA_PRESENT: &[&str] = &["ji","Sri","nI","De","jri","ve","vye","hve","Svi"];
const G1_A_FINAL: &[&str] = &["SrA","jYA"];
const BIDADI_THEMATIC: &[&str] = &["mid","med","meD","vap","vas","tF","guh"];

pub fn uses_aya_present(cgana: u8, dhatu: &str, antarganas: &str) -> bool {
    if BIDADI_THEMATIC.contains(&dhatu) { return false; }
    cgana==1 && (is_bidadi(antarganas) || G1_AYA_PRESENT.contains(&dhatu))
}
pub fn is_g1_a_final(dhatu: &str) -> bool { G1_A_FINAL.contains(&dhatu) }
pub fn is_g1_aya_present(dhatu: &str) -> bool { G1_AYA_PRESENT.contains(&dhatu) }
pub fn is_g1_nv_root(dhatu: &str) -> bool { G1_NV_ROOTS.contains(&dhatu) }

const G9_N_INFIX: &[&str] = &["Dras","Kav","SranT","aS","banD","granT","guD","jYA","kliS","knU","kunT","kzuB","mI","manT","mfd","naB","pU","si","skamB","sku","skumB","stamB","stumB","tuB","yu","lU","DU","jyA","lI","vlI","blI","plI"];
pub fn g9_uses_n_infix(dhatu: &str, _antarganas: &str) -> bool { G9_N_INFIX.contains(&dhatu) }
pub fn g9_uses_r_infix(dhatu: &str, antarganas: &str) -> bool { !g9_uses_n_infix(dhatu, antarganas) }

pub fn g9_n_lang_base(dhatu: &str) -> String {
    if dhatu.ends_with("mB") { return format!("{}B", &dhatu[..dhatu.len()-2]); }
    if dhatu=="pU" { return "pun".to_string(); }
    if dhatu.ends_with('U') && dhatu.len()==2 { let c=dhatu.chars().next().unwrap().to_ascii_lowercase(); return format!("{}un", c); }
    if dhatu=="SranT" { return "SraT".to_string(); }
    if dhatu.ends_with('I') && dhatu.len()==2 { let c=dhatu.chars().next().unwrap().to_ascii_lowercase(); return format!("{}i", c); }
    if dhatu=="jyA" { return "jin".to_string(); }
    if dhatu=="jYA" { return "jAn".to_string(); }
    if dhatu.ends_with("lI") { return format!("{}lin", &dhatu[..dhatu.len()-2]); }
    dhatu.to_string()
}

pub fn g9_r_lang_root(dhatu: &str) -> String {
    if dhatu=="SF" { return "SIr".to_string(); }
    if dhatu.ends_with('F') && dhatu.len()>=2 { return format!("{}f", &dhatu[..dhatu.len()-1]); }
    if dhatu.ends_with('I') && dhatu.len()==2 { let c=dhatu.chars().next().unwrap().to_ascii_lowercase(); return format!("{}i", c); }
    if dhatu=="F" { return "".to_string(); }
    dhatu.to_string()
}

pub fn ya_present_base(dhatu: &str) -> String {
    match dhatu {
        "jF" => return "jIr".to_string(),
        "JF" => return "JIr".to_string(),
        "raYj" => return "raj".to_string(),
        "vyaD" => return "viD".to_string(),
        "mid" => return "med".to_string(),
        "klam" => return "klA".to_string(),
        "So" => return "S".to_string(),
        "Co" => return "C".to_string(),
        "so" => return "s".to_string(),
        "do" => return "d".to_string(),
        _ => {}
    }
    if dhatu.ends_with("iv") { return format!("{}Iv", &dhatu[..dhatu.len()-2]); }
    if dhatu.ends_with("as") || dhatu.ends_with("am") || dhatu.ends_with("ah") { return dhatu.to_string(); }
    if dhatu.ends_with("MS") { return format!("{}S", &dhatu[..dhatu.len()-2]); }
    let chars: Vec<char> = dhatu.chars().collect();
    for idx in (0..chars.len().saturating_sub(1)).rev() {
        if chars[idx]=='a' && !is_vowel_final(chars[idx+1]) {
            let mut out=String::new();
            for &c in &chars[0..idx] { out.push(c); }
            out.push('A');
            for &c in &chars[idx+1..] { out.push(c); }
            return out;
        }
    }
    dhatu.to_string()
}

pub fn sad_present_base(dhatu: &str) -> Option<String> {
    match dhatu {
        // 7.3.77 इषुगमियमां छः (इष् is तुदादि `g6_plot_base`; यम् is `yam_cc` for घटादि).
        "gam" => return Some("gacC".to_string()),
        // 7.3.78 पाघ्राध्मास्थाम्नादाण्दृश्यर्तिसर्तिशदसदां पिबजिघ्रधमतिष्ठमनयच्छपश्यर्च्छधौशीयसीदाः.
        // सर्ति→धौ only वेगितायां; शद्→शीय is आत्मने — not folded here.
        "pA" => return Some("pib".to_string()),
        "GrA" => return Some("jiGr".to_string()),
        "DmA" => return Some("Dam".to_string()),
        "sTA" | "zWA" => return Some("tizW".to_string()),
        "mnA" => return Some("man".to_string()),
        "dA" | "dAR" => return Some("yacC".to_string()),
        "dfS" | "dfSir" => return Some("paSy".to_string()),
        "f" => return Some("fcC".to_string()),
        "sad" => return Some("sId".to_string()),
        "guh" => return Some("gUh".to_string()),
        "saYj" | "sanj" => return Some("saj".to_string()),
        _ => {}
    }
    if dhatu.ends_with('u') && dhatu.len()<=4 && dhatu!="gu" {
        return Some(format!("{}av", &dhatu[..dhatu.len()-1]));
    }
    None
}

pub fn g6_present_base(dhatu: &str) -> String {
    if let Some(sp) = sad_present_base(dhatu) { return sp; }
    if dhatu=="SuB" { return apply_guna_to_stem(dhatu); }
    if dhatu.ends_with('U') { return format!("{}uv", &dhatu[..dhatu.len()-1]); }
    if dhatu.ends_with('u') { return format!("{}v", dhatu); }
    if (dhatu.ends_with('i') || dhatu.ends_with('I')) && dhatu.len()<=3 { return format!("{}y", dhatu); }
    if dhatu.ends_with('F') || dhatu.ends_with('f') {
        let g=apply_guna_to_stem(dhatu);
        return if g!=dhatu { g } else { dhatu.to_string() };
    }
    if dhatu.starts_with('f') && dhatu.len()>=2 { return apply_guna_to_stem(dhatu); }
    if dhatu.contains("jj") && dhatu.contains('a') {
        if let Some(idx)=dhatu.find('a') {
            return format!("{}Bf{}", &dhatu[..idx], &dhatu[idx+1..]);
        }
    }
    if dhatu.ends_with("Sc") && dhatu.contains('a') {
        if let Some(idx)=dhatu.rfind('a') {
            return format!("{}f{}", &dhatu[..idx], &dhatu[idx+1..]);
        }
    }
    if dhatu.len()==4 && matches!(dhatu.chars().nth(2), Some('a'|'A')) {
        let c0=dhatu.chars().next().unwrap();
        let c3=dhatu.chars().nth(3).unwrap();
        return format!("{}i{}", c0, c3);
    }
    dhatu.to_string()
}

const G6_PLOT_LANG: &[&str] = &["Brajj","Kid","kft","lip","lup","majj","muc","piS","pracC","sic","stfMh","SuB","tvac","vid"];

pub fn g6_plot_base(dhatu: &str) -> String {
    if dhatu.starts_with('f') && dhatu.len()>=2 { return dhatu.to_string(); }
    if dhatu.ends_with('F') { return format!("{}ir", &dhatu[..dhatu.len()-1]); }
    if dhatu=="iz" { return "icC".to_string(); } // 7.3.77 इषु (तुदादि इच्छति)
    if dhatu=="vicC" { return "vicCAy".to_string(); }
    if dhatu=="vraSc" { return "vfSc".to_string(); }
    if dhatu=="pracC" { return "pfcC".to_string(); }
    if G6_PLOT_LANG.contains(&dhatu) { return g6_lang_base(dhatu); }
    if dhatu=="sPar" || dhatu=="sPal" { return dhatu.to_string(); }
    if dhatu.ends_with("Sc") && dhatu.contains('a') {
        if let Some(idx)=dhatu.rfind('a') { return format!("{}f{}", &dhatu[..idx], &dhatu[idx+1..]); }
    }
    g6_present_base(dhatu)
}

pub fn g6_lang_base(dhatu: &str) -> String {
    match dhatu {
        "Brajj" => return "Bfjj".to_string(),
        "majj" => return "majj".to_string(),
        "SuB" => return "SuB".to_string(),
        "iz" => return "EcC".to_string(),
        "pracC" => return "prafcC".to_string(), // must be before Sc rule
        "stfMh" => return "stfh".to_string(),
        "tvac" => return "tvac".to_string(),
        "vid" => return "vind".to_string(),
        "muc" => return "muYc".to_string(),
        "lup" => return "lump".to_string(),
        "lip" => return "limp".to_string(),
        "sic" => return "siYc".to_string(),
        "kft" => return "kfnt".to_string(),
        "Kid" => return "Kind".to_string(),
        "piS" => return "piMS".to_string(),
        _ => {}
    }
    if dhatu.ends_with('F') { return format!("{}ir", &dhatu[..dhatu.len()-1]); }
    if dhatu.starts_with('f') {
        let g=apply_guna_to_stem(dhatu);
        if g.starts_with("ar") { return format!("A{}", &g[1..]); }
        return g;
    }
    if dhatu.ends_with("Sc") && dhatu.contains('a') {
        if let Some(idx)=dhatu.rfind('a') { return format!("{}f{}", &dhatu[..idx], &dhatu[idx+1..]); }
    }
    if dhatu.len()==4 && dhatu.starts_with("sP") { return dhatu.to_string(); }
    g6_present_base(dhatu)
}

pub fn g6_lang_stem(dhatu: &str) -> (String, Option<String>) {
    if !dhatu.is_empty() && matches!(dhatu.chars().next().unwrap(), 'u'|'U') {
        if let Some(init)=vowel_initial_lang_stem(dhatu) { return (init, None); }
    }
    if dhatu.starts_with('f') || dhatu=="iz" { return (g6_lang_base(dhatu), None); }
    if let Some(init)=vowel_initial_lang_stem(dhatu) {
        if matches!(dhatu.chars().next().unwrap(), 'i'|'I') { return (init, None); }
    }
    (g6_lang_base(dhatu), Some("a".to_string()))
}

const CAUSATIVE_GUNA_AY: &[&str] = &["yam","cap","cah","rah","bal","jYap"];

pub const _CAUSATIVE_LANG_BASE: &[&str] = &["ci","jYA","kfp","Gf","kFt","cyu","BU","lI","gUd","gup","uDras","Card","raMh","mfj"];
pub const _CAUSATIVE_LANG_NO_AUG: &[&str] = &["uDras","Una","anDa","aMsa","aNka","aNga"];

fn causative_aya_base(dhatu: &str) -> String {
    // 10th gaṇa nasal infix: citi→cintaya, yatri→yantraya, jri→jaraya? etc.
    if matches!(dhatu, "citi" | "yatri" | "kudri" | "tatri" | "matri") {
        let base = &dhatu[..dhatu.len()-1]; // strip i
        if let Some(pos) = base.rfind('t') {
            let mut s = base.to_string();
            s.insert(pos, 'n');
            return format!("{}aya", s);
        }
    }
    if dhatu == "jri" { return "jAraya".to_string(); }
    if CAUSATIVE_GUNA_AY.contains(&dhatu) { return format!("{}aya", apply_guna_to_stem(dhatu)); }
    if matches!(dhatu.chars().last(), Some('U'|'u'|'f'|'F')) { return format!("{}aya", apply_guna_to_stem(dhatu)); }
    let graded=apply_causative_grade(dhatu);
    if graded.ends_with('A') && graded!=dhatu { return format!("{}aya", &graded[..graded.len()-1]); }
    format!("{}aya", graded)
}
pub fn causative_present_stem(dhatu: &str) -> String { causative_aya_base(dhatu) }

pub fn lang_geminate_stem(dhatu: &str, stem: &str) -> String {
    let chars: Vec<char>=dhatu.chars().collect();
    if chars.len()==4 && chars[0]=='C' && matches!(chars[1], 'a'|'A') && matches!(chars[2], 'r'|'n'|'Y'|'j'|'J') {
        return format!("{}{}", chars[0].to_ascii_lowercase(), stem);
    }
    if chars.len()==3 && chars[0]=='C' && matches!(chars[1], 'a'|'A') {
        return format!("{}{}", chars[0].to_ascii_lowercase(), stem);
    }
    stem.to_string()
}

pub fn apply_vrddhi_to_stem(stem: &str) -> String {
    let chars: Vec<char>=stem.chars().collect();
    for idx in (0..chars.len()).rev() {
        match chars[idx] {
            'a' => { let mut o=String::new(); for &c in &chars[0..idx]{o.push(c);} o.push('A'); for &c in &chars[idx+1..]{o.push(c);} return o; }
            'i' => { let mut o=String::new(); for &c in &chars[0..idx]{o.push(c);} o.push('I'); for &c in &chars[idx+1..]{o.push(c);} return o; }
            'u' => { let mut o=String::new(); for &c in &chars[0..idx]{o.push(c);} o.push('U'); for &c in &chars[idx+1..]{o.push(c);} return o; }
            'f' | 'F' => { let mut o=String::new(); for &c in &chars[0..idx]{o.push(c);} o.push_str("Ar"); for &c in &chars[idx+1..]{o.push(c);} return o; }
            'A'|'I'|'U'|'E'|'O' => return stem.to_string(),
            _ => {}
        }
    }
    stem.to_string()
}

pub fn causative_lang_stem(dhatu: &str) -> String {
    // ---------------------------------------------------------------------------
    // const `MAP`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    const MAP: &[(&str,&str)] = &[("ci","capay"),("jYA","jYApay"),("kfp","kalpay"),("Gf","GAray"),("kFt","kIrtay"),("cyu","cyAvay"),("BU","BAvay"),("lI","lay"),("gUd","gUrday"),("gup","gop"),("uDras","ODrAsay"),("Card","cCarday"),("raMh","raNg"),("mfj","mArj")];
    for (k,v) in MAP { if *k==dhatu { return v.to_string(); } }
    if let Some(init)=vowel_initial_lang_stem(dhatu) { return format!("{}ay", init); }
    if dhatu.len()==4 && dhatu.starts_with('C') && matches!(dhatu.chars().nth(1), Some('a'|'A')) {
        let aya=causative_aya_base(dhatu);
        let body=if aya.ends_with("aya") { &aya[..aya.len()-3] } else { dhatu };
        return format!("{}{}", dhatu.chars().next().unwrap().to_ascii_lowercase(), body);
    }
    if dhatu.len()==3 && dhatu.starts_with('C') {
        let aya=causative_aya_base(dhatu);
        let body=if aya.ends_with("aya") { &aya[..aya.len()-3] } else { dhatu };
        return format!("{}{}", dhatu.chars().next().unwrap().to_ascii_lowercase(), body);
    }
    let stem=causative_lang_base_inner(dhatu);
    lang_geminate_stem(dhatu, &stem)
}
fn causative_lang_base_inner(dhatu: &str) -> String {
    // ---------------------------------------------------------------------------
    // const `MAP`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    const MAP: &[(&str,&str)] = &[("ci","capay"),("jYA","jYApay"),("kfp","kalpay"),("Gf","GAray"),("kFt","kIrtay"),("cyu","cyAvay"),("BU","BAvay"),("lI","lay"),("gUd","gUrday"),("gup","gop"),("uDras","ODrAsay"),("Card","cCarday"),("raMh","raNg"),("mfj","mArj")];
    for (k,v) in MAP { if *k==dhatu { return v.to_string(); } }
    if dhatu.len()==4 && dhatu.starts_with('C') && matches!(dhatu.chars().nth(1), Some('a'|'A')) {
        let aya=causative_aya_base(dhatu);
        let body=if aya.ends_with("aya") { &aya[..aya.len()-3] } else { dhatu };
        return format!("{}{}", dhatu.chars().next().unwrap().to_ascii_lowercase(), body);
    }
    if dhatu.len()==3 && dhatu.starts_with('C') {
        let aya=causative_aya_base(dhatu);
        let body=if aya.ends_with("aya") { &aya[..aya.len()-3] } else { dhatu };
        return format!("{}{}", dhatu.chars().next().unwrap().to_ascii_lowercase(), body);
    }
    let aya=causative_aya_base(dhatu);
    aya[..aya.len()-1].to_string()
}

pub fn causative_vidhilin_stem(dhatu: &str, tags: &str) -> String {
    if tags.contains("nityaRic") && dhatu=="raMh" { return "raNg".to_string(); }
    // ---------------------------------------------------------------------------
    // const `MAP`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    const MAP: &[(&str,&str)] = &[("ci","capay"),("jYA","jYApay"),("kfp","kalpay"),("Gf","GAray"),("kFt","kIrtay"),("cyu","cyAvay"),("BU","BAvay"),("lI","lay"),("gUd","gUrday"),("mfj","mArj")];
    for (k,v) in MAP { if *k==dhatu { return v.to_string(); } }
    if dhatu=="sad" { return "AsAday".to_string(); }
    if dhatu.ends_with('I') { return format!("{}ay", &dhatu[..dhatu.len()-1]); }
    if dhatu.len()==3 && dhatu.starts_with('C') {
        let aya=causative_aya_base(dhatu);
        if aya.ends_with("aya") && aya[..aya.len()-3].eq_ignore_ascii_case(dhatu) { return dhatu.to_string(); }
    }
    let aya=causative_aya_base(dhatu);
    if aya.ends_with("aya") { return aya[..aya.len()-1].to_string(); }
    aya
}

pub fn g2_vidhilin_stem(dhatu: &str) -> String {
    if matches!(dhatu, "duh"|"dih"|"lih"|"i"|"as"|"brU"|"vI") { if dhatu=="as" { return "s".to_string(); } return dhatu.to_string(); }
    if dhatu.ends_with('u') && dhatu!="i" && dhatu!="as" { return dhatu.to_string(); }
    apply_guna_to_stem(dhatu)
}
pub fn g6_vidhilin_stem(dhatu: &str) -> String {
    if dhatu=="Dru" { return "Dru".to_string(); }
    g6_plot_base(dhatu)
}

pub fn g7_vidhilin_stem(dhatu: &str) -> String {
    if dhatu.ends_with("Ms") || dhatu.ends_with("nd") { return dhatu.to_string(); }
    // ---------------------------------------------------------------------------
    // const `INFIX`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    const INFIX: &[(&str,&str)]=&[("id","ind"),("ic","iYc"),("ud","und"),("uj","uYj"),("fd","fnd"),("ft","fnt"),("iz","iMz"),("fh","fMh"),("ij","iYj"),("fj","fYj"),("fc","fYc")];
    for (s,r) in INFIX { if dhatu.ends_with(s) { return format!("{}{}", &dhatu[..dhatu.len()-s.len()], r); } }
    apply_guna_to_stem(dhatu)
}

pub fn g9_vidhilin_stem(dhatu: &str, antarganas: &str) -> String {
    // ---------------------------------------------------------------------------
    // const `MAP`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    const MAP: &[(&str,&str)]=&[("F","fRI"),("Kac","KacYI"),("Kav","KOnI"),("grah","gfhRI"),("svF","svUrRI")];
    for (k,v) in MAP { if *k==dhatu { return v.to_string(); } }
    if g9_uses_n_infix(dhatu, antarganas) {
        if dhatu.ends_with('I') && dhatu.len()==2 { return format!("{}InI", &dhatu[..1]); }
        if dhatu=="DU" { return "DunI".to_string(); }
        if dhatu[1..].contains('n') {
            if let Some(idx)=dhatu.rfind('n') { return format!("{}{}nI", &dhatu[..idx], &dhatu[idx+1..]); }
        }
        let base=g9_n_lang_base(dhatu);
        return if base.ends_with('n') { format!("{}I", base) } else { format!("{}nI", base) };
    }
    format!("{}RI", g9_r_lang_root(dhatu))
}

pub fn thematic_aya_present_stem(dhatu: &str) -> Option<String> {
    if dhatu.ends_with('E') { return Some(format!("{}Aya", &dhatu[..dhatu.len()-1])); }
    if dhatu=="gup" { return Some(format!("{}Aya", apply_guna_to_stem(dhatu))); }
    if dhatu=="DUp" { return Some(format!("{}Aya", dhatu)); }
    if dhatu=="paR" || dhatu=="pan" { return Some(format!("{}Aya", dhatu)); }
    None
}

pub fn thematic_present_base(dhatu: &str, gana: u8, aupadeshik: &str) -> String {
    if let Some(sp)=sad_present_base(dhatu) { return sp; }
    if dhatu=="sUrkzy" && aupadeshik.starts_with('z') { return "sUkzy".to_string(); }
    if gana==1 {
        if let Some(rv)=g1_rv_nv_present_base(dhatu) { return rv; }
    }
    if gana==6 { return dhatu.to_string(); }
    if dhatu.contains('W') && dhatu.len()>3 && (dhatu.ends_with("iv")||dhatu.ends_with("Iv")||dhatu.ends_with("uv")||dhatu.ends_with("Uv")) {
        let idx=dhatu.len()-2;
        let ch=dhatu.chars().nth(idx).unwrap();
        if ch=='i' { return format!("{}I{}", &dhatu[..idx], &dhatu[idx+1..]); }
        if ch=='u' { return format!("{}U{}", &dhatu[..idx], &dhatu[idx+1..]); }
    }
    if (dhatu.starts_with("kzI") || dhatu.starts_with("kzU")) && (dhatu.ends_with("Iv") || dhatu.ends_with("Uv")) {
        return apply_guna_to_stem(dhatu);
    }
    if dhatu.len()==4 && matches!(dhatu.chars().next(), Some('k'|'g'|'c'|'j'|'w'|'q'|'t'|'p')) && matches!(dhatu.chars().nth(2), Some('a')) && matches!(dhatu.chars().nth(3), Some('m'|'n')) {
        return apply_vrddhi_to_stem(dhatu);
    }
    let chars: Vec<char>=dhatu.chars().collect();
    for idx in (0..chars.len()).rev() {
        if is_vowel_final(chars[idx]) {
            let trailing: String=chars[idx+1..].iter().collect();
            if trailing.len()<=1 {
                if matches!(chars[idx], 'I'|'U'|'F') && idx!=chars.len()-1 { return dhatu.to_string(); }
                return apply_guna_to_stem(dhatu);
            }
            return dhatu.to_string();
        }
    }
    dhatu.to_string()
}

pub fn thematic_join(stem_a: &str, ending: &str) -> String {
    if !(stem_a.ends_with('a') || stem_a.ends_with('A')) { return format!("{}{}", stem_a, ending); }
    if ending.is_empty() { return stem_a.to_string(); }
    let first = ending.chars().next().unwrap();
    // a/A + a -> a/A (elide one a)  e.g. gacCa + ante -> gacCante, SrA + atAm -> SrAtAm
    if first == 'a' { return format!("{}{}", stem_a, &ending[1..]); }
    // a + A/e/E/o/O/i/I/u/U/f/F -> drop stem a and keep vowel (sandhi: pra+eti->preti)
    // covers Atmanepada ete, etc.: gacCa + ete -> gacCete (not gacCaete)
    if "AEIOUaeioufF".contains(first) { // vowel-initial ending
        return format!("{}{}", &stem_a[..stem_a.len()-1], ending);
    }
    format!("{}{}", stem_a, ending)
}

/// 6.4.25 दंशसञ्जस्वञ्जां शपि; 6.4.26 रञ्जेश्च — nasal upadha drops before शप्.
pub fn sapi_upadha_lopa(root: &str) -> String {
    match root {
        "danS" | "daMS" => "daS".into(),
        "sanj" | "saYj" => "saj".into(),
        "svanj" | "svaYj" => "svaj".into(),
        "ranj" | "raYj" => "raj".into(),
        other => other.to_string(),
    }
}

/// 8.4.40 स्तोः श्चुना श्चुः (षस्ज → सज्ज्; तुक्+छ → च्छ).
pub fn stoh_scuna(root: &str) -> String {
    root.replace("tC", "cC").replace("sj", "jj").replace("sc", "cc")
}

/// 8.2.78 उपधायां च — इक् before र्/व् + हल् is lengthened (ऊर्वति, हूर्छति, कूर्दते).
pub fn rv_upadha_dirgha(root: &str) -> String {
    let mut c: Vec<char> = root.chars().collect();
    for i in 0..c.len().saturating_sub(2) {
        if matches!(c[i], 'i' | 'u' | 'f' | 'x')
            && matches!(c[i + 1], 'r' | 'v')
            && !is_vowel_final(c[i + 2])
        {
            c[i] = match c[i] {
                'i' => 'I',
                'u' => 'U',
                'f' => 'F',
                'x' => 'X',
                other => other,
            };
        }
    }
    c.into_iter().collect()
}

/// 6.1.73 छे च / 6.1.75 दीर्घात् — तुक् before छ immediately after a vowel.
pub fn che_tuk(root: &str) -> String {
    let c: Vec<char> = root.chars().collect();
    let mut out = String::new();
    for i in 0..c.len() {
        if c[i] == 'C' && i > 0 && is_vowel_final(c[i - 1]) {
            out.push('t');
        }
        out.push(c[i]);
    }
    out
}

/// 7.3.75 ष्ठिवुक्लम्याचमां शिति — ष्ठिवु / क्लम् lengthen before शित् (आचम् needs आ).
pub fn sthivu_klamu_shiti(root: &str) -> String {
    match root {
        "zWiv" => "zWIv".into(),
        "klam" => "klAm".into(),
        other => other.to_string(),
    }
}

/// 7.1.58 इदितो नुम् धातोः — nasal before the last consonant after dropping i-इत्.
pub fn idito_num(dhatu: &str) -> Option<String> {
    if !dhatu.ends_with('i') || dhatu.len() < 3 {
        return None;
    }
    let base = &dhatu[..dhatu.len() - 1];
    let last = base.chars().last()?;
    if is_vowel_final(last) {
        return None;
    }
    let nasal = if matches!(last, 'K' | 'G' | 'k' | 'g') {
        'N'
    } else if matches!(last, 'q' | 'Q' | 'w' | 'W') {
        'R'
    } else if matches!(last, 'c' | 'C' | 'j' | 'J') {
        'Y'
    } else if last == 'N' {
        'N'
    } else {
        'n'
    };
    Some(format!("{}{}{}", &base[..base.len() - last.len_utf8()], nasal, last))
}

/// 3.1.80 धिन्विकृण्व्योर च — श्नु aṅga after 7.1.58 (धिनु, कृणु).
pub fn dhinvi_krnvi_snu_base(dhatu: &str) -> Option<&'static str> {
    match dhatu {
        "Divi" => Some("Din"),
        "kfvi" => Some("kfR"),
        _ => None,
    }
}

pub fn apply_nasal_palatal(word: &str) -> String {
    // n before palatal c/C/j/J/Y/S -> Y (e.g., kunca->kuYca, kuncati->kuYcati)
    if !word.contains('n') { return word.to_string(); }
    let mut s = word.to_string();
    for (a, b) in [("nc", "Yc"), ("nC", "YC"), ("nj", "Yj"), ("nJ", "YJ")] {
        s = s.replace(a, b);
    }
    // n before labial p/P/b/B/m -> m (tunpa->tumpa, tunPa->tumPa)
    for (a, b) in [("np", "mp"), ("nP", "mP"), ("nb", "mb"), ("nB", "mB")] {
        s = s.replace(a, b);
    }
    // n before s/S/h -> M (anusvAra 8.3.23 mo'nusvAraH: raMhati, SaMsati, dfMhati)
    for (a, b) in [("ns", "Ms"), ("nS", "MS"), ("nh", "Mh")] {
        s = s.replace(a, b);
    }
    s
}

pub fn apply_natva_to_word(word: &str) -> String {
    if !word.contains('n') { return word.to_string(); }
    let mut chars: Vec<char> = word.chars().collect();
    let blockers: std::collections::HashSet<char> = ['c','C','j','J','Y','S','w','W','q','Q','R','t','T','d','D','l','s','S'].iter().cloned().collect();
    let mut trigger = false;
    let mut trigger_z = false;
    for i in 0..chars.len() {
        let ch = chars[i];
        if matches!(ch, 'r'|'f'|'F'|'z') {
            trigger = true;
            trigger_z = ch == 'z';
        } else if trigger && ch == 'n' {
            if i != chars.len() - 1 {
                let next_ch = chars[i + 1];
                let next_is_last = i + 1 == chars.len() - 1;
                // तिङ् आनि after र (प्रभवानि) is not णत्व; after ष it is (द्वेषाणि, क्षवाणि).
                if next_is_last && matches!(next_ch, 'i' | 'I') && !trigger_z {
                    // leave n
                } else if "aAiIuUfFeEoOyvm".contains(next_ch) {
                    chars[i] = 'R';
                }
            }
        } else if trigger && blockers.contains(&ch) {
            trigger = false;
            trigger_z = false;
        }
    }
    chars.into_iter().collect()
}
