use std::process::{Child, Command, Stdio};

pub fn get_context() -> Child {
    Command::new("kubectl")
        .args(["config", "get-contexts", "-o", "name"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
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

pub fn set_context(selection: &String) -> String {
    let result = Command::new("kubectl")
        .arg("config")
        .arg("--kubeconfig")
        .arg(format!("ctx/{}", &selection))
        .arg("set-context")
        .arg(&selection)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    String::from_utf8(result.stdout).unwrap()
}

pub fn use_context(selection: &String, path: &String) -> String {
    let result = Command::new("kubectl")
        .arg("config")
        .arg("--kubeconfig")
        .arg(format!("{}/ctx/{}", &path, &selection))
        .arg("use-context")
        .arg(&selection)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    String::from_utf8(result.stdout).unwrap()
}
