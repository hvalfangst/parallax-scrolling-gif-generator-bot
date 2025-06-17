use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use image::{DynamicImage, GenericImageView, Rgb};

/// Represents a color in RGB format.
/// Each color component (red, green, blue) is stored as an 8-bit unsigned integer.
/// This struct is used to encapsulate color data and provide utility methods for color manipulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8, // Red component
    pub g: u8, // Green component
    pub b: u8, // Blue component
}

impl Color {
    /// Creates a new `Color` instance from the given RGB values.
    ///
    /// # Arguments
    /// * `r` - The red component of the color (0-255).
    /// * `g` - The green component of the color (0-255).
    /// * `b` - The blue component of the color (0-255).
    ///
    /// # Returns
    /// A new `Color` instance with the specified RGB values.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Converts an `Rgb<u8>` from the `image` crate into a `Color`.
    ///
    /// # Arguments
    /// * `rgb` - A reference to an `Rgb<u8>` object representing the color.
    ///
    /// # Returns
    /// A new `Color` instance created from the `Rgb<u8>` object.
    pub fn from_rgb(rgb: &Rgb<u8>) -> Self {
        Self::new(rgb[0], rgb[1], rgb[2])
    }

    /// Converts the `Color` into a hexadecimal string representation.
    ///
    /// # Example
    /// RGB(255, 0, 0) -> "#ff0000"
    ///
    /// # Returns
    /// A `String` containing the hexadecimal representation of the color.
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Calculates the Euclidean distance between two colors in RGB space.
    ///
    /// # Arguments
    /// * `other` - A reference to another `Color` instance to calculate the distance to.
    ///
    /// # Returns
    /// A `f64` representing the Euclidean distance between the two colors.
    ///
    /// # Example
    /// ```
    /// let color1 = Color::new(255, 0, 0);
    /// let color2 = Color::new(0, 255, 0);
    /// let distance = color1.distance_to(&color2);
    /// println!("Distance: {}", distance);
    /// ```
    pub fn distance_to(&self, other: &Color) -> f64 {
        let dr = self.r as f64 - other.r as f64;
        let dg = self.g as f64 - other.g as f64;
        let db = self.b as f64 - other.b as f64;
        (dr * dr + dg * dg + db * db).sqrt()
    }
}

impl std::fmt::Display for Color {
    /// Formats the `Color` as a human-readable string in the form "RGB(r, g, b)".
    ///
    /// # Arguments
    /// * `f` - A mutable reference to the formatter.
    ///
    /// # Returns
    /// A `Result` indicating whether the formatting was successful.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RGB({}, {}, {})", self.r, self.g, self.b)
    }
}

/// Extracts a color palette from an image using clustering algorithms.
/// The palette extractor supports K-means clustering and median cut methods.
/// This struct encapsulates configuration options for palette extraction.
pub struct PaletteExtractor {
    num_colors: usize,      // Number of colors to extract
    resize_width: u32,      // Width to resize the image for processing
    max_iterations: usize,  // Maximum iterations for K-means clustering
}

impl Default for PaletteExtractor {
    /// Provides default values for the `PaletteExtractor`.
    ///
    /// # Returns
    /// A `PaletteExtractor` instance with default settings:
    /// * 5 colors
    /// * Resize width of 150 pixels
    /// * 100 iterations for K-means clustering
    fn default() -> Self {
        Self {
            num_colors: 5,
            resize_width: 150,
            max_iterations: 100,
        }
    }
}

impl PaletteExtractor {
    /// Creates a new `PaletteExtractor` with the specified number of colors.
    ///
    /// # Arguments
    /// * `num_colors` - The number of colors to extract from the image.
    ///
    /// # Returns
    /// A new `PaletteExtractor` instance with the specified number of colors.
    pub fn new(num_colors: usize) -> Self {
        Self {
            num_colors,
            ..Default::default()
        }
    }

    /// Sets the width to resize the image for processing.
    /// Resizing helps reduce computational complexity.
    ///
    /// # Arguments
    /// * `width` - The width to resize the image to.
    ///
    /// # Returns
    /// The updated `PaletteExtractor` instance.
    pub fn with_resize_width(mut self, width: u32) -> Self {
        self.resize_width = width;
        self
    }

    /// Sets the maximum number of iterations for K-means clustering.
    ///
    /// # Arguments
    /// * `iterations` - The maximum number of iterations.
    ///
    /// # Returns
    /// The updated `PaletteExtractor` instance.
    pub fn with_max_iterations(mut self, iterations: usize) -> Self {
        self.max_iterations = iterations;
        self
    }

    /// Extracts a color palette from an image using K-means clustering.
    ///
    /// # Arguments
    /// * `image_path` - The path to the image file.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Color` instances representing the extracted palette,
    /// or an error if the extraction fails.
    pub fn extract_palette<P: AsRef<Path>>(&self, image_path: P) -> Result<Vec<Color>, Box<dyn Error>> {
        let img = image::open(image_path)?; // Load the image
        let resized_img = self.preprocess_image(img); // Resize the image for processing
        let pixels = self.extract_pixels(&resized_img); // Extract RGB pixels from the image

        let palette = self.kmeans_clustering(pixels)?; // Perform K-means clustering
        Ok(palette)
    }

    /// Resizes the image while maintaining its aspect ratio.
    /// This reduces the number of pixels for faster processing.
    ///
    /// # Arguments
    /// * `img` - The image to resize.
    ///
    /// # Returns
    /// A resized `DynamicImage` instance.
    fn preprocess_image(&self, img: DynamicImage) -> DynamicImage {
        let (width, height) = img.dimensions();
        let aspect_ratio = height as f32 / width as f32;
        let new_height = (self.resize_width as f32 * aspect_ratio) as u32;

        img.resize(self.resize_width, new_height, image::imageops::FilterType::Lanczos3)
    }

    /// Extracts RGB pixels from the image and converts them into `Color` instances.
    ///
    /// # Arguments
    /// * `img` - A reference to the image to extract pixels from.
    ///
    /// # Returns
    /// A vector of `Color` instances representing the pixels in the image.
    fn extract_pixels(&self, img: &DynamicImage) -> Vec<Color> {
        let rgb_img = img.to_rgb8(); // Convert image to RGB format
        rgb_img.pixels()
            .map(|pixel| Color::from_rgb(pixel)) // Map each pixel to a `Color`
            .collect()
    }

    /// Performs K-means clustering on the given pixels to extract a color palette.
    /// This groups pixels into clusters based on color similarity.
    ///
    /// # Arguments
    /// * `pixels` - A vector of `Color` instances representing the pixels to cluster.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Color` instances representing the extracted palette,
    /// or an error if the clustering fails.
    fn kmeans_clustering(&self, pixels: Vec<Color>) -> Result<Vec<Color>, Box<dyn Error>> {
        if pixels.is_empty() {
            return Ok(vec![]); // Return an empty palette if no pixels are provided
        }

        // Initialize centroids for clustering
        let mut centroids = self.initialize_centroids(&pixels);
        let mut assignments = vec![0; pixels.len()];

        for iteration in 0..self.max_iterations {
            let mut changed = false;

            // Assign each pixel to the nearest centroid
            for (i, pixel) in pixels.iter().enumerate() {
                let mut min_distance = f64::INFINITY;
                let mut best_centroid = 0;

                for (j, centroid) in centroids.iter().enumerate() {
                    let distance = pixel.distance_to(centroid);
                    if distance < min_distance {
                        min_distance = distance;
                        best_centroid = j;
                    }
                }

                if assignments[i] != best_centroid {
                    assignments[i] = best_centroid;
                    changed = true;
                }
            }

            // Update centroids based on the average of assigned pixels
            let mut sums_r = vec![0u32; self.num_colors];
            let mut sums_g = vec![0u32; self.num_colors];
            let mut sums_b = vec![0u32; self.num_colors];
            let mut counts = vec![0u32; self.num_colors];

            for (pixel, &assignment) in pixels.iter().zip(assignments.iter()) {
                sums_r[assignment] += pixel.r as u32;
                sums_g[assignment] += pixel.g as u32;
                sums_b[assignment] += pixel.b as u32;
                counts[assignment] += 1;
            }

            let mut new_centroids = Vec::new();
            for i in 0..self.num_colors {
                if counts[i] > 0 {
                    let avg_r = (sums_r[i] / counts[i]) as u8;
                    let avg_g = (sums_g[i] / counts[i]) as u8;
                    let avg_b = (sums_b[i] / counts[i]) as u8;
                    new_centroids.push(Color::new(avg_r, avg_g, avg_b));
                } else {
                    // Retain the old centroid if no pixels are assigned
                    new_centroids.push(centroids[i]);
                }
            }

            centroids = new_centroids;

            // Stop early if centroids have stabilized
            if !changed && iteration > 5 {
                break;
            }
        }

        // Sort centroids by frequency of assigned pixels
        let mut color_counts = vec![0usize; self.num_colors];
        for &assignment in &assignments {
            color_counts[assignment] += 1;
        }

        let mut indexed_centroids: Vec<(usize, Color, usize)> = centroids
            .into_iter()
            .enumerate()
            .map(|(i, color)| (i, color, color_counts[i]))
            .collect();

        // Sort by count (most frequent colors first)
        indexed_centroids.sort_by(|a, b| b.2.cmp(&a.2));

        // Remove empty clusters and return the final palette
        let result: Vec<Color> = indexed_centroids
            .into_iter()
            .filter(|(_, _, count)| *count > 0)
            .map(|(_, color, _)| color)
            .collect();

        Ok(result)
    }

    /// Initializes centroids for K-means clustering.
    /// This ensures a better distribution of initial centroids across the pixel range.
    ///
    /// # Arguments
    /// * `pixels` - A slice of `Color` instances representing the pixels to initialize centroids from.
    ///
    /// # Returns
    /// A vector of `Color` instances representing the initialized centroids.
    fn initialize_centroids(&self, pixels: &[Color]) -> Vec<Color> {
        use std::collections::HashSet;

        if pixels.len() <= self.num_colors {
            return pixels.to_vec(); // Use all pixels if fewer than required centroids
        }

        let mut centroids = Vec::new();
        let mut used_indices = HashSet::new();

        // Distribute centroids evenly across the pixel range
        let step = pixels.len() / self.num_colors;

        for i in 0..self.num_colors {
            let idx = (i * step).min(pixels.len() - 1);
            if !used_indices.contains(&idx) {
                centroids.push(pixels[idx]);
                used_indices.insert(idx);
            } else {
                // Find the next available pixel if the current one is already used
                let mut fallback_idx = idx;
                while used_indices.contains(&fallback_idx) && fallback_idx < pixels.len() - 1 {
                    fallback_idx += 1;
                }
                centroids.push(pixels[fallback_idx]);
                used_indices.insert(fallback_idx);
            }
        }

        // Add additional centroids if needed
        while centroids.len() < self.num_colors {
            for (i, pixel) in pixels.iter().enumerate() {
                if !used_indices.contains(&i) {
                    centroids.push(*pixel);
                    used_indices.insert(i);
                    break;
                }
            }
            if centroids.len() >= self.num_colors {
                break;
            }
        }

        centroids
    }
}

/// Extracts a color palette from an image file using K-means clustering.
///
/// This function utilizes the `PaletteExtractor` to process the image and extract
/// a palette of colors. It also generates a color map and a mapping of packed RGB values
/// to their respective indices.
///
/// # Arguments
/// * `input_image_path` - A string slice representing the path to the input image file.
///
/// # Returns
/// A `Result` containing:
/// - `Vec<u8>`: A flat vector of RGB values representing the extracted colors.
/// - `HashMap<u32, u8>`: A mapping of packed RGB values (as `u32`) to their indices in the palette.
///
/// # Errors
/// Returns an error if the image cannot be loaded or the palette extraction fails.
///
/// # Example
/// ```
/// let (color_map, color_to_index_map) = extract_palette("path/to/image.png")?;
/// ```
pub fn extract_palette(input_image_path: &str) -> Result<(Vec<u8>, HashMap<u32, u8>), Box<dyn Error>> {
    let extractor = PaletteExtractor::new(256)
        .with_resize_width(150)
        .with_max_iterations(50);

    let palette = extractor.extract_palette(input_image_path)?;
    println!("Extracted {} colors using K-means:", palette.len());
    for (i, color) in palette.iter().enumerate() {
        println!("Color {}: {} ({})", i + 1, color, color.to_hex());
    }

    let color_map: Vec<u8> = palette.iter().flat_map(|color| vec![color.r, color.g, color.b]).collect();
    let color_to_index_map: HashMap<u32, u8> = palette.iter().enumerate().map(|(i, color)| {
        let packed_color = ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
        println!("Mapping color {} (RGB: {}, {}, {}) to index {}", packed_color, color.r, color.g, color.b, i);
        (packed_color, i as u8)
    }).collect();


    Ok((color_map, color_to_index_map))
}