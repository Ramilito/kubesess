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
    let mut context = Cli::parse().context;

    if !context.is_some() {
        let mut contexts = Command::new("kubectl")
            .args(["config", "get-contexts", "-o", "name"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let choice = Command::new("fzf")
            .args(["--ansi", "--no-preview"])
            .stdin(contexts.stdout.take().unwrap())
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?;

        context = Some(String::from_utf8(choice.stdout).unwrap().trim().to_owned());
    }

    let result = Command::new("kubectl")
        .args([
            "config",
            "use-context",
            // &context,
            &context.to_owned().unwrap()
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    let result_formated = String::from_utf8(result.stdout);

    println!("{}", result_formated.unwrap().trim().green());

    Ok(())
}
