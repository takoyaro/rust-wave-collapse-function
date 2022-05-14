use rand::Rng;
#[derive(Clone,Debug)]
pub struct Cell {
    pub ID: usize,
    pub DOMAIN: Vec<usize>,
    pub COLLAPSED_DOMAIN: Option<usize>,
    pub IS_PROPAGATED: bool,
    pub IS_COLLAPSED:bool
}
impl Cell{
    pub fn collapse(&mut self){
        if self.DOMAIN.len() < 1 {
            println!("Cell {:?} has no domain",self.ID);
        }
        self.IS_COLLAPSED = true;
        self.IS_PROPAGATED = true;
        let random_index = rand::thread_rng().gen_range(0..self.DOMAIN.len());
        self.COLLAPSED_DOMAIN = Some(self.DOMAIN[random_index]);
        self.DOMAIN = vec![self.COLLAPSED_DOMAIN.unwrap()];
    }
}
pub fn new_cell(id:usize,initial_domains:Vec<usize>)->Cell{
    Cell{
        ID : id,
        DOMAIN : initial_domains,
        COLLAPSED_DOMAIN : None,
        IS_PROPAGATED : false,
        IS_COLLAPSED : false
    }
}
