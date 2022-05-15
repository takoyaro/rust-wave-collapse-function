mod grid {
    pub mod terrain;
    pub mod grid;
    pub mod cell;
    pub mod utils;
}
use std::time::{Instant};

use grid::terrain::Terrain;

fn main() {
    let start:Instant = Instant::now();
    let size = 12;
	let rules = vec![vec![0,1],vec![0,1,2],vec![1,2,3],vec![2,3,4],vec![3,4,5],vec![4,5]];
	let mut t = Terrain::new(3,rules);
    t.init(size);

    let duration = start.elapsed();
    
	t.grid.print_grid();
    println!("Megagrid generated in {:?}",duration);
    println!("____________________________________________________________________________________");

}