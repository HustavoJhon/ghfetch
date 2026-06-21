# ghfetch

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.95+-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/theme-Gruvbox-green.svg" alt="Theme">
</p>

A **Neofetch/Fastfetch-inspired CLI tool** for displaying GitHub profile information with system stats, all wrapped in a beautiful retro-terminal **Gruvbox** aesthetic.

```
☀️ Buenas tardes, Gustavo Jhon

Gustavo Jhon
@HustavoJhon
👥 25  👤 9  📦 16

       ░░░░░░░░░░░░░░░░░░░░░░░     👻 OS  Arch Linux
     ░▒▒▒▓▓▓▓▓▓▒▒░░░░░░░░░░░░░░     👻 Kernel  7.0.9-arch2-1
    ░░▒▓▓▓▓▓▓▓▓▓▓▒░░░░░░░▒▒░░░░     👻 CPU  AMD Ryzen 5 3600
    ░░▒▓▓▓▓▓▓▓▓▒▒░░░░░░░▒▓▓▒░░░     👻 RAM  8.2 GiB / 16 GiB
    ░▒▓▓▓▓▓▓▓▓▒░░░░░░░░▒▓▓▓▓▒░░     👻 Shell  zsh

Contribuciones GitHub: 74

  ░░░░░░░░░░░░░░▒░░░░░░░░░░░░░▒░░░░▓░░░░░▒░░░░░▒░░░░░░░░░░░░░░
  ░░░░▒░░░░░░▒░░░░░░░░░░░░░░░░░░░▒░░░░░▒░░░▒░░░░░░░░░░░░░░░░░░
  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▒▒
  ░░░▒░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

fetched in 115ms
```

---

## Features

- **ASCII Avatar** — Downloads your GitHub avatar and renders it as colored ASCII art
- **System Info** — OS, kernel, CPU, GPU, RAM, disk, shell, terminal, resolution, uptime, arch
- **GitHub Profile** — Name, bio, followers, following, repos
- **Dynamic Greeting** — Time-based Spanish greeting (Buenos dias/tardes/noches)
- **Contribution Graph** — GitHub-style ASCII contribution calendar
- **Responsive Layout** — Adapts to terminal width (horizontal/vertical)
- **Fast** — <300ms startup with caching; 115ms cached
- **Gruvbox Dark Theme** — Consistent retro color palette
- **Cross-platform** — Linux, macOS, Windows
- **Configurable** — TOML config file with all options

---

## Installation

### Quick install (recommended)

```bash
git clone https://github.com/hustavojhon/ghfetch.git
cd ghfetch
bash install.sh
```

The installer will ask for your GitHub username, optionally a token, and set everything up.

### From source

```bash
git clone https://github.com/hustavojhon/ghfetch.git
cd ghfetch
cargo build --release
sudo cp target/release/ghfetch /usr/local/bin/
```

### From crates.io

```bash
cargo install ghfetch
```

---

## Usage

```bash
# Default — shows your profile
ghfetch

# Show a different GitHub user
ghfetch -u torvalds

# Disable sections
ghfetch --no-avatar
ghfetch --no-system
ghfetch --no-contributions

# Customize avatar width
ghfetch -w 40

# Limit contribution graph weeks
ghfetch -m 26

# Use Nerd Font icons
ghfetch --nerd-font-icons

# Custom config file
ghfetch -c ~/.config/ghfetch/custom.toml
```

### Options

| Flag | Short | Description |
|------|-------|-------------|
| `--username` | `-u` | GitHub username (default: hustavojhon) |
| `--config` | `-c` | Path to config file |
| `--no-avatar` | | Hide the ASCII avatar |
| `--no-system` | | Hide system information |
| `--no-header` | | Hide the profile header |
| `--no-contributions` | | Hide the contribution graph |
| `--avatar-width` | `-w` | Set avatar width in characters |
| `--max-weeks` | `-m` | Max weeks in contribution graph |
| `--nerd-font-icons` | | Use Nerd Font glyphs instead of emojis |

---

## Configuration

Create `~/.config/ghfetch/config.toml`:

```toml
username = "hustavojhon"

# GitHub token for higher API rate limits (5000 req/h)
# github_token = "ghp_xxxxxxxxxxxx"

[display]
show_avatar = true
show_system_info = true
show_header = true
show_greeting = true
show_contributions = true
nerd_font_icons = false
max_contrib_weeks = 53

[cache]
avatar_ttl_hours = 24
profile_ttl_hours = 1
contributions_ttl_hours = 2
cache_dir = ".cache/ghfetch"

[icons]
ghost = "👻"

[icons.overrides]
# os = "󰻀"
# kernel = "󰌽"
# cpu = "󰻠"
# ram = "󰘚"
```

See [config.example.toml](config.example.toml) for all options.

### Environment variables

| Variable | Description |
|----------|-------------|
| `GITHUB_TOKEN` | GitHub personal access token |
| `GHFETCH_USERNAME` | Default username |
| `GHFETCH_CONFIG` | Path to config file |

---

## Architecture

```
ghfetch/
├── src/
│   ├── main.rs           # CLI entry point, argument parsing
│   ├── theme.rs          # Gruvbox color palette, ANSI helpers, icons
│   ├── config.rs         # TOML config loading, defaults
│   ├── cache.rs          # File-based caching with TTL
│   ├── system.rs         # Cross-platform system info (sysinfo)
│   ├── github.rs         # GitHub REST & GraphQL API client
│   ├── avatar.rs         # Avatar download, resize, ASCII conversion
│   ├── greeting.rs       # Time-based dynamic greeting (Spanish)
│   ├── contributions.rs  # Contribution graph ASCII renderer
│   └── renderer.rs       # Responsive layout engine
├── Cargo.toml
├── config.example.toml
└── README.md
```

### Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `toml` + `serde` | Config file handling |
| `ureq` | HTTP client (GitHub API) |
| `image` | Avatar image processing |
| `sysinfo` | Cross-platform system information |
| `chrono` | Date/time handling |
| `terminal_size` | Terminal dimension detection |
| `dirs` | XDG directory resolution |

---

## Performance

- **First run**: ~500-1000ms (network requests + avatar processing)
- **Cached run**: ~100-150ms (all data from disk)
- **Binary size**: ~3.6 MB (includes image processing + TLS)

Caching strategy:
- Avatar images: 24h TTL
- GitHub profile data: 1h TTL
- Contribution data: 2h TTL
- Cache stored in `~/.cache/ghfetch/`

---

## Roadmap / Future Improvements

- [ ] Ghost/Pac-Man Nerd Font glyph discovery and mapping
- [ ] Additional theme presets (Nord, Catppuccin, Tokyo Night)
- [ ] Custom ASCII art loader for local avatars
- [ ] `--watch` mode with live updates
- [ ] JSON output mode for scripting
- [ ] iTerm2/Kitty inline image support (no ASCII conversion needed)
- [ ] Package manager distribution (AUR, Homebrew, Scoop)
- [ ] GitHub Actions integration for CI/CD badges
- [ ] Multi-panel layouts (compact, detailed, minimal)
- [ ] Language/tool stats from GitHub
- [ ] Customizable color palettes via config

---

## Why Rust?

| Factor | Rust |
|--------|------|
| **Startup time** | <100ms (no runtime) |
| **Binary size** | ~3.6MB stripped |
| **Cross-compilation** | First-class support |
| **Distribution** | Single static binary |
| **Memory safety** | Compile-time guarantees |
| **Ecosystem** | Mature CLI + image + HTTP crates |

---

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo build` and `cargo test`
5. Submit a pull request

---

## License

MIT © [Gustavo Jhon](https://github.com/hustavojhon)
