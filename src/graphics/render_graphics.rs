use crate::state::structs::State;

pub fn render_pixel_buffer(game_state: &mut State, headless: bool) {
    // Scale the buffer to the screen resolution
    scale_buffer(&game_state.window_buffer, &mut game_state.scaled_buffer, game_state.art_width, game_state.art_height, game_state.window_width, game_state.window_height);

    if !headless {
        // Ensure the window is initialized
        let window = game_state.window.as_mut().expect("Window should be initialized");

        // Draw the scaled buffer onto the window
        window.update_with_buffer(&game_state.scaled_buffer, game_state.window_width, game_state.window_height).unwrap();
    }
}

// Function to scale a buffer to a different resolution
fn scale_buffer(src: &[u32], dst: &mut [u32], src_width: usize, src_height: usize, dst_width: usize, dst_height: usize) {
    let x_ratio = src_width as f32 / dst_width as f32;
    let y_ratio = src_height as f32 / dst_height as f32;

    for y in 0..dst_height {
        for x in 0..dst_width {
            let src_x = (x as f32 * x_ratio).floor() as usize;
            let src_y = (y as f32 * y_ratio).floor() as usize;
            dst[y * dst_width + x] = src[src_y * src_width + src_x];
        }
    }
}