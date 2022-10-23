function kc --argument-names context --description "Switch current kubernetes context"
    set -l config (kubesess -v $context context) 
    set -gx KUBESESSCONFIG $config
end
