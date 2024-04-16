/*
# Arguments
## `fetch` - Fetch the latest colorscripts from the repository
- `extract_destination` - eXtract the colorscripts archive to a custom location
- `verbose` - Print colorscripts when generating

## `print` - Print a Pokemon colorscript
- `big` - Print a bigger version of the colorscript
- `id` - Print Pokemon by ID
- `list` - Print a list of all Pokemon names
- `name` - Print Pokemon by name
- `no-title` - Do not print Pokemon name
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
    } else if let Some(print_args) = args.subcommand_matches("print") {
        // print
        if let Some(name) = print_args.get_one::<String>("name") {
            // print/name
            println!("name: {}", name);
        }
        if print_args.get_flag("big") {
            // print/big
            println!("big");
        }
        println!("something else");
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
                        .help("Select Pokemon by name")
                        .short('v')
                        .long("verbose")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        // print subcommand
        .subcommand(
            clap::Command::new("print")
                .about("Print a Pokemon colorscript")
                // print/big
                .arg(
                    clap::arg!(-b --big "Print a bigger version of the colorscript")
                        .conflicts_with("list"),
                )
                // print/list
                .arg(
                    clap::arg!(-l --list "Print a list of all Pokemon names")
                        .conflicts_with("name")
                        .conflicts_with("random"),
                )
                // print/name
                .arg(
                    clap::Arg::new("name")
                        .help("Select Pokemon by name")
                        .short('n')
                        .long("name")
                        .conflicts_with("list")
                        .conflicts_with("random"),
                )
                // print/random
                .arg(
                    clap::arg!(-r --random "Print a random Pokemon colorscript")
                        .conflicts_with("list")
                        .conflicts_with("name"),
                )
                // print/no-title
                .arg(
                    clap::Arg::new("no-title")
                        .help("Do not print Pokemon name")
                        .long("no-title")
                        .action(clap::ArgAction::SetTrue)
                        .conflicts_with("list"),
                )
                // print/shiny
                .arg(
                    clap::arg!(-s --shiny "Print the shiny version of the colorscript")
                        .conflicts_with("list"),
                ),
        )
        // finalize
        .get_matches();
}
