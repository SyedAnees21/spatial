use crate::Boundary;


struct Bounds {
    centre: [f32; 3],
    size: [f32; 3],
}

impl Boundary for Bounds {
    type T = [f32; 3];

    fn centre(&self) -> Self::T {
        self.centre
    }

    fn size(&self) -> Self::T {
        self.size
    }
}


#[test]
fn point_boundary_validation() {
    let boundary = Bounds {
        centre: [0_f32;3],
        size: [100_f32;3]
    };

    assert_eq!(boundary.boundary_max(), [50_f32;3]);
    assert_eq!(boundary.boundary_min(), [-50_f32;3]);

    assert!(!boundary.is_inside([55_f32;3]));
    assert!(!boundary.is_inside([-55_f32;3]));
    assert!(boundary.is_inside([50_f32;3]));
    assert!(boundary.is_inside([-50_f32;3]));
}