use crate::{commands, config, Cli, DEST, KUBECONFIG};

fn selection(value: Option<String>, callback: fn() -> String) -> String {
    match value {
        None => callback(),
        Some(x) => x.trim().to_string(),
    }
}

pub fn default_context(args: Cli) {
    let config = config::get();

    if args.current {
        println!("{}", config.current_context);
        return;
    }

    let ctx = match args.value {
        None => {
            let mut options = Vec::new();
            for context in &config.contexts {
                options.push(context.name.to_string());
            }

            commands::selectable_list(options)
        }
        Some(x) => x.trim().to_string(),
    };

    commands::set_default_context(&ctx);
    commands::set_context(&ctx, &DEST, &config);

    println!("{}", KUBECONFIG.as_str());
}

pub fn context(args: Cli) {
    if args.current {
        let config = config::get_current_session();
        println!("{}", config.current_context);
        return;
    }

    let config = config::get();
    let ctx = match args.value {
        None => {
            let mut options = Vec::new();
            for context in &config.contexts {
                options.push(context.name.to_string());
            }

            commands::selectable_list(options)
        }
        Some(x) => x.trim().to_string(),
    };

    commands::set_context(&ctx, &DEST, &config);

    println!(
        "{}/{}:{}",
        &DEST.as_str(),
        str::replace(&ctx, ":", "_"),
        KUBECONFIG.as_str()
    );
}

pub fn namespace(args: Cli) {
    let config = config::get_current_session();
    if args.current {
        let ctx = config
            .contexts
            .iter()
            .find(|x| x.name == config.current_context);

        match ctx {
            Some(x) => {
                if x.context.namespace.is_empty() {
                    println!("default");
                } else {
                    println!("{}", x.context.namespace);
                }
            }
            None => println!("default"),
        }
        return;
    }

    let ns = selection(args.value, || -> String {
        let namespaces = commands::get_namespaces();
        commands::selectable_list(namespaces)
    });

    commands::set_namespace(&config.current_context, &ns, &DEST, &config);

    println!(
        "{}/{}:{}",
        &DEST.as_str(),
        str::replace(&config.current_context, ":", "_"),
        KUBECONFIG.as_str()
    );
}

pub fn default_namespace(args: Cli) {
    let config = config::get();
    let ctx = commands::get_current_context();

    if args.current {
        let ctx = config
            .contexts
            .iter()
            .find(|x| x.name == config.current_context);

        match ctx {
            Some(x) => {
                if x.context.namespace.is_empty() {
                    println!("default");
                } else {
                    println!("{}", x.context.namespace);
                }
            }
            None => println!("default"),
        }

        return;
    }

    let ns = selection(args.value, || -> String {
        let namespaces = commands::get_namespaces();
        commands::selectable_list(namespaces)
    });

    commands::set_default_namespace(&ns, &ctx);
    commands::set_namespace(&ctx, &ns, &DEST, &config);
}

pub fn completion_context(args: Cli) {
    let config = config::get();

    let mut options = Vec::new();
    for context in &config.contexts {
        if context
            .name
            .starts_with(&args.value.as_ref().unwrap().to_string())
        {
            options.push(context.name.to_string());
        }
    }
    println!("{}", options.join(" "));
}

pub fn completion_namespace(args: Cli) {
    let namespaces = commands::get_namespaces();
    let mut options = Vec::new();
    for ns in &namespaces {
        if ns.starts_with(&args.value.as_ref().unwrap().to_string()) {
            options.push(ns.to_string());
        }
    }
    println!("{}", options.join(" "));
}
