# PersianPKT

A modern package manager for Linux distributions focused on serving Iranian users with fast local repositories. Built with Rust for speed, reliability, and security.

## Features

- Powerful repository management
- Package installation, removal, and updating
- Automatic dependency resolution
- Fast and efficient caching system
- Smart mirror selection based on connection speed
- Security features including package verification
- User-friendly command-line interface

## Installation

Currently requires building from source:

```bash
git clone https://github.com/z0roday/persianpkt.git
cd persianpkt
cargo build --release
sudo cp target/release/persianpkt /usr/local/bin/pkt
```

## Usage

Install packages:
```bash
pkt install package-name
```

Remove packages:
```bash
pkt remove package-name
```

Update package lists:
```bash
pkt update
```

Upgrade installed packages:
```bash
pkt upgrade
```

Search for packages:
```bash
pkt search query
```

Show package information:
```bash
pkt show package-name
```

Manage repositories:
```bash
pkt repo add repo-name repo-url
pkt repo remove repo-name
pkt repo list
pkt repo enable repo-name
pkt repo disable repo-name
```

## Contributing

Want to help? Great!

1. Fork the project
2. Create your feature branch
3. Make your changes
4. Submit a pull request

## License

Released under the MIT License
