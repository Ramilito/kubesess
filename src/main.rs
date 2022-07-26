use std::{ process::{Command, Stdio}, io };

fn main() -> Result<(), io::Error> {
    let mut kubectx = Command::new("kubectl")
        .args(["config", "get-contexts", "-o", "name"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = Command::new("fzf")
        .args(["--ansi", "--no-preview"])
        .stdin(kubectx.stdout.take().unwrap())
        .stdout(Stdio::piped())
        .spawn()?;

    let output = stdout.wait_with_output()?;
    let result = String::from_utf8(output.stdout).unwrap();

    Command::new("kubectl")
        .args(["config", "use-context", &result.trim()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    println!("{}", result);

    Ok(())
}
