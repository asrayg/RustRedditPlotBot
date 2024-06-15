use clap::{Arg, Command};
use ring::aead;
use ring::pbkdf2;
use ring::rand::{SecureRandom, SystemRandom};
use ring::digest::SHA256;
use std::fs::{File, read};
use std::io::{Read, Write};
use std::num::NonZeroU32;

const PBKDF2_ITERATIONS: u32 = 100_000;
const SALT_LEN: usize = 16;
const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12;

fn main() {
    let matches = Command::new("File Encryption Tool")
        .version("1.0")
        .author("Asray Gopa <asray@iastate.edu>")
        .about("Encrypts and decrypts files")
        .subcommand(
            Command::new("encrypt")
                .about("Encrypts a file")
                .arg(Arg::new("input")
                    .short('i')
                    .long("input")
                    .value_name("FILE")
                    .about("Sets the input file to use")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .about("Sets the output file to use")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::new("key")
                    .short('k')
                    .long("key")
                    .value_name("KEY")
                    .about("Sets the encryption key")
                    .required(true)
                    .takes_value(true)),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypts a file")
                .arg(Arg::new("input")
                    .short('i')
                    .long("input")
                    .value_name("FILE")
                    .about("Sets the input file to use")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .about("Sets the output file to use")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::new("key")
                    .short('k')
                    .long("key")
                    .value_name("KEY")
                    .about("Sets the decryption key")
                    .required(true)
                    .takes_value(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("encrypt") {
        let input = matches.value_of("input").unwrap();
        let output = matches.value_of("output").unwrap();
        let key = matches.value_of("key").unwrap();
        encrypt_file(input, output, key);
    } else if let Some(matches) = matches.subcommand_matches("decrypt") {
        let input = matches.value_of("input").unwrap();
        let output = matches.value_of("output").unwrap();
        let key = matches.value_of("key").unwrap();
        decrypt_file(input, output, key);
    }
}

fn encrypt_file(input: &str, output: &str, key: &str) {
    let input_data = read(input).expect("Failed to read input file");

    let mut salt = [0u8; SALT_LEN];
    let rng = SystemRandom::new();
    rng.fill(&mut salt).expect("Failed to generate salt");

    let mut key_bytes = [0u8; KEY_LEN];
    pbkdf2::derive(SHA256, NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(), &salt, key.as_bytes(), &mut key_bytes);

    let sealing_key = aead::LessSafeKey::new(aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes).expect("Failed to create sealing key"));

    let mut nonce = [0u8; NONCE_LEN];
    rng.fill(&mut nonce).expect("Failed to generate nonce");

    let mut in_out = input_data.clone();
    in_out.resize(input_data.len() + sealing_key.algorithm().tag_len(), 0);

    sealing_key.seal_in_place_append_tag(aead::Nonce::assume_unique_for_key(nonce), aead::Aad::empty(), &mut in_out)
        .expect("Failed to encrypt data");

    let mut output_file = File::create(output).expect("Failed to create output file");
    output_file.write_all(&salt).expect("Failed to write salt");
    output_file.write_all(&nonce).expect("Failed to write nonce");
    output_file.write_all(&in_out).expect("Failed to write encrypted data");
}

fn decrypt_file(input: &str, output: &str, key: &str) {
    let mut input_file = File::open(input).expect("Failed to open input file");

    let mut salt = [0u8; SALT_LEN];
    input_file.read_exact(&mut salt).expect("Failed to read salt");

    let mut key_bytes = [0u8; KEY_LEN];
    pbkdf2::derive(SHA256, NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(), &salt, key.as_bytes(), &mut key_bytes);

    let opening_key = aead::LessSafeKey::new(aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes).expect("Failed to create opening key"));

    let mut nonce = [0u8; NONCE_LEN];
    input_file.read_exact(&mut nonce).expect("Failed to read nonce");

    let mut encrypted_data = Vec::new();
    input_file.read_to_end(&mut encrypted_data).expect("Failed to read encrypted data");

    let decrypted_data = opening_key.open_in_place(aead::Nonce::assume_unique_for_key(nonce), aead::Aad::empty(), &mut encrypted_data)
        .expect("Failed to decrypt data");

    let mut output_file = File::create(output).expect("Failed to create output file");
    output_file.write_all(decrypted_data).expect("Failed to write decrypted data");
}
