use bip39::{Language, Mnemonic};
use hdwallet::{ExtendedPrivKey, KeyIndex};
use k256::ecdsa::SigningKey;
use tiny_keccak::{Hasher, Keccak};
use std::sync::{Arc, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::time::Instant;
use std::env;
use rand::RngCore;
use ctrlc;
use colored::*;
use std::io::{self, Write};

fn eth_address_from_pubkey(pubkey: &[u8]) -> String {
    let mut hasher = Keccak::v256();
    let mut hash = [0u8; 32];
    hasher.update(&pubkey[1..]); // skip 0x04 prefix
    hasher.finalize(&mut hash);
    format!("0x{}", hex::encode(&hash[12..]))
}

fn ratio_of_numbers_or_letters(s: &str) -> f64 {
    let s = s.trim_start_matches("0x");
    let len = s.len() as f64;
    if len == 0.0 { return 0.0; }
    let digits = s.chars().filter(|c| c.is_ascii_digit()).count() as f64;
    let letters = s.chars().filter(|c| c.is_ascii_alphabetic()).count() as f64;
    digits.max(letters) / len
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prefix = args.iter().position(|x| x == "--prefix").and_then(|i| args.get(i+1)).map(|s| s.to_lowercase());
    let clean_mode = args.iter().any(|x| x == "--clean");
    let clean_ratio = args.iter().position(|x| x == "--clean-ratio").and_then(|i| args.get(i+1)).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.92);
    if args.iter().any(|x| x == "--help") {
        println!("Usage: eth-wallet-generator [--prefix <prefix>] [--clean] [--clean-ratio <0.0-1.0>]\n\
    --prefix <prefix>         Find addresses with this prefix (default: 0x1337)\n\
    --clean                   Find addresses with high ratio of numbers or letters\n\
    --clean-ratio <float>     Ratio threshold for --clean mode (default: 0.92)\n");
        return;
    }
    if clean_mode {
        println!("Searching for Ethereum addresses with >={:.0}% numbers or letters", clean_ratio * 100.0);
    } else {
        let prefix = prefix.clone().unwrap_or("0x1337".to_string());
        println!("Searching for Ethereum address with prefix: {}", prefix);
    }

    let running = Arc::new(AtomicBool::new(true));
    let counter = Arc::new(AtomicU64::new(0));
    let start = Instant::now();

    let r = Arc::clone(&running);
    ctrlc::set_handler(move || {
        r.store(false, Ordering::Relaxed);
    }).expect("Error setting Ctrl-C handler");

    rayon::scope(|s| {
        for _ in 0..num_cpus::get() {
            let running = Arc::clone(&running);
            let counter = Arc::clone(&counter);
            let clean_mode = clean_mode;
            let clean_ratio = clean_ratio;
            let prefix = prefix.clone();
            s.spawn(move |_| {
                let mut rng = rand::thread_rng();
                while running.load(Ordering::Relaxed) {
                    let mut entropy = [0u8; 16];
                    rng.fill_bytes(&mut entropy);
                    let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
                    let seed = mnemonic.to_seed("");
                    // Derive m/44'/60'/0'/0/0
                    let master = ExtendedPrivKey::with_seed(&seed).unwrap();
                    let purpose = master.derive_private_key(KeyIndex::hardened_from_normalize_index(44).unwrap()).unwrap();
                    let coin = purpose.derive_private_key(KeyIndex::hardened_from_normalize_index(60).unwrap()).unwrap();
                    let account = coin.derive_private_key(KeyIndex::hardened_from_normalize_index(0).unwrap()).unwrap();
                    let change = account.derive_private_key(KeyIndex::Normal(0)).unwrap();
                    let address_index = change.derive_private_key(KeyIndex::Normal(0)).unwrap();
                    let signing_key = SigningKey::from_bytes(address_index.private_key.as_ref().into()).unwrap();
                    let pubkey = signing_key.verifying_key().to_encoded_point(false);
                    let address = eth_address_from_pubkey(pubkey.as_bytes());
                    let is_match = if clean_mode {
                        ratio_of_numbers_or_letters(&address) >= clean_ratio
                    } else {
                        prefix.as_ref().map(|p| address.to_lowercase().starts_with(p)).unwrap_or(false)
                    };
                    if is_match {
                        if clean_mode {
                            let ratio = ratio_of_numbers_or_letters(&address);
                            println!(
                                "\n{}\n{} {}\n{} {}\n{} {:.2}\n",
                                "================ MATCH ================".bold().blue(),
                                "Address:".bold().green(),
                                address.bold().green(),
                                "Mnemonic:".bold().yellow(),
                                mnemonic.to_string().yellow(),
                                "Ratio:".bold().magenta(),
                                ratio
                            );
                        } else {
                            println!(
                                "\n{}\n{} {}\n{} {}\n",
                                "================ MATCH ================".bold().blue(),
                                "Address:".bold().green(),
                                address.bold().green(),
                                "Mnemonic:".bold().yellow(),
                                mnemonic.to_string().yellow()
                            );
                        }
                    }
                    let total = counter.fetch_add(1, Ordering::Relaxed) + 1;
                    if total % 1000 == 0 {
                        print!("\rWallets checked: {}", total);
                        io::stdout().flush().unwrap();
                    }
                }
            });
        }
    });
    let elapsed = start.elapsed().as_secs_f64();
    let total = counter.load(Ordering::Relaxed);
    println!("\nTotal time: {:.2} seconds", elapsed);
    println!("Wallets per second: {:.0}", total as f64 / elapsed);
}
