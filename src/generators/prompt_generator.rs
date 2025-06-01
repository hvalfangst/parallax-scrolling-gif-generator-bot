use crate::utils::text_processor::TextProcessor;
use anyhow::Result;
use pyo3::prelude::*;

/// A struct representing a prompt generator that interacts with the OpenAI API.
pub struct PromptGenerator {
    /// The API key used for authenticating requests to the OpenAI API.
    api_key: String,
}

impl PromptGenerator {
    /// Creates a new instance of `PromptGenerator`.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    /// Generates a prompt using the OpenAI API.
    ///
    /// # Returns
    /// A `Result` containing the generated prompt as a `String` if successful, or an error otherwise.
    pub fn generate_prompt(&self) -> Result<String> {
        let system_prompt = get_system_prompt();
        let text_prompt = get_text_prompt();

        println!("Generating prompt with system: '{}', text: '{}'", system_prompt, text_prompt);

        Python::with_gil(|py| {
            let openai = PyModule::import(py, "openai")?;

            // Create keyword arguments dictionary
            let kwargs = pyo3::types::PyDict::new(py);
            kwargs.set_item("api_key", self.api_key.clone())?;

            // Call OpenAI() with keyword arguments
            let client = openai.getattr("OpenAI")?.call((), Some(&kwargs))?;

            // Create the chat completion request
            let kwargs = pyo3::types::PyDict::new(py);
            kwargs.set_item("model", "gpt-4.1-nano")?;

            kwargs.set_item("messages", {
                let system_dict = pyo3::types::PyDict::new(py);
                system_dict.set_item("role", "system")?;
                system_dict.set_item("content", &*system_prompt)?;

                let user_dict = pyo3::types::PyDict::new(py);
                user_dict.set_item("role", "user")?;
                user_dict.set_item("content", &*text_prompt)?;

                vec![system_dict, user_dict]
            })?;

            kwargs.set_item("max_tokens", 100)?;
            kwargs.set_item("temperature", 0.3)?;
            kwargs.set_item("presence_penalty", 0.2)?;
            kwargs.set_item("frequency_penalty", 0.1)?;

            // Generate completion using new API structure
            let response = client.getattr("chat")?
                .getattr("completions")?
                .call_method("create", (), Some(&kwargs))?;

            // Extract response content (same as before)
            let choices = response.getattr("choices")?;
            let first_choice = choices.get_item(0)?;
            let message = first_choice.getattr("message")?;
            let raw_prompt: String = message.getattr("content")?.extract()?;
            let raw_prompt = raw_prompt.trim().to_string();

            println!("Raw prompt: {}", raw_prompt);

            let ascii_enforced_prompt = TextProcessor::enforce_ascii(&raw_prompt);
            println!("ASCII-enforced prompt: {}", ascii_enforced_prompt);

            let final_prompt = TextProcessor::remove_incomplete_last_sentence(&ascii_enforced_prompt);
            println!("Final prompt after sentence cleanup: {}", final_prompt);

            Ok(final_prompt)
        })
    }
}

/// Returns the system prompt used for generating OpenAI completions.
///
/// # Returns
/// A `String` containing the system prompt.
fn get_system_prompt() -> String {
    String::from(
        "You are a technical background artist specializing in parallax layer generation. \
        You MUST create backgrounds as 4 DISTINCT horizontal strips stacked vertically. \
        Each strip is EXACTLY 256px tall and represents ONE parallax layer that will be extracted separately. \
        Use CONTRASTING colors/tones between layers for easy separation. \
        ALL elements must tile perfectly horizontally. NO vertical blending between layers. \
        Each layer must be visually independent."
    )
}

/// Returns the text prompt used for generating OpenAI completions.
///
/// # Returns
/// A `String` containing the text prompt.
fn get_text_prompt() -> String {
    format!(
        "Create a 1024x1024 image structured as 4 horizontal strips stacked vertically. \
        Each strip is EXACTLY 256px tall and contains ONE parallax layer:\n\n\
        TOP STRIP (0-256px): Far background layer - [simple, light-toned repeating elements]\n\
        SECOND STRIP (256-512px): Mid-background layer - [medium-toned repeating elements]\n\
        THIRD STRIP (512-768px): Near-background layer - [darker-toned repeating elements]\n\
        BOTTOM STRIP (768-1024px): Foreground layer - [darkest/most contrasted repeating elements]\n\n\
        CRITICAL REQUIREMENTS:\n\
        - Each 256px strip must be visually DISTINCT from others\n\
        - Use different color tones for each strip for easy extraction\n\
        - NO gradients or blending between strips\n\
        - Elements within each strip must tile seamlessly horizontally\n\
        - Sharp horizontal divisions between each 256px section\n\n\
        Describe as: 'Four distinct horizontal parallax strips, each 256px tall with seamlessly tiling elements.'"
    )
}