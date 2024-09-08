use regex::Regex;

/// Stellar public key formatını kontrol eden bir fonksiyon
pub fn is_valid_stellar_address(address: &str) -> bool {
    // Stellar public keyleri genellikle G ile başlar ve 56 karakter uzunluğundadır
    let re = Regex::new(r"^G[A-Z0-9]{55}$").unwrap();
    re.is_match(address)
}

/// Kullanıcıdan giriş almak için basit bir fonksiyon
pub fn get_user_input(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Girdi alınamadı");

    input.trim().to_string()
}
