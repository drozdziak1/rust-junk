extern crate crypto;
extern crate rand;
extern crate rpassword;

use crypto::symmetriccipher::*;
use crypto::aessafe::*;
use rand::os::OsRng;
use rand::Rng;


//TODO: Chop main into smaller functions
pub fn main() {
    let mut os_gen = OsRng::new().expect("Unable to use the OS-provided random number generator");

    let key: [u8; 32] = os_gen.gen();

    let pass: String;
    loop {
        match rpassword::prompt_password_stderr("Choose a password:\t") {
            Ok(first) => println!("You've entered: {}", first),
            Err(_) => {
                println!("Oopsies!");
                break;
            }
        }
    }

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
}
