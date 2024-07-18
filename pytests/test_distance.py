import geopapyrus


class TestDistance:
    def test_distance_haversine_m_ok(self):
        res = geopapyrus.distance_haversine_m(55.793246, 37.799445, 55.803140, 37.798920)

        assert res == 1100.2216796875

    def test_distance_haversine_medium_ok(self):
        res = geopapyrus.distance_haversine_m(55.793246, 37.799445, 55.759694, 37.573519)

        assert res == 14613.3603515625

    def test_distance_haversine_big_ok(self):
        res = geopapyrus.distance_haversine_m(55.793246, 37.799445, 53.361012, 58.958361)

        assert res == 1384477.375

