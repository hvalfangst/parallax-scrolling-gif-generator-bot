use crate::graphics::sprites::SpriteMaps;
use minifb::Window;

pub struct Camera {
    pub x: f32,
    pub y: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32) -> Self {
        Camera {
            x,
            y
        }
    }
}

pub struct State<'a> {
    pub current_date: chrono::NaiveDate, // Current date
    pub camera: Camera, // Camera object
    pub sprites: SpriteMaps, // Sprite maps
    pub window_buffer: &'a mut Vec<u32>, // Window buffer
    pub window_width: usize, // Width of the window
    pub window_height: usize, // Height of the window
    pub window: Option<&'a mut Window>, // Optional window object
    pub scaled_buffer: &'a mut Vec<u32>, // Scaled buffer
    pub art_width: usize, // Width of the game world
    pub art_height: usize, // Height of the game world
    pub prompt: &'a str, // Prompt for the current state
}