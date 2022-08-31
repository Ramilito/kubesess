kc() {
  export KUBECONFIG=$(kubesess ${1:+"-v "$1} context);
}

kcd() {
  kubesess ${1:+"-v "$1} default-context;
  # Reset the $KUBECONFIG with default
  export KUBECONFIG=$HOME/.kube/config;
}

kn() {
  export KUBECONFIG=$(kubesess ${1:+"-v "$1} namespace);
}

knd() {
  kubesess ${1:+"-v "$1} default-namespace;
}
