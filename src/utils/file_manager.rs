use chrono::NaiveDate;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

/// FileManager - Handles file operations for images, prompts, and README updates.
pub struct FileManager;

impl FileManager {
    /// Get the project root directory.
    ///
    /// Returns:
    ///     PathBuf to project root directory
    fn get_project_root() -> PathBuf {
        let current_exe = std::env::current_exe().expect("Failed to get executable path");
        current_exe.parent().unwrap()
            .parent().unwrap()
            .parent().unwrap()
            .parent().unwrap()
            .to_path_buf()
    }

    /// Create directory if it doesn't exist.
    ///
    /// # Arguments
    /// * `directory` - Path to directory to create
    fn ensure_directory_exists(directory: &str) -> io::Result<()> {
        let dir_path = directory;
        fs::create_dir_all(&dir_path)?;
        println!("Directory '{}' exists or created successfully.", directory);
        Ok(())
    }

    /// Save image with both timestamped and current filenames.
    ///
    /// # Arguments
    /// * `image_bytes` - Image data to save
    /// * `current_date` - Current date for timestamping
    pub fn save_image(image_bytes: &[u8], current_date: NaiveDate) -> io::Result<()> {
        Self::ensure_directory_exists("images")?;

        // Save timestamped version
        let timestamped_path = format!("images/image_{}.png", current_date);
        let mut file = File::create(&timestamped_path)?;
        file.write_all(image_bytes)?;
        println!("Image '{}' saved successfully.", timestamped_path);

        // Save current version
        let current_path = "images/image_current.png";
        let mut file = File::create(&current_path)?;
        file.write_all(image_bytes)?;
        println!("Image '{}' saved successfully.", current_path);

        Ok(())
    }

    /// Save prompt with both timestamped and current filenames.
    ///
    /// # Arguments
    /// * `prompt` - Prompt text to save
    /// * `current_date` - Current date for timestamping
    pub fn save_prompt(prompt: &str, current_date: NaiveDate) -> io::Result<()> {
        Self::ensure_directory_exists("prompts")?;

        // Save timestamped version
        let timestamped_path = format!("prompts/prompt_{}.txt", current_date);
        let mut file = File::create(&timestamped_path)?;
        file.write_all(prompt.as_bytes())?;
        println!("Prompt '{}' saved successfully.", timestamped_path);

        // Save current version
        let current_path = "prompts/prompt_current.txt";
        let mut file = File::create(&current_path)?;
        file.write_all(prompt.as_bytes())?;
        println!("Prompt '{}' saved successfully.", current_path);

        Ok(())
    }

    /// Update README file with the new prompt.
    ///
    /// # Arguments
    /// * `prompt` - Prompt text to add to README
    pub fn update_readme(prompt: &str) -> io::Result<()> {
        let readme_path = "README.md";

        match fs::read_to_string(&readme_path) {
            Ok(content) => {
                let mut updated_content = String::new();
                let mut found_screenshot = false;

                for line in content.lines() {
                    updated_content.push_str(line);
                    updated_content.push('\n');

                    if line.trim() == "![image](images/image_current.png)" {
                        updated_content.push_str(&format!("\n**Prompt:** {}\n", prompt));
                        found_screenshot = true;
                        break;
                    }
                }

                if found_screenshot {
                    fs::write(&readme_path, updated_content)?;
                    println!("README updated successfully.");
                }

                Ok(())
            }
            Err(_) => {
                println!("Warning: README.md not found, skipping README update.");
                Ok(())
            }
        }
    }
}