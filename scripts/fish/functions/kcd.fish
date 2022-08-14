function kcd --argument-names context --description "Switch global kubernetes context"
    kubesess -v $context context
    set -gx KUBECONFIG $HOME/.kube/config
end
