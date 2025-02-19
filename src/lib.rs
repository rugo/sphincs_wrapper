#![no_std]

// Include values generated from SPHINCS param header
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use core::{result::Result};

// Define a custom error type for handling errors in a no_std context
#[derive(Debug)]
pub enum CryptoError {
    OperationFailed(i32),
}

pub const SPX_PUBKEY_SIZE: usize = SPX_PK_BYTES as usize;
pub const SPX_PRIVKEY_SIZE: usize = SPX_SK_BYTES as usize;
pub const SPX_SIG_SIZE: usize = SPX_BYTES as usize;


// FFI bindings to the C functions
#[link(name = "sphincs_wrap_c", kind = "static")]
extern "C" {
    fn crypto_sign_seed_keypair(pk: *mut u8, sk: *mut u8, seed: *const u8) -> i32;
    fn crypto_sign_keypair(pk: *mut u8, sk: *mut u8) -> i32;
    fn crypto_sign_signature(sig: *mut u8, siglen: *mut usize, m: *const u8, mlen: usize, sk: *const u8) -> i32;
    fn crypto_sign_verify(sig: *const u8, siglen: usize, m: *const u8, mlen: usize, pk: *const u8) -> i32;
}

// Wrapper for `crypto_sign_seed_keypair`
pub fn sign_seed_keypair(seed: &[u8]) -> Result<([u8; SPX_PUBKEY_SIZE], [u8; SPX_PRIVKEY_SIZE]), CryptoError> {
    let mut pk = [0u8; SPX_PUBKEY_SIZE]; 
    let mut sk = [0u8; SPX_PRIVKEY_SIZE];

    let result = unsafe {
        crypto_sign_seed_keypair(pk.as_mut_ptr(), sk.as_mut_ptr(), seed.as_ptr())
    };

    if result == 0 {
        Ok((pk, sk))
    } else {
        Err(CryptoError::OperationFailed(result))
    }
}

// Wrapper for `crypto_sign_keypair`
pub fn sign_keypair() -> Result<([u8; SPX_PUBKEY_SIZE], [u8; SPX_PRIVKEY_SIZE]), CryptoError> {
    let mut pk = [0u8; SPX_PUBKEY_SIZE]; 
    let mut sk = [0u8; SPX_PRIVKEY_SIZE];

    let result = unsafe {
        crypto_sign_keypair(pk.as_mut_ptr(), sk.as_mut_ptr())
    };

    if result == 0 {
        Ok((pk, sk))
    } else {
        Err(CryptoError::OperationFailed(result))
    }
}

// Wrapper for `crypto_sign_signature`
pub fn sign_signature(message: &[u8], sk: &[u8]) -> Result<([u8; SPX_SIG_SIZE], usize), CryptoError> {
    /*let mut sig = [0u8; SPX_SIG_SIZE]; 
    let mut siglen = 0usize;
    unsafe {
        crypto_sign_signature(sig.as_mut_ptr(), &mut siglen, message.as_ptr(), message.len(), sk.as_ptr())
    };
    Ok((sig, siglen))*/
    let mut sig = [0u8; SPX_SIG_SIZE]; 
    let mut siglen = 0usize;

    let result = unsafe {
        crypto_sign_signature(sig.as_mut_ptr(), &mut siglen, message.as_ptr(), message.len(), sk.as_ptr())
    };

    if result == 0 {
        Ok((sig, siglen))
    } else {
        Err(CryptoError::OperationFailed(result))
    }
}

// Wrapper for `crypto_sign_verify`
pub fn verify_signature(signature: &[u8], message: &[u8], pk: &[u8]) -> Result<(), CryptoError> {
    let result = unsafe {
        crypto_sign_verify(signature.as_ptr(), signature.len(), message.as_ptr(), message.len(), pk.as_ptr())
    };

    if result == 0 {
        Ok(())
    } else {
        Err(CryptoError::OperationFailed(result))
    }
}
