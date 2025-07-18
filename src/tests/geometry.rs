use crate::Geometry;


#[test]
fn geometric_types_containment() {
    let point1 = Geometry::point(1, 2);
    let point2 = Geometry::point(-0.5, 0.5);

    let circle = Geometry::radius(2.0, (1.0, 0.0));

    assert!(circle.contains(point1));
    assert!(circle.contains(point2));

    let aabb = Geometry::rect((0,0), (10,10));

    assert!(aabb.contains(point1));
    assert!(aabb.contains(point2));
    assert!(aabb.contains(circle));

    let circle2 = Geometry::radius(10, (0,0));

    assert!(circle2.contains(aabb));
    assert!(circle2.contains(point1));
    assert!(circle2.contains(point2));
    assert!(circle2.contains(circle));

    assert!(!circle.contains(aabb));
    assert!(!circle.contains(circle2));

    let aabb_2 = Geometry::rect((0,0), (20,20));

    assert!(!aabb.contains(aabb_2));
    assert!(aabb_2.contains(aabb));
    
}

#[test]
fn geometric_types_intersections() {
    let circle_1 = Geometry::radius(2, (0,0));
    let circle_2 = Geometry::radius(2, (1,0));

    assert!(circle_1.intersects(circle_2));
    assert!(circle_2.intersects(circle_1));

    let aabb_1 = Geometry::rect((0,0), (4,4));
    let aabb_2 = Geometry::rect((3,3), (4,4));

    assert!(aabb_1.intersects(aabb_2));
    assert!(aabb_2.intersects(aabb_1));

    let circle = Geometry::radius(1, (2,0));
    let aabb = Geometry::rect((0,0), (2,2));

    assert!(aabb.intersects(circle));
    assert!(circle.intersects(aabb));
}
