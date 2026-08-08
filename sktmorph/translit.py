"""Optional Devanagari ↔ SLP1 transliteration helpers."""
from typing import Optional

try:
    from indic_transliteration import sanscript
    from indic_transliteration.sanscript import transliterate as _transliterate

    _HAS_INDIC = True
except ImportError:
    sanscript = None
    _transliterate = None
    _HAS_INDIC = False


def has_devanagari_support() -> bool:
    return _HAS_INDIC


def to_slp1(text: str) -> str:
    if not text or not _HAS_INDIC:
        return text
    if _looks_like_devanagari(text):
        return _transliterate(text, sanscript.DEVANAGARI, sanscript.SLP1)
    return text


def from_slp1(text: str) -> str:
    if not text or not _HAS_INDIC:
        return text
    return _transliterate(text, sanscript.SLP1, sanscript.DEVANAGARI)


def maybe_to_slp1(text: str, devanagari: bool = False) -> str:
    if devanagari:
        return to_slp1(text)
    return text


def maybe_from_slp1(text: str, devanagari: bool = False) -> str:
    if devanagari:
        return from_slp1(text)
    return text


def _looks_like_devanagari(text: str) -> bool:
    for char in text:
        code = ord(char)
        if 0x0900 <= code <= 0x097F:
            return True
    return False
