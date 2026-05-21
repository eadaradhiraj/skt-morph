import unittest
from sktmorph.subanta import SubantaGenerator, apply_natva

class TestSubanta(unittest.TestCase):
    def setUp(self):
        self.gen = SubantaGenerator()

    def test_natva_rule(self):
        self.assertEqual(apply_natva('rAm', 'ena'), 'eRa')
        self.assertEqual(apply_natva('dev', 'ena'), 'ena')
        self.assertEqual(apply_natva('', 'ena'), 'ena')
        self.assertEqual(apply_natva('dAt', 'fnA'), 'fRA')
        self.assertEqual(apply_natva('pit', 'FnAm'), 'FRAm')

    def test_halanta_generation(self):
        # 'in' ending
        gunin = self.gen.generate('guRin', 'pum')
        self.assertEqual(gunin['prathamA'], ['guRI', 'guRinO', 'guRinaH'])
        self.assertEqual(gunin['tfIyA'], ['guRinA', 'guRiByAm', 'guRiBiH'])
        
        # 'as' ending
        manas = self.gen.generate('manas', 'nap')
        self.assertEqual(manas['prathamA'],['manaH', 'manasI', 'manAMsi'])
        self.assertEqual(manas['caturTI'],['manase', 'manoByAm', 'manoByaH']) # Shows 'as' -> 'o' sandhi logic
        
        # 'at' ending
        bhavat = self.gen.generate('Bavat', 'pum')
        self.assertEqual(bhavat['prathamA'],['BavAn', 'BavantO', 'BavantaH'])
        self.assertEqual(bhavat['tfIyA'],['BavatA', 'BavadByAm', 'BavadBiH']) # Shows 't' -> 'd' sandhi logic

    def test_analyze_subanta(self):
        matches1 = self.gen.analyze('rAmeRa')
        self.assertTrue(any(m['pratipadika'] == 'rAma' and m['vibhakti'] == 'tfIyA' for m in matches1))

    def test_halanta_analysis(self):
        # BavadBiH -> Base: Bavat, pumlinga, tritiya bahuvacana
        matches1 = self.gen.analyze('BavadBiH')
        self.assertTrue(any(m['pratipadika'] == 'Bavat' and m['vibhakti'] == 'tfIyA' for m in matches1))
        
        # manoByAm -> Base: manas, napumsakalinga
        matches2 = self.gen.analyze('manoByAm')
        self.assertTrue(any(m['pratipadika'] == 'manas' and m['vacana'] == 2 for m in matches2))
        
        # kariRAm -> Base: karin, pumlinga
        matches3 = self.gen.analyze('kariRAm')
        self.assertTrue(any(m['pratipadika'] == 'karin' and m['vacana'] == 3 for m in matches3))


    def test_I_anta(self):
        nadi = self.gen.generate("nadI", "stri")
        self.assertEqual(nadi["prathamA"], ["nadI", "nadyO", "nadyaH"])
        self.assertEqual(nadi["zazWI"], ["nadyAH", "nadyoH", "nadInAm"])

    def test_unimplemented_and_edge_cases(self):
        # 'vAc' (speech) is a c-anta which is not implemented yet
        with self.assertRaises(NotImplementedError):
            self.gen.generate('vAc', 'stri')
        self.assertIsNone(self.gen.generate('', 'pum'))

if __name__ == '__main__':
    unittest.main()
