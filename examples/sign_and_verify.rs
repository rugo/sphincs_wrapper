// examples/sign_and_verify.rs
use sphincs_wrap::{sign_keypair, sign_signature, verify_signature};

fn main() {
    // Step 1: Generate a key pair
    let (public_key, secret_key) = sign_keypair().expect("Failed to generate keypair");

    println!("Generated Key Pair:");
    println!("Public Key: {:02x?}", public_key);
    println!("Secret Key: {:02x?}", secret_key);

    // Step 2: Create a message to sign
    let message = b"Hello, SPHINCS+!";
    println!("\nMessage to be signed: {:?}", std::str::from_utf8(message).unwrap());

    // Step 3: Sign the message
    let signature = sign_signature(message, &secret_key).expect("Failed to sign message");
    println!("\nGenerated Signature: {:02x?}", signature);

    // Step 4: Verify the signature
    match verify_signature(&signature, message, &public_key) {
        Ok(_) => println!("\nSignature verified successfully."),
        Err(_) => println!("\nFailed to verify signature."),
    }
}