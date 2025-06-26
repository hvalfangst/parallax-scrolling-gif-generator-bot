use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::process::exit;
use std::time::Instant;
use chrono::NaiveDate;
use minifb::Key;
use crate::{generators, utils};
use crate::graphics::color::extract_palette;
use crate::graphics::parallax::create_parallax_layers;
use crate::state::constants::file_paths::CURRENT_GIF_PATH;
use crate::state::constants::graphics::CAMERA_X_INCREMENT;
use crate::state::structs::State;

/// Utility functions for initializing and managing Python interpreters, generators,
/// and handling image generation and processing.

/// Prepares the Python interpreter for free-threaded use.
/// This is required when using the `pyo3` crate to ensure Python's GIL (Global Interpreter Lock)
/// is properly managed in multithreaded environments.
pub fn prepare_python_interpreter() {
    pyo3::prepare_freethreaded_python();
}

/// Initializes the prompt and image generators using the OpenAI API key.
///
/// # Returns
/// A tuple containing:
/// - `PromptGenerator`: An instance of the prompt generator.
/// - `ImageGenerator`: An instance of the image generator.
///
/// # Panics///  if the `OPENAI_API_KEY` environment variable is not set or invalid.
pub fn initialize_generators() -> (generators::prompt_generator::PromptGenerator, generators::image_generator::ImageGenerator) {
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        panic!("Environment variable OPENAI_API_KEY is not set or invalid.");
    });

    let prompt_generator = generators::prompt_generator::PromptGenerator::new(api_key.clone());
    let image_generator = generators::image_generator::ImageGenerator::new(api_key);

    (prompt_generator, image_generator)
}

/// Generates an image based on a prompt and saves it to disk.
///
/// # Arguments
/// - `prompt_generator`: Reference to the prompt generator instance.
/// - `image_generator`: Reference to the image generator instance.
/// - `current_date`: The current date used for naming the saved files.
///
/// # Returns
/// `Ok((String))` with the prompt if the image is successfully generated and saved, otherwise an error.
pub fn generate_and_save_image(
    prompt_generator: &generators::prompt_generator::PromptGenerator,
    image_generator: &generators::image_generator::ImageGenerator,
    current_date: NaiveDate,
) -> Result<String, Box<dyn Error>> {


    let start_time_text_prompt = Instant::now();
    let prompt = prompt_generator.generate_prompt()?;
    let elapsed_time_text_prompt = start_time_text_prompt.elapsed();
    println!("\n*************  Generated prompt: {} in {} seconds ************* ", prompt, elapsed_time_text_prompt.as_secs_f64());

    let start_time_image_generation = Instant::now();
    match image_generator.generate_image(prompt.as_str()) {
        Ok(image_data) => {
            let elapsed_time_image_generation = start_time_image_generation.elapsed();
            println!("\n*************  Image generated with size {} bytes in {} seconds ************* ", image_data.len(), elapsed_time_image_generation.as_secs_f64());

            let start_time_save_files = Instant::now();
            utils::file_manager::FileManager::save_prompt(prompt.as_str(), current_date)?;
            utils::file_manager::FileManager::save_image(&image_data, current_date)?;
            let elapsed_time_save_files = start_time_save_files.elapsed();
            println!("\n*************  Files saved in {} seconds ************* ", elapsed_time_save_files.as_secs_f64());
        }
        Err(e) => {
            eprintln!("Error generating image: {}", e);
            return Err(Box::<dyn Error>::from(e));
        }
    }

    Ok(prompt)
}

/// Creates parallax layers for a given date based on an input image.
///
/// # Arguments
/// - `input_image_path`: Path to the input image file.
/// - `current_date`: The current date used for naming the generated layers.
///
/// # Returns
/// `Ok(())` if the layers are successfully created, otherwise an error.
pub fn create_parallax_layers_for_date(input_image_path: &str, current_date: NaiveDate) -> Result<(), Box<dyn Error>> {
    println!("Creating parallax layers for date: {}", current_date);

    create_parallax_layers(input_image_path, current_date).map_err(|e| {
        eprintln!("Error creating parallax layers: {}", e);
        Box::<dyn Error>::from(e)
    })?;

    println!("Parallax layers for date {} created successfully.", current_date);
    Ok(())
}

/// Parses command-line arguments to determine if the application should run in headless mode.
///
/// # Returns
/// `true` if the `--headless` flag is present in the arguments, otherwise `false`.
pub fn parse_headless_mode() -> bool {
    let args: Vec<String> = env::args().collect();
    let headless = args.get(1).map(|s| s == "--headless").unwrap_or(false) ||
        args.get(2).map(|s| s == "--headless").unwrap_or(false);
    println!("Headless mode activated: {}", headless);
    headless
}

/// Checks if the application window is open and not in a closed state.
///
/// # Arguments
/// - `state`: A reference to the current application state.
///
/// # Returns
/// `true` if the window is open and the Escape key is not pressed, otherwise `false`.
pub fn is_window_open(state: &State) -> bool {
    if let Some(window) = &state.window {
        window.is_open() && !window.is_key_down(Key::Escape)
    } else {
        true
    }
}

/// Simulates camera movement by incrementing its x-coordinate.
///
/// # Arguments
/// - `state`: A mutable reference to the current application state.
pub fn simulate_camera_movement(state: &mut State) {
    state.camera.x += CAMERA_X_INCREMENT;
}

/// Determines whether a frame should be processed based on the elapsed time.
///
/// # Arguments
/// - `last_update`: A reference to the timestamp of the last frame update.
///
/// # Returns
/// `true` if the elapsed time since the last update meets the threshold, otherwise `false`.
pub fn should_process_frame(last_update: &Instant) -> bool {
    last_update.elapsed() >= std::time::Duration::from_nanos(0)
}

/// Finalizes the GIF encoding process and updates the README file.
///
/// # Arguments
/// - `state`: The current application state.
/// - `frame_count`: The total number of frames captured.
/// - `path`: The file path where the GIF is saved.
pub fn finalize_gif_encoding(state: State, frame_count: usize, path: &str) {
    println!("Finished capturing {} frames to file '{}'", frame_count, path);

    std::fs::copy(path, CURRENT_GIF_PATH).expect("Failed to copy GIF to 'current.gif'");
    println!("GIF copied to '{}'", CURRENT_GIF_PATH);

    match utils::file_manager::FileManager::update_readme(state.prompt) {
        Ok(_) => println!("README updated successfully."),
        Err(e) => eprintln!("Failed to update README: {}", e),
    }
}

/// Extracts the color palette from an image or exits the program on failure.
///
/// This function attempts to extract the color palette from the specified image file.
/// If the extraction fails, an error message is printed, and the program exits with a status code of 1.
///
/// # Arguments
/// * `image_path` - The file path to the image from which the palette will be extracted.
///
/// # Returns
/// A tuple containing:
/// - A vector of palette colors.
/// - A hash map mapping pixel values to palette indices.
pub fn extract_palette_or_exit(image_path: &str) -> (Vec<u8>, HashMap<u32, u8>) {
    match extract_palette(image_path) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to extract palette: {}", e);
            exit(1);
        }
    }
}