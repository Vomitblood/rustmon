use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short = 'a', long, default_value_t = String::from(""))]
    name: String,

    // big
    /// Show a larger version of the sprite
    #[arg(short, long, default_value_t = false)]
    big: bool,

    // list
    /// Show a list of all pokemon names
    #[arg(short, long, default_value_t = false)]
    list: bool,

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
}

fn main() {
    let args = Args::parse();

    println!("no-title: {}", args.no_title);

    println!("name: {}", args.name);
}
