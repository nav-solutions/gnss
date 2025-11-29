from gnss import Constellation, SV, TimeScale

def test_sv():
    gps = Constellation.GPS
    assert "{}".format(gps), "GPS (US)"
    assert "{:x}".format(gps), "GPS" # drop country code
    
    # smart builder
    assert Constellation.from_country_code("US"), Constellation.GPS
    
    # PRN# is not checked, it is up to you to create valid satellites.
    sat = SV(Constellation.GPS, 10)
    assert sat.prn == 10
    assert sat.timescale() == TimeScale.GPST

    sat.constellation = Constellation.BeiDou
    assert "{}".format(sat.constellation, "BeiDou (CH)")
    assert "{:x}".format(sat.constellation, "BDS") # drop country code
    assert sat.timescale() == TimeScale.BDT

