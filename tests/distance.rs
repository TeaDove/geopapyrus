use geopapyrus::core::distance::distance_haversine_m;
#[test]
fn test_distance_haversine_small_ok(){
    let res = distance_haversine_m(55.793246, 37.799445, 55.803140, 37.798920);

    assert_eq!(res, 1100.2217)
}

#[test]
fn test_distance_haversine_medium_ok(){
    let res = distance_haversine_m(55.793246, 37.799445, 55.759694, 37.573519);

    assert_eq!(res, 14613.36)
}

#[test]
fn test_distance_haversine_big_ok(){
    let res = distance_haversine_m(55.793246, 37.799445, 53.361012, 58.958361);

    assert_eq!(res, 1384477.4)
}