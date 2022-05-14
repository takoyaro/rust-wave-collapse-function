use rand::Rng;
#[derive(Clone,Debug)]
pub struct Cell {
    pub id: usize,
    pub domain: Vec<usize>,
    pub collapsed_domain: Option<usize>,
    pub is_propagated: bool,
    pub is_collapsed:bool
}
impl Cell{
    pub fn collapse(&mut self){
        if self.domain.len() < 1 {
            println!("Cell {:?} has no domain",self.id);
        }
        self.is_collapsed = true;
        self.is_propagated = true;
        let random_index = rand::thread_rng().gen_range(0..self.domain.len());
        self.collapsed_domain = Some(self.domain[random_index]);
        self.domain = vec![self.collapsed_domain.unwrap()];
    }
}
pub fn new_cell(id:usize,initial_domains:Vec<usize>)->Cell{
    Cell{
        id : id,
        domain : initial_domains,
        collapsed_domain : None,
        is_propagated : false,
        is_collapsed : false
    }
}
