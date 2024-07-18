# GeoPapyrus

Small python project, that binds 2 functions from rust library [geo](https://github.com/georust/geo): 
- distance_haversine_m
- distance_geodesic_m

### Usage
Install: `pip install geopapyrus`

```python
import geopapyrus

res = geopapyrus.distance_haversine_m(
    55.793246, 37.799445, 55.803140, 37.798920
)

assert res == 1100.3792724609375

res = geopapyrus.distance_geodesic_m(
    55.793246, 37.799445, 53.361012, 58.958361
)

assert res == 1388998.3696851355
```
