use array_tool::vec::Intersect;
use array_tool::vec::Union;
use array_tool::vec::Uniq;
use rayon::prelude::*;
use rand::Rng;
use super::cell::Cell;
use super::cell::new_cell;


#[derive(Clone,Debug)]
pub struct Grid {
    pub cells: Vec<Cell>,
    pub max_domains: usize,
    pub initial_domains: Vec<usize>,
    pub domain_rules:Vec<Vec<usize>>,
    pub rows: usize,
    pub cols: usize,
    pub total_cells:usize
}
unsafe impl Sync for Grid {}
unsafe impl Send for Grid {}

impl Grid {
    pub fn new(rows:usize, cols:usize, domain_rules:Vec<Vec<usize>>)->Grid {
        Grid{
            cells: (0..rows*cols).into_par_iter().map(|i|{new_cell(i,(0..domain_rules.len()).map(|d| d).collect())}).collect(),
            rows: rows,
            cols: cols,
            max_domains: domain_rules.len(),
            initial_domains: (0..domain_rules.len()).map(|d| d).collect(),
            domain_rules: domain_rules,
            total_cells:rows*cols
        }
        
    }

    fn all_collapsed(&self) -> bool {
        return self.cells.iter().all(|tile| tile.is_collapsed);
    }
    pub fn build(&mut self,id:usize) {
        if self.all_collapsed()==true {
            println!("We're all collapsed");
            return
        }
        else{
            self.cells[id].collapse();
            let mut neighbor_ids = Vec::new();
            let mut neighbor_ids_uncollapsed = Vec::new();

            self.generate_valid_neighbor_for_tile(id).into_iter().for_each(|n|{
                neighbor_ids.push(n);
                if self.cells[n].is_collapsed==false {
                    neighbor_ids_uncollapsed.push(n);
                }
            });
            
            self.propagate_neighbors(neighbor_ids);

            if neighbor_ids_uncollapsed.len() > 0 {
                let random_neighbor_index = neighbor_ids_uncollapsed[rand::thread_rng().gen_range(0..neighbor_ids_uncollapsed.len())];
                self.build(random_neighbor_index);
            }
        }
    }

    fn generate_valid_neighbor_for_tile(&self,id:usize) -> Vec<usize>{
        let mut neighbors:Vec<usize> = Vec::new();

        let top_index =  id.checked_sub(self.cols);
        let right_index = id.checked_add(1);
        let bottom_index = id.checked_add(self.cols);
        let left_index:Option<usize> = id.checked_sub(1);

        if top_index.is_some() {
            neighbors.push(top_index.unwrap());
        }
        if bottom_index.is_some() {
            if bottom_index.unwrap() < self.total_cells{
                neighbors.push(bottom_index.unwrap());
            }
        }

        if right_index.is_some(){
            if right_index.unwrap()%self.cols!=0 && right_index.unwrap() < self.total_cells{
                neighbors.push(right_index.unwrap());
            }
        }

        if left_index.is_some() {
            if left_index.unwrap()%self.cols<self.cols-1{
               neighbors.push(left_index.unwrap());
            }
        }

        return neighbors;
    } 

    pub fn propagate(&mut self, index:usize)->Vec<usize> {
        if self.cells[index].is_collapsed {return [].to_vec();}

        let neighbors = self.generate_valid_neighbor_for_tile(index);
        let mut neighbor_rules = Vec::new();
        let mut unpropagated_neighbors = Vec::new();

        (0..neighbors.len()).into_iter().for_each(|n|{

            let neighbor = self.cells[neighbors[n]].clone();
            let d = neighbor.domain.clone();
            neighbor_rules.push(self.get_rules_from_domain(d));

            if neighbor.is_propagated==false {
                unpropagated_neighbors.push(neighbors[n]);
            }

        });

        self.cells[index].is_propagated = true;
        
        self.cells[index].domain = find_all_duplicates(neighbor_rules);
        // if self.cells[index].domain.len()==0{
        //     self.print_grid();
        //     self.print_grid_with_domains(); 
            
        // }
        return unpropagated_neighbors;
    }

    pub fn propagate_neighbors(&mut self, indexes:Vec<usize>){
        let _i = indexes.clone();
        let mut next_propagation = Vec::new();
        indexes.into_iter().for_each(|i|{
            let propagated = self.propagate(i);  //Unpropagated neighbors
            next_propagation = next_propagation.union(propagated);
        });

        if next_propagation.len() > 0 {
            self.propagate_neighbors(next_propagation)
        }
    }

    pub fn merge(&mut self, subgrid:Grid, grid_row:usize,grid_col:usize){
        (0..subgrid.cells.len()).into_iter().for_each(|i|{
            if i >= subgrid.total_cells {
                return
            }
            else{   
                let cell = subgrid.cells[i].clone();
                let sub_row = (i / subgrid.cols) as usize;
                let sub_col = i % subgrid.cols as usize;
        
                let target_row = grid_row + sub_row;
                let target_col = grid_col + sub_col;
                let target_index = target_row*self.cols + target_col;
        
                if target_index >= self.total_cells {
                    return
                }
                else{
                    self.cells[target_index] = cell;
                    self.cells[target_index].id = target_index;
                }
            }
            
        })
    }

    pub fn _range(&self, from_row:usize, from_col:usize,row_size:usize,col_size:usize)->Grid{
        if from_row+row_size > self.rows || from_col+col_size > self.cols {
            panic!("Range out of bounds")
        }
    
        let mut g= Grid::new(row_size, col_size, self.domain_rules.clone());
        (0..row_size).for_each(|i|{
            let start_index = from_row*self.cols + from_col;
            let end_index = start_index + col_size;
            let looop = end_index-start_index;
            (0..looop).for_each(|j|{
                g.cells[i*g.cols+j] = self.cells[i*self.cols+start_index+j].clone();
            });
        });
    
        return g
    }
    
    fn get_rules_from_domain(&mut self,domains:Vec<usize>)->Vec<usize>{
        let mut rules = Vec::new();
        
        domains.into_iter().for_each(|d|{
            let domain_rules = self.domain_rules[d].clone();
            rules = rules.union(domain_rules);
        }); 
        return rules.unique();
    }



    #[allow(dead_code)]
    pub fn print_grid(&self){
        for i in 0..self.rows{
            let mut chunks = Vec::new();
            for j in 0..self.cols{
                let index = i*self.cols+j;
                let mut domain:usize = 9;
                if self.cells[index as usize].is_collapsed {
                    domain = self.cells[index as usize].collapsed_domain.unwrap();
                }
                chunks.push(format!("{}",self.tile_from_domain(&domain)));
            }
            println!("{}",chunks.join(""));
        }
    }
    pub fn _print_grid_with_domains(&self){
        for i in 0..self.rows{
            let mut chunks = Vec::new();
            for j in 0..self.cols{
                let index = i*self.cols+j;
                let mut domain:usize = 9;
                if self.cells[index as usize].is_collapsed {
                    domain = self.cells[index as usize].collapsed_domain.unwrap();
                }
                chunks.push(format!("{} {:?}",self.tile_from_domain(&domain),self.cells[index as usize].domain));
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
    fn format_domain(&self,cell:&Cell,index:i32)->String{
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

}


fn find_all_duplicates(rules:Vec<Vec<usize>>)->Vec<usize>{
    let intersecting_domains = rules.into_iter().reduce(|b,a|{
        return b.intersect(a);
    });
    //println!("Intersecting domains are {:?}",intersecting_domains);
    return intersecting_domains.unwrap();
}