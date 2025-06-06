use crate::GeoTypes;


#[test]
fn geometric_types_containment() {
    let point1 = GeoTypes::point(1, 2);
    let point2 = GeoTypes::point(-0.5, 0.5);

    let circle = GeoTypes::radius(2.0, (1.0, 0.0));

    assert!(circle.contains(point1));
    assert!(circle.contains(point2));

    let aabb = GeoTypes::rect((0,0), (10,10));

    assert!(aabb.contains(point1));
    assert!(aabb.contains(point2));
    assert!(aabb.contains(circle));

    let circle2 = GeoTypes::radius(10, (0,0));

    assert!(circle2.contains(aabb));
    assert!(circle2.contains(point1));
    assert!(circle2.contains(point2));
    assert!(circle2.contains(circle));

    assert!(!circle.contains(aabb));
    assert!(!circle.contains(circle2));

    let aabb_2 = GeoTypes::rect((0,0), (20,20));

    assert!(!aabb.contains(aabb_2));
    assert!(aabb_2.contains(aabb));
    
}

#[test]
fn geometric_types_intersections() {
    let circle_1 = GeoTypes::radius(2, (0,0));
    let circle_2 = GeoTypes::radius(2, (1,0));

    assert!(circle_1.intersects(circle_2));
    assert!(circle_2.intersects(circle_1));

    let aabb_1 = GeoTypes::rect((0,0), (4,4));
    let aabb_2 = GeoTypes::rect((3,3), (4,4));

    assert!(aabb_1.intersects(aabb_2));
    assert!(aabb_2.intersects(aabb_1));

    let circle = GeoTypes::radius(1, (2,0));
    let aabb = GeoTypes::rect((0,0), (2,2));

    assert!(aabb.intersects(circle));
    assert!(circle.intersects(aabb));
}
