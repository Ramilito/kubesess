use crate::{KUBECONFIG, KUBESESSCONFIG};
use kube::config::Kubeconfig;
use kube::config::NamedContext;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;
use std::path::PathBuf;

pub struct KubeConfigs {
    pub config: Kubeconfig,
    pub configs: Vec<(Kubeconfig, PathBuf)>,
}

pub fn get(current_session: Option<&str>) -> KubeConfigs {
    use std::collections::HashSet;
    use std::path::PathBuf;

    let config_paths_str = match current_session {
        Some(path) if !path.is_empty() => format!("{}:{}", path, KUBECONFIG.clone()),
        Some(_) | None => KUBECONFIG.clone(),
    };

    let mut paths_set = HashSet::new();
    let config_paths: Vec<PathBuf> = config_paths_str
        .split(':')
        .filter(|path| !path.is_empty() && paths_set.insert(path.to_string()))
        .map(|path| PathBuf::from(path))
        .collect();

    let mut conifg = Kubeconfig::default();
    let mut configs = Vec::new();

    for path in config_paths {
        match Kubeconfig::read_from(&path) {
            Ok(kubeconfig) => {
                configs.push((kubeconfig.clone(), path.clone()));

                conifg.contexts.extend(kubeconfig.contexts.into_iter());
                conifg.clusters.extend(kubeconfig.clusters.into_iter());
                conifg.auth_infos.extend(kubeconfig.auth_infos.into_iter());

                if conifg.current_context.is_none() {
                    conifg.current_context = kubeconfig.current_context.clone();
                }
            }
            Err(err) => {
                eprintln!(
                    "Failed to load Kubeconfig from '{}': {}",
                    path.display(),
                    err
                );
            }
        }
    }

    KubeConfigs {
        config: conifg,
        configs,
    }
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
    filename
}

pub fn get_current_session() -> Kubeconfig {
    let current = if KUBESESSCONFIG.is_empty() {
        KUBECONFIG.split(':').next().unwrap()
    } else {
        KUBESESSCONFIG.as_str()
    };

    let configs = get(Some(current));

    configs.config
}
