use rand::Rng;
use std::io::prelude::*;

// set global constants
const PROGRAM_DIR: once_cell::sync::Lazy<std::path::PathBuf> = once_cell::sync::Lazy::new(|| {
    std::path::PathBuf::from(std::env::current_exe().unwrap().parent().unwrap())
});

const COLORSCRIPTS_DIR: once_cell::sync::Lazy<std::path::PathBuf> =
    once_cell::sync::Lazy::new(|| PROGRAM_DIR.join("colorscripts"));

const POKEMON_DATA_PATH: once_cell::sync::Lazy<std::path::PathBuf> =
    once_cell::sync::Lazy::new(|| PROGRAM_DIR.join("pokemon.json"));

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

fn print_file(filepath: &std::path::Path) -> std::io::Result<()> {
    let file = std::fs::File::open(filepath)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn list_pokemon_names() -> std::io::Result<()> {
    let file = std::fs::File::open(POKEMON_DATA_PATH.as_path())?;
    let reader = std::io::BufReader::new(file);
    let pokemon_json: serde_json::Value = serde_json::from_reader(reader)?;

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

    println!("Total: {} Pokémons", count);

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
    let base_path = COLORSCRIPTS_DIR;
    let color_subdir = if shiny { SHINY_SUBDIR } else { REGULAR_SUBDIR };
    let size_subdir = if is_large { LARGE_SUBDIR } else { SMALL_SUBDIR };

    let file = std::fs::File::open(POKEMON_DATA_PATH.as_path())?;
    let reader = std::io::BufReader::new(file);
    let pokemon_json: serde_json::Value = serde_json::from_reader(reader)?;

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

    let pokemon_file = base_path.join(size_subdir).join(color_subdir).join(&name);

    if show_title {
        if shiny {
            println!("{} (shiny)", name);
        } else {
            println!("{}", name);
        }
    }

    print_file(&pokemon_file)
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

    let file = std::fs::File::open(POKEMON_DATA_PATH.as_path())?;
    let reader = std::io::BufReader::new(file);
    let pokemon_json: serde_json::Value = serde_json::from_reader(reader)?;
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

// fn main() {
//     // println!("{}", PROGRAM.display());
//     // println!("{}", PROGRAM_DIR.display());
//     // println!("{}", COLORSCRIPTS_DIR.display());
//     // show_pokemon_by_name("eevee", false, false, false, Some("gmax")).unwrap();
//     show_random_pokemon("7-8", true, false, false);
//     list_pokemon_names();
// }

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
        show_pokemon_by_name(name, no_title, shiny, big, form).unwrap();
    } else if matches.is_present("random") {
        let random = matches.value_of("random").unwrap_or("");
        let no_title = matches.is_present("no-title");
        let shiny = matches.is_present("shiny");
        let big = matches.is_present("big");
        if matches.is_present("form") {
            println!("--form flag unexpected with --random");
            std::process::exit(1);
        }
        show_random_pokemon(random, no_title, shiny, big).unwrap();
    } else {
        show_random_pokemon("", true, false, false).unwrap();
    }
}
