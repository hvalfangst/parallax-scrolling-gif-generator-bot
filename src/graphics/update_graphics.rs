use crate::graphics::sprites::draw_sprite;

use crate::state::structs::State;

pub fn update_pixel_buffer(game_state: &mut State) {
    let texture_width = game_state.art_width;

    // Always draw the static background layer first in order to fill all pixels as the parallax effect can result in empty pixels
    draw_sprite(0, 0, &game_state.sprites.layer_1[0], game_state.window_buffer, game_state.art_width);

    // Loop through the layers and draw them based on the player's position in relation to the divisor to achieve parallax scrolling
    for (i, divisor) in [16, 10, 6, 1].iter().enumerate() {

        // // Layer 0 will have offset divided by 16, layer 1 by 6, layer 2 by 4, and layer 3 by 1
        let offset_x = game_state.camera.x as usize / divisor % texture_width;
        let offset_y = game_state.camera.y as usize / 666;

        let layer = match i {
            0 => &game_state.sprites.layer_1[0],
            1 => &game_state.sprites.layer_2[0],
            2 => &game_state.sprites.layer_3[0],
            3 => &game_state.sprites.layer_4[0],
            _ => unreachable!(),
        };

        draw_sprite(
            (game_state.window_width).saturating_sub(offset_x),
            offset_y,
            layer,
            game_state.window_buffer,
            game_state.art_width,
        );
    }
}