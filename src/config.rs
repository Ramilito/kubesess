use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
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

fn build_config(ctx: &str, namespace: Option<&str>, strbuf: &str) -> Config {
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

    config
}

fn get_config_file(ctx: &str, dest: &str) -> File {
    let path = Path::new(ctx);
    let parent = path.parent().unwrap();
    let dirname = str::replace(&parent.display().to_string(), ":", "_");
    let _ = fs::create_dir_all(format!("{}/{}", dest, dirname));

    let f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!(
            "{}/{}/{}",
            dest,
            dirname,
            path.file_name().unwrap().to_string_lossy()
        ))
        .expect("Unable to open file");
    f
}

pub fn set(ctx: &str, namespace: Option<&str>, dest: &str) {
    let strbuf = String::new();
    let options = get_config_file(ctx, dest);
    let mut reader = BufReader::new(&options);

    reader
        .read_line(&mut strbuf.to_string())
        .expect("Unable to read file");

    let writer = BufWriter::new(&options);
    let config = build_config(ctx, namespace, strbuf.as_str());

    serde_yaml::to_writer(writer, &config).unwrap();
}
