// Std Library
use std::{
    error::Error,
    fs::File,
    io,
    io::{Read, Write},
    path::Path,
};

// Ext crates
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    {Aes256Gcm, Nonce},
};
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;
use sha2::{Digest, Sha256};

// Custom crates
use crate::mail_info::MailInfo;

fn derive_key_from_password(password: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

pub fn encrypt_to_file(
    password: &str,
    payload: &[u8],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let key = derive_key_from_password(password);
    let cipher =
        Aes256Gcm::new_from_slice(&key).map_err(|e| format!("Error creating cipher: {}", e))?;

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, payload)
        .map_err(|e| format!("Error encrypting payload: {}", e))?;

    let mut output = nonce_bytes.to_vec();
    output.extend(ciphertext);

    let encoded = general_purpose::STANDARD.encode(output);

    let mut file = File::create(filename)?;
    file.write_all(encoded.as_bytes())?;

    Ok(())
}

pub fn decrypt_from_file(password: &str, filename: &str) -> Result<MailInfo, Box<dyn Error>> {
    let path = Path::new(filename);
    if !path.exists() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "File not found",
        )));
    }

    let key = derive_key_from_password(password);
    let cipher =
        Aes256Gcm::new_from_slice(&key).map_err(|e| format!("Error creating cipher: {}", e))?;

    let mut file = File::open(filename)?;
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded)?;

    let decoded = general_purpose::STANDARD
        .decode(&encoded)
        .map_err(|e| format!("Error decoding base64: {}", e))?;

    let (nonce_bytes, ciphertext) = decoded.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = String::from_utf8(
        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Error decrypting payload: {}", e))?,
    )
    .unwrap();

    let info = plaintext.split("\n").collect::<Vec<&str>>();
    Ok(MailInfo {
        from: info[0].split("=").collect::<Vec<&str>>()[1].to_string(),
        from_pwd: info[1].split("=").collect::<Vec<&str>>()[1].to_string(),
        to: info[2].split("=").collect::<Vec<&str>>()[1].to_string(),
        subject: info[3].split("=").collect::<Vec<&str>>()[1].to_string(),
    })
}
