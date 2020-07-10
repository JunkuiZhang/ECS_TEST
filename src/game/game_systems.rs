use crate::game::components::*;
use crate::settings::{POPULATION_NUM, WINDOW_WIDTH, WINDOW_HEIGHT};
use super::sfml::graphics::Transformable;
use super::sfml::system::Vector2f;
use std::borrow::Borrow;


fn check_bound(pos: Position) -> Position {
    let mut res = Position {
        x: pos.x,
        y: pos.y
    };
    if pos.x < 0.0 {
        res = Position {
            x: WINDOW_WIDTH as f32 + pos.x,
            y: pos.y
        };
    }
    if pos.x > WINDOW_WIDTH as f32 {
        res = Position {
            x: pos.x - WINDOW_WIDTH as f32,
            y: pos.y
        };
    }
    if pos.y < 0.0 {
        res = Position {
            x: pos.x,
            y: WINDOW_HEIGHT as f32 + pos.y
        };
    }
    if pos.y > WINDOW_HEIGHT as f32 {
        res = Position {
            x: pos.x,
            y: pos.y - WINDOW_HEIGHT as f32
        };
    }

    res
}

pub fn sys_move(vel_list: &Vec<Box<Option<Velocity>>>, pos_list: &mut Vec<Box<Option<Position>>>, dt: f32) {
    for index in 0..POPULATION_NUM {
        if vel_list[index].is_none() || pos_list[index].is_none() {
            continue;
        }
        let vel = vel_list[index].clone().take().unwrap();
        let pos = pos_list[index].clone().take().unwrap();
        pos_list[index] = Box::new(Some(check_bound(pos + vel * dt)));
    }
}

pub fn sys_image_update(pos_list: &Vec<Box<Option<Position>>>, img_list: &mut Vec<Box<Option<Image>>>) {
    for index in 0..POPULATION_NUM {
        if pos_list[index].is_none() || img_list[index].is_none() {
            continue;
        }
        img_list[index].as_mut().as_mut().unwrap().image.set_position(pos_list[index].clone().unwrap().to_vector2f());
    }
}
