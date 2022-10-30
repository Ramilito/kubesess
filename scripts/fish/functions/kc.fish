function kc --argument-names context --description "Switch current kubernetes context"
    set -l cmd kubesess context
    if test -n "$argv"
        set -a cmd -v $context
    end
    set -l config (command $cmd) || return $status
    set -gx KUBECONFIG $config
end
