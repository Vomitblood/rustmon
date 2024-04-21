use std::io::Read;

pub fn print(big: bool, pokedex: u16, name: &String, no_title: bool, random: bool, shiny: f32) {
    println!("Big: {}", big);
    println!("pokedex: {}", pokedex);
    println!("Name: {}", name);
    println!("No title: {}", no_title);
    println!("Random: {}", random);
    println!("Shiny: {}", shiny);

    // decide which function to call
    // random, by pokedex or by name
    if random {
        // random
        println!("Random");
    } else if pokedex > 0 {
        // by pokedex
        println!("By pokedex");
    } else {
        // by name
        println!("By name");
    }

    match find_pokemon_by_pokedex(&pokedex.to_string()) {
        Ok(pokemon_name) => println!("Found Pokémon: {}", pokemon_name),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn find_pokemon_by_pokedex(pokedex_number: &str) -> Result<String, Box<dyn std::error::Error>> {
    // read the file
    let mut file = std::fs::File::open(crate::constants::DATA_DIRECTORY.join("pokemon.json"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // deserialize into the struct
    let pokemons: Vec<crate::structs::Pokemon> = serde_json::from_str(&contents)?;

    // iterate through the list to find the specified pokemon
    for pokemon in pokemons {
        if pokemon.pokedex == pokedex_number {
            // if found then return the name
            return Ok(pokemon.name);
        }
    }

    // if not found the return an error
    return Err("Pokemon not found".into());
}
