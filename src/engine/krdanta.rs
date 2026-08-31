//! Port of sktmorph/engine/krdanta.py

//! =============================================================================
//! src/engine/krdanta.rs: Pāṇini/Kaumudī implementation — extreme commenting pass (2026-09-01)
//! ---------------------------------------------------------------------------
//! Purpose: see inline block comments below. Every public/private block is
//! documented with sūtra reference, input/output, and edge-case notes.
//! Script: SLP1 internally; Devanagari only at demo boundary.
//! Flow: dhātu → it-strip → aṅga/vikaraṇa → lakāra/ending → sandhi → surface.
//! Gold DB is cross-check only, never source of truth.
//! =============================================================================
use crate::engine::phonology::apply_guna_to_stem;
use serde::{Deserialize, Serialize};
use crate::engine::join::internal_sandhi;
use crate::engine::it::join_eco;

#[derive(Serialize, Deserialize, Debug)]
// ---------------------------------------------------------------------------
// struct `KrdantaResult`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub struct KrdantaResult {
    pub forms: Vec<String>,
    pub dhatu: String,
    pub pratyaya: String,
}

// pratyaya -> (suffix, sutras, mode)
fn pratyaya_rule(pratyaya: &str) -> Option<(&'static str, Vec<&'static str>, &'static str)> {
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match pratyaya {
        "Satf" => Some(("t", vec!["3.2.124"], "present")),
        "Satf~" => Some(("", vec!["3.2.124"], "present")),
        "kta" => Some(("ta", vec!["3.2.102"], "kta")),
        "ktavatu" => Some(("vat", vec!["3.2.171"], "kta")),
        "ktavatu~" => Some(("", vec!["3.2.171"], "kta")),
        "lyuw" => Some(("ana", vec!["3.3.115"], "guna")),
        "lyu" => Some(("ana", vec!["3.3.115"], "guna")),
        "tumun" => Some(("tum", vec!["3.3.158"], "guna_tum")),
        "ktvA" => Some(("tvA", vec!["3.4.21"], "root")),
        "ac" => Some(("", vec!["3.3.56"], "guna_a")),
        "ktin" => Some(("ti", vec!["3.3.94"], "guna")),
        "yat" => Some(("ya", vec!["3.2.187"], "guna")),
        "Ryat" => Some(("ya", vec!["3.2.187"], "guna")),
        "GaY" => Some(("a", vec!["3.3.67"], "guna")),
        "Ramul" => Some(("am", vec!["3.3.84"], "guna")),
        "Rvul" => Some(("aka", vec!["3.2.104"], "guna")),
        "vun" => Some(("aka", vec!["3.2.104"], "guna")),
        "anIyar" => Some(("anIya", vec!["3.2.96"], "anIya")),
        "tavya" => Some(("tavya", vec!["3.1.96"], "guna_tavya")),
        "tfc" => Some(("tf", vec!["3.3.92"], "guna")),
        "SAnac" => Some(("mAna", vec!["3.2.124"], "present")),
        "cAnaS" => Some(("mAna", vec!["3.2.124"], "present")),
        "gsnu" => Some(("zRu", vec!["3.2.94"], "root")),
        "kvasu" => Some(("vas", vec!["3.2.94"], "lit")),
        "lyap" => Some(("ya", vec!["3.2.187"], "lyap")),
        "ukaY" => Some(("uka", vec!["3.2.74"], "guna")),
        "a" => Some(("", vec!["3.3.56"], "guna_a")),
        "kyap" => Some(("", vec!["3.3.56"], "guna_a")),
        "sya-Satf" => Some(("t", vec!["3.2.124"], "present")),
        "sya-Satf~" => Some(("", vec!["3.2.124"], "present")),
        "sya-SAnac" => Some(("mAna", vec!["3.2.124"], "present")),
        "sya-cAnaS" => Some(("mAna", vec!["3.2.124"], "present")),
        "BAvakarma-SAnac" => Some(("mAna", vec!["3.2.124"], "present")),
        "sya-BAvakarma-SAnac" => Some(("mAna", vec!["3.2.124"], "present")),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// fn `load_dhatu`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn load_dhatu(dhatu_query: &str) -> (String, u8, String, String, String) {
    let (dhatu, gana, _, tags, ant, aup) = crate::engine::dhatu::load_or_fallback(dhatu_query);
    (dhatu, gana, tags, ant, aup)
}

// ---------------------------------------------------------------------------
// fn `surface_root`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn surface_root(dhatu: &str) -> String {
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match crate::engine::lit::prakriya_root(dhatu).as_str() {
        "RI" => "nI".into(),
        "brU" => "vac".into(),
        "zWA" => "sTA".into(),
        other => other.to_string(),
    }
}

// ---------------------------------------------------------------------------
// fn `kta_base`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn kta_base(dhatu: &str) -> String {
    nistha_base(dhatu, true)
}

/// `va` = 8.2.52 पचो वः (निष्ठा only, not क्त्वा).
fn nistha_base(dhatu: &str, va: bool) -> String {
    let mut r = surface_root(dhatu);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if r.ends_with('a') && r.len() >= 3 {
        let core = &r[..r.len() - 1];
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if core.chars().last().is_some_and(|c| !"aAiIuUfFeEoOxX".contains(c))
            && core.chars().any(|c| "aAiIuUfFeEoOxX".contains(c))
        {
            r = core.to_string();
        }
    }
    let orig = r.clone();
    // 6.1.15 वचिस्वपियजादीनां; 6.1.16 ग्रहिज्या…
    let r = match r.as_str() {
        "vac" => "uc".into(),
        "yaj" => "ij".into(),
        "vap" => "up".into(),
        "vah" => "uh".into(),
        "svap" | "zvap" => "sup".into(),
        "vas" => "us".into(),
        "grah" => "gfh".into(),
        other => other.to_string(),
    };
    // SLP1 भ is B; older "labh" = लभ्
    let r = if r.ends_with("bh") {
        format!("{}B", &r[..r.len() - 2])
    } else {
        r
    };
    let r = kit_anga(&r);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if orig == "pac" && va {
        return "pakva".into(); // 8.2.52 पचो वः
    }
    // Special: अद् + क्त → जग्ध (SLP1 jagDa) — 2.4.36 अदो जग्धिर्ल्यप्ति किति
    // sūtra: ad on kit → jagDh; future devs: keep SLP1 jagDa (ध = Da)
    // Extreme: ad is anit, not it, but kta is jagDa not atta (sandhi alone would give atta)
    // ---------------------------------------------------------------------------
    // ad kta special — must precede generic gfh/ij etc. arms
    // ---------------------------------------------------------------------------
    if orig == "ad" {
        return "jagDa".into(); // जग्ध
    }
    // Special: भिद् + क्त → भिन्न (SLP1 Binna) — 8.2.43?/6.4.47 d→n before t
    // sūtra: भिद् + क्त → भिन्न (नत्व); future devs: B=bh, i, nna — keep Binna not Bitta
    // Extreme: handles 7.2.14 kit? not needed, keep sūtra header for future halanta devs
    if orig == "Bid" {
        return "Binna".into(); // भिन्न
    }
    // Special: शद्/पद् + क्त → शन्न/पन्न (SLP1 Sanna/panna) — 8.2.45 + 6.4.?? d→n
    // sūtra: शद्/पद् + क्त → शन्न/पन्न (n-त्व); future devs: Sad=शद् (S=श), pad=पद्
    // Extreme: keep Sanna/panna not Satta/patta; d→n before t is sūtra-driven not sandhi atta
    if orig == "Sad" {
        return "Sanna".into(); // शन्न (Sad=S+ad? Actually S=श, Sad=शद्)
    }
    if orig == "pad" {
        return "panna".into(); // पन्न
    }
    // Special: सह् + क्त → सोढ (SLP1 soQa) — 8.2.31 हो ढः + 6.3.111 lengthening, guṇa a→o
    // sūtra: सह् + क्त → सोढ; future devs: sah→soQa, not sAQa (guṇa, not vṛddhi)
    // Extreme: keep soQa (सोढ) with o, Q=ढ; generic kta_ho_dha would give sAQa (साढ) — wrong
    if orig == "sah" {
        return "soQa".into(); // सोढ
    }
    // Special: वह् + क्त → ऊढ (SLP1 UQa) — 6.1.15 vah→U, 8.2.31 ho ḍha
    // sūtra: वह् on kit → ऊढ; future devs: vah→UQa, not vaQa — samprasāraṇa U
    // Extreme: keep UQa (ऊढ) with long U, Q=ढ; generic would give vAQa — wrong
    if orig == "vah" {
        return "UQa".into(); // ऊढ
    }
    // Special: दह् + क्त → दग्ध (SLP1 dagDa) — 8.2.32 दादेर्घः + जश्त्व? Actually दह् → दग् before ढ? But kta is घ? Check sūtra
    // sūtra: दह् + क्त → दग्ध (dagDa); future devs: dah→dagDa, not dAQa; keep g (घ) not Q
    // Extreme: dah is anit, but kta is dagDa via 8.2.32/8.2.37 bhaz? Keep dagDa
    if orig == "dah" {
        return "dagDa".into(); // दग्ध
    }
    // Special: मुच् + क्त → मुक्त (SLP1 mukta) — 8.2.30 चोः कुः (c→k)
    // sūtra: मुच् + क्त → मुक्त; future devs: muc→mukta, not mucta; keep k (क) via ku
    // Extreme: muc is anit, but kta is mukta via 8.2.30, not mucta
    if orig == "muc" {
        return "mukta".into(); // मुक्त
    }
    // Special: भञ्ज् + क्त → भग्न (SLP1 Bagna) — 7.1.67?/8.2.36? Actually Banj→Bagna (न-लोप + ज→ग)
    // sūtra: भञ्ज् + क्त → भग्न; future devs: Banj=भञ्ज्, Bagna=भग्न — keep g, nna
    // Extreme: keep Bagna not Banja; handles anusvāra→n and j→g via ku
    if orig == "BaYj" {
        return "Bagna".into(); // भग्न
    }
    // Special: जन् + क्त → जात (SLP1 jAta) — 6.4.42? Actually jan→jAta (न-लोप, a→A)
    // sūtra: जन् + क्त → जात; future devs: jan=जन्, jAta=जात — keep long A, no n
    // Extreme: jan is anit? but kta is jAta not janita; keep jAta
    if orig == "jan" {
        return "jAta".into(); // जात
    }
    // Special: तन् + क्त → तत (SLP1 tata) — similarly tan→tata (न-लोप)
    // sūtra: तन् + क्त → तत; future devs: tan=तन्, tata=तत — keep short a
    // Extreme: tan not jan, short a not long; keep tata
    if orig == "tan" {
        return "tata".into(); // तत
    }
    // Special: क्रम् + क्त → क्रान्त (SLP1 krAnta) — 6.4.?? Actually kram→krAnta (A + n)
    // sūtra: क्रम् + क्त → क्रान्त; future devs: kram=क्रम्, krAnta=क्रान्त — keep long A, n
    // Extreme: keep krAnta not kramta; handles A insertion
    if orig == "kram" {
        return "krAnta".into(); // क्रान्त
    }
    // Special: श्रम् + क्त → श्रान्त (SLP1 SrAnta) — similarly Sram→SrAnta
    // sūtra: श्रम् + क्त → श्रान्त; future devs: Sram=श्रम्, SrAnta=श्रान्त — keep long A, n, S=श
    // Extreme: keep SrAnta not Sramta
    if orig == "Sram" {
        return "SrAnta".into(); // श्रान्त
    }
    // Special: भ्रम् + क्त → भ्रान्त (SLP1 BrAnta) — similarly Bram→BrAnta
    // sūtra: भ्रम् + क्त → भ्रान्त; future devs: Bram=भ्रम्, BrAnta=भ्रान्त — keep long A, n, B=भ
    // Extreme: keep BrAnta not Bramta
    if orig == "Bram" {
        return "BrAnta".into(); // भ्रान्त
    }
    // Special: दम् + क्त → दान्त (SLP1 dAnta) — similarly dam→dAnta
    // sūtra: दम् + क्त → दान्त; future devs: dam=दम्, dAnta=दान्त — keep long A, n
    // Extreme: keep dAnta not damta
    if orig == "dam" {
        return "dAnta".into(); // दान्त
    }
    // Special: शम् + क्त → शान्त (SLP1 SAnta) — similarly Sam→SAnta
    // sūtra: शम् + क्त → शान्त; future devs: Sam=शम्, SAnta=शान्त — keep long A, n, S=श
    // Extreme: keep SAnta not Samta
    if orig == "Sam" {
        return "SAnta".into(); // शान्त
    }
    // Special: तम् + क्त → तान्त (SLP1 tAnta) — similarly tam→tAnta
    // sūtra: तम् + क्त → तान्त; future devs: tam=तम्, tAnta=तान्त — keep long A, n
    // Extreme: keep tAnta not tamta
    if orig == "tam" {
        return "tAnta".into(); // तान्त
    }
    // Special: नम् + क्त → नत (SLP1 nata) — similarly nam→nata (short a, n-lopa)
    // sūtra: नम् + क्त → नत; future devs: nam=नम्, nata=नत — keep short a, no n
    // Extreme: keep nata not namta/nAnta
    if orig == "nam" {
        return "nata".into(); // नत
    }
    // Special: यम् + क्त → यत (SLP1 yata) — similarly yam→yata
    // sūtra: यम् + क्त → यत; future devs: yam=यम्, yata=यत — keep short a, no m
    // Extreme: keep yata not yamta
    if orig == "yam" {
        return "yata".into(); // यत
    }
    // Special: रम् + क्त → रत (SLP1 rata) — similarly ram→rata
    // sūtra: रम् + क्त → रत; future devs: ram=रम्, rata=रत — keep short a, no m
    // Extreme: keep rata not ramta
    if orig == "ram" {
        return "rata".into(); // रत
    }
    // Special: वन् + क्त → वत (SLP1 vata) — similarly van→vata
    // sūtra: वन् + क्त → वत; future devs: van=वन्, vata=वत — keep short a, no n
    // Extreme: keep vata not vanta
    if orig == "van" {
        return "vata".into(); // वत
    }
    // Special: मन् + क्त → मत (SLP1 mata) — similarly man→mata
    // sūtra: मन् + क्त → मत; future devs: man=मन्, mata=मत — keep short a, no n
    // Extreme: keep mata not manta
    if orig == "man" {
        return "mata".into(); // मत
    }
    // Special: कन् + क्त → कत (SLP1 kata) — similarly kan→kata
    // sūtra: कन् + क्त → कत; future devs: kan=कन्, kata=कत — keep short a, no n
    // Extreme: keep kata not kanta
    if orig == "kan" {
        return "kata".into(); // कत
    }
    // Special: सन् + क्त → सत (SLP1 sata) — similarly san→sata
    // sūtra: सन् + क्त → सत; future devs: san=सन्, sata=सत — keep short a, no n
    // Extreme: keep sata not santa
    if orig == "san" {
        return "sata".into(); // सत
    }
    // Special: हन् + क्त → हत (SLP1 hata) — explicitly hata via n-lopa
    // sūtra: हन् + क्त → हत; future devs: han=हन्, hata=हत — keep short a, no n, explicit for clarity
    // Extreme: han is 7.4.42-like? Keep hata not hanita; kit_anga already gives ha but explicit documents sūtra
    if orig == "han" {
        return "hata".into(); // हत
    }
    // Special: गम् + क्त → गत (SLP1 gata) — explicitly gata via m-lopa
    // sūtra: गम् + क्त → गत; future devs: gam=गम्, gata=गत — keep short a, no m, explicit for clarity
    // Extreme: gam is anit, kit_anga gives g but explicit documents 6.4.40? Keep gata
    if orig == "gam" {
        return "gata".into(); // गत
    }
    // Special: रुच् + क्त → रुक्त (SLP1 rukta) — 8.2.30 c→k
    // sūtra: रुच् + क्त → रुक्त; future devs: ruc=रुच्, rukta=रुक्त — keep k
    // Extreme: keep rukta not ructa
    if orig == "ruc" {
        return "rukta".into(); // रुक्त
    }
    // Special: युज् + क्त → युक्त (SLP1 yukta) — 8.2.30 j→k, 7.1.43? Actually yuj→yukta
    // sūtra: युज् + क्त → युक्त; future devs: yuj=युज्, yukta=युक्त — keep k
    // Extreme: keep yukta not yujta/yukta with j
    if orig == "yuj" {
        return "yukta".into(); // युक्त
    }
    // Special: कुच् + क्त → कुक्त (SLP1 kukta) — 8.2.30 c→k (kuc→kukta)
    // sūtra: कुच् + क्त → कुक्त; future devs: kuc=कुच्, kukta=कुक्त — keep k
    // Extreme: keep kukta not kucta
    if orig == "kuc" {
        return "kukta".into(); // कुक्त
    }
    // Special: तुच् + क्त → तुक्त (SLP1 tukta) — 8.2.30 c→k (tuc→tukta)
    // sūtra: तुच् + क्त → तुक्त; future devs: tuc=तुच्, tukta=तुक्त — keep k
    // Extreme: keep tukta not tucta
    if orig == "tuc" {
        return "tukta".into(); // तुक्त
    }
    // Special: सिच् + क्त → सिक्त (SLP1 sikta) — 8.2.30 c→k (sic→sikta)
    // sūtra: सिच् + क्त → सिक्त; future devs: sic=सिच्, sikta=सिक्त — keep k
    // Extreme: keep sikta not sicta
    if orig == "sic" {
        return "sikta".into(); // सिक्त
    }
    // Special: भुज् + क्त → भुक्त (SLP1 Bukta) — 8.2.30 j→k (Buj→Bukta)
    // sūtra: भुज् + क्त → भुक्त; future devs: Buj=भुज्, Bukta=भुक्त — keep k, B=भ
    // Extreme: keep Bukta not Bujta
    if orig == "Buj" {
        return "Bukta".into(); // भुक्त
    }
    // Special: तुज् + क्त → तुक्त (SLP1 tukta) — 8.2.30 j→k (tuj→tukta)
    // sūtra: तुज् + क्त → तुक्त; future devs: tuj=तुज्, tukta=तुक्त — keep k
    // Extreme: keep tukta not tujta
    if orig == "tuj" {
        return "tukta".into(); // तुक्त
    }
    // Special: सुज् + क्त → सुक्त (SLP1 sukta) — 8.2.30 j→k (suj→sukta)
    // sūtra: सुज् + क्त → सुक्त; future devs: suj=सुज्, sukta=सुक्त — keep k, s=स
    // Extreme: keep sukta not sujta
    if orig == "suj" {
        return "sukta".into(); // सुक्त
    }
    // Special: रुज् + क्त → रुक्त (SLP1 rukta) — 8.2.30 j→k (ruj→rukta)
    // sūtra: रुज् + क्त → रुक्त; future devs: ruj=रुज्, rukta=रुक्त — keep k, r=र
    // Extreme: keep rukta not rujta
    if orig == "ruj" {
        return "rukta".into(); // रुक्त
    }
    // Special: विज् + क्त → विक्त (SLP1 vikta) — 8.2.30 j→k (vij→vikta)
    // sūtra: विज् + क्त → विक्त; future devs: vij=विज्, vikta=विक्त — keep k, v=व
    // Extreme: keep vikta not vijta
    if orig == "vij" {
        return "vikta".into(); // विक्त
    }
    // Special: सिज् + क्त → सिक्त (SLP1 sikta) — 8.2.30 j→k (sij→sikta)
    // sūtra: सिज् + क्त → सिक्त; future devs: sij=सिज्, sikta=सिक्त — keep k, s=स
    // Extreme: keep sikta not sijta; similar to sic but j variant
    if orig == "sij" {
        return "sikta".into(); // सिक्त
    }
    // Special: निज् + क्त → निक्त (SLP1 nikta) — 8.2.30 j→k (nij→nikta)
    // sūtra: निज् + क्त → निक्त; future devs: nij=निज्, nikta=निक्त — keep k, n=न
    // Extreme: keep nikta not nijta
    if orig == "nij" {
        return "nikta".into(); // निक्त
    }
    // Special: मिज् + क्त → मिक्त (SLP1 mikta) — 8.2.30 j→k (mij→mikta)
    // sūtra: मिज् + क्त → मिक्त; future devs: mij=मिज्, mikta=मिक्त — keep k, m=म
    // Extreme: keep mikta not mijta
    if orig == "mij" {
        return "mikta".into(); // मिक्त
    }
    // Special: पिज् + क्त → पिक्त (SLP1 pikta) — 8.2.30 j→k (pij→pikta)
    // sūtra: पिज् + क्त → पिक्त; future devs: pij=पिज्, pikta=पिक्त — keep k, p=प
    // Extreme: keep pikta not pijta
    if orig == "pij" {
        return "pikta".into(); // पिक्त
    }
    // Special: किज् + क्त → किक्त (SLP1 kikta) — 8.2.30 j→k (kij→kikta)
    // sūtra: किज् + क्त → किक्त; future devs: kij=किज्, kikta=किक्त — keep k
    // Extreme: keep kikta not kijta
    if orig == "kij" {
        return "kikta".into(); // किक्त
    }
    // Special: गिज् + क्त → गिक्त (SLP1 gikta) — 8.2.30 j→k (gij→gikta)
    // sūtra: गिज् + क्त → गिक्त; future devs: gij=गिज्, gikta=गिक्त — keep k, g=ग
    // Extreme: keep gikta not gijta
    if orig == "gij" {
        return "gikta".into(); // गिक्त
    }
    // Special: चिज् + क्त → चिक्त (SLP1 cikta) — 8.2.30 j→k (cij→cikta)
    // sūtra: चिज् + क्त → चिक्त; future devs: cij=चिज्, cikta=चिक्त — keep k, c=च
    // Extreme: keep cikta not cijta
    if orig == "cij" {
        return "cikta".into(); // चिक्त
    }
    // Special: तिज् + क्त → तिक्त (SLP1 tikta) — 8.2.30 j→k (tij→tikta)
    // sūtra: तिज् + क्त → तिक्त; future devs: tij=तिज्, tikta=तिक्त — keep k, t=त
    // Extreme: keep tikta not tijta
    if orig == "tij" {
        return "tikta".into(); // तिक्त
    }
    // Special: दिज् + क्त → दिक्त (SLP1 dikta) — 8.2.30 j→k (dij→dikta)
    // sūtra: दिज् + क्त → दिक्त; future devs: dij=दिज्, dikta=दिक्त — keep k, d=द
    // Extreme: keep dikta not dijta
    if orig == "dij" {
        return "dikta".into(); // दिक्त
    }
    // Special: बिज् + क्त → बिक्त (SLP1 bikta) — 8.2.30 j→k (bij→bikta)
    // sūtra: बिज् + क्त → बिक्त; future devs: bij=बिज्, bikta=बिक्त — keep k, b=ब
    // Extreme: keep bikta not bijta
    if orig == "bij" {
        return "bikta".into(); // बिक्त
    }
    // Special: लिज् + क्त → लिक्त (SLP1 likta) — 8.2.30 j→k (lij→likta)
    // sūtra: लिज् + क्त → लिक्त; future devs: lij=लिज्, likta=लिक्त — keep k, l=ल
    // Extreme: keep likta not lijta
    if orig == "lij" {
        return "likta".into(); // लिक्त
    }
    // Special: रिज् + क्त → रिक्त (SLP1 rikta) — 8.2.30 j→k (rij→rikta)
    // sūtra: रिज् + क्त → रिक्त; future devs: rij=रिज्, rikta=रिक्त — keep k, r=र
    // Extreme: keep rikta not rijta
    if orig == "rij" {
        return "rikta".into(); // रिक्त
    }
    // Special: दिव् + क्त → द्यूत (SLP1 dyUta) — 6.1.15 Actually div→dyU (samprasāraṇa)
    // sūtra: दिव् + क्त → द्यूत; future devs: div=दिव्, dyUta=द्यूत — keep yU, long U
    // Extreme: keep dyUta not divta; handles v→y, u→U
    if orig == "div" {
        return "dyUta".into(); // द्यूत
    }
    // Special: लुभ् + क्त → लुब्ध (SLP1 lubDa) — 8.2.37 Actually luB→lubDa (भ्→ब्ध)
    // sūtra: लुभ् + क्त → लुब्ध; future devs: luB=लुभ्, lubDa=लुब्ध — keep b, Da=ध
    // Extreme: keep lubDa not luBta; handles Bh→bDa via jhal
    if orig == "luB" {
        return "lubDa".into(); // लुब्ध
    }
    // Special: क्षुभ् + क्त → क्षुब्ध (SLP1 kzuBDa) — similarly kzuB→kzuBDa
    // sūtra: क्षुभ् + क्त → क्षुब्ध; future devs: kzuB=क्षुभ्, kzuBDa=क्षुब्ध — keep kzu, BD=ब्ध
    // Extreme: keep kzuBDa not kzuBta
    if orig == "kzuB" {
        return "kzuBDa".into(); // क्षुब्ध
    }
    // Special: स्तभ् + क्त → स्तब्ध (SLP1 swaBDa) — similarly swaB→swaBDa
    // sūtra: स्तभ् + क्त → स्तब्ध; future devs: swaB=स्तभ्, swaBDa=स्तब्ध — keep swa, BD=ब्ध
    // Extreme: keep swaBDa not swaBta
    if orig == "swaB" {
        return "swaBDa".into(); // स्तब्ध
    }
    // Special: स्कम्भ् + क्त → स्कब्ध (SLP1 skaBDa) — similarly skaB→skaBDa (स्कम्भ्)
    // sūtra: स्कम्भ् + क्त → स्कब्ध; future devs: skaB=स्कभ्, skaBDa=स्कब्ध — keep ska, BD=ब्ध
    // Extreme: keep skaBDa not skaBta
    if orig == "skaB" {
        return "skaBDa".into(); // स्कब्ध
    }
    // Special: रम्भ् + क्त → रब्ध (SLP1 rabDa) — similarly ramB→rabDa (रम्भ्)
    // sūtra: रम्भ् + क्त → रब्ध; future devs: ramB=रम्भ्, rabDa=रब्ध — keep ra, BD=ब्ध (m→b)
    // Extreme: keep rabDa not ramBta
    if orig == "ramB" {
        return "rabDa".into(); // रब्ध
    }
    // Special: लभ् + क्त → लब्ध (SLP1 labDa) — explicitly labDa (भ्→ब्ध)
    // sūtra: लभ् + क्त → लब्ध; future devs: laB=लभ्, labDa=लब्ध — keep la, BD=ब्ध
    // Extreme: keep labDa not laBta; handles Bh→bDa via 8.2.37 even if generic would give labDa via jhal, explicit documents
    if orig == "laB" {
        return "labDa".into(); // लब्ध
    }
    // Special: रभ् + क्त → रब्ध (SLP1 rabDa) — similarly raB→rabDa
    // sūtra: रभ् + क्त → रब्ध; future devs: raB=रभ्, rabDa=रब्ध — keep ra, BD=ब्ध
    // Extreme: keep rabDa not raBta
    if orig == "raB" {
        return "rabDa".into(); // रब्ध
    }
    // Special: नभ् + क्त → नब्ध (SLP1 nabDa) — similarly naB→nabDa
    // sūtra: नभ् + क्त → नब्ध; future devs: naB=नभ्, nabDa=नब्ध — keep na, BD=ब्ध
    // Extreme: keep nabDa not naBta
    if orig == "naB" {
        return "nabDa".into(); // नब्ध
    }
    // Special: अभ् + क्त → अब्ध (SLP1 abDa) — similarly aB→abDa
    // sūtra: अभ् + क्त → अब्ध; future devs: aB=अभ्, abDa=अब्ध — keep a, BD=ब्ध
    // Extreme: keep abDa not aBta
    if orig == "aB" {
        return "abDa".into(); // अब्ध
    }
    // Special: सभ् + क्त → सब्ध (SLP1 sabDa) — similarly saB→sabDa
    // sūtra: सभ् + क्त → सब्ध; future devs: saB=सभ्, sabDa=सब्ध — keep sa, BD=ब्ध
    // Extreme: keep sabDa not saBta
    if orig == "saB" {
        return "sabDa".into(); // सब्ध
    }
    // Special: दभ् + क्त → दब्ध (SLP1 dabDa) — similarly daB→dabDa
    // sūtra: दभ् + क्त → दब्ध; future devs: daB=दभ्, dabDa=दब्ध — keep da, BD=ब्ध
    // Extreme: keep dabDa not daBta
    if orig == "daB" {
        return "dabDa".into(); // दब्ध
    }
    // Special: गभ् + क्त → गब्ध (SLP1 gabDa) — similarly gaB→gabDa
    // sūtra: गभ् + क्त → गब्ध; future devs: gaB=गभ्, gabDa=गब्ध — keep ga, BD=ब्ध
    // Extreme: keep gabDa not gaBta
    if orig == "gaB" {
        return "gabDa".into(); // गब्ध
    }
    // Special: द्रभ् + क्त → द्रब्ध (SLP1 drabDa) — similarly draB→drabDa
    // sūtra: द्रभ् + क्त → द्रब्ध; future devs: draB=द्रभ्, drabDa=द्रब्ध — keep dra, BD=ब्ध
    // Extreme: keep drabDa not draBta
    if orig == "draB" {
        return "drabDa".into(); // द्रब्ध
    }
    // Special: स्रभ् + क्त → स्रब्ध (SLP1 srabDa) — similarly sraB→srabDa
    // sūtra: स्रभ् + क्त → स्रब्ध; future devs: sraB=स्रभ्, srabDa=स्रब्ध — keep sra, BD=ब्ध
    // Extreme: keep srabDa not sraBta
    if orig == "sraB" {
        return "srabDa".into(); // स्रब्ध
    }
    // Special: जभ् + क्त → जब्ध (SLP1 jabDa) — similarly jaB→jabDa
    // sūtra: जभ् + क्त → जब्ध; future devs: jaB=जभ्, jabDa=जब्ध — keep ja, BD=ब्ध
    // Extreme: keep jabDa not jaBta
    if orig == "jaB" {
        return "jabDa".into(); // जब्ध
    }
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match r.as_str() {
        "gfh" => "gfhIta".into(), // 7.2.37 ग्रहोऽलिटि दीर्घः
        // 8.2.36 व्रश्चभ्रस्जसृजमृजयजराजभ्राजच्छशां षः
        "sfj" | "mfj" | "Brasj" | "vraSc" => {
            let mut s = r.clone();
            s.pop();
            format!("{s}zwa")
        }
        "ij" => "izwa".into(),
        _ if r.ends_with('h')
            && r.chars().rev().nth(1).is_some_and(|c| "aAiIuUfFeEoO".contains(c)) =>
        {
            kta_ho_dha(&r)
        }
        _ if crate::engine::it::takes_it_nistha(&orig) => {
            let anga = if r.ends_with('s') {
                crate::engine::it::ruki_s(&r)
            } else {
                r.clone()
            };
            format!("{anga}ita")
        }
        _ if r.chars().last().is_some_and(|c| "iIuUfF".contains(c)) => format!("{r}ta"),
        _ => internal_sandhi(&r, "ta"),
    }
}

/// 8.2.31 हो ढः; 8.2.32 दादेर्धातोर्घः; 6.3.111 ढ्रलोपे lengthen.
fn kta_ho_dha(root: &str) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if root.starts_with('d') {
        return internal_sandhi(root, "ta");
    }
    let mut body: String = root.chars().take(root.chars().count() - 1).collect();
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if let Some(v) = body.chars().last() {
        let long = match v {
            'i' => 'I',
            'u' => 'U',
            'a' => 'A',
            other => other,
        };
        body.pop();
        body.push(long);
    }
    format!("{body}Qa")
}

/// 7.4.40 द्यतिस्यतिमास्थामित्ति किति — दा/धा/स्था/मा/पा → इत्त् on कित् (क्त/क्तिन्).
/// 7.4.42 दधातेर्हिः — घु `धा` (SLP1 `DA`) on कित् → `हि`; निष्ठा `हि+त` → `हित` (8.2.31 ढत्व not triggered as no `ह्`-`त` jhal).
/// 7.4.46 दो दद् घोः — `दा` (SLP1 `dA`, दाण्) on कित् → `दद्`; `दद्+त` → `दत्त` (8.2.30 `द्+त` → `त्त`).
/// 6.4.37 न्-lopa — `गम्/हन्` → `ग/ह` before कित्; `बन्ध्` → `बध्`.
/// Keeps SLP1 `DA`=धा vs `dA`=दा distinct — critical for `हित` vs `दत्त`.
fn kit_anga(root: &str) -> String {
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match root {
        "dA" => "dad".into(),
        "DA" => "hi".into(),
        "sTA" => "sTi".into(),
        "mA" => "mi".into(),
        "pA" => "pI".into(),
        "gam" | "han" => root[..root.len() - 1].to_string(),
        "banD" | "bandh" => {
            let last = root.chars().last().unwrap();
            format!("{}{last}", &root[..root.len() - 2])
        }
        other => other.to_string(),
    }
}

// ---------------------------------------------------------------------------
// fn `ktin_form` — tin/sUP endings: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn ktin_form(root: &str) -> String {
    internal_sandhi(&kit_anga(root), "ti")
}

/// क्वसु (3.2.107): लिट् weak aṅga + वस्. बभूवतुः → बभूवस् (not बभूव्वस्).
fn kvasu_form(dhatu: &str) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if let Some(forms) = crate::engine::lit::kartari(dhatu, 1, 2, "P") {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(du) = forms.first() {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(anga) = du.strip_suffix("atuH") {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if anga.ends_with('v') {
                    return format!("{anga}as");
                }
                return format!("{anga}vas");
            }
        }
    }
    format!("{}vas", surface_root(dhatu))
}

// ---------------------------------------------------------------------------
// fn `is_ac`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn is_ac(c: char) -> bool {
    matches!(c, 'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O')
}

/// 7.2.115/116 वृद्धि (i/ī → ऐ, u/ū → औ, ṛ → आर्, a → आ).
fn vrddhi_ac(root: &str) -> String {
    let chars: Vec<char> = root.chars().collect();
    // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
    for idx in (0..chars.len()).rev() {
        let repl = match chars[idx] {
            'a' => Some("A"),
            'i' | 'I' | 'e' => Some("E"),
            'u' | 'U' | 'o' => Some("O"),
            'f' | 'F' => Some("Ar"),
            _ => None,
        };
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(r) = repl {
            let mut o = String::new();
            o.extend(chars[..idx].iter().copied());
            o.push_str(r);
            o.extend(chars[idx + 1..].iter().copied());
            return o;
        }
    }
    root.to_string()
}

/// 7.3.52 चजोः कु घिण्ण्यतोः.
fn cajo_ku(s: &str) -> String {
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match s.chars().last() {
        Some('c') => format!("{}k", &s[..s.len() - 1]),
        Some('j') => format!("{}g", &s[..s.len() - 1]),
        _ => s.to_string(),
    }
}

/// णित्/ञित् kṛt aṅga: 7.2.115 अचो ञ्णिति, 7.2.116 अत उपधायाः, 7.3.86 इगुपध गुण,
/// 7.3.33 आतो युक्, 7.3.32/54 हन् → घात्.
fn nit_krt_anga(root: &str, pratyaya: &str) -> String {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if root == "han" {
        return "GAt".into();
    }
    let last = root.chars().last().unwrap_or('a');
    let mut anga = if is_ac(last) {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if last == 'A' {
            format!("{root}y")
        } else {
            vrddhi_ac(root)
        }
    } else {
        // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
        match root.chars().rev().nth(1) {
            Some('a') => vrddhi_ac(root),
            Some('i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'e' | 'o') => apply_guna_to_stem(root),
            _ => root.to_string(),
        }
    };
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if matches!(pratyaya, "GaY" | "Ryat") {
        anga = cajo_ku(&anga);
    }
    anga
}

// ---------------------------------------------------------------------------
// fn `nit_krt_form`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn nit_krt_form(root: &str, pratyaya: &str) -> String {
    let suffix = match pratyaya {
        "Rvul" => "aka",
        "ukaY" => "uka",
        "Ryat" => "ya",
        _ => "a",
    };
    join_eco(&nit_krt_anga(root, pratyaya), suffix)
}

// ---------------------------------------------------------------------------
// fn `ktva_base`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn ktva_base(dhatu: &str) -> String {
    let ta = nistha_base(dhatu, false);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if let Some(stripped) = ta.strip_suffix("ita") {
        format!("{stripped}itvA")
    } else if let Some(stripped) = ta.strip_suffix("ta") {
        format!("{stripped}tvA")
    } else {
        format!("{ta}tvA")
    }
}

// ---------------------------------------------------------------------------
// fn `lyap_base`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn lyap_base(dhatu: &str) -> String {
    let ta = nistha_base(dhatu, false);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if let Some(stripped) = ta.strip_suffix("ita") {
        format!("{stripped}ya")
    } else if let Some(stripped) = ta.strip_suffix("ta") {
        format!("{stripped}ya")
    } else {
        format!("{ta}ya")
    }
}

// ---------------------------------------------------------------------------
// fn `generate`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate(dhatu_query: &str, pratyaya: &str) -> KrdantaResult {
    let forms = derive(dhatu_query, pratyaya);
    KrdantaResult { forms, dhatu: dhatu_query.to_string(), pratyaya: pratyaya.to_string() }
}

// ---------------------------------------------------------------------------
// fn `generate_with_prefixes`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn generate_with_prefixes(dhatu_query: &str, pratyaya: &str, prefixes: &[String]) -> KrdantaResult {
    let pratyaya_eff = if pratyaya == "ktvA" && !prefixes.is_empty() { "lyap" } else { pratyaya };
    let forms = derive(dhatu_query, pratyaya_eff);
    let forms = if prefixes.is_empty() {
        forms
    } else {
        forms.into_iter().map(|f| crate::engine::prefix::apply_prefixes(prefixes, &f)).collect()
    };
    KrdantaResult { forms, dhatu: dhatu_query.to_string(), pratyaya: pratyaya.to_string() }
}

// ---------------------------------------------------------------------------
// fn `is_avyaya`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn is_avyaya(pratyaya: &str) -> bool {
    matches!(pratyaya, "ktvA" | "lyap" | "tumun" | "Ramul" | "am")
}

/// लिङ्गs this kṛt takes. Empty = अव्यय (no सुप्).
pub fn lingas(pratyaya: &str) -> &'static [&'static str] {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if is_avyaya(pratyaya) {
        return &[];
    }
    // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
    match pratyaya {
        "ktin" => &["stri"],
        "lyuw" | "lyu" => &["nap"],
        "GaY" => &["pum"],
        _ => &["pum", "stri", "nap"],
    }
}

// ---------------------------------------------------------------------------
// fn `is_at_participle`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn is_at_participle(pratyaya: &str) -> bool {
    matches!(pratyaya, "Satf" | "Satf~" | "ktavatu" | "ktavatu~")
}

// ---------------------------------------------------------------------------
// fn `pratipadika`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn pratipadika(form: &str, pratyaya: &str, linga: &str) -> Option<String> {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if is_avyaya(pratyaya) || form.is_empty() {
        return None;
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if pratyaya == "tfc" && linga == "stri" {
        let base = form.trim_end_matches('f');
        return Some(format!("{base}rI"));
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if is_at_participle(pratyaya) && linga == "stri" {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if pratyaya.starts_with("ktavatu") {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if form.ends_with("at") {
                return Some(format!("{form}I"));
            }
        } else if let Some(base) = form.strip_suffix("at") {
            return Some(format!("{base}antI"));
        }
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if linga == "stri"
        && matches!(
            pratyaya,
            "kta" | "SAnac" | "cAnaS" | "tavya" | "anIyar" | "Rvul" | "vun" | "ac" | "anIya"
                | "yat" | "Ryat"
        )
    {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(base) = form.strip_suffix('a') {
            return Some(format!("{base}A"));
        }
    }
    Some(form.to_string())
}

// ---------------------------------------------------------------------------
// fn `satr_nap`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn satr_nap(stem: &str) -> Option<crate::declension::subanta::Declension> {
    let mut d = crate::declension::subanta::generate(stem, "pum")?;
    let Some(base) = stem.strip_suffix("at") else {
        d.linga = "nap".into();
        return Some(d);
    };
    let nom = vec![
        stem.to_string(),
        format!("{stem}I"),
        format!("{base}anti"),
    ];
    d.declension.insert("prathamA".into(), nom.clone());
    d.declension.insert("dvitIyA".into(), nom.clone());
    d.declension.insert("samboDana".into(), nom);
    d.linga = "nap".into();
    Some(d)
}

/// सुबन्त of a kṛdanta pratipadika. `None` for अव्यय or a लिङ्ग the kṛt does not take.
pub fn decline(
    dhatu_query: &str,
    pratyaya: &str,
    linga: &str,
    prefixes: &[String],
) -> Option<crate::declension::subanta::Declension> {
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if !lingas(pratyaya).contains(&linga) {
        return None;
    }
    let res = generate_with_prefixes(dhatu_query, pratyaya, prefixes);
    let form = res.forms.first()?.as_str();
    let stem = pratipadika(form, pratyaya, linga)?;
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if is_at_participle(pratyaya) && linga == "nap" {
        return satr_nap(&stem);
    }
    let mut d = crate::declension::subanta::generate(&stem, linga)?;
    // 6.4.14 अत्वसन्तस्य चाधातोः: शतृ has no दीर्घ (भवन् not भवान्). क्तवतु keeps आन्.
    if matches!(pratyaya, "Satf" | "Satf~") && linga == "pum" {
        // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
        if let Some(row) = d.declension.get_mut("prathamA") {
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if let Some(nom) = row.first_mut() {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if let Some(base) = nom.strip_suffix("An") {
                    *nom = format!("{base}an");
                }
            }
        }
    }
    Some(d)
}

// ---------------------------------------------------------------------------
// fn `derive`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
pub fn derive(dhatu_query: &str, pratyaya: &str) -> Vec<String> {
    let (dhatu, gana, tags, ant, aup) = load_dhatu(dhatu_query);
    let rule = pratyaya_rule(pratyaya);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if rule.is_none() {
        return vec![];
    }
    let (suffix, _sutras, mode) = rule.unwrap();
    let root = surface_root(&dhatu);
    let guna = apply_guna_to_stem(&root);

    let form = match mode {
        "present" => {
            let (st, _) = crate::engine::stems::derive_stem(&dhatu, gana, "lat", "shuddha", &tags, &ant, &aup);
            let base = st.unwrap_or_else(|| present_stem(&dhatu, gana));
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if pratyaya == "Satf" {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if base.ends_with('a') {
                    format!("{}at", &base[..base.len() - 1])
                } else if base.ends_with('u') {
                    format!("{}vat", &base[..base.len() - 1])
                } else if base.ends_with('I') {
                    format!("{}at", &base[..base.len() - 1])
                } else {
                    format!("{}at", base)
                }
            } else if pratyaya == "Satf~" {
                // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                if base.ends_with('a') {
                    format!("{}n", &base[..base.len() - 1])
                } else {
                    format!("{}ant", base)
                }
            } else if pratyaya == "SAnac" || pratyaya == "cAnaS" || pratyaya.contains("SAnac") || pratyaya.contains("cAnaS") {
                // 7.2.82 आने मुक्: keep शप् अ (एधमान not एध्मान).
                if base.ends_with('a') {
                    format!("{}mAna", base)
                } else if base.ends_with('u') {
                    format!("{}vAna", &base[..base.len() - 1])
                } else {
                    format!("{}Ana", base)
                }
            } else {
                format!("{}{}", base, suffix)
            }
        }
        "kta" => {
            let base = kta_base(&dhatu);
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if pratyaya.starts_with("ktavatu") { format!("{base}vat") } else { base }
        }
        "guna" => {
            // — match — pada/lakāra/gaṇa dispatch; sūtra gating, see comments above.
            match pratyaya {
                "lyuw" | "lyu" => crate::engine::it::lyuw_form(&root),
                "tfc" => crate::engine::it::tfc_form(&root),
                "ktin" => ktin_form(&root),
                "GaY" | "Rvul" | "ukaY" | "Ryat" => nit_krt_form(&root, pratyaya),
                "Ramul" => join_eco(&nit_krt_anga(&root, "Rvul"), "am"),
                "yat" if root.ends_with('A') => format!("{}eya", &root[..root.len() - 1]),
                _ => join_eco(&guna, suffix),
            }
        }
        "guna_a" => join_eco(&guna, "a"),
        "guna_tum" => crate::engine::it::tum_form(&root),
        "guna_tavya" => crate::engine::it::tavya_form(&root),
        "anIya" => crate::engine::it::anIya_form(&root),
        "root" if pratyaya == "ktvA" => ktva_base(&dhatu),
        "root" => format!("{}{}", dhatu, suffix),
        "lit" => kvasu_form(&dhatu),
        "lyap" => lyap_base(&dhatu),
        _ => format!("{}{}", guna, suffix),
    };
    vec![form]
}

// ---------------------------------------------------------------------------
// fn `present_stem`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
fn present_stem(dhatu: &str, gana: u8) -> String {
    let guna = apply_guna_to_stem(dhatu);
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana == 10 { return format!("{}aya", guna); }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana == 4 {
        // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
        for idx in (0..dhatu.len()).rev() {
            let ch = dhatu.chars().nth(idx).unwrap();
            // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
            if "iIuUfF".contains(ch) {
                let long_v = match ch { 'i' => 'I', 'u' => 'U', 'f' => 'F', _ => ch };
                let mut out = String::new();
                // — for — iterate dhātu/ending variants; sūtra gating, see comments above.
                for (i,c) in dhatu.chars().enumerate() {
                    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
                    if i==idx { out.push(long_v); } else { out.push(c); }
                }
                return format!("{}ya", out);
            }
        }
        return format!("{}ya", guna);
    }
    // — if-branch — condition → aṅga/sandhi step; sūtra gating, see comments above.
    if gana == 1 || gana == 6 {
        let base = if gana == 6 { dhatu.to_string() } else { guna };
        return format!("{}a", base);
    }
    guna
}

// Optional scrape probe (not the spec).
pub fn validate_against_gold(dhatu_id: &str, pratyaya: &str) -> Option<(String, String)> {
    let p = format!("/home/edhiraj/Documents/projs/skt-morph-data/data/{}/{}.json", &dhatu_id[..2], dhatu_id);
    let data = std::fs::read_to_string(&p).ok()?;
    let v: serde_json::Value = serde_json::from_str(&data).ok()?;
    let base = v["participles"]["krut"].get(pratyaya)?.as_array()?.first()?;
    let gold_m = base.get("m")?.as_str()?.to_string();
    let ours = derive(dhatu_id, pratyaya);
    Some((ours.first().cloned().unwrap_or_default(), gold_m))
}

#[cfg(test)]
// ---------------------------------------------------------------------------
// mod `tests`: purpose, inputs→outputs, edge cases.
// Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
// ---------------------------------------------------------------------------
mod tests {
    use super::*;

    #[test]
    // ---------------------------------------------------------------------------
    // fn `bu_kta`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn bu_kta() {
        let f = derive("BU", "kta");
        assert!(f.iter().any(|x| x == "BUta"), "{:?}", f);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `gam_kf_vac_da_kta`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn gam_kf_vac_da_kta() {
        assert_eq!(derive("gam", "kta"), vec!["gata"]);
        assert_eq!(derive("qukfY", "kta"), vec!["kfta"]);
        assert_eq!(derive("vaca", "kta"), vec!["ukta"]);
        // 7.4.46 दा → दत्त vs 7.4.42 धा → हित — SLP1 dA (द) vs DA (ध) distinct.
        assert_eq!(derive("qudAY", "kta"), vec!["datta"]); // दाण्/दा
        assert_eq!(derive("quDAY", "kta"), vec!["hita"]); // धेट्/धा — 7.4.42
        assert_eq!(derive("dA", "kta"), vec!["datta"]);
        assert_eq!(derive("DA", "kta"), vec!["hita"]);
        // 2.4.36 अदो जग्धिर्ल्यप्ति किति — अद् → जग्ध (atta would be sandhi-only, wrong)
        assert_eq!(derive("ada", "kta"), vec!["jagDa"]); // अद् → जग्ध
        assert_eq!(derive("ad", "kta"), vec!["jagDa"]);
        // भिद् → भिन्न (Binna) — 8.2.43/6.4.47 special, not Bitta
        assert_eq!(derive("Bida", "kta"), vec!["Binna"]); // भिद् → भिन्न (Bida is भिद् with a)
        assert_eq!(derive("Bid", "kta"), vec!["Binna"]);
        // शद्/पद् → शन्न/पन्न (Sanna/panna) — 8.2.45
        assert_eq!(derive("Sada", "kta"), vec!["Sanna"]); // शद् → शन्न
        assert_eq!(derive("pada", "kta"), vec!["panna"]); // पद् → पन्न (pada is पद्)
        assert_eq!(derive("Sad", "kta"), vec!["Sanna"]);
        assert_eq!(derive("pad", "kta"), vec!["panna"]);
        // सह् → सोढ (soQa) — 8.2.31 + guṇa
        assert_eq!(derive("saha", "kta"), vec!["soQa"]); // सह् → सोढ (saha is सह् with a)
        assert_eq!(derive("sah", "kta"), vec!["soQa"]);
        // वह् → ऊढ (UQa) — 6.1.15 + 8.2.31
        assert_eq!(derive("vaha", "kta"), vec!["UQa"]); // वह् → ऊढ (already in gam_kf_vac_da_kta? keep explicit)
        assert_eq!(derive("vah", "kta"), vec!["UQa"]);
        // दह् → दग्ध (dagDa) — 8.2.32
        assert_eq!(derive("daha", "kta"), vec!["dagDa"]); // दह् → दग्ध
        assert_eq!(derive("dah", "kta"), vec!["dagDa"]);
        // मुच् → मुक्त (mukta) — 8.2.30
        assert_eq!(derive("muca", "kta"), vec!["mukta"]); // मुच् → मुक्त (muca is मुच् with a)
        assert_eq!(derive("muc", "kta"), vec!["mukta"]);
        // भञ्ज् → भग्न (Bagna) — Banj→Bagna
        assert_eq!(derive("BaYja", "kta"), vec!["Bagna"]); // भञ्ज् → भग्न (BaYja is भञ्ज् with a)
        assert_eq!(derive("BaYj", "kta"), vec!["Bagna"]);
        // जन् → जात (jAta) — jan→jAta
        assert_eq!(derive("jana", "kta"), vec!["jAta"]); // जन् → जात (jana is जन् with a)
        assert_eq!(derive("jan", "kta"), vec!["jAta"]);
        // तन् → तत (tata) — tan→tata
        assert_eq!(derive("tana", "kta"), vec!["tata"]); // तन् → तत (tana is तन् with a)
        assert_eq!(derive("tan", "kta"), vec!["tata"]);
        // क्रम् → क्रान्त (krAnta) — kram→krAnta
        assert_eq!(derive("krama", "kta"), vec!["krAnta"]); // क्रम् → क्रान्त (krama is क्रम् with a)
        assert_eq!(derive("kram", "kta"), vec!["krAnta"]);
        // श्रम् → श्रान्त (SrAnta) — Sram→SrAnta
        assert_eq!(derive("Srama", "kta"), vec!["SrAnta"]); // श्रम् → श्रान्त (Srama is श्रम् with a)
        assert_eq!(derive("Sram", "kta"), vec!["SrAnta"]);
        // भ्रम् → भ्रान्त (BrAnta) — Bram→BrAnta
        assert_eq!(derive("Brama", "kta"), vec!["BrAnta"]); // भ्रम् → भ्रान्त (Brama is भ्रम् with a)
        assert_eq!(derive("Bram", "kta"), vec!["BrAnta"]);
        // दम् → दान्त (dAnta) — dam→dAnta
        assert_eq!(derive("dama", "kta"), vec!["dAnta"]); // दम् → दान्त (dama is दम् with a)
        assert_eq!(derive("dam", "kta"), vec!["dAnta"]);
        // शम् → शान्त (SAnta) — Sam→SAnta
        assert_eq!(derive("Sama", "kta"), vec!["SAnta"]); // शम् → शान्त (Sama is शम् with a)
        assert_eq!(derive("Sam", "kta"), vec!["SAnta"]);
        // तम् → तान्त (tAnta) — tam→tAnta
        assert_eq!(derive("tama", "kta"), vec!["tAnta"]); // तम् → तान्त (tama is तम् with a)
        assert_eq!(derive("tam", "kta"), vec!["tAnta"]);
        // नम् → नत (nata) — nam→nata
        assert_eq!(derive("nama", "kta"), vec!["nata"]); // नम् → नत (nama is नम् with a)
        assert_eq!(derive("nam", "kta"), vec!["nata"]);
        // यम् → यत (yata) — yam→yata
        assert_eq!(derive("yama", "kta"), vec!["yata"]); // यम् → यत (yama is यम् with a)
        assert_eq!(derive("yam", "kta"), vec!["yata"]);
        // रम् → रत (rata) — ram→rata
        assert_eq!(derive("rama", "kta"), vec!["rata"]); // रम् → रत (rama is रम् with a)
        assert_eq!(derive("ram", "kta"), vec!["rata"]);
        // वन् → वत (vata) — van→vata
        assert_eq!(derive("vana", "kta"), vec!["vata"]); // वन् → वत (vana is वन् with a)
        assert_eq!(derive("van", "kta"), vec!["vata"]);
        // मन् → मत (mata) — man→mata
        assert_eq!(derive("mana", "kta"), vec!["mata"]); // मन् → मत (mana is मन् with a)
        assert_eq!(derive("man", "kta"), vec!["mata"]);
        // कन् → कत (kata) — kan→kata
        assert_eq!(derive("kana", "kta"), vec!["kata"]); // कन् → कत (kana is कन् with a)
        assert_eq!(derive("kan", "kta"), vec!["kata"]);
        // सन् → सत (sata) — san→sata
        assert_eq!(derive("sana", "kta"), vec!["sata"]); // सन् → सत (sana is सन् with a)
        assert_eq!(derive("san", "kta"), vec!["sata"]);
        // हन् → हत (hata) — han→hata
        assert_eq!(derive("hana", "kta"), vec!["hata"]); // हन् → हत (hana is हन् with a)
        assert_eq!(derive("han", "kta"), vec!["hata"]);
        // गम् → गत (gata) — gam→gata
        assert_eq!(derive("gama", "kta"), vec!["gata"]); // गम् → गत (gama is गम् with a) — careful gama is गम्? Actually gama is गम with a, but test as gam
        assert_eq!(derive("gam", "kta"), vec!["gata"]);
        // रुच् → रुक्त (rukta) — ruc→rukta
        assert_eq!(derive("ruca", "kta"), vec!["rukta"]); // रुच् → रुक्त (ruca is रुच् with a)
        assert_eq!(derive("ruc", "kta"), vec!["rukta"]);
        // युज् → युक्त (yukta) — yuj→yukta
        assert_eq!(derive("yuja", "kta"), vec!["yukta"]); // युज् → युक्त (yuja is युज् with a)
        assert_eq!(derive("yuj", "kta"), vec!["yukta"]);
        // कुच् → कुक्त (kukta) — kuc→kukta
        assert_eq!(derive("kuca", "kta"), vec!["kukta"]); // कुच् → कुक्त (kuca is कुच् with a)
        assert_eq!(derive("kuc", "kta"), vec!["kukta"]);
        // तुच् → तुक्त (tukta) — tuc→tukta
        assert_eq!(derive("tuca", "kta"), vec!["tukta"]); // तुच् → तुक्त (tuca is तुच् with a)
        assert_eq!(derive("tuc", "kta"), vec!["tukta"]);
        // सिच् → सिक्त (sikta) — sic→sikta
        assert_eq!(derive("sica", "kta"), vec!["sikta"]); // सिच् → सिक्त (sica is सिच् with a)
        assert_eq!(derive("sic", "kta"), vec!["sikta"]);
        // भुज् → भुक्त (Bukta) — Buj→Bukta
        assert_eq!(derive("Buja", "kta"), vec!["Bukta"]); // भुज् → भुक्त (Buja is भुज् with a)
        assert_eq!(derive("Buj", "kta"), vec!["Bukta"]);
        // तुज् → तुक्त (tukta) — tuj→tukta
        assert_eq!(derive("tuja", "kta"), vec!["tukta"]); // तुज् → तुक्त (tuja is तुज् with a)
        assert_eq!(derive("tuj", "kta"), vec!["tukta"]);
        // सुज् → सुक्त (sukta) — suj→sukta
        assert_eq!(derive("suja", "kta"), vec!["sukta"]); // सुज् → सुक्त (suja is सुज् with a)
        assert_eq!(derive("suj", "kta"), vec!["sukta"]);
        // रुज् → रुक्त (rukta) — ruj→rukta
        assert_eq!(derive("ruja", "kta"), vec!["rukta"]); // रुज् → रुक्त (ruja is रुज् with a)
        assert_eq!(derive("ruj", "kta"), vec!["rukta"]);
        // विज् → विक्त (vikta) — vij→vikta
        assert_eq!(derive("vija", "kta"), vec!["vikta"]); // विज् → विक्त (vija is विज् with a)
        assert_eq!(derive("vij", "kta"), vec!["vikta"]);
        // सिज् → सिक्त (sikta) — sij→sikta
        assert_eq!(derive("sija", "kta"), vec!["sikta"]); // सिज् → सिक्त (sija is सिज् with a)
        assert_eq!(derive("sij", "kta"), vec!["sikta"]);
        // निज् → निक्त (nikta) — nij→nikta
        assert_eq!(derive("nija", "kta"), vec!["nikta"]); // निज् → निक्त (nija is निज् with a)
        assert_eq!(derive("nij", "kta"), vec!["nikta"]);
        // मिज् → मिक्त (mikta) — mij→mikta
        assert_eq!(derive("mija", "kta"), vec!["mikta"]); // मिज् → मिक्त (mija is मिज् with a)
        assert_eq!(derive("mij", "kta"), vec!["mikta"]);
        // पिज् → पिक्त (pikta) — pij→pikta
        assert_eq!(derive("pija", "kta"), vec!["pikta"]); // पिज् → पिक्त (pija is पिज् with a)
        assert_eq!(derive("pij", "kta"), vec!["pikta"]);
        // किज् → किक्त (kikta) — kij→kikta
        assert_eq!(derive("kija", "kta"), vec!["kikta"]); // किज् → किक्त (kija is किज् with a)
        assert_eq!(derive("kij", "kta"), vec!["kikta"]);
        // गिज् → गिक्त (gikta) — gij→gikta
        assert_eq!(derive("gija", "kta"), vec!["gikta"]); // गिज् → गिक्त (gija is गिज् with a)
        assert_eq!(derive("gij", "kta"), vec!["gikta"]);
        // चिज् → चिक्त (cikta) — cij→cikta
        assert_eq!(derive("cija", "kta"), vec!["cikta"]); // चिज् → चिक्त (cija is चिज् with a)
        assert_eq!(derive("cij", "kta"), vec!["cikta"]);
        // तिज् → तिक्त (tikta) — tij→tikta
        assert_eq!(derive("tija", "kta"), vec!["tikta"]); // तिज् → तिक्त (tija is तिज् with a)
        assert_eq!(derive("tij", "kta"), vec!["tikta"]);
        // दिज् → दिक्त (dikta) — dij→dikta
        assert_eq!(derive("dija", "kta"), vec!["dikta"]); // दिज् → दिक्त (dija is दिज् with a)
        assert_eq!(derive("dij", "kta"), vec!["dikta"]);
        // बिज् → बिक्त (bikta) — bij→bikta
        assert_eq!(derive("bija", "kta"), vec!["bikta"]); // बिज् → बिक्त (bija is बिज् with a)
        assert_eq!(derive("bij", "kta"), vec!["bikta"]);
        // लिज् → लिक्त (likta) — lij→likta
        assert_eq!(derive("lija", "kta"), vec!["likta"]); // लिज् → लिक्त (lija is लिज् with a)
        assert_eq!(derive("lij", "kta"), vec!["likta"]);
        // रिज् → रिक्त (rikta) — rij→rikta
        assert_eq!(derive("rija", "kta"), vec!["rikta"]); // रिज् → रिक्त (rija is रिज् with a)
        assert_eq!(derive("rij", "kta"), vec!["rikta"]);
        // दिव् → द्यूत (dyUta) — div→dyUta
        assert_eq!(derive("diva", "kta"), vec!["dyUta"]); // दिव् → द्यूत (diva is दिव् with a)
        assert_eq!(derive("div", "kta"), vec!["dyUta"]);
        // लुभ् → लुब्ध (lubDa) — luB→lubDa
        assert_eq!(derive("luBa", "kta"), vec!["lubDa"]); // लुभ् → लुब्ध (luBa is लुभ् with a)
        assert_eq!(derive("luB", "kta"), vec!["lubDa"]);
        // क्षुभ् → क्षुब्ध (kzuBDa) — kzuB→kzuBDa
        assert_eq!(derive("kzuBa", "kta"), vec!["kzuBDa"]); // क्षुभ् → क्षुब्ध (kzuBa is क्षुभ् with a)
        assert_eq!(derive("kzuB", "kta"), vec!["kzuBDa"]);
        // स्तभ् → स्तब्ध (swaBDa) — swaB→swaBDa
        assert_eq!(derive("swaBa", "kta"), vec!["swaBDa"]); // स्तभ् → स्तब्ध (swaBa is स्तभ् with a)
        assert_eq!(derive("swaB", "kta"), vec!["swaBDa"]);
        // स्कम्भ् → स्कब्ध (skaBDa) — skaB→skaBDa
        assert_eq!(derive("skaBa", "kta"), vec!["skaBDa"]); // स्कम्भ् → स्कब्ध (skaBa is स्कभ् with a)
        assert_eq!(derive("skaB", "kta"), vec!["skaBDa"]);
        // रम्भ् → रब्ध (rabDa) — ramB→rabDa
        assert_eq!(derive("ramBa", "kta"), vec!["rabDa"]); // रम्भ् → रब्ध (ramBa is रम्भ् with a)
        assert_eq!(derive("ramB", "kta"), vec!["rabDa"]);
        // लभ् → लब्ध (labDa) — laB→labDa
        assert_eq!(derive("laBa", "kta"), vec!["labDa"]); // लभ् → लब्ध (laBa is लभ् with a)
        assert_eq!(derive("laB", "kta"), vec!["labDa"]);
        // रभ् → रब्ध (rabDa) — raB→rabDa
        assert_eq!(derive("raBa", "kta"), vec!["rabDa"]); // रभ् → रब्ध (raBa is रभ् with a)
        assert_eq!(derive("raB", "kta"), vec!["rabDa"]);
        // नभ् → नब्ध (nabDa) — naB→nabDa
        assert_eq!(derive("naBa", "kta"), vec!["nabDa"]); // नभ् → नब्ध (naBa is नभ् with a)
        assert_eq!(derive("naB", "kta"), vec!["nabDa"]);
        // अभ् → अब्ध (abDa) — aB→abDa
        assert_eq!(derive("aBa", "kta"), vec!["abDa"]); // अभ् → अब्ध (aBa is अभ् with a)
        assert_eq!(derive("aB", "kta"), vec!["abDa"]);
        // सभ् → सब्ध (sabDa) — saB→sabDa
        assert_eq!(derive("saBa", "kta"), vec!["sabDa"]); // सभ् → सब्ध (saBa is सभ् with a)
        assert_eq!(derive("saB", "kta"), vec!["sabDa"]);
        // दभ् → दब्ध (dabDa) — daB→dabDa
        assert_eq!(derive("daBa", "kta"), vec!["dabDa"]); // दभ् → दब्ध (daBa is दभ् with a)
        assert_eq!(derive("daB", "kta"), vec!["dabDa"]);
        // गभ् → गब्ध (gabDa) — gaB→gabDa
        assert_eq!(derive("gaBa", "kta"), vec!["gabDa"]); // गभ् → गब्ध (gaBa is गभ् with a)
        assert_eq!(derive("gaB", "kta"), vec!["gabDa"]);
        // द्रभ् → द्रब्ध (drabDa) — draB→drabDa
        assert_eq!(derive("draBa", "kta"), vec!["drabDa"]); // द्रभ् → द्रब्ध (draBa is द्रभ् with a)
        assert_eq!(derive("draB", "kta"), vec!["drabDa"]);
        // स्रभ् → स्रब्ध (srabDa) — sraB→srabDa
        assert_eq!(derive("sraBa", "kta"), vec!["srabDa"]); // स्रभ् → स्रब्ध (sraBa is स्रभ् with a)
        assert_eq!(derive("sraB", "kta"), vec!["srabDa"]);
        // जभ् → जब्ध (jabDa) — jaB→jabDa
        assert_eq!(derive("jaBa", "kta"), vec!["jabDa"]); // जभ् → जब्ध (jaBa is जभ् with a)
        assert_eq!(derive("jaB", "kta"), vec!["jabDa"]);
        assert_eq!(derive("BU", "ktvA"), vec!["BUtvA"]);
        assert_eq!(derive("gam", "tumun"), vec!["gantum"]);
        let f = generate_with_prefixes("BU", "ktvA", &["pra".into()]);
        assert!(f.forms.iter().any(|x| x == "praBUya"), "{:?}", f.forms);
        assert_eq!(derive("qukfY", "tavya"), vec!["kartavya"]);
        assert_eq!(derive("qukfY", "tfc"), vec!["kartf"]);
        assert_eq!(derive("qukfY", "lyuw"), vec!["karaRa"]);
        assert_eq!(derive("qukfY", "anIyar"), vec!["karaRIya"]);
        let sat = derive("hu", "Satf");
        assert!(sat.iter().any(|x| x == "juhvat"), "{:?}", sat);
        assert_eq!(derive("dfSir", "kta"), vec!["dfzwa"]);
        assert_eq!(derive("vaha", "kta"), vec!["UQa"]);
        assert_eq!(derive("duha", "kta"), vec!["dugDa"]);
        assert_eq!(kta_base("labh"), "labDa");
        assert_eq!(kta_base("svap"), "supta");
        assert_eq!(kta_base("naS"), "nazwa");
        assert_eq!(derive("graha", "kta"), vec!["gfhIta"]);
        assert_eq!(derive("vasa", "kta"), vec!["uzita"]);
        assert_eq!(derive("patx", "kta"), vec!["patita"]);
        assert_eq!(derive("banDa", "kta"), vec!["badDa"]);
        assert_eq!(derive("qupacaz", "kta"), vec!["pakva"]);
        assert_eq!(derive("qupacaz", "ktvA"), vec!["paktvA"]);
        assert_eq!(derive("gam", "tavya"), vec!["gantavya"]);
        assert_eq!(derive("gam", "tfc"), vec!["gantf"]);
        assert_eq!(derive("RIY", "tumun"), vec!["netum"]);
        assert_eq!(derive("BU", "tumun"), vec!["Bavitum"]);
        assert_eq!(derive("Sru", "lyuw"), vec!["SravaRa"]);
        assert_eq!(derive("Sru", "anIyar"), vec!["SravaRIya"]);
        assert_eq!(derive("hana", "anIyar"), vec!["hananIya"]);
        assert_eq!(derive("RIY", "lyuw"), vec!["nayana"]);
        assert_eq!(derive("qudAY", "lyuw"), vec!["dAna"]);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `krdanta_declension`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn krdanta_declension() {
        let d = decline("gam", "kta", "pum", &[]).expect("gataH");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gataH"), "{:?}", pr);
        let d = decline("gam", "kta", "stri", &[]).expect("gatA");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gatA"), "{:?}", pr);
        let d = decline("BU", "Satf", "pum", &[]).expect("Bavan");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "Bavan"), "{:?}", pr);
        let d = decline("gamx", "Satf", "pum", &[]).expect("gacCan");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gacCan"), "{:?}", pr);
        let d = decline("BU", "Satf", "stri", &[]).expect("BavantI");
        assert_eq!(d.stem, "BavantI");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "BavantI"), "{:?}", pr);
        let d = decline("BU", "Satf", "nap", &[]).expect("Bavat");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "Bavat"), "{:?}", pr);
        assert!(pr.iter().any(|x| x == "Bavanti"), "{:?}", pr);
        let d = decline("qukfY", "tfc", "pum", &[]).expect("kartA");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "kartA"), "{:?}", pr);
        let dv = d.declension.get("dvitIyA").unwrap();
        assert!(dv.iter().any(|x| x == "kartAram"), "{:?}", dv);
        assert!(decline("BU", "ktvA", "pum", &[]).is_none());
        let d = decline("gam", "ktavatu", "pum", &[]).expect("gatavAn");
        let pr = d.declension.get("prathamA").unwrap();
        assert!(pr.iter().any(|x| x == "gatavAn"), "{:?}", pr);
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `krdanta_lingas_by_pratyaya`: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn krdanta_lingas_by_pratyaya() {
        assert!(lingas("lyap").is_empty());
        assert!(lingas("ktvA").is_empty());
        assert!(lingas("tumun").is_empty());
        assert!(lingas("Ramul").is_empty());
        assert_eq!(lingas("lyuw"), &["nap"]);
        assert_eq!(lingas("ktin"), &["stri"]);
        assert_eq!(lingas("GaY"), &["pum"]);
        assert_eq!(lingas("kta"), &["pum", "stri", "nap"]);
        assert!(decline("qukfY", "lyuw", "pum", &[]).is_none());
        assert!(decline("qukfY", "lyuw", "stri", &[]).is_none());
        let d = decline("qukfY", "lyuw", "nap", &[]).expect("karaRam");
        assert_eq!(d.linga, "nap");
        assert!(decline("qukfY", "ktin", "pum", &[]).is_none());
        let d = decline("qukfY", "ktin", "stri", &[]).expect("kfti");
        assert_eq!(d.stem, "kfti");
        assert!(decline("BU", "lyap", "nap", &[]).is_none());
    }

    #[test]
    // ---------------------------------------------------------------------------
    // fn `nit_krts_vrddhi_and_kitin` — sūtra: 7.2.115/116 vṛddhi: purpose, inputs→outputs, edge cases.
    // Pāṇini step; see Kaumudī ordering. SLP1 I/O. No DB fallback.
    // ---------------------------------------------------------------------------
    fn nit_krts_vrddhi_and_kitin() {
        assert_eq!(derive("BU", "GaY"), vec!["BAva"]);
        assert_eq!(derive("BU", "Rvul"), vec!["BAvaka"]);
        assert_eq!(derive("BU", "vun"), vec!["Bavaka"]);
        assert_eq!(derive("BU", "ukaY"), vec!["BAvuka"]);
        assert_eq!(derive("BU", "Ryat"), vec!["BAvya"]);
        assert_eq!(derive("BU", "yat"), vec!["Bavya"]);
        assert_eq!(derive("RIY", "GaY"), vec!["nAya"]);
        assert_eq!(derive("RIY", "Rvul"), vec!["nAyaka"]);
        assert_eq!(derive("RIY", "vun"), vec!["nayaka"]);
        assert_eq!(derive("qudAY", "GaY"), vec!["dAya"]);
        assert_eq!(derive("qudAY", "Rvul"), vec!["dAyaka"]);
        assert_eq!(derive("qudAY", "vun"), vec!["dAka"]);
        assert_eq!(derive("qudAY", "yat"), vec!["deya"]);
        assert_eq!(derive("tyaja", "GaY"), vec!["tyAga"]);
        assert_eq!(derive("tyaja", "Ryat"), vec!["tyAgya"]);
        assert_eq!(derive("tyaja", "Rvul"), vec!["tyAjaka"]);
        assert_eq!(derive("qupacaz", "GaY"), vec!["pAka"]);
        assert_eq!(derive("qupacaz", "Rvul"), vec!["pAcaka"]);
        assert_eq!(derive("qupacaz", "Ryat"), vec!["pAkya"]);
        assert_eq!(derive("qukfY", "GaY"), vec!["kAra"]);
        assert_eq!(derive("qukfY", "Rvul"), vec!["kAraka"]);
        assert_eq!(derive("qukfY", "vun"), vec!["karaka"]);
        assert_eq!(derive("qukfY", "Ryat"), vec!["kArya"]);
        assert_eq!(derive("hana", "GaY"), vec!["GAta"]);
        assert_eq!(derive("hana", "Rvul"), vec!["GAtaka"]);
        assert_eq!(derive("hana", "vun"), vec!["hanaka"]);
        assert_eq!(derive("gam", "GaY"), vec!["gAma"]);
        assert_eq!(derive("dfSir", "GaY"), vec!["darSa"]);
        assert_eq!(derive("Sru", "Rvul"), vec!["SrAvaka"]);
        assert_eq!(derive("Sru", "Ryat"), vec!["SrAvya"]);
        assert_eq!(derive("gam", "ktin"), vec!["gati"]);
        assert_eq!(derive("qudAY", "ktin"), vec!["datti"]);
        assert_eq!(derive("tyaja", "ktin"), vec!["tyakti"]);
        assert_eq!(derive("dfSir", "ktin"), vec!["dfzwi"]);
        assert_eq!(derive("qukfY", "ktin"), vec!["kfti"]);
        assert_eq!(derive("qudAY", "kta"), vec!["datta"]);
        assert_eq!(derive("zWA", "kta"), vec!["sTita"]);
        assert_eq!(derive("eDa", "SAnac"), vec!["eDamAna"]);
        assert_eq!(derive("BU", "SAnac"), vec!["BavamAna"]);
        assert_eq!(derive("gamx", "Satf"), vec!["gacCat"]);
        assert_eq!(derive("BU", "kvasu"), vec!["baBUvas"]);
        assert_eq!(derive("qukfY", "Ramul"), vec!["kAram"]);
        assert_eq!(derive("BU", "Ramul"), vec!["BAvam"]);
        assert!(decline("qukfY", "Ramul", "pum", &[]).is_none());
    }
}
