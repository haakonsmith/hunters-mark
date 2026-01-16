# Hunter's Mark

Quick directory navigation for your terminal. Mark directories and jump between them instantly.

This idea came about from using zoxide. It's awesome and lets me navigate the terminal so much easier that I forgot what I ever did before it, however, I find myself mostly using it to jump between common directories. E.g. jump between projects, and I thought, what if I had a cli tool which could do that for me? I could mark directories and then navigate to them. Furthermore, I could setup some scripts to initalise project workspaces, e.g. start different cli tools launch zed windows etc. That is how "hunter's mark" came to be, it's a quick name for something to prevent bikeshedding and I thought of the name harpoon but that is kinda already used by the Primeagans cli tool.

## AI Disclosure/Code Quality

I reckon if you can't tell that it's AI then that's the right kind of AI usage, however, in it's current state you can probably guess some of it was slopped out by an AI. I'd like to acknowledge that if I end up improving it then I'll tidy it up

## Installation

### From Source

```bash
cargo install --path .
```

### From GitHub

```bash
cargo install --git https://github.com/haakonsmith/hunters-mark.git
```

### Shell Integration

Add to your shell config (required for directory jumping):

**Zsh** (`~/.zshrc`):
```bash
eval "$(hunters-mark init zsh)"
```

**Bash** (`~/.bashrc`):
```bash
eval "$(hunters-mark init bash)"
```

**Fish** (`~/.config/fish/config.fish`):
```fish
hunters-mark init fish | source
```

This will add a new command with prefix `hm` by default. You can also use `hunters-mark init --prefix "<WHATEVER>"` to change that.

### Shell Completions (Optional)

**Zsh**:
```zsh
hunters-mark completions zsh > ~/.zsh/completions/_hm
```

**Bash**:
```bash
hunters-mark completions bash > ~/.local/share/bash-completion/completions/hm
```

**Fish**:
```fish
hunters-mark completions fish > ~/.config/fish/completions/hm.fish
```

Or

```fish
hunters-mark completions fish | source
```

## Usage

### Add a mark

Mark the current directory:
```bash
hm add myproject
```

Mark a specific directory with tags:
```bash
hm add myproject ~/projects/awesome-app --tags rust,web
```

### Jump to a mark

```bash
hm myproject
```

### List marks

List all marks:
```bash
hm list
```

List by tag:
```bash
hm list --tag rust
```

List sorted by most recently accessed:
```bash
hm list --recent
```

### Remove a mark

```bash
hm remove
```

## Configuration

Marks are stored in `~/.config/hunters-mark/config.toml` (or platform equivalent).

Example configuration:
```toml
[settings]
run_init_scripts = true

[marks.myproject]
path = "/Users/username/projects/awesome-app"
tags = ["rust", "web"]
last_accessed = "2026-01-15T10:30:00Z"
created_at = "2026-01-10T15:00:00Z"
```

## Features

- Quick directory navigation with shell integration
- Tag-based organization
- Automatic timestamp tracking
- Cross-platform (macOS, Linux, Windows)
- Shell completions for bash, zsh, and fish
- TUI selector

## Planned Features

- Project initialization scripts
- Script trust/security mechanism
