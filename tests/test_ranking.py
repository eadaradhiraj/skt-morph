import unittest

from sktmorph.morphology import MorphResult
from sktmorph.ranking import rank_results, score_result


class TestRanking(unittest.TestCase):
    def _result(self, **kwargs):
        defaults = dict(
            word="x",
            prefixes=[],
            dhatu=None,
            word_type="subanta",
            derivation=None,
            pratipadika="rAma",
            linga="pum",
            vibhakti="prathamA",
            vacana=1,
            prakriya=None,
            confidence=None,
        )
        defaults.update(kwargs)
        return MorphResult(**defaults)

    def test_sarvanama_ranks_highest(self):
        sub = self._result(word="saH", word_type="subanta", pratipadika="sa")
        pron = self._result(word="saH", word_type="sarvanama", pratipadika="tad")
        ranked = rank_results([sub, pron])
        self.assertEqual(ranked[0].word_type, "sarvanama")
        self.assertIsNotNone(ranked[0].confidence)

    def test_known_pronoun_form_penalty(self):
        sub = self._result(word="saH", word_type="subanta", pratipadika="sa")
        score = score_result(sub)
        self.assertLess(score, 60)

    def test_short_pratipadika_penalty(self):
        sub = self._result(pratipadika="a", word_type="subanta")
        score = score_result(sub)
        self.assertLess(score, score_result(self._result(pratipadika="rAma")))

    def test_stri_a_ending_penalty(self):
        sub = self._result(pratipadika="rAmA", linga="stri", word_type="subanta")
        score = score_result(sub)
        self.assertLess(score, score_result(self._result(pratipadika="rAmA", linga="pum")))

    def test_taddhita_bonus(self):
        tadd = self._result(word_type="taddhita", pratyaya="tva", pratipadika="rAma")
        short = self._result(word_type="taddhita", pratyaya="tva", pratipadika="a")
        self.assertGreater(score_result(tadd), score_result(short))

    def test_analyze_results_have_confidence(self):
        from sktmorph.morphology import SktMorph

        morph = SktMorph()
        results = morph.analyze("saH")
        self.assertTrue(all(r.confidence is not None for r in results))

    def test_tinanta_no_prefix_bonus(self):
        verb = self._result(word_type="tinanta", dhatu="01.0001", prefixes=[])
        prefixed = self._result(word_type="tinanta", dhatu="01.0001", prefixes=["pra"])
        self.assertGreater(score_result(verb), score_result(prefixed))

    def test_prakriya_bonus(self):
        plain = self._result(word_type="taddhita", pratyaya="tva")
        traced = self._result(
            word_type="taddhita",
            pratyaya="tva",
            prakriya=[{"step": "a"}, {"step": "b"}, {"step": "c"}, {"step": "d"}, {"step": "e"}, {"step": "f"}],
        )
        self.assertGreater(score_result(traced), score_result(plain))

    def test_unknown_type_default_score(self):
        other = self._result(word_type="unknown")
        self.assertEqual(score_result(other), 50.0)


if __name__ == "__main__":
    unittest.main()
