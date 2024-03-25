use rand::Rng;

// set global constants
const COLORSCRIPTS_DIR: include_dir::Dir = include_dir::include_dir!("./colorscripts");

const POKEMON_JSON: &str = std::include_str!("../pokemon.json");

const REGULAR_SUBDIR: &str = "regular";
const SHINY_SUBDIR: &str = "shiny";

const LARGE_SUBDIR: &str = "large";
const SMALL_SUBDIR: &str = "small";

const SHINY_RATE: f64 = 1.0 / 128.0;

const GENERATIONS: [(&str, (u32, u32)); 8] = [
    ("1", (1, 151)),
    ("2", (152, 251)),
    ("3", (252, 386)),
    ("4", (387, 493)),
    ("5", (494, 649)),
    ("6", (650, 721)),
    ("7", (722, 809)),
    ("8", (810, 898)),
];

fn print_file(filepath: &str) -> std::io::Result<()> {
    if let Some(file) = COLORSCRIPTS_DIR.get_file(filepath) {
        let content = std::str::from_utf8(file.contents()).unwrap();
        println!("{}", content);
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        ))
    }
}

fn list_pokemon_names() -> std::io::Result<()> {
    let pokemon_json: serde_json::Value = serde_json::from_str(POKEMON_JSON)?;

    let mut count = 0;

    if let serde_json::Value::Array(array) = pokemon_json {
        for pokemon in array {
            if let Some(name) = pokemon.get("name") {
                if let serde_json::Value::String(name_str) = name {
                    println!("{}", name_str);
                    count += 1;
                }
            }
        }
    }

    println!("-------------------");
    println!("Total: {} Pokémons", count);
    println!("Use the --name flag to view a specific Pokémon");
    println!("Tip: Use `grep` to search for a specific Pokémon");

    Ok(())
}

fn show_pokemon_by_name(
    name: &str,
    show_title: bool,
    shiny: bool,
    is_large: bool,
    form: Option<&str>,
) -> std::io::Result<()> {
    // set variables
    let color_subdir = if shiny { SHINY_SUBDIR } else { REGULAR_SUBDIR };
    let size_subdir = if is_large { LARGE_SUBDIR } else { SMALL_SUBDIR };

    let pokemon_json: serde_json::Value = serde_json::from_str(POKEMON_JSON)?;

    let pokemon_names: Vec<&str> = pokemon_json
        .as_array()
        .unwrap()
        .iter()
        .map(|pokemon| pokemon["name"].as_str().unwrap())
        .collect();

    if !pokemon_names.contains(&name) {
        println!("Invalid pokemon {}", name);
        std::process::exit(1);
    }

    let mut name = name.to_string();

    if let Some(form) = form {
        let forms: Vec<&str> = pokemon_json
            .as_array()
            .unwrap()
            .iter()
            .filter(|pokemon| pokemon["name"].as_str().unwrap() == name)
            .flat_map(|pokemon| pokemon["forms"].as_array().unwrap().iter())
            .map(|form| form.as_str().unwrap())
            .collect();

        let alternate_forms: Vec<&str> =
            forms.iter().filter(|&f| *f != "regular").cloned().collect();

        if alternate_forms.contains(&form) {
            name.push_str(&format!("-{}", form));
        } else {
            println!("Invalid form '{}' for pokemon {}", form, name);
            if alternate_forms.is_empty() {
                println!("No alternate forms available for {}", name);
            } else {
                println!("Available alternate forms are");
                for form in alternate_forms {
                    println!("- {}", form);
                }
            }
            std::process::exit(1);
        }
    }

    if show_title {
        if shiny {
            println!("{} (shiny)", name);
        } else {
            println!("{}", name);
        }
    }

    // Construct the embedded file path
    let file_path = format!("{}/{}/{}", size_subdir, color_subdir, name);

    // Use the adjusted function to print file contents from embedded resources
    print_file(&file_path)?;

    Ok(())
}

fn show_random_pokemon(
    generations: &str,
    show_title: bool,
    shiny: bool,
    is_large: bool,
) -> std::io::Result<()> {
    let mut rng = rand::thread_rng();

    let start_gen = if generations.is_empty() {
        "1"
    } else if generations.contains(",") {
        let gens: Vec<&str> = generations.split(",").collect();
        let gen = gens[rng.gen_range(0..gens.len())];
        gen
    } else if generations.contains("-") {
        let gens: Vec<&str> = generations.split("-").collect();
        gens[0]
    } else {
        generations
    };

    let pokemon_json: serde_json::Value = serde_json::from_str(POKEMON_JSON)?;
    let pokemon: Vec<String> = pokemon_json
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["name"].as_str().unwrap().to_string())
        .collect();

    let generations_map: std::collections::HashMap<_, _> = GENERATIONS.iter().cloned().collect();

    if let Some((start_idx, end_idx)) = generations_map.get(start_gen) {
        let random_idx = rng.gen_range(*start_idx..=*end_idx);
        let random_pokemon = &pokemon[random_idx as usize - 1];
        let shiny = if !shiny {
            rng.gen::<f64>() <= SHINY_RATE
        } else {
            shiny
        };
        show_pokemon_by_name(random_pokemon, show_title, shiny, is_large, None)?;
    } else {
        println!("Invalid generation '{}'", generations);
        std::process::exit(1);
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn pause() {
    use std::io::{self, Read, Write};
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();

    stdout.write_all(b"Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    stdin.read(&mut [0]).unwrap();
}

#[cfg(not(target_os = "windows"))]
fn pause() {
    // do literally nothing
}

fn main() {
    let matches = clap::App::new("pokemon-colorscripts")
        .about("CLI utility to print out unicode image of a pokemon in your shell")
        .arg(
            clap::Arg::with_name("list")
                .short("l")
                .long("list")
                .help("Print list of all pokemon"),
        )
        .arg(
            clap::Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("POKEMON NAME")
                .help("Select pokemon by name. Generally spelled like in the games."),
        )
        .arg(
            clap::Arg::with_name("form")
                .short("f")
                .long("form")
                .value_name("FORM")
                .help("Show an alternate form of a pokemon"),
        )
        .arg(
            clap::Arg::with_name("no-title")
                .long("no-title")
                .help("Do not display pokemon name"),
        )
        .arg(
            clap::Arg::with_name("shiny")
                .short("s")
                .long("shiny")
                .help("Show the shiny version of the pokemon instead"),
        )
        .arg(
            clap::Arg::with_name("big")
                .short("b")
                .long("big")
                .help("Show a larger version of the sprite"),
        )
        .arg(
            clap::Arg::with_name("random")
                .short("r")
                .long("random")
                .value_name("GENERATION")
                .help("Show a random pokemon. This flag can optionally be followed by a generation number or range (1-8) to show random pokemon from a specific generation or range of generations. The generations can be provided as a continuous range (eg. 1-3) or as a list of generations (1,3,6)"),
        )
        .get_matches();

    if matches.is_present("list") {
        list_pokemon_names().unwrap();
    } else if matches.is_present("name") {
        let name = matches.value_of("name").unwrap();
        let no_title = matches.is_present("no-title");
        let shiny = matches.is_present("shiny");
        let big = matches.is_present("big");
        let form = matches.value_of("form");
        show_pokemon_by_name(name, !no_title, shiny, big, form).unwrap();
    } else if matches.is_present("random") {
        let random = matches.value_of("random").unwrap_or("");
        let no_title = matches.is_present("no-title");
        let shiny = matches.is_present("shiny");
        let big = matches.is_present("big");
        if matches.is_present("form") {
            println!("--form flag unexpected with --random");
            std::process::exit(1);
        }
        show_random_pokemon(random, !no_title, shiny, big).unwrap();
    } else {
        // show random pokemon by default with support for other flags
        let no_title = matches.is_present("no-title");
        let shiny = matches.is_present("shiny");
        let big = matches.is_present("big");
        show_random_pokemon("", !no_title, shiny, big).unwrap();
    }

    // pause the program before exiting only for windows
    pause();
}
