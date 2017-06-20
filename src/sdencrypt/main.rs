extern crate crypto;
extern crate rand;
extern crate rpassword;

use crypto::symmetriccipher::*;
use crypto::aessafe::*;
use crypto::scrypt;
use rand::os::OsRng; // For salt
use rand::Rng;
use std::io;
use std::io::Read;
use std::process;


// PKCS#7 padding
fn pad_pkcs7(data: &mut Vec<u8>, block_size: usize) {
    let len = data.len();
    let bytes_occupied = len % block_size;
    if bytes_occupied != 0 {
        let pad_bytes_count = block_size - bytes_occupied;
        data.resize(len + pad_bytes_count, pad_bytes_count as u8);
    } else {
        data.append(&mut vec![0; 16]);
    }
}

// PKCS#7 unpadding (leaves data intact on malformed padding)
fn unpad_pkcs7(data: &mut Vec<u8>, block_size: usize) -> Result<(), String> {
    let len = data.len();

    if len % block_size != 0 {
        return Err(format!(
            "Unaligned data! (length {} not divisible by block size {})",
            len,
            block_size
        ));
    }

    match data.pop() {
        // The extra block case
        Some(0) => {
            // See if the remaining (block_size - 1) bytes are also 0
            for _ in 0..(block_size - 1) {
                if let Some(popped) = data.pop() {
                    if popped != 0 {
                        // Restore last pop along with the one at match start
                        data.push(0);
                        data.push(popped);
                        return Err(String::from("Malformed extra null block"));
                    }
                }
            }
        }
        Some(n) => {
            if n >= block_size as u8 {
                data.push(n);
                return Err(String::from("Malformed padding bytes are out of bounds"));
            }
            for _ in 0..(n - 1) {
                if let Some(popped) = data.pop() {
                    if popped != n {
                        // Restore last pop along with the one at match start
                        data.push(n);
                        data.push(popped);
                        return Err(String::from("Malformed padding bytes are inconsistent"));
                    }
                }
            }
        }
        None => return Err(String::from("Empty vector")),
    }

    Ok(())
}

//TODO: Chop main into smaller functions
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

    // TODO: Probably gonna want a vector here
    let mut key: [u8; 32] = [0; 32];

    // Default scrypt params: 14, 8, 1
    let params = scrypt::ScryptParams::new(1, 1, 1);
    scrypt::scrypt(pass.as_bytes(), b"", &params, &mut key);

    let encryptor = AesSafe256Encryptor::new(&key);
    let block_size = encryptor.block_size();

    let mut input: Vec<u8> = vec![0; 0];
    println!("Input the data to encrypt:");
    let bytes_read = match io::stdin().read_to_end(&mut input) {
        Err(e) => panic!("Could not read input! ({:#?})", e),
        Ok(n) => {
            print!("\nRead {} bytes", n);
            n
        }
    };
    println!(" ({} whole blocks)", bytes_read / block_size);

    // TODO: byte dumping function
    println!("Plaintext (unpadded):");
    for chunk in input.chunks(block_size) {
        for byte in chunk.iter() {
            print!("{:02X} ", byte);
        }
        print!("\n");
    }
    print!("\n");

    pad_pkcs7(&mut input, block_size);

    println!("Plaintext (padded):");
    for chunk in input.chunks(block_size) {
        for byte in chunk.iter() {
            print!("{:02X} ", byte);
        }
        print!("\n");
    }
    print!("\n");

    println!("Key:");
    for chunk in key.chunks(block_size) {
        for byte in chunk {
            print!("{:02X} ", byte);
        }
        print!("\n");
    }
    print!("\n");

    let mut encrypted: Vec<u8> = vec![0; 0];

    for chunk in input.chunks(block_size) {
        let mut current_cipher_block = [0u8; 16];
        encryptor.encrypt_block(&chunk, &mut current_cipher_block);
        encrypted.extend_from_slice(&mut current_cipher_block);
    }

    println!("Encrypted:");
    for chunk in encrypted.chunks(block_size) {
        for byte in chunk.iter() {
            print!("{:02X} ", byte);
        }
        print!("\n");
    }
    print!("\n");

    let decryptor = AesSafe256Decryptor::new(&key);

    let mut decrypted: Vec<u8> = vec![0; 0];

    for chunk in encrypted.chunks(block_size) {
        let mut current_plain_block = [0u8; 16];
        decryptor.decrypt_block(&chunk, &mut current_plain_block);
        decrypted.extend_from_slice(&mut current_plain_block);
    }

    println!("Decrypted (padded):");
    for chunk in decrypted.chunks(block_size) {
        for byte in chunk.iter() {
            print!("{:02X} ", byte);
        }
        print!("\n");
    }
    print!("\n");

    if let Err(msg) = unpad_pkcs7(&mut decrypted, block_size) {
        println!("Error: {}", msg);
        process::exit(1);
    }
    println!("Decrypted (unpadded):");
    for chunk in decrypted.chunks(block_size) {
        for byte in chunk.iter() {
            print!("{:02X} ", byte);
        }
        print!("\n");
    }
    print!("\n");

}
