# Hunter's Mark shell integration for Zsh
# Add to ~/.zshrc: eval "$(hm init zsh)"

hm() {
    # If no arguments, show help
    if [ $# -eq 0 ]; then
        command hm --help
        return
    fi

    local subcommand="$1"

    # Handle subcommands that don't need special treatment
    case "$subcommand" in
        add|list|remove|init|completions|path|--help|-h|--version|-v)
            command hm "$@"
            return
            ;;
    esac

    # Assume it's a mark name for jumping
    local mark_name="$1"
    local target_path

    # Get the target path
    target_path=$(command hm path "$mark_name" 2>&1)

    if [ $? -eq 0 ]; then
        cd "$target_path" || return 1
    else
        echo "$target_path" >&2
        return 1
    fi
}
