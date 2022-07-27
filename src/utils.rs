use std::process::{Child, Command, Stdio};

pub fn get_context() -> Child {
    let contexts = Command::new("kubectl")
        .args(["config", "get-contexts", "-o", "name"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    contexts
}

pub fn selectable_contexts(mut contexts: Child) -> String {
    let output = Command::new("fzf")
        .args(["--ansi", "--no-preview"])
        .stdin(contexts.stdout.take().unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}
