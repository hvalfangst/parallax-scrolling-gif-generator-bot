use chrono::NaiveDate;
use image::{GenericImageView, ImageBuffer, Rgba};
use crate::graphics::sprites::draw_sprite;
use crate::state::structs::State;

/// Creates parallax layers from an input image and saves them as separate files.
///
/// # Parameters
/// - `input_path`: The file path to the input image.
/// - `current_date`: The current date used for naming the output files.
///
/// # Returns
/// - `Result<(), Box<dyn std::error::Error>>`: Returns `Ok(())` if successful, or an error if something goes wrong.
///
/// # Functionality
/// - This function loads an input image and splits it into multiple layers based on predefined heights.
/// - Each layer corresponds to a specific section of the image and is saved as a separate file.
/// - The layers are saved in the `layers/` directory, with subdirectories named after the layer index.
///
/// # Constraints
/// - The input image must be 1024x1024 pixels. If the dimensions are incorrect, the function returns an error.
///
/// # Example
/// ```
/// create_parallax_layers("input.png", NaiveDate::from_ymd(2023, 10, 1))?;
/// ```
/// This will generate layers and save them in the `layers/` directory.
pub fn create_parallax_layers(input_path: &str, current_date: NaiveDate) -> Result<(), Box<dyn std::error::Error>> {
    // Load the input image
    let img = image::open(input_path)?;
    let (width, height) = img.dimensions();

    // Ensure the input image has the correct dimensions
    if width != 1024 || height != 1024 {
        return Err("Input image must be 1024x1024.".into());
    }

    // Define the layer heights (each layer is 256 pixels tall)
    let layer_heights = [256, 512, 768, 1024];

    for (i, _) in layer_heights.iter().enumerate() {
        // Create a new image buffer for the layer
        let layer = ImageBuffer::from_fn(width, height, |x, y| {
            // Include pixels within the current layer's height range
            if y >= (i * 256) as u32 && y < ((i + 1) * 256) as u32 {
                img.get_pixel(x, y)
            } else {
                Rgba([0, 0, 0, 0]) // Transparent pixel for areas outside the layer
            }
        });

        // Define the output directory and file path for the layer
        let output_dir = format!("layers/{}", i + 1);
        let output_path = format!("{}/layer_{}.png", output_dir, current_date);

        // Log the saved layer's file path
        println!("Layer {} saved to {}", i + 1, output_path);

        // Save the layer to the specified file path
        layer.save(output_path)?;
    }

    Ok(())
}

/// Draws a parallax layer onto the window buffer based on the game state.
///
/// # Parameters
/// - `game_state`: A mutable reference to the current game state, containing camera position, window buffer, and sprite layers.
/// - `layer_index`: The index of the parallax layer to draw (0 to 3).
/// - `divisor`: A divisor used to calculate the horizontal offset for the parallax effect.
///
/// # Parallax Effect
/// The parallax effect is a visual technique used in 2D games to create a sense of depth and immersion.
/// It simulates the way objects at different distances appear to move at different speeds relative to the viewer.
/// This function calculates the offset for the layer based on the camera position and divisor, selects the appropriate layer from the game state,
/// and uses the `draw_sprite` function to render the layer onto the window buffer.
///
/// Layers closer to the camera move faster, while layers farther away move slower, creating the illusion of depth.
///
/// # Implementation Details
/// - The `offset_x` is calculated using the camera's horizontal position divided by the divisor and wrapped around the texture width.
/// - The `offset_y` is calculated using the camera's vertical position divided by a fixed value.
/// - The appropriate layer is selected based on the `layer_index`.
/// - The `draw_sprite` function is used to render the layer onto the window buffer.
pub fn draw_parallax_layer(game_state: &mut State, layer_index: usize, divisor: usize) {
    let texture_width = game_state.art_width;

    let offset_x = game_state.camera.x as usize / divisor % texture_width;
    let offset_y = game_state.camera.y as usize / 666;

    let layer = match layer_index {
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