# Hunter's Mark shell integration for Fish
# Add to ~/.config/fish/config.fish: hunters-mark init fish | source

function hm
    # If no arguments, show help
    if test (count $argv) -eq 0
        command hunters-mark --help
        return
    end

    set subcommand $argv[1]

    # Handle subcommands that don't need special treatment
    switch "$subcommand"
        case add list remove init completions path --help -h --version -v
            command hunters-mark $argv
            return
    end

    # Assume it's a mark name for jumping
    set mark_name $argv[1]
    set target_path (command hunters-mark path "$mark_name" 2>&1)

    if test $status -eq 0
        cd "$target_path"
    else
        echo "$target_path" >&2
        return 1
    end
end
