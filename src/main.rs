use minifb::{Window, WindowOptions};
use crate::graphics::color::extract_palette;
use crate::graphics::sprites::SpriteMaps;
use crate::state::constants::file_paths::INPUT_IMAGE_PATH;
use crate::state::constants::graphics::{SCALED_WINDOW_HEIGHT, SCALED_WINDOW_WIDTH};
use crate::state::event_loop::start_gif_recording_loop;
use crate::state::structs::State;
use crate::utils::misc::{create_parallax_layers_for_date, generate_and_save_image, initialize_generators, parse_headless_mode, prepare_python_interpreter};

mod graphics; mod state; mod utils; mod generators;

fn main() {
    prepare_python_interpreter();

    let (prompt_generator, image_generator) = initialize_generators();
    let current_date = chrono::Utc::now().date_naive();

    let prompt_result = generate_and_save_image(&prompt_generator, &image_generator, current_date);
    if let Err(e) = prompt_result {
        eprintln!("Error during image generation: {}", e);
        return;
    }

    let (color_map, color_to_index_map) = match extract_palette(INPUT_IMAGE_PATH) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to extract palette: {}", e);
            return;
        }
    };

    if let Err(e) = create_parallax_layers_for_date(INPUT_IMAGE_PATH, current_date) {
        eprintln!("Error during parallax layer creation: {}", e);
        return;
    }

    let headless_mode = parse_headless_mode();
    let sprites = SpriteMaps::new(current_date);
    let mut window_buffer = vec![0; 1024 * 1024];
    let mut scaled_buffer = vec![0; SCALED_WINDOW_WIDTH * SCALED_WINDOW_HEIGHT];
    let camera = state::structs::Camera::new(0.0, 0.0);

    let mut window = if !headless_mode {
        Some(Window::new(
            "Parallax Scrolling GIF Exporter",
            SCALED_WINDOW_WIDTH,
            SCALED_WINDOW_HEIGHT,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        }))
    } else {
        None
    };

    let binding = prompt_result.unwrap();
    let state = State {
        current_date,
        camera,
        sprites,
        window_buffer: &mut window_buffer,
        window_width: SCALED_WINDOW_WIDTH,
        window_height: SCALED_WINDOW_HEIGHT,
        window: window.as_mut(),
        scaled_buffer: &mut scaled_buffer,
        art_width: 1024,
        art_height: 1024,
        prompt: binding.as_str()
    };

    start_gif_recording_loop(state, headless_mode, color_map, color_to_index_map);
}