extern crate crypto;
extern crate rand;
extern crate rpassword;

use crypto::symmetriccipher::*;
use crypto::aessafe::*;
use crypto::scrypt;
use rand::os::OsRng;
use rand::Rng;


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

    // default scrypt params
    let params = scrypt::ScryptParams::new(14, 8, 1);

    let mut key: [u8; 16] = [0; 16];

    scrypt::scrypt(pass.as_bytes(), b"", &params, &mut key);

    let encryptor = AesSafe256Encryptor::new(&key);
    println!("Block size: {}", encryptor.block_size());

    print!("Key:\t\t");
    for byte in key.iter() {
        print!("{:02X} ", byte);
    }
    print!("\n");

    let mut input: Vec<u8> = vec![0; 16];
    input[..4].clone_from_slice("dupa".as_bytes());

    print!("Input:\t\t");
    for byte in input.iter() {
        print!("{:02X} ", byte);
    }
    print!("\n");

    let mut output: [u8; 16] = [0; 16];
    encryptor.encrypt_block(&input, &mut output);

    print!("Encrypted:\t");
    for byte in output.iter() {
        print!("{:02X} ", byte);
    }
    print!("\n");

    let decryptor = AesSafe256Decryptor::new(&key);

    let mut decrypted: [u8; 16] = [0; 16];
    decryptor.decrypt_block(&output, &mut decrypted);

    print!("Decrypted:\t");
    for byte in decrypted.iter() {
        print!("{:02X} ", byte);
    }
    print!("\n");

}
