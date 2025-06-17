use std::collections::HashMap;
use chrono::NaiveDate;
use crate::graphics::sprites::SpriteMaps;
use minifb::Window;
use crate::state::constants::graphics::{WINDOW_HEIGHT, WINDOW_WIDTH};

/// Represents a camera in the simulation.
pub struct Camera {
    /// The x-coordinate of the camera.
    pub x: f32,
    /// The y-coordinate of the camera.
    pub y: f32,
}

impl Camera {
    /// Creates a new `Camera` instance.
    ///
    /// # Arguments
    ///
    /// * `x` - The initial x-coordinate of the camera.
    /// * `y` - The initial y-coordinate of the camera.
    ///
    /// # Returns
    ///
    /// A new `Camera` object.
    pub fn new(x: f32, y: f32) -> Self {
        Camera { x, y }
    }
}

/// Represents the state of the application.
pub struct State<'a> {
    /// The chosen date, either as of date or current.
    pub target_date: chrono::NaiveDate,
    /// The camera object.
    pub camera: Camera,
    /// The sprite maps used in the application.
    pub sprites: SpriteMaps,
    /// The buffer for the window.
    pub window_buffer: &'a mut Vec<u32>,
    /// The width of the window.
    pub window_width: usize,
    /// The height of the window.
    pub window_height: usize,
    /// The optional window object.
    pub window: Option<&'a mut Window>,
    /// The prompt for the current state.
    pub prompt: &'a str,
    /// Whether the application is running in headless mode
    pub headless: bool,
    /// Color map for the application
    pub color_map: Option<Vec<u8>>,
    /// Map from color to index for palette management
    pub color_to_index_map: Option<HashMap<u32, u8>>
}

impl State<'_> {
    pub fn new<'a>(
        target_date: NaiveDate,
        window_buffer: &'a mut Vec<u32>,
        window: Option<&'a mut Window>,
        prompt: &'a str,
        headless: bool,
        color_map: Option<Vec<u8>>,
        color_to_index_map: Option<HashMap<u32, u8>>,
    ) -> State<'a> {
        State {
            target_date,
            camera: Camera::new(0.0, 0.0),
            sprites: SpriteMaps::new(target_date),
            window_buffer,
            window_width: WINDOW_WIDTH,
            window_height: WINDOW_HEIGHT,
            window,
            prompt,
            headless,
            color_map,
            color_to_index_map,
        }
    }
}


