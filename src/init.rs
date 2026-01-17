/// Shell types supported by kubesess init
#[derive(clap::ValueEnum, Clone, Copy)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Powershell,
}

/// Print the shell initialization script for the given shell
pub fn print_init_script(shell: Shell) {
    let script = match shell {
        Shell::Bash | Shell::Zsh => BASH_ZSH_INIT,
        Shell::Fish => FISH_INIT,
        Shell::Powershell => POWERSHELL_INIT,
    };
    print!("{}", script);
}

const BASH_ZSH_INIT: &str = r#"# kubesess shell integration
# Add to your .bashrc or .zshrc:
#   eval "$(kubesess init bash)"  # for bash
#   eval "$(kubesess init zsh)"   # for zsh

__kubesess_export() {
  local OUTPUT
  OUTPUT="$(kubesess "$@")" || return $?
  export KUBECONFIG="$OUTPUT"
}

kc() {
  __kubesess_export context ${1:+"-v" "$1"}
}

kcd() {
  __kubesess_export default-context ${1:+"-v" "$1"}
}

kn() {
  __kubesess_export namespace ${1:+"-v" "$1"}
}

knd() {
  __kubesess_export default-namespace ${1:+"-v" "$1"}
}

# Completions
_kc_completions() {
  COMPREPLY=($(compgen -W "$(kubesess completion-context -v "${COMP_WORDS[1]}")" -- "${COMP_WORDS[1]}"))
}

_kn_completions() {
  COMPREPLY=($(compgen -W "$(kubesess completion-namespace -v "${COMP_WORDS[1]}")" -- "${COMP_WORDS[1]}"))
}

complete -F _kc_completions kc
complete -F _kc_completions kcd
complete -F _kn_completions kn
complete -F _kn_completions knd
"#;

const FISH_INIT: &str = r#"# kubesess shell integration for fish
# Add to your config.fish:
#   kubesess init fish | source

function kc --argument-names context --description "Switch current kubernetes context"
    set -l cmd kubesess context
    if test -n "$argv"
        set -a cmd -v $context
    end
    set -l config (command $cmd) || return $status
    set -gx KUBECONFIG $config
end

function kcd --argument-names context --description "Switch global kubernetes context"
    set -l cmd kubesess default-context
    if test -n "$argv"
        set -a cmd -v $context
    end
    set -l config (command $cmd) || return $status
    set -gx KUBECONFIG $config
end

function kn --argument-names namespace --description "Switch current kubernetes namespace"
    set -l cmd kubesess namespace
    if test -n "$argv"
        set -a cmd -v $namespace
    end
    set -l config (command $cmd) || return $status
    set -gx KUBECONFIG $config
end

function knd --argument-names namespace --description "Switch global kubernetes namespace"
    set -l cmd kubesess default-namespace
    if test -n "$argv"
        set -a cmd -v $namespace
    end
    set -l config (command $cmd) || return $status
    set -gx KUBECONFIG $config
end

# Completions for kc/kcd (context)
function __kubesess_contexts
    kubectl config get-contexts -o name 2>/dev/null
end

complete -c kc -f -a '(__kubesess_contexts)'
complete -c kcd -f -a '(__kubesess_contexts)'

# Completions for kn/knd (namespace)
function __kubesess_namespaces
    kubectl get ns --no-headers -o custom-columns=":metadata.name" 2>/dev/null
end

complete -c kn -f -a '(__kubesess_namespaces)'
complete -c knd -f -a '(__kubesess_namespaces)'
"#;

const POWERSHELL_INIT: &str = r#"# kubesess shell integration for PowerShell
# Add to your PowerShell profile:
#   Invoke-Expression (&kubesess init powershell)

function kc {
    param([string]$Context)
    if ($Context) {
        $config = kubesess context -v $Context
    } else {
        $config = kubesess context
    }
    if ($LASTEXITCODE -eq 0) {
        $env:KUBECONFIG = $config
    }
}

function kcd {
    param([string]$Context)
    if ($Context) {
        $config = kubesess default-context -v $Context
    } else {
        $config = kubesess default-context
    }
    if ($LASTEXITCODE -eq 0) {
        $env:KUBECONFIG = $config
    }
}

function kn {
    param([string]$Namespace)
    if ($Namespace) {
        $config = kubesess namespace -v $Namespace
    } else {
        $config = kubesess namespace
    }
    if ($LASTEXITCODE -eq 0) {
        $env:KUBECONFIG = $config
    }
}

function knd {
    param([string]$Namespace)
    if ($Namespace) {
        $config = kubesess default-namespace -v $Namespace
    } else {
        $config = kubesess default-namespace
    }
    if ($LASTEXITCODE -eq 0) {
        $env:KUBECONFIG = $config
    }
}

# Tab completions
Register-ArgumentCompleter -CommandName kc, kcd -ParameterName Context -ScriptBlock {
    param($commandName, $parameterName, $wordToComplete, $commandAst, $fakeBoundParameters)
    kubectl config get-contexts -o name 2>$null | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
        [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
}

Register-ArgumentCompleter -CommandName kn, knd -ParameterName Namespace -ScriptBlock {
    param($commandName, $parameterName, $wordToComplete, $commandAst, $fakeBoundParameters)
    kubectl get ns --no-headers -o custom-columns=":metadata.name" 2>$null | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
        [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
}
"#;
