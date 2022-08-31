function kcd --argument-names context --description "Switch global kubernetes context"
    set -l config (kubesess -v $context default-context) 
    set -gx KUBECONFIG $config
end
