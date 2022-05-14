mod grid {
    pub mod terrain;
    pub mod grid;
    pub mod cell;
    pub mod utils;
}
// use rand::Rng;
use std::time::{Instant};

use crate::grid::terrain::Terrain;
// use rayon::prelude::*;

// extern crate array_tool;


// fn gen_uni_grid(width:usize,height:usize,padding:usize,max_domains:usize)->Grid{
//     let mut grid = Grid::new(height,1,max_domains);
//     grid.init();
//     grid.start(Some(0));
//     let cols_idxs:Vec<usize> = (0..width).collect();
//     cols_idxs.into_iter().for_each(|_idx|{
//         if grid.cols < width{
//             grid.append_to_grid_cols(1,padding);
//         }
//         else{
//             return;
//         }
//     });
//     return grid
//     //println!("{:?}",grid.max_domains);
// }

// fn gen_fib_grid(size:usize,padding:usize,max_domains:usize)->Grid{
//     let mut grid = Grid::new(1,1,max_domains);
//     grid.init();
//     grid.start(Some(0));
//     (0..size).for_each(|_idx|{
//         if grid.cols < size{
//             grid.append_fib_surrounding();
//         }
//         else{
//             return;
//         }
//     });
//     return grid;
// }

// fn generate_valid_neighbor_for_tile(cols:usize,rows:usize,total_len:usize,id:usize) -> Vec<usize>{
//     let mut neighbors:Vec<usize> = Vec::new();

//     let top_index =  id.checked_sub(cols);
//     let right_index = id.checked_add(1);
//     let bottom_index = id.checked_add(cols);
//     let left_index = id.checked_sub(1);
//     let r = 0..total_len;
//     if top_index.is_some(){
//         if r.contains(&top_index.unwrap()) {
//             neighbors.push(top_index.unwrap());
//         }
//     }
//     if right_index.is_some(){
//         if r.contains(&right_index.unwrap()) {
//             neighbors.push(right_index.unwrap());
//         }
//     }
//     if bottom_index.is_some(){
//         if r.contains(&bottom_index.unwrap()) {
//             neighbors.push(bottom_index.unwrap());
//         }
//     }
//     if left_index.is_some(){
//         if r.contains(&left_index.unwrap()) {
//             neighbors.push(left_index.unwrap());
//         }
//     }
//     return neighbors;
// }



fn main() {
    let start:Instant = Instant::now();
    let size = 10;
	let rules = vec![vec![0,1],vec![0,1,2],vec![1,2,3],vec![2,3,4],vec![3,4,5],vec![4,5]];
	let mut t = Terrain::new(3,rules);
    t.init(size);
	// t.Grid.Print()

    let duration = start.elapsed();
    
    println!("Megagrid generated in {:?}",duration);
    println!("____________________________________________________________________________________");
    
   (0..100).for_each(|_idx|{
        let mut t = Terrain::new(3,vec![vec![0,1],vec![0,1,2],vec![1,2,3],vec![2,3,4],vec![3,4,5],vec![4,5]]);
        t.init(size);
    });

}