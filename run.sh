#!/bin/bash

parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
NEW_CTX=$($parent_path/kubesess/kubesess)

export KUBECONFIG=$NEW_CTX:$KUBECONFIG
