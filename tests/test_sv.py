from gnss import SV, TimeScale

def test_gps_sv():
    g01 = SV("GPS", 1)
    assert g01.prn == 1
    assert g01.constellation == "GPS"
    assert g01.timescale() == TimeScale.GPST

    g01.constellation = "BDS"
    assert g01.constellation == "BDS"

    g01.constellation = "BeiDou"
    assert g01.constellation == "BDS"
