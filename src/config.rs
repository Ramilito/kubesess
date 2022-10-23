use crate::model::{Config, Context, Contexts};

use std::env;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read};
use std::path::{Path, PathBuf};

fn build(ctx: &Contexts, ns: Option<&str>, strbuf: &str) -> Config {
    let mut config: Config = serde_yaml::from_str(&strbuf).unwrap();
    config.api_version = "v1".to_string();
    config.kind = "Config".to_string();
    config.current_context = format!("{}", ctx.name);

    let ns = if ns.is_some() {
        ns.unwrap().to_string()
    } else if config.contexts.len() > 0 && !config.contexts[0].context.namespace.is_empty() {
        config.contexts[0].context.namespace.to_string()
    } else if !ctx.context.namespace.is_empty() {
        ctx.context.namespace.to_string()
    } else {
        "default".to_string()
    };

    config.contexts = vec![Contexts {
        context: Context {
            namespace: ns.to_string(),
            cluster: ctx.context.cluster.to_string(),
            user: ctx.context.user.to_string(),
        },
        name: ctx.name.to_string(),
    }];

    config
}

fn get_file(path: &String) -> File {
    let f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Unable to open file");
    f
}

fn get_path(ctx: &Contexts, dest: &str) -> String {
    let path = Path::new(ctx.name.as_str());
    let parent = path.parent().unwrap();
    let dirname = str::replace(&parent.display().to_string(), ":", "_");

    fs::create_dir_all(format!("{}/{}", dest, dirname)).expect("Could not create destination dir");

    let path = Path::new(dest)
        .join(&dirname)
        .join(path.file_name().unwrap());
    path.display().to_string()
}

pub fn write(ctx: &Contexts, namespace: Option<&str>, dest: &str) {
    let path = get_path(ctx, dest);

    let strbuf = match fs::read_to_string(&path) {
        Ok(file) => file,
        Err(_error) => "".to_string(),
    };

    let options = get_file(&path);
    let writer = BufWriter::new(&options);
    let config = build(&ctx, namespace, &strbuf);

    serde_yaml::to_writer(writer, &config).unwrap();
}

pub fn get() -> Config {
    let p = env::var("KUBECONFIG").unwrap();
    let mut configs = Config::default();

    for s in p.split(":") {
        if s.contains("/kubesess/cache") {
            continue;
        }
        let f = File::open(s).unwrap();

        let mut reader = BufReader::new(f);
        let mut tmp = String::new();
        reader
            .read_to_string(&mut tmp)
            .expect("Unable to read file");

        let config: Config = serde_yaml::from_str(&tmp.trim()).unwrap();

        configs.contexts.extend(config.contexts);
    }

    configs
}

pub fn get_current_session() -> Config {
    let path = env::split_paths(&env::var_os("KUBECONFIG").unwrap())
        .next()
        .unwrap()
        .to_owned();

    let f = File::open(path).unwrap();

    let mut reader = BufReader::new(f);
    let mut string = String::new();

    reader
        .read_to_string(&mut string)
        .expect("Unable to read file");

    let config: Config = serde_yaml::from_str(&string.trim()).unwrap();

    config
}
