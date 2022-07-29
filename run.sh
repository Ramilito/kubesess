#!/bin/bash

NEW_CTX=$(./kubesess) 

export KUBECONFIG=$NEW_CTX:$KUBECONFIG
