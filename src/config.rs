use crate::model::{Clusters, Context, Contexts, KubeConfig, Users};
use crate::{KUBECONFIG, KUBESESSCONFIG};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read};
use std::path::Path;

fn build(ctx: &Contexts, kube_config: &KubeConfig, ns: Option<&str>, strbuf: &str) -> KubeConfig {
    let mut config: KubeConfig = serde_yaml::from_str(strbuf).unwrap();
    config.api_version = "v1".to_string();
    config.kind = "Config".to_string();
    config.current_context = ctx.name.to_string();

    let ns = match ns {
        Some(namespace) => namespace.to_string(),
        None => {
            if !config.contexts.is_empty() && !config.contexts[0].context.namespace.is_empty() {
                config.contexts[0].context.namespace.to_string()
            } else if !ctx.context.namespace.is_empty() {
                ctx.context.namespace.to_string()
            } else {
                "default".to_string()
            }
        }
    };

    config.contexts = vec![Contexts {
        context: Context {
            namespace: ns,
            cluster: ctx.context.cluster.to_string(),
            user: ctx.context.user.to_string(),
        },
        name: ctx.name.to_string(),
    }];

    if let Some(user) = kube_config.users.iter().find(|x| x.name == ctx.context.user) {
        config.users = vec![Users {
            name: user.name.clone(),
            user: user.user.clone(),
        }];
    }

    if let Some(cluster) = kube_config.clusters.iter().find(|x| x.name == ctx.context.cluster) {
        config.clusters = vec![Clusters {
            name: cluster.name.clone(),
            cluster: cluster.cluster.clone(),
        }];
    }

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

pub fn write(ctx: &Contexts, config: &KubeConfig, namespace: Option<&str>, dest: &str) {
    let path = get_path(ctx, dest);

    let strbuf = match fs::read_to_string(&path) {
        Ok(file) => file,
        Err(_error) => "".to_string(),
    };

    let options = get_file(&path);
    let writer = BufWriter::new(&options);
    let config = build(ctx, config, namespace, &strbuf);

    serde_yaml::to_writer(writer, &config).unwrap();
}

pub fn get() -> KubeConfig {
    let mut configs = KubeConfig::default();

    for s in KUBECONFIG.rsplit(':') {
        if s.contains("/kubesess/cache") {
            continue;
        }
        let config: KubeConfig = get_config(s);

        configs.current_context = config.current_context;
        configs.api_version = config.api_version;
        configs.kind = config.kind;
        configs.contexts.extend(config.contexts);
        configs.users.extend(config.users);
        configs.clusters.extend(config.clusters);
    }

    let dir = format!("{}/.kube", dirs::home_dir().unwrap().display());
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "yaml" {
                let config: KubeConfig = get_config(path.to_str().unwrap());

                configs.contexts.extend(config.contexts);
                configs.users.extend(config.users);
                configs.clusters.extend(config.clusters);
            }
        }
    }

    configs
}

fn get_config(path: &str) -> KubeConfig {
    let f = File::open(path).unwrap();

    let mut reader = BufReader::new(f);
    let mut tmp = String::new();
    reader
        .read_to_string(&mut tmp)
        .expect("Unable to read file");

    let config: KubeConfig = serde_yaml::from_str(tmp.trim()).unwrap();

    config
}

pub fn get_current_session() -> KubeConfig {
    let current = if KUBESESSCONFIG.is_empty() {
        KUBECONFIG.split(':').next().unwrap()
    } else {
        KUBESESSCONFIG.as_str()
    };

    let f = File::open(current).unwrap();

    let mut reader = BufReader::new(f);
    let mut tmp = String::new();
    reader
        .read_to_string(&mut tmp)
        .expect("Unable to read file");

    let config = serde_yaml::from_str(tmp.trim()).unwrap();

    config
}
