"""Live kṛdanta (participle / secondary stem) derivation."""

import json
import re
import sqlite3
from typing import Any, Dict, List, Optional, Tuple

from .phonology import apply_guna_to_stem
from .steps import EngineStep

# pratyaya -> (suffix, sutras, stem_mode)
# stem_mode: guna | guna_a | root | present | kta_base
PRATYAYA_RULES = {
    "Satf": ("t", ["3.2.124"], "present"),
    "Satf~": ("", ["3.2.124"], "present"),
    "kta": ("ta", ["3.2.102"], "kta"),
    "ktavatu": ("vat", ["3.2.171"], "kta"),
    "ktavatu~": ("", ["3.2.171"], "kta"),
    "lyuw": ("ana", ["3.3.115"], "guna"),
    "lyu": ("ana", ["3.3.115"], "guna"),
    "tumun": ("tum", ["3.3.158"], "guna_tum"),
    "ktvA": ("tvA", ["3.4.21"], "root"),
    "ac": ("", ["3.3.56"], "guna_a"),
    "ktin": ("ti", ["3.3.94"], "guna"),
    "yat": ("ya", ["3.2.187"], "guna"),
    "Ryat": ("ya", ["3.2.187"], "guna"),
    "GaY": ("a", ["3.3.67"], "guna"),
    "Ramul": ("am", ["3.3.84"], "guna"),
    "Rvul": ("aka", ["3.2.104"], "guna"),
    "vun": ("aka", ["3.2.104"], "guna"),
    "anIyar": ("anIya", ["3.2.96"], "present"),
    "tavya": ("tavya", ["3.1.96"], "guna_tavya"),
    "tfc": ("tf", ["3.3.92"], "guna"),
    "SAnac": ("mAna", ["3.2.124"], "present"),
    "cAnaS": ("mAna", ["3.2.124"], "present"),
    "gsnu": ("zRu", ["3.2.94"], "root"),
    "kvasu": ("vas", ["3.2.94"], "lit"),
    "lyap": ("ya", ["3.2.187"], "lyap"),
    "ukaY": ("uka", ["3.2.74"], "guna"),
    "a": ("", ["3.3.56"], "guna_a"),
    "kyap": ("", ["3.3.56"], "guna_a"),
}

# Aliases and prefixed variants in bundled DB
for _base, _aliases in {
    "Satf": ("sya-Satf",),
    "Satf~": ("sya-Satf~",),
    "SAnac": ("sya-SAnac", "BAvakarma-SAnac", "sya-BAvakarma-SAnac"),
    "cAnaS": ("sya-cAnaS",),
}.items():
    for alias in _aliases:
        if alias not in PRATYAYA_RULES and _base in PRATYAYA_RULES:
            PRATYAYA_RULES[alias] = PRATYAYA_RULES[_base]


class LiveKrdantaEngine:
    """Derive krdanta forms from dhātu + kṛt pratyaya."""

    def __init__(self, conn_dhatus: sqlite3.Connection):
        self.conn_dhatus = conn_dhatus
        if not isinstance(conn_dhatus.row_factory, type(sqlite3.Row)):
            conn_dhatus.row_factory = sqlite3.Row

    def load_dhatu(self, dhatu_query: str) -> Optional[Dict[str, Any]]:
        if re.match(r"^\d{2}\.\d{4}$", dhatu_query):
            row = self.conn_dhatus.execute(
                "SELECT dhatu_id, details_json FROM dhatus WHERE dhatu_id = ?",
                (dhatu_query,),
            ).fetchone()
        else:
            row = self.conn_dhatus.execute(
                "SELECT dhatu_id, details_json FROM dhatus WHERE details_json LIKE ? LIMIT 1",
                (f'%"dhatu": "{dhatu_query}"%',),
            ).fetchone()
        if not row:
            return None
        details = json.loads(row["details_json"])
        return {
            "dhatu_id": row["dhatu_id"],
            "dhatu": details.get("dhatu", ""),
            "gana": int(details.get("gana") or 1),
            "pada": details.get("pada") or "P",
        }

    def _present_stem(self, dhatu: str, gana: int) -> str:
        guna = apply_guna_to_stem(dhatu)
        if gana == 10:
            return guna + "aya"
        if gana == 4:
            for idx in range(len(dhatu) - 1, -1, -1):
                if dhatu[idx] in "iIuUfF":
                    long_v = {"i": "I", "u": "U", "f": "F"}.get(dhatu[idx], dhatu[idx])
                    return dhatu[:idx] + long_v + dhatu[idx + 1 :] + "ya"
            return guna + "ya"
        if gana in (1, 6):
            base = dhatu if gana == 6 else guna
            return base + "a"
        return guna

    def _kta_stem(self, dhatu: str) -> str:
        if len(dhatu) >= 2 and dhatu[-1] in "iIuUfF":
            return dhatu + "ta"
        guna = apply_guna_to_stem(dhatu)
        return guna + "ta"

    def derive(
        self,
        dhatu_query: str,
        pratyaya: str,
        derivation: str = "shuddha",
    ) -> Tuple[Optional[str], List[EngineStep]]:
        info = self.load_dhatu(dhatu_query)
        if not info:
            return None, []

        rule = PRATYAYA_RULES.get(pratyaya)
        if not rule or derivation != "shuddha":
            steps = [EngineStep(info["dhatu"], ["1.3.1"], "dhatu")]
            return None, steps

        suffix, sutras, mode = rule
        steps: List[EngineStep] = [EngineStep(info["dhatu"], ["1.3.1"], "dhatu")]
        dhatu = info["dhatu"]
        guna = apply_guna_to_stem(dhatu)
        if guna != dhatu:
            steps.append(EngineStep(guna, ["7.2.115"], "guNa"))

        if mode == "present":
            base = self._present_stem(dhatu, info["gana"])
            steps.append(EngineStep(base, ["3.1.68"], "sap"))
            if pratyaya == "Satf":
                form = base[:-1] + "at" if base.endswith("a") else base + "at"
            elif pratyaya == "Satf~":
                form = base[:-1] + "n" if base.endswith("a") else base + "ant"
            elif pratyaya in ("SAnac", "cAnaS"):
                form = base[:-1] + "mAna" if base.endswith("a") else base + "mAna"
            else:
                form = base + suffix
        elif mode == "kta":
            base = self._kta_stem(dhatu)
            steps.append(EngineStep(base, ["3.2.102"], "kta"))
            if pratyaya.startswith("ktavatu"):
                form = base + "vat"
            else:
                form = base
        elif mode == "guna":
            form = guna + suffix
        elif mode == "guna_a":
            form = guna + "a"
        elif mode == "guna_tum":
            form = guna + "itum" if not guna.endswith("a") else guna[:-1] + "itum"
        elif mode == "guna_tavya":
            form = guna + "itavya" if not guna.endswith("a") else guna[:-1] + "itavya"
        elif mode == "root":
            form = dhatu + suffix
        elif mode == "lit":
            form = dhatu[0] + "a" + dhatu + suffix
        elif mode == "lyap":
            form = dhatu + suffix
            if dhatu == "BU":
                form = "pra" + dhatu + suffix
        else:
            form = guna + suffix

        steps.append(EngineStep(form, sutras, "krdanta", {"pratyaya": pratyaya}))
        return form, steps

    def generate_all(
        self,
        dhatu_query: str,
        pratyaya: str,
        derivation: str = "shuddha",
    ) -> Tuple[List[str], List[EngineStep]]:
        form, steps = self.derive(dhatu_query, pratyaya, derivation)
        return ([form] if form else []), steps
