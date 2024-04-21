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
- `shiny` - Rate of printing the shiny version of the colorscript
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
        // list

        // validate files first
        rustmon::validation::validate_files();

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

        // validate files first
        rustmon::validation::validate_files();

        // declare and define variables from arguments
        let big = print_args.get_flag("big");
        let pokedex: u16 = *print_args.get_one::<u16>("pokedex").unwrap();
        let name: &String = print_args.get_one::<String>("name").unwrap();
        let no_title: bool = print_args.get_flag("no-title");
        let random: bool = print_args.get_flag("random");
        let shiny: f32 = *print_args.get_one::<f32>("shiny").unwrap();

        rustmon::print::print(big, pokedex, name, no_title, random, shiny);
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
                        .default_value("")
                        .hide_default_value(true),
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
                .arg(
                    clap::Arg::new("big")
                        .help("Print a bigger version of the colorscript")
                        .short('b')
                        .long("big")
                        .action(clap::ArgAction::SetTrue),
                )
                // print/name
                .arg(
                    clap::Arg::new("name")
                        .help("Print Pokemon by name")
                        .short('n')
                        .long("name")
                        .default_value("")
                        .hide_default_value(true)
                        .conflicts_with("pokedex")
                        .conflicts_with("random"),
                )
                // print/no-title
                .arg(
                    clap::Arg::new("no-title")
                        .help("Do not print Pokemon name")
                        .long("no-title")
                        .action(clap::ArgAction::SetTrue),
                )
                // print/random
                .arg(
                    clap::Arg::new("random")
                        .help("Print a random Pokemon colorscript")
                        .short('r')
                        .long("random")
                        .action(clap::ArgAction::SetTrue),
                )
                // print/pokedex
                .arg(
                    clap::Arg::new("pokedex")
                        .help("Print Pokemon by Pokedex number")
                        .short('p')
                        .long("pokedex")
                        .value_parser(clap::value_parser!(u16).range(0..))
                        .default_value("0")
                        .hide_default_value(true)
                        .conflicts_with("name")
                        .conflicts_with("random"),
                )
                // print/shiny
                .arg(
                    clap::Arg::new("shiny")
                        .help(
                            "Rate of printing the shiny version of the colorscript (e.g. 0.10 for 10% chance)",
                        )
                        .short('s')
                        .long("shiny")
                        .value_parser(clap::value_parser!(f32))
                        .default_value("0.10"),
                ),
        )
        // finalize
        .get_matches();
}
