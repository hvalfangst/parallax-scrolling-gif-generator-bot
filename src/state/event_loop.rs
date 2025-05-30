use crate::graphics::gif::{initialize_gif_encoder, process_frame};
use crate::graphics::render_graphics::render_pixel_buffer;
use crate::graphics::update_graphics::update_pixel_buffer;
use crate::state::constants::graphics::MAX_GIF_FRAMES;
use crate::state::structs::State;
use minifb::Key;
use std::fs::File;
use std::process::exit;
use std::time::Instant;
use crate::utils;

pub fn start_event_loop(mut state: State, headless: bool) {
    let (width, height) = (state.window_width as u16, state.window_height as u16);
    let path = format!("gifs/gif_{}.gif", state.current_date);
    let mut image = File::create(&path).unwrap();
    let mut encoder = initialize_gif_encoder(&mut image, width, height);

    println!(
        "Starting GIF encoding in {} mode with dimensions: {}x{}",
        if headless { "headless" } else { "windowed" },
        width,
        height
    );

    let mut frame_count = 0;
    let mut last_update = Instant::now();

    loop {
        if !headless && !is_window_open(&state) {
            break;
        }

        update_pixel_buffer(&mut state);
        render_pixel_buffer(&mut state, headless);
        simulate_camera_movement(&mut state);

        if should_process_frame(&last_update) {
            if frame_count < MAX_GIF_FRAMES {
                process_frame(&mut state, &mut encoder, width, height, &mut frame_count);
                last_update = Instant::now();
            } else {
                finalize_gif_encoding(state, frame_count, path.as_str());
                exit(0);
            }
        }
    }
}

fn is_window_open(state: &State) -> bool {
    if let Some(window) = &state.window {
        window.is_open() && !window.is_key_down(Key::Escape)
    } else {
        true
    }
}

fn simulate_camera_movement(state: &mut State) {
    state.camera.x += 5.0;
}

fn should_process_frame(last_update: &Instant) -> bool {
    last_update.elapsed() >= std::time::Duration::from_nanos(0)
}

fn finalize_gif_encoding(state: State, frame_count: usize, path: &str) {
    println!("Finished capturing {} frames to file '{}'", frame_count, path);

    let current_gif_path = "gifs/gif_current.gif";
    std::fs::copy(path, current_gif_path).expect("Failed to copy GIF to 'current.gif'");
    println!("GIF copied to '{}'", current_gif_path);

    match utils::file_manager::FileManager::update_readme(state.prompt) {
        Ok(_) => println!("README updated successfully."),
        Err(e) => eprintln!("Failed to update README: {}", e),
    }
}