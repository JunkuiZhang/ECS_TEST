use specs::{Component, VecStorage};
use std::ops::{Mul, Add, Div, AddAssign};


#[derive(Clone, Debug, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}


#[derive(Clone, Debug, Default)]
pub struct Direction {
    pub x: f32,
    pub y: f32,
}

impl Mul<f32> for Direction {
    type Output = Direction;

    fn mul(self, rhs: f32) -> Direction {
        Direction {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Direction {
    type Output = Direction;

    fn div(self, rhs: f32) -> Direction {
        Direction {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Add<Direction> for Direction {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Direction {
        Direction {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Direction> for Direction {
    fn add_assign(&mut self, other: Direction) {
        *self = Direction {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Component for Direction {
    type Storage = VecStorage<Self>;
}


#[derive(Clone, Debug, Default)]
pub struct Status {
    pub id: usize,
    pub is_infected: bool,
    pub is_traveling: bool,
    pub is_dist_kept: bool,
}

impl Component for Status {
    type Storage = VecStorage<Self>;
}


#[derive(Clone, Default, Debug)]
pub struct NeighborList {
    pub neighbors_direction: Option<Direction>,
    pub any_neighbor_infected: bool,
}

impl Component for NeighborList {
    type Storage = VecStorage<Self>;
}
