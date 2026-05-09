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

    def test_a_anta_pumlinga(self):
        rama = self.gen.generate('rAma', 'pum')
        self.assertEqual(rama['prathamA'],['rAmaH', 'rAmO', 'rAmAH'])
        self.assertEqual(rama['tfIyA'],['rAmeRa', 'rAmAByAm', 'rAmEH'])

    def test_a_anta_napumsaka(self):
        phala = self.gen.generate('Pala', 'nap')
        self.assertEqual(phala['prathamA'], ['Palam', 'Pale', 'PalAni'])

    def test_A_anta_strilinga(self):
        rama_stri = self.gen.generate('ramA', 'stri')
        self.assertEqual(rama_stri['prathamA'],['ramA', 'rame', 'ramAH'])

    def test_unimplemented_and_edge_cases(self):
        with self.assertRaises(NotImplementedError):
            self.gen.generate('hari', 'pum')
        self.assertIsNone(self.gen.generate('', 'pum'))

if __name__ == '__main__':
    unittest.main()
