#!/bin/bash

NEW_CTX=$(./kubesess) 

echo $NEW_CTX

export KUBECONFIG=$NEW_CTX:$KUBECONFIG
