pub fn argument_parser() -> clap::ArgMatches {
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
                        .default_value(crate::constants::DATA_DIRECTORY.to_str().unwrap()),
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
                        .value_delimiter(' ')
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
                        .value_delimiter(' ')
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
                        .value_delimiter(' ')
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
                        .args(["name", "pokedex"])
                        .required(false),
                )
        )
        // say subcommand
        .subcommand(
            clap::Command::new("say")
                .about("Print a speaking Pokemon")
                .after_help(
                    "Tip: Pipe the output to `rustmon say` to see the Pokemon speak!
Example: `echo \"Never gonna give you up\" | rustmon say`"
                )
                // say/text
                .arg(
                    clap::Arg::new("text")
                        .help("Input text for Pokemon to say")
                        .short('t')
                        .long("text")
                        .default_value("")
                        .hide_default_value(true)
                        .required(false)
                )
        )
        .subcommand_required(true)
        // finalize
        .get_matches();
}
