use crate::graphics::parallax::draw_parallax_layer;
use crate::graphics::sprites::draw_sprite;

use crate::state::structs::State;

/// Updates the pixel buffer by drawing the background and parallax layers.
///
/// # Parallax Effect
/// The function draws layers at different speeds to achieve the parallax effect,
/// where closer layers move faster and farther layers move slower relative to the camera.
/// This creates a sense of depth in the scene.
///
/// # Parameters
/// - `game_state`: A mutable reference to the current game state, containing camera position, window buffer, and sprite layers.
pub fn update_pixel_buffer(game_state: &mut State) {

    // Always draw the static background layer first in order to fill all pixels as the parallax effect can result in empty pixels
    draw_sprite(0, 0, &game_state.sprites.layer_1[0], game_state.window_buffer, game_state.art_width);

    // Draw each parallax layer
    draw_parallax_layer(game_state, 0, 16);
    draw_parallax_layer(game_state, 1, 6);
    draw_parallax_layer(game_state, 2, 4);
    draw_parallax_layer(game_state, 3, 1);
}