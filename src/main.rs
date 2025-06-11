use anyhow::{anyhow, Context};
use base32::Alphabet;
use clap::{Parser, Subcommand};
use dirs::config_dir;
use keyring::Entry;
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};
use totp_rs::{Algorithm, TOTP};

#[derive(Parser)]
#[command(name = "raptor", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Store a new Base32-encoded secret (length ≥128 bits)
    Add {
        account: String,
        secret: String,
    },
    /// Delete a stored secret
    Remove {
        account: String,
    },
    /// List stored accounts
    List,
    /// Generate the current TOTP code for an account
    Code {
        account: String,
        #[arg(long, default_value_t = 6)]
        digits: usize,
        #[arg(long, default_value_t = 30)]
        period: u64,
        #[arg(long, default_value = "sha1")]
        algorithm: String,
    },
}

fn parse_algo(s: &str) -> Result<Algorithm, anyhow::Error> {
    match s.to_ascii_lowercase().as_str() {
        "sha1" => Ok(Algorithm::SHA1),
        "sha256" => Ok(Algorithm::SHA256),
        "sha512" => Ok(Algorithm::SHA512),
        _ => Err(anyhow!("unsupported algorithm: {}", s)),
    }
}

/// Try Base32 decode with padding=true, then padding=false.
fn decode_secret(s: &str) -> Result<Vec<u8>, anyhow::Error> {
    base32::decode(Alphabet::Rfc4648 { padding: true }, s)
        .or_else(|| base32::decode(Alphabet::Rfc4648 { padding: false }, s))
        .ok_or_else(|| anyhow!("invalid Base32 secret"))
}

fn accounts_file() -> anyhow::Result<PathBuf> {
    let mut dir = config_dir().ok_or_else(|| anyhow!("could not find config dir"))?;
    dir.push("raptor");
    fs::create_dir_all(&dir)?;
    dir.push("accounts");
    Ok(dir)
}

fn index_add(account: &str) -> anyhow::Result<()> {
    let path = accounts_file()?;
    let mut seen = false;
    if path.exists() {
        for line in BufReader::new(fs::File::open(&path)?).lines() {
            if line?.trim() == account {
                seen = true;
                break;
            }
        }
    }
    if !seen {
        let mut f = OpenOptions::new().create(true).append(true).open(&path)?;
        writeln!(f, "{}", account)?;
    }
    Ok(())
}

fn index_remove(account: &str) -> anyhow::Result<()> {
    let path = accounts_file()?;
    if !path.exists() {
        return Ok(());
    }
    let lines: Vec<_> = BufReader::new(fs::File::open(&path)?)
        .lines()
        .filter_map(Result::ok)
        .filter(|l| l.trim() != account)
        .collect();
    fs::write(&path, lines.join("\n") + "\n")?;
    Ok(())
}

fn list_accounts() -> anyhow::Result<()> {
    let path = accounts_file()?;
    if !path.exists() {
        println!("(no accounts)");
        return Ok(());
    }
    for line in BufReader::new(fs::File::open(&path)?).lines() {
        println!("{}", line?);
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let service = "raptor";

    match cli.command {
        Commands::Add { account, secret } => {
            // Validate Base32 + length ≥128 bits
            let key = decode_secret(&secret).context("invalid Base32 secret")?;
            if key.len() < 16 {
                anyhow::bail!(
                    "secret too short: got {} bytes ({} bits), need ≥16 bytes (128 bits)\n\
                     Are you sure this is a valid Base32-encoded TOTP secret?",
                    key.len(),
                    key.len() * 8
                );
            }
            Entry::new(service, &account)?
                .set_password(&secret)
                .context("writing secret to keyring")?;
            index_add(&account).context("updating account index")?;
            println!("Stored secret for \"{}\"", account);
        }
        Commands::Remove { account } => {
            Entry::new(service, &account)?
                .delete_password()
                .context("deleting secret from keyring")?;
            index_remove(&account).context("updating account index")?;
            println!("Removed secret for \"{}\"", account);
        }
        Commands::List => {
            list_accounts().context("listing accounts")?;
        }
        Commands::Code {
            account,
            digits,
            period,
            algorithm,
        } => {
            let secret_str = Entry::new(service, &account)?
                .get_password()
                .context("no secret found for that account")?;
            let key = decode_secret(&secret_str)
                .context("invalid Base32 secret in keyring")?;
            let algo = parse_algo(&algorithm)?;
            let totp = TOTP::new(algo, digits, 1, period, key)
                .context("configuring TOTP")?;
            let code = totp.generate_current().context("generating code")?;
            println!("Code for {}: {}", account, code);
        }
    }

    Ok(())
}