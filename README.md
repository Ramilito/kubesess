# kubesess(ion)

<details>
  <summary>Table of Contents</summary>

- [kubesess(ion)](#kubesess-ion-)
  * [Showcase](#showcase)
  * [About The Project](#about-the-project)
    + [What](#what)
    + [Why](#why)
    + [How](#how)
    + [Benchmark](#benchmark)
  * [Getting Started](#getting-started)
    + [Prerequisite](#prerequisite)
    + [Installation](#installation)
  * [Usage](#usage)
  * [Roadmap](#roadmap)

</details>

## Showcase

![showcase](https://raw.github.com/Ramilito/kubesess/main/docs/images/kubesess.gif)

The showcase demonstrates the multiple sessions per shell feature, selecting items and fuzzy searching for them.
The same features apply for handling namespace as well


## About The Project

### What

This BLAZINGLY FAST plugin written in Rust makes it possible to have one context per shell active.

### Why

Why isolated context?
Typically when you switch context in kubectl (with ex. kubectx), the change happens on all terminal instances. 
That is because the change is saved in your $HOME/.kube/config file and is read on every interaction with kubectl.

This will lead to the inevitable scenario of working on a local cluster, and needing to do something quickly in production. 
You open another terminal, switch context, do your work and then go right back to your old terminal. 
The issue is that the prompt has not visually refreshed to the actual context. 
Often the following command you type will not be dangerous, and you will notice that you did it in production, but sometimes the damage is severe💥.

### How

We will use the config merge capability of kubectl to override the current-context setting.
By creating a file with the correct context and prepend it to the KUBECONFIG environment variable 

The program will output the SESSION_CONTEXT and the alias created in <a href="#installation">Installation</a> will do the prepending
```
export KUBECONFIG=$SESSION_CONTEXT:$KUBECONFIG
```

### Benchmark

Tool: [custom script](./tests/benchmark.sh)
Command | [kubesess](https://github.com/Ramilito/kubesess) | [kubectx](https://github.com/ahmetb/kubectx/tree/master/cmd/kubectx)
---- | ---- | ----
20 runs with no ctx switch and no kubectl calls | .024931342 | 1.744966963
20 runs with ctx switch and no kubectl calls | .049247181 | 3.775905777
20 runs with ctx switch and calling kubectl get nodes | 11.167763585 | 15.265837926

<sup>I am using the input argument variant for both tools, using fzf or tab completion is harder to do.</sup>


Tool: [hyperfine](./tests/hyperfine/markdown.md)
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `kubesess -v docker-desktop context` | 1.3 ± 0.2 | 1.0 | 2.2 | 1.00 |
| `kubectx docker-desktop` | 91.8 ± 3.3 | 85.1 | 100.7 | 71.23 ± 13.64 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `kubesess -v monitoring namespace` | 42.8 ± 1.4 | 41.6 | 46.3 | 1.00 |
| `kubens monitoring` | 914.1 ± 26.4 | 869.2 | 953.2 | 21.35 ± 0.92 |

\* Disclaimer *
kubectx and kubens are a wonderful tools, been using it allot but my workflow is different and thus this tool was created.
Probably most of the speed gains are because I am bypassing kubectl and just editing config files.

## Getting Started

### Prerequisite

* No dependencies

### Installation

Download and extract the binary.
```
wget "https://github.com/Ramilito/kubesess/releases/download/1.2.0/kubesess_1.2.0_x86_64-unknown-linux-musl.tar.gz" && \
mkdir -p $HOME/.kube/kubesess && tar zxpf kubesess_1.2.0_x86_64-unknown-linux-musl.tar.gz -C $HOME/.kube/kubesess
```

A script wrapper called kubesess.sh is provided for easier use, source the script wrapper in your .bashrc, .zshrc.
```
source ~/.kube/kubesess/kubesess.sh
```

## Usage

See the available commands by running kubesess -h, output from the program needs to be added to $KUBECONFIG env variable.

#### Aliases are provided for easier use, when sourced three aliases will be created.
``` bash
kc #kube_context: Sets session context

kcd #kube_context_default: Sets context across all shells

kn #kube_namespace: Sets namespace
```

<!-- ROADMAP -->
## Roadmap

- [x] List all contexts
- [x] Present it with fzf
- [x] Write selection to file
- [x] Output link to file
- [x] Add alias to handle output
- [x] Cleanup after use
    - [x] clean prepended env variable
    - [x] output files to $HOME/.cache/kubesess
- [x] Handle different namespaces per context
- [x] Use rust tui instead of fzf
- [x] Add option to make changes stick (default-context)
- [ ] Add tests 
- [ ] Add tab completion - https://github.com/clap-rs/clap/issues/1232
- [ ] Add to brew
