use crate::state::structs::State;

/// Renders the pixel buffer to the screen or scales it for headless mode.
///
/// # Parameters
/// - `game_state`: A mutable reference to the `State` struct containing the game state.
/// - `headless`: A boolean indicating whether the rendering is done in headless mode.
///
/// In headless mode, the buffer is scaled but not displayed. Otherwise, the scaled buffer is drawn onto the window.
pub fn render_pixel_buffer(state: &mut State) {

    if !state.headless {
        // Ensure the window is initialized
        let window = state.window.as_mut().expect("Window should be initialized");

        // Draw the scaled buffer onto the window
        window.update_with_buffer(&state.window_buffer, state.window_width, state.window_height).unwrap();
    }
}