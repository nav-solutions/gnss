from gnss import Constellation, SV

def test_gps_sv():
    g01 = SV(Constellation.GPS, 1)
    assert g01.py_get_prn() == 1
