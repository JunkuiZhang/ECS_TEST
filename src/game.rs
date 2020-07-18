mod components;
mod systems;

use crate::settings::*;
use specs::prelude::*;
use specs::{World, WorldExt, Builder, Dispatcher, DispatcherBuilder};
use components::*;
use systems::*;
use sfml::system::{Clock, Vector2f};
use sfml::graphics::{RenderWindow, RenderTarget, Color, CircleShape, Transformable};
use sfml::window::{VideoMode, Style, ContextSettings, Event, Key};
use std::option::Option::Some;
use rand::Rng;


#[derive(Default)]
pub struct DeltaTime {
    pub dt: f32,
}


pub struct Game<'a, 'b> {
    window: RenderWindow,
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    clock: Clock,
}

impl<'a, 'b> Game<'_, '_> {

    pub fn new() -> Game<'a, 'b> {
        let window = RenderWindow::new(
            VideoMode::new(
                WINDOW_WIDTH, WINDOW_HEIGHT,
                VideoMode::desktop_mode().bits_per_pixel
            ),
            TITLE,
            Style::default(),
            &ContextSettings::default()
        );
        let mut dispatcher = DispatcherBuilder::new()
            .with(EntityNeighborListUpdate, "entity_nl_update", &[])
            .with(EntityGetNewDirection, "entity_new_dir", &["entity_nl_update"])
            .with(EntityInfoUpdate, "entity_info_update",&["entity_new_dir"])
            .with(EntityMove, "entity_move", &["entity_new_dir", "entity_info_update"])
            .build();
        let mut world = World::new();
        dispatcher.setup(&mut world);
        renderer::SystemData::setup(&mut world);
        let clock = Clock::default();
        Game {
            window,
            world,
            dispatcher,
            clock,
        }
    }

    fn images_init(&mut self) {

    }

    fn init(&mut self) {
        let mut rand_gen = rand::thread_rng();
        let unif_width = rand::distributions::Uniform::new(0.0, WINDOW_WIDTH as f32);
        let unif_height = rand::distributions::Uniform::new(0.0, WINDOW_HEIGHT as f32);
        let unif_prob = rand::distributions::Uniform::new(0.0, 1.0 as f32);
        for i in 0..POPULATION_NUM {
            let x = rand_gen.sample(&unif_width);
            let y = rand_gen.sample(&unif_height);

            let mut is_infected = false;
            if rand_gen.sample(&unif_prob) < INITIAL_CHANCE { is_infected = true; }

            let mut is_dist_kept = false;
            if rand_gen.sample(&unif_prob) < DIST_KEPT_INIT_PORTION { is_dist_kept = true; }

            self.world.create_entity()
                .with(Position { x, y, })
                .with(Direction { x: 1.0, y: 0.0 })
                .with(Status {
                    id: i,
                    is_infected,
                    is_traveling: false,
                    is_dist_kept,
                })
                .with(NeighborList {
                    neighbors_direction: None,
                    any_neighbor_infected: false,
                })
                .build();

        }
    }

    pub fn run(&mut self) {
        self.init();
        while self.window.is_open() {
            let dt = self.clock.restart().as_seconds();
            self.events();
            *(self.world.write_resource::<DeltaTime>()) = DeltaTime { dt, };
            self.dispatcher.dispatch(&mut self.world);
            self.world.maintain();
            renderer::render(&mut self.window, self.world.system_data());
        }
    }

    fn events(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed {code: Key::Escape, ..} => self.window.close(),
                _ => {}
            }
        }
    }
}
