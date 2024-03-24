use rand::Rng;
use std::io::prelude::*;

// set global constants
const PROGRAM: once_cell::sync::Lazy<std::path::PathBuf> =
    once_cell::sync::Lazy::new(|| std::env::current_exe().unwrap());

const PROGRAM_DIR: once_cell::sync::Lazy<std::path::PathBuf> = once_cell::sync::Lazy::new(|| {
    std::path::PathBuf::from(std::env::current_exe().unwrap().parent().unwrap())
});

const COLORSCRIPTS_DIR: once_cell::sync::Lazy<std::path::PathBuf> =
    once_cell::sync::Lazy::new(|| PROGRAM_DIR.join("colorscripts"));

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

fn list_pokemon_names(filepath: &std::path::Path) -> std::io::Result<()> {
    let file = std::fs::File::open(filepath)?;
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

    let file = std::fs::File::open(PROGRAM_DIR.join("pokemon.json"))?;
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

    let (start_gen, end_gen) = if generations.is_empty() {
        ("1", "8")
    } else if generations.contains(",") {
        let gens: Vec<&str> = generations.split(",").collect();
        let gen = gens[rng.gen_range(0..gens.len())];
        (gen, gen)
    } else if generations.contains("-") {
        let gens: Vec<&str> = generations.split("-").collect();
        (gens[0], gens[1])
    } else {
        (generations, generations)
    };

    let file = std::fs::File::open(PROGRAM_DIR.join("pokemon.json"))?;
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

fn main() {
    // println!("{}", PROGRAM.display());
    // println!("{}", PROGRAM_DIR.display());
    // println!("{}", COLORSCRIPTS_DIR.display());
    // show_pokemon_by_name("eevee", false, false, false, Some("gmax")).unwrap();
    show_random_pokemon("1-999", true, false, false);
}
