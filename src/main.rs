mod commands;

use clap::Parser;
use std::io;

#[derive(Parser)]
struct Cli {
    #[clap(value_enum)]
    mode: Mode,
    context: Option<String>,
}

#[derive(clap::ValueEnum, Clone)]
enum Mode {
    Namespace,
    Context,
}

fn main() -> Result<(), io::Error> {
    let arg = Cli::parse().context;
    let temp_dir = format!("{}/.cache/kubesess", dirs::home_dir().unwrap().display());

    let selection;

    if arg.is_some() {
        selection = arg.unwrap().to_string();
    } else {
        let contexts = commands::get_context();
        selection = commands::selectable_contexts(contexts);
    }
    commands::set_contextfile(&selection, &temp_dir);

    println!("{}/{}", &temp_dir, str::replace(&selection, ":", "_"));

    Ok(())
}
