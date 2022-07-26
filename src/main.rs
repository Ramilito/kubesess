use std::{
    process::{Command, Stdio}, io,
};

// use crossterm::{cursor, execute, terminal::EnterAlternateScreen};
// use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
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

    let result = Command::new("kubectl")
        .args([
            "config",
            "use-context",
            String::from_utf8(choice.stdout).unwrap().trim(),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?
        .wait()?;

    println!("{}", result);

    Ok(())
}
