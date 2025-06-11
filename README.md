# Raptor 🦖

**Rust Authenticator Platform for Time-based OTP Retrieval** - A secure, minimalist TOTP CLI tool that stores secrets in your system keyring.

## Features

- **Secure storage** - Uses system keyring
- **Flexible** - Supports SHA1/SHA256/SHA512, custom digits/periods

## Installation

### From crates.io (recommended)
```bash
cargo install raptor-cli
```

### From source
```bash
git clone https://github.com/pap-stavros/raptor.git
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

- [ ] Clipboard integration
- [ ] QR code import from images
- [ ] Export functionality for backups
- [ ] Fuzzy search for account names
- [ ] Time remaining display
- [ ] TUI interface (future major version)

## License

MIT