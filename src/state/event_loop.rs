use crate::graphics::gif::{initialize_gif_encoder, process_frame};
use crate::graphics::render_graphics::render_pixel_buffer;
use crate::graphics::update_graphics::update_pixel_buffer;
use crate::state::constants::graphics::MAX_GIF_FRAMES;
use crate::state::structs::State;
use crate::utils::misc::{finalize_gif_encoding, is_window_open, should_process_frame, simulate_camera_movement};
use std::fs::File;
use std::process::exit;
use std::time::Instant;

pub fn start_gif_recording_loop(mut state: State) {
    let (width, height) = (state.window_width as u16, state.window_height as u16);
    let path = format!("gifs/gif_{}.gif", state.target_date);
    let mut image = File::create(&path).unwrap();
    let mut encoder = initialize_gif_encoder(&mut image, width, height);
    let mut frame_count = 0;
    let mut last_update = Instant::now();

    loop {
        if !state.headless && !is_window_open(&state) {
            break;
        }

        update_pixel_buffer(&mut state);
        render_pixel_buffer(&mut state);
        simulate_camera_movement(&mut state);

        if should_process_frame(&last_update) {
            if frame_count < MAX_GIF_FRAMES {
                process_frame(state.window_buffer, &mut encoder, &mut frame_count, &state.color_map.clone(), &mut state.color_to_index_map.clone());
                last_update = Instant::now();
            } else {
                finalize_gif_encoding(state, frame_count, path.as_str());
                exit(0);
            }
        }
    }
}