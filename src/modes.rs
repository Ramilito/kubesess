use crate::{commands::{self}, config, Cli, DEST, KUBECONFIG};

pub fn default_context(args: Cli) -> Result<(), String> {
    let config = config::get();

    if args.current {
        println!("{}", config.current_context);
        return Ok(());
    }

    let ctx = match args.value {
        None => {
            let options : Vec<String> = config.contexts.iter()
                .map(|context| context.name.to_string())
                .collect();

            commands::selectable_list(options).expect("No item selected")
        }
        Some(x) => x.trim().to_string(),
    };

    commands::set_default_context(&ctx);

    let set_context_result = commands::set_context(&ctx, &DEST, &config)
        .map_err(|err| err.to_string());

    if set_context_result.is_ok() {
        println!("{}", KUBECONFIG.as_str());
    }

    set_context_result
}

pub fn context(args: Cli) -> Result<(), String> {
    if args.current {
        let config = config::get_current_session();
        println!("{}", config.current_context);
        return Ok(());
    }

    let config = config::get();
    let ctx = match args.value {
        None => {
            let options : Vec<String> = config.contexts.iter()
                .map(|context| context.name.to_string())
                .collect();

            commands::selectable_list(options).ok_or("No item selected")?
        }
        Some(x) => x.trim().to_string(),
    };

    let set_context_result = commands::set_context(&ctx, &DEST, &config)
        .map_err(|err| err.to_string());

    if set_context_result.is_ok() {
        println!(
            "{}/{}:{}",
            &DEST.as_str(),
            str::replace(&ctx, ":", "_"),
            *KUBECONFIG
        );
    }

    set_context_result
}

pub fn namespace(args: Cli) -> Result<(), String> {
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
        return Ok(());
    }

    let ns = match args.value {
        None => {
            let namespaces : Vec<String> = commands::get_namespaces();
            commands::selectable_list(namespaces).ok_or("No item selected")?
        }
        Some(x) => x.trim().to_string(),
    };

    commands::set_namespace(&config.current_context, &ns, &DEST, &config);

    println!(
        "{}/{}:{}",
        &DEST.as_str(),
        str::replace(&config.current_context, ":", "_"),
        *KUBECONFIG
    );
    Ok(())
}

pub fn default_namespace(args: Cli) -> Result<(), String> {
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

        return Ok(());
    }

    let ns = match args.value {
        None => {
            let namespaces : Vec<String> = commands::get_namespaces();
            commands::selectable_list(namespaces).ok_or("No item selected")?
        }
        Some(x) => x.trim().to_string(),
    };

    commands::set_default_namespace(&ns, &ctx);
    commands::set_namespace(&ctx, &ns, &DEST, &config);
    Ok(())
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
