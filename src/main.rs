/*
# Arguments
## `fetch` - Fetch the latest colorscripts from the repository
- `extract_destination` - eXtract the colorscripts archive to a custom location
- `verbose` - Print colorscripts when generating

## `list` - Print a list of Pokemon names and Pokedex number
- `forms` - Print a list of forms of the specified Pokemon

## `print` - Print a Pokemon colorscript
- `big` - Print a bigger version of the colorscript
- `name` - Print Pokemon by name
- `no-title` - Do not print Pokemon name
- `pokedex` - Print Pokemon by Pokedex number
- `random` - Print a random Pokemon colorscript
- `shiny` - Print the shiny version of the colorscript
*/

/// Pokemon Colorscripts written in Rust
fn main() {
    let args = argument_parser();

    if let Some(fetch_args) = args.subcommand_matches("fetch") {
        // fetch
        let extract_destination_raw: &String =
            fetch_args.get_one::<String>("extract_destination").unwrap();
        let extract_destination: &std::path::Path = std::path::Path::new(extract_destination_raw);
        let verbose: bool = fetch_args.get_flag("verbose");

        // display selections
        println!("Extract destination: {}", extract_destination.display());
        println!("Verbose: {}", verbose);

        // invoke bigchungus fetch function
        rustmon::fetch::fetch(extract_destination, verbose)
    } else if let Some(list_args) = args.subcommand_matches("list") {
        let pokemon_name: &String = list_args.get_one::<String>("forms").unwrap();
        if pokemon_name.is_empty() {
            // list
            match rustmon::list::print_pokemon_list() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            // list/forms
            match rustmon::list::print_pokemon_forms(pokemon_name) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    } else if let Some(print_args) = args.subcommand_matches("print") {
        // print
        // declare and define variables from arguments
        let big: bool = print_args.get_flag("big");
        let id = print_args.get_one::<String>("id").unwrap();
        let name = print_args.get_one::<String>("name").unwrap();
        let no_title: bool = print_args.get_flag("no-title");
        let random: bool = print_args.get_flag("random");
        let shiny: bool = print_args.get_flag("shiny");

        println!("Big: {}", big);
        println!("ID: {}", id);
        println!("Name: {}", name);
        println!("No title: {}", no_title);
        println!("Random: {}", random);
        println!("Shiny: {}", shiny);

        rustmon::print::print();
    }
}

fn argument_parser() -> clap::ArgMatches {
    return clap::command!()
        // info
        .about("Pokemon Colorscripts written in Rust")
        .author("Vomitblood")
        // fetch subcommand
        .subcommand(
            clap::Command::new("fetch")
                .about("Fetch the latest colorscripts from the repository")
                // fetch/extract_destination
                .arg(
                    clap::Arg::new("extract_destination")
                        .help("eXtract the colorscripts archive to a custom location")
                        .short('x')
                        .long("extract-destination")
                        .default_value(&*rustmon::constants::DATA_DIRECTORY.to_str().unwrap()),
                )
                // fetch/verbose
                .arg(
                    clap::Arg::new("verbose")
                        .help("Print colorscripts when generating")
                        .short('v')
                        .long("verbose")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        // list subcommand
        .subcommand(
            clap::Command::new("list")
                .about("Print a list of Pokemon names and Pokedex number")
                // list/forms
                .arg(
                    clap::Arg::new("forms")
                        .help("Print a list of forms of the specified Pokemon")
                        .short('f')
                        .long("forms")
                        .default_value(""),
                )
                .after_help(
                    "Tip: Use `grep` to search for a specific Pokemon form!
Example: `rustmon list | grep 'pikachu'`
For more advanced usage, use `less` or `more` to scroll through the list!",
                ),
        )
        // print subcommand
        .subcommand(
            clap::Command::new("print")
                .about("Print a Pokemon colorscript")
                // print/big
                .arg(clap::arg!(-b --big "Print a bigger version of the colorscript"))
                // print/name
                .arg(
                    clap::Arg::new("name")
                        .help("Print Pokemon by name")
                        .short('n')
                        .long("name")
                        .conflicts_with("pokedex")
                        .conflicts_with("random"),
                )
                // print/random
                .arg(
                    clap::arg!(-r --random "Print a random Pokemon colorscript")
                        .conflicts_with("name"),
                )
                // print/no-title
                .arg(
                    clap::Arg::new("no-title")
                        .help("Do not print Pokemon name")
                        .long("no-title")
                        .action(clap::ArgAction::SetTrue),
                )
                // print/pokedex
                .arg(
                    clap::Arg::new("pokedex")
                        .help("Print Pokemon by Pokedex number")
                        .short('p')
                        .long("pokedex")
                        .conflicts_with("name")
                        .conflicts_with("random"),
                )
                // print/shiny
                .arg(clap::arg!(-s --shiny "Print the shiny version of the colorscript")),
        )
        // finalize
        .get_matches();
}
