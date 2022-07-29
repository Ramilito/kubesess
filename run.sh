#!/bin/bash

pushd $HOME/kubesess

NEW_CTX=$(./kubesess)

popd

export KUBECONFIG=$NEW_CTX:$KUBECONFIG
