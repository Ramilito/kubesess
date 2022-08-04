use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{BufRead, BufReader, BufWriter},
    path::Path,
};

#[derive(Default, Debug, Serialize, Deserialize)]
struct Contexts {
    context: Context,
    name: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Context {
    namespace: String,
    cluster: String,
    user: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    kind: String,
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "String::is_empty", default)]
    api_version: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    #[serde(rename = "current-context")]
    current_context: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    contexts: Vec<Contexts>,
}

pub fn create_file(ctx: &String, namespace: Option<&String>, temp_dir: &str) {
    let path = Path::new(ctx);
    let parent = path.parent().unwrap();
    let dirname = str::replace(&parent.display().to_string(), ":", "_");
    let filename = path.file_name().unwrap().to_str().unwrap();
    let _ = fs::create_dir_all(format!("{}/{}", temp_dir, dirname));

    let f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}/{}/{}", temp_dir, dirname, filename))
        .expect("Unable to open file");

    let mut reader = BufReader::new(&f);
    let mut strbuf = String::new();

    reader.read_line(&mut strbuf).expect("Unable to read file");

    let mut config: Config = serde_yaml::from_str(&strbuf).unwrap();
    config.api_version = "v1".to_string();
    config.kind = "Config".to_string();
    config.current_context = format!("{}", ctx);

    if let Some(ns) = namespace {
        config.contexts = vec![Contexts {
            context: Context {
                namespace: ns.to_string(),
                cluster: ctx.to_string(),
                user: ctx.to_string(),
            },
            name: ctx.to_string(),
        }];
    }

    let f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}/{}/{}", temp_dir, dirname, filename))
        .expect("Unable to open file");

    let writer = BufWriter::new(&f);
    serde_yaml::to_writer(writer, &config).unwrap();
}
