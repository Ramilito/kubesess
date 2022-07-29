# kubesess

## What
This plugin makes it possible to have one context per shell active.
## Why
Why isolated context?
Typically when you switch context in kubectl (with ex. kubectx), the change happens on all terminal instances. That is because the change is saved in your $HOME/.kube/config file and is read on every interaction with kubectl.

This will lead to the inevitable scenario of working on a local cluster, and needing to do something quickly in production. You open another terminal, switch context, do your work and then go right back to your old terminal. The issue is that the prompt has not visually refreshed to the actual context. Often the following command you type will not be dangerous, and you will notice that you did it in production, but sometimes the damage is severe💥.
