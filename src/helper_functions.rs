use std::fs::File;
use std::io::Read;
use std::path::Path;
use clipboard::{ClipboardContext, ClipboardProvider};

// Load a txt file and return a string
pub(crate) fn load_txt_file(path : String) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

/// Function to copy text to the clipboard
pub(crate) fn copy_to_clipboard(text: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard: ClipboardContext = ClipboardProvider::new()?; // Create clipboard context
    clipboard.set_contents(text.to_string())?; // Copy the text to clipboard
    Ok(())
}