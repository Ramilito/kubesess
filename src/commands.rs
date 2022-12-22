use crate::model::Config;
use crate::config;

use std::process;
use std::{
    io::Cursor,
    process::{Command, Stdio},
};
extern crate skim;
use skim::prelude::*;

pub fn set_default_namespace(ns: &str, ctx: &str) {
    Command::new("kubectl")
        .arg("config")
        .arg(format!(
            "--kubeconfig={}/.kube/config",
            dirs::home_dir().unwrap().display().to_string()
        ))
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
    Command::new("kubectl")
        .arg("config")
        .arg(format!(
            "--kubeconfig={}/.kube/config",
            dirs::home_dir().unwrap().display().to_string()
        ))
        .arg("use-context")
        .arg(ctx)
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
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
    let input: Vec<String> = input.into_iter().rev().collect();
    let options = SkimOptionsBuilder::default().multi(false).build().unwrap();
    let item_reader = SkimItemReader::default();

    let items = item_reader.of_bufread(Cursor::new(input.join("\n")));
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| match out.final_key {
            Key::Enter => out.selected_items,
            _ => Vec::new(),
        })
        .unwrap_or_default();

    if selected_items.is_empty() {
        eprintln!("No item selected");
        process::exit(1);
    }

    selected_items[0].output().to_string()
}

pub fn set_namespace(ctx: &str, selection: &str, temp_dir: &str, config: &Config) {
    let choice = config.contexts.iter().find(|x| x.name == ctx);
    config::write(choice.unwrap(), Some(selection), temp_dir)
}

pub fn set_context(ctx: &str, temp_dir: &str, config: &Config) {
    let choice = config.contexts.iter().find(|x| x.name == ctx);
    config::write(choice.unwrap(), None, temp_dir);
}
