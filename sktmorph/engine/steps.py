from dataclasses import dataclass, field
from typing import Any, Dict, List


@dataclass
class EngineStep:
    form: str
    sutras: List[str]
    kind: str
    meta: Dict[str, Any] = field(default_factory=dict)

    def to_prakriya(self) -> Dict[str, Any]:
        payload: Dict[str, Any] = {
            "step": self.form,
            "sutras": self.sutras,
            "kind": self.kind,
        }
        payload.update(self.meta)
        return payload
