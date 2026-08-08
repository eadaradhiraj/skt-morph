import json
import re
import sqlite3
from typing import Any, Dict, List, Optional, Tuple

from .endings import family_endings
from .join import join_variants
from .lakara import lakara_family, normalize_lakara
from .stems import derive_stem
from .steps import EngineStep


class LiveTinantaEngine:
    """Derive tinanta forms by applying Pāṇinian-style rules (not SQLite lookup)."""

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
            if row:
                return self._row_to_info(row)
            return None

        row = self.conn_dhatus.execute(
            "SELECT dhatu_id, details_json FROM dhatus WHERE details_json LIKE ? LIMIT 1",
            (f'%"dhatu": "{dhatu_query}"%',),
        ).fetchone()
        if row:
            return self._row_to_info(row)
        return None

    def _row_to_info(self, row) -> Dict[str, Any]:
        details = json.loads(row["details_json"])
        return {
            "dhatu_id": row["dhatu_id"],
            "dhatu": details.get("dhatu", ""),
            "gana": int(details.get("gana") or 1),
            "pada": details.get("pada") or "P",
            "raw": details,
        }

    def derive_present_stem(
        self,
        dhatu: str,
        gana: int,
        derivation: str,
    ) -> Tuple[Optional[str], List[EngineStep]]:
        stem, _aug, steps = derive_stem(dhatu, gana, "lat", derivation)
        return stem, steps

    def generate(
        self,
        dhatu_query: str,
        lakara: str,
        purusha: int,
        vacana: int,
        derivation: str = "shuddha",
        prayoga: str = "kartari",
    ) -> Tuple[Optional[str], List[EngineStep]]:
        forms, steps = self.generate_all(
            dhatu_query, lakara, purusha, vacana, derivation, prayoga
        )
        return (forms[0] if forms else None), steps

    def generate_all(
        self,
        dhatu_query: str,
        lakara: str,
        purusha: int,
        vacana: int,
        derivation: str = "shuddha",
        prayoga: str = "kartari",
    ) -> Tuple[List[str], List[EngineStep]]:
        info = self.load_dhatu(dhatu_query)
        if not info:
            return [], []

        canonical, db_lakara = normalize_lakara(lakara)
        family = lakara_family(db_lakara)
        if not family:
            return [], []

        pada = "A" if db_lakara.startswith("a") or canonical.startswith("a") else "P"
        if db_lakara == "plit":
            pada = "P"

        root_pada = info["pada"]
        if root_pada == "P" and pada == "A" and prayoga == "kartari":
            return [], []
        if root_pada == "A" and pada == "P" and prayoga == "kartari":
            return [], []

        stem, augment, steps = derive_stem(info["dhatu"], info["gana"], family, derivation)
        if not stem:
            return [], steps

        table = family_endings(family, prayoga, pada, info["gana"], info["dhatu"])
        if not table:
            return [], steps

        idx = (purusha - 1) * 3 + (vacana - 1)
        if idx >= len(table):
            return [], steps

        variants, sutras = table[idx]
        forms = join_variants(
            stem,
            variants,
            info["gana"],
            family,
            purusha,
            pada,
            augment,
            info["dhatu"],
            vacana,
        )
        for form in forms:
            steps.append(
                EngineStep(
                    form,
                    sutras,
                    "tinanta",
                    {
                        "lakara": canonical,
                        "purusha": purusha,
                        "vacana": vacana,
                        "prayoga": prayoga,
                    },
                )
            )
        return forms, steps

    def generate_paradigm(
        self,
        dhatu_query: str,
        lakara: str,
        derivation: str = "shuddha",
        prayoga: str = "kartari",
    ) -> Tuple[List[str], List[EngineStep]]:
        forms: List[str] = []
        all_steps: List[EngineStep] = []
        for purusha in (1, 2, 3):
            for vacana in (1, 2, 3):
                cell_forms, steps = self.generate_all(
                    dhatu_query, lakara, purusha, vacana, derivation, prayoga
                )
                forms.extend(cell_forms)
                all_steps.extend(steps)
        return forms, all_steps
