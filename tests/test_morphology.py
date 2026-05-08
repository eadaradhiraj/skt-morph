# tests/test_morphology.py
import unittest
from sktmorph.morphology import SktMorph, apply_forward_sandhi

class TestSktMorph(unittest.TestCase):

    @classmethod
    def setUpClass(cls):
        # Initializes the SktMorph analyzer, which automatically finds the bundled SQLite DB
        cls.morph = SktMorph()

    def test_forward_sandhi_vowels(self):
        """Test Sandhi rules for generating words."""
        self.assertEqual(apply_forward_sandhi('pra', 'aBavat'), 'prABavat')
        self.assertEqual(apply_forward_sandhi('pra', 'iRvat'), 'preRvat')
        self.assertEqual(apply_forward_sandhi('vi', 'Akaroti'), 'vyAkaroti')
        self.assertEqual(apply_forward_sandhi('anu', 'aBavat'), 'anvaBavat')
        self.assertEqual(apply_forward_sandhi('ud', 'harati'), 'uddharati')

    def test_analyzer_base_verb(self):
        """Test analyzing a base verb without prefixes."""
        # bhavati -> Root 01.0001
        res = self.morph.analyze('Bavati')
        self.assertTrue(len(res) > 0)
        self.assertEqual(res[0].prefixes, [])
        self.assertEqual(res[0].dhatu_id, '01.0001')
        self.assertEqual(res[0].word_type, 'tinanta')
        self.assertEqual(res[0].lakara, 'plat')

    def test_analyzer_single_prefix(self):
        """Test analyzing a word with one prefix."""
        # prabhavati = pra + bhavati
        res = self.morph.analyze('praBavati')
        valid_res =[r for r in res if r.prefixes == ['pra'] and r.dhatu_id == '01.0001']
        self.assertTrue(len(valid_res) > 0)
        
        # prAbhavat = pra + abhavat
        res2 = self.morph.analyze('prABavat')
        valid_res2 = [r for r in res2 if r.prefixes == ['pra'] and r.dhatu_id == '01.0001']
        self.assertTrue(len(valid_res2) > 0)

    def test_analyzer_krdanta(self):
        """Test analyzing a derivative noun (Krdanta)."""
        # bhavanam -> lyut pratyaya
        res = self.morph.analyze('Bavanam')
        valid_res =[r for r in res if r.word_type == 'krdanta' and r.pratyaya == 'lyuw']
        self.assertTrue(len(valid_res) > 0)

    def test_generator(self):
        """Test generating words from database with attached prefixes."""
        forms = self.morph.generate_tinanta('01.0001', 'plat', 1, 1, prefixes=['pra'])
        self.assertIn('praBavati', forms)

        forms_past = self.morph.generate_tinanta('01.0001', 'plang', 1, 1, prefixes=['pra'])
        self.assertIn('prABavat', forms_past)

if __name__ == '__main__':
    unittest.main()