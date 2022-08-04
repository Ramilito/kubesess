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

fn build_config(ctx: &String, namespace: Option<&String>, strbuf: String) -> Config {
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

fn read_config(ctx: &String, dest: &str, mut strbuf: String) {
    let f = get_config_file(ctx, dest, None);
    let mut reader = BufReader::new(&f);

    reader.read_line(&mut strbuf).expect("Unable to read file");
}

fn write_config(
    ctx: &String,
    dest: &str,
    file_name: Option<String>,
    namespace: Option<&String>,
    strbuf: String,
) {
    let f = get_config_file(ctx, dest, file_name);
    let writer = BufWriter::new(&f);
    let config = build_config(ctx, namespace, strbuf);

    serde_yaml::to_writer(writer, &config).unwrap();
}

fn get_config_file(ctx: &String, dest: &str, file_name: Option<String>) -> File {
    let path = Path::new(ctx);
    let parent = path.parent().unwrap();
    let dirname = str::replace(&parent.display().to_string(), ":", "_");
    let destination = match file_name {
        Some(ref file_name) => {
            format!("{}/{}", dest, &file_name)
        }
        None => {
            format!(
                "{}/{}/{}",
                dest,
                dirname,
                path.file_name().unwrap().to_string_lossy()
            )
        }
    };

    let _ = fs::create_dir_all(format!("{}/{}", dest, dirname));

    let f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(file_name.is_some())
        .open(destination)
        .expect("Unable to open file");
    f
}

pub fn set(ctx: &String, namespace: Option<&String>, dest: &str) {
    let strbuf = String::new();

    read_config(ctx, dest, strbuf.to_owned());
    write_config(ctx, dest, None, namespace, strbuf.to_owned());
    write_config(
        ctx,
        dest,
        Some("config".to_string()),
        namespace,
        strbuf.to_owned(),
    );
}
