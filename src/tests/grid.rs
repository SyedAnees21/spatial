use crate::hashgrid::{Boundary, Coordinate, Entity, HashGrid, Query, QueryType};

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

#[derive(Debug, PartialEq, PartialOrd)]
struct Player2D {
    id: u32,
    position: [f32; 2],
}

impl Player2D {
    fn new(id: u32, position: [f32; 2]) -> Self {
        Self { id, position }
    }
}

impl Entity for Player2D {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl Coordinate for Player2D {
    type Item = f32;
    fn x(&self) -> Self::Item {
        self.position[0]
    }

    fn y(&self) -> Self::Item {
        self.position[1]
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
    // println!("{hashgrid_3d}");

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
    // println!("{hashgrid_2d}");
}

#[test]
fn data_insertion_2d() {
    let bounds_2d = Bounds {
        centre: [0_f32; 3],
        size: [100_f32, 100_f32, 0_f32],
    };

    let mut hashgrid_2d = HashGrid::<f32, Player2D>::new([2, 2], 0, &bounds_2d, true);

    // asserting the initialized grid parameters
    assert_eq!(hashgrid_2d.cell_size_x(), 50_f32);
    assert_eq!(hashgrid_2d.cell_size_y(), 50_f32);
    assert_eq!(hashgrid_2d.floor_size(), 1_f32);

    // asserting the initialized grid boundary parameters
    assert_eq!(hashgrid_2d.bounds.max(), [50_f32, 50_f32, 0_f32,]);
    assert_eq!(hashgrid_2d.bounds.min(), [-50_f32, -50_f32, 0_f32,]);

    let player1 = Player2D::new(0, [22.5, 30.0]);
    let player2 = Player2D::new(2, [15.5, 45.6]);

    hashgrid_2d.insert(&player1);
    hashgrid_2d.insert(&player2);

    // uncomment the line to print the hashgrid
    println!("{hashgrid_2d}");

    let query = Query {
        coordinates: (10.0, 10.0, 0.0),
        ty: QueryType::Relevant,
        radius: 0.0,
    };

    let res = hashgrid_2d.query(query);

    println!("{res}");
}
