function kc --argument-names context --description "Switch current kubernetes context"
    set -l config (kubesess -v $context context) 
    set -gx KUBECONFIG $config:$HOME/.kube/config
end
