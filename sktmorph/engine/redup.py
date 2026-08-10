"""Gaṇa 3 reduplicating-verb stems (6.1.1, 3.1.3)."""

from dataclasses import dataclass
from typing import Optional

from .phonology import apply_guna_to_stem

GANA3 = 3


@dataclass(frozen=True)
class Gana3Profile:
    present: str
    join: str  # nu | ad
    lang: str
    vidhilin: str
    future: str


def _profile(dhatu: str, guna: str) -> Gana3Profile:
    """Return stem profile for a gaṇa-3 root."""
    if dhatu == "hu":
        return Gana3Profile("juhu", "nu", "juh", "juhuy", guna + "zya")
    if dhatu == "BI":
        return Gana3Profile("biBi", "ad", "biBi", "biBi", guna + "zya")
    if dhatu == "hrI":
        return Gana3Profile("jihrI", "ad", "jihrI", "jihrI", guna + "zya")
    if dhatu in ("pF", "pf"):
        return Gana3Profile("pipUr", "ad", "pipar", "pipUr", "parizya")
    if dhatu == "Bf":
        return Gana3Profile("biBf", "ad", "biBar", "biBf", "Barizya")
    if dhatu == "mA":
        return Gana3Profile("mimI", "ad", "mimI", "mimI", guna + "sya")
    if dhatu == "hA":
        # 03.0008 (A) vs 03.0009 (P) share orthography; infer from length/context via guna
        if guna == "hA":
            return Gana3Profile("jihI", "ad", "jihI", "jihI", guna + "sya")
        return Gana3Profile("jahA", "ad", "jah", "jah", guna + "sya")
    if dhatu == "dA":
        return Gana3Profile("dadA", "ad", "dad", "dad", guna + "sya")
    if dhatu == "DA":
        return Gana3Profile("daDA", "ad", "daD", "daD", guna + "sya")
    if dhatu == "nij":
        return Gana3Profile("nenij", "ad", "nenij", "nenij", "nekzya")
    if dhatu == "vij":
        return Gana3Profile("vevij", "ad", "vevij", "vevij", "vejizya")
    if dhatu == "viz":
        return Gana3Profile("veviz", "ad", "veviz", "veviz", "vejizya")

    # Generic fallback: duplicate with guṇa vowel in prefix syllable
    if len(dhatu) == 1:
        return Gana3Profile("j" + guna + dhatu, "nu", guna, guna + "uy", guna + "zya")
    if len(dhatu) == 2 and dhatu[0] == "h":
        return Gana3Profile("ji" + guna, "ad", "ji" + guna, "ji" + guna, guna + "zya")
    prefix = dhatu[0].lower() + "i" + dhatu[0]
    present = prefix + dhatu[1:] if len(dhatu) > 1 else prefix
    return Gana3Profile(present, "ad", present, present, guna + "zya")


def gana3_present_stem(dhatu: str, guna: Optional[str] = None) -> str:
    g = guna or apply_guna_to_stem(dhatu)
    return _profile(dhatu, g).present


def gana3_join_mode(dhatu: str, guna: Optional[str] = None) -> str:
    g = guna or apply_guna_to_stem(dhatu)
    return _profile(dhatu, g).join


def gana3_lang_stem(dhatu: str, guna: Optional[str] = None) -> str:
    g = guna or apply_guna_to_stem(dhatu)
    return _profile(dhatu, g).lang


def gana3_vidhilin_stem(dhatu: str, guna: Optional[str] = None) -> str:
    g = guna or apply_guna_to_stem(dhatu)
    return _profile(dhatu, g).vidhilin


def gana3_future_stem(dhatu: str, guna: Optional[str] = None) -> str:
    g = guna or apply_guna_to_stem(dhatu)
    return _profile(dhatu, g).future


def gana3_weak_stem(dhatu: str, guna: str, ending: str, purusha: int) -> str:
    """Person/ending-specific weak stem for reduplicated gaṇa-3."""
    prof = _profile(dhatu, guna)
    if ending == "ti" and purusha == 1:
        if dhatu == "BI":
            return "bi" + guna
        if dhatu == "hrI":
            return "jihre"
        if dhatu in ("pF", "pf"):
            return "pipa"
        if dhatu == "Bf":
            return "biBa"
    return prof.present


def gana3_perfect_stem(dhatu: str, guna: Optional[str] = None) -> str:
    g = guna or apply_guna_to_stem(dhatu)
    return _profile(dhatu, g).present
