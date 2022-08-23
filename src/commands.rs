use crate::config;
use crate::model::Config;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::process::{Command, Stdio};

pub fn set_default_namespace(ns: &str) {
    Command::new("kubectl")
        .arg("config")
        .arg(format!(
            "--kubeconfig={}/.kube/config",
            dirs::home_dir().unwrap().display().to_string()
        ))
        .arg("set-context")
        .arg("--current")
        .arg(format!("--namespace={}", ns))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn set_default_context(ctx: &str) {
    Command::new("kubectl")
        .arg("config")
        .arg(format!(
            "--kubeconfig={}/.kube/config",
            dirs::home_dir().unwrap().display().to_string()
        ))
        .arg("use-context")
        .arg(ctx)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn get_config() -> Config {
    let output = Command::new("kubectl")
        .args(["config", "view", "-o", "json"])
        .output()
        .unwrap();

    let string = String::from_utf8(output.stdout).unwrap();
    let config: Config = serde_json::from_str(&string.trim()).unwrap();

    config
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

pub fn set_namespace(ctx: &str, selection: &str, temp_dir: &str, config: &Config) {
    let choice = config.contexts.iter().find(|x| x.name == ctx);
    config::set(choice.unwrap(), Some(selection), temp_dir)
}

pub fn set_context(ctx: &str, temp_dir: &str, config: &Config) {
    let choice = config.contexts.iter().find(|x| x.name == ctx);

    config::set(choice.unwrap(), None, temp_dir);
}
