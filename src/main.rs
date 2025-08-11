mod app;

use crate::app::config::Config;
use arboard::Clipboard;

// use std::time::Instant;

fn main() {
    // let now = Instant::now();
    let password_length = 128; // Default password length

    let config = Config::new(password_length, true);
    // let elapsed = now.elapsed();
    // println!("Time to create config: {:.2?}", elapsed);

    // let now = Instant::now();
    let mut clipboard = Clipboard::new().unwrap();
    let password = config.generate_password();
    println!("Generated password: {}", password);
    clipboard.set_text(password).unwrap();
    println!("\nCopied password to clipboard.");
    // let elapsed = now.elapsed();
    // println!("Time to generate password: {:.2?}", elapsed);
}
