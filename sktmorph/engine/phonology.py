from typing import Optional

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


def apply_guna_to_stem(stem: str) -> str:
    """Guṇa on the last vowel of a dhātu/stem (7.2.115)."""
    for idx in range(len(stem) - 1, -1, -1):
        repl = GUNA_MAP.get(stem[idx])
        if repl is not None:
            return stem[:idx] + repl + stem[idx + 1 :]
    return stem


def thematic_join(stem_a: str, ending: str) -> str:
    """Join a thematic stem (ending in 'a') to a tinanta ending with sandhi."""
    if not stem_a.endswith("a"):
        return stem_a + ending
    if ending.startswith("a"):
        return stem_a + ending[1:]
    if ending.startswith("A"):
        return stem_a[:-1] + ending
    return stem_a + ending
