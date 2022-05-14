use super::grid::Grid;

pub struct Terrain {
    pub size:usize,
    pub rules:Vec<Vec<usize>>,
    pub grid:Grid
}

impl Terrain {
    pub fn new(size:usize,rules:Vec<Vec<usize>>) -> Terrain {
        let grules = rules.clone();
        Terrain{size:size,rules:rules,grid:Grid::new(size, size, grules)}
        
    }
    pub fn init(&mut self,size:usize)->&mut Terrain{
        let build_starting_index = self.size * self.size / 2;
        self.grid.build(build_starting_index);
        let loop_count = (((size as f64)-(self.size as f64)) / 2 as f64).ceil() as i32;
        (0..loop_count).for_each(|_i|{
            self.expand();
        });
        return self;
    }

    fn expand(&mut self){
        let mut grid = Grid::new(self.size+2, self.size+2, self.rules.clone());
        grid.merge(self.grid.clone(), 1, 1);
        grid.propagate(1);
        
        grid.build(1);
        self.size += 2;
        self.grid = grid;
        
        // println!("{:?}",self.grid.CELLS[1]);
        // println!("{:?}",self.grid.CELLS[self.size+1]);
    }

    
}

pub fn _window(mut terrain:Terrain,from_row:usize,from_col:usize,row_size:usize,col_size:usize)->Grid{
    let diff_row = from_row + row_size - terrain.grid.rows;
    let diff_col = from_col + col_size - terrain.grid.cols;

    if diff_row > 0 || diff_col > 0 {
        let max = std::cmp::max(diff_row,diff_col);
        (0..max).for_each(|_i| {
            terrain.expand()
        });
    }

    return terrain.grid._range(from_row, from_col, row_size, col_size)
}