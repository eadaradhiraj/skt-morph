import unittest
from sktmorph.morphology import SktMorph

class TestPadaRestrictions(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.morph = SktMorph()

    def test_kri_vi_atmanepada_only(self):
        res_a = self.morph.analyze("vikrIRIte", allowed_types=["tinanta"])
        self.assertTrue(len(res_a) > 0)
        res_p = self.morph.analyze("vikrIRAti", allowed_types=["tinanta"])
        self.assertEqual(len(res_p), 0)

    def test_gam_sam_ubhayapadi(self):
        res_a = self.morph.analyze("saNgacCate", allowed_types=["tinanta"])
        self.assertTrue(len(res_a) > 0)
        res_p = self.morph.analyze("saNgacCati", allowed_types=["tinanta"])
        self.assertTrue(len(res_p) > 0)

    def test_stha_pra_atmanepada_only(self):
        res_a = self.morph.analyze("pratizWate", allowed_types=["tinanta"])
        self.assertTrue(len(res_a) > 0)
        res_p = self.morph.analyze("pratizWati", allowed_types=["tinanta"])
        self.assertEqual(len(res_p), 0)

    def test_stha_no_prefix_ubhayapadi(self):
        """Pāṇini 1.3.23: sTA without prefixes is effectively Ubhayapadi."""
        res_p = self.morph.analyze("tizWati", allowed_types=["tinanta"])
        self.assertTrue(len(res_p) > 0)
        res_a = self.morph.analyze("tizWate", allowed_types=["tinanta"])
        self.assertTrue(len(res_a) > 0, "Failed: tizWate should be valid due to 1.3.23.")

    def test_participles_sanac_restrictions(self):
        """Ensure participles inherit the exact same rules."""
        res_pra_stha = self.morph.analyze("pratizWamAnaH", allowed_types=["krdanta"])
        self.assertTrue(len(res_pra_stha) > 0)
        res_stha = self.morph.analyze("tizWamAnaH", allowed_types=["krdanta"])
        self.assertTrue(len(res_stha) > 0, "Failed: tizWamAnaH should be valid due to 1.3.23.")
        res_sam_gam = self.morph.analyze("saNgacCamAnaH", allowed_types=["krdanta"])
        self.assertTrue(len(res_sam_gam) > 0)

if __name__ == '__main__':
    unittest.main()
