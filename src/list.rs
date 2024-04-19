#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Pokemon {
    pokedex: String,
    name: String,
    forms: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct PokemonList {
    pokemons: Vec<Pokemon>,
}

pub fn print_pokemon_list() -> Result<(), serde_json::Error> {
    // open the file in read only mode with buffer
    let file = std::fs::File::open(crate::constants::DATA_DIRECTORY.join("pokemon.json"))
        .expect("File not found");
    let reader = std::io::BufReader::new(file);

    // parse json into pokemonlist struct
    let pokemon_list: Vec<Pokemon> = serde_json::from_reader(reader)?;

    // iterate through the vector and print the pokedex and name
    for pokemon in pokemon_list {
        println!("[{}] {}", pokemon.pokedex, pokemon.name);
    }

    println!("\nHint: Having trouble finding a Pokemon? Pass in --help to see tips!");

    return Ok(());
}

pub fn print_pokemon_forms(pokemon_name: &str) -> std::io::Result<()> {
    // open the file in read only mode with buffer
    let file = std::fs::File::open(crate::constants::DATA_DIRECTORY.join("pokemon.json"))?;
    let reader = std::io::BufReader::new(file);

    // parse json into pokemonlist struct
    let pokemon_list: Vec<Pokemon> = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // iterate through the list to find the specified pokemon and print its forms
    let mut found = false;
    for pokemon in pokemon_list {
        // case insensitive comparison
        if pokemon.name.eq_ignore_ascii_case(pokemon_name) {
            println!("{} has the following forms:", pokemon.name);
            for form in pokemon.forms {
                println!(" - {}", form);
            }
            found = true;
            break;
        }
    }

    if !found {
        println!("No Pokemon found with the name '{}'.", pokemon_name);
        println!("Hint: Do `rustmon list` to see all available Pokemon.")
    }

    return Ok(());
}
