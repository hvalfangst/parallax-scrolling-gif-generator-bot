use crate::state::structs::State;

/// Renders the pixel buffer to the screen or scales it for headless mode.
///
/// # Parameters
/// - `game_state`: A mutable reference to the `State` struct containing the game state.
/// - `headless`: A boolean indicating whether the rendering is done in headless mode.
///
/// In headless mode, the buffer is scaled but not displayed. Otherwise, the scaled buffer is drawn onto the window.
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

/// Scales a source buffer to a destination buffer with a different resolution.
///
/// # Parameters
/// - `src`: A slice of `u32` representing the source buffer.
/// - `dst`: A mutable slice of `u32` representing the destination buffer.
/// - `src_width`: The width of the source buffer in pixels.
/// - `src_height`: The height of the source buffer in pixels.
/// - `dst_width`: The width of the destination buffer in pixels.
/// - `dst_height`: The height of the destination buffer in pixels.
///
/// This function uses nearest-neighbor scaling to map pixels from the source buffer to the destination buffer.
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