use std::io;
use std::fs::{self, File};
use std::io::Write;
use crate::utils::sample_text;

pub fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}



pub fn create_folder_and_file(folder_path: &str) -> std::io::Result<()> {
    let src_folder_path = format!("{}/src", folder_path);
    let asset_path = format!("./{}/assets", folder_path);
    let file_path = format!("./{}/src/main.py", folder_path);
    let todo_file_path = format!("./{}/TODO.md", folder_path);

    fs::create_dir_all(&src_folder_path)?;
    fs::create_dir_all(&asset_path)?;

    let mut file = File::create(&file_path)?;
    
    writeln!(file, "{}", sample_text::PYTHON_CODE)?;

    let mut todo_file = File::create(&todo_file_path)?;
    writeln!(todo_file, "{}", sample_text::TODO_SAMPLE)?;

    println!("Folder and file created successfully at: {}", file_path);

    Ok(())
}
