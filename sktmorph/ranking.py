"""Rank morphological analyses to surface the most likely parse first."""
from typing import Any, List

KNOWN_PRONOUN_FORMS = {
    "ayam", "iyam", "idam", "enam", "ime", "imAni", "asmin", "asya",
    "saH", "sA", "tat", "te", "yaH", "yA", "yat", "ezaH", "etat",
    "aham", "tvam", "vayam", "sarvaH", "sarvA", "kaH", "kA", "kim",
}

TYPE_BASE_SCORE = {
    "sarvanama": 100,
    "tinanta": 90,
    "krdanta": 85,
    "taddhita": 80,
    "subanta": 60,
}


def score_result(result: Any) -> float:
    score = float(TYPE_BASE_SCORE.get(result.word_type, 50))

    if result.word_type == "sarvanama":
        score += 20
    elif result.word_type == "subanta" and result.word in KNOWN_PRONOUN_FORMS:
        score -= 40
    elif result.word_type == "subanta" and result.pratipadika:
        if len(result.pratipadika) <= 2:
            score -= 15
        if result.pratipadika.endswith("A") and result.linga == "stri":
            score -= 5
    elif result.word_type == "taddhita" and result.pratyaya:
        score += 10
        if result.pratipadika and len(result.pratipadika) >= 3:
            score += 5
    elif result.word_type == "tinanta" and not result.prefixes:
        score += 5

    if result.prakriya:
        score += min(5, len(result.prakriya))

    return score


def rank_results(results: List[Any]) -> List[Any]:
    ranked = sorted(results, key=score_result, reverse=True)
    for r in ranked:
        r.confidence = score_result(r)
    return ranked
