"""Join stems to tinanta endings with gaṇa-specific sandhi."""

from typing import List, Optional

from .phonology import apply_guna_to_stem, g9_r_lang_root, g9_uses_r_infix, thematic_join, _G1_A_FINAL
from .redup import GANA3, gana3_join_mode, gana3_weak_stem
from .stems import AD_GANAS, CAUSATIVE_GANAS, N_GANA, NI_GANA, NU_GANAS, THEMATIC_GANAS, YA_GANA

AD_T_INFLECT = frozenset({"ti", "taH", "si", "thaH", "tha", "tAt", "tAd", "tu", "tam", "ta"})
_VOWEL = "aeiouAIUEO"
_G2_U_LANG_AVIT = frozenset({"ru", "tu", "stu"})
_G2_U_LANG_OZY_LRT = frozenset({"su", "tu", "dyu", "ku", "stu"})
_G2_U_LAT_O = frozenset({"yu", "nu", "ku", "su", "dyu", "kzu", "snu", "kzRu", "UrRu"})
_G2_A_LAT_ROOTS = frozenset(
    {"yA", "vA", "BA", "snA", "SrA", "drA", "psA", "pA", "rA", "lA", "dA", "KyA", "prA", "mA"}
)
_G6_PLOT_ARI = frozenset(
    {
        "Cur",
        "Dru",
        "Gur",
        "Kur",
        "bfh",
        "dfB",
        "dfP",
        "dfmP",
        "dfp",
        "fP",
        "fh",
        "fmP",
        "fz",
        "gF",
        "kF",
        "kfz",
        "kur",
        "kzi",
        "kzip",
        "kzur",
        "miz",
        "mur",
        "pur",
        "ri",
        "riP",
        "rih",
        "sPar",
        "sPur",
        "stfMh",
        "stfh",
        "sur",
        "tfMh",
        "tfP",
        "tfh",
        "tfmP",
        "tfp",
        "vfh",
    }
)
_G6_LOT_A_ENDINGS = frozenset({"tAt", "tAm", "tAd", "tu", "tam", "ta", "Di", "t"})
_G2_IH_PLOT = {
    "duh": ("dugD", "dogD", "doh"),
    "dih": ("digD", "degD", "deh"),
    "lih": ("lIQ", "leQ", "leh"),
}
_G2_IH_LRT = {
    "duh": "Dok",
    "dih": "Dek",
    "lih": "lek",
}


def _g2_u_plot_body(dhatu: str) -> str:
    if dhatu in _G2_U_LANG_AVIT:
        return dhatu[0] if len(dhatu) == 2 else dhatu[:-1]
    return dhatu[:-1]


def _g2_u_plot_join(
    dhatu: str,
    ending: str,
    purusha: int,
    vacana: int,
) -> Optional[str]:
    if dhatu in _G2_IH_PLOT:
        stem, tu_stem, guna = _G2_IH_PLOT[dhatu]
        if ending in ("tAt", "tAd"):
            return stem + ending[1:]
        if ending == "tu" and purusha == 1 and vacana == 1:
            return tu_stem + "u"
        if ending == "tAm":
            return stem + "Am"
        if ending == "antu":
            return dhatu + "antu"
        if ending == "Di" and purusha == 2 and vacana == 1:
            return stem + "i"
        if ending == "tam":
            return stem + "am"
        if ending == "ta":
            return stem + "a"
        if purusha == 3:
            if ending == "Ani":
                return guna + "Ani"
            if ending == "Ava":
                return guna + "Ava"
            if ending == "Ama":
                return guna + "Ama"
        return None
    if not dhatu.endswith("u") or dhatu in ("i", "as"):
        return None
    body = _g2_u_plot_body(dhatu)
    ot = "O" if dhatu == "UrRu" else apply_guna_to_stem(dhatu)[-1]
    if dhatu in _G2_U_LANG_AVIT:
        if ending == "tu" and purusha == 1 and vacana == 1:
            return body + "avItu"
        if ending in ("tAt", "tAd"):
            return body + "ut" + ending[1:]
        if ending == "tAm":
            return body + "utAm"
        if ending == "antu":
            return body + "uvantu"
        if ending == "Di" and purusha == 2 and vacana == 1:
            return body + "uhi"
        if ending == "tam":
            return body + "utam"
        if ending == "ta":
            return body + "uta"
        if purusha == 3:
            if ending == "Ani":
                return body + "avARi"
            if ending == "Ava":
                return body + "avAva"
            if ending == "Ama":
                return body + "avAma"
        return None
    if ending in ("tAt", "tAd"):
        return body + "ut" + ending[1:]
    if ending == "tu" and purusha == 1 and vacana == 1:
        return body + ot + "tu"
    if ending == "tAm":
        return body + "utAm"
    if ending == "antu":
        return body + "uvantu"
    if ending == "Di" and purusha == 2 and vacana == 1:
        return body + "uhi"
    if ending == "tam":
        return body + "utam"
    if ending == "ta":
        return body + "uta"
    if purusha == 3:
        if ending == "Ani":
            return body + "avAni"
        if ending == "Ava":
            return body + "avAva"
        if ending == "Ama":
            return body + "avAma"
    return None


def _g2_u_lat_join(
    dhatu: str,
    ending: str,
    purusha: int,
    vacana: int,
) -> Optional[str]:
    if not dhatu.endswith("u") or dhatu in ("i", "as"):
        return None
    if dhatu in _G2_U_LANG_AVIT:
        body = dhatu[0] if len(dhatu) == 2 else dhatu[:-1]
        if ending == "ti":
            return body + "avIti"
        if ending == "taH":
            return body + "utaH"
        if ending == "anti":
            return body + "uvanti"
        if ending == "si":
            return body + "uze"
        if ending in ("TaH", "Ta"):
            return body + "u" + ending
        if ending == "mi":
            return body + "Omi"
        if ending in ("vaH", "maH"):
            return body + "u" + ending
        return None
    if dhatu not in _G2_U_LAT_O and not (len(dhatu) == 2 and dhatu.endswith("u")):
        return None
    body = dhatu[:-1]
    if ending == "ti" and purusha == 1 and vacana == 1:
        return body + "Oti"
    if ending == "taH" and purusha == 1 and vacana == 2:
        return body + "utaH"
    if ending == "anti" and purusha == 1 and vacana == 3:
        return body + "uvanti"
    if ending == "si" and purusha == 2 and vacana == 1:
        return body + "uze"
    if ending in ("TaH", "Ta") and purusha == 2:
        return body + "u" + ending
    if ending == "mi" and purusha == 1 and vacana == 1:
        return body + "Omi"
    if ending in ("vaH", "maH") and purusha == 3:
        return body + "u" + ending
    return None


def _g2_a_lang_join(dhatu: str, ending: str) -> Optional[str]:
    if dhatu not in _G2_A_LAT_ROOTS:
        return None
    if ending in ("at", "ad"):
        return dhatu[:-1] + "A" + ending[1:]
    return None


def _g2_a_lat_join(dhatu: str, ending: str, purusha: int, vacana: int) -> Optional[str]:
    if dhatu not in _G2_A_LAT_ROOTS:
        return None
    body = dhatu[0]
    if ending == "anti" and purusha == 1 and vacana == 3:
        return body + "Anti"
    if ending in ("TaH", "Ta") and purusha == 2 and vacana == 2:
        return body + "AT" + ending[1:]
    return None


def _g2_u_lang_body(dhatu: str) -> str:
    if dhatu == "UrRu":
        return dhatu[1:-1]
    if dhatu in _G2_U_LANG_AVIT:
        return dhatu[0] if len(dhatu) == 2 else dhatu[:-1]
    return dhatu[:-1]


def _g2_u_lang_prefix(dhatu: str) -> str:
    return "O" if dhatu == "UrRu" else "a"


def _g2_u_lang_join(
    dhatu: str,
    ending: str,
    purusha: int,
    vacana: int,
) -> Optional[str]:
    if not dhatu.endswith("u") or dhatu in ("i", "as"):
        return None
    body = _g2_u_lang_body(dhatu)
    prefix = _g2_u_lang_prefix(dhatu)
    if dhatu in _G2_U_LANG_AVIT:
        if purusha == 1 and vacana == 1 and ending in ("at", "ad"):
            return prefix + body + "avI" + ending[1:]
        if purusha == 1 and vacana == 2 and ending == "atAm":
            return prefix + body + "utAm"
        if purusha == 1 and vacana == 3 and ending == "an":
            return prefix + body + "uvan"
        if purusha == 2 and vacana == 1 and ending == "aH":
            return prefix + body + "avIH"
        if purusha == 2 and vacana == 2 and ending == "atam":
            return prefix + body + "utam"
        if purusha == 2 and vacana == 3 and ending == "ata":
            return prefix + body + "uta"
        if purusha == 3 and vacana == 1 and ending == "am":
            return prefix + body + "avam"
        if purusha == 3 and vacana == 2 and ending == "va":
            return prefix + body + "uva"
        if purusha == 3 and vacana == 3 and ending == "ma":
            return prefix + body + "uma"
        return None
    ot_vowel = "o" if dhatu == "UrRu" else "O"
    if purusha == 1 and vacana == 1 and ending in ("at", "ad"):
        return prefix + body + ot_vowel + ending[1:]
    if purusha == 1 and vacana == 2 and ending == "atAm":
        return prefix + body + "utAm"
    if purusha == 1 and vacana == 3 and ending == "an":
        return prefix + body + "uvan"
    if purusha == 2 and vacana == 1 and ending == "aH":
        return prefix + body + ot_vowel + "H"
    if purusha == 2 and vacana == 2 and ending == "atam":
        return prefix + body + "utam"
    if purusha == 2 and vacana == 3 and ending == "ata":
        return prefix + body + "uta"
    if purusha == 3 and vacana == 1 and ending == "am":
        return prefix + body + "avam"
    if purusha == 3 and vacana == 2 and ending == "va":
        return prefix + body + "uva"
    if purusha == 3 and vacana == 3 and ending == "ma":
        return prefix + body + "uma"
    return None


def _kfnv_vowel(purusha: int, vacana: int, family: str) -> str:
    if family == "lot":
        if purusha == 3:
            return "a"
        if purusha == 1 and vacana == 3:
            return "v"
        return "u"
    if family == "vidhilin":
        return "u"
    if family == "lang" and purusha == 3 and vacana == 1:
        return "a"
    if purusha == 1 and vacana == 3:
        return "v"
    if vacana in (2, 3):
        return "u"
    return "o"


def _kfnv_stem(purusha: int, vacana: int, family: str) -> str:
    return "kfR" + _kfnv_vowel(purusha, vacana, family)


def _join_kfnv(
    stem: str,
    ending: str,
    family: str,
    purusha: int,
    vacana: int,
) -> str:
    base = _kfnv_stem(purusha, vacana, family)
    if family == "lang":
        if ending in ("at", "ad"):
            return "a" + base + ending[1:]
        if ending == "atAm":
            return "a" + base + "tAm"
        if ending == "an":
            return "a" + base + "an"
        if ending == "aH":
            return "a" + base + "H"
        if ending == "atam":
            return "a" + base + "tam"
        if ending == "ata":
            return "a" + base + "ta"
        if ending == "am":
            return "a" + base + "vam"
        if ending == "va":
            return "a" + base + "va"
        if ending == "ma":
            return "a" + base + "ma"
        if ending == "Ava":
            return "a" + _kfnv_stem(3, 2, family) + "va"
        if ending == "Ama":
            return "a" + _kfnv_stem(3, 3, family) + "ma"
    if family == "vidhilin":
        vidhi = {
            "et": "yAt",
            "ed": "yAd",
            "etAm": "yAtAm",
            "eyuH": "yuH",
            "eH": "yAH",
            "etam": "yAtam",
            "eta": "yAta",
            "eyam": "yAm",
            "eva": "yAva",
            "ema": "yAma",
        }
        return base + vidhi.get(ending, ending)
    if family == "lot":
        if purusha == 2 and vacana == 1 and ending == "tha":
            return base
        if ending in ("Ani", "Ava", "Ama"):
            return base + "v" + ending
        return base + ending
    if family == "lat":
        if ending == "si":
            return "kfRo" + "zi"
        if ending == "nti":
            return base + "anti"
        if ending == "Ami":
            return _kfnv_stem(3, 1, family) + "mi"
        if ending == "AvaH":
            return _kfnv_stem(3, 2, family) + "vaH"
        if ending == "AmaH":
            return _kfnv_stem(3, 3, family) + "maH"
        return base + ending
    return stem + ending


def _join_g1_a_final(
    stem: str,
    ending: str,
    family: str,
    purusha: int,
    dhatu: str = "",
) -> str:
    if family == "lang":
        if ending in ("at", "ad"):
            return "a" + stem + ending[1:]
        if ending == "atAm":
            return "a" + stem + "tAm"
        if ending == "an":
            return "a" + stem + "n"
        if ending == "aH":
            return "a" + stem + "H"
        if ending == "atam":
            return "a" + stem + "tam"
        if ending == "ata":
            return "a" + stem + "ta"
        if ending == "am":
            return "a" + stem + "m"
        if ending == "va":
            return "a" + stem + "va"
        if ending == "ma":
            return "a" + stem + "ma"
        if ending == "Ava":
            return "a" + stem + "va"
        if ending == "Ama":
            return "a" + stem + "ma"
    if family == "lat" and purusha == 3:
        if ending == "Ami":
            return stem + "mi"
        if ending == "AvaH":
            return stem + "vaH"
        if ending == "AmaH":
            return stem + "maH"
    if family == "lot":
        if purusha == 1 and ending == "antu":
            return stem[:-1] + "Antu"
        if purusha == 3 and ending == "Ani":
            if dhatu == "SrA":
                return stem[:-1] + "ARi"
            return stem[:-1] + "Ani"
        if purusha == 3 and ending in ("Ava", "Ama"):
            return stem + ending[1:]
    return stem + ending


def _g6_lot_join(
    stem: str,
    ending: str,
    family: str,
    purusha: int,
    gana: int,
    dhatu: Optional[str],
) -> Optional[str]:
    if gana != 6 or family not in ("lot", "plot") or stem.endswith("a"):
        return None
    if purusha == 3 and ending.startswith("A"):
        if ending == "Ani":
            return _thematic_lot_third(stem, ending, gana, dhatu)
        return stem + ending
    if ending in _G6_LOT_A_ENDINGS:
        return stem + "a" + ending
    return stem + ending


def _plot_uses_ari(base: str) -> bool:
    if base.endswith(("z", "Z", "r")):
        return True
    if base.endswith("reK"):
        return True
    if len(base) >= 3 and base[-3] == "r" and base.endswith(("AK", "aK")):
        return True
    if len(base) >= 2 and base[-2] == "r" and base[-1] not in _VOWEL and base[-1] not in "dDcCjJ":
        return True
    if "r" in base and base.endswith(("NK", "Ng", "nK")):
        return True
    if base.endswith("p") and "r" in base[:-1]:
        return True
    if base.endswith("P") and "r" in base[:-1]:
        return True
    if base.endswith("m") and "r" in base[:-1]:
        return True
    if base.endswith("Mh"):
        return True
    if base.endswith("h") and "r" in base[:-1]:
        return True
    if base.endswith("g") and "r" in base[:-1]:
        return True
    if base.endswith(("fmB", "amB")):
        return True
    if base.endswith("kzy"):
        return True
    if base.startswith("kz") and base.endswith("ay"):
        return True
    if base.startswith("kz") and base.endswith("v"):
        return True
    if len(base) >= 3 and "r" in base and base.endswith("zy"):
        return True
    return False


def _thematic_lot_third(
    base: str, ending: str, gana: int, dhatu: Optional[str] = None
) -> str:
    if ending != "Ani":
        return base + ending
    if gana in CAUSATIVE_GANAS and base.endswith("ay") and "r" in base[:-2]:
        return base + "ARi"
    if gana == YA_GANA:
        return base + "ARi"
    if gana == 6 and dhatu in _G6_PLOT_ARI:
        return base + "ARi"
    if gana in THEMATIC_GANAS:
        if (
            base.endswith(("aya", "Aya"))
            or (base.endswith("ay") and "r" in base[:-2])
            or (base.endswith("Ay") and ("r" in base or "z" in base))
            or (gana != 6 and _plot_uses_ari(base))
        ):
            return base + "ARi"
    return base + "Ani"


def join_form(
    stem: str,
    ending: str,
    gana: int,
    family: str,
    purusha: int,
    pada: str,
    augment: Optional[str] = None,
    dhatu: Optional[str] = None,
    vacana: int = 1,
    antarganas: Optional[str] = None,
) -> str:
    form = _join_raw(
        stem, ending, gana, family, purusha, pada, dhatu, vacana, antarganas
    )
    if augment:
        if dhatu == "i" and family == "lang":
            return form
        if dhatu and dhatu.endswith("u") and family == "lang" and dhatu not in ("i", "as"):
            if gana in AD_GANAS:
                return form
        if dhatu == "kfnv" and family == "lang":
            return form
        if dhatu in _G1_A_FINAL and family == "lang" and gana == 1:
            return form
        if dhatu == "dviz" and family == "lang":
            return "a" + form
        if gana in AD_GANAS and family == "lang" and form and form[0] == "A":
            return form
        if gana in AD_GANAS and form and form[0] in "aeiouAIUEO":
            form = augment + form
        elif gana in AD_GANAS:
            form = "a" + form
        else:
            form = augment + form
    return form


def _join_raw(
    stem: str,
    ending: str,
    gana: int,
    family: str,
    purusha: int,
    pada: str,
    dhatu: Optional[str] = None,
    vacana: int = 1,
    antarganas: Optional[str] = None,
) -> str:
    if gana == GANA3 and dhatu:
        if family == "lit":
            return _join_gana3_lit(stem, ending, purusha, vacana, dhatu)
        mode = gana3_join_mode(dhatu, apply_guna_to_stem(dhatu))
        if mode == "nu":
            return _join_nu(stem, ending, GANA3, family, purusha, pada)
        if pada == "A" and family == "lat":
            return _join_gana3_a(stem, ending, purusha, vacana, dhatu)
        return _join_gana3_ad(stem, ending, family, purusha, pada, dhatu)
    if gana in NU_GANAS:
        return _join_nu(stem, ending, gana, family, purusha, pada)
    if gana == N_GANA:
        return _join_n(stem, ending, family, purusha, pada)
    if gana == NI_GANA:
        return _join_ni(stem, ending, family, purusha, pada, antarganas, dhatu)
    if gana in AD_GANAS:
        return _join_ad(stem, ending, family, dhatu, purusha, vacana)
    if gana in THEMATIC_GANAS or gana in CAUSATIVE_GANAS or gana == YA_GANA:
        if dhatu == "kfnv" and family in ("lat", "lot", "lang", "vidhilin"):
            return _join_kfnv(stem, ending, family, purusha, vacana)
        if dhatu == "Dinv":
            return _join_dinv(stem, ending, family, purusha, vacana)
        if dhatu in _G1_A_FINAL and family in ("lat", "lot", "lang", "vidhilin"):
            return _join_g1_a_final(stem, ending, family, purusha, dhatu or "")
        if family in ("lang", "vidhilin", "lit"):
            if family == "lang" and stem.endswith("o") and ending in ("at", "ad"):
                return stem + ending[1:]
            return stem + ending
        if family == "lrt":
            if stem.endswith("zy") and not stem.endswith(("zya", "izya", "tsya")):
                if ending == "ti":
                    return stem + "ati"
                if ending in ("nti", "anti"):
                    return stem + "anti"
                if ending == "taH":
                    return stem + "ataH"
                if ending == "si":
                    return stem + "asi"
                if ending in ("TaH", "Ta"):
                    return stem + "a" + ending
                if ending and ending[0] in "aA":
                    return stem + ending
            if stem.endswith("sy") and not stem.endswith(("zya", "tsya", "izya")):
                if ending and ending[0] in "aA":
                    return stem + ending
                return stem + "a" + ending
            return thematic_join(stem, ending) if stem.endswith("a") else stem + ending
        if family in ("lot", "plot"):
            g6j = _g6_lot_join(stem, ending, family, purusha, gana, dhatu)
            if g6j is not None:
                return g6j
        if purusha == 3 and ending.startswith("A"):
            base = stem[:-1] if stem.endswith("a") else stem
            if family in ("lot", "plot") and ending == "Ani":
                return _thematic_lot_third(base, ending, gana, dhatu)
            return base + ending
        return thematic_join(stem, ending)
    return stem + ending


def _join_gana3_ad(
    stem: str,
    ending: str,
    family: str,
    purusha: int,
    pada: str,
    dhatu: str,
) -> str:
    guna = apply_guna_to_stem(dhatu)
    base = gana3_weak_stem(dhatu, guna, ending, purusha)
    if family == "lat":
        if ending == "nti":
            return stem[:-1] + "y" + "ati"
        if ending in ("TaH", "Ta") and stem.endswith("I"):
            return stem + ending
        if ending == "taH" and stem.endswith("I"):
            return stem + "taH"
        if ending == "si" and dhatu in ("BI", "hrI", "pF", "pf", "Bf"):
            return stem[:-1] + "o" + "zi" if stem.endswith("I") else base + ending
        if ending == "mi" and dhatu in ("BI", "hrI"):
            return base[:-1] + "o" + "mi" if base.endswith("e") else base + ending
        if ending in ("vaH", "maH") and dhatu in ("BI", "hrI"):
            return base + ending
    if family == "lot":
        if ending in ("tAt", "tAd") and stem.endswith("I"):
            return stem + "t" + ending
        if ending == "tu" and dhatu == "BI":
            return base + "tu"
        if ending == "tu" and dhatu == "hrI":
            return base + "tu"
        if ending == "Di" and dhatu in ("BI", "hrI"):
            return base + "Di"
        if ending == "vantu" and stem.endswith("I"):
            return stem[:-1] + "y" + "atu"
        if ending in ("avAni", "avAva", "avAma") and stem.endswith("Ur"):
            root = stem[:-2] + "ur"
            if ending == "avAni":
                return root + "avAni"
            if ending == "avAva":
                return root + "avAva"
            return root + "avAma"
    if family == "lang":
        if ending in ("ot", "od") and len(stem) <= 4:
            return stem + ending[1:]
        if ending == "van":
            return stem[:-1] + "y" + "an" if stem.endswith("I") else stem + "van"
    if family == "vidhilin":
        if ending.startswith("y") and not stem.endswith("y"):
            return stem + ending
    use = base if ending == "ti" and purusha == 1 else stem
    return _join_ad(use, ending, family)


def _join_gana3_a(
    stem: str,
    ending: str,
    purusha: int,
    vacana: int,
    dhatu: str,
) -> str:
    if stem.endswith("I"):
        base = stem[:-1]
        if ending == "te":
            return stem + "te"
        if ending == "ete":
            return base + "Ate"
        if ending == "ante":
            return base + "ate"
        if ending == "se":
            return stem + "ze"
        if ending == "eTe":
            return base + "ATe"
        if ending == "aDve":
            return stem + "Dve"
        if ending == "e" and purusha == 3:
            return base + "e"
        if ending == "Avahe":
            return stem + "vahe"
        if ending == "Amahe":
            return stem + "mahe"
    return stem + ending


def _join_gana3_lit(stem: str, ending: str, purusha: int, vacana: int, dhatu: str) -> str:
    if dhatu == "hu":
        if ending == "va":
            if purusha == 2 and vacana == 3:
                return "juhuva"
            return "juhAva"
        if ending == "vatuH":
            return "juhuvatuH"
        if ending == "vuH":
            return "juhuvuH"
        if ending == "viTa":
            return "juhu"[:-1] + "oTa"
        if ending == "vaTuH":
            return "juhuvaTuH"
        if ending == "viva":
            return "juhuviva"
        if ending == "vima":
            return "juhuvima"
    return stem + ending


def _join_han(stem: str, ending: str, family: str, purusha: int, vacana: int) -> str:
    """Irregular gaṇa-2 root han (7.4.1)."""
    if family == "lat":
        if ending == "taH":
            return "hataH"
        if ending in ("nti", "anti"):
            return "Gnanti"
        if ending == "si":
            return "haMsi"
        if ending in ("thaH", "TaH"):
            return "haTaH"
        if ending in ("tha", "Ta"):
            return "haTa"
        return stem + ending
    if family == "lot":
        if ending in ("tAt", "tAd"):
            return "hat" + ending[1:]
        if ending == "tu":
            return "hantu"
        if ending == "tAm":
            return "hatAm"
        if ending == "antu":
            return "Gnantu"
        if ending == "taH":
            return "hataH"
        if ending == "tam":
            return "hatam"
        if ending == "ta":
            return "hata"
        if ending == "Di" and purusha == 2 and vacana == 1:
            return "jahi"
        if ending in ("Ani", "Ava", "Ama"):
            return "han" + ending
        if ending in ("va", "ma") and purusha == 3:
            return "hanA" + ending
        return stem + ending
    if family == "lang":
        if ending == "aH" and purusha == 2 and vacana == 1:
            return "han"
        if ending in ("at", "ad") and purusha in (1, 2) and vacana == 1:
            return "han"
        if ending == "atAm":
            return "hatAm"
        if ending == "an":
            return "Gnan"
        if ending == "atam":
            return "hatam"
        if ending == "ata":
            return "hata"
        if ending == "am":
            return "hanam"
        if ending == "va":
            return "hanva"
        if ending == "ma":
            return "hanma"
        return stem + ending
    if family == "lrt":
        if stem.endswith(("zya", "tsya", "izya")):
            if ending == "anti":
                return stem[:-1] + "anti"
            if ending and ending[0] in "aA":
                return stem[:-1] + ending
        return stem + ending
    return stem + ending


def _join_dinv(stem: str, ending: str, family: str, purusha: int, vacana: int) -> str:
    """Gaṇa-1 Dinv present/imperative (Dino / Dinu / Din + v)."""
    base = "Din"
    if family == "lat":
        if ending == "ti":
            return base + "o" + ending
        if ending == "taH":
            return base + "u" + ending
        if ending in ("nti", "anti"):
            return base + "v" + "anti"
        if ending == "si":
            return base + "o" + "zi"
        if ending in ("TaH", "thaH"):
            return base + "u" + ending
        if ending in ("Ta", "tha"):
            return base + "u" + ending
        if ending == "Ami":
            return base + "o" + "mi"
        if ending == "AvaH":
            return base + "u" + "vaH"
        if ending == "AmaH":
            return base + "u" + "maH"
    if family == "lot":
        if ending in ("tAt", "tAd"):
            return base + "u" + ending
        if ending == "tu":
            return base + "o" + ending
        if ending == "tAm":
            return base + "u" + ending
        if ending == "Di" and purusha == 2:
            return base + "u"
        if ending == "antu":
            return base + "v" + "antu"
        if ending == "tam":
            return base + "u" + ending
        if ending == "ta":
            return base + "u" + ending
        if ending == "Ani":
            return base + "av" + "Ani"
        if ending == "Ava":
            return base + "av" + "Ava"
        if ending == "Ama":
            return base + "av" + "Ama"
    if family == "lang":
        if ending in ("at", "ad"):
            return base + "o" + ending[1:]
        if ending == "atAm":
            return base + "utAm"
        if ending == "an":
            return base + "v" + "an"
        if ending == "aH" and purusha == 2:
            return base + "o" + "H"
        if ending == "atam":
            return base + "utam"
        if ending == "ata":
            return base + "uta"
        if ending == "am":
            return base + "av" + "am"
        if ending == "Ava":
            return base + "u" + "va"
        if ending == "Ama":
            return base + "u" + "ma"
    if family == "vidhilin":
        if ending == "et":
            return base + "uyAt"
        if ending == "ed":
            return base + "uyAd"
        if ending == "etAm":
            return base + "uyAtAm"
        if ending == "eyuH":
            return base + "uyuH"
        if ending == "eH":
            return base + "uyAH"
        if ending == "etam":
            return base + "uyAtam"
        if ending == "eta":
            return base + "uyAta"
        if ending == "eyam":
            return base + "uyAm"
        if ending == "eva":
            return base + "uyAva"
        if ending == "ema":
            return base + "uyAma"
        if ending.startswith("y"):
            return base + "u" + ending
    if family == "lrt" and stem.endswith("izya"):
        if ending and ending[0] in "aA":
            return stem[:-1] + ending
    return stem + ending


def _join_ad(
    stem: str,
    ending: str,
    family: str,
    dhatu: Optional[str] = None,
    purusha: int = 1,
    vacana: int = 1,
) -> str:
    if dhatu == "i":
        if family == "lat" and ending == "si":
            return stem + "zi"
        if family == "lrt":
            body = apply_guna_to_stem("i") + "zy"
            if ending == "ti":
                return body + "ati"
            if ending and ending[0] in "aA":
                return body + ending
            return body + ending
        if family == "lang":
            if purusha == 1:
                if ending in ("at", "ad"):
                    return "E" + ending[1:]
                if ending == "atAm":
                    return "EtAm"
                if ending == "an":
                    return "Ayan"
            if purusha == 2:
                if ending == "aH":
                    return "EH"
                if ending == "atam":
                    return "Etam"
                if ending == "ata":
                    return "Eta"
            if purusha == 3:
                if ending == "am":
                    return "Ayam"
                if ending == "va":
                    return "Eva"
                if ending == "ma":
                    return "Ema"
    if family == "lang" and dhatu:
        joined = _g2_a_lang_join(dhatu, ending)
        if joined is not None:
            return joined
    if family == "lat" and dhatu:
        joined = _g2_u_lat_join(dhatu, ending, purusha, vacana)
        if joined is not None:
            return joined
        joined = _g2_a_lat_join(dhatu, ending, purusha, vacana)
        if joined is not None:
            return joined
    if family in ("lot", "plot") and dhatu:
        joined = _g2_u_plot_join(dhatu, ending, purusha, vacana)
        if joined is not None:
            return joined
    if dhatu in _G2_IH_LRT and family == "lrt":
        body = _G2_IH_LRT[dhatu]
        if ending == "ti":
            return body + "zyati"
        if ending == "taH":
            return body + "zyataH"
        if ending == "anti":
            return body + "zyanti"
        if ending == "si":
            return body + "zyasi"
        if ending in ("TaH", "Ta"):
            return body + "zya" + ending
        if ending and ending[0] in "A":
            return body + "zy" + ending
    if dhatu and dhatu.endswith("u") and dhatu not in ("i",) and family == "lrt":
        if dhatu in _G2_U_LANG_OZY_LRT:
            body = apply_guna_to_stem(dhatu) + "zy"
        else:
            body = dhatu[:-1] + "avizy"
        if ending == "ti":
            return body + "ati"
        if ending == "anti":
            return body + "anti"
        if ending == "taH":
            return body + "ataH" if body.endswith("zy") else body + ending
        if ending == "si":
            return body + "asi" if body.endswith("zy") else body + ending
        if ending in ("TaH", "Ta"):
            return body + "a" + ending if body.endswith("zy") else body + ending
        if ending and ending[0] in "aA":
            return body + ending
        return body + ending
    if family == "lang" and dhatu and dhatu.endswith("u") and dhatu not in ("i", "as"):
        joined = _g2_u_lang_join(dhatu, ending, purusha, vacana)
        if joined is not None:
            return joined
    if dhatu == "dviz":
        if family == "lang":
            if ending in ("at", "ad", "aH"):
                return "dvew"
            if ending == "atAm":
                return "dvizwAm"
            if ending == "an":
                return "dvizan"
            if ending == "atam":
                return "dvizwam"
            if ending == "ata":
                return "dvizwa"
            if ending == "am":
                return "dvezam"
            if ending == "va":
                return "dvizva"
            if ending == "ma":
                return "dvizma"
        if family == "lat":
            if ending == "ti":
                return "dvezwi"
            if ending == "taH":
                return "dvizwaH"
            if ending in ("nti", "anti"):
                return "dvizanti"
            if ending == "si":
                return "dvekzi"
            if ending in ("thaH", "TaH"):
                return "dvizWaH"
            if ending in ("tha", "Ta"):
                return "dvizWa"
            if ending == "mi":
                return "dvezmi"
            if ending == "vaH":
                return "dvizvaH"
            if ending == "maH":
                return "dvizmaH"
        if family in ("lot", "plot"):
            if ending in ("tAt", "tAd"):
                return "dvizw" + ending
            if ending == "tu":
                return "dvezwu"
            if ending == "tAm":
                return "dvizwAm"
            if ending == "antu":
                return "dvizantu"
            if ending == "Di" and purusha == 2:
                return "dviqQi"
            if ending == "tam":
                return "dvizwam"
            if ending == "ta":
                return "dvizwa"
            if purusha == 3:
                if ending == "Ani":
                    return "dvezARi"
                if ending in ("Ava", "va"):
                    return "dvezAva"
                if ending in ("Ama", "ma"):
                    return "dvezAma"
        if family == "lrt":
            if ending == "ti":
                return "dvekzyati"
            if ending == "taH":
                return "dvekzyataH"
            if ending in ("nti", "anti"):
                return "dvekzyanti"
            if ending == "si":
                return "dvekzyasi"
            if ending and ending[0] in "A":
                return "dvekzy" + ending
            if ending and (ending[0] in "aA" or ending in ("TaH", "Ta")):
                return "dvekzya" + ending
        if family == "vidhilin" and ending.startswith("y"):
            return "dviz" + ending
        return stem + ending
    if dhatu == "han":
        return _join_han(stem, ending, family, purusha, vacana)
    if not ending:
        return stem
    if stem != "ad":
        if family == "lrt" and stem.endswith("zy") and not stem.endswith(("zya", "izya", "tsya")):
            if ending == "ti":
                return stem + "ati"
            if ending == "anti":
                return stem + "anti"
            if ending and ending[0] in "aA":
                return stem + ending
        if family == "lrt" and stem.endswith(("zya", "tsya", "izya")):
            if ending == "anti":
                return stem[:-1] + "anti"
            if ending == "taH":
                return stem[:-1] + "ataH"
            if ending == "si":
                return stem[:-1] + "asi"
            if ending == "ti":
                return stem[:-1] + "ati"
            if ending[0] in "aA":
                return stem[:-1] + ending
        return stem + ending
    if ending in AD_T_INFLECT:
        if ending == "thaH":
            return "a" + "t" + "TaH"
        if ending == "tha":
            return "a" + "t" + "Ta"
        return "a" + "t" + ending
    if ending == "Di" and stem == "ad":
        return "ad" + "Di"
    if family == "lot":
        if ending == "tAm":
            return "at" + ending
        if ending in ("va", "ma"):
            return "ad" + "A" + ending
    if family == "lang":
        if ending in ("at", "ad"):
            return "A" + "d" + ending
        if ending == "atAm":
            return "A" + "ttAm"
        if ending == "an":
            return "A" + "d" + "an"
        if ending == "aH":
            return "A" + "d" + "aH"
        if ending == "atam":
            return "A" + "tt" + "am"
        if ending == "ata":
            return "A" + "tt" + "a"
        if ending == "am":
            return "A" + "d" + "am"
        if ending == "va":
            return "A" + "d" + "va"
        if ending == "ma":
            return "A" + "d" + "ma"
    return stem + ending


def _join_nu(stem: str, ending: str, gana: int, family: str, purusha: int, pada: str) -> str:
    if not ending:
        return stem if family == "lot" else (stem[:-1] if stem.endswith("u") else stem)
    if family == "lrt":
        if stem.endswith(("zya", "tsya", "izya")) and ending[0] in "aA":
            return stem[:-1] + ending
        return stem + ending
    if family == "lang":
        base = stem[:-1] if stem.endswith("u") else stem
        if ending in ("ot", "od", "oH"):
            return base + ending
        if ending.startswith("ut"):
            return base + ending
        if ending == "van":
            if gana == GANA3:
                return base + "avuH"
            return base + "v" + "an"
        if ending in ("avam", "uva", "va", "uma", "ma"):
            if ending in ("avam",):
                return base + "av" + "am"
            if ending in ("uva", "va"):
                return base + ("uv" if ending == "uva" else "v") + "a"
            if ending in ("uma", "ma"):
                return base + ("um" if ending == "uma" else "m") + "a"
    if family == "lot":
        if ending == "otu" and stem.endswith("u"):
            return stem[:-1] + "o" + "tu"
        if ending == "vantu" and stem.endswith("u"):
            if gana == GANA3:
                return stem[:-1] + "v" + "atu"
            return stem[:-1] + "v" + "antu"
        if ending in ("avAni", "avAva", "avAma") and stem.endswith("u"):
            return stem[:-1] + "av" + ending[2:]
        if ending == "u" and stem.endswith("u"):
            return stem
        if stem.endswith("u") and ending.startswith("ut"):
            return stem[:-1] + ending
    if ending == "ti":
        return stem[:-1] + "o" + "ti" if stem.endswith("u") else stem + ending
    if ending == "taH":
        return stem + "taH" if stem.endswith("u") else stem + ending
    if ending == "nti":
        if gana == GANA3:
            return stem[:-1] + "v" + "ati" if stem.endswith("u") else stem + ending
        return stem[:-1] + "v" + "anti" if stem.endswith("u") else stem + ending
    if ending == "zi":
        return stem[:-1] + "o" + "zi" if stem.endswith("u") else stem + ending
    if ending in ("TaH", "Ta"):
        return stem + ending
    if ending == "mi":
        return stem[:-1] + "o" + "mi" if stem.endswith("u") else stem + ending
    if ending == "vaH":
        return stem + "vaH" if stem.endswith("u") else stem + ending
    if ending == "maH":
        return stem[:-1] + "um" + "aH" if stem.endswith("u") else stem + ending
    if ending == "Ani" and stem.endswith("u"):
        return stem[:-1] + "av" + "Ani"
    if ending == "antu" and stem.endswith("u"):
        if gana == GANA3:
            return stem[:-1] + "v" + "atu"
        return stem[:-1] + "v" + "antu"
    if ending == "tu":
        return stem[:-1] + "o" + "tu" if stem.endswith("u") else stem + ending
    if ending.startswith("t") and stem.endswith("u"):
        return stem + ending
    if pada == "A":
        if ending.startswith("v") and stem.endswith("u"):
            return stem[:-1] + ending
        if ending.startswith("z") and stem.endswith("u"):
            return stem[:-1] + "u" + ending
    return stem + ending


def _join_n(stem: str, ending: str, family: str, purusha: int, pada: str) -> str:
    if family == "lrt":
        if stem.endswith(("zya", "tsya")) and ending.startswith("A"):
            return stem[:-1] + ending
        return stem + ending
    if family in ("lang", "vidhilin"):
        if stem.endswith("Ra"):
            base_run = stem[:-2] + "n"
            base_ruR = stem[:-1]
            if family == "lang":
                if ending in ("at", "ad"):
                    return base_ruR + ending
                if ending == "atAm":
                    return base_run + "dDAm"
                if ending == "an":
                    return base_run + "Dan"
                if ending == "aH":
                    return base_ruR + "aH"
                if ending == "atam":
                    return base_run + "dDam"
                if ending == "ata":
                    return base_run + "dDa"
                if ending == "am":
                    return base_ruR + "aDam"
                if ending == "va":
                    return base_run + "Dva"
                if ending == "ma":
                    return base_run + "Dma"
        elif family == "lang" and stem.endswith("R"):
            base_run = stem[:-1] + "n"
            if ending in ("at", "ad"):
                return stem + ending
            if ending == "atAm":
                return base_run + "dDAm"
            if ending == "an":
                return base_run + "Dan"
            if ending == "atam":
                return base_run + "dDam"
            if ending == "ata":
                return base_run + "dDa"
            if ending == "am":
                return stem + "aDam"
            if ending == "va":
                return base_run + "Dva"
            if ending == "ma":
                return base_run + "Dma"
        return stem + ending
    if stem.endswith("Ra"):
        base_run = stem[:-2] + "n"
        base_ruR = stem[:-1]
        if ending == "ti" and purusha == 1:
            return base_ruR + "adDi"
        if ending in ("taH", "TaH"):
            return base_run + "dDaH"
        if ending == "Ta":
            return base_run + "dDa"
        if ending == "nti":
            return base_run + "Danti"
        if ending == "si":
            return base_ruR + "atsi"
        if ending == "Ami":
            return stem + "Dmi"
        if ending == "AvaH":
            return base_run + "DvaH"
        if ending == "AmaH":
            return base_run + "DmaH"
        if family == "lot":
            if ending in ("tAt", "tAd"):
                return base_run + "dD" + ending[1:]
            if ending == "Di":
                return base_ruR + "adDi"
            if ending == "tAm":
                return base_run + "dDAm"
            if ending == "tu":
                return base_ruR + "adDu"
            if ending == "tam":
                return base_run + "dDam"
            if ending == "ta":
                return base_run + "dDa"
            if ending == "antu":
                return base_run + "Dantu"
            if ending == "Ani":
                return stem + "DAni"
            if ending in ("Ava", "Ama"):
                return stem + "D" + ending
    return thematic_join(stem, ending) if stem.endswith("a") else stem + ending


def _join_ni_npattern(
    stem: str,
    ending: str,
    family: str,
    purusha: int,
    pada: str,
) -> str:
    """Gaṇa-9 join without R-infix (mI, stI, …)."""
    if family == "lrt":
        if stem.endswith(("zya", "tsya")) and ending.startswith("A"):
            return stem[:-1] + ending
        return stem + ending
    if family in ("lang", "vidhilin") and stem.endswith("R"):
        root = stem[:-1]
        if family == "lang":
            if ending in ("At", "Ad"):
                return root + "n" + ending
            if ending == "ItAm":
                return root + "nItAm"
            if ending == "an":
                return root + "nan"
            if ending == "AH":
                return root + "nAH"
            if ending == "Itam":
                return root + "nItam"
            if ending == "Ita":
                return root + "nIta"
            if ending == "Am":
                return root + "nAm"
            if ending == "Iva":
                return root + "nIva"
            if ending == "Ima":
                return root + "nIma"
        return stem + ending
    if not stem.endswith("nA"):
        return stem + ending
    root = stem[:-2]
    if family == "lot":
        if ending == "Atu":
            return root + "nAtu"
        if ending in ("ItAt", "ItAd"):
            return root + "n" + "I" + ending[1:]
        if ending == "ItAm":
            return root + "nItAm"
        if ending == "antu":
            return root + "nantu"
        if ending == "Ihi":
            return root + "nIhi"
        if ending == "Itam":
            return root + "nItam"
        if ending == "Ani":
            return root + "nAni"
        if ending in ("Ava", "Ama"):
            return root + "n" + ending
        if ending == "ta":
            return root + "nIta"
    if ending == "ti" and purusha == 1:
        return stem[:-1] + "Ati"
    if ending == "taH":
        return root + "nItaH"
    if ending == "nti":
        return root + "nanti"
    if ending == "si":
        return stem[:-1] + "Asi"
    if ending in ("TaH", "Ta"):
        return root + "nI" + ending
    if ending == "Ami":
        return root + "nAmi"
    if ending == "AvaH":
        return root + "nIvaH"
    if ending == "AmaH":
        return root + "nImaH"
    if ending == "te" and purusha == 1:
        return root + "nIte"
    if ending in ("ete", "ante", "se", "eTe", "aDve", "e", "Avahe", "Amahe"):
        if ending.startswith("e") and ending not in ("ete", "eTe"):
            return root + "n" + "e" if ending == "e" else root + "n" + ending
        return root + "nI" + ending if not ending.startswith("e") else root + "n" + ending
    return stem + ending


def _join_ni(
    stem: str,
    ending: str,
    family: str,
    purusha: int,
    pada: str,
    antarganas: Optional[str] = None,
    dhatu: Optional[str] = None,
) -> str:
    if dhatu and not g9_uses_r_infix(dhatu, antarganas or ""):
        return _join_ni_npattern(stem, ending, family, purusha, pada)
    if family == "lang" and stem.endswith("R"):
        base = g9_r_lang_root(stem[:-1])
        return base + "R" + ending
    if family == "lrt":
        if stem.endswith(("zya", "tsya")) and ending.startswith("A"):
            return stem[:-1] + ending
        return stem + ending
    if not stem.endswith("nA"):
        return stem + ending
    root = g9_r_lang_root(stem[:-2])
    if family == "lang":
        if ending in ("At", "Ad"):
            return root + "R" + ending
        if ending == "ItAm":
            return root + "RItAm"
        if ending == "an":
            return root + "Ran"
        if ending == "AH":
            return root + "RAH"
        if ending == "Itam":
            return root + "RItam"
        if ending == "Ita":
            return root + "RIta"
        if ending == "Am":
            return root + "RAm"
        if ending == "Iva":
            return root + "RIva"
        if ending == "Ima":
            return root + "RIma"
    if family == "lot":
        if ending == "Atu":
            return root + "RAtu"
        if ending in ("ItAt", "ItAd"):
            return root + "R" + "I" + ending[1:]
        if ending == "ItAm":
            return root + "RItAm"
        if ending == "antu":
            return root + "Rantu"
        if ending == "Ihi":
            return root + "RIhi"
        if ending == "Itam":
            return root + "RItam"
        if ending == "Ani":
            return root + "RAni"
        if ending in ("Ava", "Ama"):
            return root + "R" + ending
        if ending == "ta":
            return root + "RIta"
    if ending == "ti" and purusha == 1:
        return root + "RAti"
    if ending == "taH":
        return root + "RItaH"
    if ending == "nti":
        return root + "Ranti"
    if ending == "si":
        return root + "RAsi"
    if ending in ("TaH", "Ta"):
        return root + "R" + "I" + ending
    if ending == "Ami":
        return root + "RAmi"
    if ending == "AvaH":
        return root + "RIvaH"
    if ending == "AmaH":
        return root + "RImaH"
    if ending == "te" and purusha == 1:
        return root + "RIte"
    if ending in ("ete", "ante", "se", "eTe", "aDve", "e", "Avahe", "Amahe"):
        return root + "RI" + ending if not ending.startswith("e") else root + "R" + ending
    return stem + ending


def join_variants(
    stem: str,
    endings: List[str],
    gana: int,
    family: str,
    purusha: int,
    pada: str,
    augment: Optional[str] = None,
    dhatu: Optional[str] = None,
    vacana: int = 1,
    antarganas: Optional[str] = None,
) -> List[str]:
    return [
        join_form(
            stem, e, gana, family, purusha, pada, augment, dhatu, vacana, antarganas
        )
        for e in endings
    ]
