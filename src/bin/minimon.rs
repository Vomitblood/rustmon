use rand::prelude::SliceRandom;
use rust_embed::RustEmbed;
use std::str;

#[derive(RustEmbed)]
#[folder = "colorscripts/small/regular/"]
struct ColorScriptsDir;

fn main() {
    let files: Vec<_> = ColorScriptsDir::iter().collect();
    let mut rng = rand::thread_rng();

    if let Some(random_file) = files.choose(&mut rng) {
        println!("{}", random_file);
        if let Some(file_data) = ColorScriptsDir::get(random_file) {
            println!("{}", std::str::from_utf8(file_data.data.as_ref()).unwrap());
        }
    }
}
