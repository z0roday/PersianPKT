# PersianPKT

A modern package manager for Linux distributions focusing on serving Iranian users

## About

PersianPKT is a powerful, modern package manager designed for Linux distributions. This tool was created to improve the experience of Iranian users when installing and managing software.

## Features

- Install and manage packages
- Support for multiple repositories
- Downloads with progress display
- Dependency management system
- Modern and user-friendly command line interface

## Development Requirements

To develop this project, you'll need:

- Rust 1.70.0 or higher
- C++ compiler (on Windows: Visual Studio Build Tools)

### Fixing linker error on Windows

If you encounter this error:
```
error: linker `link.exe` not found
```

Please install Visual Studio Build Tools:

1. Download and install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. During installation, select the "Desktop development with C++" component
3. After installation, open a new terminal and run:

```
cargo build
```

## Installation Instructions

```bash
# Clone the repository
git clone https://github.com/youruser/persianpkt.git
cd persianpkt

# Build the project
cargo build --release

# Install
cargo install --path .
```

## How to Use

```bash
# Search for a package
pkt search <package-name>

# Install a package
pkt install <package-name>

# Update packages
pkt update

# Manage repositories
pkt repo list
pkt repo add <repo-url>
```

## License

This project is released under the MIT license.
