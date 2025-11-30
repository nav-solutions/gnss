from gnss import COSPAR

def test_cospar():
    year, launch_num, launch_code = (2023, 1, "001")
    cospar = COSPAR(year, launch_num, launch_code)
    assert "{}".format(cospar), "GPS (US)"
