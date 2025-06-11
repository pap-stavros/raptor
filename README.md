# raptor-cli 🦖

**Rust Authenticator Platform for Time-based OTP Retrieval** - A secure, minimalist TOTP CLI tool that stores secrets in your system keyring.

## Features

- **Secure storage** - Uses system keyring
- **Multiplatform** - Supports Windows/MacOS/Linux
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
raptor-cli add github JBSWY3DPEHPK3PXP
```

### Generate a code
```bash
raptor-cli code github
# Output: Code for github: 123456
```

### List accounts
```bash
raptor-cli list
```

### Remove an account
```bash
raptor-cli remove github
```

### Advanced options
```bash
# Custom algorithm, digits, period
raptor-cli code github --algorithm sha256 --digits 8 --period 60
```

## Security

- Secrets are stored in your system's secure keyring
- No plaintext storage on disk
- Codes are only shown when explicitly requested
- Validates secret length (≥128 bits) to prevent weak keys

## Roadmap

### v0.2.0
- [ ] Clipboard integration
- [ ] Customizable config file
      (auto copy, show time remaining etc)
- [ ] Show time remaining
### v0.3.0+
- [ ] Export functionality for backups
- [ ] Fuzzy search for account names

### Later...
- [ ] TUI interface (future major version)

## License

MIT