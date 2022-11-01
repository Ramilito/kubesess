function knd --argument-names context --description "Switch global kubernetes namespace"
    set -l cmd kubesess default-namespace
    if test -n "$argv"
        set -a cmd -v $context
    end
    command $cmd
end
