use crate::state::structs::State;
use gif::{Encoder, Frame, Repeat};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fs::File;

pub fn initialize_gif_encoder(image: &mut File, width: u16, height: u16) -> Encoder<&mut File> {
    let color_map = &[];
    let mut encoder = Encoder::new(image, width, height, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    encoder
}

pub fn process_frame(
    game_state: &mut State,
    encoder: &mut Encoder<&mut File>,
    width: u16,
    height: u16,
    frame_count: &mut usize
) {
    let (color_map, mut color_to_index_map) = generate_color_map(&game_state.scaled_buffer);
    *frame_count += 1;


    let buffer = map_pixels_to_indices(&game_state.scaled_buffer, &mut color_to_index_map);
    if buffer.is_empty() {
        println!("Warning: Buffer is empty, skipping frame {}", *frame_count);
        return;
    }
    write_frame_to_gif(encoder, width, height, &color_map, &buffer, *frame_count);
}

fn generate_color_map(buffer: &[u32]) -> (Vec<u8>, HashMap<u32, u8>) {
    let unique_colors: HashSet<u32> = buffer.iter().cloned().collect();
    let mut color_map = Vec::new();
    let mut color_to_index_map = HashMap::new();

    // Limit the number of colors to 256
    let limited_colors: Vec<u32> = unique_colors.into_iter().take(256).collect();

    for (index, &color) in limited_colors.iter().enumerate() {
        let red = ((color >> 16) & 0xFF) as u8;
        let green = ((color >> 8) & 0xFF) as u8;
        let blue = (color & 0xFF) as u8;

        color_map.push(red);
        color_map.push(green);
        color_map.push(blue);

        color_to_index_map.insert(color, index as u8);
    }

    (color_map, color_to_index_map)
}

fn map_pixels_to_indices(buffer: &[u32], color_to_index_map: &mut HashMap<u32, u8>) -> Vec<u8> {
    let mut logged_pixels = HashSet::new();
    let mut next_index = color_to_index_map.len() as u8;
    let mut color_to_index = |pixel: u32| {
        logged_pixels.insert(pixel);
        *color_to_index_map.entry(pixel).or_insert_with(|| {
            if next_index == u8::MAX {
                return next_index; // Return the maximum index without incrementing
            }
            let index = next_index;
            next_index += 1;
            index
        })
    };

    buffer.iter().map(|&pixel| color_to_index(pixel)).collect()
}

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