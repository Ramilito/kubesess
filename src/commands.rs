use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::process::{Command, Stdio};

use crate::config;

pub fn get_context() -> Vec<String> {
    let output = Command::new("kubectl")
        .args(["config", "get-contexts", "-o", "name"])
        .output()
        .unwrap();

    let string = String::from_utf8(output.stdout).unwrap();
    string.lines().map(ToOwned::to_owned).collect()
}

pub fn get_namespaces() -> Vec<String> {
    let output = Command::new("kubectl")
        .args(["get", "namespace", "-o=custom-columns=Name:.metadata.name"])
        .output()
        .unwrap();

    let string = String::from_utf8(output.stdout).unwrap();
    string.lines().skip(1).map(ToOwned::to_owned).collect()
}

pub fn get_current_context() -> String {
    let output = Command::new("kubectl")
        .args(["config", "current-context"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

pub fn selectable_list(input: Vec<String>) -> String {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        // .with_prompt("Pick")
        .default(0)
        .items(&input[..])
        .interact()
        .unwrap();

    input[selection].to_string()
}

pub fn set_default_cotext(ctx: &String) {
    Command::new("kubectl")
        .args(["config", "use-context", ctx])
        .spawn()
        .unwrap()
        .wait().unwrap();
 }

pub fn set_namespace(ctx: &String, selection: &String, temp_dir: &str) {
    config::set(ctx, Some(selection), temp_dir)
}

pub fn set_context(ctx: &String, temp_dir: &str) {
    config::set(ctx, None, temp_dir)
}


