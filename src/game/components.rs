extern crate rand;
extern crate rand_distr;

use super::sfml::system::{Vector2f};
use super::sfml::graphics::{CircleShape, Transformable, Shape, Color, RenderWindow, RenderTarget};
use super::util::{vector_length, vector_normalize, vector_normalize_with_len};
use crate::settings::{INFECTION_RADIUS, INFECTION_CHANCE, POPULATION_NUM, ENTITY_RADIUS, WINDOW_WIDTH,
                      WINDOW_HEIGHT, ENTITY_MAX_SPEED};
use self::rand::Rng;
use std::ops::{Add, Mul};


pub struct Movement {
    pub position: Vector2f,
    pub velocity: Vector2f,
}

#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {

    pub fn new(x: f32, y: f32) -> Position {
        Position { x, y }
    }

    pub fn to_vector2f(&self) -> Vector2f {
        Vector2f::new(self.x, self.y)
    }
}

#[derive(Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {

    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity { x, y }
    }

    pub fn to_vector2f(&self) -> Vector2f {
        Vector2f::new(self.x, self.y)
    }
}

impl Add<Velocity> for Position {
    type Output = Position;
    fn add(self, rhs: Velocity) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<f32> for Velocity {
    type Output = Velocity;
    fn mul(self, rhs: f32) -> Self::Output {
        Velocity {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}


pub struct NeighborList {
    pub id: usize,
    pub new_dir: Vector2f,
    pub neighbor_infected: bool,
}

impl NeighborList {
    pub fn neighbor_list_update(&mut self, pos_list: &Vec<Movement>, status_list: &Vec<Status>) {
        self.neighbor_infected = false;
        let mut res = Vector2f::new(0.0, 0.0);
        for id_ in 0..POPULATION_NUM {
            if self.id == id_ { continue; }
            let direct = pos_list[id_].position - pos_list[self.id].position;
            let len = vector_length(&direct);
            if len < INFECTION_RADIUS * 1.5 {
                res = vector_normalize(&(res + vector_normalize_with_len(&direct, len)));
                if len < INFECTION_RADIUS && status_list[id_].is_infected {
                    self.neighbor_infected = true;
                }
            }
        }

        self.new_dir = res;
    }
}


pub struct Status {
    pub id: usize,
    pub is_infected: bool,
    pub is_traveling: bool,
    pub is_dist_kept: bool,
}

impl Status {
    pub fn status_update(&mut self, neighbor_list: &Vec<NeighborList>) {
        if self.is_infected { return; }

        if neighbor_list[self.id].neighbor_infected {
            let mut rand_gen = rand::thread_rng();
            let unif = rand::distributions::Uniform::new(0.0, 1.0);
            if rand_gen.sample(&unif) < INFECTION_CHANCE {
                self.is_infected = true;
            }
        }
    }
}


pub struct Image {
    pub id: usize,
    pub image: CircleShape<'static>,
    pub radius_image: CircleShape<'static>,
}

impl Image {

    pub fn new(id: usize, is_infected: bool, pos: Vector2f) -> Image {
        let mut image = CircleShape::new(ENTITY_RADIUS, 50);
        image.set_origin(Vector2f::new(ENTITY_RADIUS, ENTITY_RADIUS));
        image.set_position(pos);
        let mut color = Color::BLACK;
        if is_infected { color = Color::RED; }
        image.set_fill_color(color);

        let mut radius_image = CircleShape::new(INFECTION_RADIUS, 30);
        radius_image.set_origin(Vector2f::new(INFECTION_RADIUS, INFECTION_RADIUS));
        radius_image.set_position(pos);
        radius_image.set_fill_color(Color::TRANSPARENT);
        radius_image.set_outline_thickness(1.5);
        radius_image.set_outline_color(Color::RED);

        Image {
            id,
            image,
            radius_image,
        }
    }

    fn color_update(&mut self, status: &Vec<Status>) -> bool {
        let s = &status[self.id];
        let mut ent_color = Color::BLACK;
        let mut rad_color = Color::RED;
        if s.is_dist_kept {
            ent_color = Color::BLUE;
            rad_color = Color::BLUE;
        }
        if s.is_infected {
            ent_color = Color::RED
        }
        if s.is_traveling {
            ent_color = Color::GREEN;
            rad_color = Color::GREEN;
        }

        self.image.set_fill_color(ent_color);
        self.radius_image.set_outline_color(rad_color);
        return s.is_infected;
    }

    // pub fn draw(&mut self, win: &mut RenderWindow, status: &Vec<Status>) {
    pub fn draw(&mut self, win: &mut RenderWindow) {
        // let indicator = self.color_update(status);
        win.draw(&self.image);
        // if indicator {
        //     win.draw(&self.radius_image);
        // }
    }

    fn update_pos(&mut self, pos: Vector2f) {
        self.image.set_position(pos);
        self.radius_image.set_position(pos);
    }
}
