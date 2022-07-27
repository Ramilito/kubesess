mod utils;

use clap::Parser;
use crossterm::style::Stylize;
use std::{
    io,
    process::{Command, Stdio},
};

#[derive(Parser)]
struct Cli {
    context: Option<String>,
}

fn main() -> Result<(), io::Error> {
    let arg = Cli::parse().context;
    let mut context = String::new();

    if arg.is_some() {
        context = arg.unwrap().to_string();
    } else {
        let contexts = utils::get_context();
        context = utils::selectable_contexts(contexts);
    }

    let result = Command::new("kubectl")
        .args(["config", "use-context", &context])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    let result_formated = String::from_utf8(result.stdout);

    println!("{}", result_formated.unwrap().trim().green());

    Ok(())
}
