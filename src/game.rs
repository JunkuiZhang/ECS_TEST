extern crate sfml;
extern crate rand;
mod components;
mod entities;
mod game_systems;
mod util;

use sfml::{graphics, window, system};
use std::option::Option::Some;
use self::sfml::graphics::RenderTarget;
use crate::settings::{POPULATION_NUM, INITIAL_CHANCE, WINDOW_WIDTH, WINDOW_HEIGHT, DIST_KEPT_INIT_PORTION};
use self::rand::Rng;
use self::sfml::system::Vector2f;


pub struct Game {
    window: graphics::RenderWindow,
    clock: system::Clock,
    entity_list: Vec<entities::Entity>,
    movement_list: Vec<components::Movement>,
    neighbor_list_list: Vec<components::NeighborList>,
    status_list: Vec<components::Status>,
    image_list: Vec<components::Image>,
}

impl Game {

    pub fn new(width: u32, height:u32, title: &str) -> Game {
        let window = graphics::RenderWindow::new(window::VideoMode::new(width, height, window::VideoMode::desktop_mode().bits_per_pixel),
                                                                title, window::Style::default(), &window::ContextSettings::default());
        let mut entity_list = Vec::with_capacity(POPULATION_NUM);
        let mut movement_list = Vec::with_capacity(POPULATION_NUM);
        let mut neighbor_list_list = Vec::with_capacity(POPULATION_NUM);
        let mut status_list = Vec::with_capacity(POPULATION_NUM);
        let mut image_list = Vec::with_capacity(POPULATION_NUM);

        let mut rand_gen = rand::thread_rng();
        for id in 0..POPULATION_NUM {
            entity_list.push(entities::Entity::new(id));

            let unif1 = rand::distributions::Uniform::new(0.0, WINDOW_WIDTH as f32);
            let unif2 = rand::distributions::Uniform::new(0.0, WINDOW_HEIGHT as f32);
            let pos = Vector2f::new(rand_gen.sample(&unif1), rand_gen.sample(&unif2));
            movement_list.push(components::Movement {id, position: pos, velocity: Vector2f::new(1.0, 0.0)});

            neighbor_list_list.push(components::NeighborList {id, new_dir: Vector2f::new(0.0, 0.0),
                                                                    neighbor_infected: false});

            let unif = rand::distributions::Uniform::new(0.0, 1.0);
            let mut infected_indicator = false;
            if rand_gen.sample(&unif) < INITIAL_CHANCE { infected_indicator = true; }
            let mut dist_indicator = false;
            if rand_gen.sample(&unif) < DIST_KEPT_INIT_PORTION { dist_indicator = true; }
            status_list.push(components::Status {id, is_infected: infected_indicator,
                                                is_traveling: false, is_dist_kept: dist_indicator});

            image_list.push(components::Image::new(id, infected_indicator, pos));
        }
        Game {
            window,
            clock: system::Clock::default(),
            entity_list,
            movement_list,
            neighbor_list_list,
            status_list,
            image_list,
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
        for id_ in 0..POPULATION_NUM {
            self.neighbor_list_list[id_].neighbor_list_update(&self.movement_list, &self.status_list);
            self.status_list[id_].status_update(&self.neighbor_list_list);
            self.movement_list[id_].move_(dt, &mut self.image_list, &self.neighbor_list_list, &self.status_list);
            self.image_list[id_].draw(&mut self.window, &self.status_list);
        }

        self.window.display();
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            self.events();
            let dt = self.clock.restart().as_seconds();
            self.update_and_draw(dt);
        }
    }
}