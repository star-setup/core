# Star Setup

A lightweight CLI to clone, configure, and wire single or multi-repo ecosystems.

[![GitHub Release](https://img.shields.io/github/v/release/star-setup/core?include_prereleases&sort=semver)](https://github.com/star-setup/core/releases)
[![CI](https://github.com/star-setup/core/actions/workflows/ci.yml/badge.svg)](https://github.com/star-setup/core/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/github/license/star-setup/core)](./LICENSE)

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
# Clone and build via HTTPS
star-setup username/repo

# Clone and build via SSH
star-setup username/repo --ssh

# Common flags
star-setup username/repo --build-type Release
star-setup username/repo --build-dir out
star-setup username/repo --no-build
star-setup username/repo --clean
star-setup username/repo --verbose
star-setup username/repo --timing
star-setup username/repo --cmake-arg=-DCMAKE_CXX_COMPILER=clang++
star-setup username/repo --meson-arg=-Db_lto=true
```

### Mono-Repo Mode
Clones multiple repositories into a single workspace and auto-detects the build system. For CMake projects, generates a root `CMakeLists.txt` wiring all repositories as subdirectories. For Meson projects, generates a root `meson.build` and auto-generates local `.wrap` files bridging canonical dependency names to cloned directories.

```bash
# Manual repo list
star-setup username/repo --repos user/lib1 user/lib2

# Use a saved profile
star-setup username/repo --profile myprofile

# With SSH and custom directory
star-setup username/repo --repos user/lib1 user/lib2 --ssh --mono-dir my-workspace
```

#### Workspace Structure (CMake)
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

### Profile Mode
Profiles represent a saved ecosystem of libraries commonly used together.
```bash
# Add a profile
star-setup --profile-add myprofile user/lib1 user/lib2

# List profiles
star-setup --list-profiles

# Remove a profile
star-setup --profile-remove myprofile

# Use a profile
star-setup username/repo --profile myprofile
```

### Config Mode
Config files are checked in this order:
- `./.star-setup.json` (current directory)
- `~/.star-setup.json` (home directory)

```bash
# Initialize a default config file
star-setup --init-config

# Add a named config
star-setup --config-add myconfig --ssh --build-type Release

# List configs
star-setup --list-configs

# Remove a config
star-setup --config-remove myconfig

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
