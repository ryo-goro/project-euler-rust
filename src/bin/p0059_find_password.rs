// Comma-separated list of ascii codes.
const ENCRYPTED_TEXT: &str = include_str!("../../data/p0059_cipher.txt");
const PASSWORD_LEN: usize = 3;

fn main() {
    let encrypted: Vec<u8> = ENCRYPTED_TEXT
        .trim()
        .split(',')
        .map(|ascii_code| ascii_code.parse().unwrap())
        .collect();

    for password in generate_passwords() {
        let decrypted = decrypt(&encrypted, &password);

        // The decrypted text must be valid UTF-8.
        let Ok(decrypted) = String::from_utf8(decrypted) else {
            continue;
        };

        // The text decrypted with the correct password must contain "The " or "the ".
        if decrypted.contains("The ") || decrypted.contains("the ") {
            let password = str::from_utf8(&password).unwrap();

            println!("\"{password}\": {decrypted}\n");
        }
    }
}

// Generate all lowercase alphabet passwords.
// The seed is treated as a base-26 number:
// 0 -> aaa, 1 -> aab, ..., 17575 -> zzz.
fn generate_passwords() -> impl Iterator<Item = [u8; PASSWORD_LEN]> {
    (0..26u32.pow(PASSWORD_LEN as u32))
        .map(|mut seed| {
            let mut password = [0u8; PASSWORD_LEN];

            for i in 0..PASSWORD_LEN {
                password[PASSWORD_LEN - 1 - i] = (seed % 26) as u8 + b'a';
                seed /= 26;
            }

            password
        })
}

fn decrypt(encrypted: &[u8], password: &[u8]) -> Vec<u8> {
    encrypted
        .iter()
        .enumerate()
        .map(|(i, &e)| e ^ password[i % password.len()])
        .collect()
}
