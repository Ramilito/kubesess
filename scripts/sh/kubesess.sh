#!/usr/bin/env bash

__kubesess_export() {
  local OUTPUT
  OUTPUT="$(kubesess "$@")" || return $?
  export KUBECONFIG="$OUTPUT"
}

kc() {
  __kubesess_export ${1:+"-v "$1} context
}

kcd() {
  __kubesess_export ${1:+"-v "$1} default-context
}

kn() {
  __kubesess_export ${1:+"-v "$1} namespace
}

knd() {
  __kubesess_export ${1:+"-v "$1} default-namespace
}
