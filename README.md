# PersianPKT

A modern package manager designed for Linux distributions with a focus on serving Iranian users.

## What is PersianPKT?

PersianPKT is a powerful package management system built with Rust that makes software installation and management easier on Linux systems. It's specially designed to provide fast, reliable access to software repositories for users in Iran.

## Key Features

- Fast and efficient package installation and removal
- Smart repository management with mirror selection
- Dependency tracking and resolution
- Clean and intuitive command-line interface
- Progress visualization for downloads and installations
- Configurable package sources
- Secure package verification

## Platform Support

PersianPKT is designed to run on Linux distributions. While the code can be developed on other platforms like Windows, the tool itself is intended for use on Linux systems.

## Development Setup

### Linux (Recommended)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone https://github.com/youruser/persianpkt.git
cd persianpkt

# Build the project
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .
```

### Windows (Development Only)

For Windows users who want to contribute to the code:

1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the "Desktop development with C++" workload
2. Install Rust from [rustup.rs](https://rustup.rs/)
3. Build with `cargo build`

Alternatively, use WSL (Windows Subsystem for Linux) for a more native Linux development experience.

## Usage Examples

```bash
# Update repository information
pkt update

# Search for a package
pkt search firefox

# Install a package
pkt install firefox

# Remove a package
pkt remove firefox

# List installed packages
pkt list

# Add a new repository
pkt repo add myrepo https://example.com/repo
```

## Configuration

PersianPKT stores its configuration in `~/.config/persianpkt/`. You can modify repository sources, priorities, and other settings there.

## Contributing

We welcome contributions! Please feel free to submit a Pull Request.

## License

This project is released under the MIT License.
