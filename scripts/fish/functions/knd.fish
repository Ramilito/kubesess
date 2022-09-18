function knd --argument-names namespace --description "Switch global kubernetes namespace"
    kubesess -v $namespace default-namespace
end
