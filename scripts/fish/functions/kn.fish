function kn --argument-names namespace --description "Switch current kubernetes namespace"
    set -l config (kubesess -v $namespace namespace) 
    set -gx KUBECONFIG $config
end
