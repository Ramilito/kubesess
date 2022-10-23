function kcd --argument-names context --description "Switch global kubernetes context"
    set -l config (kubesess -v $context default-context) 
    set -gx KUBESESSCONFIG $config
end
