use crate::utils::text_processor::TextProcessor;
use anyhow::Result;
use pyo3::prelude::*;

pub struct PromptGenerator {
    api_key: String,
}

impl PromptGenerator {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

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

fn get_system_prompt() -> String {
    String::from(
        "You are a background artist. Create concise, clear descriptions for parallax backgrounds with 4 distinct, SEPARATE horizontal layers (256px segments each). \
        Each layer must be clearly distinguishable for use in parallax scrolling. Use a LIMITED COLOR PALETTE. \
        All patterns MUST tile seamlessly horizontally for infinite scrolling in GIFs. Prioritize seamless looping and clarity over fine detail. \
        Keep responses under 100 tokens."
    )
}

fn get_text_prompt() -> String {
    format!(
        "Design a 1024x1024 parallax background in a limited palette style, split into 4 clearly SEPARATE horizontal layers (256px segments each):\n\n\
        Layer 1 (256px): [describe far background elements - MUST tile seamlessly]\n\
        Layer 2 (512px): [describe mid-distant elements - MUST tile seamlessly]\n\
        Layer 3 (768px): [describe near background elements - MUST tile seamlessly]\n\
        Layer 4 (1024px): [describe foreground elements - MUST tile seamlessly]\n\n\
        IMPORTANT: Use a limited palette across ALL layers. Each layer must be easy to separate for parallax. \
        The pattern MUST repeat seamlessly for horizontal scrolling in a GIF.\n\n\
        Format: \"A parallax background with limited palette. Layer 1: [simple tiling elements]. Layer 2: [simple tiling elements]. Layer 3: [simple tiling elements]. Layer 4: [simple tiling elements].\" Keep under 100 tokens.",
    )
}