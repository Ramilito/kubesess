kc() {
  config=$(kubesess ${1:+"-v "$1} context):$HOME/.kube/config;
  export KUBECONFIG=$config;
}

kn() {
  kubesess ${1:+"-v "$1} namespace;
}
