use crate::graphics::sprites::SpriteMaps;
use minifb::Window;

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
    /// The current date.
    pub current_date: chrono::NaiveDate,
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
    /// The buffer for the scaled graphics.
    pub scaled_buffer: &'a mut Vec<u32>,
    /// The width of the game world.
    pub art_width: usize,
    /// The height of the game world.
    pub art_height: usize,
    /// The prompt for the current state.
    pub prompt: &'a str,
}