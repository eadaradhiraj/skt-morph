import unittest
from sktmorph.subanta import SubantaGenerator, apply_natva

class TestSubanta(unittest.TestCase):
    def setUp(self):
        self.gen = SubantaGenerator()

    def test_natva_rule(self):
        self.assertEqual(apply_natva('rAm', 'ena'), 'eRa')
        self.assertEqual(apply_natva('dev', 'ena'), 'ena')
        self.assertEqual(apply_natva('mArg', 'AnAm'), 'ARAm')
        self.assertEqual(apply_natva('darSan', 'ena'), 'ena')
        self.assertEqual(apply_natva('', 'ena'), 'ena')
        self.assertEqual(apply_natva('dAt', 'fnA'), 'fRA')
        self.assertEqual(apply_natva('pit', 'FnAm'), 'FRAm')

    def test_generate_noun(self):
        rama = self.gen.generate('rAma', 'pum')
        self.assertEqual(rama['prathamA'], ['rAmaH', 'rAmO', 'rAmAH'])
        self.assertEqual(rama['tfIyA'],['rAmeRa', 'rAmAByAm', 'rAmEH'])
        
        hari = self.gen.generate('hari', 'pum')
        self.assertEqual(hari['zazWI'],['hareH', 'haryoH', 'harIRAm'])
        
        mati = self.gen.generate('mati', 'stri')
        self.assertEqual(mati['caturTI'], ['matyE/mataye', 'matiByAm', 'matiByaH'])

    def test_analyze_subanta(self):
        # rAmeRa -> Base: rAma, tfIyA, eka
        matches1 = self.gen.analyze('rAmeRa')
        self.assertTrue(any(m['pratipadika'] == 'rAma' and m['vibhakti'] == 'tfIyA' for m in matches1))
        
        # matyE -> Base: mati, caturTI, eka (Dual form handling check)
        matches2 = self.gen.analyze('matyE')
        self.assertTrue(any(m['pratipadika'] == 'mati' and m['vibhakti'] == 'caturTI' for m in matches2))

        # pitFn -> padanta check verification
        matches3 = self.gen.analyze('pitFn')
        self.assertTrue(any(m['pratipadika'] == 'pitf' and m['vacana'] == 3 for m in matches3))
        
        # Edge case: No base (word is just suffix)
        self.assertEqual(self.gen.analyze('A'),[])

    def test_unimplemented_and_edge_cases(self):
        with self.assertRaises(NotImplementedError):
            self.gen.generate('go', 'pum')
        self.assertIsNone(self.gen.generate('', 'pum'))

if __name__ == '__main__':
    unittest.main()
