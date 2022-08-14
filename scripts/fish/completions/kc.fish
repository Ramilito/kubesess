# fish completion for kc                                  -*- shell-script -*-

function __kc_debug
    set -l file "$BASH_COMP_DEBUG_FILE"
    if test -n "$file"
        echo "$argv" >> $file
    end
end

function __kc_perform_completion
    __kc_debug "Starting __kc_perform_completion"

    # Retrieve available contexts
    set -l comps (kubectl config get-contexts -o name 2> /dev/null)

    __kc_debug "Comps: $comps"

    for comp in $comps
        printf "%s%s\n" "$comp"
    end
end

# This function does two things:
# - Obtain the completions and store them in the global __kc_comp_results
# - Return false if file completion should be performed
function __kc_prepare_completions
    __kc_debug ""
    __kc_debug "========= starting completion logic =========="

    # Start fresh
    set --erase __kc_comp_results

    set -l results (__kc_perform_completion)
    __kc_debug "Completion results: $results"

    if test -z "$results"
        __kc_debug "No completion, probably due to a failure"
        # Might as well do file completion, in case it helps
        return 1
    end

    set --global __kc_comp_results $results

    __kc_debug "Completions are: $__kc_comp_results"

    return 0
end

# Since Fish completions are only loaded once the user triggers them, we trigger them ourselves
# so we can properly delete any completions provided by another script.
# Only do this if the program can be found, or else fish may print some errors; besides,
# the existing completions will only be loaded if the program can be found.
if type -q "kc"
    # The space after the program name is essential to trigger completion for the program
    # and not completion of the program name itself.
    # Also, we use '> /dev/null 2>&1' since '&>' is not supported in older versions of fish.
    complete --do-complete "kc" > /dev/null 2>&1
end

# Remove any pre-existing completions for the program since we will be handling all of them.
complete -c kc -e

# The call to __kc_prepare_completions will setup __kc_comp_results
# which provides the program's completion choices.
complete -c kc -n '__kc_prepare_completions' -f -a '$__kc_comp_results'

