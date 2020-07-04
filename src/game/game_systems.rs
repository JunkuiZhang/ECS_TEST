use crate::game::components::{Movement, Status};

pub trait Move {
    fn move_(&mut self, dt: f32);
}

pub trait StatusUpdate {
    fn status_update(&mut self, pos_components: &Vec<Movement>, status_components: &Vec<Status>);
}

pub trait ImageUpdate {
    fn image_update(&mut self, current_status: Status);
}
