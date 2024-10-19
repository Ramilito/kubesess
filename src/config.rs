use kube::config::NamedContext;

use crate::model::KubeConfig;
use crate::{KUBECONFIG, KUBESESSCONFIG};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read};
use std::path::Path;

use dirs;
use kube::config::Kubeconfig;

pub fn get() -> Kubeconfig {
    let mut merged_config = Kubeconfig::default();

    let kube_dir = dirs::home_dir()
        .map(|home| home.join(".kube"))
        .expect("Failed to find home directory");

    if let Ok(entries) = fs::read_dir(&kube_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() {
                    match Kubeconfig::read_from(&path) {
                        Ok(kubeconfig) => {
                            println!("Loaded Kubeconfig from: {:?}", path);
                            merged_config.contexts.extend(kubeconfig.contexts);
                            merged_config.clusters.extend(kubeconfig.clusters);
                            merged_config.auth_infos.extend(kubeconfig.auth_infos);
                        }
                        Err(err) => {
                            println!("Failed to parse file {:?} as a Kubeconfig: {:?}", path, err);
                        }
                    }
                }
            }
        }
    }

    merged_config
}

pub fn build(
    selected_context: &NamedContext,
    namespace: Option<&str>,
    kubeconfig: &Kubeconfig,
) -> Kubeconfig {
    let context_name = &selected_context.name;

    // Handle the case where the context is None
    let context = match &selected_context.context {
        Some(ctx) => ctx,
        None => {
            eprintln!("Error: The selected context has no associated context data.");
            return Kubeconfig::default();
        }
    };

    // Find the corresponding cluster based on the context's cluster reference
    let cluster_name = &context.cluster;
    let selected_cluster = kubeconfig
        .clusters
        .iter()
        .find(|cluster| &cluster.name == cluster_name)
        .expect("Cluster for the selected context not found");

    // Find the corresponding auth_info (user) based on the context's user reference
    let user_name = &context.user;
    let selected_auth_info = kubeconfig
        .auth_infos
        .iter()
        .find(|auth_info| &auth_info.name == user_name)
        .expect("Auth info for the selected context not found");

    // Determine the namespace: use the provided one or fallback to the context's namespace
    let final_namespace = match namespace {
        Some(ns) => ns.to_string(),
        None => context
            .namespace
            .clone()
            .unwrap_or_else(|| "default".to_string()),
    };

    let mut minimal_context = selected_context.clone();
    if let Some(ref mut ctx) = minimal_context.context {
        ctx.namespace = Some(final_namespace);
    }

    Kubeconfig {
        current_context: Some(context_name.clone()),
        contexts: vec![minimal_context],
        clusters: vec![selected_cluster.clone()],
        auth_infos: vec![selected_auth_info.clone()],
        ..Kubeconfig::default()
    }
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

fn get_path(filename: &str, dest: &str) -> String {
    let path = Path::new(&filename);
    let parent = path.parent().unwrap();
    let dirname = str::replace(&parent.display().to_string(), ":", "_");

    fs::create_dir_all(format!("{}/{}", dest, dirname)).expect("Could not create destination dir");

    let path = Path::new(dest)
        .join(&dirname)
        .join(path.file_name().unwrap());
    path.display().to_string()
}

pub fn write(
    ctx: &NamedContext,
    namespace: Option<&str>,
    dest: &str,
    config: &Kubeconfig,
) -> String {
    let minimal_config = build(ctx, namespace, config);
    let selected_context = minimal_config.current_context.clone().unwrap_or_default();
    let selected_ns = minimal_config
        .contexts
        .first()
        .and_then(|ctx| ctx.context.as_ref().and_then(|c| c.namespace.clone()))
        .unwrap_or_else(|| "default".to_string());

    let filename = selected_context.to_owned() + "_" + &selected_ns.to_owned();
    let path = get_path(&filename, dest);

    let options = get_file(&path);
    let writer = BufWriter::new(&options);

    serde_yaml::to_writer(writer, &minimal_config).unwrap();
    return filename;
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
