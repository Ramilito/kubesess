use crate::config;
use crate::error::SetContextError;
use crate::KUBECONFIG;

use std::{
    io::Cursor,
    path::Path,
    process::{Command, Stdio},
};
extern crate skim;
use kube::config::Kubeconfig;
use skim::prelude::*;

pub fn set_default_namespace(ns: &str, ctx: &str, target: &Path) {
    Command::new("kubectl")
        .arg("config")
        .arg(format!("--kubeconfig={}", target.to_string_lossy()))
        .arg("set-context")
        .arg(ctx)
        .arg(format!("--namespace={}", ns))
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn set_default_context(ctx: &str) {
    let output = Command::new("kubectl")
        .arg("config")
        .arg("use-context")
        .arg(ctx)
        .env("KUBECONFIG", KUBECONFIG.as_str())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn get_namespaces() -> Vec<String> {
    let output = Command::new("kubectl")
        .args(["get", "namespace", "-o=custom-columns=Name:.metadata.name"])
        .output()
        .unwrap();

    let string = String::from_utf8(output.stdout).unwrap();
    string.lines().skip(1).map(ToOwned::to_owned).collect()
}

/// Prompts the user to select an item from a list.
/// Returns the selected item or `None` if no item was selected
pub fn selectable_list(input: Vec<String>) -> Option<String> {
    let input: Vec<String> = input.into_iter().rev().collect();
    let options = SkimOptionsBuilder::default().multi(false).build().unwrap();
    let item_reader = SkimItemReader::default();

    let items = item_reader.of_bufread(Cursor::new(input.join("\n")));
    Skim::run_with(&options, Some(items))
        .and_then(|out| match out.final_key {
            Key::Enter => Some(out.selected_items),
            _ => None,
        })
        .filter(|selected_items| !selected_items.is_empty())
        .map(|selected_items| selected_items[0].output().to_string())
}

pub fn set_namespace(ctx: &str, selection: &str, temp_dir: &str, config: &Kubeconfig) -> String {
    let choice = config.contexts.iter().find(|x| x.name == ctx);
    config::write(choice.unwrap(), Some(selection), temp_dir, config)
}

pub fn set_context(
    ctx: &str,
    temp_dir: &str,
    config: &Kubeconfig,
) -> Result<String, SetContextError> {
    if let Some(choice) = config.contexts.iter().find(|x| x.name == ctx) {
        let filename = config::write(choice, None, temp_dir, config);
        Ok(filename)
    } else {
        Err(SetContextError::KubeContextNotFound {
            ctx: ctx.to_owned(),
        })
    }
}
