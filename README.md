# Star Setup

A lightweight CLI to clone, configure, and wire single or multi-repo ecosystems.

[![GitHub Release](https://img.shields.io/github/v/release/star-setup/core?include_prereleases&sort=semver)](https://github.com/star-setup/core/releases)
[![CI](https://github.com/star-setup/core/actions/workflows/ci.yml/badge.svg)](https://github.com/star-setup/core/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/github/license/star-setup/core)](./LICENSE)

> **Note:** This tool is primarily designed for my own projects and workflows. While it may work for other ecosystems, it is not guaranteed to work with all project structures or build configurations.

## Quick Start

```bash
# Interactive mode
star-setup

# Single-repo mode
star-setup username/repo

# Mono-repo mode
star-setup username/repo --repos user/lib1 user/lib2
```

## Prerequisites
- Git
- At least one supported build system:
  - CMake
  - Meson
  - npm (Node.js 22+)

## Installation

Download the latest binary from [Releases](https://github.com/star-setup/core/releases), or:

### Homebrew (macOS/Linux)
```bash
brew install star-setup/tap/star-setup
```

### npm
```bash
npm install -g @star-setup/star-setup
```

### Shell (Linux/macOS)
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/star-setup/core/releases/latest/download/star-setup-installer.sh | sh
```

### PowerShell (Windows)
```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/star-setup/core/releases/latest/download/star-setup-installer.ps1 | iex"
```

### pip
```bash
pip install star-setup
```

### Windows Installer
Download the `.msi` from [Releases](https://github.com/star-setup/core/releases).

### Build from source
```bash
cargo install --git https://github.com/star-setup/core
```

## Usage

### Flags
#### Connection
| Flag | Description |
|------|-------------|
| `--ssh` | Clone via SSH instead of HTTPS |
| `--https` | Force HTTPS (default) |
| `--verbose` | Print commands as they run |

#### Build
| Flag | Description |
|------|-------------|
| `--build-type <TYPE>` | Build type: `Debug` (default) or `Release` |
| `--build-dir <DIR>` | Build output directory (default: `build`) |
| `--build-system <SYSTEM>` | Skip auto-detection and use `cmake`, `meson`, or `npm` |
| `--no-build` | Configure only, skip build step |
| `--clean` | Remove build directory before configuring |
| `--cmake-arg <ARG>` | Pass additional argument to CMake |
| `--meson-arg <ARG>` | Pass additional argument to Meson |
| `--watch` | Generate and open watch scripts (npm mono-repo mode) |
| `--no-watch` | Skip generating watch scripts (npm mono-repo mode) |


#### Mono-Repo
| Flag | Description |
|------|-------------|
| `--repos <REPOS>...` | List of dependency repositories |
| `--mono-dir <DIR>` | Workspace directory (default: `build-mono`) |
| `--profile <NAME>` | Use a saved profile |

#### Diagnostic
| Flag | Description |
|------|-------------|
| `--dry-run` | Print what would happen without making any changes |
| `--timing` | Show timing for each phase |

### Interactive Mode
Running `star-setup` without arguments launches interactive mode, guiding you through all options.

```
Star Setup Interactive Mode
Enter repository (user/repo or URL): user/repo
Use SSH? (y/n) [N]:
Verbose? (y/n) [N]:
Clean build directory if exists? (y/n) [N]:
Select mode: (1) Single Repo (2) Mono-Repo: 1
Build type [Debug]:
Build directory [build]:
Configure only (skip build)? (y/n) [N]:

Interactive mode complete
```

### Single Repository Mode
```bash
# Clone and build using a single repository
star-setup username/repo
```

Build system is auto-detected from the repository root (`CMakeLists.txt` → CMake, `meson.build` → Meson, `package.json` → npm).

### Mono-Repo Mode
Clones multiple repositories into a single workspace and auto-detects the build system.

```bash
# Clone and build a test repo and a manual repo list
star-setup username/repo --repos user/lib1 user/lib2

# Clone and build a test repo and a saved profile
star-setup username/repo --profile myprofile
```

#### Workspace Structure (CMake)
Generates a root `CMakeLists.txt` wiring all repositories as subdirectories

```
build-mono/
├── CMakeLists.txt  # Auto-generated root project
├── repos/
│   ├── user-my-repo/ # Test repository
│   ├── user-lib1/
│   └── user-lib2/
└── build/          # Build output
```

##### BUILD_LOCAL
Mono-repo mode sets `-DBUILD_LOCAL=ON` when configuring CMake. This flag tells your test repository to link against local module directories instead of fetching them remotely via FetchContent:
```cmake
# In your test repo's CMakeLists.txt
if(NOT BUILD_LOCAL)
  FetchContent_Declare(mylib
    GIT_REPOSITORY https://github.com/user/mylib.git
    GIT_TAG main
  )
endif()
```
This allows the same repository to work both standalone (fetching dependencies automatically) and inside a mono-repo workspace (linking locally for full cross-module debugging).

#### Workspace Structure (Meson)
Generates a root `meson.build` and auto-generates local `.wrap` files bridging canonical dependency names to cloned directories.

```
build-mono/
├── meson.build     # Auto-generated root project
├── repos/
│   ├── user-my-repo/ # Test repository
│   ├── user-lib1/
│   ├── user-lib2/
│   ├── lib1.wrap     # Auto-generated local wrap
│   └── lib2.wrap     # Auto-generated local wrap
└── build/          # Build output
```

#### Workspace Structure (Npm)
Generates a workspace root `package.json` that wires all cloned repositories together using npm workspaces and forces local resolution via `overrides` — no changes to individual repository files. Each lib's `package.json` is read after cloning to extract its package name for the overrides map.

```
build-mono/
├── package.json    # Auto-generated workspace root with workspaces + overrides
├── watch.ps1       # Auto-generated watch script (Windows)
├── watch.sh        # Auto-generated watch script (Linux/macOS)
└── repos/
    ├── user-my-repo/ # Test repository
    ├── user-lib1/
    └── user-lib2/
```

Watch scripts are generated by default and run each lib's `watch` script if present, falling back to `build -- --watch`. Use `--watch` to open them automatically in new terminals, or `--no-watch` to skip generation entirely.

```bash
# Generate workspace and open watchers
star-setup username/repo --repos user/lib1 user/lib2 --watch

# Generate workspace without opening watchers
star-setup username/repo --repos user/lib1 user/lib2

# Skip watch script generation
star-setup username/repo --repos user/lib1 user/lib2 --no-watch
```

After setup, run the game from its repo directory:
```bash
cd build-mono/repos/user-my-repo
npm run dev
```

#### Workspace Mode
Manage an existing mono-repo workspace.

```bash
# Pull latest changes for all repos
star-setup workspace update

# Show status of all repos
star-setup workspace status

# Show status with ahead/behind remote
star-setup workspace status --fetch

# Remove build directory
star-setup workspace clean
```

Workspace flags:
| Flag | Description |
|------|-------------|
| `--path <DIR>` | Workspace root directory (default: current directory) |
| `--mono-dir <DIR>` | Workspace directory name (default: `build-mono`) |
| `--build-dir <DIR>` | Build directory name (default: `build`) |

### Profile Mode
Profiles represent a saved ecosystem of libraries commonly used together.
```bash
# Add a profile
star-setup profile add myprofile user/lib1 user/lib2

# List profiles
star-setup profile list

# Remove a profile
star-setup profile remove myprofile

# Use a profile
star-setup username/repo --profile myprofile
```

### Config Mode
Config files are checked in this order:
- `./.star-setup.json` (current directory)
- `~/.star-setup.json` (home directory)

```bash
# Initialize a default config file
star-setup config init

# Add a named config
star-setup config add myconfig --ssh --build-type Release

# List configs
star-setup config list

# Remove a config
star-setup config remove myconfig

# Use a config
star-setup username/repo --config myconfig
```

### Development
```bash
git clone https://github.com/star-setup/core
cd core
cargo test
cargo clippy --all-targets -- -D warnings
```

## License
MIT License — see [LICENSE](./LICENSE) for details.
