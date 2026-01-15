# Hunter's Mark shell integration for Bash
# Add to ~/.bashrc: eval "$(hunters-mark init bash)"

hm() {
    # If no arguments, show help
    if [ $# -eq 0 ]; then
        command hunters-mark --help
        return
    fi

    local subcommand="$1"

    # Handle subcommands that don't need special treatment
    case "$subcommand" in
        add|list|remove|init|completions|path|--help|-h|--version|-v)
            command hunters-mark "$@"
            return
            ;;
    esac

    # Assume it's a mark name for jumping
    local mark_name="$1"
    local target_path

    # Get the target path
    target_path=$(command hunters-mark path "$mark_name" 2>&1)

    if [ $? -eq 0 ]; then
        cd "$target_path" || return 1
    else
        echo "$target_path" >&2
        return 1
    fi
}
