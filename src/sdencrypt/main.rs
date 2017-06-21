extern crate crypto;
extern crate rand;
extern crate rpassword;

mod pkcs7;

use crypto::symmetriccipher::*;
use crypto::aessafe::*;
use crypto::scrypt;
use std::io;
use std::io::Read;
use std::process;

use pkcs7::*;

fn pretty_print_hex(data: &[u8], bytes_per_line: usize) {
    for chunk in data.chunks(bytes_per_line) {
        for byte in chunk.iter() {
            print!("{:02X} ", byte);
        }
        print!("\n");
    }
}

pub fn main() {

    let mut pass: String;
    loop {
        pass = match rpassword::prompt_password_stderr("Choose a password: ") {
            Ok(input) => input,
            Err(_) => {
                println!("Something's wrong with your input. Please try again!");
                continue;
            }
        };

        let confirm = match rpassword::prompt_password_stderr("Confirm: ") {
            Ok(input) => input,
            Err(_) => {
                println!("Something's wrong with your input. Please try again!");
                continue;
            }
        };

        if pass.len() < 5 {
            println!("Your password's length is less than 5, please try something longer!");
            continue;
        }

        if pass == confirm {
            break;
        } else {
            println!("Passwords don't match, please try again!");
        }
    }

    println!(
        "Generating key with password {}...{}",
        pass.chars().nth(0).unwrap(),
        pass.chars().nth(pass.len() - 1).unwrap()
    );

    let mut key: [u8; 32] = [0; 32];

    // Default scrypt params: 14, 8, 1
    let params = scrypt::ScryptParams::new(1, 1, 1);
    scrypt::scrypt(pass.as_bytes(), b"", &params, &mut key);

    let encryptor = AesSafe256Encryptor::new(&key);
    let block_size = encryptor.block_size();

    let mut input: Vec<u8> = vec![0; 0];
    println!("Input the data to encrypt (press Ctrl+D when you're done):");
    let bytes_read = match io::stdin().read_to_end(&mut input) {
        Err(e) => panic!("Could not read input! ({:#?})", e),
        Ok(n) => {
            print!("\nRead {} bytes", n);
            n
        }
    };
    println!(" ({} whole blocks)", bytes_read / block_size);

    println!("Plain bytes (unpadded):");
    pretty_print_hex(&input, block_size);
    print!("\n");

    pad_pkcs7(&mut input, block_size);

    println!("Plain bytes (padded):");
    pretty_print_hex(&input, block_size);
    print!("\n");

    println!("Key:");
    pretty_print_hex(&key, block_size);
    print!("\n");

    let mut encrypted: Vec<u8> = vec![0; 0];

    for chunk in input.chunks(block_size) {
        let mut current_cipher_block = [0u8; 16];
        encryptor.encrypt_block(&chunk, &mut current_cipher_block);
        encrypted.extend_from_slice(&mut current_cipher_block);
    }

    println!("Encrypted:");
    pretty_print_hex(&encrypted, block_size);
    print!("\n");

    let decryptor = AesSafe256Decryptor::new(&key);

    let mut decrypted: Vec<u8> = vec![0; 0];

    for chunk in encrypted.chunks(block_size) {
        let mut current_plain_block = [0u8; 16];
        decryptor.decrypt_block(&chunk, &mut current_plain_block);
        decrypted.extend_from_slice(&mut current_plain_block);
    }

    println!("Decrypted (padded):");
    pretty_print_hex(&decrypted, block_size);
    print!("\n");

    if let Err(_) = unpad_pkcs7(&mut decrypted, block_size) {
        println!("Malformed ciphertext padding! Exiting...");
        process::exit(1);
    }

    if let Ok(s) = String::from_utf8(decrypted.clone()) {
        println!("Decrypted (raw_text):");
        println!("{}", s);
    } else {
        println!("The decrypted bytes don't make sense in utf-8");
        println!("Decrypted (unpadded):");
        pretty_print_hex(&decrypted, block_size);
    }
}
