use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
struct Input {
    #[serde(flatten)]
    other: HashMap<String, Value>, // Capturing the rest of the data
    gen_8: Option<Generation8>, // Making gen_8 optional
}

#[derive(Deserialize)]
struct Generation8 {
    forms: HashMap<String, Value>,
}

#[derive(Serialize)]
struct Pokemon {
    name: String,
    id: String,
    forms: Vec<String>,
}

fn main() -> Result<()> {
    let data = fs::read_to_string("pokemon.json").expect("Unable to read file");
    let input_pokemons: HashMap<String, Input> = serde_json::from_str(&data)?;

    let mut output_pokemons: Vec<Pokemon> = Vec::new();

    for (id, pokemon) in input_pokemons {
        // Check if gen_8 data exists
        let forms: Vec<String> = if let Some(gen_8) = pokemon.gen_8 {
            gen_8
                .forms
                .keys()
                .map(|key| {
                    if key == "$" {
                        "regular".to_string()
                    } else {
                        key.clone()
                    }
                })
                .collect()
        } else {
            // If there's no gen_8 data, you might want to handle it differently
            // For now, let's just use an empty Vec
            Vec::new()
        };

        // Proceed as before
        output_pokemons.push(Pokemon {
            name: pokemon.other["name"]["eng"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            id,
            forms,
        });
    }

    let output_json = serde_json::to_string_pretty(&output_pokemons)?;
    println!("{}", output_json);

    fs::write("output_pokemon.json", output_json).expect("Unable to write file");

    Ok(())
}
