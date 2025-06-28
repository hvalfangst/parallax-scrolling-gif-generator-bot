use gif::{Encoder, Frame, Repeat};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::process::exit;
use timing_macro::timed;
use crate::state::constants::graphics::{WINDOW_HEIGHT, WINDOW_WIDTH};

/// Initializes a GIF encoder with the specified image file, width, and height.
///
/// GIFs are limited to a maximum of 256 colors in their palette. This function
/// sets up the encoder with an empty color map and ensures the GIF will loop
/// infinitely.
///
/// # Arguments
/// * `image` - A mutable reference to the file where the GIF will be written.
/// * `width` - The width of the GIF in pixels.
/// * `height` - The height of the GIF in pixels.
///
/// # Returns
/// An `Encoder` instance configured for the GIF file.
pub fn initialize_gif_encoder(image: &mut File, width: u16, height: u16) -> Encoder<&mut File> {
    let color_map = &[];
    let mut encoder = Encoder::new(image, width, height, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    encoder
}

/// Processes a single frame for the GIF encoder.
///
/// When the number of colors in the image exceeds 256, we use Euclidean color
/// distance to map each pixel to the nearest color in the palette. This is
/// necessary because GIFs have a hard limit of 256 colors in their palette.
///
/// # Arguments
/// * `scaled_buffer` - A mutable reference to the pixel buffer of the image.
/// * `encoder` - The GIF encoder instance.
/// * `width` - The width of the frame in pixels.
/// * `height` - The height of the frame in pixels.
/// * `frame_count` - A mutable reference to the current frame count.
/// * `color_map` - The palette of colors used in the GIF.
/// * `map` - A mutable hash map for mapping pixel values to palette indices.
#[timed]
pub fn process_frame(
    window_buffer: &mut Vec<u32>,
    encoder: &mut Encoder<&mut File>,
    frame_count: &mut usize,
    color_map: &Option<Vec<u8>>,
    map: &mut Option<HashMap<u32, u8>>,
) {
    *frame_count += 1;

    let palette: Vec<(u8, u8, u8)> = if let Some(color_map) = color_map {
        color_map
            .chunks(3)
            .map(|chunk| (chunk[0], chunk[1], chunk[2]))
            .collect()
    } else {
        vec![]
    };


    let buffer = if let Some(ref mut map) = map {
        map_pixels_to_indices(window_buffer, map, &palette)
    } else {
        vec![]
    };

    write_frame_to_gif(encoder, WINDOW_WIDTH as u16, WINDOW_HEIGHT as u16, color_map.as_deref().unwrap_or(&[]), &buffer, *frame_count);
}

/// Maps pixel values to their nearest palette indices using Euclidean color distance.
///
/// GIFs are limited to 256 colors, so when an image exceeds this limit, we need
/// to approximate each pixel's color by finding the closest match in the palette.
/// Euclidean color distance is used to measure the similarity between colors.
///
/// # Arguments
/// * `buffer` - A slice of pixel values.
/// * `color_to_index_map` - A mutable hash map for caching pixel-to-index mappings.
/// * `palette` - A slice of RGB tuples representing the palette.
///
/// # Returns
/// A vector of indices corresponding to the palette colors.
#[timed]
fn map_pixels_to_indices(buffer: &[u32], color_to_index_map: &mut HashMap<u32, u8>, palette: &[(u8, u8, u8)]) -> Vec<u8> {
    let mut logged_pixels = HashSet::new();
    let next_index = color_to_index_map.len() as u8;

    let mut color_to_index = |pixel: u32| {
        logged_pixels.insert(pixel);

        let index = *color_to_index_map.entry(pixel).or_insert_with(|| {
            if next_index == u8::MAX {
                eprintln!("Error: No color index available for pixel {}. Exiting to prevent overflow.", pixel);
                exit(1); // GitHub Actions will detect this as a failure
            }

            let pixel_rgb = (
                ((pixel >> 16) & 0xFF) as u8,
                ((pixel >> 8) & 0xFF) as u8,
                (pixel & 0xFF) as u8,
            );

            let closest_color_index = palette
                .iter()
                .enumerate()
                .min_by(|(_, &color_a), (_, &color_b)| {
                    let dist_a = color_distance(pixel_rgb, color_a);
                    let dist_b = color_distance(pixel_rgb, color_b);
                    dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(index, _)| index as u8)
                .unwrap_or(0); // Default to the first color in the palette if no unique closest color is found

            // println!("Mapping pixel {} to color index {}", pixel, closest_color_index);

            closest_color_index
        });

        index
    };

    buffer.iter().map(|&pixel| color_to_index(pixel)).collect()
}

/// Calculates the Euclidean distance between two colors.
///
/// This function is used to determine the similarity between two RGB colors.
/// The smaller the distance, the more similar the colors are.
///
/// # Arguments
/// * `color1` - The first color as an RGB tuple.
/// * `color2` - The second color as an RGB tuple.
///
/// # Returns
/// The Euclidean distance between the two colors.
fn color_distance(color1: (u8, u8, u8), color2: (u8, u8, u8)) -> f64 {
    let (r1, g1, b1) = color1;
    let (r2, g2, b2) = color2;

    let dr = ((r1 as i32 - r2 as i32).pow(2)) as i32;
    let dg = ((g1 as i32 - g2 as i32).pow(2)) as i32;
    let db = ((b1 as i32 - b2 as i32).pow(2)) as i32;

    ((dr + dg + db) as f64).sqrt()
}

/// Writes a single frame to the GIF file.
///
/// This function takes the pixel buffer and palette, and writes the frame to
/// the GIF encoder. The frame is configured with a delay to control playback speed.
///
/// # Arguments
/// * `encoder` - The GIF encoder instance.
/// * `width` - The width of the frame in pixels.
/// * `height` - The height of the frame in pixels.
/// * `color_map` - The palette of colors used in the GIF.
/// * `buffer` - The pixel buffer containing palette indices.
/// * `frame_count` - The current frame count.
#[timed]
fn write_frame_to_gif(
    encoder: &mut Encoder<&mut File>,
    width: u16,
    height: u16,
    color_map: &[u8],
    buffer: &[u8],
    frame_count: usize,
) {
    let mut frame = Frame::default();
    frame.width = width;
    frame.height = height;
    frame.palette = Some(color_map.to_vec());
    frame.buffer = Cow::Borrowed(buffer);
    frame.delay = 10;

    encoder.write_frame(&frame).expect("Failed to write frame to GIF");
    println!("Frame {} written to GIF file.", frame_count);
}