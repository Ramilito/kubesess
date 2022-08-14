# fish completion for kn                                  -*- shell-script -*-

function __kn_debug
    set -l file "$BASH_COMP_DEBUG_FILE"
    if test -n "$file"
        echo "$argv" >> $file
    end
end

function __kn_perform_completion
    __kn_debug "Starting __kn_perform_completion"

    # Retrieve available contexts
    set -l comps (kubectl get ns --no-headers -o custom-columns=":metadata.name" 2> /dev/null)

    __kn_debug "Comps: $comps"

    for comp in $comps
        printf "%s%s\n" "$comp"
    end
end

# This function does two things:
# - Obtain the completions and store them in the global __kn_comp_results
# - Return false if file completion should be performed
function __kn_prepare_completions
    __kn_debug ""
    __kn_debug "========= starting completion logic =========="

    # Start fresh
    set --erase __kn_comp_results

    set -l results (__kn_perform_completion)
    __kn_debug "Completion results: $results"

    if test -z "$results"
        __kn_debug "No completion, probably due to a failure"
        # Might as well do file completion, in case it helps
        return 1
    end

    set --global __kn_comp_results $results

    __kn_debug "Completions are: $__kn_comp_results"

    return 0
end

# Since Fish completions are only loaded once the user triggers them, we trigger them ourselves
# so we can properly delete any completions provided by another script.
# Only do this if the program can be found, or else fish may print some errors; besides,
# the existing completions will only be loaded if the program can be found.
if type -q "kn"
    # The space after the program name is essential to trigger completion for the program
    # and not completion of the program name itself.
    # Also, we use '> /dev/null 2>&1' since '&>' is not supported in older versions of fish.
    complete --do-complete "kn" > /dev/null 2>&1
end

# Remove any pre-existing completions for the program since we will be handling all of them.
complete -c kn -e

# The call to __kn_prepare_completions will setup __kn_comp_results
# which provides the program's completion choices.
complete -c kn -n '__kn_prepare_completions' -f -a '$__kn_comp_results'

