use rand::Rng;
use std::time::{Duration, Instant};
extern crate array_tool;


#[derive(Clone,Debug)]
struct Tile {
    pub id: usize,
    pub row: usize,
    pub col: usize,
    pub neighbors: Vec<usize>,
    pub domain: Vec<usize>,
    pub collapsed_domain: Option<usize>,
    pub is_propagated: bool,
    pub is_collapsed:bool
}
impl Tile{
    fn get_domain(&self) -> Vec<usize> {
        self.domain.clone()
    }
}
struct Grid {
    pub cells: Vec<Tile>,
    pub max_domains: usize,
    pub initial_domains: Vec<usize>,
    pub rows: usize,
    pub cols: usize,
    pub rng:rand::prelude::ThreadRng
}

impl Grid {
    fn new(rows:usize, cols:usize, max_domains: usize)->Grid {
        Grid{
            cells: Vec::new(),
            rows: rows,
            cols: cols,
            max_domains: max_domains,
            initial_domains: (0..max_domains+1).collect(),
            rng:rand::thread_rng()
        }
        
    }

    fn init(&mut self){
        for i in 0..&self.rows * &self.cols {
            let row:usize = i / self.rows;
            let col:usize = i % self.cols;
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
        return self.cells.iter().all(|tile| tile.is_collapsed);
    }
    fn start(&mut self,mut id:Option<usize>) {
        if self.all_collapsed() {
            return
        }
        else{
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
            //set all cells is_propagated property to false
            for cell in self.cells.iter_mut(){cell.is_propagated=false}

            let good_id = id.unwrap(); 
            self.collapse_tile(good_id);

            let uncollapsed_neighbors:Vec<usize> = self.cells[good_id].neighbors.clone().into_iter().filter(|n| !self.is_collapsed(*n)).collect();
            if !uncollapsed_neighbors.is_empty() {
                let random_neighbor = uncollapsed_neighbors[self.rng.gen_range(0..uncollapsed_neighbors.len())];
                self.start(Some(random_neighbor));
            } else {
                self.start(None);
            } 
        }
    }
    fn start_mega(&mut self){
        if self.all_collapsed() {
            return
        }
        else{
            let id = self.first_uncollapsed_tile_index();
            // let mut random_tile:Tile;
            //set all cells is_propagated property to false
            for cell in self.cells.iter_mut(){cell.is_propagated=false}
            self.collapse_tile(id);
            self.start_mega();
        }
    }
    fn generate_valid_neighbor_for_tile(&self,id:usize) -> Vec<usize>{
        let mut neighbors:Vec<usize> = Vec::new();

        let top_index =  id.checked_sub(self.cols);
        let right_index = id.checked_add(1);
        let bottom_index = id.checked_add(self.cols);
        let left_index = id.checked_sub(1);
        let r = 0..self.cells.len();
        if top_index.is_some(){
            if r.contains(&top_index.unwrap()) {
                neighbors.push(top_index.unwrap());
            }
        }
        if right_index.is_some(){
            if r.contains(&right_index.unwrap()) {
                neighbors.push(right_index.unwrap());
            }
        }
        if bottom_index.is_some(){
            if r.contains(&bottom_index.unwrap()) {
                neighbors.push(bottom_index.unwrap());
            }
        }
        if left_index.is_some(){
            if r.contains(&left_index.unwrap()) {
                neighbors.push(left_index.unwrap());
            }
        }
        return neighbors;
    } 
    fn get_neighbor_domains_for_tile(&self,id:usize) -> Vec<Vec<usize>>{
        return self.generate_valid_neighbor_for_tile(id).into_iter().map(|n|self.cells[n].domain.clone()).collect::<Vec<Vec<usize>>>();
    }

    fn propagate(&mut self, index:usize)->Vec<usize> {
        let tile = &self.cells[index];
        if tile.is_collapsed || tile.is_propagated {return [].to_vec();}

        let neighbor_domains = self.get_neighbor_domains_for_tile(index);
        let c = neighbor_domains.clone();
        let neighbor_rules:Vec<Vec<usize>> = neighbor_domains.into_iter().map(|domain| 
            {let res:Vec<usize> = domain.into_iter().map(|x| 
                self.valid_domains_from_domain(x)
            ).flatten().collect();
            return res}
        ).collect();

        use array_tool::vec::Intersect;
        let x:Vec<usize> = neighbor_rules.into_iter().reduce(|a, b| a.intersect(b)).unwrap();
        if x.is_empty(){
            println!("Propagating {}, neighbors have {:?}",index,c);
        }
        self.cells[index] = Tile { id: tile.id, row: tile.row, col: tile.col, neighbors: tile.neighbors.clone(), domain: x, collapsed_domain: tile.collapsed_domain, is_propagated: true, is_collapsed:tile.is_collapsed };

        return self.generate_valid_neighbor_for_tile(index);
    }

    fn propagate_all(&mut self, indexes:Vec<usize>){
        if self.is_done(){
            return
        }
        else{
            let n_indexes:Vec<Vec<usize>> = indexes.into_iter().map(|i| self.propagate(i)).collect::<Vec<Vec<usize>>>();
            let neighbors:Vec<usize> = n_indexes.into_iter().map(|nidxs| 
                nidxs.into_iter().filter(|nidx| !self.is_propagated(*nidx)  && !self.is_collapsed(*nidx)).collect::<Vec<usize>>()
            ).flatten().collect();
            
            if !neighbors.is_empty() {
                use array_tool::vec::Uniq;
                self.propagate_all(neighbors.unique()); 
            }
        }
    }

    fn collapse_tile(&mut self, index:usize){
        let cell_domain = &self.cells[index].domain;
        if cell_domain.is_empty(){
            println!("Tile {} domain is empty.", index);
            println!("{:?}", self.cells[index]);
            println!("Neighbors have {:?}", self.get_neighbor_domains_for_tile(index));
            self.print_grid();
        }
        self.cells[index].collapsed_domain = Some(cell_domain[self.rng.gen_range(0..cell_domain.len())]);
        self.cells[index].domain = vec![self.cells[index].collapsed_domain.unwrap()];
        self.cells[index].is_propagated = true;
        self.cells[index].is_collapsed = true;
        self.propagate_all(self.cells[index].neighbors.clone());
    }

    fn random_uncollapsed_tile_index(&mut self)->Option<usize>{
        let indexes = self.cells.iter().filter_map(|x| if x.is_collapsed {None} else {Some(x.id)}).collect::<Vec<usize>>();
        if indexes.is_empty(){
            return None;
        }
        else{
            return Some(indexes[self.rng.gen_range(0..indexes.len())]);
        }
    }
    fn first_uncollapsed_tile_index(&mut self)->usize{
        for (i,x) in self.cells.iter().enumerate(){
            if !x.is_collapsed {
                return i;
            }
        }
        return 0;
    }
    fn valid_domains_from_domain(&self, domain:usize)->Vec<usize>{
        let mut res:Vec<usize> = Vec::new();
        let min_one = domain.checked_sub(1);
        if min_one.is_some(){
            res.push(min_one.unwrap());
        }
        res.push(domain);
        let max_one = domain.checked_add(1);
        if max_one.is_some(){
            let max = max_one.unwrap();
            if max<self.max_domains{
                res.push(max);
            }
        }
        return res;
    }
    
    fn is_done(&self)->bool{
        return self.cells.iter().all(|tile| tile.is_propagated);
    }
    fn is_propagated(&self,index:usize)->bool{
        return self.cells[index].is_propagated;
    }
    fn is_collapsed(&self,index:usize)->bool{
        return self.cells[index].is_collapsed;
    }

    #[allow(dead_code)]
    fn print_grid(&self){
        for i in 0..self.rows{
            let mut chunks = Vec::new();
            for j in 0..self.cols{
                
                let index = i*self.cols+j;
                let mut domain:usize = 9;
                if self.cells[index as usize].is_collapsed {
                    domain = self.cells[index as usize].collapsed_domain.unwrap();
                }
                chunks.push(format!("█{}█",self.tile_from_domain(&domain)));
            }
            println!("{}",chunks.join(""));
        }
    }
    #[allow(dead_code)]
    fn tile_from_domain(&self,domain:&usize)->String{
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
        else if domain==&9 {r=0;g=0;b=0;}
        if domain ==&9 {
            return format!("\x1b[48;2;{};{};{}m{:?}\x1b[0m",r,g,b,"_");
        }
        return format!("\x1b[48;2;{};{};{}m{:?}\x1b[0m",r,g,b,domain.to_string());
    }

    #[allow(dead_code)]
    fn format_domain(&self,cell:&Tile,index:i32)->String{
        if cell.id == index as usize{
            if cell.collapsed_domain.is_some() {
                return format!("\x1b[93m*{}*\x1b[0m",cell.collapsed_domain.unwrap_or(9));
            }
            else{
                return format!("\x1b[96m*{}*\x1b[0m",cell.collapsed_domain.unwrap_or(9));
            }
            
        }
        else{
            if cell.collapsed_domain.is_some() {
                return format!("\x1b[92m{}\x1b[0m",cell.collapsed_domain.unwrap_or(9));
            }
            else{
                return format!("\x1b[90m{}\x1b[0m",cell.collapsed_domain.unwrap_or(9));
            }
        }
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
        }
        println!("============================END [{}]===================================",index);
    }  

}

fn gen_mega_grid(size:usize,chunk_size:usize,max_domains:usize)->bool{
    let mut grids = Vec::new();
    for _ in 0..size{
        let mut grid = Grid::new(chunk_size,chunk_size,max_domains);
        grid.init();
        grid.start(Some(1));
        grids.push(grid);
        
    }
    let mut megagrid = Grid::new(size,size,max_domains);
    megagrid.init();
    megagrid.cells.iter_mut().enumerate().for_each(|(i,cell)|{
        let subgrid_row_in_megagrid = (cell.row as f32/chunk_size as f32).floor() as usize;         
        let subgrid_col_in_megagrid = (cell.col as f32/chunk_size as f32).floor() as usize;
        let subgrid_index_in_megagrid = subgrid_row_in_megagrid*chunk_size+subgrid_col_in_megagrid;
        let cell_row_in_grid = cell.row.checked_sub(subgrid_row_in_megagrid*chunk_size).unwrap_or(0);
        let cell_col_in_grid = cell.col.checked_sub(subgrid_col_in_megagrid*chunk_size).unwrap_or(0);
        let cell_index_in_grid = cell_row_in_grid+cell_col_in_grid.checked_sub(1).unwrap_or(0);

        println!("Megagrid Cell #{} takes data from subgrid #{} at ({},{}) - MegaGrid({},{})",i,subgrid_index_in_megagrid,cell_row_in_grid,cell_col_in_grid,subgrid_row_in_megagrid,subgrid_col_in_megagrid);
        cell.row = cell_row_in_grid;
        cell.col = cell_col_in_grid;
        cell.id = i;
        cell.collapsed_domain = grids[subgrid_index_in_megagrid].cells[cell_index_in_grid.checked_sub(1).unwrap_or(0)].collapsed_domain;
        cell.domain = grids[subgrid_index_in_megagrid].cells[cell_index_in_grid.checked_sub(1).unwrap_or(0)].domain.clone();
        cell.is_collapsed = grids[subgrid_index_in_megagrid].cells[cell_index_in_grid.checked_sub(1).unwrap_or(0)].is_collapsed;
    });
    
    let hv_stripes = chunk_size-1;
    let stripe_size = max_domains-1;  //This is the magical variable.
    let mut indexes = Vec::new();
    for h_stripe_index in 0..hv_stripes-1{
        let starting_column = (h_stripe_index)*(chunk_size-stripe_size);
        let ending_column =  (h_stripe_index)*(chunk_size+stripe_size);
        
        megagrid.cells.iter_mut().for_each(|cell|{
            if cell.col>=starting_column && cell.col<ending_column{
                cell.collapsed_domain = None;
                cell.domain = (0..max_domains+1).collect();
                cell.is_collapsed = false;
                cell.is_propagated = false;
                indexes.push(cell.id);
            }
        });
    }

    for v_stripe_index in 0..hv_stripes-1{
        let starting_row = (v_stripe_index+1)*(chunk_size-stripe_size);
        let ending_row =  (v_stripe_index+1)*(chunk_size+stripe_size);
        megagrid.cells.iter_mut().for_each(|cell|{
            if cell.row>=starting_row && cell.row<ending_row{
                cell.collapsed_domain = None;
                cell.domain =(0..max_domains+1).collect();
                cell.is_collapsed = false;
                cell.is_propagated = false;
                indexes.push(cell.id);
            }
        });
    }
    megagrid.propagate_all(indexes);
   
    megagrid.start_mega();
    megagrid.print(0);
    megagrid.print_grid();
    //megagrid.start(None);
    //megagrid.print_grid();
    return true;
}

fn main() {
    //Grid Properties
    // let rows:i32 = 30;
    // let cols:i32 = 30;
    // let max_domain:i32 = 5;
    // let mut start = Instant::now();
    // let mut grid = Grid::new(rows, cols, max_domain);
    // let mut duration = start.elapsed();
    // grid.init();
    // println!("{:?} {:?} Grid initialized in: {:?}",rows,cols, duration);
    // start = Instant::now();
    // grid.start(Some(1));
    // duration = start.elapsed();
    // println!("Grid Generated in: {:?}", duration);
    // grid.print_grid();
    // print_color_legend();
    //benchmark(50,25);
    //gen_mega_grid(230400, 5)
    let mut start:Instant = Instant::now();
    let size = 7;
    println!("{}",3 as usize/2 as usize);
    let done = gen_mega_grid(size*size,size,5);
    let duration = start.elapsed();

    println!("Megagrid generated in {:?}",duration);
    println!("____________________________________________________________________________________");
    println!("{}",3 as usize/2 as usize);
    // if done == true{
    //     start = Instant::now();
    //     let mut grid = Grid::new(size*size,size*size,5);
    //     grid.init();
    //     grid.start(Some(1));
    //     let duration = start.elapsed();
    //     println!("Grid generated in {:?}",duration);
    //     grid.print_grid();
    // }
   
    // benchmark(720,5); 
    // benchmark(222,9); 
    // benchmark(125,12); 
}

fn benchmark(count:usize,size:usize)->bool{
    let mut durations:Vec<Duration> = Vec::new();
    for i in 0..count{
        let mut grid = Grid::new(size, size, 5);
        grid.init();
        let start = Instant::now();
        grid.start(Some(1));
        let duration = start.elapsed();
        durations.push(duration);
        //grid.print_grid();
        //println!("------------------------------------------------------")
    }
    let mut total_duration = Duration::new(0,0);
    for duration in durations.clone(){
        total_duration = total_duration + duration;
    }
    let min_duration = durations.iter().min().unwrap();
    let max_duration = durations.iter().max().unwrap();
    println!("{:?}({:?} cells) Grid generated in: {:?}",size*size*count,size,total_duration);
    println!("Avg Time/Grid: {:?}",total_duration/count as u32);
    println!("Fastest Grid: {:?}",min_duration);
    println!("Slowest Grid: {:?}",max_duration);
    return true;
}