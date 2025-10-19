use aes_gcm::{
    AeadCore, Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng},
};
use base64::{Engine as _, engine::general_purpose};

const SECRET_KEY: &[u8; 32] = b"E4xKTDh!jWN2AQVi4%euE6sS#9w@*j3j";

#[allow(dead_code)]
fn encrypt(data: &str) -> Option<String> {
    let cipher = Aes256Gcm::new(SECRET_KEY.into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    if let Ok(ciphertext) = cipher.encrypt(&nonce, data.as_bytes()) {
        let mut result = Vec::new();
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        return Some(general_purpose::STANDARD.encode(result));
    }
    None
}

#[allow(deprecated)]
pub fn decrypt(token: &str) -> Option<String> {
    if let Ok(data) = general_purpose::STANDARD.decode(token) {
        if data.len() < 12 {
            return None;
        }
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let cipher = Aes256Gcm::new(SECRET_KEY.into());

        if let Ok(plaintext) = cipher.decrypt(Nonce::from_slice(nonce_bytes), ciphertext) {
            return Some(String::from_utf8_lossy(&plaintext).to_string());
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_token() {
        let test_token = "kkkkkkknnnnnnooooooxxxxx";
        println!("origin token: {test_token}");
        let _en = encrypt(test_token);
        if let Some(_e) = _en {
            println!("encrypted token: {_e}");
            if let Some(_de) = decrypt(&_e) {
                assert_eq!(test_token, _de);
            }
        }
    }
}
