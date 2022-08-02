use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

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

pub fn set_namespace(current_ctx: &String, selection: &String, temp_dir: &str) {
    create_file(current_ctx, Some(selection), temp_dir)
}

pub fn set_context(selection: &String, temp_dir: &str) {
    create_file(selection, None, temp_dir)
}

fn create_file(context: &String, namespace: Option<&String>, temp_dir: &str) {
    let path = Path::new(context);
    let parent = path.parent().unwrap();
    let dirname = str::replace(&parent.display().to_string(), ":", "_");
    let filename = path.file_name().unwrap().to_str().unwrap();
    let _ = fs::create_dir_all(format!("{}/{}", temp_dir, dirname));

    let mut f = File::create(format!("{}/{}/{}", temp_dir, dirname, filename)).unwrap();

    write!(f, "apiVersion: v1\n").unwrap();
    write!(f, "current-context: {}\n", context).unwrap();
    write!(f, "kind: Config\n").unwrap();
    write!(f, "contexts:\n").unwrap();
    write!(f, "- context:\n").unwrap();
    write!(f, "{:indent$}cluster: {}\n", "", context, indent = 4).unwrap();
    if let Some(x) = namespace {
        write!(f, "{:indent$}namespace: {}\n", "", x, indent = 4).unwrap();
    }
    write!(f, "{:indent$}user: {}\n", "", context, indent = 4).unwrap();
    write!(f, "{:indent$}name: {}\n", "", context, indent = 2).unwrap();
}
