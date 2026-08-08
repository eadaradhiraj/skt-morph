"""Tinanta personal endings keyed by lakāra family, prayoga, pada, gaṇa class."""

from typing import List, Optional, Tuple

from .redup import GANA3
from .stems import AD_GANAS, NU_GANAS, N_GANA, NI_GANA, YA_GANA

EndingCell = Tuple[List[str], List[str]]  # (variants, sutras)

# --- Present parasmaipada (thematic / causative / ya-gaṇa) ---
LAT_KARTARI_P = [
    (["ti"], ["3.4.78"]),
    (["taH"], ["3.4.78"]),
    (["nti"], ["3.4.78"]),
    (["si"], ["3.4.78"]),
    (["TaH"], ["3.4.78"]),
    (["Ta"], ["3.4.78"]),
    (["Ami"], ["3.4.78"]),
    (["AvaH"], ["3.4.78"]),
    (["AmaH"], ["3.4.78"]),
]

LAT_KARTARI_A = [
    (["te"], ["3.4.78"]),
    (["ete"], ["3.4.78"]),
    (["ante"], ["3.4.78"]),
    (["se"], ["3.4.78"]),
    (["eTe"], ["3.4.78"]),
    (["aDve"], ["3.4.78"]),
    (["e"], ["3.4.78"]),
    (["Avahe"], ["3.4.78"]),
    (["Amahe"], ["3.4.78"]),
]

LAT_AD_P = [
    (["ti"], ["3.1.3", "3.4.78"]),
    (["taH"], ["3.1.3", "3.4.78"]),
    (["anti"], ["3.1.3", "3.4.78"]),
    (["si"], ["3.1.3", "3.4.78"]),
    (["thaH"], ["3.1.3", "3.4.78"]),
    (["tha"], ["3.1.3", "3.4.78"]),
    (["mi"], ["3.1.3", "3.4.78"]),
    (["vaH"], ["3.1.3", "3.4.78"]),
    (["maH"], ["3.1.3", "3.4.78"]),
]

# Imperative
LOT_KARTARI_P = [
    (["tAt", "tAd", "tu"], ["3.4.78"]),
    (["tAm"], ["3.4.78"]),
    (["antu"], ["3.4.78"]),
    (["", "tAt", "tAd"], ["3.4.78"]),
    (["tam"], ["3.4.78"]),
    (["ta"], ["3.4.78"]),
    (["Ani"], ["3.4.78"]),
    (["Ava"], ["3.4.78"]),
    (["Ama"], ["3.4.78"]),
]

LOT_KARTARI_A = [
    (["atAm"], ["3.4.78"]),
    (["etAm"], ["3.4.78"]),
    (["antAm"], ["3.4.78"]),
    (["sva"], ["3.4.78"]),
    (["eTAm"], ["3.4.78"]),
    (["aDvam"], ["3.4.78"]),
    (["E"], ["3.4.78"]),
    (["AvahE"], ["3.4.78"]),
    (["AmahE"], ["3.4.78"]),
]

LOT_KARTARI_P_CAUS = [
    (["tAt", "tAd", "tu"], ["3.4.78"]),
    (["tAm"], ["3.4.78"]),
    (["antu"], ["3.4.78"]),
    (["", "tAt", "tAd"], ["3.4.78"]),
    (["tam"], ["3.4.78"]),
    (["ta"], ["3.4.78"]),
    (["ARi"], ["3.4.78"]),
    (["Ava"], ["3.4.78"]),
    (["Ama"], ["3.4.78"]),
]

LOT_NI_P = [
    (["Atu", "ItAt", "ItAd"], ["3.4.78"]),
    (["ItAm"], ["3.4.78"]),
    (["antu"], ["3.4.78"]),
    (["ItAt", "ItAd", "Ihi"], ["3.4.78"]),
    (["Itam"], ["3.4.78"]),
    (["ta"], ["3.4.78"]),
    (["Ani"], ["3.4.78"]),
    (["Ava"], ["3.4.78"]),
    (["Ama"], ["3.4.78"]),
]

LOT_NU_P = [
    (["utAt", "utAd", "otu"], ["3.1.75", "3.4.78"]),
    (["utAm"], ["3.1.75", "3.4.78"]),
    (["vantu"], ["3.1.75", "3.4.78"]),
    (["u", "utAt", "utAd"], ["3.1.75", "3.4.78"]),
    (["utam"], ["3.1.75", "3.4.78"]),
    (["uta"], ["3.1.75", "3.4.78"]),
    (["avAni"], ["3.1.75", "3.4.78"]),
    (["avAva"], ["3.1.75", "3.4.78"]),
    (["avAma"], ["3.1.75", "3.4.78"]),
]

LOT_AD_P = [
    (["tAt", "tAd", "tu"], ["3.1.3", "3.4.78"]),
    (["tAm"], ["3.1.3", "3.4.78"]),
    (["antu"], ["3.1.3", "3.4.78"]),
    (["tAt", "tAd", "Di"], ["3.1.3", "3.4.78"]),
    (["tam"], ["3.1.3", "3.4.78"]),
    (["ta"], ["3.1.3", "3.4.78"]),
    (["Ani"], ["3.1.3", "3.4.78"]),
    (["va"], ["3.1.3", "3.4.78"]),
    (["ma"], ["3.1.3", "3.4.78"]),
]

# Future (lṛṭ) — same endings as present for thematic
LRT_KARTARI_P = LAT_KARTARI_P
LRT_KARTARI_A = LAT_KARTARI_A
LRT_AD_P = [
    (["ti"], ["3.1.3", "3.4.78"]),
    (["taH"], ["3.1.3", "3.4.78"]),
    (["anti"], ["3.1.3", "3.4.78"]),
    (["si"], ["3.1.3", "3.4.78"]),
    (["TaH"], ["3.1.3", "3.4.78"]),
    (["Ta"], ["3.1.3", "3.4.78"]),
    (["Ami"], ["3.1.3", "3.4.78"]),
    (["AvaH"], ["3.1.3", "3.4.78"]),
    (["AmaH"], ["3.1.3", "3.4.78"]),
]

# Optative (laṅ) — stem is guṇa without thematic vowel; augment applied separately
LANG_KARTARI_P = [
    (["at", "ad"], ["3.4.111"]),
    (["atAm"], ["3.4.111"]),
    (["an"], ["3.4.111"]),
    (["aH"], ["3.4.111"]),
    (["atam"], ["3.4.111"]),
    (["ata"], ["3.4.111"]),
    (["am"], ["3.4.111"]),
    (["Ava"], ["3.4.111"]),
    (["Ama"], ["3.4.111"]),
]

LANG_KARTARI_A = [
    (["ata"], ["3.4.111"]),
    (["etAm"], ["3.4.111"]),
    (["anta"], ["3.4.111"]),
    (["aTAH"], ["3.4.111"]),
    (["eTAm"], ["3.4.111"]),
    (["aDvam"], ["3.4.111"]),
    (["e"], ["3.4.111"]),
    (["Avahi"], ["3.4.111"]),
    (["Amahi"], ["3.4.111"]),
]

LANG_AD_P = [
    (["at", "ad"], ["3.1.3", "3.4.111"]),
    (["atAm"], ["3.1.3", "3.4.111"]),
    (["an"], ["3.1.3", "3.4.111"]),
    (["aH"], ["3.1.3", "3.4.111"]),
    (["atam"], ["3.1.3", "3.4.111"]),
    (["ata"], ["3.1.3", "3.4.111"]),
    (["am"], ["3.1.3", "3.4.111"]),
    (["va"], ["3.1.3", "3.4.111"]),
    (["ma"], ["3.1.3", "3.4.111"]),
]

LANG_NU_P = [
    (["ot", "od"], ["3.1.75", "3.4.111"]),
    (["utAm"], ["3.1.75", "3.4.111"]),
    (["van"], ["3.1.75", "3.4.111"]),
    (["oH"], ["3.1.75", "3.4.111"]),
    (["utam"], ["3.1.75", "3.4.111"]),
    (["uta"], ["3.1.75", "3.4.111"]),
    (["avam"], ["3.1.75", "3.4.111"]),
    (["uva", "va"], ["3.1.75", "3.4.111"]),
    (["uma", "ma"], ["3.1.75", "3.4.111"]),
]

LANG_NI_P = [
    (["At", "Ad"], ["3.1.81", "3.4.111"]),
    (["ItAm"], ["3.1.81", "3.4.111"]),
    (["an"], ["3.1.81", "3.4.111"]),
    (["AH"], ["3.1.81", "3.4.111"]),
    (["Itam"], ["3.1.81", "3.4.111"]),
    (["Ita"], ["3.1.81", "3.4.111"]),
    (["Am"], ["3.1.81", "3.4.111"]),
    (["Iva"], ["3.1.81", "3.4.111"]),
    (["Ima"], ["3.1.81", "3.4.111"]),
]

# Benedictive (vidhi-liṅ)
VIDHILIN_KARTARI_P = [
    (["et", "ed"], ["3.4.104"]),
    (["etAm"], ["3.4.104"]),
    (["eyuH"], ["3.4.104"]),
    (["eH"], ["3.4.104"]),
    (["etam"], ["3.4.104"]),
    (["eta"], ["3.4.104"]),
    (["eyam"], ["3.4.104"]),
    (["eva"], ["3.4.104"]),
    (["ema"], ["3.4.104"]),
]

VIDHILIN_AD_P = [
    (["yAt", "yAd"], ["3.1.3", "3.4.104"]),
    (["yAtAm"], ["3.1.3", "3.4.104"]),
    (["yuH"], ["3.1.3", "3.4.104"]),
    (["yAH"], ["3.1.3", "3.4.104"]),
    (["yAtam"], ["3.1.3", "3.4.104"]),
    (["yAta"], ["3.1.3", "3.4.104"]),
    (["yAm"], ["3.1.3", "3.4.104"]),
    (["yAva"], ["3.1.3", "3.4.104"]),
    (["yAma"], ["3.1.3", "3.4.104"]),
]

VIDHILIN_NU_P = [
    (["At", "Ad"], ["3.1.75", "3.4.104"]),
    (["AtAm"], ["3.1.75", "3.4.104"]),
    (["uH"], ["3.1.75", "3.4.104"]),
    (["AH"], ["3.1.75", "3.4.104"]),
    (["Atam"], ["3.1.75", "3.4.104"]),
    (["Ata"], ["3.1.75", "3.4.104"]),
    (["Am"], ["3.1.75", "3.4.104"]),
    (["Ava"], ["3.1.75", "3.4.104"]),
    (["Ama"], ["3.1.75", "3.4.104"]),
]

VIDHILIN_NI_P = [
    (["yAt", "yAd"], ["3.1.81", "3.4.104"]),
    (["yAtAm"], ["3.1.81", "3.4.104"]),
    (["yuH"], ["3.1.81", "3.4.104"]),
    (["yAH"], ["3.1.81", "3.4.104"]),
    (["yAtam"], ["3.1.81", "3.4.104"]),
    (["yAta"], ["3.1.81", "3.4.104"]),
    (["yAm"], ["3.1.81", "3.4.104"]),
    (["yAva"], ["3.1.81", "3.4.104"]),
    (["yAma"], ["3.1.81", "3.4.104"]),
]

# Perfect parasmaipada (plit)
LIT_KARTARI_P = [
    (["va"], ["3.2.115"]),
    (["vatuH"], ["3.2.115"]),
    (["vuH"], ["3.2.115"]),
    (["viTa"], ["3.2.115"]),
    (["vaTuH"], ["3.2.115"]),
    (["va"], ["3.2.115"]),
    (["va"], ["3.2.115"]),
    (["viva"], ["3.2.115"]),
    (["vima"], ["3.2.115"]),
]

LIT_KARTARI_A = [
    (["e"], ["3.4.78"]),
    (["Ate"], ["3.4.78"]),
    (["ire"], ["3.4.78"]),
    (["iTe"], ["3.4.78"]),
    (["ATe"], ["3.4.78"]),
    (["iDve"], ["3.4.78"]),
    (["e"], ["3.4.78"]),
    (["i vahe"], ["3.4.78"]),
    (["i mahe"], ["3.4.78"]),
]

# nu-gaṇa present P (gaṇa 5/8) — variant endings
NU_LAT_KARTARI_P = [
    (["ti"], ["3.1.75", "3.4.78"]),
    (["taH"], ["3.1.75", "3.4.78"]),
    (["nti"], ["3.1.75", "3.4.78"]),
    (["zi"], ["3.1.75", "3.4.78"]),
    (["TaH"], ["3.1.75", "3.4.78"]),
    (["Ta"], ["3.1.75", "3.4.78"]),
    (["mi"], ["3.1.75", "3.4.78"]),
    (["vaH"], ["3.1.75", "3.4.78"]),
    (["maH"], ["3.1.75", "3.4.78"]),
]

NU_LAT_KARTARI_A = [
    (["te"], ["3.1.75", "3.4.78"]),
    (["vAte"], ["3.1.75", "3.4.78"]),
    (["vate"], ["3.1.75", "3.4.78"]),
    (["ze"], ["3.1.75", "3.4.78"]),
    (["vATe"], ["3.1.75", "3.4.78"]),
    (["uDve"], ["3.1.75", "3.4.78"]),
    (["ve"], ["3.1.75", "3.4.78"]),
    (["uvahe"], ["3.1.75", "3.4.78"]),
    (["vahe"], ["3.1.75", "3.4.78"]),
]

FAMILY_TABLES = {
    ("lat", "kartari", "P", "thematic"): LAT_KARTARI_P,
    ("lat", "kartari", "A", "thematic"): LAT_KARTARI_A,
    ("lat", "kartari", "P", "ad"): LAT_AD_P,
    ("lot", "kartari", "P", "thematic"): LOT_KARTARI_P,
    ("lot", "kartari", "A", "thematic"): LOT_KARTARI_A,
    ("lot", "kartari", "P", "ad"): LOT_AD_P,
    ("lot", "kartari", "P", "nu"): LOT_NU_P,
    ("lrt", "kartari", "P", "thematic"): LRT_KARTARI_P,
    ("lrt", "kartari", "A", "thematic"): LRT_KARTARI_A,
    ("lrt", "kartari", "P", "ad"): LRT_AD_P,
    ("lang", "kartari", "P", "thematic"): LANG_KARTARI_P,
    ("lang", "kartari", "A", "thematic"): LANG_KARTARI_A,
    ("lang", "kartari", "P", "ad"): LANG_AD_P,
    ("lang", "kartari", "P", "nu"): LANG_NU_P,
    ("vidhilin", "kartari", "P", "thematic"): VIDHILIN_KARTARI_P,
    ("vidhilin", "kartari", "P", "ad"): VIDHILIN_AD_P,
    ("vidhilin", "kartari", "P", "nu"): VIDHILIN_NU_P,
    ("lit", "kartari", "P", "thematic"): LIT_KARTARI_P,
    ("lit", "kartari", "A", "thematic"): LIT_KARTARI_A,
    ("lat", "kartari", "P", "nu"): NU_LAT_KARTARI_P,
    ("lat", "kartari", "A", "nu"): NU_LAT_KARTARI_A,
}


def gana_class(gana: int) -> str:
    if gana in AD_GANAS:
        return "ad"
    if gana in NU_GANAS:
        return "nu"
    if gana in (N_GANA, NI_GANA):
        return "thematic"
    return "thematic"


def family_endings(
    family: str,
    prayoga: str,
    pada: str,
    gana: int,
    dhatu: Optional[str] = None,
) -> Optional[List[EndingCell]]:
    if prayoga != "kartari":
        return None
    if gana == GANA3 and dhatu:
        from .phonology import apply_guna_to_stem
        from .redup import gana3_join_mode

        if gana3_join_mode(dhatu, apply_guna_to_stem(dhatu)) == "nu":
            if family == "lot":
                return LOT_NU_P
            if family == "lang":
                return LANG_NU_P
            if family == "vidhilin":
                return VIDHILIN_NU_P
            if family == "lat" and pada == "P":
                return NU_LAT_KARTARI_P
            if family == "lat" and pada == "A":
                return NU_LAT_KARTARI_A
    if family == "vidhilin" and prayoga == "kartari" and pada == "P" and gana == NI_GANA:
        return VIDHILIN_NI_P
    if family == "vidhilin" and prayoga == "kartari" and pada == "P" and gana in NU_GANAS:
        return VIDHILIN_NU_P
    if family == "lot" and prayoga == "kartari" and pada == "P" and gana == 10:
        return LOT_KARTARI_P_CAUS
    if family == "lang" and prayoga == "kartari" and pada == "P" and gana == YA_GANA:
        from .lang_ya import LANG_YA_P
        return LANG_YA_P
    if family == "lang" and prayoga == "kartari" and pada == "P" and gana == N_GANA:
        return LANG_AD_P
    if family == "vidhilin" and prayoga == "kartari" and pada == "P" and gana == N_GANA:
        return VIDHILIN_AD_P
    if family == "lang" and prayoga == "kartari" and pada == "P" and gana == NI_GANA:
        return LANG_NI_P
    if family == "lang" and prayoga == "kartari" and pada == "P" and gana in NU_GANAS:
        return LANG_NU_P
    if family == "lot" and prayoga == "kartari" and pada == "P" and gana == NI_GANA:
        return LOT_NI_P
    if family == "lot" and prayoga == "kartari" and pada == "P" and gana in NU_GANAS:
        return LOT_NU_P
    gclass = gana_class(gana)
    if gana in NU_GANAS and family == "lat":
        gclass = "nu"
    key = (family, prayoga, pada, gclass)
    table = FAMILY_TABLES.get(key)
    if table is None and gclass == "nu":
        key = (family, prayoga, pada, "thematic")
        table = FAMILY_TABLES.get(key)
    if table is None and gclass == "ad":
        return FAMILY_TABLES.get((family, prayoga, pada, "thematic"))
    return table


# Backward-compatible API (lakara code -> first variant endings)
def ending_table(lakara: str, prayoga: str, pada: str, gana: int):
    from .lakara import lakara_family, normalize_lakara

    _, db = normalize_lakara(lakara)
    family = lakara_family(db)
    if not family:
        return None
    table = family_endings(family, prayoga, pada, gana)
    if not table:
        return None
    return [(variants[0], sutras) for variants, sutras in table]
