"""Stem formation per gaṇa and lakāra family."""

from typing import List, Optional, Tuple

from .phonology import apply_guna_to_stem
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


def _append_step(steps: List[EngineStep], form: str, sutras: List[str], kind: str) -> None:
    if not steps or steps[-1].form != form or steps[-1].kind != kind:
        steps.append(EngineStep(form, sutras, kind))


def future_stem(guna: str, gana: int, present_stem: Optional[str] = None) -> str:
    """Derive lṛṭ stem (3.2.135 etc.)."""
    if present_stem and present_stem.endswith("aya"):
        return present_stem[:-1] + "izya"
    if guna.endswith("t") and gana == 6:
        return guna + "sya"
    if gana in NU_GANAS and guna.endswith("o"):
        return guna + "zya"
    if gana == GANA3:
        return guna + "zya"
    if gana in AD_GANAS and guna[-1:] in ("d", "D", "t", "T"):
        return guna[:-1] + "tsya"
    if gana == NI_GANA:
        return guna + "zya"
    if gana == N_GANA and guna[-1:] in ("d", "D"):
        return guna[:-1] + "tsya"
    if present_stem and present_stem.endswith("ya"):
        base = present_stem[:-1]
        return base + "izya"
    if guna.endswith("v"):
        return guna + "izya"
    if present_stem and present_stem.endswith("a"):
        base = present_stem[:-1]
        if base.endswith("v"):
            return base + "izya"
        return base + "sya"
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

    present_stem: Optional[str] = None

    if gana in THEMATIC_GANAS:
        base = dhatu if gana == 6 else guna
        present_stem = base + "a"
        _append_step(steps, present_stem, ["3.1.68", "3.1.69"], "sap")
    elif gana == YA_GANA:
        ya_base = dhatu
        for idx in range(len(dhatu) - 1, -1, -1):
            if dhatu[idx] in "iIuUfF":
                long_v = {"i": "I", "u": "U", "f": "F"}.get(dhatu[idx], dhatu[idx])
                ya_base = dhatu[:idx] + long_v + dhatu[idx + 1 :]
                break
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
        present_stem = guna + "aya"
        _append_step(steps, guna + "ay", ["3.1.25"], "causal_aya")
        _append_step(steps, present_stem, ["3.1.68"], "sap")
    else:
        return None, None, steps

    if family in ("lat", "lot"):
        return present_stem, None, steps

    if family == "lrt" and gana == GANA3:
        fstem = gana3_future_stem(dhatu, guna)
        _append_step(steps, fstem, ["3.2.135"], "lrt")
        return fstem, None, steps

    if family == "lrt":
        g = apply_guna_to_stem(dhatu) if gana in (6, YA_GANA) else guna
        if gana == 6 and g.endswith("d"):
            g = g[:-1] + "t"
        fstem = future_stem(g, gana, present_stem if gana not in (YA_GANA,) else None)
        _append_step(steps, fstem, ["3.2.135"], "lrt")
        return fstem, None, steps

    if family == "lang":
        if gana in CAUSATIVE_GANAS:
            root = guna + "ay"
        elif gana == YA_GANA:
            for idx in range(len(dhatu) - 1, -1, -1):
                if dhatu[idx] in "iIuUfF":
                    long_v = {"i": "I", "u": "U", "f": "F"}.get(dhatu[idx], dhatu[idx])
                    root = dhatu[:idx] + long_v + dhatu[idx + 1 :]
                    break
            else:
                root = guna
        elif gana == 6:
            root = dhatu
        elif gana == GANA3:
            root = gana3_lang_stem(dhatu, guna)
        elif gana in NU_GANAS:
            root = present_stem[:-1] if present_stem and present_stem.endswith("u") else dhatu + "u"
        elif gana == N_GANA:
            root = dhatu[:-1] + "R" if dhatu.endswith("D") else guna
        elif gana == NI_GANA:
            root = dhatu + "R"
        else:
            root = guna
        _append_step(steps, root, ["3.4.111"], "lang_stem")
        return root, "a", steps

    if family == "vidhilin":
        if gana == YA_GANA and present_stem:
            root = present_stem[:-1] if present_stem.endswith("a") else present_stem
        elif gana == 6:
            root = dhatu
        elif gana == GANA3:
            root = gana3_vidhilin_stem(dhatu, guna)
        elif gana in NU_GANAS:
            base = present_stem[:-1] if present_stem and present_stem.endswith("u") else dhatu
            root = base + "uy"
        elif gana == NI_GANA:
            root = dhatu + "RI"
        elif gana == N_GANA and dhatu.endswith("D"):
            root = dhatu[:-1] + "nD"
        elif gana in CAUSATIVE_GANAS:
            root = guna + "ay"
        else:
            root = guna
        _append_step(steps, root, ["3.4.104"], "vidhilin_stem")
        return root, None, steps

    if family == "lit" and gana == GANA3:
        pstem = gana3_perfect_stem(dhatu, guna)
        _append_step(steps, pstem, ["6.1.1"], "lit")
        return pstem, None, steps

    if family == "lit":
        pstem = perfect_stem(dhatu, guna)
        _append_step(steps, pstem, ["6.1.1"], "lit")
        return pstem, None, steps

    return present_stem, None, steps
