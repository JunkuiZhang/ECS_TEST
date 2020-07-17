use specs::{System, ReadStorage, WriteStorage, Read, Join};
use specs::prelude::*;
use super::components::*;
use crate::game::{DeltaTime};
use sfml::graphics::{Color, CircleShape, Transformable, Shape, RenderWindow, RenderTarget};
use crate::settings::*;
use sfml::system::Vector2f;
use std::ops::{Mul, Add, Div};
use rand::Rng;


pub struct EntityMove;

impl<'a> System<'a> for EntityMove {
    type SystemData = (
        Read<'a, DeltaTime>, WriteStorage<'a, Position>, ReadStorage<'a, Direction>,
        ReadStorage<'a, Status>
    );

    fn run(&mut self, (delta_time, mut pos, vel, status): Self::SystemData) {
        use specs::ParJoin;
        // use rayon::preclude::*;

        let dt = delta_time.dt;
        for (pos, vel, status) in (&mut pos, &vel, &status).join() {
            if !status.is_traveling {
                if status.is_dist_kept {
                    pos.x += vel.x * dt * ENTITY_MAX_SPEED * 60.0 * POP_DIST_KEPT_WALK_SPEED_FACTOR;
                    pos.y += vel.y * dt * ENTITY_MAX_SPEED * 60.0 * POP_DIST_KEPT_WALK_SPEED_FACTOR;
                } else {
                    pos.x += vel.x * dt * ENTITY_MAX_SPEED * 60.0;
                    pos.y += vel.y * dt * ENTITY_MAX_SPEED * 60.0;
                }
            } else {
                // Nothing
            }
        }
        // (&mut pos, &vel)
        //     .par_join()
        //     .for_each(|(pos, vel)| {
        //         pos.x += vel.x * dt;
        //         pos.y += vel.y * dt;
        //     });
    }
}

fn vector_length(vec: &Direction) -> f32 {
    (vec.x.powf(2.0) + vec.y.powf(2.0)).sqrt()
}

fn vector_normalization(vec: &Direction) -> Direction {
    let length = vector_length(vec);
    // let length = (vec.x.powf(2.0) + vec.y.powf(2.0)).sqrt();
    (*vec).clone() / length
}


pub struct EntityNeighborListUpdate;

impl<'a> System<'a> for EntityNeighborListUpdate {
    type SystemData = (ReadStorage<'a, Status>, ReadStorage<'a, Position>, ReadStorage<'a, Direction>, WriteStorage<'a, NeighborList>);

    fn run(&mut self, (status, position, direction, mut neighbor_list): Self::SystemData) {
        for (status1, position1, direction1, nl) in (&status, &position, &direction, &mut neighbor_list).join() {
            if !status1.is_dist_kept { continue; }

            let mut neighbor_dir = Direction { x: 0.0, y: 0.0};
            let mut flag = false;
            for (status2, position2, direction2) in (&status, &position, &direction).join() {
                if status1.id == status2.id { continue; }
                let dist = vector_length(&Direction {
                    x: position2.x - position1.x,
                    y: position2.y - position1.y,
                });

                if dist < INFECTION_RADIUS * 1.5 {
                    neighbor_dir += (*direction2).clone();
                    neighbor_dir = vector_normalization(&neighbor_dir);
                    flag = true;
                }
                if dist < INFECTION_RADIUS && status2.is_infected {
                    nl.any_neighbor_infected = true;
                }
            }

            if !flag {
                nl.neighbors_direction = None;
            } else {
                nl.neighbors_direction = Some(neighbor_dir);
            }
        }
    }
}


pub struct EntityGetNewDirection;

impl<'a> System<'a> for EntityGetNewDirection {
    type SystemData = (ReadStorage<'a, Status>, ReadStorage<'a, NeighborList>, WriteStorage<'a, Direction>);

    fn run(&mut self, (status, nl, mut velocity): Self::SystemData) {
        for (status, nl, velocity) in (&status, &nl, &mut velocity).join() {
            if let Some(neighbor_dir) = (nl.neighbors_direction).clone() {
                let mut dir = (*velocity).clone();
                dir = vector_normalization(&(dir + neighbor_dir * (-1.0)));
                (*velocity) = dir;
            } else {
                let mut dir = (*velocity).clone();
                let mut rand_gen = rand::thread_rng();
                let normal_dir = rand_distr::Normal::new(0.0, 1.0)
                    .expect("ERROR: Get random num from crate Rand_distr");
                let dir_x = rand_gen.sample(&normal_dir) as f32;
                let dir_y = rand_gen.sample(&normal_dir) as f32;
                dir += vector_normalization(&mut Direction { x: dir_x, y: dir_y });
                (*velocity) = vector_normalization(&dir);
            }
        }
    }
}


pub struct EntityInfoUpdate;

impl<'a> System<'a> for EntityInfoUpdate {
    type SystemData = (WriteStorage<'a, NeighborList>, WriteStorage<'a, Status>);

    fn run(&mut self, (mut nl, mut status): Self::SystemData) {
        for (nl, status) in (&mut nl, &mut status).join() {
            if nl.any_neighbor_infected {
                let mut rand_gen = rand::thread_rng();
                let unif = rand::distributions::Uniform::new(0.0, 1.0);
                if rand_gen.sample(&unif) < INFECTION_CHANCE {
                    status.is_infected = true;
                }
            }

            nl.neighbors_direction = None;
            nl.any_neighbor_infected = false;
        }
    }
}


pub mod renderer {
    use specs::prelude::*;
    use crate::game::components::*;
    use sfml::graphics::{RenderWindow, Color, CircleShape, Transformable, Shape, RenderTarget};
    use sfml::system::Vector2f;
    use crate::settings::*;

    pub type SystemData<'a> = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Status>,
    );

    pub fn render(
        window: &mut RenderWindow,
        (pos, status): SystemData,
    ) {
        window.clear(Color::WHITE);

        for (pos, status) in (&pos, &status).join() {
            let mut ent_color = Color::BLACK;
            let mut radius_color = Color::RED;
            if status.is_infected {
                ent_color = Color::RED;
            }
            if status.is_dist_kept {
                ent_color = Color::BLUE;
                radius_color = Color::BLUE;
            }
            if status.is_traveling {
                ent_color = Color::GREEN;
                radius_color = Color::GREEN;
            }

            let mut ent_image = CircleShape::new(ENTITY_RADIUS, 30);
            let mut radius_image = CircleShape::new(INFECTION_RADIUS, 30);

            ent_image.set_origin(Vector2f::new(ENTITY_RADIUS, ENTITY_RADIUS));
            radius_image.set_origin(Vector2f::new(INFECTION_RADIUS, INFECTION_RADIUS));

            ent_image.set_position(Vector2f::new(pos.x, pos.y));
            radius_image.set_position(Vector2f::new(pos.x, pos.y));

            ent_image.set_fill_color(ent_color);
            radius_image.set_fill_color(Color::TRANSPARENT);
            radius_image.set_outline_thickness(1.5);
            radius_image.set_outline_color(radius_color);

            window.draw(&ent_image);
            if status.is_infected {
                window.draw(&radius_image);
            }
        }

        window.display();
    }
}
