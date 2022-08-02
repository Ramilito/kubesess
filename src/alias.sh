kc() {
  config=$(~/kubesess/kubesess ${1:+"-c "$1} context):$HOME/.kube/config;
  export KUBECONFIG=$config;
}

kn() {
  config=$(~/kubesess/kubesess ${1:+"-n "$1} namespace):$HOME/.kube/config;
  export KUBECONFIG=$config;
}
