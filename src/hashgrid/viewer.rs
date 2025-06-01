// use eframe::{egui, emath::Float};
// use num_traits::{FromPrimitive, ToPrimitive};
// use std::{collections::HashMap, ops::Div};

// use super::HashGrid;

// // // A mock structure for the Hashgrid to demonstrate the visualization
// // struct Hashgrid {
// //     boundary: Bounds,                 // Boundary of the grid
// //     cell_size: f32,                   // Size of each cell
// //     data: HashMap<(i32, i32), Vec2>,  // Cell coordinates to data points
// // }

// // struct Bounds {
// //     min: Vec2,
// //     max: Vec2,
// // }

// // #[derive(Clone, Copy)]
// // struct Vec2 {
// //     x: f32,
// //     y: f32,
// // }

// // impl Hashgrid {
// //     pub fn new(boundary: Bounds, cell_size: f32) -> Self {
// //         Self {
// //             boundary,
// //             cell_size,
// //             data: HashMap::new(),
// //         }
// //     }

// //     pub fn add_point(&mut self, point: Vec2) {
// //         let cell_x = ((point.x - self.boundary.min.x) / self.cell_size).floor() as i32;
// //         let cell_y = ((point.y - self.boundary.min.y) / self.cell_size).floor() as i32;
// //         self.data.entry((cell_x, cell_y)).or_default().push(point);
// //     }

// //     pub fn cells(&self) -> Vec<(i32, i32)> {
// //         self.data.keys().cloned().collect()
// //     }

// //     pub fn points_in_cell(&self, cell: (i32, i32)) -> Vec<Vec2> {
// //         self.data.get(&cell).cloned().unwrap_or_default()
// //     }
// // }

// pub fn view<'a,F,T>(hashgrid: HashGrid<'a,F,T>)
// where
//     F: Float + FromPrimitive + ToPrimitive,
// {
//     // Launch the eframe application
//     eframe::run_native(
//         "Hashgrid Viewer",
//         eframe::NativeOptions::default(),
//         Box::new(move |_cc| Ok(Box::new(HashgridViewer::new(hashgrid)))),
//     );
// }

// // Struct for the graphical viewer
// struct HashgridViewer<'a, F,T> {
//     hashgrid: HashGrid<'a, F,T>,
// }

// impl<'a, F,T> HashgridViewer<'a,F,T>
// where
//     F: Float + FromPrimitive + ToPrimitive,
// {
//     pub fn new(hashgrid: HashGrid<'a,F,T>) -> Self {
//         Self { hashgrid }
//     }
// }

// impl<'a, F,T> eframe::App for HashgridViewer<'a, F,T>
// where
//     F: Float + FromPrimitive + ToPrimitive,
// {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             // Set up the grid area as a rectangle
//             let boundary = self.hashgrid.bounds;
//             let cell_size = self.hashgrid.params.cell_sizes;

//             let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

//             // Draw the grid and data
//             let painter = ui.painter_at(rect);

//             // Calculate grid dimensions
//             let grid_width = boundary.size[0];
//             let grid_height = boundary.size[1];
//             let cols = (grid_width / cell_size.x_size).ceil() as i32;
//             let rows = (grid_height / cell_size.y_size).ceil() as i32;

//             // Draw the grid
//             for i in 0..=cols {
//                 let x = rect.left() + i as f32 * rect.width() / cols as f32;
//                 painter.line_segment(
//                     [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
//                     egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY),
//                 );
//             }
//             for i in 0..=rows {
//                 let y = rect.top() + i as f32 * rect.height() / rows as f32;
//                 painter.line_segment(
//                     [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
//                     egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY),
//                 );
//             }

//             // Draw the points
//             for (cell, points) in &self.hashgrid.data {
//                 for point in points {
//                     let normalized_x = (point.x - boundary.min.x) / grid_width;
//                     let normalized_y = (point.y - boundary.min.y) / grid_height;

//                     let screen_x = rect.left() + normalized_x * rect.width();
//                     let screen_y = rect.top() + normalized_y * rect.height();

//                     painter.circle_filled(
//                         egui::pos2(screen_x, screen_y),
//                         3.0, // radius of the point
//                         egui::Color32::RED,
//                     );
//                 }
//             }
//         });
//     }
// }
