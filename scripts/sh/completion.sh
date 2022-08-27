#/usr/bin/env bash

_kc_completions()
{
  COMPREPLY=($(compgen -W "$(kubesess -v "${COMP_WORDS[1]}" completion-context)"))
}

_kn_completions()
{
  COMPREPLY=($(compgen -W "$(kubesess -v "${COMP_WORDS[1]}" completion-namespace)"))
}

complete -F _kc_completions kc 
complete -F _kc_completions kcd
complete -F _kn_completions kn
complete -F _kn_completions knd
