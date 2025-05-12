use core::f32;
use std::{default, i32};

use crate::{
    hashgrid::{Boundary, Coordinate, Entity, HashGrid, Query, QueryType},
    vertex,
    // traits::{Float, FromPrimitive, Primitive, ToPrimitive},
};

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
        size: [100_f32, 100_f32, 0_f32],
    };

    let mut hashgrid_2d = HashGrid::<f32, Player2D>::new([2, 2], 0, &bounds_2d, true);

    // asserting the initialized grid parameters
    assert_eq!(hashgrid_2d.cell_size_x(), 50_f32);
    assert_eq!(hashgrid_2d.cell_size_y(), 50_f32);
    assert_eq!(hashgrid_2d.floor_size(), 1_f32);

    // asserting the initialized grid boundary parameters
    assert_eq!(hashgrid_2d.bounds.boundary_max(), [50_f32, 50_f32, 0_f32,]);
    assert_eq!(
        hashgrid_2d.bounds.boundary_min(),
        [-50_f32, -50_f32, 0_f32,]
    );

    let player1 = Player2D::new(0, [22.5, 30.0]);
    let player2 = Player2D::new(2, [15.5, 45.6]);

    hashgrid_2d.insert(&player1);
    hashgrid_2d.insert(&player2);

    // uncomment the line to print the hashgrid
    println!("{hashgrid_2d}");

    let query = Query {
        coordinates: [10.0, 10.0, 0.0],
        ty: QueryType::Relevant,
        radius: 0.0,
    };

    let res = hashgrid_2d.query(query);

    println!("{res}");
}

// fn get_pi<F: Float + FromPrimitive + ToPrimitive>() -> F {
//     F::PI
// }

// fn add_two_sqrt<FLOAT: Float>(v1: FLOAT, v2: FLOAT) -> FLOAT {
//     v1.sqrt() + v2.sqrt()
// }

struct Type;

#[test]
fn generic_floats() {
    // let a: f32 = get_pi();
    // let b = get_pi::<f64>();

    // let c = add_two_sqrt(2.0_f32, 3.0_f32);
    // let d = add_two_sqrt(2.0, 3.0);

    // println!("a:{}\nb:{}\nc:{}\nd:{}", a, b, c, d);
    use crate::{
        traits::{Float, FromPrimitive},
        Vertex,
    };

    let a = vertex!(20.0, 2, 3);
    let b = vertex!(1, 2);
    let c = vertex!();

    let a: Vec<Type> = vec![];

    let d = vertex!(f32, 1, 2, 3);
    let e = vertex!(f64, 1, 2, 3);

    // println!("{:?}", a);
}

#[derive(Default, Debug)]
struct Stack {
    elements: Vec<i32>,
    min_element: i32,
}

impl Stack {
    fn push(&mut self, x: i32) {
        if self.elements.is_empty() {
            self.elements.push(x);
            self.min_element = x;

            return;
        }

        if x < self.min_element {
            let encoded_int = 2 * x - self.min_element;
            self.elements.push(encoded_int);
            self.min_element = x;

            return;
        }

        self.elements.push(x);
    }

    fn pop(&mut self) -> Option<i32> {
        if let Some(element) = self.elements.pop() {
            if element < self.min_element {
                self.min_element = 2 * self.min_element - element;
                return Some(element);
            }

            return Some(element);
        }

        None
    }

    fn min_element(&self) -> i32 {
        self.min_element
    }
}

#[test]
fn min_Stack() {

    let mut stack = Stack::default();

    for number in [5,5,6,7,8,2,9] {
        stack.push(number);   
    }

    println!("{:?}", stack);

    stack.pop();
    stack.pop();

    println!("{:?}", stack);

}
