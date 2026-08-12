"""Stem formation per gaṇa and lakāra family."""

from typing import List, Optional, Tuple

from .phonology import (
    VOWEL_FINAL,
    apply_causative_grade,
    apply_guna_to_stem,
    apply_vrddhi_to_stem,
    bidadi_lang_stem,
    bidadi_present_stem,
    bidadi_vidhilin_stem,
    causative_lang_stem,
    causative_vidhilin_stem,
    causative_present_stem,
    g6_plot_base,
    g6_present_base,
    g6_vidhilin_stem,
    g6_lang_stem,
    g2_vidhilin_stem,
    g7_vidhilin_stem,
    g9_vidhilin_stem,
    g9_n_lang_base,
    g9_uses_n_infix,
    is_bidadi,
    is_yajadi,
    uses_aya_present,
    g1_nv_present_stem,
    g1_nv_vidhilin_stem,
    yam_cc_future_stem,
    yam_cc_lang_stem,
    yam_cc_present_stem,
    lang_geminate_stem,
    _causative_lang_base,
    _CAUSATIVE_LANG_BASE,
    _CAUSATIVE_LANG_NO_AUG,
    thematic_aya_present_stem,
    thematic_present_base,
    vowel_initial_lang_stem,
    ya_present_base,
    _G1_AYA_PRESENT,
    _G1_A_FINAL,
    _G1_NV_ROOTS,
    _BIDADI_THEMATIC,
    _YA_THEMATIC,
)
from .redup import (
    GANA3,
    gana3_future_stem,
    gana3_lang_stem,
    gana3_perfect_stem,
    gana3_present_stem,
    gana3_vidhilin_stem,
)
from .steps import EngineStep

THEMATIC_GANAS = {1, 6}
CAUSATIVE_GANAS = {10}
AD_GANAS = {2, 3}
NU_GANAS = {5, 8}
N_GANA = 7
NI_GANA = 9
YA_GANA = 4


def conjugation_gana(gana: int, tags: str = "") -> int:
    """Effective gaṇa for conjugation (g10 without nityaRic → thematic g1)."""
    if gana == 10 and "nityaRic" not in (tags or ""):
        return 1
    return gana


def _g6_skip_future_guna(dhatu: str) -> bool:
    """Roots whose lṛṭ stem keeps the ungraded dhātu (cluster/uppercase patterns)."""
    if dhatu.endswith("uq") or dhatu == "qip":
        return True
    if dhatu.startswith("f") and len(dhatu) >= 2:
        return True
    if "mP" in dhatu or "fM" in dhatu or "Mh" in dhatu:
        return True
    if any(ch in "IUA" for ch in dhatu[:-1]):
        return True
    if "mB" in dhatu and dhatu[0].isupper():
        return True
    if dhatu.endswith("uw"):
        return True
    if dhatu.endswith(("ump", "mp")) and len(dhatu) <= 4:
        return True
    if len(dhatu) <= 4 and len(dhatu) >= 3 and not dhatu.endswith(("d", "t", "D", "T")):
        if dhatu[0] in "ui" or dhatu[-1] in "cCjJ":
            return True
    return False


def _g6_future_suffix(graded: str) -> str:
    if graded.endswith("S"):
        return graded[:-1] + "kzya"
    if graded.endswith("Sc"):
        return graded[:-2] + "kzya"
    if graded.endswith("cC"):
        return graded[:-2] + "kzya"
    if graded.endswith(("jj", "JJ")):
        return graded[:-2] + "kzya"
    if graded.endswith("D"):
        return graded + "izya"
    if graded.endswith("d"):
        return graded[:-1] + "t" + "sya"
    if graded.endswith("z") and len(graded) <= 3:
        return graded + "izya"
    if graded.endswith("z"):
        return graded[:-1] + "kzya"
    if graded.endswith("fh"):
        return graded + "izya"
    if graded.endswith("ep"):
        return graded + "sya"
    if graded.endswith(("p", "P", "b", "B")):
        return graded + "izya"
    if graded.endswith(("c", "C", "j", "J")):
        return graded + "izya"
    return graded + "izya"


_G6_NO_FUTURE_GUNA = frozenset({"Cur"})


def g6_future_stem(dhatu: str) -> str:
    """Gaṇa 6 lṛṭ stem with guṇa and consonant-specific suffix."""
    if dhatu == "kzi":
        return apply_guna_to_stem(dhatu) + "zya"
    if dhatu == "SuB":
        return apply_guna_to_stem(dhatu) + "izya"
    if dhatu == "majj":
        return "maNkzy"
    if dhatu.endswith("ajj"):
        return dhatu[0] + "arkzya"
    if dhatu == "sfj":
        return dhatu[0] + "rakzya"
    if dhatu.endswith("U"):
        return dhatu[:-1] + "uvizya"
    if len(dhatu) == 2 and dhatu[-1] in "ui":
        if dhatu == "gu":
            return dhatu + "zya"
        return apply_guna_to_stem(dhatu) + "zya"
    if dhatu == "Dru":
        return dhatu + "zya"
    if dhatu.endswith("fh"):
        return apply_guna_to_stem(dhatu) + "izya"
    if dhatu in _G6_NO_FUTURE_GUNA:
        return _g6_future_suffix(dhatu)
    if len(dhatu) == 3 and dhatu[1] in "uU" and dhatu[0].isupper() and dhatu[2].isupper():
        graded_u = apply_guna_to_stem(dhatu)
        if graded_u != dhatu:
            return graded_u + "izya"
    base = dhatu if _g6_skip_future_guna(dhatu) else apply_guna_to_stem(dhatu)
    return _g6_future_suffix(base)


def _append_step(steps: List[EngineStep], form: str, sutras: List[str], kind: str) -> None:
    if not steps or steps[-1].form != form or steps[-1].kind != kind:
        steps.append(EngineStep(form, sutras, kind))


def _g1_future_base(dhatu: str, present_base: str, guna: str) -> str:
    """Gaṇa-1 future base before -sya/-izya (may differ from present base)."""
    if dhatu == "sad":
        return dhatu
    if dhatu == "pA":
        return "pib"
    if dhatu == "yaB":
        return "yap"
    if dhatu == "sfp":
        return "sarp"
    if dhatu == "tap":
        return "tap"
    if dhatu.endswith("nv") and len(dhatu) >= 4:
        if dhatu[0] == "r" or dhatu.endswith("fnv"):
            return dhatu[:-2] + "Rv"
    if "W" in dhatu and len(dhatu) > 3 and dhatu.endswith(("iv", "Iv", "uv", "Uv")):
        return apply_guna_to_stem(dhatu)
    if dhatu == "guh":
        return "gUh"
    if dhatu == "f":
        return "ar"
    if dhatu in ("SrA", "jYA"):
        return dhatu[:-1] + "i"
    vrddhi = apply_vrddhi_to_stem(dhatu)
    if present_base == vrddhi and present_base != dhatu:
        return dhatu
    if present_base == dhatu and guna != dhatu and dhatu.endswith(("Iv", "Uv")) and len(dhatu) > 3 and "W" not in dhatu:
        return guna
    return present_base


_G1_KZYA_ROOTS = frozenset({"Siz", "viz", "kruS", "ruh", "saYj"})
_G1_TSYA_ROOTS = frozenset({"sad", "Sad", "Gas", "SfD"})
_G1_ZY_FUTURES = {
    "sru": "srozy",
    "su": "sozy",
    "Sru": "Srozy",
    "Dru": "Drozy",
    "du": "dozy",
    "dru": "drozy",
    "tyaj": "tyakzy",
    "skand": "skantsy",
    "nam": "naMsy",
}
_G1_LRT_STEMS = {
    "dfS": "drakzya",
    "daMS": "daNkzya",
    "kfz": "karkzya",
    "dah": "Dakzya",
    "mih": "mekzya",
    "pac": "pakzya",
    "Baj": "Bakzya",
    "raYj": "raNkzya",
    "tviz": "tvekzya",
    "yaj": "yakzya",
    "vap": "vapsya",
    "vah": "vakzya",
    "vas": "vatsya",
    "Sap": "Sapsya",
}
_G2_U_LRT_OZY = frozenset({"su", "tu", "dyu", "ku", "stu"})


def _g1_special_lrt_stem(dhatu: str) -> Optional[str]:
    if dhatu in _G1_ZY_FUTURES:
        return _G1_ZY_FUTURES[dhatu]
    if dhatu in _G1_NV_ROOTS:
        return dhatu + "izya"
    if dhatu.endswith("A") and 2 <= len(dhatu) <= 4 and dhatu not in ("SrA", "jYA"):
        return dhatu + "sy"
    return None


def _g1_future_suffix(base: str, dhatu: str) -> str:
    if dhatu in _G1_LRT_STEMS:
        return _G1_LRT_STEMS[dhatu]
    if dhatu in _G1_KZYA_ROOTS:
        if dhatu == "saYj":
            return "saNkzy"
        graded = apply_guna_to_stem(dhatu)
        body = graded[:-1] if graded.endswith(("S", "h", "z")) else graded
        if dhatu.endswith("uS"):
            body = graded[:-1]
        return body + "kzya"
    if dhatu == "yam":
        return base + "izya"
    if dhatu in _G1_TSYA_ROOTS:
        if base.endswith(("d", "D")):
            return base[:-1] + "tsya"
        if base.endswith("s"):
            return base[:-1] + "tsya"
    if dhatu in ("yaB", "sfp", "tap"):
        return base + "sya"
    if dhatu == "kzi":
        return apply_guna_to_stem(dhatu) + "zya"
    if dhatu.endswith("kz"):
        return dhatu + "izya"
    if base.endswith("v"):
        return base + "izya"
    if base.endswith("e") and len(base) <= 2:
        return base + "zya"
    return base + "izya"


def _g1_future_from_present(dhatu: str, present_stem: str, guna: str) -> str:
    present_base = present_stem[:-1] if present_stem.endswith("a") else present_stem
    base = _g1_future_base(dhatu, present_base, guna)
    if dhatu in ("SrA", "jYA"):
        return base + "zy"
    if dhatu.endswith("nv") and len(dhatu) >= 4 and (dhatu[0] == "r" or dhatu.endswith("fnv")):
        return base + "izya"
    return _g1_future_suffix(base, dhatu)


def future_stem(
    guna: str,
    gana: int,
    present_stem: Optional[str] = None,
    dhatu: str = "",
) -> str:
    """Derive lṛṭ stem (3.2.135 etc.)."""
    if dhatu == "kzi":
        return apply_guna_to_stem(dhatu) + "zya"
    if gana == 1:
        special = _g1_special_lrt_stem(dhatu)
        if special:
            return special
    if gana == 2 and dhatu == "i":
        return apply_guna_to_stem(dhatu) + "zy"
    if gana == 2 and dhatu.endswith("u"):
        if dhatu in _G2_U_LRT_OZY:
            return apply_guna_to_stem(dhatu) + "zy"
        return dhatu[:-1] + "avizy"
    if gana == 1 and dhatu.endswith("kz") and dhatu != "kzi":
        return dhatu + "izya"
    if gana == 1 and dhatu in _G1_A_FINAL and present_stem:
        return _g1_future_from_present(dhatu, present_stem, guna)
    if gana == 1 and present_stem and dhatu in _G1_AYA_PRESENT and present_stem.endswith("aya"):
        if dhatu.endswith(("e", "E")):
            body = present_stem[:-2]
            return body[:-1] + "Asy"
        if dhatu == "Sri":
            return present_stem[:-1] + "izya"
        if dhatu.endswith(("i", "I")):
            return apply_guna_to_stem(dhatu) + "zya"
    if present_stem and present_stem.endswith("Aya"):
        if dhatu.endswith("E"):
            return present_stem[:-2] + "sy"
        return present_stem[:-1] + "izya"
    if present_stem and present_stem.endswith("yAa"):
        return present_stem[:-1] + "sy"
    if present_stem and present_stem.endswith("aya"):
        return present_stem[:-1] + "izya"
    if present_stem and present_stem.endswith("ya"):
        base = present_stem[:-1]
        return base + "izya"
    if present_stem and present_stem.endswith("a"):
        if gana == 1 and dhatu:
            return _g1_future_from_present(dhatu, present_stem, guna)
        base = present_stem[:-1]
        if base.endswith("v"):
            return base + "izya"
        if base.endswith("e") and len(base) <= 3:
            return base + "zya"
        if gana in CAUSATIVE_GANAS or gana == 1:
            return base + "izya"
        if gana == 6:
            return base + "sya"
        return base + "sya"
    if guna.endswith("t") and gana == 6:
        return guna + "sya"
    if gana in NU_GANAS and guna.endswith("o"):
        return guna + "zya"
    if gana == GANA3:
        return guna + "zya"
    if gana in AD_GANAS and guna[-1:] in ("d", "D", "t", "T"):
        return guna[:-1] + "tsya"
    if gana == NI_GANA:
        if dhatu == "mI":
            return "mAsya"
        if dhatu.endswith("mB"):
            return dhatu + "izya"
        graded = apply_guna_to_stem(dhatu)
        if dhatu.endswith("I") and len(dhatu) <= 3:
            return graded + "zya"
        return graded + "izya"
    if gana == N_GANA and guna[-1:] in ("d", "D"):
        return guna[:-1] + "tsya"
    if guna.endswith("v"):
        return guna + "izya"
    return guna + "izya"


def perfect_stem(dhatu: str, guna: str) -> str:
    """Reduplicated liṭ stem (6.1.1) — simplified for common gaṇa-1 pattern."""
    if len(dhatu) >= 2 and dhatu[0] in "kgcjwqtp":
        redupl = dhatu[0] + "a"
    elif len(guna) >= 2:
        redupl = guna[:2] if guna[1] in "aeiouAIUEO" else guna[0] + "a"
    else:
        redupl = (guna[0] if guna else dhatu[0]) + "a"
    if guna.endswith("v") and dhatu.endswith("U"):
        return "ba" + dhatu
    if guna.endswith("v"):
        return redupl + dhatu + "a"
    if guna.endswith("a"):
        return redupl + guna
    return redupl + guna + "a"


def derive_stem(
    dhatu: str,
    gana: int,
    family: str,
    derivation: str,
    tags: str = "",
    antarganas: str = "",
    aupadeshik: str = "",
) -> Tuple[Optional[str], Optional[str], List[EngineStep]]:
    """
    Return (stem, augment, steps).
    augment is a prefix applied after join (e.g. 'a' for laṅ).
    """
    steps: List[EngineStep] = []
    if derivation != "shuddha":
        steps.append(EngineStep(dhatu, ["1.3.1"], "dhatu"))
        return None, None, steps

    steps.append(EngineStep(dhatu, ["1.3.1"], "dhatu"))
    guna = apply_guna_to_stem(dhatu)
    if guna != dhatu:
        _append_step(steps, guna, ["7.2.115"], "guNa")

    cgana = conjugation_gana(gana, tags)
    present_stem: Optional[str] = None
    bidadi = cgana == 1 and is_bidadi(antarganas) and dhatu not in _BIDADI_THEMATIC
    aya_present = uses_aya_present(cgana, dhatu, antarganas)

    if aya_present:
        present_stem = bidadi_present_stem(dhatu)
        _append_step(steps, present_stem, ["3.1.33"], "yap")
    elif cgana in THEMATIC_GANAS:
        yam_stem = yam_cc_present_stem(dhatu, antarganas)
        nv_stem = g1_nv_present_stem(dhatu)
        aya_stem = thematic_aya_present_stem(dhatu) if cgana == 1 else None
        if yam_stem:
            present_stem = yam_stem
            _append_step(steps, present_stem, ["7.2.9"], "samprasaran")
        elif nv_stem:
            present_stem = nv_stem
            _append_step(steps, present_stem, ["7.3.84"], "nv_stem")
        elif aya_stem:
            present_stem = aya_stem
            _append_step(steps, present_stem, ["3.1.33"], "yap")
        elif dhatu in _G1_A_FINAL:
            present_stem = dhatu
            _append_step(steps, present_stem, ["3.1.68"], "sap")
        else:
            base = g6_plot_base(dhatu) if cgana == 6 else thematic_present_base(dhatu, cgana, aupadeshik)
            if base != dhatu:
                _append_step(steps, base, ["7.2.115"], "guNa")
            present_stem = base + "a"
            _append_step(steps, present_stem, ["3.1.68", "3.1.69"], "sap")
    elif cgana == YA_GANA:
        if dhatu in _YA_THEMATIC:
            present_stem = dhatu + "a"
            _append_step(steps, present_stem, ["3.1.68", "3.1.69"], "sap")
        else:
            ya_base = ya_present_base(dhatu)
            present_stem = ya_base + "ya"
            _append_step(steps, present_stem, ["3.1.33"], "yap")
    elif gana == GANA3:
        present_stem = gana3_present_stem(dhatu, guna)
        _append_step(steps, present_stem, ["6.1.1", "3.1.3"], "redup")
    elif gana in AD_GANAS:
        present_stem = guna
        _append_step(steps, present_stem, ["3.1.3"], "ad")
    elif gana in NU_GANAS:
        present_stem = dhatu + ("u" if dhatu.endswith("n") else "nu")
        _append_step(steps, present_stem, ["3.1.75"], "nu")
    elif gana == N_GANA:
        if dhatu.endswith("D"):
            present_stem = dhatu[:-1] + "Ra"
        else:
            present_stem = guna + "a"
        _append_step(steps, present_stem, ["7.3.88"], "n_gana")
    elif gana == NI_GANA:
        present_stem = dhatu + "nA"
        _append_step(steps, present_stem, ["3.1.81"], "nI")
    elif gana in CAUSATIVE_GANAS:
        present_stem = causative_present_stem(dhatu)
        if present_stem != dhatu + "aya":
            graded = present_stem[:-3] if present_stem.endswith("aya") else present_stem
            if graded != dhatu:
                _append_step(steps, graded, ["7.2.115", "3.1.25"], "causal_grade")
        _append_step(steps, present_stem[:-1] if present_stem.endswith("aya") else present_stem, ["3.1.25"], "causal_aya")
        _append_step(steps, present_stem, ["3.1.68"], "sap")
    else:
        return None, None, steps

    if family == "lat":
        return present_stem, None, steps
    if family == "lot" and cgana == 6:
        root = g6_plot_base(dhatu)
        _append_step(steps, root, ["3.2.69"], "plot_stem")
        return root, None, steps
    if family == "lot" and gana in AD_GANAS and dhatu not in ("i", "as"):
        if dhatu.endswith("u") or dhatu in ("duh", "dih", "lih"):
            _append_step(steps, dhatu, ["3.1.3"], "plot_stem")
            return dhatu, None, steps
    if family == "lot":
        return present_stem, None, steps

    if bidadi:
        if family == "lrt":
            if dhatu == "kzi":
                fstem = apply_guna_to_stem(dhatu) + "zya"
            elif dhatu.endswith("kz"):
                fstem = dhatu + "izya"
            else:
                graded = apply_guna_to_stem(dhatu)
                fstem = graded + ("izya" if len(graded) >= 3 else "zya")
            _append_step(steps, fstem, ["3.2.135"], "lrt")
            return fstem, None, steps
        if family == "lang":
            root = bidadi_lang_stem(dhatu)
            _append_step(steps, root, ["3.4.111"], "lang_stem")
            return root, "a", steps
        if family == "vidhilin":
            root = bidadi_vidhilin_stem(dhatu)
            _append_step(steps, root, ["3.4.104"], "vidhilin_stem")
            return root, None, steps

    if aya_present and not bidadi:
        if family == "lang":
            root = bidadi_lang_stem(dhatu)
            _append_step(steps, root, ["3.4.111"], "lang_stem")
            return root, "a", steps
        if family == "vidhilin":
            root = bidadi_vidhilin_stem(dhatu)
            _append_step(steps, root, ["3.4.104"], "vidhilin_stem")
            return root, None, steps

    if family == "lrt" and gana == GANA3:
        fstem = gana3_future_stem(dhatu, guna)
        _append_step(steps, fstem, ["3.2.135"], "lrt")
        return fstem, None, steps

    if family == "lrt" and gana == 6:
        fstem = g6_future_stem(dhatu)
        _append_step(steps, fstem, ["3.2.135"], "lrt")
        return fstem, None, steps

    yam_fut = yam_cc_future_stem(dhatu, antarganas)
    if family == "lrt" and yam_fut:
        _append_step(steps, yam_fut, ["3.2.135"], "lrt")
        return yam_fut, None, steps

    if family == "lrt" and gana == 1:
        special = _g1_special_lrt_stem(dhatu)
        if special and dhatu not in _G1_AYA_PRESENT:
            _append_step(steps, special, ["3.2.135"], "lrt")
            return special, None, steps
        if dhatu == "sUrkzy" and aupadeshik.startswith("z"):
            fstem = "sUkzyizya"
            _append_step(steps, fstem, ["3.2.135"], "lrt")
            return fstem, None, steps

    if family == "lrt":
        g = apply_guna_to_stem(dhatu) if gana in (YA_GANA,) else guna
        lrt_present = present_stem
        if aya_present and not bidadi and g and dhatu not in _G1_AYA_PRESENT:
            lrt_present = g + "a"
        fstem = future_stem(
            g,
            gana,
            lrt_present if gana not in (YA_GANA,) else None,
            dhatu,
        )
        _append_step(steps, fstem, ["3.2.135"], "lrt")
        return fstem, None, steps

    if family == "lang":
        yam_lang = yam_cc_lang_stem(dhatu, antarganas)
        if yam_lang and cgana == 1:
            _append_step(steps, yam_lang, ["7.2.9"], "lang_stem")
            return yam_lang, "a", steps
        nv_stem = g1_nv_present_stem(dhatu)
        if nv_stem and cgana == 1:
            _append_step(steps, nv_stem, ["7.3.84"], "lang_stem")
            return nv_stem, "a", steps
        if dhatu in _G1_A_FINAL and cgana == 1:
            _append_step(steps, dhatu, ["3.4.111"], "lang_stem")
            return dhatu, "a", steps
        if dhatu == "f" and cgana == 1:
            _append_step(steps, "Ar", ["3.4.111"], "lang_stem")
            return "Ar", None, steps
        if gana in CAUSATIVE_GANAS:
            init = vowel_initial_lang_stem(dhatu)
            if init is not None and dhatu not in _CAUSATIVE_LANG_BASE:
                root = init + "ay"
                _append_step(steps, root, ["3.4.111"], "lang_stem")
                return root, None, steps
            root = causative_lang_stem(dhatu)
            lang_aug = None if dhatu in _CAUSATIVE_LANG_NO_AUG else "a"
            _append_step(steps, root, ["3.4.111"], "lang_stem")
            return root, lang_aug, steps
        elif gana == YA_GANA:
            init = vowel_initial_lang_stem(dhatu)
            if init is not None:
                _append_step(steps, init, ["3.4.111"], "lang_stem")
                return init, None, steps
            root = dhatu if dhatu in _YA_THEMATIC else ya_present_base(dhatu)
        elif cgana == 6:
            root, lang_aug = g6_lang_stem(dhatu)
            root = lang_geminate_stem(dhatu, root)
            if len(dhatu) >= 3 and dhatu[0] == "C" and dhatu[1] not in "aA":
                root = "c" + root
            _append_step(steps, root, ["3.4.111"], "lang_stem")
            return root, lang_aug, steps
        elif cgana in THEMATIC_GANAS:
            aya_stem = thematic_aya_present_stem(dhatu) if cgana == 1 else None
            if aya_stem:
                root = aya_stem[:-1]
            else:
                init = vowel_initial_lang_stem(dhatu)
                if init is not None:
                    _append_step(steps, init, ["7.2.115"], "lang_stem")
                    return init, None, steps
                root = thematic_present_base(dhatu, cgana, aupadeshik)
        elif gana == GANA3:
            root = gana3_lang_stem(dhatu, guna)
        elif gana in NU_GANAS:
            root = present_stem[:-1] if present_stem and present_stem.endswith("u") else dhatu + "u"
        elif gana == N_GANA:
            root = dhatu[:-1] + "R" if dhatu.endswith("D") else guna
        elif gana == NI_GANA:
            if g9_uses_n_infix(dhatu, antarganas):
                base = g9_n_lang_base(dhatu)
                root = base if base.endswith("n") else base + "R"
            else:
                root = dhatu + "R"
        else:
            root = guna
        root = lang_geminate_stem(dhatu, root)
        _append_step(steps, root, ["3.4.111"], "lang_stem")
        return root, "a", steps

    if family == "vidhilin":
        yam_vid = yam_cc_lang_stem(dhatu, antarganas)
        if yam_vid and cgana == 1:
            _append_step(steps, yam_vid, ["7.2.9"], "vidhilin_stem")
            return yam_vid, None, steps
        nv_vid = g1_nv_vidhilin_stem(dhatu)
        if nv_vid and cgana == 1:
            _append_step(steps, nv_vid, ["7.3.84"], "vidhilin_stem")
            return nv_vid, None, steps
        if dhatu in _G1_A_FINAL and cgana == 1:
            root = dhatu[:-1]
            _append_step(steps, root, ["3.4.104"], "vidhilin_stem")
            return root, None, steps
        if gana in CAUSATIVE_GANAS:
            root = causative_vidhilin_stem(dhatu, tags)
            _append_step(steps, root, ["3.4.104"], "vidhilin_stem")
            return root, None, steps
        if gana == GANA3:
            root = gana3_vidhilin_stem(dhatu, guna)
            _append_step(steps, root, ["3.4.104"], "vidhilin_stem")
            return root, None, steps
        if gana in AD_GANAS:
            root = g2_vidhilin_stem(dhatu)
            _append_step(steps, root, ["3.4.104"], "vidhilin_stem")
            return root, None, steps
        if gana == YA_GANA and present_stem:
            root = present_stem[:-1] if present_stem.endswith("a") else present_stem
        elif cgana in THEMATIC_GANAS:
            aya_stem = thematic_aya_present_stem(dhatu) if cgana == 1 else None
            if cgana == 6:
                root = g6_vidhilin_stem(dhatu)
            elif aya_stem:
                root = aya_stem[:-1]
            else:
                root = thematic_present_base(dhatu, cgana, aupadeshik)
        elif gana in NU_GANAS:
            base = present_stem[:-1] if present_stem and present_stem.endswith("u") else dhatu
            root = base + "uy"
        elif gana == NI_GANA:
            root = g9_vidhilin_stem(dhatu, antarganas)
        elif gana == N_GANA:
            if dhatu.endswith("D"):
                root = dhatu[:-1] + "nD"
            else:
                root = g7_vidhilin_stem(dhatu)
        _append_step(steps, root, ["3.4.104"], "vidhilin_stem")
        return root, None, steps

    if family == "lit" and gana == GANA3:
        pstem = gana3_perfect_stem(dhatu, guna)
        _append_step(steps, pstem, ["6.1.1"], "lit")
        return pstem, None, steps

    if family == "lit":
        grade = (
            apply_guna_to_stem(dhatu)
            if cgana in THEMATIC_GANAS and thematic_present_base(dhatu, cgana, aupadeshik) != dhatu
            else (guna if cgana not in THEMATIC_GANAS else dhatu)
        )
        pstem = perfect_stem(dhatu, grade)
        _append_step(steps, pstem, ["6.1.1"], "lit")
        return pstem, None, steps

    return present_stem, None, steps
