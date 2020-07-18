use specs::{Component, VecStorage};
use std::ops::{Mul, Add, Div, AddAssign};
use sfml::graphics::{CircleShape, Transformable, Shape, Color};
use crate::settings::{ENTITY_RADIUS, INFECTION_RADIUS};
use sfml::system::Vector2f;


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


/*
#[derive(Clone, Debug)]
pub struct EntityImage<'a> {
    pub entity_circle: Box<CircleShape<'a>>,
    pub radius_circle: Box<CircleShape<'a>>,
}

impl Component for EntityImage<'_> {
    type Storage = VecStorage<Self>;
}

impl<'a> Default for EntityImage<'_> {
    fn default() -> EntityImage<'a> {
        let mut ent_img = CircleShape::new(ENTITY_RADIUS, 30);
        let mut rad_img = CircleShape::new(INFECTION_RADIUS, 30);
        ent_img.set_origin(Vector2f::new(ENTITY_RADIUS, ENTITY_RADIUS));
        rad_img.set_origin(Vector2f::new(INFECTION_RADIUS, INFECTION_RADIUS));
        ent_img.set_fill_color(Color::BLACK);
        rad_img.set_fill_color(Color::TRANSPARENT);
        rad_img.set_outline_thickness(2.0);
        rad_img.set_outline_color(Color::RED);

        EntityImage {
            entity_circle: Box::new(ent_img),
            radius_circle: Box::new(rad_img),
        }
    }
}

impl<'a> EntityImage<'_> {
    pub fn generate(is_infected: bool, is_dist_kept: bool, x: f32, y: f32) -> EntityImage<'a> {
        let mut ent_img = CircleShape::new(ENTITY_RADIUS, 30);
        let mut rad_img = CircleShape::new(INFECTION_RADIUS, 30);
        ent_img.set_origin(Vector2f::new(ENTITY_RADIUS, ENTITY_RADIUS));
        rad_img.set_origin(Vector2f::new(INFECTION_RADIUS, INFECTION_RADIUS));
        ent_img.set_fill_color(Color::BLACK);
        rad_img.set_fill_color(Color::TRANSPARENT);
        rad_img.set_outline_thickness(2.0);
        rad_img.set_outline_color(Color::RED);

        if is_infected {
            ent_img.set_fill_color(Color::RED);
            rad_img.set_outline_color(Color::RED);
        }
        if is_dist_kept {
            ent_img.set_fill_color(Color::BLUE);
            rad_img.set_outline_color(Color::BLUE);
        }

        ent_img.set_position(Vector2f::new(x, y));
        rad_img.set_position(Vector2f::new(x, y));

        EntityImage {
            entity_circle: Box::new(ent_img),
            radius_circle: Box::new(rad_img),
        }
    }
}
*/

#[derive(Clone, Default, Debug)]
pub struct NeighborList {
    pub neighbors_direction: Option<Direction>,
    pub any_neighbor_infected: bool,
}

impl Component for NeighborList {
    type Storage = VecStorage<Self>;
}
