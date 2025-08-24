from gnss import Constellation, SV

def test_gps_sv():
    g01 = SV("GPS", 1)
    assert g01.prn == 1
    assert g01.constellation == "GPS"

    g01.constellation = "BDS"
    assert g01.constellation == "BDS"

    g01.constellation = "BeiDou"
    assert g01.constellation == "BDS"
