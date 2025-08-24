from gnss import Constellation, SV

if __name__ == "__main__":
    g01 = SV(Constellation.GPS, 1)
    assert g01.prn == 1
    assert g01.constellation == Constellation.GPS
    print("G01: ", g01)
