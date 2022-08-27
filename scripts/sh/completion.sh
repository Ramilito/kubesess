#/usr/bin/env bash
# complete -W "now tomorrow never" kc

_kc_completions()
{
  # COMPREPLY=($(compgen -W "$(fc kc | sed 's/\t//')" -- "${COMP_WORDS[1]}"))
  COMPREPLY=($(compgen -W "$(kubesess -v "${COMP_WORDS[1]}" completion-context)"))
}

complete -F _kc_completions kc
