# fish completion for kcd                                  -*- shell-script -*-

function __kcd_debug
    set -l file "$BASH_COMP_DEBUG_FILE"
    if test -n "$file"
        echo "$argv" >> $file
    end
end

function __kcd_perform_completion
    __kcd_debug "Starting __kcd_perform_completion"

    # Retrieve available contexts
    set -l comps (kubectl config get-contexts -o name 2> /dev/null)

    __kcd_debug "Comps: $comps"

    for comp in $comps
        printf "%s%s\n" "$comp"
    end
end

# This function does two things:
# - Obtain the completions and store them in the global __kcd_comp_results
# - Return false if file completion should be performed
function __kcd_prepare_completions
    __kcd_debug ""
    __kcd_debug "========= starting completion logic =========="

    # Start fresh
    set --erase __kcd_comp_results

    set -l results (__kcd_perform_completion)
    __kcd_debug "Completion results: $results"

    if test -z "$results"
        __kcd_debug "No completion, probably due to a failure"
        # Might as well do file completion, in case it helps
        return 1
    end

    set --global __kcd_comp_results $results

    __kcd_debug "Completions are: $__kcd_comp_results"

    return 0
end

# Since Fish completions are only loaded once the user triggers them, we trigger them ourselves
# so we can properly delete any completions provided by another script.
# Only do this if the program can be found, or else fish may print some errors; besides,
# the existing completions will only be loaded if the program can be found.
if type -q "kcd"
    # The space after the program name is essential to trigger completion for the program
    # and not completion of the program name itself.
    # Also, we use '> /dev/null 2>&1' since '&>' is not supported in older versions of fish.
    complete --do-complete "kcd" > /dev/null 2>&1
end

# Remove any pre-existing completions for the program since we will be handling all of them.
complete -c kcd -e

# The call to __kcd_prepare_completions will setup __kcd_comp_results
# which provides the program's completion choices.
complete -c kcd -n '__kcd_prepare_completions' -f -a '$__kcd_comp_results'

