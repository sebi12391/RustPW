use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

use anyhow::{bail, Result};
use yafo::pipeline::ProgressReporter;
use yafo::{Cipher, DecryptState, EncryptState, Pipeline};

const PASSWORD_FILE: &str = "passwords.txt";
const ENCRYPTED_PASSWORD_FILE: &str = "passwords.txt.SK";
const YAFO_FILE_EXTENSION: &str = ".SK";
const SILENT: bool = true;

pub fn encrypt_file(file_path: &str, key: &str, silent: bool) -> Result<()> {
    let path = Path::new(file_path);
    if !path.exists() {
        bail!("File not found: {}", path.display());
    }

    let pipeline = Pipeline::new().with_buffer();
    let encrypt = EncryptState::with_seed_phrase(key);
    if silent {
        pipeline.process_file(path, encrypt)?;
    } else {
        pipeline.process_file(path, encrypt)?;
    }

    let mut new_path = String::from(file_path);
    new_path.push_str(YAFO_FILE_EXTENSION);
    std::fs::rename(file_path, &new_path)?;

    Ok(())
}

pub fn decrypt_file(file_path: &str, key: &str, silent: bool) -> Result<()> {
    let path = Path::new(file_path);
    if !path.exists() {
        bail!("File not found: {}", path.display());
    }

    let pipeline = Pipeline::new().with_buffer();
    let decrypt = DecryptState::with_seed_phrase(key);
    if silent {
        pipeline.process_file(path, decrypt)?;
    } else {
        pipeline.process_file(path, decrypt)?;
    }

    if let Some(stripped) = file_path.strip_suffix(YAFO_FILE_EXTENSION) {
        std::fs::rename(file_path, stripped)?;
    }

    Ok(())
}
