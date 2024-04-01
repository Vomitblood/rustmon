use clap::Parser;

/*
# Arguments

## Fetch
- `fetch` - Fetch the latest colorscripts from the repository
- `silent` - Don't print colorscripts to the console when generating
- `extract_destination` - eXtract the colorscripts archive to a specified location

## Print
- `name` - Select pokemon by name
- `big` - Show a bigger version of the sprite
- `list` - Show a list of all pokemon names
- `no-title` - Do not display pokemon name
- `shiny` - Show the shiny version of the sprite
*/

/// Pokemon Colorscripts written in Rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // fetch
    /// Fetch the latest colorscripts from the repository
    #[arg(short, long, default_value_t = false)]
    fetch: bool,

    // silent
    /// Don't print colorscripts to the console when generating
    #[arg(long = "silent", default_value_t = false)]
    silent: bool,

    // extract destination
    /// eXtract the colorscripts archive to a specified location
    #[arg(short = 'x', long = "extract", default_value_t = String::from(""))]
    extract_destination: String,
    /*
    // big
    /// Show a bigger version of the sprite
    #[arg(short, long, default_value_t = false)]
    big: bool,

    // list
    /// Show a list of all pokemon names
    #[arg(short, long, default_value_t = false)]
    list: bool,

    // name
    /// Select pokemon by name
    #[arg(short = 'a', long, default_value_t = String::from(""))]
    name: String,

    // no-title
    // NOTE: clap will convert the kebab-case to snake_case
    // very smart!
    // ...but very annoying for beginners
    /// Do not display pokemon name
    #[arg(long, default_value_t = false)]
    no_title: bool,

    // shiny
    /// Show the shiny version of the sprite
    #[arg(short, long, default_value_t = false)]
    shiny: bool,
    */
}

fn main() {
    let args = argument_validation();

    if args.fetch == true {
        // get data directory
        let data_directory = match dirs::data_dir() {
            Some(dir) => dir.join("rustmon"),
            None => {
                println!("Data directory not found");
                std::process::exit(1);
            }
        };

        // decicde whether to use the default data directory or the one specified by the user
        // if the user specifies a directory, use that
        let extract_destination = if args.extract_destination.is_empty() {
            data_directory
        } else {
            std::path::PathBuf::from(&args.extract_destination)
        };

        rustmon::fetch::fetch(&extract_destination, args.silent);
    } else {
        println!("print deez nuts");
    }
}

fn argument_validation() -> Args {
    let args = Args::parse();

    return args;
}
