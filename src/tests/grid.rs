use crate::hashgrid::{Boundary, Coordinate, Entity, HashGrid, Query, QueryType};

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

impl Coordinate for [f32; 2] {
    fn new(x: f32, y: f32, _z: f32) -> Self {
        [x, y]
    }

    fn x(&self) -> f32 {
        self[0]
    }

    fn y(&self) -> f32 {
        self[1]
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
    type Position = [f32; 2];

    fn id(&self) -> u32 {
        self.id
    }

    fn position(&self) -> [f32; 2] {
        self.position
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
    assert_eq!(hashgrid_3d.bounds.boundary_max(), [500_f32; 3]);
    assert_eq!(hashgrid_3d.bounds.boundary_min(), [-500_f32; 3]);

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
    assert_eq!(
        hashgrid_2d.bounds.boundary_max(),
        [500_f32, 500_f32, 0_f32,]
    );
    assert_eq!(
        hashgrid_2d.bounds.boundary_min(),
        [-500_f32, -500_f32, 0_f32,]
    );

    // uncomment the line to print the hashgrid
    // println!("{hashgrid_2d}");
}

#[test]
fn data_insertion_2d() {
    let bounds_2d = Bounds {
        centre: [0_f32; 3],
        size: [1000_f32, 1000_f32, 0_f32],
    };

    let mut hashgrid_2d = HashGrid::<f32, Player2D>::new([2, 2], 0, &bounds_2d, false);


    let player1 = Player2D::new(0, [400.0, 400.0]);
    let player2 = Player2D::new(1, [-400.0, 400.0]);

    
    let player3 = Player2D::new(2, [-400.0, -400.0]);
    let player4 = Player2D::new(3, [400.0, -400.0]);

    hashgrid_2d.insert(&player1);
    hashgrid_2d.insert(&player2);
    hashgrid_2d.insert(&player3);
    hashgrid_2d.insert(&player4);

    // uncomment the line to print the hashgrid
    println!("{hashgrid_2d}");

    let query = Query {
        coordinates: [-490.0, 490.0, 0.0],
        ty: QueryType::Single,
        radius: 0.0,
    };

    let res = hashgrid_2d.query(query);
    let cell_hash = hashgrid_2d.get_cell_coordinates(-490.0, 490.0, 0.0);

    println!("cell cords: {} {}", cell_hash.0, cell_hash.1);
    println!("{res} cell hash: {:?}", hashgrid_2d.key(cell_hash.0, cell_hash.1));

    let query = Query {
        coordinates: [490.0, -490.0, 0.0],
        ty: QueryType::Single,
        radius: 0.0,
    };

    let cell_hash = hashgrid_2d.get_cell_coordinates(490.0, -490.0, 0.0);

    let res = hashgrid_2d.query(query);

    println!("cell cords: {} {}", cell_hash.0, cell_hash.1);
    println!("{res} cell hash: {:?}", hashgrid_2d.key(cell_hash.0, cell_hash.1));

    println!("{:?}", hashgrid_2d.grids[0])
}
