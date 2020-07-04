use crate::game::components::{Status};


pub struct Entity {
    id: usize,
    components_list: Vec<bool>,
}

impl Entity {

    pub fn new(id: usize) -> Entity {
        let mut components_list = Vec::with_capacity(2);
        components_list.push(true);
        components_list.push(true);
        Entity {
            id,
            components_list,
        }
    }

    pub fn add_component(&mut self) {
    }
}
