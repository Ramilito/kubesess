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
  * [Configuration](#configuration)
  * [Usage](#usage)
  * [Roadmap](#roadmap)

</details>

## Showcase

![showcase](https://raw.github.com/Ramilito/kubesess/main/docs/images/kubesess.gif)

<p> The showcase demonstrates the multiple sessions per shell, selecting items and fuzzy searching for them.</p>

## About The Project

### What

This BLAZINGLY FAST plugin makes it possible to have one context per shell active.

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

Tool | [kubesess](https://github.com/Ramilito/kubesess) | [kubectx](https://github.com/ahmetb/kubectx/tree/master/cmd/kubectx)
---- | ---- | ----
20 runs with no ctx switch and no kubectl calls | .024931342 | 1.744966963
20 runs with ctx switch and no kubectl calls | .049247181 | 3.775905777
20 runs with ctx switch and calling kubectl get nodes | 11.167763585 | 15.265837926

###### I am using the input argument variant for both tools, using fzf or tab completion is harder to measure.

## Getting Started

### Prerequisite

* No dependencies

### Installation

Download and extract the binary.
```
wget "https://github.com/Ramilito/kubesess/releases/download/0.1.0/kubesess_0.1.0_x86_64-unknown-linux-musl.tar.gz" &&
mkdir $HOME/kubesess && tar zxpf kubesess_0.1.0_x86_64-unknown-linux-musl.tar.gz -C $HOME/kubesess
```

Finally, add aliases to run it in your .bashrc, .zshrc.
```
alias Switch='export KUBECONFIG=$(~/kubesess/kubesess -- context):$HOME/.kube/config'
alias kn='export KUBECONFIG=$(~/kubesess/kubesess -- namespace):$HOME/.kube/config'
```

## Configuration

You can set default context using context name as an input in your .bashrc, .zshrc and exporting the result to KUBECONFIG environment variable.

```
export KUBECONFIG=$(~/kubesess/kubesess -- context docker-desktop):$HOME/.kube/config
alias Switch='export KUBECONFIG=$(~/kubesess/kubesess -- context):$HOME/.kube/config'
```

## Usage

Use the alias, can be whatever you want it to be, and then pick one of the suggested context to switch to.

<!-- ROADMAP -->
## Roadmap

- [x] List all contexts
- [x] Present it with fzf
- [x] Write selection to file
- [x] Output link to file
- [x] Add alias to handle output
- [x] Cleanup environment each use
    - [x] env variable
    - [x] output files to $HOME/.cache/kubesess
- [x] Handle different namespaces per shell
- [x] Use rust tui instead of fzf
- [ ] Add option to make changes stick (all shells)
- [ ] Add tab completion

