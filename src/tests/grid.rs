use crate::hashgrid::{Boundary, HashGrid};

struct Bounds {
    centre: [f32; 3],
    size: [f32; 3],
}

impl Boundary for Bounds {
    type Item = f32;

    fn centre(&self) -> [Self::Item; 3] {
        self.centre
    }

    fn size(&self) -> [Self::Item; 3] {
        self.size
    }
}

#[test]
fn grid_2d_3d_initialization() {
    let bounds_3d = Bounds {
        centre: [0_f32; 3],
        size: [1000_f32; 3],
    };

    let hashgrid_3d = HashGrid::<f32, ()>::new([100, 100], 2, &bounds_3d, true);

    // asserting the initialized grid parameters
    assert_eq!(hashgrid_3d.cell_size_x(), 10_f32);
    assert_eq!(hashgrid_3d.cell_size_y(), 10_f32);
    assert_eq!(hashgrid_3d.floor_size(), 500_f32);

    // asserting the initialized grid boundary parameters
    assert_eq!(hashgrid_3d.bounds.max(), [500_f32; 3]);
    assert_eq!(hashgrid_3d.bounds.min(), [-500_f32; 3]);

    // uncomment the line to print the hashgrid
    println!("HashGrid = {:#?}", hashgrid_3d);

    let bounds_2d = Bounds {
        centre: [0_f32; 3],
        size: [1000_f32, 1000_f32, 0_f32],
    };

    let hashgrid_2d = HashGrid::<f32, ()>::new([100, 100], 0, &bounds_2d, true);

    // asserting the initialized grid parameters
    assert_eq!(hashgrid_2d.cell_size_x(), 10_f32);
    assert_eq!(hashgrid_2d.cell_size_y(), 10_f32);
    assert_eq!(hashgrid_2d.floor_size(), 1_f32);

    // asserting the initialized grid boundary parameters
    assert_eq!(hashgrid_2d.bounds.max(), [500_f32, 500_f32, 0_f32,]);
    assert_eq!(hashgrid_2d.bounds.min(), [-500_f32, -500_f32, 0_f32,]);

    // uncomment the line to print the hashgrid
    // println!("HashGrid = {:#?}", hashgrid_2d);
}
