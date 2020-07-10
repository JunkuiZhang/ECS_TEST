extern crate sfml;
extern crate rand;
mod components;
mod entities;
mod game_systems;
mod util;

use sfml::{graphics, window, system};
use std::option::Option::Some;
use self::sfml::graphics::RenderTarget;
use crate::settings::{POPULATION_NUM, INITIAL_CHANCE, WINDOW_WIDTH, WINDOW_HEIGHT, DIST_KEPT_INIT_PORTION, ENTITY_MAX_SPEED};
use self::rand::Rng;
use self::sfml::system::Vector2f;
use crate::game::game_systems::{sys_move, sys_image_update};
use crate::game::components::{Velocity, Position};
use entities::Entity;
use components::Image;


enum GameStates {
    GamePaused,
    GamePlaying,
}


pub struct Game {
    window: graphics::RenderWindow,
    clock: system::Clock,
    entity_list: Vec<Box<Entity>>,
    image_list: Vec<Box<Option<Image>>>,
    comp_velocity: Vec<Box<Option<Velocity>>>,
    comp_position: Vec<Box<Option<Position>>>,
}

impl Game {

    pub fn new(width: u32, height:u32, title: &str) -> Game {
        let window = graphics::RenderWindow::new(window::VideoMode::new(width, height, window::VideoMode::desktop_mode().bits_per_pixel),
                                                                title, window::Style::default(), &window::ContextSettings::default());
        let mut entity_list = Vec::with_capacity(POPULATION_NUM);
        let mut image_list = Vec::with_capacity(POPULATION_NUM);
        let mut comp_velocity = Vec::with_capacity(POPULATION_NUM);
        let mut comp_position = Vec::with_capacity(POPULATION_NUM);

        let mut rand_gen = rand::thread_rng();
        for id in 0..POPULATION_NUM {
            entity_list.push(Box::new(Entity::new(id)));

            let unif1 = rand::distributions::Uniform::new(0.0, WINDOW_WIDTH as f32);
            let unif2 = rand::distributions::Uniform::new(0.0, WINDOW_HEIGHT as f32);
            let pos = Position::new(rand_gen.sample(&unif1), rand_gen.sample(&unif2));

            comp_position.push(Box::new(Some(pos.clone())));
            comp_velocity.push(Box::new(Some(Velocity::new(ENTITY_MAX_SPEED as f32, 0.0))));
            image_list.push(Box::new(Some(Image::new(id, false, pos.to_vector2f()))));
        }
        Game {
            window,
            clock: system::Clock::default(),
            entity_list,
            image_list,
            comp_position,
            comp_velocity,
        }
    }

    fn events(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                window::Event::Closed | window::Event::KeyPressed {code: window::Key::Escape, ..} => self.window.close(),
                _ => {},
            }
        }
    }

    fn update_and_draw(&mut self, dt: f32) {
        self.window.clear(graphics::Color::WHITE);
        // Draw
        sys_move(&self.comp_velocity, &mut self.comp_position, dt);
        sys_image_update(&self.comp_position, &mut self.image_list);
        for id_ in 0..POPULATION_NUM {
            self.image_list[id_].as_mut().as_mut().unwrap().draw(&mut self.window);
        }

        self.window.display();
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            self.events();
            let dt = self.clock.restart().as_seconds();
            println!("FPS: {:.0}", 1.0 / dt);
            self.update_and_draw(dt);
        }
    }
}