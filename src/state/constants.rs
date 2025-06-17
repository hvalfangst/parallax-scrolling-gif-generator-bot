pub mod graphics {
    pub const WINDOW_WIDTH: usize = 1024;
    pub const WINDOW_HEIGHT: usize = 1024;
    pub const MAX_GIF_FRAMES: usize = 40; // More frames equals smoother GIFs, but larger file sizes and thus slower rendering
    pub const CAMERA_X_INCREMENT: f32 = 20.0; // Speed of camera movement in pixels per frame
}

pub mod file_paths {
    pub const INPUT_IMAGE_PATH: &str = "images/image_current.png";
    pub const CURRENT_GIF_PATH: &str = "gifs/gif_current.gif";
    pub const CURRENT_PROMPT_PATH: &str = "prompts/prompt_current.txt";
}


