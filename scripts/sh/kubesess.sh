kc() {
  export KUBECONFIG=$(kubesess ${1:+"-v "$1} context);
}

kcd() {
  export KUBECONFIG=$(kubesess ${1:+"-v "$1} default-context);
}

kn() {
  export KUBECONFIG=$(kubesess ${1:+"-v "$1} namespace);
}

knd() {
  kubesess ${1:+"-v "$1} default-namespace;
}
