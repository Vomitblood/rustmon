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
        println!("Verbose: {verbose}");

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
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        } else {
            // list/forms
            match rustmon::list::print_pokemon_forms(pokemon_name) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {e}");
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
        let forms: Vec<&String> = print_args.get_many("form").unwrap().collect();
        let hide_name: bool = print_args.get_flag("hide-name");
        let names: Vec<&String> = print_args.get_many("name").unwrap().collect();
        let pokedexes: Vec<u16> = print_args.get_many("pokedex").unwrap().copied().collect();
        let shiny_rate: f32 = *print_args.get_one::<f32>("shiny").unwrap();
        let spacing: u8 = *print_args.get_one::<u8>("spacing").unwrap();

        // print
        rustmon::print::print(big, forms, hide_name, names, pokedexes, shiny_rate, spacing);
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
                        .default_value(rustmon::constants::DATA_DIRECTORY.to_str().unwrap()),
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
                .arg_required_else_help(true)
                // print/big
                .arg(
                    clap::Arg::new("big")
                        .help("Print a bigger version of the colorscript")
                        .short('b')
                        .long("big")
                        .action(clap::ArgAction::SetTrue),
                )
                // print/form
                .arg(
                    clap::Arg::new("form")
                        .help("Print Pokemon by list of space-separated forms. Follows the order of the names/Pokedex number specified. If not specified, it will print the regular form. Has no effect on random Pokemon.")
                        .short('f')
                        .long("form")
                        .default_value("regular")
                        .multiple_values(true)
                        .requires("name_or_pokedex"),
                    )
                // print/hide-name
                .arg(
                    clap::Arg::new("hide-name")
                        .help("Do not print Pokemon name")
                        .long("hide-name")
                        .action(clap::ArgAction::SetTrue),
                )
                // print/name
                .arg(
                    clap::Arg::new("name")
                        .help("Print Pokemon by list of space-separated names. Use `random` to print a random Pokemon.")
                        .short('n')
                        .long("name")
                        .default_value("")
                        .hide_default_value(true)
                        .multiple_values(true)
                        .conflicts_with("pokedex")
                )
                // print/pokedex
                .arg(
                    clap::Arg::new("pokedex")
                        .help("Print Pokemon by list of space-separated Pokedex numbers. Use `0` to print a random Pokemon.")
                        .short('p')
                        .long("pokedex")
                        // TODO: use a dynamic range instead of 0..906
                        // try not to hardcode?
                        .value_parser(clap::value_parser!(u16).range(0..906))
                        .default_value("0")
                        .hide_default_value(true)
                        .multiple_values(true)
                        .conflicts_with("name")
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
                        .default_value("0.00"),
                )
                // print/spacing
                .arg(
                    clap::Arg::new("spacing")
                        .help(
                            "Number of spaces between colorscripts",
                        )
                        .long("spacing")
                        .value_parser(clap::value_parser!(u8).range(0..21))
                        .default_value("4"),
                )
                .group(
                    clap::ArgGroup::new("name_or_pokedex")
                        .args(&["name", "pokedex"])
                        .required(false),
                )
        )
        .subcommand_required(true)
        // finalize
        .get_matches();
}
