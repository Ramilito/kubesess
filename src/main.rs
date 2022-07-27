mod commands;

use clap::Parser;
use crossterm::style::Stylize;
use std::io;

#[derive(Parser)]
struct Cli {
    context: Option<String>,
}

fn main() -> Result<(), io::Error> {
    let arg = Cli::parse().context;
    let selection;

    if arg.is_some() {
        selection = arg.unwrap().to_string();
    } else {
        let contexts = commands::get_context();
        selection = commands::selectable_contexts(contexts);
    }

    let result = commands::select_context(selection);
    println!("{}", result.trim().green());

    Ok(())
}
