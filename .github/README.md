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
  * [Troubleshooting](#troubleshooting))

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

Tool: [custom script](./benches/benchmark.sh)
Command | [kubesess](https://github.com/Ramilito/kubesess) | [kubectx](https://github.com/ahmetb/kubectx/tree/master/cmd/kubectx)
---- | ---- | ----
20 runs with no ctx switch and no kubectl calls | .024931342 | 1.744966963
20 runs with ctx switch and no kubectl calls | .049247181 | 3.775905777
20 runs with ctx switch and calling kubectl get nodes | 11.167763585 | 15.265837926

<sup>I am using the input argument variant for both tools, using fzf or tab completion is harder to do.</sup>


Tool: [hyperfine](./benches/hyperfine/markdown.md)
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `kubesess -v docker-desktop context` | 1.3 ± 0.2 | 1.0 | 2.2 | 1.00 |
| `kubectx docker-desktop` | 91.8 ± 3.3 | 85.1 | 100.7 | 71.23 ± 13.64 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `kubesess -v monitoring namespace` | 0.8 ± 0.1 | 0.7 | 0.9 | 1.00 |
| `kubens monitoring` | 215.3 ± 27.0 | 199.5 | 288.9 | 270.22 ± 38.34 |

\* Disclaimer *
kubectx and kubens are a wonderful tools, been using it allot but my workflow is different and thus this tool was created.
Probably most of the speed gains are because I am bypassing kubectl and just editing config files.

## Getting Started

### Prerequisite

* No dependencies

### Installation

#### Binary
Download and extract the binary.
```zsh
KUBESESS_VERSION=1.2.8 && \
KUBESESS_OS=x86_64-unknown-linux-gnu && \
wget "https://github.com/Ramilito/kubesess/releases/download/${KUBESESS_VERSION}/kubesess_${KUBESESS_VERSION}_${KUBESESS_OS}.tar.gz" && \
mkdir -p $HOME/.kube/kubesess && tar zxpf kubesess_${KUBESESS_VERSION}_${KUBESESS_OS}.tar.gz -C $HOME/.kube/kubesess && \
sudo mv ~/.kube/kubesess/target/${KUBESESS_OS}/release/kubesess /usr/local/bin/kubesess
```

A script wrapper called kubesess.sh is provided for easier use, source the script wrapper in your .bashrc, .zshrc.
```zsh
source ~/.kube/kubesess/scripts/sh/kubesess.sh
```

For zsh users, source the completion script to your .zsh/.zshrc file
```zsh
source ~/.kube/kubesess/scripts/sh/completion.sh
```

For fish users, copy functions and completion scripts in your fish config.
```shell
rsync -a ~/.kube/kubesess/scripts/fish/ ~/.config/fish/
```

#### Brew
```zsh
brew install kubesess
```
If you have https://docs.brew.sh/Shell-Completion setup then you are done, if not, you will need to source the script wrapper manually like in the binary installation.

## Usage

See the available commands by running kubesess -h, output from the program needs to be added to $KUBECONFIG env variable.

#### Aliases are provided for easier use, when sourced these aliases will be created.
```zsh
kc  #kube_context: Sets session context

kcd #kube_context_default: Sets context across all shells

kn  #kube_namespace: Sets namespace

knd #kube_namespace_default: Sets namespace across all shells
```

#### Multiple config files
We follow kubectl recommended way of adding multiconfig files found [here](https://kubernetes.io/docs/tasks/access-application-cluster/configure-access-multiple-clusters/#create-a-second-configuration-file).
Example:
```
export KUBECONFIG=$HOME/.kube/config:$HOME/.kube/config-demo:$HOME/.kube/config-demo-2
```

#### Add information to prompt (there are other good tools for this, kube-ps1 and p10k)
```
prompt_context() {
    KUBE_CTX=$(kubesess -c context)
    KUBE_NS=$(kubesess -c namespace)

    if [[ $KUBE_CTX == *"prod"* ]]; then
      echo "❗%{$fg[yellow]%}|$KUBE_CTX%{$reset_color%}:%F{6}$KUBE_NS%f"
    else
      echo "✅️%{$fg[green]%}|$KUBE_CTX%{$reset_color%}:%F{6}$KUBE_NS%f"
    fi
}

RPROMPT='$(prompt_context)'
```

![showcase](https://raw.github.com/Ramilito/kubesess/main/docs/images/prompt.png)

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
- [x] Add option to make changes stick (default-namespace)
- [x] Add tests 
- [x] Add tab completion - https://github.com/clap-rs/clap/issues/1232
- [x] Add to brew
- [ ] Add error handling
- [ ] Add support for multiple .kube/config files

## Troubleshooting

### Just fix it
A hard reset will fix most issues, to do that just remove the ```~/.kube/kubesess/cache``` folder.

\*The downside is that your last visited namespace per context will be lost.


### Why is it happening?
There are only two places that can go wrong, either the ```$KUBECONFIG``` env is 
not set correctly or the generated file is corrupt.

This is how the ```$KUBECONFIG``` should look like (replace ```${USER}``` with your user name):
```zsh
/home/${USER}/.kube/kubesess/cache/docker-desktop:/home/${USER}/.kube/config
```

This is how the generated file should look like:
```yaml
kind: Config
apiVersion: v1
current-context: docker-desktop
contexts:
- context:
    namespace: default
    cluster: docker-desktop
    user: docker-desktop
  name: docker-desktop
```

