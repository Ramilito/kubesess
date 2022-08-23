use crate::model::{Config, Context, Contexts};

use std::{
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter},
    path::Path,
};

fn build_config(ctx: &Contexts, ns: Option<&str>, strbuf: &str) -> Config {
    let mut config: Config = serde_yaml::from_str(&strbuf).unwrap();
    config.api_version = "v1".to_string();
    config.kind = "Config".to_string();
    config.current_context = format!("{}", ctx.name);

    config.contexts = vec![Contexts {
        context: Context {
            namespace: ns.unwrap_or("default").to_string(),
            cluster: ctx.context.cluster.to_string(),
            user: ctx.context.user.to_string(),
        },
        name: ctx.name.to_string(),
    }];

    config
}

fn get_config_file(ctx: &Contexts, dest: &str) -> File {
    let path = Path::new(ctx.name.as_str());
    let parent = path.parent().unwrap();
    let dirname = str::replace(&parent.display().to_string(), ":", "_");
    let _ = fs::create_dir_all(format!("{}/{}", dest, dirname));

    let f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!(
            "{}/{}/{}",
            dest,
            dirname,
            path.file_name().unwrap().to_string_lossy()
        ))
        .expect("Unable to open file");
    f
}

pub fn set(ctx: &Contexts, namespace: Option<&str>, dest: &str) {
    let strbuf = String::new();
    let options = get_config_file(&ctx, dest);
    let mut reader = BufReader::new(&options);

    reader
        .read_line(&mut strbuf.to_string())
        .expect("Unable to read file");

    let writer = BufWriter::new(&options);
    let config = build_config(&ctx, namespace, strbuf.as_str());

    serde_yaml::to_writer(writer, &config).unwrap();
}
