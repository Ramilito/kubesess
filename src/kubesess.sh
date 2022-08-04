kc() {
  config=$(kubesess ${1:+"-v "$1} context):$HOME/.kube/config;
  export KUBECONFIG=$config;
}

kcd() {
  kubesess ${1:+"-v "$1} default-context;
}

kn() {
  kubesess ${1:+"-v "$1} namespace;
}
