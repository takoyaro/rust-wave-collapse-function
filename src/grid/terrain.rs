use super::grid::Grid;

pub struct Terrain {
    pub SIZE:usize,
    pub RULES:Vec<Vec<usize>>,
    pub GRID:Grid
}

impl Terrain {
    pub fn new(size:usize,rules:Vec<Vec<usize>>) -> Terrain {
        let grules = rules.clone();
        Terrain{SIZE:size,RULES:rules,GRID:Grid::new(size, size, grules)}
        
    }
    pub fn init(&mut self,size:usize)->&mut Terrain{
        self.GRID.print_grid();
        let buildStartingIndex = self.SIZE * self.SIZE / 2;
        self.GRID.build(buildStartingIndex);
        let loop_count = (((size as f64)-(self.SIZE as f64)) / 2 as f64).ceil() as i32;
        (0..loop_count).for_each(|_i|{
            self.Expand();
        });
        return self;
    }

    fn Expand(&mut self){
        let mut grid = Grid::new(self.SIZE+2, self.SIZE+2, self.RULES.clone());
        grid.merge(self.GRID.clone(), 1, 1);
        grid.propagate(1);
        // (1..self.GRID.CELLS.len()-1).for_each(|i|{
        //     if self.GRID.CELLS[i].IS_COLLAPSED==false {
        //         self.GRID.propagate(i);
        //     }
        //     print!("Propagating {}/{}\r",i,self.GRID.CELLS.len()-1);
        // });
        
        grid.build(1);
        self.SIZE += 2;
        self.GRID = grid;
        
        // println!("{:?}",self.GRID.CELLS[1]);
        // println!("{:?}",self.GRID.CELLS[self.SIZE+1]);
    }

    
}

pub fn Window(mut terrain:Terrain,from_row:usize,from_col:usize,row_size:usize,col_size:usize)->Grid{
    let diffRow = from_row + row_size - terrain.GRID.ROWS;
    let diffCol = from_col + col_size - terrain.GRID.COLS;

    if diffRow > 0 || diffCol > 0 {
        let max = std::cmp::max(diffRow,diffCol);
        (0..max).for_each(|_i| {
            terrain.Expand()
        });
    }

    return terrain.GRID.range(from_row, from_col, row_size, col_size)
}