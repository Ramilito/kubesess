kc() {
  kubesess ${1:+"-v "$1} context;
}

kcd() {
  kubesess ${1:+"-v "$1} default-context;
}

kn() {
  kubesess ${1:+"-v "$1} namespace;
}
