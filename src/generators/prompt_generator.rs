use crate::utils::text_processor::TextProcessor;
use anyhow::Result;
use pyo3::prelude::*;
use rand::Rng;

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
        let text_prompt = generate_random_rpg_prompt();

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

/// Background themes for RPG parallax layers
#[derive(Clone, Debug)]
pub enum BackgroundTheme {
    Forest,
    Mountain,
    Desert,
    Ocean,
    City,
    Village,
    Cave,
    Sky,
}

/// Generates nature/cityscape appropriate elements for each parallax layer
impl BackgroundTheme {
    fn get_layer_elements(&self, layer: u8) -> (String, String, String) {
        match (self, layer) {
            // Forest theme
            (BackgroundTheme::Forest, 0) => (
                "Pale sky blue (#E6F3FF)".to_string(),
                "Distant mountain silhouettes and cloud wisps".to_string(),
                "soft gray and light blue mountain peaks, wispy white clouds".to_string()
            ),
            (BackgroundTheme::Forest, 1) => (
                "Soft green (#E8F5E8)".to_string(),
                "Far forest treeline and rolling hills".to_string(),
                "dark green tree silhouettes, gentle hill contours".to_string()
            ),
            (BackgroundTheme::Forest, 2) => (
                "Medium forest green (#C8E6C9)".to_string(),
                "Mid-distance trees and foliage".to_string(),
                "detailed pine and oak trees, scattered bushes, dappled sunlight".to_string()
            ),
            (BackgroundTheme::Forest, 3) => (
                "Rich dark green (#2E7D32)".to_string(),
                "Foreground vegetation and tree trunks".to_string(),
                "large tree trunks, detailed bark texture, ferns, mushrooms, fallen logs".to_string()
            ),

            // Mountain theme
            (BackgroundTheme::Mountain, 0) => (
                "Light mountain blue (#F0F8FF)".to_string(),
                "Distant peaks and sky".to_string(),
                "snow-capped mountain peaks, wispy clouds, pale blue sky gradient".to_string()
            ),
            (BackgroundTheme::Mountain, 1) => (
                "Cool gray-blue (#B0C4DE)".to_string(),
                "Mid-distance rocky slopes".to_string(),
                "rocky cliff faces, distant waterfalls, alpine meadows".to_string()
            ),
            (BackgroundTheme::Mountain, 2) => (
                "Earthy brown-gray (#8D6E63)".to_string(),
                "Near mountain features".to_string(),
                "detailed rock formations, scattered boulders, hardy mountain plants".to_string()
            ),
            (BackgroundTheme::Mountain, 3) => (
                "Dark stone gray (#424242)".to_string(),
                "Foreground rocks and debris".to_string(),
                "large weathered stones, pebbles, mountain grass tufts, lichen".to_string()
            ),

            // City theme
            (BackgroundTheme::City, 0) => (
                "Urban sky gray (#F5F5F5)".to_string(),
                "Distant city skyline".to_string(),
                "tall skyscrapers silhouettes, radio towers, distant smoke stacks".to_string()
            ),
            (BackgroundTheme::City, 1) => (
                "Cool concrete gray (#E0E0E0)".to_string(),
                "Mid-distance buildings".to_string(),
                "apartment buildings, office towers, rooftop details, windows".to_string()
            ),
            (BackgroundTheme::City, 2) => (
                "Urban brown-gray (#BCAAA4)".to_string(),
                "Near city structures".to_string(),
                "detailed building facades, fire escapes, street lamps, signs".to_string()
            ),
            (BackgroundTheme::City, 3) => (
                "Dark asphalt (#424242)".to_string(),
                "Street level elements".to_string(),
                "street furniture, trash cans, mailboxes, sidewalk cracks, manholes".to_string()
            ),

            // Ocean theme
            (BackgroundTheme::Ocean, 0) => (
                "Horizon blue (#E3F2FD)".to_string(),
                "Sky and distant horizon".to_string(),
                "seagulls, distant islands, horizon line, wispy clouds".to_string()
            ),
            (BackgroundTheme::Ocean, 1) => (
                "Ocean blue (#81D4FA)".to_string(),
                "Far ocean waves".to_string(),
                "gentle wave crests, distant sailing ships, sea foam".to_string()
            ),
            (BackgroundTheme::Ocean, 2) => (
                "Deep sea blue (#0277BD)".to_string(),
                "Near ocean surface".to_string(),
                "detailed wave patterns, floating debris, sea spray".to_string()
            ),
            (BackgroundTheme::Ocean, 3) => (
                "Sandy beige (#D7CCC8)".to_string(),
                "Beach and shore elements".to_string(),
                "sand dunes, seashells, driftwood, beach grass, rocks".to_string()
            ),

            // Default fallback
            _ => (
                "Light neutral (#F5F5F5)".to_string(),
                "Generic background elements".to_string(),
                "simple repeating natural elements".to_string()
            ),
        }
    }
}

/// Returns the system prompt for RPG parallax generation
fn get_system_prompt() -> String {
    String::from(
        "You are an expert 2D RPG background artist specializing in parallax scrolling layers. \
        Create natural, organic backgrounds suitable for side-scrolling RPG games. \
        Focus on realistic nature and cityscape elements that feel immersive and atmospheric. \
        Each layer must be visually distinct with different depths and color tones. \
        All elements must tile seamlessly horizontally for infinite scrolling."
    )
}

/// Generates a themed RPG parallax background prompt
fn generate_rpg_parallax_prompt(theme: BackgroundTheme) -> String {
    let mut rng = rand::thread_rng();

    // Get elements for each layer
    let (color0, desc0, elements0) = theme.get_layer_elements(0);
    let (color1, desc1, elements1) = theme.get_layer_elements(1);
    let (color2, desc2, elements2) = theme.get_layer_elements(2);
    let (color3, desc3, elements3) = theme.get_layer_elements(3);

    // Add some variation
    let lighting = match rng.gen_range(0..4) {
        0 => "dawn lighting with soft golden glow",
        1 => "midday bright natural lighting",
        2 => "dusk lighting with warm orange tones",
        _ => "overcast lighting with soft shadows",
    };

    format!(
        "Create a 1024x1024 2D RPG parallax background with four distinct horizontal strips, each 256px tall. \
        Theme: {:?} landscape with {}.\n\n\
        **Top Strip (0-256px):** Distant background layer\n\
        - Color tone: {}\n\
        - Elements: {} that tile seamlessly horizontally\n\
        - Style: Soft, atmospheric, low detail for maximum distance\n\n\
        **Second Strip (256-512px):** Mid-background layer\n\
        - Color tone: {}\n\
        - Elements: {} with moderate detail\n\
        - Style: More defined than distance layer but still atmospheric\n\n\
        **Third Strip (512-768px):** Near-background layer\n\
        - Color tone: {}\n\
        - Elements: {} with good detail and texture\n\
        - Style: Clearly defined features, medium contrast\n\n\
        **Bottom Strip (768-1024px):** Foreground layer\n\
        - Color tone: {}\n\
        - Elements: {} with high detail and contrast\n\
        - Style: Sharp, detailed, highest contrast for immediate foreground\n\n\
        **Critical Requirements:**\n\
        - Each strip must have distinctly different color tones for easy layer extraction\n\
        - NO gradients or blending between the four 256px strips\n\
        - Sharp horizontal divisions between each layer\n\
        - All elements within each strip tile perfectly horizontally\n\
        - Natural, organic RPG game aesthetic - avoid geometric or abstract shapes",
        theme, lighting, color0, elements0, color1, elements1, color2, elements2, color3, elements3
    )
}

/// Generate multiple themed prompts for variety
pub fn generate_random_rpg_prompt() -> String {
    let mut rng = rand::thread_rng();
    let themes = vec![
        BackgroundTheme::Forest,
        BackgroundTheme::Mountain,
        BackgroundTheme::Desert,
        BackgroundTheme::Ocean,
        BackgroundTheme::City,
        BackgroundTheme::Village,
        BackgroundTheme::Cave,
    ];

    let chosen_theme = themes[rng.gen_range(0..themes.len())].clone();
    generate_rpg_parallax_prompt(chosen_theme)
}