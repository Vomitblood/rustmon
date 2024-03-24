use clap::{App, Arg};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fs;
use std::{fs::File, io::Read, path::Path};

fn main() {
    let matches = App::new("pokemon-colorscripts")
        .version("0.1.0")
        .author("Your Name")
        .about("Displays Pokemon in your terminal")
        .arg(
            Arg::with_name("list")
                .short('l')
                .long("list")
                .help("Print list of all Pokemon"),
        )
        .arg(
            Arg::with_name("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Select Pokémon by name.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("form")
                .short('f')
                .long("form")
                .value_name("FORM")
                .help("Show an alternate form of a Pokémon")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("shiny")
                .short('s')
                .long("shiny")
                .help("Show the shiny version of the Pokémon instead"),
        )
        .arg(
            Arg::with_name("big")
                .short('b')
                .long("big")
                .help("Show a larger version of the sprite"),
        )
        .arg(
            Arg::with_name("random")
                .short('r')
                .long("random")
                .value_name("GEN")
                .help("Show a random Pokémon. Optionally specify a generation.")
                .takes_value(true),
        )
        // Add more arguments as per your Python script
        .get_matches();

    if matches.is_present("list") {
        list_pokemon_names();
    }

    if let Some(name) = matches.value_of("name") {
        let shiny = matches.is_present("shiny");
        let is_large = matches.is_present("big");
        let form = matches.value_of("form");
        show_pokemon_by_name(name, shiny, is_large, form);
    } else if matches.is_present("random") {
        let generation = matches.value_of("random");
        let shiny = matches.is_present("shiny");
        let is_large = matches.is_present("big");
        show_random_pokemon(generation, shiny, is_large);
    }
    // Handle other commands similarly
}

#[derive(Serialize, Deserialize)]
struct Pokemon {
    name: String,
    // Define other fields as needed
}

fn load_pokemon_data() -> Vec<Pokemon> {
    let path = Path::new("pokemon.json");
    let mut file = File::open(&path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    serde_json::from_str(&contents).expect("Failed to parse JSON")
}

fn list_pokemon_names() {
    let pokemon_list = load_pokemon_data();
    for pokemon in pokemon_list {
        println!("{}", pokemon.name);
    }
}

fn print_file(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(contents) => println!("{}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn show_pokemon_by_name(name: &str, shiny: bool, is_large: bool, form: Option<&str>) {
    let color_subdir = if shiny { "shiny" } else { "regular" };
    let size_subdir = if is_large { "large" } else { "small" };
    let mut pokemon_name = name.to_string();

    if let Some(f) = form {
        pokemon_name.push_str("-");
        pokemon_name.push_str(f);
    }

    let filepath = format!(
        "{}/{}/{}/{}",
        "path_to_colorscripts_dir", size_subdir, color_subdir, pokemon_name
    );

    println!("Showing: {}", pokemon_name);
    print_file(&filepath);
}

fn show_random_pokemon(generation: Option<&str>, shiny: bool, is_large: bool) {
    let pokemon_list = load_pokemon_data();
    // Simplified: Add logic to filter Pokémon based on the generation range
    let mut rng = thread_rng();
    let pokemon_index = rng.gen_range(0..pokemon_list.len());
    let pokemon_name = &pokemon_list[pokemon_index].name;

    // Determine shininess based on a random chance or if explicitly requested
    let is_shiny = if shiny {
        true
    } else {
        rng.gen_bool(1.0 / 128.0)
    };

    show_pokemon_by_name(pokemon_name, is_shiny, is_large, None);
}
