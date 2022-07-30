use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
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

pub fn set_contextfile(selection: &String) {
    let path = Path::new(selection);
    let parent = path.parent().unwrap();
    let dirname = str::replace(&parent.display().to_string(), ":", "_");
    let filename = path.file_name().unwrap().to_str().unwrap();

    let _ = fs::create_dir(format!("ctx/{}", dirname));

    let mut f = File::create(format!("ctx/{}/{}", dirname, filename)).unwrap();

    write!(f, "apiVersion: v1\n").unwrap();
    write!(f, "current-context: {}\n", selection).unwrap();
    write!(f, "kind: Config\n").unwrap();
}
