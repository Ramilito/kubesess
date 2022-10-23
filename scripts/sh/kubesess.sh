#!/usr/bin/env bash

kc() {
  export KUBESESSCONFIG=$(kubesess ${1:+"-v "$1} context);
}

kcd() {
  export KUBESESSCONFIG=$(kubesess ${1:+"-v "$1} default-context);
}

kn() {
  export KUBESESSCONFIG=$(kubesess ${1:+"-v "$1} namespace);
}

knd() {
  kubesess ${1:+"-v "$1} default-namespace;
}
