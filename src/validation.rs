pub fn validate_files() {
    match validate_pokemon_json() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match validate_colorscripts_directory() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}

fn validate_pokemon_json() -> Result<(), Box<dyn std::error::Error>> {
    let file_path: std::path::PathBuf = crate::constants::DATA_DIRECTORY.join("pokemon.json");

    // check if pokemon.json exists
    if !crate::constants::DATA_DIRECTORY
        .join("pokemon.json")
        .exists()
    {
        return Err("`pokemon.json` does not exist. Please run the `fetch` subcommand.".into());
    }

    // open pokemon.json in read only mode
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    // try to parse the json into an array of pokemons struct
    let pokemon_data: Result<Vec<crate::structs::Pokemon>, serde_json::Error> =
        serde_json::from_reader(reader);

    match pokemon_data {
        Ok(_) => return Ok(()),
        Err(_) => {
            return Err(format!(
                "JSON structure is not correct. Please run the `fetch` subcommand."
            )
            .into())
        }
    }
}

fn validate_colorscripts_directory() -> Result<(), String> {
    let base_path: std::path::PathBuf = crate::constants::DATA_DIRECTORY.join("colorscripts");

    let subdirectories = ["big/regular", "big/shiny", "small/regular", "small/shiny"];

    for subdirectory in subdirectories.iter() {
        let path = base_path.join(subdirectory);
        if !path.exists() {
            return Err(format!(
                "Directory does not exist. Please run the `fetch` subcommand."
            ));
        }
    }

    return Ok(());
}
