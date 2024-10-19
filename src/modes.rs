use crate::{
    commands::{self},
    config,
    error::Error,
    Cli, DEST, KUBECONFIG,
};

// pub fn default_context(args: Cli) -> Result<(), Error> {
//     let config = config::get();
//
//     if args.current {
//         println!("{}", config.current_context);
//         return Ok(());
//     }
//
//     let ctx = match args.value {
//         None => {
//             let options: Vec<String> = config
//                 .contexts
//                 .iter()
//                 .map(|context| context.name.to_string())
//                 .collect();
//
//             commands::selectable_list(options).ok_or(Error::NoItemSelected { prompt: "context" })?
//         }
//         Some(x) => x.trim().to_string(),
//     };
//
//     commands::set_default_context(&ctx);
//
//     let set_context_result = commands::set_context(&ctx, &DEST, &config).map_err(Error::SetContext);
//
//     if set_context_result.is_ok() {
//         println!("{}", KUBECONFIG.as_str());
//     }
//
//     set_context_result
// }

pub fn context(args: Cli) -> Result<(), Error> {
    if args.current {
        let config = config::get_current_session();
        println!(
            "{}",
            config
                .current_context
                .as_deref()
                .unwrap_or("No current context set")
        );
        return Ok(());
    }

    let config = config::get();
    let ctx = match args.value {
        None => {
            let options: Vec<String> = config
                .contexts
                .iter()
                .map(|context| context.name.to_string())
                .collect();

            commands::selectable_list(options).ok_or(Error::NoItemSelected { prompt: "context" })?
        }
        Some(x) => x.trim().to_string(),
    };

    let set_context_result = commands::set_context(&ctx, &DEST, &config).map_err(Error::SetContext);

    if set_context_result.is_ok() {
        println!(
            "{}/{}:{}",
            &DEST.as_str(),
            str::replace(&set_context_result.unwrap(), ":", "_"),
            *KUBECONFIG
        );
    }

    return Ok(());
}

pub fn namespace(args: Cli) -> Result<(), Error> {
    let config = config::get_current_session();
    let current_ctx = &config
        .current_context
        .as_deref()
        .unwrap_or("No current context set");
    if args.current {
        if let Some(ctx) = config.contexts.iter().find(|x| {
            x.name
                == config
                    .current_context
                    .as_deref()
                    .unwrap_or("No current context set")
        }) {
            let namespace = ctx
                .context
                .as_ref()
                .and_then(|c| c.namespace.as_deref())
                .unwrap_or("default");

            println!("{}", namespace);
        } else {
            println!("default");
        }
        return Ok(());
    }

    let ns = match args.value {
        None => {
            let namespaces: Vec<String> = commands::get_namespaces();
            commands::selectable_list(namespaces).ok_or(Error::NoItemSelected {
                prompt: "namespace",
            })?
        }
        Some(x) => x.trim().to_string(),
    };

    let result = commands::set_namespace(current_ctx, &ns, &DEST, &config);

    println!(
        "{}/{}:{}",
        &DEST.as_str(),
        str::replace(&result, ":", "_"),
        *KUBECONFIG
    );
    Ok(())
}
//
// pub fn default_namespace(args: Cli) -> Result<(), Error> {
//     let config = config::get();
//     let ctx = commands::get_current_context();
//
//     if args.current {
//         let ctx = config
//             .contexts
//             .iter()
//             .find(|x| x.name == config.current_context);
//
//         match ctx {
//             Some(x) => {
//                 if x.context.namespace.is_empty() {
//                     println!("default");
//                 } else {
//                     println!("{}", x.context.namespace);
//                 }
//             }
//             None => println!("default"),
//         }
//
//         return Ok(());
//     }
//
//     let ns = match args.value {
//         None => {
//             let namespaces: Vec<String> = commands::get_namespaces();
//             commands::selectable_list(namespaces).ok_or(Error::NoItemSelected {
//                 prompt: "namespace",
//             })?
//         }
//         Some(x) => x.trim().to_string(),
//     };
//
//     commands::set_default_namespace(&ns, &ctx);
//     commands::set_namespace(&ctx, &ns, &DEST, &config);
//     Ok(())
// }

pub fn completion_context(args: Cli) {
    let config = config::get();

    let search_value = args.value.as_deref().unwrap_or("");

    let options: Vec<String> = config
        .contexts
        .iter()
        .filter(|context| context.name.starts_with(search_value))
        .map(|context| context.name.clone())
        .collect();

    println!("{}", options.join(" "));
}

pub fn completion_namespace(args: Cli) {
    let namespaces = commands::get_namespaces();
    let mut options = Vec::new();

    let search_value = args.value.as_deref().unwrap_or("");

    for ns in &namespaces {
        if ns.starts_with(search_value) {
            options.push(ns.to_string());
        }
    }

    println!("{}", options.join(" "));
}
