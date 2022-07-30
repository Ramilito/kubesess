# kubesess(ion)

## Showcase

![showcase](https://rentarami.se/assets/images/posts/kube_context.gif)

## What?
This plugin makes it possible to have one context per shell active.

## Why?
Why isolated context?
Typically when you switch context in kubectl (with ex. kubectx), the change happens on all terminal instances. 
That is because the change is saved in your $HOME/.kube/config file and is read on every interaction with kubectl.

This will lead to the inevitable scenario of working on a local cluster, and needing to do something quickly in production. 
You open another terminal, switch context, do your work and then go right back to your old terminal. 
The issue is that the prompt has not visually refreshed to the actual context. 
Often the following command you type will not be dangerous, and you will notice that you did it in production, but sometimes the damage is severe💥.

## How?
We will use the config merge capability of kubectl to override the current-context setting.
By creating a file with the correct context and prepend it to the KUBECONFIG environment variable 

```
export KUBECONFIG=$SESSION_CONTEXT:$KUBECONFIG
```

## Prerequisite
  * [fzf](https://github.com/junegunn/fzf)

## Installation
```
wget "https://github.com/Ramilito/kubesess/releases/download/0.1.0/kubesess_0.1.0_x86_64-unknown-linux-musl.tar.gz" &&
mkdir ~/kubesess && tar zxpf kubesess_0.1.0_x86_64-unknown-linux-musl.tar.gz -C ~/kubesess &&
```

Finally add an alias to run it in your .bashrc, .zshrc
```
alias Switch="export KUBECONFIG=$(~/kubesess/kubesess):$KUBECONFIG"
```

