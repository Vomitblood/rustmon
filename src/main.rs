/*
# Arguments
## `fetch` - Fetch the latest colorscripts from the repository
- `extract_destination` - eXtract the colorscripts archive to a custom location
- `verbose` - Print colorscripts when generating

## `list` - Print a list of Pokemon names and Pokedex number
- `forms` - Print a list of forms of the specified Pokemon

## `print` - Print a Pokemon colorscript
- `big` - Print a bigger version of the colorscript
- `form` - Print Pokemon by list of space-separated forms. Follows the order of the names/Pokedex number specified. If not specified, it will print the regular form.
- `hide-name` - Do not print Pokemon name
- `name` - Print Pokemon by list of space-separated names. Use `random` to print a random Pokemon.
- `pokedex` - Print Pokemon by list of space-separated Pokedex numbers. Use `0` to print a random Pokemon.
- `shiny` - Rate of printing the shiny version of the colorscript
- `spacing` - Number of spaces between colorscripts

## `say` - Print a speaking Pokemon
- `text` - Input text for Pokemon to say
*/

/// Pokemon Colorscripts written in Rust
fn main() {
    let args = rustmon::args::argument_parser();

    if let Some(fetch_args) = args.subcommand_matches("fetch") {
        // fetch
        let extract_destination_raw: &String =
            fetch_args.get_one::<String>("extract_destination").unwrap();
        let extract_destination: &std::path::Path = std::path::Path::new(extract_destination_raw);
        let verbose: bool = fetch_args.get_flag("verbose");

        // display selections
        println!("Extract destination: {}", extract_destination.display());
        println!("Verbose: {verbose}");

        // invoke bigchungus fetch function
        rustmon::fetch::fetch(extract_destination, verbose);
    } else if let Some(list_args) = args.subcommand_matches("list") {
        // list

        // validate files first
        rustmon::validation::validate_files();

        let pokemon_name: &String = list_args.get_one::<String>("forms").unwrap();
        if pokemon_name.is_empty() {
            // list
            match rustmon::list::print_pokemon_list() {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            };
        } else {
            // list/forms
            match rustmon::list::print_pokemon_forms(pokemon_name) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            };
        }
    } else if let Some(print_args) = args.subcommand_matches("print") {
        // print

        // validate files first
        rustmon::validation::validate_files();

        // declare and define variables from arguments
        let big = print_args.get_flag("big");
        let forms: Vec<&String> = print_args.get_many("form").unwrap().collect();
        let hide_name: bool = print_args.get_flag("hide-name");
        let names: Vec<&String> = print_args.get_many("name").unwrap().collect();
        let pokedexes: Vec<u16> = print_args.get_many("pokedex").unwrap().copied().collect();
        let shiny_rate: f32 = *print_args.get_one::<f32>("shiny").unwrap();
        let spacing: u8 = *print_args.get_one::<u8>("spacing").unwrap();

        // print
        rustmon::print::print(big, forms, hide_name, names, pokedexes, shiny_rate, spacing);
    } else if let Some(say_args) = args.subcommand_matches("say") {
        // say

        // validate files first
        rustmon::validation::validate_files();

        let text: &String = say_args.get_one::<String>("text").unwrap();

        rustmon::say::say(text);
    }
}
