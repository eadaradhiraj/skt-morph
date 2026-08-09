"""Join stems to tinanta endings with gaṇa-specific sandhi."""

from typing import List, Optional

from .phonology import apply_guna_to_stem, g9_r_lang_root, g9_uses_r_infix, thematic_join
from .redup import GANA3, gana3_join_mode, gana3_weak_stem
from .stems import AD_GANAS, CAUSATIVE_GANAS, N_GANA, NI_GANA, NU_GANAS, THEMATIC_GANAS, YA_GANA

AD_T_INFLECT = frozenset({"ti", "taH", "si", "thaH", "tha", "tAt", "tAd", "tu", "tam", "ta"})
_VOWEL = "aeiouAIUEO"


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
    if base.endswith(("fmB", "amB")):
        return True
    if base.endswith("kzy"):
        return True
    if base.startswith("kz") and base.endswith("ay"):
        return True
    if len(base) >= 3 and "r" in base and base.endswith("zy"):
        return True
    return False


def _thematic_lot_third(base: str, ending: str, gana: int) -> str:
    if ending != "Ani":
        return base + ending
    if gana in CAUSATIVE_GANAS and base.endswith("ay") and "r" in base[:-2]:
        return base + "ARi"
    if gana in THEMATIC_GANAS:
        if (base.endswith("Ay") and len(base) <= 4) or base.endswith("aya") or _plot_uses_ari(base):
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
        if family in ("lang", "vidhilin", "lit"):
            return stem + ending
        if family == "lrt":
            return thematic_join(stem, ending) if stem.endswith("a") else stem + ending
        if purusha == 3 and ending.startswith("A"):
            base = stem[:-1] if stem.endswith("a") else stem
            if family == "lot" and ending == "Ani":
                return _thematic_lot_third(base, ending, gana)
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
            return "ahanaH"
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
    return stem + ending


def _join_ad(
    stem: str,
    ending: str,
    family: str,
    dhatu: Optional[str] = None,
    purusha: int = 1,
    vacana: int = 1,
) -> str:
    if dhatu == "han":
        return _join_han(stem, ending, family, purusha, vacana)
    if dhatu == "dviz":
        if family == "lat" and ending == "anti":
            return "dvez" + "anti"
        if family == "lat" and ending == "si":
            return "dvezw" + "i"
    if not ending:
        return stem
    if stem != "ad":
        if family == "lrt" and stem.endswith(("zya", "tsya", "izya")):
            if ending == "anti":
                return stem[:-1] + "anti"
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
