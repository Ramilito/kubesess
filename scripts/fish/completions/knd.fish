# fish completion for knd                                  -*- shell-script -*-

function __knd_debug
    set -l file "$BASH_COMP_DEBUG_FILE"
    if test -n "$file"
        echo "$argv" >> $file
    end
end

function __knd_perform_completion
    __knd_debug "Starting __knd_perform_completion"

    # Retrieve available contexts
    set -l comps (kubectl get ns --no-headers -o custom-columns=":metadata.name" 2> /dev/null)

    __knd_debug "Comps: $comps"

    for comp in $comps
        printf "%s%s\n" "$comp"
    end
end

# This function does two things:
# - Obtain the completions and store them in the global __knd_comp_results
# - Return false if file completion should be performed
function __knd_prepare_completions
    __knd_debug ""
    __knd_debug "========= starting completion logic =========="

    # Start fresh
    set --erase __knd_comp_results

    set -l results (__knd_perform_completion)
    __knd_debug "Completion results: $results"

    if test -z "$results"
        __knd_debug "No completion, probably due to a failure"
        # Might as well do file completion, in case it helps
        return 1
    end

    set --global __knd_comp_results $results

    __knd_debug "Completions are: $__knd_comp_results"

    return 0
end

# Since Fish completions are only loaded once the user triggers them, we trigger them ourselves
# so we can properly delete any completions provided by another script.
# Only do this if the program can be found, or else fish may print some errors; besides,
# the existing completions will only be loaded if the program can be found.
if type -q "knd"
    # The space after the program name is essential to trigger completion for the program
    # and not completion of the program name itself.
    # Also, we use '> /dev/null 2>&1' since '&>' is not supported in older versions of fish.
    complete --do-complete "knd" > /dev/null 2>&1
end

# Remove any pre-existing completions for the program since we will be handling all of them.
complete -c knd -e

# The call to __knd_prepare_completions will setup __knd_comp_results
# which provides the program's completion choices.
complete -c knd -n '__knd_prepare_completions' -f -a '$__knd_comp_results'

