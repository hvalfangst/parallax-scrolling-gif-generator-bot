use crate::utils::text_processor::TextProcessor;
use anyhow::Result;
use pyo3::prelude::*;
use timing_macro::timed;

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
    #[timed]
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
        Focus on creating atmospheric, backgrounds with 4 distinct horizontal layers. \
        The generated background is used as input to an algorithm that divides it into 4 layers, which \
        requires 4 distinct 256 pixel horizontal layers. \
        Use only the following colors in your design: \
        0x001f41, 0x82c6d6, 0x2f5a78, 0xafa287, 0x2f6581, 0x2d627f, 0xfdf0c1, 0x457a94, 0x578a9b, 0x82cec8, \
        0x9cf1e6, 0xdefbce, 0xac9f84, 0x477c9e, 0x44789c, 0x2a5270, 0x5c9ea3, 0xe7fdcd, 0x6795ba, 0x2d5674, \
        0xf8e1b7, 0xfefec9, 0xaed0a6, 0x95ccc3, 0x97ece2, 0x8ab9c2, 0x5ea0ba, 0x76aeb2, 0x2b5f7c, 0xc4d0a0, \
        0x4b81a2, 0xa8bd8b, 0xf0bfac, 0x244964, 0x79ac8e, 0x3f758f, 0x34607a, 0xe2b29f, 0x204561, 0xa5f9ec, \
        0x4e8095, 0x949b75, 0x022141, 0x1a3748, 0xb0c492, 0x5c909f, 0x85ccd9, 0x64a8c0, 0x1f3b4c, 0x61a6bf, \
        0x2a5c79, 0x6fa78c, 0x8ee7de, 0xfbf6c3, 0x538598, 0xaabf8f, 0xfce6bb, 0x001d3e, 0x3b6f89, 0x6a98bc, \
        0x1c405c, 0x3c573b, 0x78936a, 0x285876, 0x265674, 0x69855e, 0xcef8dc, 0x526646, 0x042343, 0x5a7755, \
        0x244051, 0x6d9ec1, 0x45807a, 0xe7b6a0, 0xa2b271, 0xf1fed1, 0x96a96b, 0xc0ce98, 0xb8c67d, 0x26475d, \
        0x68adc2, 0xb6a88d, 0x4b7c92, 0x6a9fac, 0x637e5b, 0x6d8b6d, 0x839a6d, 0x396781, 0x748c5d, 0xd9f6ca, \
        0xd1ba9a, 0x7cb3b8, 0x9edace, 0xbbf1da, 0x245372, 0x538394, 0x23516f, 0x6fa5af, 0xa9e6d5, 0xf7d9b2, \
        0x092a4c, 0x173852, 0x80965f, 0x7b867e, 0x42769b, 0x5b9782, 0x12395d, 0x4f8a7d, 0x0c3155, 0x2b4c61, \
        0xadbd75, 0x082848, 0x768177, 0x1a4769, 0x7fc0d3, 0x183243, 0x3d7299, 0x90d2c9, 0x336a86, 0x98be9d, \
        0xe6fed8, 0xc5d181, 0xf0cfac, 0xd9e08e, 0x164062, 0x2f6b73, 0x0d2f46, 0x94b098, 0xebbba3, 0x83776e, \
        0xbdd9af, 0x314a54, 0xbaca95, 0xcdd9a7, 0x4c674f, 0xa3c7a3, 0x53585d, 0xced78a, 0x204e6e, 0x8ea994, \
        0x9aae81, 0x37554e, 0x3a6e95, 0x83b38f, 0x589aa0, 0xa4b98b, 0x606262, 0x8ca167, 0x597763, 0x61806a, \
        0x79716a, 0x30566a, 0x90a578, 0x1e3744, 0x71a9b6, 0x001a3d, 0x09283e, 0x1a3c56, 0x5388a6, 0x132e42, \
        0xabd3bf, 0x415f4d, 0x05223e, 0x464f58, 0x73adc4, 0x7dcec8, 0x273a44, 0xb5e3cd, 0x3c4852, 0x4e7a87, \
        0x495f45, 0x547050, 0x184365, 0x0c2942, 0x9dc6b8, 0xeec4a7, 0xdac3a2, 0x153d5f, 0x8abdb6, 0x123348, \
        0x354d48, 0xa99c82, 0x456e7d, 0x437287, 0x2c4b4a, 0x274f6a, 0x30404c, 0x113758, 0x0f3254, 0x54979e, \
        0x6599a4, 0x506d5e, 0x46655d, 0x3d7777, 0x14334e, 0x89b2a8, 0x5f724b, 0x405644, 0x8a8173, 0x6d6a67, \
        0x071f39, 0xc1ad91, 0xe5ea99, 0x1e4b6a, 0x254246, 0xdae4b6, 0xb7c6ac, 0x697f52, 0x3b5a5e, 0x7ab5c8, \
        0x87e3dc, 0x3e6778, 0x021e3a, 0x99e8dd, 0xe6efc0, 0x112639, 0x618b93, 0xc9e1c1, 0x152b3d, 0x40636d, \
        0xf6f8b0, 0x6092b6, 0x99907d, 0x061930, 0x5994ad, 0x5399a3, 0x938878, 0x7ba9a4, 0x5a8dad, 0x2f473f, \
        0x011a37, 0x6e9e82, 0x6fa4a3, 0x62a9ae, 0xa2977e, 0x011734, 0x092d42, 0x6d989d, 0x00102b, 0x6db6b8, \
        0x1f313c, 0x83a19e, 0x0d2135, 0x2b646e, 0x5e9bb2, 0x00183a, 0x366b8f, 0x8cdad5, 0x7bc5c5, 0x00143b, \
        0x5d837e, 0x789691, 0x001036, 0x265f6b, 0xdeae9c."
    )
}
/// Generates a themed RPG parallax background prompt
fn generate_text_prompt() -> String {
    String::from(
    "Design a 1024x1024 parallax background for a 2d side-scrolling game, which consists of 4 horizontal layers (256px segments each):\n\n\
        Layer 1 (256px): [describe far background elements]\n\
        Layer 2 (512px): [describe mid-distant elements]\n\
        Layer 3 (768px): [describe near background elements]\n\
        Layer 4 (1024px): [describe foreground elements]\n\n\
        The pattern MUST repeat seamlessly for horizontal scrolling in a GIF.\n\n\
        Format: \"Background for 2d side-scrolling game, which have 4 separate horizontal layers. Layer 1: []. Layer 2: []. Layer 3: []. Layer 4: [].\
        0x001f41, 0x82c6d6, 0x2f5a78, 0xafa287, 0x2f6581, 0x2d627f, 0xfdf0c1, 0x457a94, 0x578a9b, 0x82cec8, \
        0x9cf1e6, 0xdefbce, 0xac9f84, 0x477c9e, 0x44789c, 0x2a5270, 0x5c9ea3, 0xe7fdcd, 0x6795ba, 0x2d5674, \
        0xf8e1b7, 0xfefec9, 0xaed0a6, 0x95ccc3, 0x97ece2, 0x8ab9c2, 0x5ea0ba, 0x76aeb2, 0x2b5f7c, 0xc4d0a0, \
        0x4b81a2, 0xa8bd8b, 0xf0bfac, 0x244964, 0x79ac8e, 0x3f758f, 0x34607a, 0xe2b29f, 0x204561, 0xa5f9ec, \
        0x4e8095, 0x949b75, 0x022141, 0x1a3748, 0xb0c492, 0x5c909f, 0x85ccd9, 0x64a8c0, 0x1f3b4c, 0x61a6bf, \
        0x2a5c79, 0x6fa78c, 0x8ee7de, 0xfbf6c3, 0x538598, 0xaabf8f, 0xfce6bb, 0x001d3e, 0x3b6f89, 0x6a98bc, \
        0x1c405c, 0x3c573b, 0x78936a, 0x285876, 0x265674, 0x69855e, 0xcef8dc, 0x526646, 0x042343, 0x5a7755, \
        0x244051, 0x6d9ec1, 0x45807a, 0xe7b6a0, 0xa2b271, 0xf1fed1, 0x96a96b, 0xc0ce98, 0xb8c67d, 0x26475d, \
        0x68adc2, 0xb6a88d, 0x4b7c92, 0x6a9fac, 0x637e5b, 0x6d8b6d, 0x839a6d, 0x396781, 0x748c5d, 0xd9f6ca, \
        0xd1ba9a, 0x7cb3b8, 0x9edace, 0xbbf1da, 0x245372, 0x538394, 0x23516f, 0x6fa5af, 0xa9e6d5, 0xf7d9b2, \
        0x092a4c, 0x173852, 0x80965f, 0x7b867e, 0x42769b, 0x5b9782, 0x12395d, 0x4f8a7d, 0x0c3155, 0x2b4c61, \
        0xadbd75, 0x082848, 0x768177, 0x1a4769, 0x7fc0d3, 0x183243, 0x3d7299, 0x90d2c9, 0x336a86, 0x98be9d, \
        0xe6fed8, 0xc5d181, 0xf0cfac, 0xd9e08e, 0x164062, 0x2f6b73, 0x0d2f46, 0x94b098, 0xebbba3, 0x83776e, \
        0xbdd9af, 0x314a54, 0xbaca95, 0xcdd9a7, 0x4c674f, 0xa3c7a3, 0x53585d, 0xced78a, 0x204e6e, 0x8ea994, \
        0x9aae81, 0x37554e, 0x3a6e95, 0x83b38f, 0x589aa0, 0xa4b98b, 0x606262, 0x8ca167, 0x597763, 0x61806a, \
        0x79716a, 0x30566a, 0x90a578, 0x1e3744, 0x71a9b6, 0x001a3d, 0x09283e, 0x1a3c56, 0x5388a6, 0x132e42, \
        0xabd3bf, 0x415f4d, 0x05223e, 0x464f58, 0x73adc4, 0x7dcec8, 0x273a44, 0xb5e3cd, 0x3c4852, 0x4e7a87, \
        0x495f45, 0x547050, 0x184365, 0x0c2942, 0x9dc6b8, 0xeec4a7, 0xdac3a2, 0x153d5f, 0x8abdb6, 0x123348, \
        0x354d48, 0xa99c82, 0x456e7d, 0x437287, 0x2c4b4a, 0x274f6a, 0x30404c, 0x113758, 0x0f3254, 0x54979e, \
        0x6599a4, 0x506d5e, 0x46655d, 0x3d7777, 0x14334e, 0x89b2a8, 0x5f724b, 0x405644, 0x8a8173, 0x6d6a67, \
        0x071f39, 0xc1ad91, 0xe5ea99, 0x1e4b6a, 0x254246, 0xdae4b6, 0xb7c6ac, 0x697f52, 0x3b5a5e, 0x7ab5c8, \
        0x87e3dc, 0x3e6778, 0x021e3a, 0x99e8dd, 0xe6efc0, 0x112639, 0x618b93, 0xc9e1c1, 0x152b3d, 0x40636d, \
        0xf6f8b0, 0x6092b6, 0x99907d, 0x061930, 0x5994ad, 0x5399a3, 0x938878, 0x7ba9a4, 0x5a8dad, 0x2f473f, \
        0x011a37, 0x6e9e82, 0x6fa4a3, 0x62a9ae, 0xa2977e, 0x011734, 0x092d42, 0x6d989d, 0x00102b, 0x6db6b8, \
        0x1f313c, 0x83a19e, 0x0d2135, 0x2b646e, 0x5e9bb2, 0x00183a, 0x366b8f, 0x8cdad5, 0x7bc5c5, 0x00143b, \
        0x5d837e, 0x789691, 0x001036, 0x265f6b, 0xdeae9c.\""
    )
}
