import unittest
from sktmorph.sarvanama import SarvanamaGenerator

class TestSarvanama(unittest.TestCase):
    def setUp(self):
        self.gen = SarvanamaGenerator()

    def test_generate_tad(self):
        tad = self.gen.generate('tad', 'pum')
        self.assertEqual(tad['prathamA'], ['saH', 'tO', 'te'])
        self.assertEqual(tad['caturTI'],['tasmE', 'tAByAm', 'teByaH'])

    def test_generate_asmad(self):
        # Even if user passes pum, it overrides to 'any'
        asmad = self.gen.generate('asmad', 'pum')
        self.assertEqual(asmad['prathamA'], ['aham', 'AvAm', 'vayam'])
        self.assertEqual(asmad['dvitIyA'], ['mAm/mA', 'AvAm/nO', 'asmAn/naH'])

    def test_generate_idam_etad_yad(self):
        idam = self.gen.generate('idam', 'pum')
        self.assertEqual(idam['prathamA'], ['ayam', 'imO', 'ime'])
        etad = self.gen.generate('etad', 'nap')
        self.assertEqual(etad['prathamA'], ['etat', 'ete', 'etAni'])
        yad = self.gen.generate('yad', 'stri')
        self.assertEqual(yad['prathamA'], ['yA', 'ye', 'yAH'])

    def test_analyze_sarvanama(self):
        res = self.gen.analyze('te')
        # 'te' is heavily overloaded (tad pum bahu, tad stri dvi, yuzmad caturthi eka, etc.)
        self.assertTrue(any(m['pratipadika'] == 'tad' and m['linga'] == 'pum' for m in res))
        self.assertTrue(any(m['pratipadika'] == 'yuzmad' for m in res))
        self.assertTrue(any(m['pratipadika'] == 'tad' and m['linga'] == 'stri' for m in res))

        res_idam = self.gen.analyze('ayam')
        self.assertTrue(any(m['pratipadika'] == 'idam' and m['linga'] == 'pum' for m in res_idam))

    def test_unimplemented(self):
        with self.assertRaises(NotImplementedError):
            self.gen.generate('unknown', 'pum')

if __name__ == '__main__':
    unittest.main()
