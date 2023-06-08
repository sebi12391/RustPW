use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

const KEY_SIZE: usize = 32;
const SALT_SIZE: usize = 16;
const NONCE_SIZE: usize = 12;

pub fn encrypt_file(file_path: &str, password: &str, silent: bool) {
    let password_bytes = password.as_bytes();
    let salt = generate_salt();

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            if !silent {
                println!("Failed to open the file: {}", file_path);
            }
            return;
        }
    };

    let mut buffer = Vec::new();
    if let Err(_) = file.read_to_end(&mut buffer) {
        if !silent {
            println!("Failed to read the file: {}", file_path);
        }
        return;
    }

    let (key, nonce) = derive_key_and_nonce(password_bytes, &salt);
    let encrypted_data = encrypt_data(&buffer, &key, &nonce);

    let encrypted_file_path = format!("{}.SK", file_path);
    let mut encrypted_file = match File::create(&encrypted_file_path) {
        Ok(file) => file,
        Err(_) => {
            if !silent {
                println!("Failed to create the encrypted file: {}", encrypted_file_path);
            }
            return;
        }
    };

    let mut writer = BufWriter::new(&mut encrypted_file);
    if let Err(_) = writer.write_all(&salt) {
        if !silent {
            println!("Failed to write salt to the encrypted file: {}", encrypted_file_path);
        }
        return;
    }
    if let Err(_) = writer.write_all(&encrypted_data) {
        if !silent {
            println!("Failed to write encrypted data to the encrypted file: {}", encrypted_file_path);
        }
    }
}

pub fn decrypt_file(file_path: &str, password: &str, silent: bool) {
    let password_bytes = password.as_bytes();
    let salt = match read_salt_from_file(file_path, silent) {
        Some(salt) => salt,
        None => return,
    };

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            if !silent {
                println!("Failed to open the encrypted file: {}", file_path);
            }
            return;
        }
    };

    let mut encrypted_data = Vec::new();
    if let Err(_) = file.read_to_end(&mut encrypted_data) {
        if !silent {
            println!("Failed to read the encrypted file: {}", file_path);
        }
        return;
    }

    let (key, nonce) = derive_key_and_nonce(password_bytes, &salt);
    let decrypted_data = decrypt_data(&encrypted_data, &key, &nonce);

    let decrypted_file_path = file_path.trim_end_matches(".SK");
    let mut decrypted_file = match File::create(decrypted_file_path) {
        Ok(file) => file,
        Err(_) => {
            if !silent {
                println!("Failed to create the decrypted file: {}", decrypted_file_path);
            }
            return;
        }
    };

    if let Err(_) = decrypted_file.write_all(&decrypted_data) {
        if !silent {
            println!("Failed to write decrypted data to the decrypted file: {}", decrypted_file_path);
        }
    }
}

pub fn verify_master_password(password: &str) -> bool {
    let encrypted_file_path = format!("{}.SK", PASSWORD_FILE);
    let password_bytes = password.as_bytes();
    let salt = match read_salt_from_file(&encrypted_file_path, true) {
        Some(salt) => salt,
        None => return false,
    };

    let (key, nonce) = derive_key_and_nonce(password_bytes, &salt);
    let decrypted_data = decrypt_data_from_file(&encrypted_file_path, &key, &nonce, true);

    decrypted_data.is_some()
}

fn generate_salt() -> [u8; SALT_SIZE] {
    rand::random()
}

fn derive_key_and_nonce(password: &[u8], salt: &[u8; SALT_SIZE]) -> ([u8; KEY_SIZE], [u8; NONCE_SIZE]) {
    let mut key = [0; KEY_SIZE];
    let mut nonce = [0; NONCE_SIZE];
    argon2::hash_raw(password, salt, &mut key, &mut nonce, &argon2::Config::default(), argon2::Version::Version13).expect("Failed to derive key and nonce.");

    (key, nonce)
}

fn encrypt_data(data: &[u8], key: &[u8; KEY_SIZE], nonce: &[u8; NONCE_SIZE]) -> Vec<u8> {
    let cipher = aes_gcm::Aes256Gcm::new(GenericArray::from_slice(key));
    let mut encrypted_data = Vec::new();
    let ciphertext = cipher.encrypt(nonce.into(), data).expect("Failed to encrypt data.");
    encrypted_data.extend_from_slice(&ciphertext);

    encrypted_data
}

fn decrypt_data(encrypted_data: &[u8], key: &[u8; KEY_SIZE], nonce: &[u8; NONCE_SIZE]) -> Vec<u8> {
    let cipher = aes_gcm::Aes256Gcm::new(GenericArray::from_slice(key));
    let decrypted_data = cipher.decrypt(nonce.into(), encrypted_data).expect("Failed to decrypt data.");

    decrypted_data
}

fn read_salt_from_file(file_path: &str, silent: bool) -> Option<[u8; SALT_SIZE]> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            if !silent {
                println!("Failed to open the file: {}", file_path);
            }
            return None;
        }
    };

    let mut salt = [0; SALT_SIZE];
    if let Err(_) = file.read_exact(&mut salt) {
        if !silent {
            println!("Failed to read salt from the file: {}", file_path);
        }
        return None;
    }

    Some(salt)
}

fn decrypt_data_from_file(file_path: &str, key: &[u8; KEY_SIZE], nonce: &[u8; NONCE_SIZE], silent: bool) -> Option<Vec<u8>> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            if !silent {
                println!("Failed to open the file: {}", file_path);
            }
            return None;
        }
    };

    let mut encrypted_data = Vec::new();
    if let Err(_) = file.read_to_end(&mut encrypted_data) {
        if !silent {
            println!("Failed to read the file: {}", file_path);
        }
        return None;
    }

    let decrypted_data = decrypt_data(&encrypted_data, key, nonce);

    Some(decrypted_data)
}
