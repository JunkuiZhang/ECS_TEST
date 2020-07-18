// Window settings
pub static WINDOW_WIDTH: u32 = 1280;
pub static WINDOW_HEIGHT: u32 = 720;
pub static TITLE: &str = "Covid-19 Simulation";

// Population settings
pub static POPULATION_NUM: usize = 200;
pub const POP_DIST_KEPT_WALK_SPEED_FACTOR: f32 = 0.2;
pub const ENTITY_RADIUS: f32 = 3.0;
pub const ENTITY_MAX_SPEED: f32 = 2.0;

// Virus settings
pub static DIST_KEPT_INIT_PORTION: f32 = 0.5;
pub static INITIAL_CHANCE: f32 = 0.05;
pub static INFECTION_RADIUS: f32 = 20.0;
pub static INFECTION_CHANCE: f32 = 0.2;
