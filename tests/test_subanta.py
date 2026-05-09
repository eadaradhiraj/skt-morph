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
        # Suffix-based triggers
        self.assertEqual(apply_natva('dAt', 'fnA'), 'fRA')
        self.assertEqual(apply_natva('pit', 'FnAm'), 'FRAm')

    def test_a_anta_pumlinga(self):
        rama = self.gen.generate('rAma', 'pum')
        self.assertEqual(rama['prathamA'], ['rAmaH', 'rAmO', 'rAmAH'])
        self.assertEqual(rama['tfIyA'],['rAmeRa', 'rAmAByAm', 'rAmEH'])

    def test_a_anta_napumsaka(self):
        phala = self.gen.generate('Pala', 'nap')
        self.assertEqual(phala['prathamA'],['Palam', 'Pale', 'PalAni'])

    def test_A_anta_strilinga(self):
        rama_stri = self.gen.generate('ramA', 'stri')
        self.assertEqual(rama_stri['prathamA'], ['ramA', 'rame', 'ramAH'])

    def test_i_anta(self):
        hari = self.gen.generate('hari', 'pum')
        self.assertEqual(hari['zazWI'],['hareH', 'haryoH', 'harIRAm'])
        mati = self.gen.generate('mati', 'stri')
        self.assertEqual(mati['caturTI'], ['matyE/mataye', 'matiByAm', 'matiByaH'])
        vari = self.gen.generate('vAri', 'nap')
        self.assertEqual(vari['tfIyA'], ['vAriRA', 'vAriByAm', 'vAriBiH'])

    def test_u_anta(self):
        guru = self.gen.generate('guru', 'pum')
        self.assertEqual(guru['zazWI'],['guroH', 'gurvoH', 'gurURAm'])
        dhenu = self.gen.generate('Denu', 'stri')
        self.assertEqual(dhenu['caturTI'],['DenvE/Denave', 'DenuByAm', 'DenuByaH'])
        madhu = self.gen.generate('maDu', 'nap')
        self.assertEqual(madhu['tfIyA'],['maDunA', 'maDuByAm', 'maDuBiH'])

    def test_f_anta(self):
        pitr = self.gen.generate('pitf', 'pum')
        self.assertEqual(pitr['zazWI'],['pituH', 'pitroH', 'pitFRAm'])
        matr = self.gen.generate('mAtf', 'stri')
        self.assertEqual(matr['dvitIyA'],['mAtaram', 'mAtarO', 'mAtFH'])
        datr = self.gen.generate('dAtf', 'nap')
        self.assertEqual(datr['prathamA'],['dAtf', 'dAtfRI', 'dAtFRi'])

    def test_unimplemented_and_edge_cases(self):
        with self.assertRaises(NotImplementedError):
            self.gen.generate('go', 'pum')
        self.assertIsNone(self.gen.generate('', 'pum'))

if __name__ == '__main__':
    unittest.main()
