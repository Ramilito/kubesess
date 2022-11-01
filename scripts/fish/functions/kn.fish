function kn --argument-names namespace --description "Switch current kubernetes namespace"
    set -l cmd kubesess namespace
    if test -n "$argv"
        set -a cmd -v $namespace
    end
    set -l config (command $cmd) || return $status
    set -gx KUBECONFIG $config
end
