"""Shared constants for CLI and morphology."""

# Lakāra codes used in bundled tinanta SQLite databases (ashtadhyayi.com / vidyut).
LAKARAS = (
    "plat",
    "plrt",
    "plot",
    "plan",
    "pvidhilin",
    "alat",
    "alrt",
    "alot",
    "alan",
    "aling",
    "alit",
)

LAKARA_LABELS = {
    "plat": "present (laṭ), parasmaipada",
    "plrt": "future (lṛṭ), parasmaipada",
    "plot": "imperative (loṭ), parasmaipada",
    "plan": "optative (laṅ), parasmaipada",
    "pvidhilin": "benedictive (vidhi-liṅ), parasmaipada",
    "alat": "present (laṭ), ātmanepada",
    "alrt": "future (lṛṭ), ātmanepada",
    "alot": "imperative (loṭ), ātmanepada",
    "alan": "optative (laṅ), ātmanepada",
    "aling": "precative/benedictive, ātmanepada",
    "alit": "perfect (liṭ), ātmanepada",
}

DERIVATIONS = ("shuddha", "nich", "san", "yang", "yangluk")
PRAYOGAS = ("kartari", "karmani")
