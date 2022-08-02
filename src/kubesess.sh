kc() {
  config=$(kubesess ${1:+"-c "$1} context):$HOME/.kube/config;
  export KUBECONFIG=$config;
}

kn() {
  kubesess ${1:+"-n "$1} namespace;
}
