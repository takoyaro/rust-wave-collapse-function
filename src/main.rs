use rand::Rng;
use rayon::prelude::*;
use std::time::{Duration, Instant};


#[derive(Clone,Debug)]
struct Tile {
    pub id: i32,
    pub row: i32,
    pub col: i32,
    pub neighbors: Vec<usize>,
    pub domain: Vec<i32>,
    pub collapsed_domain: Option<i32>,
    pub is_propagated: bool,
    pub is_collapsed:bool
}
struct Grid {
    pub cells: Vec<Tile>,
    pub max_domains: i32,
    pub initial_domains: Vec<i32>,
    pub rows: i32,
    pub cols: i32
}

impl Grid {
    fn new(rows:i32, cols:i32, max_domains: i32)->Grid {
        Grid{
            cells: Vec::new(),
            rows: rows,
            cols: cols,
            max_domains: max_domains,
            initial_domains: (0..max_domains+1).collect()
        }
        
    }

    fn init(&mut self){
        for i in 0..&self.rows * &self.cols {
            let row:i32 = i / self.rows;
            let col:i32 = i % self.cols;
            let tile = Tile { id: i, row: row, col: col, neighbors: Vec::new(), domain:self.initial_domains.clone(), collapsed_domain: None, is_propagated: false, is_collapsed: false };
            self.cells.push(tile);
        }
        for i in 0..self.rows * self.cols{
            let rawtile = &self.cells[i as usize];//Tile { row: grid[i as usize].row, col: grid[i as usize].col};
            let neighbors:Vec<usize> = self.generate_valid_neighbor_for_tile(rawtile.id as usize);
            self.cells[i as usize].neighbors = neighbors;
        }
    }
    fn all_collapsed(&self) -> bool {
        for i in 0..self.cells.len(){
            if self.cells[i].is_collapsed == false{
                return false;
            }
        }
        return true;
    }
    fn start(&mut self,mut id:Option<usize>) {
        if self.all_collapsed() {
            return
        }
        else{
            let mut rng = rand::thread_rng();
            // let mut random_tile:Tile;
            if id.is_none() {
                if self.random_uncollapsed_tile_index().is_none() {
                    println!("All tiles have been collapsed. We are Done");
                    return;
                }
                else{
                    id = self.random_uncollapsed_tile_index(); 
                }
            }
            for index in 0..self.cells.len(){
                self.cells[index].is_propagated = false;
            }
            let good_id = id.unwrap(); 
            self.cells[good_id].collapsed_domain = Some(self.cells[good_id].domain.clone()[rng.gen_range(0..self.cells[good_id].domain.len())]);
            self.cells[good_id].domain = vec![self.cells[good_id].collapsed_domain.unwrap()];
            self.cells[good_id].is_propagated = true;
            self.cells[good_id].is_collapsed = true;
            let init_neighbors = self.cells[good_id].neighbors.clone();
            self.propagate_all(init_neighbors);

            let neighbors = self.cells[good_id].neighbors.clone();
            let uncollapsed_neighbors:Vec<usize> = neighbors.into_iter().filter(|n| self.cells[*n].is_collapsed==false).collect();

            if uncollapsed_neighbors.len() > 0 as usize {
                let random_neighbor = uncollapsed_neighbors[rng.gen_range(0..uncollapsed_neighbors.len())];
                self.start(Some(random_neighbor));
            } else {
                self.start(None);
            } 
        }
    }
    
    fn get_rowcol_from_id(&self, id:i32) -> (i32,i32){
        let row:i32 = id/self.rows;
        let col:i32 = id % self.cols;
        return (row,col);
    }
    
    fn is_valid_row_col(&self, id:i32) -> bool{
        let rowcol = self.get_rowcol_from_id(id);
        if rowcol.0 < 0 || rowcol.0 >= self.rows {
            return false;
        }
        if rowcol.1 < 0 || rowcol.1 >= self.cols {
            return false;
        }
        return true;
    }
    
    fn is_valid_neighbor(&self, neighbor_id:i32,tile_id:i32,row_modifier:i32,col_modifier:i32) -> bool{
        let source_tile = &self.cells[tile_id as usize];
        if self.get_rowcol_from_id(neighbor_id).0 == source_tile.row+row_modifier && self.get_rowcol_from_id(neighbor_id).1 == source_tile.col+col_modifier && self.is_valid_row_col(neighbor_id){
           return true
        }
        return false;
    }
    
    fn generate_valid_neighbor_for_tile(&self,id:usize) -> Vec<usize>{
        let mut neighbors:Vec<usize> = Vec::new();
        let id32 = id as i32;
        let top_index =  id32 - self.cols;
        let right_index = id32 + 1;
        let bottom_index = id32 + self.cols;
        let left_index = id32 - 1;
        if self.is_valid_neighbor(top_index, id32, -1, 0) {
            neighbors.push(top_index as usize);
        }
        if self.is_valid_neighbor(right_index, id32, 0, 1) {
            neighbors.push(right_index as usize);
        }
        if self.is_valid_neighbor(bottom_index, id32, 1, 0) {
            neighbors.push(bottom_index as usize);
        }
        if self.is_valid_neighbor(left_index, id32, 0, -1) {
            neighbors.push(left_index as usize);
        }
        return neighbors;
    } 

    fn propagate(&self, index:usize)->(Vec<usize>, Option<Tile>) {
        let tile: &Tile = &self.cells[index];

        if tile.is_collapsed || tile.is_propagated {
            return ([].to_vec(), None);
        }
		let neighbors = self.generate_valid_neighbor_for_tile(index);
        let ret = neighbors.clone();
        let neighbor_domains = neighbors.into_iter().map(|n| &self.cells[n as usize].domain);
        let neighbor_rules:Vec<Vec<i32>> = neighbor_domains.map(|domain| domain.into_iter().map(|x| self.valid_domains_from_domain(*x)).flatten().collect()).collect();
        let mut x:Vec<i32> = Some(neighbor_rules.into_iter().reduce(|a, b| a.into_iter().filter(|v| b.contains(v)).collect())).unwrap().unwrap();
        x.sort();
        x.dedup();
        let res:Vec<i32> = x.into_iter().filter(|x| x >= &(0) && x <= &self.max_domains).map(|n| n).collect();
        if res.len() < 1 {
            println!("possible domains ended up empty for index {:?}", tile.id);
            panic!("PANIC!!!");
        }
        
        //self.update_cell(tile.id as usize, Tile { id: tile.id, row: tile.row, col: tile.col, neighbors: tile.neighbors.clone(), domain: res, collapsed_domain: tile.collapsed_domain, is_propagated: true, is_collapsed:tile.is_collapsed });
        //self.cells[index] = Tile{id:tile.id, row:tile.row, col:tile.col, neighbors:tile.neighbors, domain:tile.domain, collapsed_domain:tile.collapsed_domain, is_propagated:tile.is_propagated, is_collapsed:tile.is_collapsed};
        //self.cells[index] = Tile { id: tile.id, row: tile.row, col: tile.col, neighbors: tile.neighbors.clone(), domain: res, collapsed_domain: tile.collapsed_domain, is_propagated: true, is_collapsed:tile.is_collapsed };
        // thread::sleep_ms(1000);
        return (ret, Some(Tile { id: tile.id, row: tile.row, col: tile.col, neighbors: tile.neighbors.clone(), domain: res, collapsed_domain: tile.collapsed_domain, is_propagated: true, is_collapsed:tile.is_collapsed }))
    }

    fn neighbor_propagation(&self,index:usize)->(Vec<usize>, Option<Tile>){
        let propagation = self.propagate(index);
        let neighbor_indexes = propagation.0;
        let mut neighbors:Vec<usize> = Vec::new();
        for neighbor_index in neighbor_indexes {
            if self.cells[neighbor_index].is_propagated == false {
                neighbors.push(neighbor_index as usize);
            };
        };
        return (neighbors,propagation.1)
    }

    fn propagate_all(&mut self, indexes:Vec<usize>){
        let done:bool = self.is_done();
        if done{
            return
        }
        else{
            let mut neighbors: Vec<usize> = Vec::new();
            let propagated_neighbors:Vec<(Vec<usize>, Option<Tile>)> = indexes.into_par_iter().map(|index| return self.neighbor_propagation(index)).collect();
            // let propagated_neighbors:Vec<(&mut Grid, Vec<usize>, &Tile)> = indexes.into_par_iter().map(|index| {
            //     let (neighbors,tile) = self.neighbor_propagation(index);
            //     return (self, neighbors,tile);
            // }).collect();
            let mut test:Vec<(usize,Tile)> = Vec::new();
            for propagated_neighbor in propagated_neighbors{
                let (pneighbors, tile) = propagated_neighbor;
                
                for n in pneighbors {
                    neighbors.push(n)
                }

                if tile.is_some() {
                    let unwrapped_tile = tile.unwrap();
                    test.push((unwrapped_tile.id as usize, Tile{id:unwrapped_tile.id, row:unwrapped_tile.row, col:unwrapped_tile.col, neighbors:unwrapped_tile.neighbors.clone(), domain:unwrapped_tile.domain.clone(), collapsed_domain:unwrapped_tile.collapsed_domain, is_propagated:unwrapped_tile.is_propagated, is_collapsed:unwrapped_tile.is_collapsed}));
                }
                //self.cells[tile.id as usize] = Tile{id:tile.id, row:tile.row, col:tile.col, neighbors:tile.neighbors.clone(), domain:tile.domain.clone(), collapsed_domain:tile.collapsed_domain, is_propagated:tile.is_propagated, is_collapsed:tile.is_collapsed};
            }
            for (idx,tile) in test{
                self.cells[idx] = tile;
            }
            neighbors.sort();
            neighbors.dedup();

            if neighbors.len()>0 {
                self.propagate_all(neighbors); 
            }
        }
    }

    fn random_uncollapsed_tile_index(&self)->Option<usize>{
        let mut indexes:Vec<usize> = Vec::new();
        for i in 0..self.cells.len(){
            if self.cells[i].is_collapsed==false{
                indexes.push(i);
            }
        } 
        if indexes.len() > 0 {
            return Some(indexes[rand::thread_rng().gen_range(0..indexes.len())]);
        }
        else{
            return None;
        }
    }
    fn valid_domains_from_domain(&self, domain:i32)->Vec<i32>{
        let mut res:Vec<i32> = Vec::new();
        let min_one = domain-1;
        if min_one >= 0 {
            res.push(min_one);
        }
        res.push(domain);
        let plus_one = domain+1;
        if plus_one <= self.max_domains {
            res.push(plus_one);
        }
        return res;
    }
    
    fn is_done(&self)->bool{
        for i in 0..self.cells.len(){
            if self.cells[i].is_propagated==false{
                return false;
            }
        }
        return true;
    }

    fn print(&self, index:i32){
        println!("===========================START [{}]==================================",index);
        for i in 0..self.rows{
            
            //get columns collapsed_domains
            let collapsed_domains:Vec<String> = self.cells.clone().into_iter().filter(|cell| 
                cell.row == i).map(|cell|
                    format!("{},[{}]",
                    self.format_domain(&cell, index),
                    cell.domain.into_iter().map(|d| 
                        d.to_string()
                    ).collect::<Vec<String>>().join(",")
                )
            ).collect();
            //concatenated domains as string
            let concatenated_domains:String = collapsed_domains.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join("|");
            println!("{}", concatenated_domains);
            println!("--------------------------------------------------------------");
        }
        println!("============================END [{}]===================================",index);
    }   

    fn format_domain(&self,cell:&Tile,index:i32)->String{
        if cell.id == index{
            if cell.collapsed_domain.is_some() {
                return format!("\x1b[93m*{}*\x1b[0m",cell.collapsed_domain.unwrap_or(-1));
            }
            else{
                return format!("\x1b[96m*{}*\x1b[0m",cell.collapsed_domain.unwrap_or(-1));
            }
            
        }
        else{
            if cell.collapsed_domain.is_some() {
                return format!("\x1b[92m{}\x1b[0m",cell.collapsed_domain.unwrap_or(-1));
            }
            else{
                return format!("\x1b[90m{}\x1b[0m",cell.collapsed_domain.unwrap_or(-1));
            }
        }
    }
    fn print_grid(&self){
        for i in 0..self.rows{
            let mut chunks = Vec::new();
            for j in 0..self.cols{
                let index = i*self.cols+j;
                let mut domain:i32 = 9;
                if self.cells[index as usize].is_collapsed {
                    domain = self.cells[index as usize].collapsed_domain.unwrap();
                }
                chunks.push(self.tile_from_domain(&domain));
            }
            println!("{}",chunks.join(""));
        }
    }
    fn tile_from_domain(&self,domain:&i32)->String{
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        if domain==&0 {r=112;g=181;b=255;}
        else if domain==&1 {r=255;g=205;b=112;}
        else if domain==&2 {r=93;g=184;b=79;}
        else if domain==&3 {r=57;g=105;b=49;}
        else if domain==&4 {r=84;g=59;b=19;}
        else if domain==&5 {r=43;g=41;b=36;}
        else if domain==&6 {r=255;g=255;b=255;}
        else if domain==&-1 {r=0;g=0;b=0;}
        return format!("\x1b[48;2;{};{};{}m{:?}\x1b[0m",r,g,b,domain);
    }

}

fn print_color_legend(){
    println!("\x1b[90m{}\x1b[0m | \x1b[96m{}\x1b[0m | \x1b[92m{}\x1b[0m | \x1b[93m{}\x1b[0m","Uncollapsed","Uncollapsed(Propagated here)","Collapsed","Collapsed (Propagated here)");
}

fn main() {
    //Grid Properties
    let rows:i32 = 64;
    let cols:i32 = 64;
    let max_domain:i32 = 5;
    let mut start = Instant::now();
    let mut grid = Grid::new(rows, cols, max_domain);
    let mut duration = start.elapsed();
    grid.init();
    println!("{:?} {:?} Grid initialized in: {:?}",rows,cols, duration);
    start = Instant::now();
    grid.start(Some(1));
    duration = start.elapsed();
    println!("Grid Generated in: {:?}", duration);
    grid.print_grid();
    print_color_legend();
}