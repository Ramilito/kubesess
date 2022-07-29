#!/bin/bash

NEW_CTX=$(./target/debug/switch_context) 

export KUBECONFIG=$NEW_CTX:$KUBECONFIG
