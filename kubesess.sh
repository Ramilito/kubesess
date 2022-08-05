kc() {
  config=$(kubesess ${1:+"-v "$1} context);
  export KUBECONFIG=$config:$HOME/.kube/config;
}

kcd() {
  kubesess ${1:+"-v "$1} default-context;
  export KUBECONFIG=$HOME/.kube/config;
}

kn() {
  config=$(kubesess ${1:+"-v "$1} namespace);
  export KUBECONFIG=$config:$HOME/.kube/config;

}
