// Comma-separated list of ascii codes.
const ENCRYPTED_TEXT: &str = include_str!("../../data/p0059_cipher.txt");

// The password ("exp") was obtained using the helper program:
//     cargo run --bin p0059_find_password
const PASSWORD: &[u8] = b"exp";

pub fn solve() -> i64 {
    let encrypted: Vec<u8> = ENCRYPTED_TEXT
        .trim()
        .split(',')
        .map(|ascii_code| ascii_code.parse().unwrap())
        .collect();

    let decrypted = decrypt(&encrypted, PASSWORD);

    decrypted
        .iter()
        .map(|&c| c as i64)
        .sum()
}

fn decrypt(encrypted: &[u8], password: &[u8]) -> Vec<u8> {
    encrypted
        .iter()
        .enumerate()
        .map(|(i, &e)| e ^ password[i % password.len()])
        .collect()
}
