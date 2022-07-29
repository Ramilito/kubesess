mod commands;

use std::{io, env};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    context: Option<String>,
}

fn main() -> Result<(), io::Error> {
    let arg = Cli::parse().context;
    let path = env::current_dir()?;
    let selection;

    if arg.is_some() {
        selection = arg.unwrap().to_string();
    } else {
        let contexts = commands::get_context();
        selection = commands::selectable_contexts(contexts);
    }

    commands::set_context(&selection);
    commands::use_context(&selection, &path.display().to_string());

    println!("{}/ctx/{}", path.display(), &selection);

    Ok(())
}
