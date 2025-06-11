# Raptor 🦖

A secure, minimalist TOTP (Time-based One-Time Password) CLI tool that stores secrets in your system keyring.

## Features

- **Secure storage** - Uses system keyring (no plaintext files)
- **Privacy-focused** - Never displays all codes at once
- **Cross-platform** - Works on Windows, macOS, and Linux
- **Flexible** - Supports SHA1/SHA256/SHA512, custom digits/periods
- **Simple** - Clean CLI interface, no GUI bloat

## Installation

```bash
# Clone and build
git clone https://github.com/yourusername/raptor.git
cd raptor
cargo install --path .
```

## Usage

### Add a new account
```bash
raptor add github JBSWY3DPEHPK3PXP
```

### Generate a code
```bash
raptor code github
# Output: Code for github: 123456
```

### List accounts
```bash
raptor list
```

### Remove an account
```bash
raptor remove github
```

### Advanced options
```bash
# Custom algorithm, digits, period
raptor code github --algorithm sha256 --digits 8 --period 60
```

## Security

- Secrets are stored in your system's secure keyring
- No plaintext storage on disk
- Codes are only shown when explicitly requested
- Validates secret length (≥128 bits) to prevent weak keys

## Roadmap

- [ ] Clipboard integration (`--copy` flag)
- [ ] QR code import from images
- [ ] Export functionality for backups
- [ ] Fuzzy search for account names
- [ ] Time remaining display
- [ ] TUI interface (future major version)

## License

MIT