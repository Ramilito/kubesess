# kubesess 3.0.0

## ⚠️ Breaking Changes

This release introduces breaking changes to simplify shell integration. Please read the migration guide below.

### CLI Argument Order Changed

Options now come **after** the subcommand:

```bash
# Before (v2.x)
kubesess -v docker-desktop context
kubesess -c context

# After (v3.0)
kubesess context -v docker-desktop
kubesess context -c
```

### New Shell Integration

Shell integration is now built into the binary. Replace your existing shell setup:

**Bash** (`~/.bashrc`):
```bash
eval "$(kubesess init bash)"
```

**Zsh** (`~/.zshrc`):
```zsh
eval "$(kubesess init zsh)"
```

**Fish** (`~/.config/fish/config.fish`):
```fish
kubesess init fish | source
```

**PowerShell**:
```powershell
Invoke-Expression (&kubesess init powershell)
```

## ✨ New Features

- **`kubesess init` command** - Single command to set up shell integration with functions (`kc`, `kcd`, `kn`, `knd`) and tab completions
- **PowerShell support** - Full support including tab completions
- **Simplified installation** - No more copying script files, just one line in your shell config

## 📦 Installation

### Homebrew
```bash
brew upgrade kubesess
```

### Binary
```bash
KUBESESS_VERSION=3.0.0 && \
KUBESESS_OS=x86_64-unknown-linux-gnu && \
wget "https://github.com/Ramilito/kubesess/releases/download/${KUBESESS_VERSION}/kubesess_${KUBESESS_VERSION}_${KUBESESS_OS}.tar.gz" && \
tar zxpf kubesess_${KUBESESS_VERSION}_${KUBESESS_OS}.tar.gz && \
sudo mv kubesess /usr/local/bin/kubesess
```

Then add shell integration (see above).

## 🔄 Migration from v2.x

1. Update the binary (brew upgrade or download new version)
2. Remove old shell config lines:
   ```bash
   # Remove these:
   source ~/.kube/kubesess/scripts/sh/kubesess.sh
   source ~/.kube/kubesess/scripts/sh/completion.sh
   # Or for Homebrew:
   source ${HOMEBREW_PREFIX}/share/zsh/site-functions/kubesess.sh
   source ${HOMEBREW_PREFIX}/opt/kubesess/etc/bash_completion.d/completion.sh
   ```
3. Add new shell integration:
   ```bash
   eval "$(kubesess init bash)"  # or zsh
   ```
4. Restart your shell

## 📝 Full Changelog

See [CHANGELOG.md](https://github.com/Ramilito/kubesess/blob/main/CHANGELOG.md) for details.
