use anyhow::{anyhow, Result};
use base64::engine::general_purpose;
use base64::Engine;
use pyo3::prelude::*;

/// A struct representing an image generator that interacts with the OpenAI API.
pub struct ImageGenerator {
    /// The API key used for authenticating requests to the OpenAI API.
    api_key: String,
}

impl ImageGenerator {
    /// Creates a new instance of `ImageGenerator`.
    ///
    /// # Arguments
    /// * `api_key` - A `String` containing the OpenAI API key.
    ///
    /// # Returns
    /// A new `ImageGenerator` instance.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    /// Generates an image based on the provided prompt using the OpenAI API.
    ///
    /// # Arguments
    /// * `prompt` - A `&str` containing the text prompt for image generation.
    ///
    /// # Returns
    /// A `Result` containing the image data as a `Vec<u8>` if successful, or an error otherwise.
    pub fn generate_image(&self, prompt: &str) -> Result<Vec<u8>> {
        Python::with_gil(|py| {
            let openai = PyModule::import(py, "openai")?;

            // Create keyword arguments dictionary
            let kwargs = pyo3::types::PyDict::new(py);
            kwargs.set_item("api_key", self.api_key.clone())?;

            // Call OpenAI() with keyword arguments
            let client = openai.getattr("OpenAI")?.call((), Some(&kwargs))?;

            // Create the image generation request
            let img_kwargs = pyo3::types::PyDict::new(py);
            img_kwargs.set_item("model", "dall-e-3")?;
            img_kwargs.set_item("prompt", prompt)?;
            img_kwargs.set_item("size", "1024x1024")?;
            img_kwargs.set_item("quality", "standard")?; // or "hd" for higher quality
            img_kwargs.set_item("n", 1)?; // number of images
            img_kwargs.set_item("response_format", "b64_json")?;

            let response = client.getattr("images")?
                .call_method("generate", (), Some(&img_kwargs))?;

            // Extract base64 image data
            let data = response.getattr("data")?;
            let first_image = data.get_item(0)?;
            let b64_json = first_image.getattr("b64_json")?;
            let image_data: String = b64_json.extract()?;

            println!("Generated image, base64 length: {}", image_data.len());

            // Decode base64 to bytes
            general_purpose::STANDARD
                .decode(&image_data)
                .map_err(|e| anyhow!("Failed to decode base64 image: {}", e))
        })
    }
}