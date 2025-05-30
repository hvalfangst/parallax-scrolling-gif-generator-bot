use std::env;
use chrono::NaiveDate;
use image::{GenericImageView, ImageBuffer, Rgba};
use minifb::{Window, WindowOptions};
use pyo3::prepare_freethreaded_python;
use crate::graphics::sprites::SpriteMaps;
use crate::state::constants::graphics::{SCALED_WINDOW_HEIGHT, SCALED_WINDOW_WIDTH};
use crate::state::event_loop::start_event_loop;
use crate::state::structs::State;

mod generators;
mod utils;
mod state;
mod graphics;

fn main() {
    prepare_freethreaded_python();

    let prompt_generator = generators::prompt_generator::PromptGenerator::new(env::var("OPENAI_API_KEY").unwrap());
    let image_generator = generators::image_generator::ImageGenerator::new(env::var("OPENAI_API_KEY").unwrap());


    let current_date = chrono::Utc::now().date_naive();
    println!("Current date: {}", current_date);

    let prompt = prompt_generator.generate_prompt().unwrap();
    println!("Generated prompt: {}", prompt);

    match image_generator.generate_image(prompt.as_str()) {

        Ok(image_data) => {
            println!("Image generated successfully, size: {} bytes", image_data.len());

            match utils::file_manager::FileManager::save_prompt(prompt.as_str(), current_date) {
                Ok(_) => println!("Prompt saved successfully."),
                Err(e) => eprintln!("Error saving prompt: {}", e),
            }

            match utils::file_manager::FileManager::save_image(&image_data, current_date) {
                Ok(_) => println!("Image saved successfully."),
                Err(e) => eprintln!("Error saving image: {}", e),
            }

        },
        Err(e) => {
            eprintln!("Error generating image: {}", e);
        }
    }

    println!("Creating parallax layers for date: {}", current_date);


    create_parallax_layers("images/image_current.png", current_date)
        .unwrap_or_else(|e| {
            eprintln!("Error creating parallax layers: {}", e);
        });

    println!("Parallax layers for date {} created successfully.", current_date);

    println!("Parsing command line arguments...");
    // Check command line arguments for headless mode
    let args: Vec<String> = env::args().collect();
    let headless_mode = args.get(1).map(|s| s == "--headless").unwrap_or(false) ||
        args.get(2).map(|s| s == "--headless").unwrap_or(false);

    println!("Headless mode: {}", headless_mode);

    println!("Loading sprite maps for date: {}", current_date);
    let sprites = SpriteMaps::new(current_date);

    // Initialize window buffer and scaled buffer
    let mut window_buffer = vec![0; 1024 * 1024];
    let mut scaled_buffer = vec![0; SCALED_WINDOW_WIDTH * SCALED_WINDOW_HEIGHT];
    let camera = state::structs::Camera::new(0.0, 0.0);

    // Create window only if not in headless mode
    let mut window = if !headless_mode {
        Some(Window::new(
            "Parallax Exporter",
            SCALED_WINDOW_WIDTH,
            SCALED_WINDOW_HEIGHT,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        }))
    } else {
        None
    };

    println!("Initializing state with current date: {}", current_date);

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
        prompt: prompt.as_str(),
    };

    println!("Starting GIF recording event loop...");

    start_event_loop(state, headless_mode);
}

pub fn create_parallax_layers(input_path: &str, current_date: NaiveDate) -> Result<(), Box<dyn std::error::Error>> {
    // Load the input image
    let img = image::open(input_path)?;
    let (width, height) = img.dimensions();

    if width != 1024 || height != 1024 {
        return Err("Input image must be 1024x1024.".into());
    }

    // Define the layer heights
    let layer_heights = [256, 512, 768, 1024];

    for (i, &layer_height) in layer_heights.iter().enumerate() {
        // Create a new image buffer for the layer
        let mut layer = ImageBuffer::from_fn(width, height, |x, y| {
            if y >= (i * 256) as u32 && y < ((i + 1) * 256) as u32 {
                img.get_pixel(x, y)
            } else {
                Rgba([0, 0, 0, 0]) // Transparent pixel
            }
        });

        // Save the layer to a file
        let output_dir = format!("layers/{}", i + 1);
        let output_path = format!("{}/layer_{}.png", output_dir, current_date);
        println!("Layer {} saved to {}", i + 1, output_path);
        layer.save(output_path)?;

    }

    Ok(())
}