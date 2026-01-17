# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.0] - Unreleased

### Breaking Changes

- **CLI restructured to use subcommands**: The argument order has changed. Options like `-v` and `-c` now come *after* the subcommand, not before.

  ```bash
  # Old (v2.x)
  kubesess -v docker-desktop context
  kubesess -c context

  # New (v3.0)
  kubesess context -v docker-desktop
  kubesess context -c
  ```

- **Shell integration now uses `kubesess init`**: Instead of sourcing separate script files, shell integration is now built into the binary.

### Added

- New `init` subcommand for shell integration
  - `kubesess init bash` - Bash initialization script
  - `kubesess init zsh` - Zsh initialization script
  - `kubesess init fish` - Fish initialization script
  - `kubesess init powershell` - PowerShell initialization script (new!)
- PowerShell support with tab completions

### Migration Guide

#### Updating Shell Configuration

Replace your existing shell integration with the new `init` command:

**Bash** - Update `~/.bashrc`:
```bash
# Remove old lines:
# source ~/.kube/kubesess/scripts/sh/kubesess.sh
# source ~/.kube/kubesess/scripts/sh/completion.sh

# Add new line:
eval "$(kubesess init bash)"
```

**Zsh** - Update `~/.zshrc`:
```zsh
# Remove old lines:
# source ${HOMEBREW_PREFIX}/share/zsh/site-functions/kubesess.sh
# source ${HOMEBREW_PREFIX}/opt/kubesess/etc/bash_completion.d/completion.sh

# Add new line:
eval "$(kubesess init zsh)"
```

**Fish** - Update `~/.config/fish/config.fish`:
```fish
# Remove old setup (delete copied function files if present)

# Add new line:
kubesess init fish | source
```

**PowerShell** - Add to your profile:
```powershell
Invoke-Expression (&kubesess init powershell)
```

#### Updating Scripts

If you have scripts that call `kubesess` directly, update the argument order:

```bash
# Old
kubesess -v my-context context
kubesess -v my-namespace namespace

# New
kubesess context -v my-context
kubesess namespace -v my-namespace
```

## [2.0.3] - Previous

See [GitHub Releases](https://github.com/Ramilito/kubesess/releases) for previous changelog entries.
