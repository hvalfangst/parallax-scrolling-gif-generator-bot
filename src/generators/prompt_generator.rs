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
        let text_prompt = generate_text_prompt();

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

/// Returns the system prompt for RPG parallax generation
fn get_system_prompt() -> String {
    String::from(
        "You are a specialized 2D background artist for side-scrolling adventure games. \
        Focus on creating atmospheric, pixel-perfect backgrounds with 4 distinct parallax layers (256px each). \
        Design each layer for seamless horizontal tiling with clear depth separation. \
        Emphasize visual hierarchy through limited color palettes (4-8 colors per scene). \
        Create backgrounds that support the game's movement flow and maintain readability during scrolling. \
        Prioritize designs that complement player navigation while avoiding visual noise that could distract from gameplay. \
        All elements must tile perfectly for infinite horizontal scrolling. Keep descriptions under 100 tokens."
    )
}
/// Generates a themed RPG parallax background prompt
fn generate_text_prompt() -> String {
    "Design a 1024x1024 parallax background for a 2d side-scrolling game, which consists of 4 layers (256px segments each):\n\n\
        Layer 1 (256px): [describe far background elements - MUST tile seamlessly]\n\
        Layer 2 (512px): [describe mid-distant elements - MUST tile seamlessly]\n\
        Layer 3 (768px): [describe near background elements - MUST tile seamlessly]\n\
        Layer 4 (1024px): [describe foreground elements - MUST tile seamlessly]\n\n\
        IMPORTANT: Each layer must be easy to separate for parallax. \
        The pattern MUST repeat seamlessly for horizontal scrolling in a GIF.\n\n\
        Format: \"Background for 2d side-scrolling game, which have 4 separate horizontal layers for parallax scrolling. Layer 1: [simple tiling elements]. Layer 2: [simple tiling elements]. Layer 3: [simple tiling elements]. Layer 4: [simple tiling elements].\" Keep under 100 tokens.".to_string()
}
