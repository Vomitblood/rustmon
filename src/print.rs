use rand::prelude::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use std::io::BufRead;
use std::io::Read;

pub fn print(
    big: bool,
    forms: Vec<&String>,
    hide_name: bool,
    names: Vec<&String>,
    pokedexes: Vec<u16>,
    shiny_rate: f32,
    spacing: u8,
) {
    // decide which function to call
    if !big
        // uber fast random
        && forms.len() == 1
        && !hide_name
        && (names.len() == 1 && pokedexes.len() == 1)
        && shiny_rate == 0.0
        && forms[0] == "regular"
        && (names[0] == "random" && pokedexes[0] == 0)
    {
        random_lite().unwrap();
    } else {
        // convert list of names to list of pokedex numbers
        let pokedexes = if names[0].is_empty() {
            pokedexes.clone()
        } else {
            let mut pokedexes: Vec<u16> = Vec::new();
            for name in names {
                match find_pokedex_by_pokemon(name) {
                    Ok(pokedex) => pokedexes.push(pokedex.parse().unwrap()),
                    Err(e) => {
                        println!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            pokedexes
        };

        // process the pokedexes list
        // iterate through the pokedexes list, if value is 0, then generate a random number between 1 and 905
        // if the value is not 0, then use the value as is
        let pokedexes = process_pokedexes_list(pokedexes);

        // process the forms list
        // the length of the forms list should be the same as the pokedexes list, resize with `regular` if different length
        // if the form is not available for the pokemon then print the available forms and exit
        let forms = process_forms_list(&pokedexes, forms);

        // generate a list of slugs
        let slugs = generate_slug_list(big, forms, &pokedexes, shiny_rate);

        // if hide_name is false then print the names of the slugs, separated by comma
        if !hide_name {
            print_name(&slugs);
        }

        // print the actual thing
        print_colorscripts(&slugs, spacing).unwrap();
    }
}

fn random_lite() -> std::io::Result<()> {
    let path = crate::constants::DATA_DIRECTORY.join("colorscripts/small/regular/");
    let mut files: Vec<std::path::PathBuf> = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let dir_entry = entry?;
        files.push(dir_entry.path());
    }

    let mut rng = rand::rngs::SmallRng::from_entropy();
    if let Some(random_file) = files.choose(&mut rng) {
        if let Some(file_name) = random_file.file_name() {
            match file_name.to_str() {
                Some(name) => println!("{}", name),
                None => println!("Invalid UTF-8 sequence in file name"),
            }
        }

        match std::fs::read_to_string(random_file) {
            Ok(file_data) => {
                println!("{}", file_data);
                Ok(())
            }
            Err(e) => {
                println!("Failed to read file contents: {}", e);
                Err(e)
            }
        }
    } else {
        println!("No files found in the directory");
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No files available",
        ))
    }
}

fn get_pokemon_data(pokedex_number: u16) -> crate::structs::Pokemon {
    // read the file
    let mut file =
        std::fs::File::open(crate::constants::DATA_DIRECTORY.join("pokemon.json")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // deserialize into the struct
    let pokemons: Vec<crate::structs::Pokemon> = serde_json::from_str(&contents).unwrap();

    // get the pokemon data
    // remember that the pokedex number is 1-indexed
    // gawdamn it
    let pokemon_data: crate::structs::Pokemon =
        pokemons.get(pokedex_number as usize - 1).unwrap().clone();

    // return the data
    pokemon_data
}

fn find_pokedex_by_pokemon(pokemon_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    // handle random
    if pokemon_name == "random" {
        Ok("0".to_string())
    } else {
        // read the file
        let mut file = std::fs::File::open(crate::constants::DATA_DIRECTORY.join("pokemon.json"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // deserialize into the struct
        let pokemons: Vec<crate::structs::Pokemon> = serde_json::from_str(&contents)?;

        // iterate through the list to find the specified pokemon
        for pokemon in pokemons {
            if pokemon.name == pokemon_name {
                // if found then return the pokedex number
                return Ok(pokemon.pokedex);
            }
        }

        // if not found the return an error
        Err(format!("Pokemon {} not found", pokemon_name).into())
    }
}

fn is_shiny(shiny_rate: f32) -> bool {
    // generate a random number between 0 and 1
    let random_number = rand::random::<f32>();

    // if the random number is less than the shiny rate then return true
    random_number < shiny_rate
}

fn process_pokedexes_list(pokedexes: Vec<u16>) -> Vec<u16> {
    let mut pokedexes_processed: Vec<u16> = pokedexes.clone();

    for i in 0..pokedexes.len() {
        if pokedexes[i] == 0 {
            let random_pokedex = rand::thread_rng().gen_range(1..906);
            pokedexes_processed[i] = random_pokedex;
        }
    }

    pokedexes_processed
}

fn process_forms_list(pokedexes: &[u16], forms: Vec<&String>) -> Vec<String> {
    let mut forms_processed: Vec<String> = forms.iter().map(|s| s.to_string()).collect();

    // ensure forms_processed has the same length as pokedexes
    forms_processed.resize_with(pokedexes.len(), || "regular".to_string());

    for i in 0..pokedexes.len() {
        let pokemon = get_pokemon_data(pokedexes[i]);
        let form = &forms_processed[i];

        if !pokemon.forms.contains(&form.to_string()) {
            // iterate and print out the available forms
            // consider using crate::list::print_pokemon_forms(pokemon_name)
            println!("Available forms for {}:", pokemon.name);
            for available_form in &pokemon.forms {
                println!(" - {available_form}");
            }
            std::process::exit(1);
        }
    }

    forms_processed
}

fn slug_generator(big: bool, form: String, name: String, shiny_rate: f32) -> std::path::PathBuf {
    // big is just a boolean, convert it to big or small
    // form is a string, if `regular` then replace with empty string. else keep it as is.
    // name is a string, should be cleaned up already. there should be no `random` as a name should be generated before this.
    // shiny_rate is a float, let it be

    // if big is true then use big, else use small
    let big: String = if big {
        "big".to_string()
    } else {
        "small".to_string()
    };

    // if form is regular then replace with empty string
    let form: String = if form == "regular" {
        "".to_string()
    } else {
        format!("-{}", form)
    };

    // determine if shiny directory is to be used
    let shiny_directory: String = if is_shiny(shiny_rate) {
        "shiny".to_string()
    } else {
        "regular".to_string()
    };

    // construct the path using PathBuf
    let mut path = std::path::PathBuf::new();
    path.push(crate::constants::DATA_DIRECTORY.join("colorscripts"));
    path.push(&big);
    path.push(shiny_directory);
    path.push(format!("{name}{form}"));

    path
}

fn generate_slug_list(
    big: bool,
    forms: Vec<String>,
    pokedexes: &[u16],
    shiny_rate: f32,
) -> Vec<std::path::PathBuf> {
    let mut slugs: Vec<std::path::PathBuf> = Vec::new();

    // iterate through the pokedexes to generate the slugs with the complementing form
    for i in 0..pokedexes.len() {
        let pokemon = get_pokemon_data(pokedexes[i]);
        let form = &forms[i];

        let slug = slug_generator(big, form.to_string(), pokemon.name, shiny_rate);
        slugs.push(slug);
    }

    slugs
}

fn print_name(paths: &[std::path::PathBuf]) {
    let last_parts: Vec<&str> = paths
        .iter()
        .filter_map(|path| path.file_name())
        .filter_map(|os_str| os_str.to_str())
        .collect();

    let output = last_parts.join(", ");
    println!("{}", output);
}

// I HATE ANSI ESCAPE CHARACTERS
fn print_colorscripts(
    paths: &Vec<std::path::PathBuf>,
    spacing: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut max_widths = vec![];
    let mut max_height = 0;
    let mut file_contents: Vec<Vec<String>> = vec![];

    // first read all files and calculate maximum widths and heights by iterating through
    // MUST IGNORE ANSI ESCAPE CODES❗❗❗
    for path in paths {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        // put all the lines in a vector
        let mut lines: Vec<String> = vec![];
        let mut max_width = 0;

        // get the longest line length
        for line in reader.lines() {
            let line = line?;
            // remove ansi escape codes for width calculation
            let plain_line = regex::Regex::new("\x1b\\[[^m]*m")?.replace_all(&line, "");
            max_width = max_width.max(plain_line.chars().count());
            lines.push(line);
        }

        // push the max width and lines to the respective vectors
        max_widths.push(max_width);
        max_height = max_height.max(lines.len());
        file_contents.push(lines);
    }

    // construct spacing string
    let separator = " ".repeat(spacing as usize);

    // print each combined line
    for line_index in 0..max_height {
        let mut line_to_print = String::new();

        // construct the combined line
        for (file_index, lines) in file_contents.iter().enumerate() {
            if line_index < lines.len() {
                line_to_print.push_str(&lines[line_index]);
            }

            // pad the rest of the line if this artwork is shorter
            if line_index >= lines.len()
                || line_index < lines.len()
                    && lines[line_index].chars().count() < max_widths[file_index]
            {
                let current_length = lines.get(line_index).map_or(0, |l| l.chars().count());
                line_to_print.push_str(&" ".repeat(max_widths[file_index] - current_length));
            }

            // check if this is the last file to print
            if file_index < file_contents.len() - 1 {
                // do not add separator to the last file
                line_to_print.push_str(&separator);
            }
        }

        // finally print the thing
        println!("{}", line_to_print);
    }

    // reset terminal color by printing a reset code
    println!("\x1b[0m");

    Ok(())
}
