mod commands;

use std::io;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    context: Option<String>,
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
