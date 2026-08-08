import unittest

from sktmorph.prakriya import (
    merge_traces,
    trace_declension,
    trace_declension_table,
    trace_taddhita_derivation,
)


class TestPrakriya(unittest.TestCase):
    def test_trace_taddhita_derivation(self):
        steps = trace_taddhita_derivation("rAma", "tva", "rAmatva")
        self.assertEqual(len(steps), 2)
        self.assertEqual(steps[0]["step"], "rAma + tva")
        self.assertIn("5.3.23", steps[0]["sutras"])
        self.assertEqual(steps[1]["step"], "rAmatva")

    def test_trace_taddhita_unknown_pratyaya(self):
        steps = trace_taddhita_derivation("x", "zzz", "xzzz")
        self.assertEqual(steps[0]["sutras"], ["4.3.1"])

    def test_trace_declension(self):
        steps = trace_declension("rAma", "H", "rAmaH", "prathamA")
        self.assertEqual(steps[0]["vibhakti"], "prathamA")
        self.assertEqual(steps[1]["step"], "rAmaH")

    def test_merge_traces(self):
        a = trace_taddhita_derivation("nara", "tva", "naratva")
        b = trace_declension("naratva", "m", "naratvam", "prathamA")
        merged = merge_traces(a, None, b)
        self.assertEqual(len(merged), 4)
        self.assertEqual(merge_traces(None), [])

    def test_trace_declension_table(self):
        from sktmorph.subanta import SubantaGenerator

        detail = SubantaGenerator().generate_detail("rAma", "pum")
        steps = trace_declension_table(
            detail["base"], detail["ending"], detail["endings_table"], detail["declension"]
        )
        self.assertEqual(len(steps), 16)
        self.assertEqual(steps[0]["kind"], "declension")


if __name__ == "__main__":
    unittest.main()
