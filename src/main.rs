use crate::state::constants::file_paths::INPUT_IMAGE_PATH;
use crate::state::constants::graphics::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::state::event_loop::record_gif;
use crate::state::structs::State;
use crate::utils::misc::{create_parallax_layers_for_date, extract_palette_or_exit, generate_and_save_image, initialize_generators, parse_headless_mode, prepare_python_interpreter};
use chrono::NaiveDate;
use minifb::{Window, WindowOptions};
use std::fs;
use std::io::stdin;
use std::process::exit;


mod graphics; mod state; mod utils; mod generators;

fn main() {
    prepare_python_interpreter();
    let headless = parse_headless_mode();

    if headless {
        println!("\nRunning in headless mode, tailored for the GitHub runner.");
        let (prompt_generator, image_generator) = initialize_generators();
        let current_date = chrono::Utc::now().date_naive();

        let prompt_result = generate_and_save_image(&prompt_generator, &image_generator, current_date);
        if let Err(e) = prompt_result {
            eprintln!("Error during image generation: {}", e);
            return;
        }

        let (color_map, color_to_index_map) = extract_palette_or_exit(INPUT_IMAGE_PATH);

        if let Err(e) = create_parallax_layers_for_date(INPUT_IMAGE_PATH, current_date) {
            eprintln!("Error during parallax layer creation: {}", e);
            return;
        }


        let binding = prompt_result.unwrap();
        let mut window_buffer = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

        let state = State::new(
            current_date,
            &mut window_buffer,
            None,
            binding.as_str(),
            headless,
            Some(color_map),
            Some(color_to_index_map),
        );

        record_gif(state);
    }

    else {

        println!("Running in windowed, local mode. You will be prompted with a selection of available files");

        // List all images in the directory
        let images = fs::read_dir("./images")
            .expect("Failed to read images directory")
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_name = entry.file_name().into_string().ok()?;
                if file_name.starts_with("image_") && file_name.ends_with(".png") {
                    Some(file_name)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if images.is_empty() {
            eprintln!("No images found in the directory.");
            return;
        }

        println!("Available images:");
        for (index, image) in images.iter().enumerate() {
            println!("{}: {}", index + 1, image);
        }

        // Let the user select an image
        println!("Please select an image by entering its number:");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        let selected_index: usize = input.trim().parse::<usize>().expect("Invalid input") - 1;

        if selected_index >= images.len()-1 {
            eprintln!("Invalid selection.");
            return;
        }

        let selected_image = &images[selected_index];
        let date_part = &selected_image[6..16]; // Extract YYYY-MM-DD
        let naive_date_part = NaiveDate::parse_from_str(date_part, "%Y-%m-%d")
            .expect("Invalid date format in image name");
        println!("You selected the image with date: {}", date_part);

        let image_path = format!("./images/{}", selected_image);
        let (color_map, color_to_index_map) = extract_palette_or_exit(image_path.as_str());

        let mut window_buffer = vec![0; 1024 * 1024];

        let mut window =
            Some(Window::new(
                "Parallax Scrolling GIF Exporter",
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                WindowOptions::default(),
            ).unwrap_or_else(|e| {
                panic!("{}", e);
            }));


        let state = State::new(
            naive_date_part,
            &mut window_buffer,
            window.as_mut(),
            "NIX",
            false,
            Some(color_map),
            Some(color_to_index_map),
        );

        record_gif(state);
    }


}