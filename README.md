
# raptor-cli 🦖

**Rust Authenticator Platform for Time-based OTP Retrieval** - A secure, minimalist TOTP CLI tool that stores secrets in your system keyring.

## Features

-   **Secure storage:** Secrets are stored in your operating system's native keyring.
-   **Cross-platform:** Works seamlessly across Windows, macOS, and Linux.
-   **Highly flexible:** Supports various TOTP configurations including SHA1, SHA256, SHA512 algorithms, and custom periods/skew values.

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

Add a secret with default parameters (SHA1, 6 digits, 30s period, 1 skew):
```bash
raptor-cli add github JBSWY3DPEHPK3PXP
```

**Add an account with custom parameters:**
Use the `--custom` flag to interactively set digits, algorithm, period, and skew.
```bash
raptor-cli add SSSWYY3DPEHPK3PXP --custom
```
You will be prompted for:
```
Configuring TOTP for custom-service (custom)
--------------------------------
Press ENTER to accept defaults.
Make sure your platform matches the chosen settings.
If you are unsure, use the default values.

Digits (6 or 8)    [6]: 
Period (seconds)   [30]: 
Skew (time periods) [1]: 
Algorithm (sha1|sha256|sha512) [sha1]: 

Stored secret for "custom-service" with parameters:
  Digits:    6
  Period:    30
  Skew:      1
  Algorithm: sha1
```

### Generate a code
```bash
raptor-cli code github
# Output: Code for github: 123456
```

### URI Mode
Generate a code directly from an `otpauth://` URI (parameters from URI take precedence):
```bash
raptor-cli code --uri "otpauth://totp/GitHub:user?secret=JBSWY3DPEHPK3PXP&issuer=GitHub"
```

### List accounts
```bash
raptor-cli list
```

### Remove an account
```bash
raptor-cli remove github
```

## Security

-   Secrets are securely stored in your system's native keyring.
-   No plaintext storage on disk.
-   TOTP codes are only generated and displayed when explicitly requested.
-   Secrets are validated for minimum length (≥128 bits) to prevent weak keys.

## Roadmap

### v0.2.0
-   [ ] Clipboard integration (auto-copy generated codes)
-   [ ] Configurable settings:
    -   [ ] Default auto-copy behavior
    -   [ ] Display time remaining for current code
-   [ ] Show time remaining for current code

### v0.3.0+
-   [ ] Export functionality for secure backups
-   [ ] Fuzzy search for account names

### Later...
-   [ ] TUI (Terminal User Interface) mode (future major version)

## License

MIT