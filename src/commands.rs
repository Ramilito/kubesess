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

pub fn set_namespace(ctx: &String, selection: &String, temp_dir: &str) {
    create_file(ctx, Some(selection), temp_dir)
}

pub fn set_context(ctx: &String, temp_dir: &str) {
    create_file(ctx, None, temp_dir)
}

fn create_file(ctx: &String, namespace: Option<&String>, temp_dir: &str) {
    let path = Path::new(ctx);
    let parent = path.parent().unwrap();
    let dirname = str::replace(&parent.display().to_string(), ":", "_");
    let filename = path.file_name().unwrap().to_str().unwrap();
    let _ = fs::create_dir_all(format!("{}/{}", temp_dir, dirname));

    let mut f = File::create(format!("{}/{}/{}", temp_dir, dirname, filename)).unwrap();

    let mut content = format!("apiVersion: v1\n");
    content.push_str(&format!("current-context: {}\n", ctx));
    content.push_str("kind: Config\n");
    content.push_str("contexts:\n");
    content.push_str("- context:\n");
    content.push_str(&format!("{:indent$}cluster: {}\n", "", ctx, indent = 4));
    if let Some(x) = namespace {
        content.push_str(&format!("{:indent$}namespace: {}\n", "", x, indent = 4));
    }
    content.push_str(&format!("{:indent$}user: {}\n", "", ctx, indent = 4));
    content.push_str(&format!("{:indent$}name: {}\n", "", ctx, indent = 2));

    f.write_all(content.as_bytes()).unwrap();
}
