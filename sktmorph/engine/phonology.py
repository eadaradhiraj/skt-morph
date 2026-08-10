from typing import Optional

VOWEL_FINAL = frozenset("aeiouAIUEOfF")

GUNA_MAP = {
    "i": "e",
    "I": "e",
    "u": "o",
    "U": "av",
    "f": "ar",
    "F": "ar",
    "A": "A",
    "a": "a",
}


def ends_with_vowel(stem: str) -> bool:
    return bool(stem) and stem[-1] in VOWEL_FINAL


def apply_guna_to_stem(stem: str) -> str:
    """Guṇa on the last vowel of a dhātu/stem (7.2.115)."""
    for idx in range(len(stem) - 1, -1, -1):
        repl = GUNA_MAP.get(stem[idx])
        if repl is not None:
            return stem[:idx] + repl + stem[idx + 1 :]
    return stem


def apply_causative_grade(stem: str) -> str:
    """Vṛddhi/guṇa grade for causative (gaṇa 10) stems."""
    for idx in range(len(stem) - 1, -1, -1):
        ch = stem[idx]
        if ch not in VOWEL_FINAL:
            continue
        trailing = stem[idx + 1 :]
        if len(trailing) > 1:
            return stem
        if ch in "aA":
            return stem[:idx] + "A" + stem[idx + 1 :]
        if ch in "IUUF":
            return stem
        repl = GUNA_MAP.get(ch)
        if repl is not None:
            return stem[:idx] + repl + stem[idx + 1 :]
    return stem


def vowel_initial_lang_stem(dhatu: str) -> Optional[str]:
    """Laṅ stem when dhātu begins with a vowel (augment merges into stem)."""
    if not dhatu or dhatu[0] not in VOWEL_FINAL:
        return None
    ch = dhatu[0]
    if ch in "aA":
        return "A" + dhatu[1:]
    if ch in "iI":
        return "E" + dhatu[1:]
    if ch in "eE":
        return "E" + dhatu[1:]
    if ch in "uUoO":
        return "O" + dhatu[1:]
    return None


def bidadi_present_stem(dhatu: str) -> str:
    if dhatu.endswith(("i", "I", "u", "U", "e", "E")):
        return dhatu[:-1] + "aya"
    return dhatu + "ya"


def bidadi_lang_stem(dhatu: str) -> str:
    if dhatu.endswith(("i", "I", "u", "U", "e", "E")):
        return dhatu[:-1] + "ay"
    return dhatu + "ay"


def bidadi_vidhilin_stem(dhatu: str) -> str:
    if dhatu.endswith(("i", "I", "u", "U", "e", "E")):
        return dhatu[:-1] + "ay"
    return dhatu + "ay"


def is_bidadi(antarganas: str) -> bool:
    return "BidAdi" in (antarganas or "")


def is_yajadi(antarganas: str) -> bool:
    return "yajAdi" in (antarganas or "")


def is_gawadi(antarganas: str) -> bool:
    return "GawAdi" in (antarganas or "")


def g1_rv_nv_present_base(dhatu: str) -> Optional[str]:
    """Gaṇa-1 r…nv roots with ṛ-vṛddhi present base (rinv → riRv)."""
    if dhatu.endswith("nv") and len(dhatu) >= 4 and dhatu[0] == "r":
        return dhatu[:-2] + "Rv"
    return None


def g1_nv_present_stem(dhatu: str) -> Optional[str]:
    """Gaṇa-1 roots with n-infix present (Dinv → Dino)."""
    if dhatu in _G1_NV_ROOTS:
        return dhatu[:-2] + "no"
    return None


def g1_nv_vidhilin_stem(dhatu: str) -> Optional[str]:
    if dhatu in _G1_NV_ROOTS:
        return dhatu[:-2] + "nu"
    return None


def yam_cc_present_stem(dhatu: str, antarganas: str) -> Optional[str]:
    """Anudātta yam/dA (yacCati), distinct from GawAdi yam (yamati)."""
    if dhatu in ("yam", "dA") and not is_gawadi(antarganas):
        return "yacCa"
    return None


def yam_cc_lang_stem(dhatu: str, antarganas: str) -> Optional[str]:
    if dhatu in ("yam", "dA") and not is_gawadi(antarganas):
        return "yacC"
    return None


def yam_cc_future_stem(dhatu: str, antarganas: str) -> Optional[str]:
    if dhatu == "yam" and not is_gawadi(antarganas):
        return "yaMsy"
    if dhatu == "dA":
        return "dAsy"
    return None


_G1_AYA_PRESENT = frozenset({"ji", "Sri", "nI", "De", "jri"})
_G1_A_FINAL = frozenset({"SrA", "jYA"})
_G1_NV_ROOTS = frozenset({"Dinv"})
_BIDADI_THEMATIC = frozenset({"mid", "med", "meD", "vap", "vas", "tF", "guh"})
_YA_THEMATIC = frozenset({"tras", "Bram", "yas"})


def uses_aya_present(cgana: int, dhatu: str, antarganas: str) -> bool:
    if dhatu in _BIDADI_THEMATIC:
        return False
    return cgana == 1 and (
        is_bidadi(antarganas) or is_yajadi(antarganas) or dhatu in _G1_AYA_PRESENT
    )


_G9_N_INFIX = frozenset(
    {
        "Dras",
        "Kav",
        "SranT",
        "aS",
        "banD",
        "granT",
        "guD",
        "jYA",
        "kliS",
        "knU",
        "kunT",
        "kzuB",
        "mI",
        "manT",
        "mfd",
        "naB",
        "pU",
        "si",
        "skamB",
        "sku",
        "skumB",
        "stamB",
        "stumB",
        "tuB",
        "yu",
        "lU",
        "DU",
        "jyA",
        "lI",
        "vlI",
        "blI",
        "plI",
    }
)


def g9_uses_n_infix(dhatu: str, antarganas: str = "") -> bool:
    """Gaṇa-9 roots that take n-infix without R (mInAti, not krIRAti)."""
    return dhatu in _G9_N_INFIX


def g9_uses_r_infix(dhatu: str, antarganas: str = "") -> bool:
    return not g9_uses_n_infix(dhatu, antarganas)


def g9_n_lang_base(dhatu: str) -> str:
    """Laṅ stem body for gaṇa-9 n-infix roots (before join adds n or R)."""
    if dhatu.endswith("mB"):
        return dhatu[:-2] + "B"
    if dhatu == "pU":
        return "pun"
    if dhatu.endswith("U") and len(dhatu) == 2:
        return dhatu[0].lower() + "un"
    if dhatu == "SranT":
        return "SraT"
    if dhatu.endswith("I") and len(dhatu) == 2:
        return dhatu[0].lower() + "i"
    if dhatu == "jyA":
        return "jin"
    if dhatu == "jYA":
        return "jAn"
    if dhatu.endswith("lI"):
        return dhatu[:-2] + "lin"
    return dhatu


def g9_r_lang_root(dhatu: str) -> str:
    """Root before R-infix for gaṇa-9 R-class laṅ/lot/lat."""
    if dhatu == "SF":
        return "SIr"
    if dhatu.endswith("F") and len(dhatu) >= 2:
        return dhatu[:-1] + "f"
    if dhatu.endswith("I") and len(dhatu) == 2:
        return dhatu[0].lower() + "i"
    if dhatu == "F":
        return ""
    return dhatu


def ya_present_base(dhatu: str) -> str:
    """Gaṇa 4 root before -ya (lengthen i only, not u/ṛ)."""
    for idx in range(len(dhatu) - 1, -1, -1):
        if dhatu[idx] == "i":
            return dhatu[:idx] + "I" + dhatu[idx + 1 :]
        if dhatu[idx] == "I":
            return dhatu
    return dhatu


def sad_present_base(dhatu: str) -> Optional[str]:
    """Present base for sad (sīdati): a-strengthening to Id."""
    if dhatu == "sad":
        return "sId"
    if dhatu == "guh":
        return "gUh"
    if dhatu == "pA":
        return "pib"
    if dhatu == "GrA":
        return "jiGr"
    if dhatu == "saYj":
        return "saj"
    return None


def g6_present_base(dhatu: str) -> str:
    """Gaṇa 6 present base before thematic -a."""
    sp = sad_present_base(dhatu)
    if sp is not None:
        return sp
    if dhatu == "SuB":
        return apply_guna_to_stem(dhatu)
    if dhatu.endswith("U"):
        return dhatu[:-1] + "uv"
    if dhatu.endswith("u"):
        return dhatu + "v"
    if dhatu.endswith(("i", "I")) and len(dhatu) <= 3:
        return dhatu + "y"
    if dhatu.endswith(("F", "f")):
        graded = apply_guna_to_stem(dhatu)
        return graded if graded != dhatu else dhatu
    if "jj" in dhatu and "a" in dhatu:
        idx = dhatu.index("a")
        return dhatu[:idx] + "Bf" + dhatu[idx + 1 :]
    if dhatu.endswith("Sc") and "a" in dhatu:
        idx = dhatu.rfind("a")
        return dhatu[:idx] + "f" + dhatu[idx + 1 :]
    if len(dhatu) == 4 and dhatu[2] in "aA":
        return dhatu[0] + "i" + dhatu[3]
    return dhatu


_CAUSATIVE_GUNA_AY = frozenset({"yam", "cap", "cah", "rah", "bal", "jYap"})


def lang_geminate_stem(dhatu: str, stem: str) -> str:
    """Initial consonant gemination in laṅ (Card → cCarday, Cand → cCand)."""
    if (
        len(dhatu) == 4
        and dhatu[0] == "C"
        and dhatu[1] in "aA"
        and dhatu[2] in "rnYjJ"
    ):
        return dhatu[0].lower() + stem
    if len(dhatu) == 3 and dhatu[0] == "C" and dhatu[1] in "aA":
        return dhatu[0].lower() + stem
    return stem


def apply_vrddhi_to_stem(stem: str) -> str:
    """Vṛddhi on the last vowel (selected gaṇa-1 presents)."""
    for idx in range(len(stem) - 1, -1, -1):
        ch = stem[idx]
        if ch == "a":
            return stem[:idx] + "A" + stem[idx + 1 :]
        if ch == "i":
            return stem[:idx] + "I" + stem[idx + 1 :]
        if ch == "u":
            return stem[:idx] + "U" + stem[idx + 1 :]
        if ch in "AIUEO":
            return stem
    return stem


def _causative_aya_base(dhatu: str) -> str:
    """Causative stem body before present -aya- (mArga→mArgaya, yam→yamaya)."""
    if dhatu in _CAUSATIVE_GUNA_AY:
        return apply_guna_to_stem(dhatu) + "aya"
    if dhatu[-1] in "UufF":
        return apply_guna_to_stem(dhatu) + "aya"
    graded = apply_causative_grade(dhatu)
    if graded.endswith("A") and graded != dhatu:
        return graded[:-1] + "aya"
    return graded + "aya"


def causative_present_stem(dhatu: str) -> str:
    """Present/imperative stem for gaṇa-10 causative (-aya-)."""
    return _causative_aya_base(dhatu)


def _causative_lang_base(dhatu: str) -> str:
    aya = _causative_aya_base(dhatu)
    return aya[:-1] if aya.endswith("aya") else aya + "ay"


def causative_lang_stem(dhatu: str) -> str:
    """Laṅ stem for gaṇa-10 causative."""
    init = vowel_initial_lang_stem(dhatu)
    if init is not None:
        return init + "ay"
    stem = _causative_lang_base(dhatu)
    return lang_geminate_stem(dhatu, stem)


def thematic_aya_present_stem(dhatu: str) -> Optional[str]:
    """Present stems with long A in -Aya- (e.g. gopAyati, glAyati)."""
    if dhatu.endswith("E"):
        return dhatu[:-1] + "Aya"
    if dhatu == "gup":
        return apply_guna_to_stem(dhatu) + "Aya"
    if dhatu == "DUp":
        return dhatu + "Aya"
    if dhatu in ("paR", "pan"):
        return dhatu + "Aya"
    return None


def thematic_present_base(dhatu: str, gana: int, aupadeshik: str = "") -> str:
    """Thematic present root before -a (gaṇa 1/6)."""
    sp = sad_present_base(dhatu)
    if sp is not None:
        return sp
    if dhatu == "sUrkzy" and aupadeshik.startswith("z"):
        return "sUkzy"
    if gana == 1:
        rv = g1_rv_nv_present_base(dhatu)
        if rv:
            return rv
    if gana == 6:
        return dhatu
    if "W" in dhatu and len(dhatu) > 3 and dhatu.endswith(("iv", "Iv", "uv", "Uv")):
        idx = len(dhatu) - 2
        if dhatu[idx] == "i":
            return dhatu[:idx] + "I" + dhatu[idx + 1 :]
        if dhatu[idx] == "u":
            return dhatu[:idx] + "U" + dhatu[idx + 1 :]
    if dhatu.startswith(("kzI", "kzU")) and dhatu.endswith(("Iv", "Uv")):
        return apply_guna_to_stem(dhatu)
    if len(dhatu) == 4 and dhatu[0] in "kgcjwqtp" and dhatu[2] == "a" and dhatu[3] in "mn":
        return apply_vrddhi_to_stem(dhatu)
    for idx in range(len(dhatu) - 1, -1, -1):
        if dhatu[idx] in VOWEL_FINAL:
            trailing = dhatu[idx + 1 :]
            if len(trailing) <= 1:
                if dhatu[idx] in "IUUF" and idx != len(dhatu) - 1:
                    return dhatu
                return apply_guna_to_stem(dhatu)
            return dhatu
    return dhatu


def thematic_join(stem_a: str, ending: str) -> str:
    """Join a thematic stem (ending in 'a') to a tinanta ending with sandhi."""
    if not stem_a.endswith("a"):
        return stem_a + ending
    if ending.startswith("a"):
        return stem_a + ending[1:]
    if ending.startswith("A"):
        return stem_a[:-1] + ending
    return stem_a + ending
