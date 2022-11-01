function kcd --argument-names context --description "Switch global kubernetes context"
    set -l cmd kubesess default-context
    if test -n "$argv"
        set -a cmd -v $context
    end
    set -l config (command $cmd) || return $status
    set -gx KUBECONFIG $config
end
